//! The lossless mode of grammar interchange.
//!
//! Importing a source in lossless mode keeps, beside the grammar, the layout
//! of the source: the text before the first rule, every rule definition's own
//! text with the fingerprint of the rule it defines (its native links line)
//! and the text after it up to the next definition. Rules the importer adds
//! without a definition in the source (the ABNF core rules) are kept as
//! implicit fingerprints. Emitting in lossless mode walks the layout: a rule
//! whose fingerprint still matches is written with its original text, a
//! changed rule is written where it was defined with the text a fresh emission
//! gives it, a new rule follows the rule before it in grammar order, and an
//! unchanged implicit rule is left implicit. An unchanged grammar therefore
//! reconstructs its source exactly, and an edited grammar keeps every
//! untouched definition, comment and blank line. The layout has its own links
//! form, so the source is reconstructed from links alone. It mirrors
//! `js/src/grammar-lossless.js`.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};

use super::super::emit::{EmitReport, GrammarEmitError};
use super::super::import::GrammarImportError;
use super::super::{Grammar, GrammarFormat};
use super::links::{percent_decode_links_text, percent_encode_links_text, render_rule_link};
use super::{grammar_emitter, grammar_importer};

/// The formats the lossless mode reads and writes.
pub const GRAMMAR_LOSSLESS_FORMATS: &[&str] = &[
    "abnf",
    "antlr",
    "bnf",
    "ebnf",
    "gbnf",
    "lark",
    "pest",
    "tree-sitter-json",
];

/// Formats whose importers read the comment lines above a rule as its
/// documentation.
const DOC_FORMATS: &[&str] = &["antlr", "gbnf", "lark"];

/// One rule definition of a source and the text after it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarSourceDefinition {
    /// The rule it defines.
    pub name: String,
    /// The definition's own text, comments above it included where the
    /// importer reads them as documentation.
    pub text: String,
    /// The text after the definition up to the next one or the end.
    pub gap: String,
}

/// A source split into the text before its first definition and the
/// definitions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarSourceSplit {
    /// The text before the first definition.
    pub prefix: String,
    /// The definitions in source order.
    pub members: Vec<GrammarSourceDefinition>,
}

/// One definition of a layout with the fingerprint of its rule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarLayoutDefinition {
    /// The rule it defines.
    pub name: String,
    /// The definition's own text.
    pub text: String,
    /// The text after the definition up to the next one or the end.
    pub gap: String,
    /// The native links line of the rule when the source was imported.
    pub fingerprint: String,
}

/// A rule the importer adds without a definition in the source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarLayoutImplicit {
    /// The rule name.
    pub name: String,
    /// The native links line of the rule when the source was imported.
    pub fingerprint: String,
}

/// The layout that reconstructs a source from its grammar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarLayout {
    /// The interchange format tag of the source.
    pub format: String,
    /// The text before the first definition.
    pub prefix: String,
    /// The definitions in source order.
    pub members: Vec<GrammarLayoutDefinition>,
    /// The rules defined nowhere in the source.
    pub implicit: Vec<GrammarLayoutImplicit>,
}

/// Why the lossless mode could not run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarLosslessError {
    /// The format is not one of [`GRAMMAR_LOSSLESS_FORMATS`].
    UnsupportedFormat(String),
    /// The source, the layout or its links are malformed.
    Import(GrammarImportError),
    /// The emitter rejected the grammar.
    Emit(GrammarEmitError),
}

impl fmt::Display for GrammarLosslessError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedFormat(format) => write!(
                formatter,
                "the lossless mode does not support the format {format}"
            ),
            Self::Import(error) => write!(formatter, "{error}"),
            Self::Emit(error) => write!(formatter, "{error}"),
        }
    }
}

impl Error for GrammarLosslessError {}

impl From<GrammarImportError> for GrammarLosslessError {
    fn from(error: GrammarImportError) -> Self {
        Self::Import(error)
    }
}

impl From<GrammarEmitError> for GrammarLosslessError {
    fn from(error: GrammarEmitError) -> Self {
        Self::Emit(error)
    }
}

fn layout_error(detail: impl AsRef<str>) -> GrammarLosslessError {
    GrammarLosslessError::Import(GrammarImportError::Parse {
        format: GrammarFormat::MetaLanguage,
        message: format!("layout: {}", detail.as_ref()),
    })
}

fn check_format(format: &str) -> Result<&'static str, GrammarLosslessError> {
    GRAMMAR_LOSSLESS_FORMATS
        .iter()
        .find(|known| **known == format)
        .copied()
        .ok_or_else(|| GrammarLosslessError::UnsupportedFormat(format.to_owned()))
}

/// A rule definition span of a source, in bytes.
struct Span {
    name: String,
    start: usize,
    end: usize,
}

/// Splits `source` into the text before the first rule definition and the
/// definitions of the rules in `names`, each with the text after it.
///
/// # Errors
///
/// Returns [`GrammarLosslessError::UnsupportedFormat`] for a format outside
/// [`GRAMMAR_LOSSLESS_FORMATS`] and a layout error for malformed JSON.
pub fn split_grammar_source(
    source: &str,
    format: &str,
    names: &[&str],
) -> Result<GrammarSourceSplit, GrammarLosslessError> {
    let format = check_format(format)?;
    let spans = if format == "tree-sitter-json" {
        json_rule_spans(source, names)?
    } else {
        line_rule_spans(source, format, names)
    };
    let Some(first) = spans.first() else {
        return Ok(GrammarSourceSplit {
            prefix: source.to_owned(),
            members: Vec::new(),
        });
    };
    Ok(GrammarSourceSplit {
        prefix: source[..first.start].to_owned(),
        members: spans
            .iter()
            .enumerate()
            .map(|(index, span)| {
                let next = spans.get(index + 1).map_or(source.len(), |next| next.start);
                GrammarSourceDefinition {
                    name: span.name.clone(),
                    text: source[span.start..span.end].to_owned(),
                    gap: source[span.end..next].to_owned(),
                }
            })
            .collect(),
    })
}

/// Trims what `String.prototype.trim` trims, so both runtimes agree on blank
/// lines.
fn js_trim(text: &str) -> &str {
    text.trim_matches(|value: char| {
        (value.is_whitespace() && value != '\u{85}') || value == '\u{feff}'
    })
}

const fn ident_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

const fn ident_rest(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The end of a run of bytes matching `first` then `rest`, if any.
fn run(bytes: &[u8], at: usize, first: fn(u8) -> bool, rest: fn(u8) -> bool) -> Option<usize> {
    if !bytes.get(at).copied().is_some_and(first) {
        return None;
    }
    let mut end = at + 1;
    while bytes.get(end).copied().is_some_and(rest) {
        end += 1;
    }
    Some(end)
}

fn skip_blanks(bytes: &[u8], at: usize) -> usize {
    let mut end = at;
    while matches!(bytes.get(end), Some(b' ' | b'\t')) {
        end += 1;
    }
    end
}

fn starts_at(bytes: &[u8], at: usize, text: &str) -> bool {
    bytes
        .get(at..)
        .is_some_and(|rest| rest.starts_with(text.as_bytes()))
}

/// The rule name a line starting with a rule head of `format` defines.
fn head_name<'a>(format: &str, line: &'a str) -> Option<&'a str> {
    let bytes = line.as_bytes();
    let named = |start: usize, end: usize, follows: bool| follows.then(|| &line[start..end]);
    match format {
        "abnf" => {
            let end = run(
                bytes,
                0,
                |byte| byte.is_ascii_alphabetic(),
                |byte| byte.is_ascii_alphanumeric() || byte == b'-',
            )?;
            named(0, end, starts_at(bytes, skip_blanks(bytes, end), "="))
        }
        "antlr" => {
            let rule_at = |start: usize| {
                let end = run(bytes, start, ident_start, ident_rest)?;
                let after = skip_blanks(bytes, end);
                named(start, end, after == bytes.len() || bytes[after] == b':')
            };
            let fragment = b"fragment".len();
            let spaced = skip_blanks(bytes, fragment);
            (starts_at(bytes, 0, "fragment") && spaced > fragment)
                .then(|| rule_at(spaced))
                .flatten()
                .or_else(|| rule_at(0))
        }
        "bnf" => {
            if bytes.first() != Some(&b'<') {
                return None;
            }
            let close = 1 + bytes[1..]
                .iter()
                .position(|byte| matches!(byte, b'<' | b'>'))?;
            (bytes[close] == b'>' && starts_at(bytes, skip_blanks(bytes, close + 1), "::="))
                .then(|| &line[1..close])
        }
        "ebnf" => {
            let end = run(bytes, 0, ident_start, |byte| {
                ident_rest(byte) || byte == b'-'
            })?;
            let after = skip_blanks(bytes, end);
            named(
                0,
                end,
                starts_at(bytes, after, "::=") || starts_at(bytes, after, "="),
            )
        }
        "gbnf" => {
            let word = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'-';
            let end = run(bytes, 0, word, word)?;
            named(0, end, starts_at(bytes, skip_blanks(bytes, end), "::="))
        }
        "lark" => {
            let start = usize::from(matches!(bytes.first(), Some(b'?' | b'!')));
            let end = run(bytes, start, ident_start, ident_rest)?;
            let mut after = end;
            if bytes.get(after) == Some(&b'.') {
                let digits = after + 1 + usize::from(bytes.get(after + 1) == Some(&b'-'));
                if let Some(digits_end) = run(
                    bytes,
                    digits,
                    |byte| byte.is_ascii_digit(),
                    |byte| byte.is_ascii_digit(),
                ) {
                    after = digits_end;
                }
            }
            named(start, end, starts_at(bytes, skip_blanks(bytes, after), ":"))
        }
        "pest" => {
            let end = run(bytes, 0, ident_start, ident_rest)?;
            named(0, end, starts_at(bytes, skip_blanks(bytes, end), "="))
        }
        _ => None,
    }
}

/// How a comment line of each line-based format starts.
fn comment_prefixes(format: &str) -> &'static [&'static str] {
    match format {
        "abnf" | "bnf" => &[";"],
        "antlr" => &["//", "/*", "*"],
        "ebnf" => &["(*"],
        "gbnf" => &["#"],
        _ => &["//"],
    }
}

struct Line<'a> {
    start: usize,
    end: usize,
    text: &'a str,
}

fn line_rule_spans(source: &str, format: &str, names: &[&str]) -> Vec<Span> {
    let mut known: BTreeMap<String, &str> = BTreeMap::new();
    for name in names {
        known.insert((*name).to_owned(), name);
        if format == "abnf" {
            known.entry(name.to_lowercase()).or_insert(name);
        }
    }
    let mut lines = Vec::new();
    let mut start = 0;
    while start < source.len() {
        let end = source[start..]
            .find('\n')
            .map_or(source.len(), |offset| start + offset);
        lines.push(Line {
            start,
            end,
            text: &source[start..end],
        });
        start = if end < source.len() { end + 1 } else { end };
    }
    let prefixes = comment_prefixes(format);
    let trivia = |line: &Line<'_>| {
        let text = js_trim(line.text);
        text.is_empty() || prefixes.iter().any(|prefix| text.starts_with(prefix))
    };
    let mut heads: Vec<(usize, &str)> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let Some(found) = head_name(format, line.text) else {
            continue;
        };
        let name = known.get(found).or_else(|| {
            (format == "abnf")
                .then(|| known.get(&found.to_lowercase()))
                .flatten()
        });
        if let Some(name) = name {
            heads.push((index, name));
        }
    }
    let documented = DOC_FORMATS.contains(&format);
    let firsts: Vec<usize> = heads
        .iter()
        .enumerate()
        .map(|(position, (index, _))| {
            let mut first = *index;
            let floor = if position == 0 {
                0
            } else {
                heads[position - 1].0 + 1
            };
            if documented {
                while first > floor
                    && !js_trim(lines[first - 1].text).is_empty()
                    && trivia(&lines[first - 1])
                {
                    first -= 1;
                }
            }
            first
        })
        .collect();
    heads
        .iter()
        .enumerate()
        .map(|(position, (index, name))| {
            let limit = firsts.get(position + 1).copied().unwrap_or(lines.len());
            let mut last = limit - 1;
            while last > *index && trivia(&lines[last]) {
                last -= 1;
            }
            Span {
                name: (*name).to_owned(),
                start: lines[firsts[position]].start,
                end: lines[last].end,
            }
        })
        .collect()
}

fn skip_json_space(source: &[u8], index: usize) -> usize {
    let mut at = index;
    while matches!(source.get(at), Some(b' ' | b'\t' | b'\r' | b'\n')) {
        at += 1;
    }
    at
}

fn json_string_end(source: &[u8], index: usize) -> Result<usize, GrammarLosslessError> {
    if source.get(index) != Some(&b'"') {
        return Err(layout_error(format!(
            "expected a JSON string at offset {index}"
        )));
    }
    let mut at = index + 1;
    while at < source.len() {
        match source[at] {
            b'\\' => at += 1,
            b'"' => return Ok(at + 1),
            _ => {}
        }
        at += 1;
    }
    Err(layout_error("unterminated JSON string"))
}

fn json_value_end(source: &[u8], index: usize) -> Result<usize, GrammarLosslessError> {
    match source.get(index) {
        Some(b'"') => return json_string_end(source, index),
        Some(b'{' | b'[') => {}
        _ => {
            let mut at = index;
            while at < source.len() && !b",}] \t\r\n".contains(&source[at]) {
                at += 1;
            }
            if at == index {
                return Err(layout_error(format!(
                    "expected a JSON value at offset {index}"
                )));
            }
            return Ok(at);
        }
    }
    let mut depth = 0_usize;
    let mut at = index;
    while at < source.len() {
        match source[at] {
            b'"' => at = json_string_end(source, at)? - 1,
            b'{' | b'[' => depth += 1,
            b'}' | b']' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(at + 1);
                }
            }
            _ => {}
        }
        at += 1;
    }
    Err(layout_error("unterminated JSON value"))
}

/// One member of a JSON object: its key, where the key starts, where the
/// value starts and where it ends.
struct JsonMember {
    key: String,
    start: usize,
    value: usize,
    end: usize,
}

fn json_members(source: &str, index: usize) -> Result<Vec<JsonMember>, GrammarLosslessError> {
    let bytes = source.as_bytes();
    if bytes.get(index) != Some(&b'{') {
        return Err(layout_error(format!(
            "expected a JSON object at offset {index}"
        )));
    }
    let mut members = Vec::new();
    let mut at = skip_json_space(bytes, index + 1);
    while bytes.get(at) != Some(&b'}') {
        let key_end = json_string_end(bytes, at)?;
        let key: String = serde_json::from_str(&source[at..key_end])
            .map_err(|error| layout_error(error.to_string()))?;
        let mut value = skip_json_space(bytes, key_end);
        if bytes.get(value) != Some(&b':') {
            return Err(layout_error(format!("expected : at offset {value}")));
        }
        value = skip_json_space(bytes, value + 1);
        let end = json_value_end(bytes, value)?;
        members.push(JsonMember {
            key,
            start: at,
            value,
            end,
        });
        at = skip_json_space(bytes, end);
        if bytes.get(at) == Some(&b',') {
            at = skip_json_space(bytes, at + 1);
        } else if bytes.get(at) != Some(&b'}') {
            return Err(layout_error(format!("expected , or }} at offset {at}")));
        }
    }
    Ok(members)
}

fn json_rule_spans(source: &str, names: &[&str]) -> Result<Vec<Span>, GrammarLosslessError> {
    let members = json_members(source, skip_json_space(source.as_bytes(), 0))?;
    let Some(rules) = members.iter().find(|member| member.key == "rules") else {
        return Ok(Vec::new());
    };
    if source.as_bytes()[rules.value] != b'{' {
        return Ok(Vec::new());
    }
    Ok(json_members(source, rules.value)?
        .into_iter()
        .filter(|member| names.contains(&member.key.as_str()))
        .map(|member| Span {
            name: member.key,
            start: member.start,
            end: member.end,
        })
        .collect())
}

fn fingerprint(grammar: &Grammar, name: &str) -> String {
    render_rule_link(grammar.rule(name).expect("the name comes from the grammar"))
}

/// The layout of `source` for `grammar`, the grammar imported from it: its
/// definitions with the fingerprints of their rules, and the fingerprints of
/// the rules defined nowhere in the source.
///
/// # Errors
///
/// Returns [`GrammarLosslessError::UnsupportedFormat`] for a format outside
/// [`GRAMMAR_LOSSLESS_FORMATS`] and a layout error for malformed JSON.
pub fn capture_grammar_layout(
    source: &str,
    format: &str,
    grammar: &Grammar,
) -> Result<GrammarLayout, GrammarLosslessError> {
    let names = grammar.rule_names();
    let split = split_grammar_source(source, format, &names)?;
    let defined: BTreeSet<&str> = split
        .members
        .iter()
        .map(|member| member.name.as_str())
        .collect();
    let implicit = names
        .iter()
        .filter(|name| !defined.contains(**name))
        .map(|name| GrammarLayoutImplicit {
            name: (*name).to_owned(),
            fingerprint: fingerprint(grammar, name),
        })
        .collect();
    Ok(GrammarLayout {
        format: format.to_owned(),
        prefix: split.prefix,
        members: split
            .members
            .into_iter()
            .map(|member| GrammarLayoutDefinition {
                fingerprint: fingerprint(grammar, &member.name),
                name: member.name,
                text: member.text,
                gap: member.gap,
            })
            .collect(),
        implicit,
    })
}

/// Imports `source` of `format` with the layout that reconstructs it.
///
/// # Errors
///
/// Returns [`GrammarLosslessError::UnsupportedFormat`] for a format outside
/// [`GRAMMAR_LOSSLESS_FORMATS`] and the importer's error for a malformed
/// source.
pub fn import_grammar_lossless(
    source: &str,
    format: &str,
) -> Result<(Grammar, GrammarLayout), GrammarLosslessError> {
    let format = check_format(format)?;
    let importer = grammar_importer(format).expect("every lossless format has an importer");
    let grammar = importer(source)?;
    let layout = capture_grammar_layout(source, format, &grammar)?;
    Ok((grammar, layout))
}

/// One piece of the reconstructed source: its text and, for a definition kept
/// in place, its index in the layout.
struct Item {
    text: String,
    index: Option<usize>,
}

/// Emits `grammar` with `layout`, the layout of the source it was imported
/// from: unchanged definitions keep their original text and only changed or
/// new rules are written fresh.
///
/// The report carries the fresh emission's notes, plus a note when the layout
/// could not place a rule and the whole grammar was emitted fresh instead.
///
/// # Errors
///
/// Returns [`GrammarLosslessError::UnsupportedFormat`] for a layout format
/// outside [`GRAMMAR_LOSSLESS_FORMATS`] and the emitter's error.
pub fn emit_grammar_lossless(
    grammar: &Grammar,
    layout: &GrammarLayout,
) -> Result<(String, EmitReport), GrammarLosslessError> {
    let format = check_format(&layout.format)?;
    let names = grammar.rule_names();
    let fingerprints: BTreeMap<&str, String> = names
        .iter()
        .map(|name| (*name, fingerprint(grammar, name)))
        .collect();
    let kept = |name: &str, print: &str| fingerprints.get(name).map(String::as_str) == Some(print);
    let mut defined_at: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
    for (index, member) in layout.members.iter().enumerate() {
        defined_at
            .entry(member.name.as_str())
            .or_default()
            .push(index);
    }
    let implicit: BTreeSet<&str> = layout
        .implicit
        .iter()
        .filter(|entry| kept(&entry.name, &entry.fingerprint))
        .map(|entry| entry.name.as_str())
        .collect();
    let unchanged = |name: &str| {
        defined_at.get(name).is_some_and(|indices| {
            indices.iter().all(|index| {
                let member = &layout.members[*index];
                kept(&member.name, &member.fingerprint)
            })
        })
    };
    let fresh: BTreeSet<&str> = names
        .iter()
        .copied()
        .filter(|name| !unchanged(name) && !implicit.contains(name))
        .collect();

    let mut report = EmitReport::default();
    let mut fresh_text: BTreeMap<String, String> = BTreeMap::new();
    if !fresh.is_empty() {
        let emitter = grammar_emitter(format).expect("every lossless format has an emitter");
        let (whole, whole_report) = emitter(grammar)?;
        report = whole_report;
        for member in split_grammar_source(&whole, format, &names)?.members {
            fresh_text.entry(member.name).or_insert(member.text);
        }
        if let Some(missing) = names
            .iter()
            .find(|name| fresh.contains(**name) && !fresh_text.contains_key(**name))
        {
            report.lossy.push(format!(
                "the layout could not place rule {missing}, so the whole grammar was emitted fresh"
            ));
            return Ok((whole, report));
        }
    }

    // New rules follow the nearest rule before them in grammar order.
    let mut following: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    let mut leading = Vec::new();
    let mut previous: Option<&str> = None;
    for name in names.iter().copied() {
        if implicit.contains(name) {
            continue;
        }
        if !defined_at.contains_key(name) {
            match previous {
                None => leading.push(name),
                Some(previous) => following.entry(previous).or_default().push(name),
            }
        }
        previous = Some(name);
    }
    let mut items = Vec::new();
    let place_new = |items: &mut Vec<Item>, names: &[&str]| {
        let mut stack: Vec<&str> = names.iter().rev().copied().collect();
        while let Some(name) = stack.pop() {
            items.push(Item {
                text: fresh_text[name].clone(),
                index: None,
            });
            if let Some(next) = following.get(name) {
                stack.extend(next.iter().rev());
            }
        }
    };
    place_new(&mut items, &leading);
    for (index, member) in layout.members.iter().enumerate() {
        if !fingerprints.contains_key(member.name.as_str()) {
            continue;
        }
        let indices = &defined_at[member.name.as_str()];
        if !fresh.contains(member.name.as_str()) {
            items.push(Item {
                text: member.text.clone(),
                index: Some(index),
            });
        } else if indices.first() == Some(&index) {
            items.push(Item {
                text: fresh_text[&member.name].clone(),
                index: Some(index),
            });
        }
        if indices.last() == Some(&index)
            && let Some(next) = following.get(member.name.as_str())
        {
            place_new(&mut items, next);
        }
    }

    Ok((join_items(layout, &items), report))
}

/// Joins the reconstructed pieces. The text between two definitions stays
/// before the definition it preceded (comments above a rule stay with it); a
/// definition followed by a new rule keeps the text after it when the
/// definition after it is gone. Each text is written once, and the text after
/// the last definition ends the source.
fn join_items(layout: &GrammarLayout, items: &[Item]) -> String {
    let last = layout.members.len().checked_sub(1);
    let fallback = if layout.format == "tree-sitter-json" {
        match last {
            Some(last) if last > 0 => layout.members[0].gap.as_str(),
            _ => ",\n    ",
        }
    } else {
        "\n"
    };
    let placed: BTreeSet<usize> = items.iter().filter_map(|item| item.index).collect();
    let mut used = BTreeSet::new();
    let mut source = layout.prefix.clone();
    for (position, item) in items.iter().enumerate() {
        if position > 0 {
            let before = &items[position - 1];
            let gap = match (before.index, item.index) {
                (_, Some(after)) if after > 0 => Some(after - 1),
                (Some(before), None)
                    if last.is_some_and(|last| before < last)
                        && !placed.contains(&(before + 1)) =>
                {
                    Some(before)
                }
                _ => None,
            };
            match gap {
                Some(gap) if used.insert(gap) => source.push_str(&layout.members[gap].gap),
                _ => source.push_str(fallback),
            }
        }
        source.push_str(&item.text);
    }
    if let Some(last) = last {
        source.push_str(&layout.members[last].gap);
    }
    source
}

/// Renders the links form of `layout`, one link per line.
#[must_use]
pub fn render_grammar_layout_links(layout: &GrammarLayout) -> String {
    let text = |value: &str| percent_encode_links_text(value);
    let mut lines = vec![format!(
        "(layout {} {})",
        layout.format,
        text(&layout.prefix)
    )];
    for member in &layout.members {
        lines.push(format!(
            "(define {} {} {} {})",
            text(&member.name),
            text(&member.fingerprint),
            text(&member.text),
            text(&member.gap)
        ));
    }
    for entry in &layout.implicit {
        lines.push(format!(
            "(implicit {} {})",
            text(&entry.name),
            text(&entry.fingerprint)
        ));
    }
    lines.iter().fold(String::new(), |mut text, line| {
        text.push_str(line);
        text.push('\n');
        text
    })
}

/// The words of one layout link.
fn words(statement: &LiNo<String>) -> Result<Vec<&str>, GrammarLosslessError> {
    fn word(value: &LiNo<String>) -> Result<&str, GrammarLosslessError> {
        match value {
            LiNo::Ref(word) => Ok(word.as_str()),
            LiNo::Link {
                id: Some(word),
                values,
            } if values.is_empty() => Ok(word.as_str()),
            LiNo::Link { .. } => Err(layout_error("layout links hold words only")),
        }
    }
    match statement {
        LiNo::Link { id: None, values } if !values.is_empty() => values.iter().map(word).collect(),
        LiNo::Link {
            id: Some(id),
            values,
        } if !values.is_empty() => Err(layout_error(format!("unexpected identified link {id}"))),
        _ => Ok(vec![word(statement)?]),
    }
}

/// Reads the links form written by [`render_grammar_layout_links`].
///
/// # Errors
///
/// Returns a layout error for malformed links, an unknown format or an
/// unexpected link.
pub fn parse_grammar_layout_links(source: &str) -> Result<GrammarLayout, GrammarLosslessError> {
    let statements = parse_lino_to_links_with_config(source, &ParserConfig::without_comments())
        .map_err(|error| layout_error(error.to_string()))?;
    let mut statements = statements.iter().map(words);
    let header = statements.next().transpose()?.unwrap_or_default();
    let [head, format, prefix] = header.as_slice() else {
        return Err(layout_error(
            "the first link must be (layout FORMAT PREFIX)",
        ));
    };
    if *head != "layout" {
        return Err(layout_error(
            "the first link must be (layout FORMAT PREFIX)",
        ));
    }
    if !GRAMMAR_LOSSLESS_FORMATS.contains(format) {
        return Err(layout_error(format!("unknown layout format {format}")));
    }
    let mut layout = GrammarLayout {
        format: (*format).to_owned(),
        prefix: percent_decode_links_text(prefix)?,
        members: Vec::new(),
        implicit: Vec::new(),
    };
    for statement in statements {
        let statement = statement?;
        let (head, values) = statement
            .split_first()
            .expect("a statement has at least one word");
        let decoded = values
            .iter()
            .map(|value| percent_decode_links_text(value))
            .collect::<Result<Vec<_>, _>>()?;
        match (*head, decoded.as_slice()) {
            ("define", [name, fingerprint, text, gap]) if layout.implicit.is_empty() => {
                layout.members.push(GrammarLayoutDefinition {
                    name: name.clone(),
                    text: text.clone(),
                    gap: gap.clone(),
                    fingerprint: fingerprint.clone(),
                });
            }
            ("implicit", [name, fingerprint]) => layout.implicit.push(GrammarLayoutImplicit {
                name: name.clone(),
                fingerprint: fingerprint.clone(),
            }),
            _ => return Err(layout_error(format!("unexpected layout link {head}"))),
        }
    }
    Ok(layout)
}
