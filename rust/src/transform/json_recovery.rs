//! Conservative JSON recovery over the native grammar's concrete syntax tree.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::decorators::{DecoratorLevel, DecoratorRecord, DecoratorSet};
use crate::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, compile_feature_grammar,
    parse_grammar_links,
};

/// An operation a caller may enable for JSON recovery.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum JsonRepairKind {
    /// Keep a partial value by closing literals and containers at the input boundary.
    CloseOpenContainers,
    /// Remove commas immediately before a closing delimiter or the end of an open container.
    DropTrailingCommas,
    /// Remove JSON Markdown fence lines.
    StripCodeFence,
    /// Keep the first value and remove surrounding prose or subsequent values.
    ExtractFirstValue,
    /// Remove a non-value suffix.
    StripTrailingGarbage,
    /// Preserve malformed escape spellings and raw control characters as string data.
    RepairEscapes,
}

/// The actions and optional schema used to recover a JSON value.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct JsonRecoveryOptions {
    /// Allowed repairs. Defaults to all six operations.
    pub allow: Vec<JsonRepairKind>,
    /// Schema used to report missing required properties.
    pub schema: Option<Value>,
}

impl Default for JsonRecoveryOptions {
    fn default() -> Self {
        Self {
            allow: vec![
                JsonRepairKind::CloseOpenContainers,
                JsonRepairKind::DropTrailingCommas,
                JsonRepairKind::StripCodeFence,
                JsonRepairKind::ExtractFirstValue,
                JsonRepairKind::StripTrailingGarbage,
                JsonRepairKind::RepairEscapes,
            ],
            schema: None,
        }
    }
}

/// A half-open range of original UTF-8 bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct JsonRecoverySpan {
    /// First byte of the range.
    pub start: usize,
    /// Byte immediately after the range.
    pub end: usize,
}

impl JsonRecoverySpan {
    const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// An edit witnessed by recovery nodes in the original concrete syntax tree.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct JsonRecoveryEdit {
    /// Operation that admitted the edit.
    pub kind: JsonRepairKind,
    /// Original bytes replaced, empty for insertion.
    pub span: JsonRecoverySpan,
    /// Replacement text.
    pub inserted: String,
    /// Exact original text removed.
    pub removed: String,
}

/// A diagnostic that accompanies the repair report.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum JsonRecoveryIssue {
    /// No permitted plan produced one valid JSON value.
    UnrecoveredSyntax {
        /// Original bytes at which recovery stopped.
        span: JsonRecoverySpan,
    },
    /// A long, short-period repetition in the original source.
    DegenerateRepetition {
        /// Original bytes occupied by the selected run.
        span: JsonRecoverySpan,
    },
    /// The schema calls for a property absent from the parsed object.
    MissingRequiredProperty {
        /// JSON Pointer of the absent property.
        pointer: String,
        /// Its original property name from the schema.
        property: String,
    },
}

/// Recovered source and its parsed value, with a replayable edit report.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct JsonRecoveryResult {
    /// Source after applying every reported edit, or the original on refusal.
    pub text: String,
    /// Parsed value, including `Some(Value::Null)` for a valid JSON null.
    pub value: Option<Value>,
    /// Edits in original source order. Insertions at one offset appear in application order.
    pub edits: Vec<JsonRecoveryEdit>,
    /// Whether a value was recovered without a truncation or missing required properties.
    pub complete: bool,
    /// Unrecovered syntax, missing required properties or possible degenerate repetition.
    pub issues: Vec<JsonRecoveryIssue>,
}

fn parse_recovery_tree(
    text: &str,
    start_rule: &str,
    offset: usize,
) -> Result<SyntaxTree, JsonRecoverySpan> {
    static PARSER: OnceLock<FeatureGrammarParser> = OnceLock::new();
    let parser = PARSER.get_or_init(|| {
        let grammar = parse_grammar_links(include_str!("../data/json-recovery-grammar.lino"))
            .expect("shipped JSON recovery grammar");
        compile_feature_grammar(&grammar, None, FeatureParseOptions::default())
            .expect("compiled JSON recovery grammar")
    });
    let outcome = parser
        .parse_tree(
            text.as_bytes(),
            &FeatureParseOptions {
                start_rule: Some(start_rule.to_owned()),
                accept_recovery: Some(true),
                // Match the rule nesting bound used by the JavaScript recovery executor.
                max_depth: Some(192),
                ..FeatureParseOptions::default()
            },
        )
        .expect("registered JSON recovery start rule");
    if let Some(rejection) = outcome.rejection {
        let position = offset + rejection.offset.unwrap_or(0);
        let end = if rejection.reason == "syntax" {
            position
        } else {
            offset + text.len()
        };
        return Err(JsonRecoverySpan::new(position, end));
    }
    Ok(outcome.tree.expect("successful JSON recovery tree"))
}

const fn is_json_space(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\n' | '\r')
}

fn starts_value(text: &str) -> bool {
    let text = text.trim_start_matches(is_json_space);
    matches!(
        text.as_bytes().first().copied(),
        Some(b'[' | b'{' | b'"' | b'-' | b'0'..=b'9')
    ) || ["true", "false", "null"].iter().any(|prefix| {
        text.strip_prefix(prefix).is_some_and(|rest| {
            rest.as_bytes()
                .first()
                .is_none_or(|byte| !byte.is_ascii_alphanumeric() && *byte != b'_')
        })
    })
}

fn fence_marker(text: &str) -> Option<(u8, usize)> {
    let marker = *text.as_bytes().first()?;
    if !matches!(marker, b'`' | b'~') {
        return None;
    }
    let count = text.bytes().take_while(|&byte| byte == marker).count();
    (count >= 3).then_some((marker, count))
}

fn repetition_issue(text: &str) -> Option<JsonRecoveryIssue> {
    const MINIMUM_LENGTH: usize = 256;
    let bytes = text.as_bytes();
    if bytes.len() < MINIMUM_LENGTH {
        return None;
    }
    let mut best: Option<JsonRecoverySpan> = None;
    for period in 1..=32 {
        let minimum = MINIMUM_LENGTH.max(period * 8);
        let mut start = 0;
        for end in period..=bytes.len() {
            if end < bytes.len() && bytes[end] == bytes[end - period] {
                continue;
            }
            if end - start >= minimum {
                let mut from = start;
                let mut to = end;
                while !text.is_char_boundary(from) {
                    from += 1;
                }
                while !text.is_char_boundary(to) {
                    to -= 1;
                }
                let length = to - from;
                if length >= minimum
                    && best.is_none_or(|span| {
                        length > span.end - span.start
                            || (length == span.end - span.start && from < span.start)
                    })
                    && bytes[from..from + period]
                        .iter()
                        .any(|&byte| !is_json_space(char::from(byte)))
                {
                    best = Some(JsonRecoverySpan::new(from, to));
                }
            }
            start = end - period + 1;
        }
    }
    best.map(|span| JsonRecoveryIssue::DegenerateRepetition { span })
}

fn pointer_part(key: &str) -> String {
    key.replace('~', "~0").replace('/', "~1")
}

fn missing_required_properties(value: &Value, schema: &Value, issues: &mut Vec<JsonRecoveryIssue>) {
    let mut pending = vec![(value, schema, String::new())];
    while let Some((item, rule, pointer)) = pending.pop() {
        match item {
            Value::Array(items) => {
                let prefixes = rule["prefixItems"].as_array();
                for (index, item) in items.iter().enumerate().rev() {
                    let child = prefixes
                        .and_then(|prefix| prefix.get(index))
                        .unwrap_or_else(|| &rule["items"]);
                    if child.is_object() {
                        pending.push((item, child, format!("{pointer}/{index}")));
                    }
                }
            }
            Value::Object(object) => {
                if let Some(required) = rule["required"].as_array() {
                    let mut reported = std::collections::BTreeSet::new();
                    for key in required.iter().filter_map(Value::as_str) {
                        if !object.contains_key(key) && reported.insert(key) {
                            issues.push(JsonRecoveryIssue::MissingRequiredProperty {
                                pointer: format!("{pointer}/{}", pointer_part(key)),
                                property: key.to_owned(),
                            });
                        }
                    }
                }
                if let Some(properties) = rule["properties"].as_object() {
                    let mut applicable: Vec<_> = properties
                        .iter()
                        .filter_map(|(key, child)| object.get(key).map(|item| (key, item, child)))
                        .collect();
                    applicable.sort_unstable_by_key(|(key, _, _)| std::cmp::Reverse(*key));
                    for (key, item, child) in applicable {
                        pending.push((item, child, format!("{pointer}/{}", pointer_part(key))));
                    }
                }
            }
            _ => {}
        }
    }
}

fn kind_for_name(name: &str) -> Option<JsonRepairKind> {
    Some(match name {
        "closeOpenContainers" => JsonRepairKind::CloseOpenContainers,
        "dropTrailingCommas" => JsonRepairKind::DropTrailingCommas,
        "stripCodeFence" => JsonRepairKind::StripCodeFence,
        "extractFirstValue" => JsonRepairKind::ExtractFirstValue,
        "stripTrailingGarbage" => JsonRepairKind::StripTrailingGarbage,
        "repairEscapes" => JsonRepairKind::RepairEscapes,
        _ => return None,
    })
}

fn collect_edits(
    tree: &SyntaxTree,
    text: &str,
    offset: usize,
    allow: &[JsonRepairKind],
    edits: &mut Vec<JsonRecoveryEdit>,
    from_prose: bool,
) -> Result<(), JsonRecoverySpan> {
    static DECORATORS: OnceLock<DecoratorSet> = OnceLock::new();
    let decorators = DECORATORS.get_or_init(|| {
        DecoratorSet::from_lino(include_str!("../data/json-recovery-decorators.lino"))
            .expect("shipped JSON recovery decorators")
    });
    let mut pending = vec![(tree, "")];
    while let Some((node, region)) = pending.pop() {
        let node_type = match node {
            SyntaxTree::Node { kind, children, .. } => {
                let mut region = kind.as_str();
                if region == "trailing_region"
                    && (from_prose
                        || !allow.contains(&JsonRepairKind::StripTrailingGarbage)
                        || starts_value(&text[offset + node.start()..offset + node.end()]))
                {
                    region = "following_value";
                }
                pending.extend(children.iter().rev().map(|child| (child, region)));
                continue;
            }
            SyntaxTree::Error { .. } => "error",
            SyntaxTree::Missing { .. } => "missing",
            _ => continue,
        };
        let span = JsonRecoverySpan::new(offset + node.start(), offset + node.end());
        let record = DecoratorRecord::from([
            ("type".to_owned(), node_type.to_owned()),
            ("region".to_owned(), region.to_owned()),
        ]);
        let decorated = decorators
            .decorate(DecoratorLevel::Transformation, &record)
            .ok_or(span)?;
        let kind = decorated
            .get("kind")
            .and_then(|name| kind_for_name(name))
            .ok_or(span)?;
        if !allow.contains(&kind) {
            return Err(span);
        }
        let removed = &text[span.start..span.end];
        let inserted = match decorated.get("action").map(String::as_str) {
            Some("insert") => match node {
                SyntaxTree::Missing {
                    kind: Some(kind), ..
                } => kind.clone(),
                _ => return Err(span),
            },
            Some("escapeBackslash") => r"\\".to_owned(),
            Some("escapeControl") => format!(r"\u00{:02x}", text.as_bytes()[span.start]),
            Some("remove") => String::new(),
            _ => return Err(span),
        };
        if matches!(
            kind,
            JsonRepairKind::StripCodeFence | JsonRepairKind::ExtractFirstValue
        ) && let Some(previous) = edits.last_mut()
            && previous.kind == kind
            && previous.span.end == span.start
        {
            previous.span.end = span.end;
            previous.removed.push_str(removed);
        } else {
            edits.push(JsonRecoveryEdit {
                kind,
                span,
                inserted,
                removed: removed.to_owned(),
            });
        }
    }
    Ok(())
}

fn children(tree: &SyntaxTree) -> impl Iterator<Item = &SyntaxTree> {
    let children = match tree {
        SyntaxTree::Node { children, .. } => children.as_slice(),
        _ => &[],
    };
    children.iter()
}

fn node_named(tree: &SyntaxTree, name: &str) -> bool {
    matches!(tree, SyntaxTree::Node { kind, .. } if kind == name)
}

fn plan_fenced_recovery(
    text: &str,
    frame: &SyntaxTree,
    allow: &[JsonRepairKind],
    edits: &mut Vec<JsonRecoveryEdit>,
    from_prose: bool,
) -> Result<(), JsonRecoverySpan> {
    let fenced = children(frame)
        .find(|node| node_named(node, "fenced_region"))
        .expect("fence frame has a content region");
    let mut parts = children(fenced).filter(|node| matches!(node, SyntaxTree::Node { .. }));
    let opening = parts.next().expect("opening fence");
    let body = parts.next().expect("fenced body");
    let closing = parts.next();
    let tail = parts.next();
    if let Some(closing) = closing {
        let (opener, opening_count) =
            fence_marker(&text[opening.start()..opening.end()]).expect("validated opening marker");
        let (closer, closing_count) =
            fence_marker(text[closing.start()..closing.end()].trim_start_matches(is_json_space))
                .expect("validated closing marker");
        if opener != closer || closing_count < opening_count {
            return Err(JsonRecoverySpan::new(closing.start(), closing.end()));
        }
    }
    for prefix in children(frame)
        .take_while(|node| !node_named(node, "fenced_region"))
        .chain(std::iter::once(opening))
    {
        collect_edits(prefix, text, 0, allow, edits, false)?;
    }
    let parsed = parse_recovery_tree(&text[body.start()..body.end()], "document", body.start())?;
    collect_edits(&parsed, text, body.start(), allow, edits, false)?;
    if let Some(closing) = closing {
        collect_edits(closing, text, 0, allow, edits, false)?;
    }
    if let Some(tail) = tail {
        collect_edits(tail, text, 0, allow, edits, from_prose)?;
    }
    Ok(())
}

fn plan_recovery(
    text: &str,
    allow: &[JsonRepairKind],
    edits: &mut Vec<JsonRecoveryEdit>,
) -> Result<(), JsonRecoverySpan> {
    if fence_marker(text.trim_start_matches(is_json_space)).is_some() {
        let frame = parse_recovery_tree(text, "fenced_document", 0)?;
        return plan_fenced_recovery(text, &frame, allow, edits, false);
    }
    let parsed = parse_recovery_tree(text, "document", 0);
    if parsed.is_ok() || !allow.contains(&JsonRepairKind::ExtractFirstValue) || starts_value(text) {
        return collect_edits(&parsed?, text, 0, allow, edits, false);
    }

    let parsed = parse_recovery_tree(text, "prose_document", 0);
    if text
        .split('\n')
        .skip(1)
        .any(|line| fence_marker(line.trim_start_matches([' ', '\t'])).is_some())
        && let Ok(frame) = parse_recovery_tree(text, "prose_fenced_document", 0)
    {
        let opening = children(&frame)
            .find(|node| node_named(node, "fenced_region"))
            .expect("fence frame has a content region");
        let prefix_end = parsed.as_ref().ok().and_then(|tree| {
            children(tree)
                .find(|node| node_named(node, "leading_region"))
                .map(SyntaxTree::end)
        });
        if prefix_end.is_none_or(|end| opening.start() < end) {
            return plan_fenced_recovery(text, &frame, allow, edits, true);
        }
    }
    collect_edits(&parsed?, text, 0, allow, edits, true)
}

fn apply_edits(text: &str, edits: &[JsonRecoveryEdit]) -> String {
    let mut output = String::with_capacity(text.len());
    let mut offset = 0;
    for edit in edits {
        output.push_str(&text[offset..edit.span.start]);
        output.push_str(&edit.inserted);
        offset = edit.span.end;
    }
    output.push_str(&text[offset..]);
    output
}

fn parse_value(text: &str) -> Option<Value> {
    let mut value: Value = serde_json::from_str(text).ok()?;
    let mut pending = vec![&mut value];
    while let Some(item) = pending.pop() {
        match item {
            Value::Object(properties) => pending.extend(properties.values_mut()),
            Value::Array(items) => pending.extend(items.iter_mut()),
            Value::Number(number)
                if number
                    .as_i64()
                    .is_some_and(|value| value.unsigned_abs() > 9_007_199_254_740_991)
                    || number
                        .as_u64()
                        .is_some_and(|value| value > 9_007_199_254_740_991) =>
            {
                *number = serde_json::Number::from_f64(number.as_f64()?)?;
            }
            _ => {}
        }
    }
    Some(value)
}

/// Recover one JSON value and report every edit in original UTF-8 byte coordinates.
#[must_use]
pub fn recover_json(text: &str, options: &JsonRecoveryOptions) -> JsonRecoveryResult {
    let mut report = JsonRecoveryResult {
        text: text.to_owned(),
        value: parse_value(text),
        edits: Vec::new(),
        complete: false,
        issues: repetition_issue(text).into_iter().collect(),
    };
    if report.value.is_none() {
        let recovered = plan_recovery(text, &options.allow, &mut report.edits).and_then(|()| {
            let source = apply_edits(text, &report.edits);
            let value = parse_value(&source).ok_or_else(|| JsonRecoverySpan::new(0, text.len()))?;
            Ok((source, value))
        });
        match recovered {
            Ok((source, value)) => {
                report.text = source;
                report.value = Some(value);
            }
            Err(span) => {
                report.edits.clear();
                report
                    .issues
                    .push(JsonRecoveryIssue::UnrecoveredSyntax { span });
                return report;
            }
        }
    }

    let issue_count = report.issues.len();
    if let Some((value, schema)) = report.value.as_ref().zip(options.schema.as_ref()) {
        missing_required_properties(value, schema, &mut report.issues);
    }
    report.complete = report.issues.len() == issue_count
        && !report
            .edits
            .iter()
            .any(|edit| edit.kind == JsonRepairKind::CloseOpenContainers);
    report
}
