//! The native links form of a grammar: Links Notation that both runtimes write
//! byte for byte the same and read back without the source the grammar came
//! from.
//!
//! A `(grammar (format TAG) (start NAME))` link is followed by one
//! `(rule NAME KIND EXPRESSION)` link per rule in grammar order, with an
//! optional trailing `(doc TEXT)`. Expressions are nested links headed by the
//! native listing's words: `empty`, `any`, `(literal TEXT)`,
//! `(literalInsensitive TEXT)`, `(range FROM TO)`, `(class plain|negated
//! ITEM...)` with `(char C)` and `(range FROM TO)` items, `(ref NAME)`,
//! `(choice ordered|unordered ITEM...)`, `(seq ITEM...)`, `(optional ITEM)`,
//! `(repeat0 ITEM)`, `(repeat1 ITEM)`, `(repeat MIN MAX|unbounded ITEM)`,
//! `(and ITEM)`, `(not ITEM)` and `(capture labeled LABEL ITEM)` or
//! `(capture unlabeled ITEM)`. The grammar feature union adds an optional
//! `(matching MODE)` header field, declaration links before the first rule,
//! rule fields before the doc and feature expressions (see the
//! `feature_links` module). The metadata `(kind NAME (source-names (SOURCE
//! NAME)...))` links after the declarations keep the source names of the
//! node kinds no rule defines, such as the kind an alias names. Names, texts and characters are percent-encoded
//! as [`LinkNetwork`](crate::LinkNetwork) `LiNo` terms are (ASCII letters,
//! digits and `-_.` kept, every other UTF-8 byte as `%XX`, the empty text as
//! `%`). It mirrors `js/src/grammar-links.js`.

use std::fmt::Write as _;

use links_notation::{LiNo, ParserConfig, parse_lino_to_links_with_config};

use super::super::{
    CharClassItem, FeatureExpr, Grammar, GrammarDeclarations, GrammarExpr, GrammarFormat,
    GrammarImportError, GrammarKind, GrammarRule, GrammarSourceName, MATCHING_MODES, RuleKind,
};
use super::feature_links::{
    read_class, read_declaration, read_feature, read_rule_fields, read_source_names,
    render_declaration_links, render_links_feature, render_rule_fields,
};

/// The longest repetition bound the links may spell, in decimal digits.
const MAX_BOUND_DIGITS: usize = 9;

pub(super) type Node = LiNo<String>;

pub(super) fn links_error(detail: impl AsRef<str>) -> GrammarImportError {
    GrammarImportError::Parse {
        format: GrammarFormat::MetaLanguage,
        message: format!("links: {}", detail.as_ref()),
    }
}

const fn is_plain(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')
}

/// Percent-encodes `value` as [`LinkNetwork`](crate::LinkNetwork) `LiNo` terms are.
#[must_use]
pub fn percent_encode_links_text(value: &str) -> String {
    if value.is_empty() {
        return "%".to_owned();
    }
    let mut encoded = String::with_capacity(value.len());
    for &byte in value.as_bytes() {
        if is_plain(byte) {
            encoded.push(char::from(byte));
        } else {
            write!(encoded, "%{byte:02X}").expect("writing to a String never fails");
        }
    }
    encoded
}

/// Reverses [`percent_encode_links_text`].
///
/// # Errors
///
/// Returns a [`GrammarImportError`] for a malformed escape, an unescaped
/// character or text that is not UTF-8.
pub fn percent_decode_links_text(value: &str) -> Result<String, GrammarImportError> {
    if value == "%" {
        return Ok(String::new());
    }
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'%' {
            let hex = bytes.get(index + 1..index + 3).filter(|hex| {
                hex.iter()
                    .all(|digit| digit.is_ascii_digit() || (b'A'..=b'F').contains(digit))
            });
            let Some(hex) = hex else {
                return Err(links_error(format!("invalid percent escape in {value}")));
            };
            let text = std::str::from_utf8(hex).expect("hex digits are ASCII");
            decoded.push(u8::from_str_radix(text, 16).expect("two hex digits fit a byte"));
            index += 3;
        } else if is_plain(byte) {
            decoded.push(byte);
            index += 1;
        } else {
            return Err(links_error(format!("unescaped character in {value}")));
        }
    }
    String::from_utf8(decoded).map_err(|_| links_error(format!("{value} is not UTF-8")))
}

/// Renders the native links form of `grammar`, one link per line.
#[must_use]
pub fn render_grammar_links(grammar: &Grammar) -> String {
    let mut header = vec!["grammar".to_owned()];
    if let Some(format) = grammar.source_format() {
        header.push(format!("(format {})", format.as_str()));
    }
    if let Some(start) = grammar.start_rule() {
        header.push(format!(
            "(start {})",
            percent_encode_links_text(start.name())
        ));
    }
    let declarations = grammar.declarations();
    if let Some(matching) = &declarations.matching {
        header.push(format!("(matching {matching})"));
    }
    let mut lines = vec![format!("({})", header.join(" "))];
    lines.extend(render_declaration_links(declarations));
    lines.extend(grammar.kinds().iter().map(|kind| {
        link(&[
            "kind".to_owned(),
            percent_encode_links_text(&kind.name),
            render_source_names(&kind.source_names),
        ])
    }));
    lines.extend(grammar.rules().iter().map(render_rule_link));
    lines.iter().fold(String::new(), |mut text, line| {
        text.push_str(line);
        text.push('\n');
        text
    })
}

/// Renders the `(rule NAME KIND EXPRESSION [FIELDS...] [(concept ID)]
/// [(source-names (SOURCE NAME)...)] [(doc TEXT)])` link of one rule.
#[must_use]
pub fn render_rule_link(rule: &GrammarRule) -> String {
    let mut parts = vec![
        "rule".to_owned(),
        percent_encode_links_text(rule.name()),
        rule.kind().as_str().to_owned(),
        render_links_expression(rule.expr()),
    ];
    parts.extend(render_rule_fields(&rule.attributes));
    if let Some(concept) = rule.concept() {
        parts.push(format!("(concept {})", percent_encode_links_text(concept)));
    }
    if !rule.source_names().is_empty() {
        parts.push(render_source_names(rule.source_names()));
    }
    if let Some(doc) = rule.doc() {
        parts.push(format!("(doc {})", percent_encode_links_text(doc)));
    }
    link(&parts)
}

/// Renders `(source-names (SOURCE NAME)...)`.
fn render_source_names(source_names: &[GrammarSourceName]) -> String {
    let mut names = vec!["source-names".to_owned()];
    names.extend(source_names.iter().map(|source_name| {
        format!(
            "({} {})",
            percent_encode_links_text(&source_name.source),
            percent_encode_links_text(&source_name.name)
        )
    }));
    link(&names)
}

pub(super) fn link(parts: &[String]) -> String {
    format!("({})", parts.join(" "))
}

pub(super) fn char_text(value: char) -> String {
    percent_encode_links_text(value.encode_utf8(&mut [0; 4]))
}

/// Renders one expression of the native links form.
#[must_use]
pub fn render_links_expression(expr: &GrammarExpr) -> String {
    let word = |value: &str| value.to_owned();
    let headed = |head: &str, items: &[GrammarExpr]| {
        let mut parts = vec![word(head)];
        parts.extend(items.iter().map(render_links_expression));
        link(&parts)
    };
    let unary =
        |head: &str, inner: &GrammarExpr| link(&[word(head), render_links_expression(inner)]);
    match expr {
        GrammarExpr::Empty => word("empty"),
        GrammarExpr::AnyChar => word("any"),
        GrammarExpr::Terminal(value) => link(&[word("literal"), percent_encode_links_text(value)]),
        GrammarExpr::TerminalInsensitive(value) => {
            link(&[word("literalInsensitive"), percent_encode_links_text(value)])
        }
        GrammarExpr::CharRange(start, end) => {
            link(&[word("range"), char_text(*start), char_text(*end)])
        }
        GrammarExpr::CharClass { negated, items } => {
            let mut parts = vec![
                word("class"),
                word(if *negated { "negated" } else { "plain" }),
            ];
            parts.extend(items.iter().map(|item| match item {
                CharClassItem::Char(value) => link(&[word("char"), char_text(*value)]),
                CharClassItem::Range(start, end) => {
                    link(&[word("range"), char_text(*start), char_text(*end)])
                }
            }));
            link(&parts)
        }
        GrammarExpr::NonTerminal(name) => link(&[word("ref"), percent_encode_links_text(name)]),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => {
            let mut parts = vec![
                word("choice"),
                word(if *ordered { "ordered" } else { "unordered" }),
            ];
            parts.extend(alternatives.iter().map(render_links_expression));
            link(&parts)
        }
        GrammarExpr::Sequence(items) => headed("seq", items),
        GrammarExpr::Optional(inner) => unary("optional", inner),
        GrammarExpr::ZeroOrMore(inner) => unary("repeat0", inner),
        GrammarExpr::OneOrMore(inner) => unary("repeat1", inner),
        GrammarExpr::And(inner) => unary("and", inner),
        GrammarExpr::Not(inner) => unary("not", inner),
        GrammarExpr::Repeat { expr, min, max } => link(&[
            word("repeat"),
            min.to_string(),
            max.map_or_else(|| word("unbounded"), |max| max.to_string()),
            render_links_expression(expr),
        ]),
        GrammarExpr::Capture { label, expr } => label.as_ref().map_or_else(
            || {
                link(&[
                    word("capture"),
                    word("unlabeled"),
                    render_links_expression(expr),
                ])
            },
            |label| {
                link(&[
                    word("capture"),
                    word("labeled"),
                    percent_encode_links_text(label),
                    render_links_expression(expr),
                ])
            },
        ),
        GrammarExpr::Feature(feature) => render_links_feature(feature),
    }
}

/// A word: a bare reference, or an identified link without values.
fn as_word(node: &Node) -> Option<&str> {
    match node {
        LiNo::Ref(word) => Some(word),
        LiNo::Link {
            id: Some(word),
            values,
        } if values.is_empty() => Some(word),
        LiNo::Link { .. } => None,
    }
}

/// A parsed value as a head and its arguments: a word is a head without
/// arguments.
pub(super) fn parts(node: &Node) -> Result<(&str, &[Node]), GrammarImportError> {
    if let Some(word) = as_word(node) {
        return Ok((word, &[]));
    }
    match node {
        LiNo::Link { id: None, values } => match values.split_first() {
            None => Err(links_error("empty link")),
            Some((head, rest)) => as_word(head)
                .map(|head| (head, rest))
                .ok_or_else(|| links_error("a link must start with a word")),
        },
        LiNo::Link { id: Some(id), .. } => {
            Err(links_error(format!("unexpected identified link {id}")))
        }
        LiNo::Ref(_) => unreachable!("a reference is a word"),
    }
}

pub(super) fn word<'a>(node: Option<&'a Node>, what: &str) -> Result<&'a str, GrammarImportError> {
    node.and_then(as_word)
        .ok_or_else(|| links_error(format!("expected {what}")))
}

pub(super) fn decoded_word(node: Option<&Node>, what: &str) -> Result<String, GrammarImportError> {
    percent_decode_links_text(word(node, what)?)
}

pub(super) fn character(node: Option<&Node>, what: &str) -> Result<char, GrammarImportError> {
    let decoded = decoded_word(node, what)?;
    let mut chars = decoded.chars();
    match (chars.next(), chars.next()) {
        (Some(value), None) => Ok(value),
        _ => Err(links_error(format!("{what} must be one code point"))),
    }
}

pub(super) fn arity(head: &str, args: &[Node], count: usize) -> Result<(), GrammarImportError> {
    if args.len() == count {
        Ok(())
    } else {
        Err(links_error(format!(
            "{head} takes {count} value(s), not {}",
            args.len()
        )))
    }
}

fn bound(node: &Node, allow_unbounded: bool) -> Result<Option<usize>, GrammarImportError> {
    let text = word(Some(node), "a repetition bound")?;
    if allow_unbounded && text == "unbounded" {
        return Ok(None);
    }
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(links_error(format!("invalid repetition bound {text}")));
    }
    if text.len() > MAX_BOUND_DIGITS {
        return Err(links_error(format!("repetition bound {text} is too large")));
    }
    Ok(Some(text.parse().expect("nine decimal digits fit usize")))
}

fn flag(node: &Node, yes: &str, no: &str) -> Result<bool, GrammarImportError> {
    let text = word(Some(node), &format!("{yes} or {no}"))?;
    if text != yes && text != no {
        return Err(links_error(format!("expected {yes} or {no}, not {text}")));
    }
    Ok(text == yes)
}

fn inner(head: &str, args: &[Node]) -> Result<Box<GrammarExpr>, GrammarImportError> {
    arity(head, args, 1)?;
    Ok(Box::new(parse_node(&args[0])?))
}

pub(super) fn parse_node(node: &Node) -> Result<GrammarExpr, GrammarImportError> {
    let (head, args) = parts(node)?;
    Ok(match head {
        "empty" | "any" => {
            arity(head, args, 0)?;
            if head == "empty" {
                GrammarExpr::Empty
            } else {
                GrammarExpr::AnyChar
            }
        }
        "literal" => {
            arity(head, args, 1)?;
            GrammarExpr::Terminal(decoded_word(args.first(), "a text")?)
        }
        "literalInsensitive" => {
            arity(head, args, 1)?;
            GrammarExpr::TerminalInsensitive(decoded_word(args.first(), "a text")?)
        }
        "range" => {
            arity(head, args, 2)?;
            GrammarExpr::CharRange(
                character(args.first(), "a range start")?,
                character(args.get(1), "a range end")?,
            )
        }
        "class" => {
            let Some((first, items)) = args.split_first() else {
                return Err(links_error("class needs plain or negated"));
            };
            read_class(flag(first, "negated", "plain")?, items)?
        }
        "ref" => {
            let Some((name, arguments)) = args.split_first() else {
                return Err(links_error("ref takes a rule name"));
            };
            let name = decoded_word(Some(name), "a rule name")?;
            if arguments.is_empty() {
                GrammarExpr::NonTerminal(name)
            } else {
                GrammarExpr::feature(FeatureExpr::Call {
                    name,
                    arguments: arguments.iter().map(parse_node).collect::<Result<_, _>>()?,
                })
            }
        }
        "choice" => {
            let Some((first, items)) = args.split_first() else {
                return Err(links_error("choice needs ordered or unordered"));
            };
            let ordered = flag(first, "ordered", "unordered")?;
            GrammarExpr::Choice {
                ordered,
                alternatives: items.iter().map(parse_node).collect::<Result<_, _>>()?,
            }
        }
        "seq" => GrammarExpr::Sequence(args.iter().map(parse_node).collect::<Result<_, _>>()?),
        "repeat" => {
            arity(head, args, 3)?;
            let min = bound(&args[0], false)?.expect("a lower bound is never unbounded");
            let max = bound(&args[1], true)?;
            if max.is_some_and(|max| max < min) {
                return Err(links_error(format!(
                    "repetition bounds {min}, {} are reversed",
                    max.unwrap_or_default()
                )));
            }
            GrammarExpr::Repeat {
                expr: Box::new(parse_node(&args[2])?),
                min,
                max,
            }
        }
        "capture" => {
            let Some(first) = args.first() else {
                return Err(links_error("capture needs labeled or unlabeled"));
            };
            if flag(first, "labeled", "unlabeled")? {
                arity(head, args, 3)?;
                GrammarExpr::Capture {
                    label: Some(decoded_word(args.get(1), "a capture label")?),
                    expr: Box::new(parse_node(&args[2])?),
                }
            } else {
                arity(head, args, 2)?;
                GrammarExpr::Capture {
                    label: None,
                    expr: Box::new(parse_node(&args[1])?),
                }
            }
        }
        "optional" => GrammarExpr::Optional(inner(head, args)?),
        "repeat0" => GrammarExpr::ZeroOrMore(inner(head, args)?),
        "repeat1" => GrammarExpr::OneOrMore(inner(head, args)?),
        "and" => GrammarExpr::And(inner(head, args)?),
        "not" => GrammarExpr::Not(inner(head, args)?),
        other => {
            return read_feature(other, args)?
                .ok_or_else(|| links_error(format!("unknown expression {other}")));
        }
    })
}

/// Reads one expression of the native links form.
///
/// # Errors
///
/// Returns a [`GrammarImportError`] of the meta-language format when the text
/// is not one well-formed expression link.
pub fn parse_links_expression(source: &str) -> Result<GrammarExpr, GrammarImportError> {
    let statements = parse_statements(source)?;
    match statements.as_slice() {
        [node] => parse_node(node),
        _ => Err(links_error("expected one expression")),
    }
}

fn parse_statements(source: &str) -> Result<Vec<Node>, GrammarImportError> {
    parse_lino_to_links_with_config(source, &ParserConfig::without_comments())
        .map_err(|error| links_error(error.to_string()))
}

/// Reads the native links form written by [`render_grammar_links`].
///
/// # Errors
///
/// Returns a [`GrammarImportError`] of the meta-language format when the links
/// are malformed, define a rule twice or no rule, or name an undefined start
/// rule.
pub fn parse_grammar_links(source: &str) -> Result<Grammar, GrammarImportError> {
    let statements = parse_statements(source)?;
    let Some((header, statements)) = statements.split_first() else {
        return Err(links_error("the links define no grammar"));
    };
    let (header_head, header_args) = parts(header)?;
    if header_head != "grammar" {
        return Err(links_error("the first link must be the grammar link"));
    }
    let mut source_format = None;
    let mut start = None;
    let mut declarations = GrammarDeclarations::default();
    for field in header_args {
        let (key, values) = parts(field)?;
        arity(key, values, 1)?;
        if key == "format" && source_format.is_none() {
            let tag = word(values.first(), "a format")?;
            source_format = Some(
                GrammarFormat::from_tag(tag)
                    .ok_or_else(|| links_error(format!("unknown source format {tag}")))?,
            );
        } else if key == "start" && start.is_none() {
            start = Some(decoded_word(values.first(), "a start rule")?);
        } else if key == "matching" && declarations.matching.is_none() {
            let matching = word(values.first(), "a matching")?;
            if !MATCHING_MODES.contains(&matching) {
                return Err(links_error(format!("unknown matching {matching}")));
            }
            declarations.matching = Some(matching.to_owned());
        } else {
            return Err(links_error(format!("unexpected grammar field {key}")));
        }
    }
    let mut rules: Vec<GrammarRule> = Vec::new();
    let mut kinds: Vec<GrammarKind> = Vec::new();
    for statement in statements {
        let (head, args) = parts(statement)?;
        if head == "kind" && rules.is_empty() {
            arity(head, args, 2)?;
            let name = decoded_word(args.first(), "a kind name")?;
            if kinds.iter().any(|kind| kind.name == name) {
                return Err(links_error(format!("kind {name} is given twice")));
            }
            let (field, names) = parts(&args[1])?;
            if field != "source-names" {
                return Err(links_error(format!(
                    "expected (source-names ...), not {field}"
                )));
            }
            let source_names = read_source_names(names)?;
            kinds.push(GrammarKind { name, source_names });
            continue;
        }
        if head != "rule" {
            if !rules.is_empty() || !read_declaration(head, args, &mut declarations)? {
                return Err(links_error(format!("unexpected link {head}")));
            }
            continue;
        }
        if args.len() < 3 {
            return Err(links_error(
                "rule takes a name, a kind, an expression and optional fields",
            ));
        }
        let name = decoded_word(args.first(), "a rule name")?;
        let kind = word(args.get(1), "a rule kind")?;
        let kind = RuleKind::from_tag(kind)
            .ok_or_else(|| links_error(format!("unknown rule kind {kind}")))?;
        if rules.iter().any(|rule| rule.name == name) {
            return Err(links_error(format!("rule {name} is defined twice")));
        }
        let expr = parse_node(&args[2])?;
        let fields = read_rule_fields(&args[3..])?;
        let mut rule = GrammarRule::new(name, expr)
            .with_kind(kind)
            .with_attributes(fields.attributes)
            .with_source_names(fields.source_names);
        if let Some(concept) = fields.concept {
            rule = rule.with_concept(concept);
        }
        if let Some(doc) = fields.doc {
            rule = rule.with_doc(doc);
        }
        rules.push(rule);
    }
    let Some(first) = rules.first().map(|rule| rule.name.clone()) else {
        return Err(links_error("the links define no rules"));
    };
    if let Some(start) = &start
        && !rules.iter().any(|rule| &rule.name == start)
    {
        return Err(links_error(format!("start rule {start} is not defined")));
    }
    let mut grammar = Grammar::new();
    for rule in rules {
        grammar.add_rule(rule);
    }
    grammar.set_start(start.unwrap_or(first));
    if let Some(format) = source_format {
        grammar.set_source_format(format);
    }
    grammar.set_declarations(declarations);
    grammar.set_kinds(kinds);
    Ok(grammar)
}
