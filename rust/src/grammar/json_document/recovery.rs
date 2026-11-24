use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

use super::super::feature_runtime::GrammarSearchOptions;
use super::document::{Document, Node, span};
use super::{JsonDocumentIssue, JsonSchemaIssue, parser, whitespace};
use crate::{ByteRange, LinkId, LinkNetwork};

/// An explicit JSON recovery policy choice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JsonRecoveryAction {
    /// Close containers or strings whose closing delimiter is missing at EOF.
    CloseOpenContainers,
    /// Remove a comma before a closing delimiter if a member is missing.
    DropTrailingCommas,
    /// Strip a surrounding JSON or unlabeled Markdown code fence.
    StripCodeFence,
    /// Keep the first complete JSON value in a sequence of values.
    ExtractFirstValue,
    /// Remove trailing non-JSON text after the first complete value.
    StripTrailingGarbage,
    /// Escape an invalid control character or backslash inside a string.
    RepairEscapes,
}

/// The original source interval and the replacement from one permitted repair.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JsonRecoveryEdit {
    /// The authorized action.
    pub kind: JsonRecoveryAction,
    /// The original half-open UTF-8 byte range.
    pub span: ByteRange,
    /// Text inserted in place of the original bytes.
    pub inserted: String,
    /// Exact text removed from the source.
    pub removed: String,
    /// The missing or error link authorizing this edit.
    pub link_id: LinkId,
}

/// The result of applying only the selected repairs.
pub struct JsonRecovery {
    /// The repaired text.
    pub text: String,
    /// The complete value in the repaired text, when it can be decoded.
    pub value: Option<Value>,
    /// Edits in ascending original source order.
    pub edits: Vec<JsonRecoveryEdit>,
    /// True when the result parses without an EOF closure.
    pub complete: bool,
    /// Document diagnostics and degenerate repetitions.
    pub issues: Vec<JsonDocumentIssue>,
    /// Unsupported or malformed schema constraints.
    pub schema_issues: Vec<JsonSchemaIssue>,
    /// Concrete syntax links for the repaired output.
    pub network: LinkNetwork,
    /// Original syntax links, including anchors for each edit.
    pub source_network: LinkNetwork,
}

fn delimiters(kind: &str) -> Option<(&'static str, &'static str)> {
    match kind {
        "array" => Some(("[", "]")),
        "object" => Some(("{", "}")),
        "string" => Some(("\"", "\"")),
        _ => None,
    }
}

fn covers_errors<'node, 'source>(
    errors: &'node [&'node Node<'source>],
    start: usize,
    end: usize,
) -> Option<LinkId> {
    let mut matching = errors.iter().copied().filter(|node| node.end > start);
    let first = matching.next()?;
    let mut covered = start;
    for node in std::iter::once(first).chain(matching) {
        if node.start > covered {
            return None;
        }
        covered = covered.max(node.end);
        if covered >= end {
            return Some(first.link_id);
        }
    }
    None
}

fn only_json_values(source: &str) -> bool {
    let options = GrammarSearchOptions {
        rule: Some("value".to_owned()),
        word_boundaries: true,
        ..GrammarSearchOptions::default()
    };
    let matches = parser()
        .find_all(source, &options)
        .expect("registered JSON value search rule");
    let mut end = 0;
    for item in &matches {
        if !source[end..item.span.start].chars().all(whitespace) {
            return false;
        }
        end = item.span.end;
    }
    !matches.is_empty() && source[end..].chars().all(whitespace)
}

struct RecoveryEdits<'source, 'document> {
    document: &'document Document<'source>,
    edits: Vec<JsonRecoveryEdit>,
}

impl RecoveryEdits<'_, '_> {
    fn add(
        &mut self,
        kind: JsonRecoveryAction,
        start: usize,
        end: usize,
        inserted: &str,
        link_id: LinkId,
    ) {
        self.edits.push(JsonRecoveryEdit {
            kind,
            span: ByteRange::new(start, end),
            inserted: inserted.to_owned(),
            removed: self.document.source[start..end].to_owned(),
            link_id,
        });
    }

    fn fences(&mut self, errors: &[&Node<'_>]) {
        static OPEN: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r"\A[ \t\r\n]*(```(?:[Jj][Ss][Oo][Nn])?)[ \t]*\r?\n").unwrap()
        });
        static CLOSE: LazyLock<Regex> =
            LazyLock::new(|| Regex::new(r"\r?\n[ \t]*(```)[ \t\r\n]*\z").unwrap());
        let source = self.document.source;
        if let (Some(open), Some(close)) = (OPEN.captures(source), CLOSE.captures(source))
            && close.get(0).unwrap().start() >= open.get(0).unwrap().end()
        {
            let opening = open.get(1).unwrap();
            let closing = close.get(1).unwrap();
            if let (Some(first), Some(last)) = (
                covers_errors(errors, opening.start(), opening.end()),
                covers_errors(errors, closing.start(), closing.end()),
            ) {
                self.add(
                    JsonRecoveryAction::StripCodeFence,
                    opening.start(),
                    opening.end(),
                    "",
                    first,
                );
                self.add(
                    JsonRecoveryAction::StripCodeFence,
                    closing.start(),
                    closing.end(),
                    "",
                    last,
                );
            }
        }
    }

    fn trailing_commas(&mut self, nodes: &[&Node<'_>], close_at_end: bool) {
        for node in nodes {
            if !matches!(node.kind.as_str(), "array" | "object") {
                continue;
            }
            let mut significant = node.children.iter().rev().filter(|child| {
                child.start < child.end
                    && !child.flags.is_extra()
                    && !(child.flags.has_error() && child.text.chars().all(whitespace))
            });
            let closing = if close_at_end {
                node.children
                    .last()
                    .filter(|last| last.flags.is_missing() && last.end == self.document.root.end)
            } else {
                None
            }
            .or_else(|| significant.next());
            let (Some(comma), Some(closing)) = (significant.next(), closing) else {
                continue;
            };
            if comma.text != ","
                || delimiters(&node.kind).map(|(_, token)| token) != Some(closing.kind.as_str())
            {
                continue;
            }
            let anchor = node
                .children
                .iter()
                .find(|child| {
                    child.start >= comma.end
                        && child.end <= closing.start
                        && child.flags.has_error()
                })
                .or_else(|| closing.flags.has_error().then_some(closing));
            if let Some(anchor) = anchor {
                self.add(
                    JsonRecoveryAction::DropTrailingCommas,
                    comma.start,
                    comma.end,
                    "",
                    anchor.link_id,
                );
            }
        }
    }

    fn close_at_end(&mut self, nodes: &[&Node<'_>]) -> bool {
        let before = self.edits.len();
        for node in nodes {
            if let Some((open, close)) = delimiters(&node.kind)
                && let Some(closing) = node.children.last()
                && closing.flags.is_missing()
                && closing.kind == close
                && closing.end == self.document.root.end
                && node
                    .children
                    .iter()
                    .any(|child| child.text == open && !child.flags.has_error())
            {
                self.add(
                    JsonRecoveryAction::CloseOpenContainers,
                    closing.start,
                    closing.end,
                    close,
                    closing.link_id,
                );
            }
        }
        self.edits.len() != before
    }

    fn escapes(&mut self, nodes: &[&Node<'_>], errors: &[&Node<'_>]) {
        static ESCAPE: LazyLock<Regex> = LazyLock::new(|| {
            Regex::new(r#"\\(?:["\\/bfnrt]|u[0-9a-fA-F]{4})|\\|[\x00-\x1f]"#).unwrap()
        });
        for node in nodes.iter().copied().filter(|node| node.kind == "string") {
            for error in errors {
                let inside = error.start > node.start && error.end <= node.end;
                let marked = if inside {
                    error.text
                } else if node.children.last().is_some_and(|closing| {
                    closing.flags.is_missing() && error.start == closing.start
                }) && error
                    .text
                    .as_bytes()
                    .first()
                    .is_some_and(|byte| *byte <= 31)
                {
                    &error.text[..1]
                } else {
                    continue;
                };
                for found in ESCAPE.find_iter(marked).filter(|found| found.len() == 1) {
                    let quoted = serde_json::to_string(found.as_str()).expect("JSON character");
                    let start = error.start + found.start();
                    self.add(
                        JsonRecoveryAction::RepairEscapes,
                        start,
                        start + 1,
                        &quoted[1..quoted.len() - 1],
                        error.link_id,
                    );
                }
            }
        }
    }

    fn suffix(&mut self, allow: &[JsonRecoveryAction]) {
        if let Some(value) = self.document.value()
            && !value.flags.has_error()
            && self.edits.is_empty()
        {
            for node in &self.document.root.children {
                if !node.flags.is_error() || node.start < value.end {
                    continue;
                }
                let kind = if allow.contains(&JsonRecoveryAction::ExtractFirstValue)
                    && only_json_values(node.text)
                {
                    JsonRecoveryAction::ExtractFirstValue
                } else if allow.contains(&JsonRecoveryAction::StripTrailingGarbage) {
                    JsonRecoveryAction::StripTrailingGarbage
                } else {
                    continue;
                };
                self.add(kind, node.start, node.end, "", node.link_id);
            }
        }
    }

    fn apply(mut self) -> (String, Vec<JsonRecoveryEdit>) {
        self.edits.sort_by_key(|edit| edit.span.start());
        let mut cursor = 0;
        let mut text = String::with_capacity(self.document.source.len());
        self.edits.retain(|edit| {
            if edit.span.start() < cursor {
                return false;
            }
            text.push_str(&self.document.source[cursor..edit.span.start()]);
            text.push_str(&edit.inserted);
            cursor = edit.span.end();
            true
        });
        text.push_str(&self.document.source[cursor..]);
        (text, self.edits)
    }
}

fn repetitions(document: &Document<'_>, nodes: &[&Node<'_>]) -> Vec<JsonDocumentIssue> {
    let mut issues = Vec::new();
    for node in nodes
        .iter()
        .copied()
        .filter(|node| node.kind == "string" || node.flags.is_error())
    {
        if node.text.len() < 32 {
            continue;
        }
        let mut units = Vec::new();
        let mut offsets = Vec::new();
        for (offset, character) in node.text.char_indices() {
            let mut encoded = [0; 2];
            let code_units = character.encode_utf16(&mut encoded);
            units.extend_from_slice(code_units);
            offsets.push(Some(offset));
            if code_units.len() == 2 {
                offsets.push(None);
            }
        }
        offsets.push(Some(node.text.len()));
        let mut longest = None;
        let mut length = 31;
        for period in 1..=32.min(units.len() / 8) {
            let mut run = 0;
            for end in period..=units.len() {
                if units.get(end) == units.get(end - period) {
                    continue;
                }
                if end - run >= period * 8
                    && end - run > length
                    && let (Some(start), Some(last), Some(pattern_end)) =
                        (offsets[run], offsets[end], offsets[run + period])
                {
                    length = end - run;
                    longest = Some((start, last, pattern_end));
                }
                run = end - period + 1;
            }
        }
        if let Some((start, end, pattern_end)) = longest {
            let mut issue = node.issue("repetition", Vec::new());
            issue.pattern = Some(node.text[start..pattern_end].to_owned());
            issue.span = span(&document.lines, node.start + start, node.start + end);
            issues.push(issue);
        }
    }
    issues
}

/// Proposes and applies only enabled edits supported by concrete syntax recovery.
#[must_use]
pub fn recover_json(
    source: &str,
    schema: Option<&Value>,
    allow: &[JsonRecoveryAction],
) -> JsonRecovery {
    let document = Document::read(source);
    let nodes = document.nodes();
    let mut errors: Vec<_> = nodes
        .iter()
        .copied()
        .filter(|node| node.flags.is_error())
        .collect();
    errors.sort_by_key(|node| node.start);
    let mut repairs = RecoveryEdits {
        document: &document,
        edits: Vec::new(),
    };
    if allow.contains(&JsonRecoveryAction::StripCodeFence) {
        repairs.fences(&errors);
    }
    let close = allow.contains(&JsonRecoveryAction::CloseOpenContainers);
    if allow.contains(&JsonRecoveryAction::DropTrailingCommas) {
        repairs.trailing_commas(&nodes, close);
    }
    let truncated = close && repairs.close_at_end(&nodes);
    if allow.contains(&JsonRecoveryAction::RepairEscapes) {
        repairs.escapes(&nodes, &errors);
    }
    repairs.suffix(allow);
    let (text, edits) = repairs.apply();
    let mut issues = repetitions(&document, &nodes);
    let output = Document::read(&text);
    let syntax = output.syntax_issue();
    let complete = !truncated && syntax.is_none();
    let (value, diagnostics, schema_issues) = schema.map_or_else(
        || {
            let diagnostics: Vec<_> = syntax.into_iter().collect();
            let value = diagnostics
                .is_empty()
                .then(|| output.value().and_then(Node::read))
                .flatten();
            (value, diagnostics, Vec::new())
        },
        |schema| super::validation::validate(&output, schema),
    );
    issues.extend(diagnostics);
    JsonRecovery {
        value,
        edits,
        complete,
        issues,
        schema_issues,
        network: output.network,
        source_network: document.network,
        text,
    }
}
