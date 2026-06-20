use serde_json::{Map, Value};

use super::{parse_error, unsupported_error, GrammarImportError};
use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind};

const FORMAT: GrammarFormat = GrammarFormat::TreeSitter;

/// Parses tree-sitter `grammar.json` text into the grammar IR.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the JSON cannot be parsed or validated,
/// or when a node type cannot be represented in the grammar IR.
pub fn import_tree_sitter_json(text: &str) -> Result<Grammar, GrammarImportError> {
    let value = serde_json::from_str::<Value>(text)
        .map_err(|error| parse_error(FORMAT, error.to_string()))?;
    let root = value
        .as_object()
        .ok_or_else(|| parse_error(FORMAT, "grammar.json root must be an object"))?;
    let rules = required_object(root, "rules")?;
    let rule_names = source_ordered_rule_names(text, rules)?;
    if rule_names.is_empty() {
        return Err(parse_error(FORMAT, "rules object must not be empty"));
    }

    let mut grammar = Grammar::new().with_source_format(FORMAT);
    for name in rule_names {
        let node = rules
            .get(&name)
            .ok_or_else(|| parse_error(FORMAT, format!("rule {name:?} is missing")))?;
        grammar.add_rule(lower_rule(&name, node)?);
    }

    if let Some(start) = grammar.rules().first().map(|rule| rule.name.clone()) {
        grammar.set_start(start);
    }
    if let Some(extras) = root.get("extras") {
        if let Some(expr) = lower_extras(extras)? {
            grammar.add_rule(GrammarRule::new("_extras", expr).with_kind(RuleKind::Silent));
        }
    }

    Ok(grammar)
}

fn lower_rule(name: &str, node: &Value) -> Result<GrammarRule, GrammarImportError> {
    match node_type(node)? {
        "TOKEN" => Ok(GrammarRule::new(name, lower_content(node)?).with_kind(RuleKind::Token)),
        "IMMEDIATE_TOKEN" => Ok(GrammarRule::new(
            name,
            GrammarExpr::capture("immediate_token", lower_content(node)?),
        )
        .with_kind(RuleKind::Token)),
        _ => lower_node(node).map(|expr| GrammarRule::new(name, expr)),
    }
}

fn lower_node(node: &Value) -> Result<GrammarExpr, GrammarImportError> {
    match node_type(node)? {
        "SYMBOL" => string_field(node, "name").map(GrammarExpr::NonTerminal),
        "STRING" => string_field(node, "value").map(GrammarExpr::Terminal),
        "PATTERN" => string_field(node, "value").map(|value| lower_pattern(&value)),
        "BLANK" => Ok(GrammarExpr::Empty),
        "SEQ" => lower_sequence(array_field(node, "members")?),
        "CHOICE" => lower_choice(array_field(node, "members")?),
        "REPEAT" => lower_content(node).map(GrammarExpr::zero_or_more),
        "REPEAT1" => lower_content(node).map(GrammarExpr::one_or_more),
        "PREC" => lower_precedence(node, "prec"),
        "PREC_LEFT" => lower_precedence(node, "prec_left"),
        "PREC_RIGHT" => lower_precedence(node, "prec_right"),
        "PREC_DYNAMIC" => lower_precedence(node, "prec_dynamic"),
        "TOKEN" => lower_content(node).map(|expr| GrammarExpr::capture("token", expr)),
        "IMMEDIATE_TOKEN" => {
            lower_content(node).map(|expr| GrammarExpr::capture("immediate_token", expr))
        }
        "FIELD" => {
            let label = string_field(node, "name")?;
            lower_content(node).map(|expr| GrammarExpr::capture(label, expr))
        }
        "ALIAS" => {
            let value = string_field(node, "value")?;
            lower_content(node).map(|expr| GrammarExpr::capture(format!("alias:{value}"), expr))
        }
        "RESERVED" => lower_reserved(node),
        construct => Err(unsupported_error(FORMAT, construct)),
    }
}

fn lower_sequence(members: &[Value]) -> Result<GrammarExpr, GrammarImportError> {
    let mut lowered = Vec::new();
    for member in members {
        push_sequence_item(&mut lowered, lower_node(member)?);
    }

    Ok(match lowered.len() {
        0 => GrammarExpr::Empty,
        1 => lowered.remove(0),
        _ => GrammarExpr::Sequence(lowered),
    })
}

fn push_sequence_item(items: &mut Vec<GrammarExpr>, item: GrammarExpr) {
    match item {
        GrammarExpr::Empty => {}
        GrammarExpr::Sequence(nested) => {
            for item in nested {
                push_sequence_item(items, item);
            }
        }
        item => items.push(item),
    }
}

fn lower_choice(members: &[Value]) -> Result<GrammarExpr, GrammarImportError> {
    let mut alternatives = Vec::new();
    for member in members {
        alternatives.push(lower_node(member)?);
    }

    if alternatives.len() == 2 {
        if alternatives[0] == GrammarExpr::Empty {
            return Ok(GrammarExpr::optional(alternatives.remove(1)));
        }
        if alternatives[1] == GrammarExpr::Empty {
            return Ok(GrammarExpr::optional(alternatives.remove(0)));
        }
    }

    Ok(match alternatives.len() {
        0 => GrammarExpr::Empty,
        1 => alternatives.remove(0),
        _ => GrammarExpr::Choice {
            ordered: false,
            alternatives,
        },
    })
}

fn lower_precedence(node: &Value, label: &str) -> Result<GrammarExpr, GrammarImportError> {
    let value = precedence_value(node)?;
    lower_content(node).map(|expr| GrammarExpr::capture(format!("{label}={value}"), expr))
}

fn lower_reserved(node: &Value) -> Result<GrammarExpr, GrammarImportError> {
    let label = node
        .get("context_name")
        .and_then(Value::as_str)
        .map_or_else(
            || "reserved".to_string(),
            |context| format!("reserved:{context}"),
        );
    lower_content(node).map(|expr| GrammarExpr::capture(label, expr))
}

fn lower_content(node: &Value) -> Result<GrammarExpr, GrammarImportError> {
    lower_node(
        node.as_object()
            .and_then(|object| object.get("content"))
            .ok_or_else(|| parse_error(FORMAT, "node is missing content"))?,
    )
}

fn lower_extras(node: &Value) -> Result<Option<GrammarExpr>, GrammarImportError> {
    let members = node
        .as_array()
        .ok_or_else(|| parse_error(FORMAT, "extras must be an array"))?;
    if members.is_empty() {
        return Ok(None);
    }
    lower_choice(members).map(Some)
}

fn lower_pattern(value: &str) -> GrammarExpr {
    parse_char_class_pattern(value)
        .unwrap_or_else(|| GrammarExpr::capture("regex", GrammarExpr::Terminal(value.to_string())))
}

fn parse_char_class_pattern(pattern: &str) -> Option<GrammarExpr> {
    if !pattern.starts_with('[') || !pattern.ends_with(']') {
        return None;
    }
    let content = &pattern[1..pattern.len() - 1];
    let (negated, content) = content
        .strip_prefix('^')
        .map_or((false, content), |content| (true, content));
    let chars = content.chars().collect::<Vec<_>>();
    let mut items = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        let start = read_class_char(&chars, &mut index)?;
        if index + 1 < chars.len() && chars[index] == '-' {
            index += 1;
            let end = read_class_char(&chars, &mut index)?;
            if start > end {
                return None;
            }
            items.push(CharClassItem::Range(start, end));
        } else {
            items.push(CharClassItem::Char(start));
        }
    }
    if items.is_empty() {
        return None;
    }
    Some(GrammarExpr::CharClass { negated, items })
}

fn read_class_char(chars: &[char], index: &mut usize) -> Option<char> {
    let character = *chars.get(*index)?;
    *index += 1;
    if character != '\\' {
        return Some(character);
    }
    let escaped = *chars.get(*index)?;
    *index += 1;
    match escaped {
        'n' => Some('\n'),
        'r' => Some('\r'),
        't' => Some('\t'),
        '\\' | '"' | '\'' | '[' | ']' | '-' | '^' => Some(escaped),
        _ => None,
    }
}

fn node_type(node: &Value) -> Result<&str, GrammarImportError> {
    node.as_object()
        .ok_or_else(|| parse_error(FORMAT, "grammar node must be an object"))?
        .get("type")
        .and_then(Value::as_str)
        .ok_or_else(|| parse_error(FORMAT, "grammar node is missing string type"))
}

fn string_field(node: &Value, field: &str) -> Result<String, GrammarImportError> {
    node.as_object()
        .and_then(|object| object.get(field))
        .and_then(Value::as_str)
        .map(ToString::to_string)
        .ok_or_else(|| parse_error(FORMAT, format!("node is missing string {field}")))
}

fn array_field<'node>(
    node: &'node Value,
    field: &str,
) -> Result<&'node [Value], GrammarImportError> {
    node.as_object()
        .and_then(|object| object.get(field))
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| parse_error(FORMAT, format!("node is missing array {field}")))
}

fn precedence_value(node: &Value) -> Result<String, GrammarImportError> {
    let value = node
        .as_object()
        .and_then(|object| object.get("value"))
        .ok_or_else(|| parse_error(FORMAT, "precedence node is missing value"))?;
    match value {
        Value::Number(number) => Ok(number.to_string()),
        Value::String(value) => Ok(value.clone()),
        _ => Err(parse_error(
            FORMAT,
            "precedence value must be a number or string",
        )),
    }
}

fn required_object<'value>(
    root: &'value Map<String, Value>,
    field: &str,
) -> Result<&'value Map<String, Value>, GrammarImportError> {
    root.get(field)
        .and_then(Value::as_object)
        .ok_or_else(|| parse_error(FORMAT, format!("grammar.json must contain object {field}")))
}

fn source_ordered_rule_names(
    text: &str,
    rules: &Map<String, Value>,
) -> Result<Vec<String>, GrammarImportError> {
    let names = JsonScanner::new(text).top_level_object_keys("rules")?;
    if names.len() != rules.len() || names.iter().any(|name| !rules.contains_key(name)) {
        return Err(parse_error(
            FORMAT,
            "rules object in source text does not match parsed JSON object",
        ));
    }
    Ok(names)
}

#[derive(Clone, Debug)]
struct JsonScanner<'text> {
    text: &'text str,
    cursor: usize,
}

impl<'text> JsonScanner<'text> {
    const fn new(text: &'text str) -> Self {
        Self { text, cursor: 0 }
    }

    fn top_level_object_keys(mut self, property: &str) -> Result<Vec<String>, GrammarImportError> {
        self.skip_whitespace();
        self.consume_byte(b'{')?;
        self.skip_whitespace();
        if self.try_consume_byte(b'}') {
            return Err(parse_error(FORMAT, format!("missing {property} object")));
        }

        loop {
            let key = self.parse_string()?;
            self.skip_whitespace();
            self.consume_byte(b':')?;
            self.skip_whitespace();
            if key == property {
                return self.object_keys();
            }
            self.skip_value()?;
            self.skip_whitespace();
            if self.try_consume_byte(b'}') {
                return Err(parse_error(FORMAT, format!("missing {property} object")));
            }
            self.consume_byte(b',')?;
            self.skip_whitespace();
        }
    }

    fn object_keys(&mut self) -> Result<Vec<String>, GrammarImportError> {
        self.consume_byte(b'{')?;
        self.skip_whitespace();
        let mut keys = Vec::new();
        if self.try_consume_byte(b'}') {
            return Ok(keys);
        }

        loop {
            keys.push(self.parse_string()?);
            self.skip_whitespace();
            self.consume_byte(b':')?;
            self.skip_whitespace();
            self.skip_value()?;
            self.skip_whitespace();
            if self.try_consume_byte(b'}') {
                return Ok(keys);
            }
            self.consume_byte(b',')?;
            self.skip_whitespace();
        }
    }

    fn skip_value(&mut self) -> Result<(), GrammarImportError> {
        self.skip_whitespace();
        match self.peek_byte() {
            Some(b'"') => self.parse_string().map(|_| ()),
            Some(b'{') => self.skip_object(),
            Some(b'[') => self.skip_array(),
            Some(_) => {
                while let Some(byte) = self.peek_byte() {
                    if byte.is_ascii_whitespace() || matches!(byte, b',' | b'}' | b']') {
                        break;
                    }
                    self.cursor += 1;
                }
                Ok(())
            }
            None => Err(parse_error(FORMAT, "unexpected end of JSON value")),
        }
    }

    fn skip_object(&mut self) -> Result<(), GrammarImportError> {
        self.consume_byte(b'{')?;
        self.skip_whitespace();
        if self.try_consume_byte(b'}') {
            return Ok(());
        }
        loop {
            self.parse_string()?;
            self.skip_whitespace();
            self.consume_byte(b':')?;
            self.skip_value()?;
            self.skip_whitespace();
            if self.try_consume_byte(b'}') {
                return Ok(());
            }
            self.consume_byte(b',')?;
            self.skip_whitespace();
        }
    }

    fn skip_array(&mut self) -> Result<(), GrammarImportError> {
        self.consume_byte(b'[')?;
        self.skip_whitespace();
        if self.try_consume_byte(b']') {
            return Ok(());
        }
        loop {
            self.skip_value()?;
            self.skip_whitespace();
            if self.try_consume_byte(b']') {
                return Ok(());
            }
            self.consume_byte(b',')?;
            self.skip_whitespace();
        }
    }

    fn parse_string(&mut self) -> Result<String, GrammarImportError> {
        self.skip_whitespace();
        let start = self.cursor;
        self.consume_byte(b'"')?;
        let bytes = self.text.as_bytes();
        while let Some(byte) = bytes.get(self.cursor).copied() {
            self.cursor += 1;
            match byte {
                b'\\' => {
                    if self.cursor >= bytes.len() {
                        return Err(parse_error(FORMAT, "unterminated JSON string escape"));
                    }
                    self.cursor += 1;
                }
                b'"' => {
                    let text = &self.text[start..self.cursor];
                    return serde_json::from_str::<String>(text)
                        .map_err(|error| parse_error(FORMAT, error.to_string()));
                }
                _ => {}
            }
        }
        Err(parse_error(FORMAT, "unterminated JSON string"))
    }

    fn skip_whitespace(&mut self) {
        while self
            .peek_byte()
            .is_some_and(|byte| byte.is_ascii_whitespace())
        {
            self.cursor += 1;
        }
    }

    fn consume_byte(&mut self, expected: u8) -> Result<(), GrammarImportError> {
        self.skip_whitespace();
        if self.try_consume_byte(expected) {
            Ok(())
        } else {
            Err(parse_error(
                FORMAT,
                format!("expected JSON byte {:?}", char::from(expected)),
            ))
        }
    }

    fn try_consume_byte(&mut self, expected: u8) -> bool {
        if self.peek_byte() == Some(expected) {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn peek_byte(&self) -> Option<u8> {
        self.text.as_bytes().get(self.cursor).copied()
    }
}
