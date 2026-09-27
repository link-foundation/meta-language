use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind};

use super::{ordered_rules, unsupported_error, EmitReport, GrammarEmitError};

const FORMAT: GrammarFormat = GrammarFormat::TreeSitter;
const EXTRAS_RULE: &str = "_extras";
const PRECEDENCE_TYPES: [(&str, &str); 4] = [
    ("prec=", "PREC"),
    ("prec_left=", "PREC_LEFT"),
    ("prec_right=", "PREC_RIGHT"),
    ("prec_dynamic=", "PREC_DYNAMIC"),
];
const MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;

/// Emits the declarative tree-sitter `grammar.json` form, the inverse of
/// [`crate::grammar::import_tree_sitter_json`].
///
/// Captures produced by the importer map back to their `FIELD`, `ALIAS`,
/// `PREC*`, `TOKEN`, `IMMEDIATE_TOKEN`, `RESERVED`, and `PATTERN` nodes. The
/// text matches JavaScript `JSON.stringify(document, null, 2)` so both runtimes
/// emit byte-identical documents.
pub fn emit_tree_sitter_json(grammar: &Grammar) -> Result<(String, EmitReport), GrammarEmitError> {
    let mut report = EmitReport::default();
    let mut rules = Vec::new();
    let mut inline = Vec::new();
    let mut extras = None;
    for rule in ordered_rules(grammar) {
        if rule.name() == EXTRAS_RULE && rule.kind() == RuleKind::Silent {
            extras = Some(extras_members(rule.expr(), &mut report)?);
            continue;
        }
        if rule.kind() == RuleKind::Silent {
            report.add_lossy(format!(
                "tree-sitter emits RuleKind::Silent rule {:?} in the inline list",
                rule.name()
            ));
            inline.push(Json::string(rule.name()));
        }
        rules.push((rule.name().to_string(), emit_rule(rule, &mut report)?));
    }

    let mut document = vec![
        ("name".to_string(), Json::string(&grammar_name(grammar))),
        ("rules".to_string(), Json::Object(rules)),
    ];
    if let Some(extras) = extras {
        document.push(("extras".to_string(), Json::Array(extras)));
    }
    if !inline.is_empty() {
        document.push(("inline".to_string(), Json::Array(inline)));
    }
    let mut output = String::new();
    Json::Object(document).write(&mut output, 0);
    output.push('\n');
    Ok((output, report))
}

fn emit_rule(rule: &GrammarRule, report: &mut EmitReport) -> Result<Json, GrammarEmitError> {
    if rule.kind() == RuleKind::Token {
        if let GrammarExpr::Capture {
            label: Some(label),
            expr,
        } = rule.expr()
        {
            if label == "immediate_token" {
                return Ok(node(
                    "IMMEDIATE_TOKEN",
                    vec![("content", emit_node(expr, report)?)],
                ));
            }
        }
    }
    if rule.kind() == RuleKind::Atomic {
        report.add_lossy(format!(
            "tree-sitter emits RuleKind::Atomic rule {:?} as a token",
            rule.name()
        ));
    }
    let body = emit_node(rule.expr(), report)?;
    Ok(match rule.kind() {
        RuleKind::Token | RuleKind::Atomic => node("TOKEN", vec![("content", body)]),
        RuleKind::Normal | RuleKind::Silent => body,
    })
}

fn extras_members(
    expr: &GrammarExpr,
    report: &mut EmitReport,
) -> Result<Vec<Json>, GrammarEmitError> {
    match expr {
        GrammarExpr::Choice {
            ordered: false,
            alternatives,
        } => emit_nodes(alternatives, report),
        expr => Ok(vec![emit_node(expr, report)?]),
    }
}

fn emit_nodes(
    items: &[GrammarExpr],
    report: &mut EmitReport,
) -> Result<Vec<Json>, GrammarEmitError> {
    items.iter().map(|item| emit_node(item, report)).collect()
}

fn emit_node(expr: &GrammarExpr, report: &mut EmitReport) -> Result<Json, GrammarEmitError> {
    Ok(match expr {
        GrammarExpr::Empty => blank(),
        GrammarExpr::Terminal(value) => node("STRING", vec![("value", Json::string(value))]),
        GrammarExpr::TerminalInsensitive(value) => {
            report.add_lossy(format!(
                "tree-sitter expands case-insensitive terminal {value:?} to a pattern"
            ));
            pattern(case_insensitive_pattern(value))
        }
        GrammarExpr::CharRange(start, end) => pattern(format!(
            "[{}-{}]",
            escape_class_char(*start),
            escape_class_char(*end)
        )),
        GrammarExpr::CharClass { negated, items } => pattern(char_class_pattern(*negated, items)?),
        GrammarExpr::AnyChar => pattern(".".to_string()),
        GrammarExpr::NonTerminal(name) => node("SYMBOL", vec![("name", Json::string(name))]),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => {
            if alternatives.is_empty() {
                return Err(unsupported_error(FORMAT, "empty Choice"));
            }
            if *ordered {
                report.add_lossy("tree-sitter treats ordered choice as unordered choice");
            }
            node(
                "CHOICE",
                vec![("members", Json::Array(emit_nodes(alternatives, report)?))],
            )
        }
        GrammarExpr::Sequence(items) => node(
            "SEQ",
            vec![("members", Json::Array(emit_nodes(items, report)?))],
        ),
        GrammarExpr::Optional(inner) => optional(emit_node(inner, report)?),
        GrammarExpr::ZeroOrMore(inner) => {
            node("REPEAT", vec![("content", emit_node(inner, report)?)])
        }
        GrammarExpr::OneOrMore(inner) => {
            node("REPEAT1", vec![("content", emit_node(inner, report)?)])
        }
        GrammarExpr::Repeat { expr, min, max } => emit_repeat(expr, *min, *max, report)?,
        GrammarExpr::And(_) | GrammarExpr::Not(_) => {
            return Err(unsupported_error(FORMAT, "predicate"));
        }
        GrammarExpr::Capture { label, expr } => emit_capture(label.as_deref(), expr, report)?,
    })
}

fn emit_repeat(
    expr: &GrammarExpr,
    min: usize,
    max: Option<usize>,
    report: &mut EmitReport,
) -> Result<Json, GrammarEmitError> {
    if let Some(max) = max {
        if max < min {
            return Err(unsupported_error(
                FORMAT,
                format!("Repeat with min {min} greater than max Some({max})"),
            ));
        }
    }
    report.add_lossy(format!(
        "tree-sitter desugared Repeat with min {min} and max {max:?}"
    ));
    let content = emit_node(expr, report)?;
    let mut members = vec![content.clone(); min];
    match max {
        None => members.push(node("REPEAT", vec![("content", content)])),
        Some(max) => {
            for _ in min..max {
                members.push(optional(content.clone()));
            }
        }
    }
    Ok(match members.len() {
        0 => blank(),
        1 => members.remove(0),
        _ => node("SEQ", vec![("members", Json::Array(members))]),
    })
}

fn emit_capture(
    label: Option<&str>,
    expr: &GrammarExpr,
    report: &mut EmitReport,
) -> Result<Json, GrammarEmitError> {
    let Some(label) = label else {
        report.add_lossy("tree-sitter dropped anonymous capture");
        return emit_node(expr, report);
    };
    if label == "regex" {
        if let GrammarExpr::Terminal(value) = expr {
            return Ok(pattern(value.clone()));
        }
    }
    for (prefix, kind) in PRECEDENCE_TYPES {
        if let Some(value) = label.strip_prefix(prefix) {
            return Ok(node(
                kind,
                vec![
                    ("value", precedence_value(value)),
                    ("content", emit_node(expr, report)?),
                ],
            ));
        }
    }
    if let Some(value) = label.strip_prefix("alias:") {
        return Ok(node(
            "ALIAS",
            vec![
                ("content", emit_node(expr, report)?),
                ("named", Json::Bool(is_identifier(value))),
                ("value", Json::string(value)),
            ],
        ));
    }
    let content = emit_node(expr, report)?;
    Ok(match label {
        "token" => node("TOKEN", vec![("content", content)]),
        "immediate_token" => node("IMMEDIATE_TOKEN", vec![("content", content)]),
        "reserved" => node("RESERVED", vec![("content", content)]),
        label => match label.strip_prefix("reserved:") {
            Some(context_name) => node(
                "RESERVED",
                vec![
                    ("context_name", Json::string(context_name)),
                    ("content", content),
                ],
            ),
            None => node(
                "FIELD",
                vec![("name", Json::string(label)), ("content", content)],
            ),
        },
    })
}

fn char_class_pattern(negated: bool, items: &[CharClassItem]) -> Result<String, GrammarEmitError> {
    if items.is_empty() {
        return Err(unsupported_error(FORMAT, "empty CharClass"));
    }
    let mut output = String::from("[");
    if negated {
        output.push('^');
    }
    for item in items {
        match item {
            CharClassItem::Char(value) => output.push_str(&escape_class_char(*value)),
            CharClassItem::Range(start, end) => {
                output.push_str(&escape_class_char(*start));
                output.push('-');
                output.push_str(&escape_class_char(*end));
            }
        }
    }
    output.push(']');
    Ok(output)
}

fn case_insensitive_pattern(value: &str) -> String {
    value
        .chars()
        .map(|character| {
            let lower = character.to_lowercase().collect::<Vec<_>>();
            let upper = character.to_uppercase().collect::<Vec<_>>();
            if lower == upper || lower.len() != 1 || upper.len() != 1 {
                escape_pattern_char(character)
            } else {
                format!(
                    "[{}{}]",
                    escape_class_char(lower[0]),
                    escape_class_char(upper[0])
                )
            }
        })
        .collect()
}

fn escape_class_char(character: char) -> String {
    match character {
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        '\\' | ']' | '[' | '-' | '^' => format!("\\{character}"),
        character => character.to_string(),
    }
}

fn escape_pattern_char(character: char) -> String {
    match character {
        '\n' => "\\n".to_string(),
        '\r' => "\\r".to_string(),
        '\t' => "\\t".to_string(),
        '\\' | '^' | '$' | '.' | '|' | '?' | '*' | '+' | '(' | ')' | '[' | ']' | '{' | '}'
        | '/' => format!("\\{character}"),
        character => character.to_string(),
    }
}

fn precedence_value(value: &str) -> Json {
    let digits = value.strip_prefix('-').unwrap_or(value);
    let canonical = digits == "0"
        || (digits.starts_with(|c: char| ('1'..='9').contains(&c))
            && digits.chars().all(|c| c.is_ascii_digit()));
    match value.parse::<i64>() {
        Ok(number) if canonical && number.abs() <= MAX_SAFE_INTEGER => Json::Number(number),
        _ => Json::string(value),
    }
}

fn grammar_name(grammar: &Grammar) -> String {
    let source = grammar
        .start_rule()
        .or_else(|| grammar.rules().first())
        .map_or("grammar", GrammarRule::name);
    let name = source
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();
    if name.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_') {
        name
    } else {
        format!("_{name}")
    }
}

fn is_identifier(value: &str) -> bool {
    value.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn node(kind: &str, fields: Vec<(&str, Json)>) -> Json {
    let mut entries = vec![("type".to_string(), Json::string(kind))];
    entries.extend(
        fields
            .into_iter()
            .map(|(key, value)| (key.to_string(), value)),
    );
    Json::Object(entries)
}

fn blank() -> Json {
    node("BLANK", Vec::new())
}

fn pattern(value: String) -> Json {
    node("PATTERN", vec![("value", Json::String(value))])
}

fn optional(content: Json) -> Json {
    node(
        "CHOICE",
        vec![("members", Json::Array(vec![content, blank()]))],
    )
}

/// Insertion-ordered JSON value rendered like `JSON.stringify(value, null, 2)`.
#[derive(Clone, Debug)]
enum Json {
    String(String),
    Number(i64),
    Bool(bool),
    Array(Vec<Self>),
    Object(Vec<(String, Self)>),
}

impl Json {
    fn string(value: &str) -> Self {
        Self::String(value.to_string())
    }

    fn write(&self, output: &mut String, indent: usize) {
        match self {
            Self::String(value) => write_json_string(output, value),
            Self::Number(value) => output.push_str(&value.to_string()),
            Self::Bool(value) => output.push_str(if *value { "true" } else { "false" }),
            Self::Array(items) if items.is_empty() => output.push_str("[]"),
            Self::Object(entries) if entries.is_empty() => output.push_str("{}"),
            Self::Array(items) => {
                output.push_str("[\n");
                for (index, item) in items.iter().enumerate() {
                    push_separator(output, index, indent + 2);
                    item.write(output, indent + 2);
                }
                push_close(output, indent, ']');
            }
            Self::Object(entries) => {
                output.push_str("{\n");
                for (index, (key, value)) in entries.iter().enumerate() {
                    push_separator(output, index, indent + 2);
                    write_json_string(output, key);
                    output.push_str(": ");
                    value.write(output, indent + 2);
                }
                push_close(output, indent, '}');
            }
        }
    }
}

fn push_separator(output: &mut String, index: usize, indent: usize) {
    if index > 0 {
        output.push_str(",\n");
    }
    output.push_str(&" ".repeat(indent));
}

fn push_close(output: &mut String, indent: usize, close: char) {
    output.push('\n');
    output.push_str(&" ".repeat(indent));
    output.push(close);
}

fn write_json_string(output: &mut String, value: &str) {
    // serde_json escapes strings exactly like ECMAScript JSON.stringify for
    // well-formed Unicode: short escapes plus lowercase `\u00xx` controls.
    output.push_str(&serde_json::to_string(value).unwrap_or_else(|_| format!("{value:?}")));
}
