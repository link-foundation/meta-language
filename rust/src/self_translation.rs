//! Translates meta-language's own modules between JavaScript, TypeScript and
//! Rust through meta-language links.
//!
//! The source is parsed losslessly with its default grammar; a translation
//! into the same language writes the links back byte for byte, and a
//! translation into the other language translates each top-level item the
//! portable core can express and carries every other item verbatim in a
//! marked comment. Both kinds of item keep their source as provenance, so
//! translating an unedited translation back restores the source byte for
//! byte.
//!
//! Mirrors `js/src/self-translation.js`.

use crate::translation::frontend_rules::{
    accept_binding_scope, accept_literal_binding, accept_module_binding_scope, find_binding_run_end,
};
use crate::translation::surface::{SEffect, SExpr, SNode};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Write as _;
use std::sync::LazyLock;

use regex::Regex;
use sha2::{Digest, Sha256};

use crate::decorators::DecoratorSet;
use crate::grammar::decorate_emitted;
use crate::translation::check::check_program;
use crate::translation::diagnostics::TranslationError;
use crate::translation::emit_common::Emitted;
use crate::translation::{
    emit_javascript::emit_javascript,
    emit_rust::{emit_rust, emit_rust_constants},
    javascript::{parse_javascript, parse_javascript_bound},
    rust::parse_rust,
};
use crate::{LinkNetwork, ParseConfiguration};

/// The languages self-translation reads and writes.
pub const SELF_TRANSLATION_LANGUAGES: [&str; 3] = ["JavaScript", "TypeScript", "Rust"];

const HEADER: &str = "// meta-language:self-translation:v1 ";
const CARRIED: &str = "// meta-language:carried ";
const TRANSLATED: &str = "// meta-language:translated ";
const SOURCE_LINE: &str = "// |";
const PRELUDE_BEGIN: &str = "// meta-language:prelude begin";
const PRELUDE_END: &str = "// meta-language:prelude end";
// Portable-core Rust keeps the source's parentheses and names.
const RUST_ALLOW: &str = "#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types, invalid_nan_comparisons)]";
const COMMENTS: [&str; 3] = ["comment", "line_comment", "block_comment"];

static RUST_EXPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^pub(\([^)]*\))?\s").expect("a valid pattern"));
static JAVASCRIPT_EXPORT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^export\s").expect("a valid pattern"));
static RUST_MAIN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bfn\s+main\s*\(").expect("a valid pattern"));
static LINE_BREAK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\r?\n").expect("a valid pattern"));

/// An unknown language or a source the links do not reproduce.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelfTranslationError {
    /// What went wrong.
    pub message: String,
}

impl fmt::Display for SelfTranslationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for SelfTranslationError {}

/// A top-level item of the source and what self-translation did with it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelfTranslationItem {
    /// The item's syntax node kind.
    pub term: String,
    /// The item's first byte in the source.
    pub start: usize,
    /// The byte after the item in the source.
    pub end: usize,
    /// `kept` (same language), `translated`, `restored` (provenance gave back
    /// its source), `comment`, `provenance` (a self-translation header or
    /// prelude) or `carried`.
    pub status: &'static str,
    /// Why a carried item was carried.
    pub reason: Option<String>,
}

/// A translated module with the status of each of its top-level items.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SelfTranslation {
    /// The canonical source language.
    pub source_language: &'static str,
    /// The canonical target language.
    pub target_language: &'static str,
    /// The translated module.
    pub code: String,
    /// The source's top-level items in order.
    pub items: Vec<SelfTranslationItem>,
}

/// The self-translation language `language` names, or `None`.
#[must_use]
pub fn self_translation_language(language: &str) -> Option<&'static str> {
    match language.to_lowercase().as_str() {
        "javascript" | "js" | "mjs" => Some("JavaScript"),
        "typescript" | "ts" => Some("TypeScript"),
        "rust" | "rs" => Some("Rust"),
        _ => None,
    }
}

fn required(language: &str) -> Result<&'static str, SelfTranslationError> {
    self_translation_language(language).ok_or_else(|| SelfTranslationError {
        message: format!(
            "self-translation reads and writes {}, not {language}",
            SELF_TRANSLATION_LANGUAGES.join(", ")
        ),
    })
}

// The family a language's code is written in: TypeScript items are read and
// written as JavaScript.
fn family(language: &str) -> &'static str {
    if language == "Rust" {
        "Rust"
    } else {
        "JavaScript"
    }
}

fn sha256(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .fold(String::new(), |mut hex, byte| {
            let _ = write!(hex, "{byte:02x}");
            hex
        })
}

#[derive(Clone, Debug)]
struct Item<'a> {
    term: String,
    start: usize,
    end: usize,
    comment: bool,
    text: &'a str,
    after: &'a str,
}

/// Translates `source` from `source_language` to `target_language`, each one
/// of [`SELF_TRANSLATION_LANGUAGES`] or an alias of one.
///
/// # Errors
///
/// A [`SelfTranslationError`] for an unknown language or a source its links
/// do not reproduce.
pub fn self_translate(
    source: &str,
    source_language: &str,
    target_language: &str,
) -> Result<SelfTranslation, SelfTranslationError> {
    self_translate_decorated(
        source,
        source_language,
        target_language,
        &DecoratorSet::default(),
    )
}

/// [`self_translate`] with the emitter `decorators` applied.
///
/// They decorate the code each translated item writes before its provenance
/// marker is hashed, so the decorated translation still translates back to
/// `source`.
///
/// # Errors
///
/// As [`self_translate`].
pub fn self_translate_decorated(
    source: &str,
    source_language: &str,
    target_language: &str,
    decorators: &DecoratorSet,
) -> Result<SelfTranslation, SelfTranslationError> {
    let from = required(source_language)?;
    let to = required(target_language)?;
    let reconstructed =
        LinkNetwork::parse(source, from, ParseConfiguration::default()).reconstruct_text();
    if reconstructed != source {
        return Err(SelfTranslationError {
            message: format!("the {from} links of the source do not reproduce it"),
        });
    }
    let items = top_level_items(source, from);
    if family(from) == family(to) {
        let items = items
            .iter()
            .map(|item| SelfTranslationItem {
                term: item.term.clone(),
                start: item.start,
                end: item.end,
                status: "kept",
                reason: None,
            })
            .collect();
        return Ok(SelfTranslation {
            source_language: from,
            target_language: to,
            code: reconstructed,
            items,
        });
    }
    let mut blocks: Vec<String> = Vec::new();
    let mut preludes: Vec<String> = Vec::new();
    let mut recorded = Vec::new();
    let mut gap = String::new();
    for (group, out) in
        translate_binding_groups(&group_items(&items, source), source, from, to, decorators)
    {
        for prelude in out.preludes {
            if !preludes.contains(&prelude) {
                preludes.push(prelude);
            }
        }
        for item in &group.items {
            recorded.push(SelfTranslationItem {
                term: item.term.clone(),
                start: item.start,
                end: item.end,
                status: out.status,
                reason: out.reason.clone(),
            });
        }
        if let Some(code) = out.code {
            let separator = if blocks.is_empty() {
                String::new()
            } else {
                breaks(&gap)
            };
            blocks.push(format!("{separator}{code}"));
            group.after.clone_into(&mut gap);
        } else if gap.is_empty() {
            // A dropped group keeps the layout before it.
            group.after.clone_into(&mut gap);
        }
    }
    let body = format!("{}\n", blocks.concat());
    // An unedited translation back gives the source its header describes.
    let header = items
        .iter()
        .find(|item| item.comment && item.text.starts_with(HEADER));
    if let Some(header) = header
        && header.text.contains(&format!(" source={to} "))
        && header.text.contains(&format!(" sha256={} ", sha256(&body)))
    {
        return Ok(SelfTranslation {
            source_language: from,
            target_language: to,
            code: body,
            items: recorded,
        });
    }
    if family(to) == "Rust" && recorded.iter().any(|item| item.status == "translated") {
        preludes.insert(0, RUST_ALLOW.to_owned());
    }
    let mut lines = vec![
        format!(
            "{HEADER}source={from} target={to} sha256={} bytes={}",
            sha256(source),
            source.len()
        ),
        String::new(),
    ];
    if !preludes.is_empty() {
        lines.push(PRELUDE_BEGIN.to_owned());
        for (index, prelude) in preludes.into_iter().enumerate() {
            if index > 0 {
                lines.push(String::new());
            }
            lines.push(prelude);
        }
        lines.push(PRELUDE_END.to_owned());
        lines.push(String::new());
    }
    Ok(SelfTranslation {
        source_language: from,
        target_language: to,
        code: format!("{}\n{body}", lines.join("\n")),
        items: recorded,
    })
}

// The layout between two emitted blocks: its line breaks, at least one.
fn breaks(gap: &str) -> String {
    "\n".repeat(gap.matches('\n').count().max(1))
}

/// The top-level items of `text`, with their text and the layout after each.
fn top_level_items<'a>(text: &'a str, language: &str) -> Vec<Item<'a>> {
    let bytes = text.as_bytes();
    let nodes: Vec<(String, usize, usize)> =
        crate::tree_sitter_adapter::top_level_nodes(text, language)
            .unwrap_or_default()
            .into_iter()
            .filter(|(term, _, _)| term != "whitespace")
            .map(|(term, start, mut end)| {
                // A Rust line comment ends with its line break, which is layout here.
                if COMMENTS.contains(&term.as_str()) && end > 0 && bytes[end - 1] == b'\n' {
                    end -= if end > 1 && bytes[end - 2] == b'\r' {
                        2
                    } else {
                        1
                    };
                }
                (term, start, end)
            })
            .collect();
    nodes
        .iter()
        .enumerate()
        .map(|(index, (term, start, end))| Item {
            term: term.clone(),
            start: *start,
            end: *end,
            comment: COMMENTS.contains(&term.as_str()),
            text: &text[*start..*end],
            after: &text[*end..nodes.get(index + 1).map_or(text.len(), |next| next.1)],
        })
        .collect()
}

fn line_break(layout: &str) -> bool {
    layout == "\n" || layout == "\r\n"
}

fn source_lines(items: &[Item<'_>]) -> Vec<String> {
    items
        .iter()
        .map(|item| {
            let rest = &item.text[SOURCE_LINE.len()..];
            rest.strip_prefix(' ').unwrap_or(rest).to_owned()
        })
        .collect()
}

fn is_marker(item: &Item<'_>) -> bool {
    item.comment
        && [HEADER, CARRIED, TRANSLATED, PRELUDE_BEGIN]
            .iter()
            .any(|marker| item.text.starts_with(marker))
}

#[derive(Clone, Debug)]
enum GroupKind {
    Provenance,
    Carried {
        language: String,
        lines: Vec<String>,
    },
    Comment {
        text: String,
        term: String,
    },
    Item {
        text: String,
        term: String,
    },
}

#[derive(Clone)]
struct Group<'a> {
    kind: GroupKind,
    items: Vec<Item<'a>>,
    after: &'a str,
}

/// Groups the items: a self-translation header, a prelude block, a carried or
/// translated item with its provenance, and an item with the comments directly
/// before it are one group each.
fn group_items<'a>(items: &[Item<'a>], text: &'a str) -> Vec<Group<'a>> {
    let mut groups = Vec::new();
    let between = |first: &Item<'_>, last: &Item<'_>| text[first.start..last.end].to_owned();
    // The `// |` source lines that follow the item at `at` line by line.
    let lines_after = |at: usize| {
        let mut end = at;
        while end + 1 < items.len()
            && items[end + 1].comment
            && items[end + 1].text.starts_with(SOURCE_LINE)
            && line_break(items[end].after)
        {
            end += 1;
        }
        end
    };
    let mut index = 0;
    while index < items.len() {
        let item = &items[index];
        let (end, kind) = if item.comment && item.text.starts_with(HEADER) {
            (index, GroupKind::Provenance)
        } else if item.comment && item.text == PRELUDE_BEGIN {
            let mut end = index;
            while end + 1 < items.len() && items[end].text != PRELUDE_END {
                end += 1;
            }
            (end, GroupKind::Provenance)
        } else if item.comment && item.text.starts_with(CARRIED) {
            let end = lines_after(index);
            let language = first_word(&item.text[CARRIED.len()..]);
            let lines = source_lines(&items[index + 1..=end]);
            (end, GroupKind::Carried { language, lines })
        } else if item.comment && item.text.starts_with(TRANSLATED) {
            let rest = &item.text[TRANSLATED.len()..];
            let field = |name: &str| {
                rest.split(' ').skip(2).find_map(|field| {
                    let (key, value) = field.split_once('=').unwrap_or((field, ""));
                    (key == name).then_some(value)
                })
            };
            let lines_end = lines_after(index);
            let count = field("items").and_then(|count| count.parse::<usize>().ok());
            match count {
                Some(count) if count > 0 && lines_end + count < items.len() => {
                    let last = lines_end + count;
                    let code = between(&items[lines_end + 1], &items[last]);
                    if field("sha256") == Some(sha256(&code).as_str()) {
                        let lines = source_lines(&items[index + 1..=lines_end]);
                        (
                            last,
                            GroupKind::Carried {
                                language: first_word(rest),
                                lines,
                            },
                        )
                    } else {
                        // An edited translation is translated again; its provenance is dropped.
                        (lines_end, GroupKind::Provenance)
                    }
                }
                _ => (
                    index,
                    GroupKind::Comment {
                        text: item.text.to_owned(),
                        term: item.term.clone(),
                    },
                ),
            }
        } else {
            // Comments directly before an item document it and travel with it.
            let mut end = index;
            while items[end].comment
                && end + 1 < items.len()
                && line_break(items[end].after)
                && !is_marker(&items[end + 1])
            {
                end += 1;
            }
            let text = between(item, &items[end]);
            let term = items[end].term.clone();
            // A run of comments no item follows is one comment group.
            if items[end].comment {
                (end, GroupKind::Comment { text, term })
            } else {
                (end, GroupKind::Item { text, term })
            }
        };
        groups.push(Group {
            kind,
            items: items[index..=end].to_vec(),
            after: items[end].after,
        });
        index = end + 1;
    }
    groups
}

fn first_word(text: &str) -> String {
    text.split(' ').next().unwrap_or_default().to_owned()
}

struct Translated {
    code: Option<String>,
    preludes: Vec<String>,
    status: &'static str,
    reason: Option<String>,
}

// Only the collection/provenance adapter is host code; run boundaries and
// retry eligibility use the same JavaScript-generated frontend decisions.
fn translate_binding_groups<'a>(
    groups: &[Group<'a>],
    source: &'a str,
    from: &str,
    to: &str,
    decorators: &DecoratorSet,
) -> Vec<(Group<'a>, Translated)> {
    let bind = family(from) == "JavaScript" && family(to) == "Rust";
    let (literals, literal_groups) = collect_literal_bindings(groups, bind, from, to, decorators);
    let terms: Vec<String> = groups
        .iter()
        .enumerate()
        .map(|(index, group)| match &group.kind {
            GroupKind::Item { term, .. } if !literal_groups.contains(&index) => term.clone(),
            _ => String::new(),
        })
        .collect();
    let mut results = Vec::new();
    let mut index = 0;
    while index < groups.len() {
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss
        )]
        let end = if bind {
            find_binding_run_end(&terms, index as f64) as usize
        } else {
            index + 1
        };
        let run = &groups[index..end];
        let isolated: Vec<_> = run
            .iter()
            .map(|group| {
                (
                    group.clone(),
                    translate_group(group, from, to, decorators, &literals),
                )
            })
            .collect();
        let statuses: Vec<String> = isolated
            .iter()
            .map(|(_, out)| out.status.to_owned())
            .collect();
        let reasons: Vec<String> = isolated
            .iter()
            .map(|(_, out)| out.reason.clone().unwrap_or_default())
            .collect();
        if accept_binding_scope(&statuses, &reasons) {
            let items: Vec<_> = run.iter().flat_map(|group| group.items.clone()).collect();
            let first = items.first().expect("a binding run has source items");
            let last = items.last().expect("a binding run has source items");
            let GroupKind::Item { term, .. } = &run[0].kind else {
                unreachable!("only item runs can combine")
            };
            let combined = Group {
                kind: GroupKind::Item {
                    text: source[first.start..last.end].to_owned(),
                    term: term.clone(),
                },
                items,
                after: run.last().expect("a binding run is nonempty").after,
            };
            let out = translate_group(&combined, from, to, decorators, &literals);
            if out.status == "translated" {
                results.push((combined, out));
            } else {
                results.extend(isolated);
            }
        } else {
            results.extend(isolated);
        }
        index = end;
    }
    let module_terms: Vec<String> = groups
        .iter()
        .map(|group| match &group.kind {
            GroupKind::Item { term, .. } => term.clone(),
            GroupKind::Comment { .. } => String::new(),
            _ => "blocked".to_owned(),
        })
        .collect();
    if bind
        && accept_module_binding_scope(
            &module_terms,
            results.iter().any(|(_, out)| out.status == "carried"),
        )
    {
        let items: Vec<_> = groups
            .iter()
            .flat_map(|group| group.items.clone())
            .collect();
        let first = items.first().expect("a declaration module has items");
        let last = items.last().expect("a declaration module has items");
        let term = match &groups[0].kind {
            GroupKind::Item { term, .. } | GroupKind::Comment { term, .. } => term.clone(),
            _ => unreachable!("provenance and carried groups are not module declarations"),
        };
        let combined = Group {
            kind: GroupKind::Item {
                text: source[first.start..last.end].to_owned(),
                term,
            },
            items,
            after: groups
                .last()
                .expect("a declaration module is nonempty")
                .after,
        };
        let out = translate_group(&combined, from, to, decorators, &literals);
        if out.status == "translated" {
            return vec![(combined, out)];
        }
    }
    results
}

fn collect_literal_bindings(
    groups: &[Group<'_>],
    bind: bool,
    from: &str,
    to: &str,
    decorators: &DecoratorSet,
) -> (HashMap<String, SExpr>, HashSet<usize>) {
    let mut literals = HashMap::new();
    let mut literal_groups = HashSet::new();
    if bind {
        for (index, group) in groups.iter().enumerate() {
            let GroupKind::Item { text, .. } = &group.kind else {
                continue;
            };
            let Ok(parsed) = parse_javascript(text) else {
                continue;
            };
            let Some(main) = &parsed.main else {
                continue;
            };
            let Some(SEffect::Let {
                name,
                value,
                constant,
                ..
            }) = main.effects.first()
            else {
                continue;
            };
            let kind = match &value.node {
                SNode::Num { .. } => "num",
                SNode::Bool { .. } => "bool",
                SNode::Str { .. } => "str",
                _ => "",
            };
            #[allow(clippy::cast_precision_loss)]
            if accept_literal_binding(
                kind,
                *constant,
                main.effects.len() as f64,
                parsed.items.len() as f64,
            ) {
                if translate_group(group, from, to, decorators, &HashMap::new()).status
                    != "translated"
                {
                    continue;
                }
                literals.insert(name.clone(), value.clone());
                literal_groups.insert(index);
            }
        }
    }
    (literals, literal_groups)
}

fn translate_group(
    group: &Group<'_>,
    from: &str,
    to: &str,
    decorators: &DecoratorSet,
    literal_bindings: &HashMap<String, SExpr>,
) -> Translated {
    let done = |code: String, status: &'static str| Translated {
        code: Some(code),
        preludes: Vec::new(),
        status,
        reason: None,
    };
    let (text, term) = match &group.kind {
        GroupKind::Provenance => {
            return Translated {
                code: None,
                preludes: Vec::new(),
                status: "provenance",
                reason: None,
            };
        }
        GroupKind::Carried { language, lines } => {
            if family(language) == family(to) {
                return done(lines.join("\n"), "restored");
            }
            let mut code = vec![group.items[0].text.to_owned()];
            code.extend(lines.iter().map(|line| source_line(line)));
            return Translated {
                reason: Some("carried from another language".to_owned()),
                ..done(code.join("\n"), "carried")
            };
        }
        GroupKind::Comment { text, term } => {
            // A copied comment has no provenance, so it must also fit the source.
            if group
                .items
                .iter()
                .all(|item| comment_fits(item.text, from) && comment_fits(item.text, to))
            {
                return done(text.clone(), "comment");
            }
            return carry(text, term, from, "comment the target cannot hold");
        }
        GroupKind::Item { text, term } => (text, term),
    };
    let emitted = match emit_item(text, from, to, literal_bindings) {
        Ok(Some(emitted)) => emitted,
        Ok(None) => return carry(text, term, from, "top-level statement"),
        Err(error) => return carry(text, term, from, error.kind.as_str()),
    };
    if emitted.definitions.is_empty() {
        return carry(text, term, from, "no definition");
    }
    let exported = family(to) == "JavaScript"
        && if family(from) == "Rust" {
            RUST_EXPORT.is_match(text)
        } else {
            JAVASCRIPT_EXPORT.is_match(text)
        };
    let generic = emitted
        .definitions
        .iter()
        .map(|definition| {
            if exported {
                format!("export {definition}")
            } else {
                definition.clone()
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n");
    let code = decorate_emitted(to, &generic, decorators);
    if code.trim().is_empty() {
        return carry(text, term, from, "dropped by a decorator");
    }
    // Count the CST children the provenance reader consumes, including
    // attributes and documentation that accompany a declaration.
    let count = top_level_items(&code, to).len();
    if count == 0 {
        return carry(text, term, from, "no target item");
    }
    let marker = format!(
        "{TRANSLATED}{from} {term} items={} sha256={}",
        count,
        sha256(&code)
    );
    let mut lines = vec![marker];
    lines.extend(LINE_BREAK.split(text).map(source_line));
    lines.push(code);
    Translated {
        code: Some(lines.join("\n")),
        preludes: emitted.preludes,
        status: "translated",
        reason: None,
    }
}

/// The item emitted by the portable core, or `None` for a top-level statement.
fn emit_item(
    text: &str,
    from: &str,
    to: &str,
    literal_bindings: &HashMap<String, SExpr>,
) -> Result<Option<Emitted>, TranslationError> {
    let surface = if family(from) == "Rust" {
        // The Rust frontend reads a program, so an item alone gets an empty main.
        if RUST_MAIN.is_match(text) {
            parse_rust(text)?
        } else {
            parse_rust(&format!("{text}\nfn main() {{}}\n"))?
        }
    } else {
        parse_javascript_bound(text, literal_bindings)?
    };
    let program = check_program(&surface)?;
    if family(to) == "Rust"
        && let Some(emitted) = emit_rust_constants(&program)?
    {
        return Ok(Some(emitted));
    }
    if program
        .main
        .as_ref()
        .is_some_and(|main| !main.effects.is_empty())
    {
        return Ok(None);
    }
    let emitted = if family(to) == "Rust" {
        emit_rust(&program)?
    } else {
        emit_javascript(&program)?
    };
    Ok(Some(emitted))
}

fn carry(text: &str, term: &str, from: &str, reason: &str) -> Translated {
    let mut lines = vec![format!("{CARRIED}{from} {term} ({reason})")];
    lines.extend(LINE_BREAK.split(text).map(source_line));
    Translated {
        code: Some(lines.join("\n")),
        preludes: Vec::new(),
        status: "carried",
        reason: Some(reason.to_owned()),
    }
}

fn source_line(line: &str) -> String {
    if line.is_empty() {
        SOURCE_LINE.to_owned()
    } else {
        format!("{SOURCE_LINE} {line}")
    }
}

// A comment keeps its text when the target reads it as an ordinary comment:
// a Rust doc comment needs an item after it, and Rust block comments nest.
fn comment_fits(text: &str, to: &str) -> bool {
    if text.starts_with("/*") {
        let inner = text
            .get(2..text.len().saturating_sub(2))
            .unwrap_or_default();
        if inner.contains("/*") || inner.contains("*/") {
            return false;
        }
    }
    family(to) != "Rust"
        || !["///", "//!", "/**", "/*!"]
            .iter()
            .any(|prefix| text.starts_with(prefix))
}
