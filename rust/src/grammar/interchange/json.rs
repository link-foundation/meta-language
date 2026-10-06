//! The serialized JSON form of a grammar: `{schemaVersion: 1, start,
//! sourceFormat, rules, declarations?}` written with two-space indentation and
//! the key order of `Grammar.normalized()` in `js/src/grammar.js`, so both
//! runtimes write the same bytes for the same grammar and read each other's
//! documents.

use serde_json::Value;

use super::super::feature::{class_expression, feature_expression_fields, operation_form};
use super::super::{
    ByteClassItem, CharClassItem, FeatureExpr, FeatureForm, FieldType, FieldValue, Grammar,
    GrammarDeclarations, GrammarExpr, GrammarFormat, GrammarImportError, GrammarMacro, GrammarRule,
    GrammarScanner, Operation, OperationCategory, PrecedenceEntry, RuleAttributes, RuleKind,
    UnicodeClassItem,
};

/// A JSON value whose objects keep their key order.
enum Json {
    Null,
    Bool(bool),
    Int(i64),
    Str(String),
    Array(Vec<Self>),
    Object(Vec<(&'static str, Self)>),
}

impl Json {
    fn str(value: &str) -> Self {
        Self::Str(value.to_owned())
    }

    fn strings(values: &[String]) -> Self {
        Self::Array(values.iter().map(|value| Self::str(value)).collect())
    }

    fn render(&self, indent: usize, out: &mut String) {
        match self {
            Self::Null => out.push_str("null"),
            Self::Bool(value) => out.push_str(if *value { "true" } else { "false" }),
            Self::Int(value) => out.push_str(&value.to_string()),
            Self::Str(value) => {
                out.push_str(&serde_json::to_string(value).unwrap_or_default());
            }
            Self::Array(items) if items.is_empty() => out.push_str("[]"),
            Self::Object(entries) if entries.is_empty() => out.push_str("{}"),
            Self::Array(items) => {
                out.push('[');
                for (index, item) in items.iter().enumerate() {
                    out.push_str(if index == 0 { "\n" } else { ",\n" });
                    push_indent(indent + 1, out);
                    item.render(indent + 1, out);
                }
                out.push('\n');
                push_indent(indent, out);
                out.push(']');
            }
            Self::Object(entries) => {
                out.push('{');
                for (index, (key, value)) in entries.iter().enumerate() {
                    out.push_str(if index == 0 { "\n" } else { ",\n" });
                    push_indent(indent + 1, out);
                    out.push('"');
                    out.push_str(key);
                    out.push_str("\": ");
                    value.render(indent + 1, out);
                }
                out.push('\n');
                push_indent(indent, out);
                out.push('}');
            }
        }
    }
}

fn push_indent(indent: usize, out: &mut String) {
    for _ in 0..indent {
        out.push_str("  ");
    }
}

fn int(value: usize) -> Json {
    Json::Int(i64::try_from(value).unwrap_or(i64::MAX))
}

fn char_json(value: char) -> Json {
    Json::Str(value.to_string())
}

fn unary(kind: &'static str, item: &GrammarExpr) -> Json {
    Json::Object(vec![
        ("kind", Json::str(kind)),
        ("item", expression_json(item)),
    ])
}

fn class_json(kind: &'static str, negated: bool, items: Vec<Json>) -> Json {
    Json::Object(vec![
        ("kind", Json::str(kind)),
        ("negated", Json::Bool(negated)),
        ("items", Json::Array(items)),
    ])
}

fn char_item_json(item: &CharClassItem) -> Json {
    match item {
        CharClassItem::Char(value) => Json::Object(vec![
            ("kind", Json::str("char")),
            ("value", char_json(*value)),
        ]),
        CharClassItem::Range(start, end) => Json::Object(vec![
            ("kind", Json::str("range")),
            ("start", char_json(*start)),
            ("end", char_json(*end)),
        ]),
    }
}

fn expressions_json(items: &[GrammarExpr]) -> Json {
    Json::Array(items.iter().map(expression_json).collect())
}

fn expression_json(expr: &GrammarExpr) -> Json {
    let kind = |kind: &str| ("kind", Json::str(kind));
    match expr {
        GrammarExpr::Empty => Json::Object(vec![kind("empty")]),
        GrammarExpr::AnyChar => Json::Object(vec![kind("any")]),
        GrammarExpr::Terminal(value) => {
            Json::Object(vec![kind("literal"), ("value", Json::str(value))])
        }
        GrammarExpr::TerminalInsensitive(value) => Json::Object(vec![
            kind("literalInsensitive"),
            ("value", Json::str(value)),
        ]),
        GrammarExpr::CharRange(start, end) => Json::Object(vec![
            kind("charRange"),
            ("start", char_json(*start)),
            ("end", char_json(*end)),
        ]),
        GrammarExpr::CharClass { negated, items } => class_json(
            "charClass",
            *negated,
            items.iter().map(char_item_json).collect(),
        ),
        GrammarExpr::NonTerminal(name) => {
            Json::Object(vec![kind("ref"), ("name", Json::str(name))])
        }
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => Json::Object(vec![
            kind("choice"),
            ("items", expressions_json(alternatives)),
            ("ordered", Json::Bool(*ordered)),
        ]),
        GrammarExpr::Sequence(items) => {
            Json::Object(vec![kind("seq"), ("items", expressions_json(items))])
        }
        GrammarExpr::Optional(item) => unary("optional", item),
        GrammarExpr::ZeroOrMore(item) => unary("repeat0", item),
        GrammarExpr::OneOrMore(item) => unary("repeat1", item),
        GrammarExpr::And(item) => unary("and", item),
        GrammarExpr::Not(item) => unary("not", item),
        GrammarExpr::Repeat { expr, min, max } => Json::Object(vec![
            kind("repeat"),
            ("item", expression_json(expr)),
            ("min", int(*min)),
            ("max", max.map_or(Json::Null, int)),
        ]),
        GrammarExpr::Capture { label, expr } => Json::Object(vec![
            kind("capture"),
            ("label", label.as_deref().map_or(Json::Null, Json::str)),
            ("item", expression_json(expr)),
        ]),
        GrammarExpr::Feature(feature) => feature_json(feature),
    }
}

fn feature_json(feature: &FeatureExpr) -> Json {
    match feature {
        FeatureExpr::UnicodeClass { negated, items } => class_json(
            "charClass",
            *negated,
            items
                .iter()
                .map(|item| match item {
                    UnicodeClassItem::Char(value) => char_item_json(&CharClassItem::Char(*value)),
                    UnicodeClassItem::Range(start, end) => {
                        char_item_json(&CharClassItem::Range(*start, *end))
                    }
                    UnicodeClassItem::Category(value) => Json::Object(vec![
                        ("kind", Json::str("category")),
                        ("value", Json::str(value)),
                    ]),
                    UnicodeClassItem::Script(value) => Json::Object(vec![
                        ("kind", Json::str("script")),
                        ("value", Json::str(value)),
                    ]),
                })
                .collect(),
        ),
        FeatureExpr::ByteClass { negated, items } => class_json(
            "byteClass",
            *negated,
            items
                .iter()
                .map(|item| match item {
                    ByteClassItem::Byte(value) => Json::Object(vec![
                        ("kind", Json::str("byte")),
                        ("value", Json::Int(i64::from(*value))),
                    ]),
                    ByteClassItem::Range(start, end) => Json::Object(vec![
                        ("kind", Json::str("byteRange")),
                        ("start", Json::Int(i64::from(*start))),
                        ("end", Json::Int(i64::from(*end))),
                    ]),
                })
                .collect(),
        ),
        FeatureExpr::Call { name, arguments } => Json::Object(vec![
            ("kind", Json::str("ref")),
            ("name", Json::str(name)),
            ("arguments", expressions_json(arguments)),
        ]),
        FeatureExpr::Form(form) => form_json("kind", form),
    }
}

/// A feature form or an operation: its head under `head_key`, then its fields.
fn form_json(head_key: &'static str, form: &FeatureForm) -> Json {
    let mut entries = vec![(head_key, Json::str(&form.head))];
    for ((key, _), value) in form.spec().iter().zip(&form.fields) {
        let value = match value {
            FieldValue::Word(text) | FieldValue::Name(text) | FieldValue::Text(text) => {
                Json::str(text)
            }
            FieldValue::Integer(value) => Json::Int(*value),
            FieldValue::Expression(expr) => expression_json(expr),
            FieldValue::Expressions(items) => expressions_json(items),
            FieldValue::Operation(operation) => operation_json(operation),
            FieldValue::Operations(items) | FieldValue::Block(Some(items)) => {
                operations_json(items)
            }
            FieldValue::Block(None) => continue,
        };
        entries.push((key, value));
    }
    Json::Object(entries)
}

fn operation_json(operation: &Operation) -> Json {
    form_json("operation", operation)
}

fn operations_json(operations: &[Operation]) -> Json {
    Json::Array(operations.iter().map(operation_json).collect())
}

fn rule_json(rule: &GrammarRule) -> Json {
    let mut entries = vec![
        ("name", Json::str(&rule.name)),
        ("kind", Json::str(rule.kind.as_str())),
        ("expression", expression_json(&rule.expr)),
    ];
    let attributes = &rule.attributes;
    if !attributes.parameters.is_empty() {
        entries.push(("parameters", Json::strings(&attributes.parameters)));
    }
    if let Some(channel) = &attributes.channel {
        entries.push(("channel", Json::str(channel)));
    }
    if let Some(modes) = &attributes.modes {
        entries.push(("modes", Json::strings(modes)));
    }
    if let Some(action) = &attributes.action {
        entries.push(("action", operations_json(action)));
    }
    Json::Object(entries)
}

fn declarations_json(declarations: &GrammarDeclarations) -> Json {
    let mut entries = Vec::new();
    if let Some(matching) = &declarations.matching {
        entries.push(("matching", Json::str(matching)));
    }
    if let Some(settling) = &declarations.settling {
        entries.push(("settling", Json::strings(settling)));
    }
    if !declarations.imports.is_empty() {
        entries.push(("imports", Json::strings(&declarations.imports)));
    }
    if !declarations.modes.is_empty() {
        entries.push(("modes", Json::strings(&declarations.modes)));
    }
    if !declarations.extras.is_empty() {
        entries.push(("extras", expressions_json(&declarations.extras)));
    }
    if !declarations.conflicts.is_empty() {
        let groups = declarations
            .conflicts
            .iter()
            .map(|group| Json::strings(group));
        entries.push(("conflicts", Json::Array(groups.collect())));
    }
    if !declarations.precedences.is_empty() {
        let orders = declarations.precedences.iter().map(|order| {
            Json::Array(
                order
                    .iter()
                    .map(|entry| {
                        Json::Object(vec![
                            ("kind", Json::str(entry.kind())),
                            ("value", Json::str(entry.value())),
                        ])
                    })
                    .collect(),
            )
        });
        entries.push(("precedences", Json::Array(orders.collect())));
    }
    if !declarations.macros.is_empty() {
        let macros = declarations.macros.iter().map(|declared| {
            Json::Object(vec![
                ("name", Json::str(&declared.name)),
                ("parameters", Json::strings(&declared.parameters)),
                ("expression", expression_json(&declared.expression)),
            ])
        });
        entries.push(("macros", Json::Array(macros.collect())));
    }
    if !declarations.scanners.is_empty() {
        let scanners = declarations.scanners.iter().map(|scanner| {
            Json::Object(vec![
                ("name", Json::str(&scanner.name)),
                ("tokens", Json::strings(&scanner.tokens)),
                ("operations", operations_json(&scanner.operations)),
            ])
        });
        entries.push(("scanners", Json::Array(scanners.collect())));
    }
    Json::Object(entries)
}

/// Serializes `grammar` as `JSON.stringify(grammar.normalized(), null, 2)`
/// followed by a newline, as `serializeGrammar` of `js/src/grammar.js` does.
#[must_use]
pub fn serialize_grammar(grammar: &Grammar) -> String {
    let mut entries = vec![
        ("schemaVersion", Json::Int(1)),
        ("start", grammar.start().map_or(Json::Null, Json::str)),
        (
            "sourceFormat",
            grammar
                .source_format()
                .map_or(Json::Null, |format| Json::str(format.as_str())),
        ),
        (
            "rules",
            Json::Array(grammar.rules().iter().map(rule_json).collect()),
        ),
    ];
    if !grammar.declarations().is_empty() {
        entries.push(("declarations", declarations_json(grammar.declarations())));
    }
    let mut out = String::new();
    Json::Object(entries).render(0, &mut out);
    out.push('\n');
    out
}

fn json_error(detail: impl Into<String>) -> GrammarImportError {
    GrammarImportError::Parse {
        format: GrammarFormat::MetaLanguage,
        message: detail.into(),
    }
}

fn invalid(what: &str) -> GrammarImportError {
    json_error(format!("invalid serialized grammar {what}"))
}

fn field<'a>(value: &'a Value, key: &str, what: &str) -> Result<&'a Value, GrammarImportError> {
    value.get(key).ok_or_else(|| invalid(what))
}

fn string(value: &Value, key: &str, what: &str) -> Result<String, GrammarImportError> {
    field(value, key, what)?
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| invalid(what))
}

fn optional_string(
    value: &Value,
    key: &str,
    what: &str,
) -> Result<Option<String>, GrammarImportError> {
    match value.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(_) => Err(invalid(what)),
    }
}

fn strings(value: Option<&Value>, what: &str) -> Result<Vec<String>, GrammarImportError> {
    match value {
        None | Some(Value::Null) => Ok(Vec::new()),
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| {
                item.as_str()
                    .map(str::to_owned)
                    .ok_or_else(|| invalid(what))
            })
            .collect(),
        Some(_) => Err(invalid(what)),
    }
}

fn array<'a>(value: &'a Value, key: &str, what: &str) -> Result<&'a [Value], GrammarImportError> {
    field(value, key, what)?
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| invalid(what))
}

fn character(value: &Value, key: &str) -> Result<char, GrammarImportError> {
    let text = string(value, key, "expression")?;
    let mut chars = text.chars();
    match (chars.next(), chars.next()) {
        (Some(value), None) => Ok(value),
        _ => Err(invalid("expression")),
    }
}

fn integer(value: &Value, key: &str, what: &str) -> Result<i64, GrammarImportError> {
    field(value, key, what)?
        .as_i64()
        .ok_or_else(|| invalid(what))
}

fn count(value: &Value, key: &str) -> Result<usize, GrammarImportError> {
    usize::try_from(integer(value, key, "expression")?).map_err(|_| invalid("expression"))
}

fn byte(value: &Value, key: &str) -> Result<u8, GrammarImportError> {
    u8::try_from(integer(value, key, "expression")?).map_err(|_| invalid("expression"))
}

fn read_expressions(value: &Value, key: &str) -> Result<Vec<GrammarExpr>, GrammarImportError> {
    array(value, key, "expression")?
        .iter()
        .map(read_expression)
        .collect()
}

fn read_item(value: &Value) -> Result<Box<GrammarExpr>, GrammarImportError> {
    Ok(Box::new(read_expression(field(
        value,
        "item",
        "expression",
    )?)?))
}

fn read_expression(value: &Value) -> Result<GrammarExpr, GrammarImportError> {
    let kind = string(value, "kind", "expression")?;
    Ok(match kind.as_str() {
        "empty" => GrammarExpr::Empty,
        "any" => GrammarExpr::AnyChar,
        "literal" => GrammarExpr::Terminal(string(value, "value", "expression")?),
        "literalInsensitive" => {
            GrammarExpr::TerminalInsensitive(string(value, "value", "expression")?)
        }
        "charRange" => GrammarExpr::CharRange(character(value, "start")?, character(value, "end")?),
        "charClass" => {
            let negated = value
                .get("negated")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let items = array(value, "items", "expression")?
                .iter()
                .map(read_class_item)
                .collect::<Result<_, _>>()?;
            class_expression(negated, items)
        }
        "byteClass" => {
            let negated = value
                .get("negated")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let items = array(value, "items", "expression")?
                .iter()
                .map(|item| match string(item, "kind", "expression")?.as_str() {
                    "byte" => Ok(ByteClassItem::Byte(byte(item, "value")?)),
                    "byteRange" => Ok(ByteClassItem::Range(
                        byte(item, "start")?,
                        byte(item, "end")?,
                    )),
                    _ => Err(invalid("expression")),
                })
                .collect::<Result<_, _>>()?;
            GrammarExpr::feature(FeatureExpr::ByteClass { negated, items })
        }
        "ref" => {
            let name = string(value, "name", "expression")?;
            match value.get("arguments") {
                None | Some(Value::Null) => GrammarExpr::NonTerminal(name),
                Some(_) => GrammarExpr::feature(FeatureExpr::Call {
                    name,
                    arguments: read_expressions(value, "arguments")?,
                }),
            }
        }
        "choice" => GrammarExpr::Choice {
            ordered: value
                .get("ordered")
                .and_then(Value::as_bool)
                .unwrap_or(false),
            alternatives: read_expressions(value, "items")?,
        },
        "seq" => GrammarExpr::Sequence(read_expressions(value, "items")?),
        "optional" => GrammarExpr::Optional(read_item(value)?),
        "repeat0" => GrammarExpr::ZeroOrMore(read_item(value)?),
        "repeat1" => GrammarExpr::OneOrMore(read_item(value)?),
        "and" => GrammarExpr::And(read_item(value)?),
        "not" => GrammarExpr::Not(read_item(value)?),
        "repeat" => GrammarExpr::Repeat {
            expr: read_item(value)?,
            min: count(value, "min")?,
            max: match value.get("max") {
                None | Some(Value::Null) => None,
                Some(_) => Some(count(value, "max")?),
            },
        },
        "capture" => GrammarExpr::Capture {
            label: optional_string(value, "label", "expression")?,
            expr: read_item(value)?,
        },
        other => {
            let fields = feature_expression_fields(other).ok_or_else(|| {
                json_error(format!(
                    "unsupported serialized grammar expression kind {other}"
                ))
            })?;
            GrammarExpr::feature(FeatureExpr::Form(read_form(other, fields, value)?))
        }
    })
}

fn read_class_item(item: &Value) -> Result<UnicodeClassItem, GrammarImportError> {
    Ok(match string(item, "kind", "expression")?.as_str() {
        "char" => UnicodeClassItem::Char(character(item, "value")?),
        "range" => UnicodeClassItem::Range(character(item, "start")?, character(item, "end")?),
        "category" => UnicodeClassItem::Category(string(item, "value", "expression")?),
        "script" => UnicodeClassItem::Script(string(item, "value", "expression")?),
        _ => return Err(invalid("expression")),
    })
}

fn read_form(
    head: &str,
    fields: &[(&'static str, FieldType)],
    value: &Value,
) -> Result<FeatureForm, GrammarImportError> {
    let mut values = Vec::with_capacity(fields.len());
    for (key, kind) in fields {
        let what = "expression";
        values.push(match kind {
            FieldType::Choice(words) => {
                let word = string(value, key, what)?;
                if !words.contains(&word.as_str()) {
                    return Err(invalid(what));
                }
                FieldValue::Word(word)
            }
            FieldType::Integer => FieldValue::Integer(integer(value, key, what)?),
            FieldType::Name => FieldValue::Name(string(value, key, what)?),
            FieldType::Text => FieldValue::Text(string(value, key, what)?),
            FieldType::Expression => {
                FieldValue::Expression(read_expression(field(value, key, what)?)?)
            }
            FieldType::Expressions => FieldValue::Expressions(read_expressions(value, key)?),
            FieldType::Condition => FieldValue::Operation(read_operation(
                field(value, key, what)?,
                OperationCategory::Condition,
            )?),
            FieldType::Value => FieldValue::Operation(read_operation(
                field(value, key, what)?,
                OperationCategory::Value,
            )?),
            FieldType::Conditions => FieldValue::Operations(read_operations(
                array(value, key, what)?,
                OperationCategory::Condition,
            )?),
            FieldType::Block(_, optional) => match value.get(*key) {
                None | Some(Value::Null) if *optional => FieldValue::Block(None),
                _ => FieldValue::Block(Some(read_operations(
                    array(value, key, what)?,
                    OperationCategory::Statement,
                )?)),
            },
        });
    }
    Ok(FeatureForm::new(head, values))
}

fn read_operation(
    value: &Value,
    category: OperationCategory,
) -> Result<Operation, GrammarImportError> {
    let head = string(value, "operation", "operation")?;
    let (found, fields) = operation_form(&head)
        .ok_or_else(|| json_error(format!("unsupported serialized grammar operation {head}")))?;
    if found != category {
        return Err(json_error(format!(
            "serialized grammar operation {head} is not a {}",
            category.as_str()
        )));
    }
    read_form(&head, fields, value)
}

fn read_operations(
    values: &[Value],
    category: OperationCategory,
) -> Result<Vec<Operation>, GrammarImportError> {
    values
        .iter()
        .map(|value| read_operation(value, category))
        .collect()
}

fn read_declarations(value: &Value) -> Result<GrammarDeclarations, GrammarImportError> {
    let what = "declarations";
    let mut declarations = GrammarDeclarations {
        matching: optional_string(value, "matching", what)?,
        settling: value
            .get("settling")
            .map(|steps| strings(Some(steps), what))
            .transpose()?,
        imports: strings(value.get("imports"), what)?,
        modes: strings(value.get("modes"), what)?,
        ..GrammarDeclarations::default()
    };
    if let Some(Value::Array(extras)) = value.get("extras") {
        declarations.extras = extras
            .iter()
            .map(read_expression)
            .collect::<Result<_, _>>()?;
    }
    if let Some(Value::Array(groups)) = value.get("conflicts") {
        declarations.conflicts = groups
            .iter()
            .map(|group| strings(Some(group), what))
            .collect::<Result<_, _>>()?;
    }
    if let Some(Value::Array(orders)) = value.get("precedences") {
        for order in orders {
            let Value::Array(entries) = order else {
                return Err(invalid(what));
            };
            let mut read = Vec::new();
            for entry in entries {
                let kind = string(entry, "kind", what)?;
                let value = string(entry, "value", what)?;
                let Some(entry) = PrecedenceEntry::from_kind(&kind, value) else {
                    return Err(invalid(what));
                };
                read.push(entry);
            }
            declarations.precedences.push(read);
        }
    }
    if let Some(Value::Array(macros)) = value.get("macros") {
        for declared in macros {
            declarations.macros.push(GrammarMacro {
                name: string(declared, "name", what)?,
                parameters: strings(declared.get("parameters"), what)?,
                expression: read_expression(field(declared, "expression", what)?)?,
            });
        }
    }
    if let Some(Value::Array(scanners)) = value.get("scanners") {
        for scanner in scanners {
            declarations.scanners.push(GrammarScanner {
                name: string(scanner, "name", what)?,
                tokens: strings(scanner.get("tokens"), what)?,
                operations: read_operations(
                    array(scanner, "operations", what)?,
                    OperationCategory::Statement,
                )?,
            });
        }
    }
    Ok(declarations)
}

fn read_rule(value: &Value) -> Result<GrammarRule, GrammarImportError> {
    let name = value.get("name").and_then(Value::as_str);
    let expression = value
        .get("expression")
        .filter(|expr| expr.get("kind").is_some());
    let (Some(name), Some(expression)) = (name, expression) else {
        return Err(invalid("rule"));
    };
    let kind = match value.get("kind") {
        None | Some(Value::Null) => RuleKind::Normal,
        Some(kind) => kind
            .as_str()
            .and_then(RuleKind::from_tag)
            .ok_or_else(|| invalid("rule"))?,
    };
    let attributes = RuleAttributes {
        parameters: strings(value.get("parameters"), "rule")?,
        channel: optional_string(value, "channel", "rule")?,
        modes: match value.get("modes") {
            None | Some(Value::Null) => None,
            modes => Some(strings(modes, "rule")?),
        },
        action: match value.get("action") {
            None | Some(Value::Null) => None,
            Some(_) => Some(read_operations(
                array(value, "action", "rule")?,
                OperationCategory::Statement,
            )?),
        },
    };
    Ok(GrammarRule::new(name, read_expression(expression)?)
        .with_kind(kind)
        .with_attributes(attributes))
}

/// Reads a document [`serialize_grammar`] (or `serializeGrammar` of
/// `js/src/grammar.js`) wrote.
///
/// # Errors
///
/// Returns a [`GrammarImportError`] when the text is not JSON, is not a
/// schema version 1 grammar document, names a rule twice, holds an
/// expression or operation this runtime does not know, or has no start rule.
pub fn deserialize_grammar(source: &str) -> Result<Grammar, GrammarImportError> {
    let value: Value = serde_json::from_str(source)
        .map_err(|error| json_error(format!("invalid serialized grammar JSON: {error}")))?;
    let (Some(1), Some(Value::Array(rules))) = (
        value.get("schemaVersion").and_then(Value::as_i64),
        value.get("rules"),
    ) else {
        return Err(invalid("document"));
    };
    let mut grammar = Grammar::new();
    for rule in rules {
        let rule = read_rule(rule)?;
        if grammar.rule(&rule.name).is_some() {
            return Err(json_error(format!(
                "duplicate serialized grammar rule {}",
                rule.name
            )));
        }
        grammar.add_rule(rule);
    }
    if let Some(start) = value.get("start").and_then(Value::as_str) {
        grammar.set_start(start);
    }
    if let Some(format) = value.get("sourceFormat").and_then(Value::as_str) {
        let format = GrammarFormat::from_tag(format)
            .ok_or_else(|| json_error(format!("unknown serialized grammar format {format}")))?;
        grammar.set_source_format(format);
    }
    if let Some(declarations) = value.get("declarations").filter(|value| !value.is_null()) {
        grammar.set_declarations(read_declarations(declarations)?);
    }
    if grammar.start_rule().is_none() {
        return Err(json_error("serialized grammar has no start rule"));
    }
    Ok(grammar)
}
