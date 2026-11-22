use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

use super::super::{
    CharClassItem, ExprBuilder, Grammar, GrammarExpr, GrammarRule, import::lower_lark_regex,
    parse_grammar_links,
};
use super::numeric_ranges::{DecimalRange, signed_decimal_range, signed_integer_range};
use super::property_names::additional_property_name;
use super::schema::{Analysis, formats, keywords, reference, types};
use super::{JsonSchemaIssue, pointer_child};

type Expression = GrammarExpr;
const EXPRESSION: ExprBuilder = ExprBuilder;

fn reference_expression(name: &str) -> Expression {
    EXPRESSION.nt(name)
}

fn separated(item: Expression) -> Expression {
    EXPRESSION.seq([
        reference_expression("json_space"),
        EXPRESSION.term(","),
        reference_expression("json_space"),
        item,
    ])
}

fn collection(left: &str, right: &str, members: Expression) -> Expression {
    EXPRESSION.seq([
        EXPRESSION.term(left),
        reference_expression("json_space"),
        members,
        reference_expression("json_space"),
        EXPRESSION.term(right),
    ])
}

fn characters(value: &str) -> Expression {
    EXPRESSION.char_class(false, value.chars().map(CharClassItem::Char))
}

fn quoted(value: &str) -> Expression {
    EXPRESSION.seq(
        std::iter::once(EXPRESSION.term("\""))
            .chain(value.chars().map(|character| {
                let text = serde_json::to_string(&character).expect("JSON character");
                let mut units = [0; 2];
                let escaped =
                    EXPRESSION.seq(character.encode_utf16(&mut units).iter().map(|unit| {
                        let encoded = format!("{unit:04x}");
                        EXPRESSION.seq(std::iter::once(EXPRESSION.term("\\u")).chain(
                            encoded.chars().map(|digit| {
                                if digit.is_ascii_alphabetic() {
                                    EXPRESSION.char_class(
                                        false,
                                        [
                                            CharClassItem::Char(digit),
                                            CharClassItem::Char(digit.to_ascii_uppercase()),
                                        ],
                                    )
                                } else {
                                    EXPRESSION.term(digit.to_string())
                                }
                            }),
                        ))
                    }));
                let mut alternatives = vec![EXPRESSION.term(&text[1..text.len() - 1]), escaped];
                if character == '/' {
                    alternatives.push(EXPRESSION.term("\\/"));
                }
                EXPRESSION.choice_unordered(alternatives)
            }))
            .chain(std::iter::once(EXPRESSION.term("\""))),
    )
}

fn member(key: Expression, item: Expression) -> Expression {
    EXPRESSION.seq([
        key,
        reference_expression("json_space"),
        EXPRESSION.term(":"),
        reference_expression("json_space"),
        item,
    ])
}

fn property(name: &str, item: Expression) -> Expression {
    member(quoted(name), item)
}

fn bounded_tail(item: Expression, min: usize, max: Option<usize>, has_prefix: bool) -> Expression {
    if max == Some(0) {
        return EXPRESSION.empty();
    }
    if has_prefix {
        return EXPRESSION.repeat(separated(item), min, max);
    }
    let rest = EXPRESSION.repeat(
        separated(item.clone()),
        min.saturating_sub(1),
        max.map(|max| max - 1),
    );
    let body = EXPRESSION.seq([item, rest]);
    if min == 0 { EXPRESSION.opt(body) } else { body }
}

fn constant(value: &Value) -> Expression {
    match value {
        Value::String(value) => quoted(value),
        Value::Array(items) => collection(
            "[",
            "]",
            EXPRESSION.seq(items.iter().enumerate().map(|(index, item)| {
                let entry = constant(item);
                if index == 0 { entry } else { separated(entry) }
            })),
        ),
        Value::Object(members) => collection(
            "{",
            "}",
            EXPRESSION.seq(members.iter().enumerate().map(|(index, (name, item))| {
                let entry = property(name, constant(item));
                if index == 0 { entry } else { separated(entry) }
            })),
        ),
        _ => EXPRESSION.term(value.to_string()),
    }
}

fn safe_range(first: char, last: char) -> bool {
    [('\\', '\\'), ('"', '"'), ('\0', '\x1f')]
        .into_iter()
        .all(|(low, high)| first > high || last < low)
}

fn safe_pattern(expression: &Expression) -> bool {
    match expression {
        Expression::Empty => true,
        Expression::Terminal(value) => value.chars().all(|value| safe_range(value, value)),
        Expression::CharRange(first, last) => safe_range(*first, *last),
        Expression::CharClass {
            negated: false,
            items,
        } => items.iter().all(|item| match *item {
            CharClassItem::Char(value) => safe_range(value, value),
            CharClassItem::Range(first, last) => safe_range(first, last),
        }),
        Expression::Sequence(items)
        | Expression::Choice {
            alternatives: items,
            ..
        } => items.iter().all(safe_pattern),
        Expression::Optional(child)
        | Expression::ZeroOrMore(child)
        | Expression::OneOrMore(child)
        | Expression::Repeat { expr: child, .. } => safe_pattern(child),
        _ => false,
    }
}

fn lower_string_pattern(source: &str) -> Option<Expression> {
    static ATOM: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
        r"\A(\[(?:\\.|[^\\\]])*\]|[^()\[\]{}*+?.|^$\\])(?:(\{([0-9]+)(?:,([0-9]*))?\})|([?+*]))?"
    ).expect("JSON pattern production")
    });
    let mut source = source.strip_prefix('^')?.strip_suffix('$')?;
    let mut terms = Vec::new();
    while !source.is_empty() {
        let parts = ATOM.captures(source)?;
        let suffix = parts.get(5).map_or("", |item| item.as_str());
        let mut term = lower_lark_regex(&format!("{}{suffix}", &parts[1]), "", 0).ok()?;
        if !safe_pattern(&term) {
            return None;
        }
        if parts.get(2).is_some() {
            let min = parts[3].parse().ok()?;
            let max = match parts.get(4).map(|item| item.as_str()) {
                None => Some(min),
                Some("") => None,
                Some(value) => Some(value.parse().ok()?),
            };
            if min > 1024 || max.is_some_and(|max| max > 1024 || max < min) {
                return None;
            }
            term = EXPRESSION.repeat(term, min, max);
        }
        terms.push(term);
        source = &source[parts.get(0)?.end()..];
    }
    Some(EXPRESSION.seq(terms))
}

/// A constrained grammar with explicit residual schema constraints.
#[derive(Clone, Debug)]
pub struct JsonSchemaGrammar {
    /// The shared grammar accepted by the grammar emitters.
    pub grammar: Grammar,
    /// Unsupported keywords and restrictions the grammar cannot fully express.
    pub diagnostics: Vec<JsonSchemaIssue>,
    source: Value,
    original: Grammar,
}

impl JsonSchemaGrammar {
    /// Emits the original schema for the imported grammar, or infers it after edits.
    #[must_use]
    pub fn emit_json_schema(&self) -> super::JsonSchemaEmission {
        if self.grammar == self.original {
            return super::JsonSchemaEmission {
                schema: Some(self.source.clone()),
                report: super::super::EmitReport::default(),
            };
        }
        super::emit_json_schema(&self.grammar)
    }
}

/// Why a schema cannot be imported as a grammar.
#[derive(Clone, Debug)]
pub struct JsonSchemaImportError {
    /// Constraints relevant to the failed import.
    pub diagnostics: Vec<JsonSchemaIssue>,
}

impl std::fmt::Display for JsonSchemaImportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("the JSON Schema has no generatable document")
    }
}

impl std::error::Error for JsonSchemaImportError {}

/// Imports a draft 2020-12 schema into the common grammar algebra.
pub fn import_json_schema(schema: &Value) -> Result<JsonSchemaGrammar, JsonSchemaImportError> {
    static BASE: LazyLock<Grammar> = LazyLock::new(|| {
        parse_grammar_links(include_str!("../../data/json-document/decoder.lino"))
            .expect("shipped JSON decoder grammar")
    });
    let mut importer = Importer {
        root: schema,
        grammar: BASE.clone(),
        compiled: HashMap::new(),
        diagnostics: Analysis::new(schema).issues,
        next: 1,
    };
    let Some(start) = importer.named(schema, "") else {
        return Err(JsonSchemaImportError {
            diagnostics: importer.diagnostics,
        });
    };
    importer.grammar.add_rule(GrammarRule::new(
        "start",
        EXPRESSION.seq([
            reference_expression("json_space"),
            start,
            reference_expression("json_space"),
        ]),
    ));
    let mut reachable = BTreeSet::new();
    let mut pending = BTreeSet::from([String::from("start")]);
    while let Some(name) = pending.pop_first() {
        if reachable.contains(&name) {
            continue;
        }
        if let Some(rule) = importer.grammar.rule(&name) {
            rule.expr().collect_nonterminals(&mut pending);
        }
        reachable.insert(name);
    }
    let mut grammar = Grammar::default();
    grammar.set_start("start");
    for rule in importer.grammar.rules() {
        if reachable.contains(rule.name()) {
            grammar.add_rule(rule.clone());
        }
    }
    Ok(JsonSchemaGrammar {
        original: grammar.clone(),
        grammar,
        diagnostics: importer.diagnostics,
        source: schema.clone(),
    })
}

struct Member {
    required: bool,
    expression: Expression,
}

struct ObjectMembers<'members> {
    fields: &'members [Member],
    additional: Option<Expression>,
    group: usize,
}

fn append_member(item: &Expression, started: bool) -> Expression {
    if started {
        separated(item.clone())
    } else {
        item.clone()
    }
}

struct Importer<'schema> {
    root: &'schema Value,
    grammar: Grammar,
    compiled: HashMap<*const Value, String>,
    diagnostics: Vec<JsonSchemaIssue>,
    next: usize,
}

impl<'schema> Importer<'schema> {
    fn report(&mut self, pointer: &str, keyword: &str, message: &str) {
        self.diagnostics.push(JsonSchemaIssue {
            keyword: keyword.to_owned(),
            pointer: pointer_child(pointer, keyword),
            message: message.to_owned(),
        });
    }

    fn next(&mut self) -> usize {
        let next = self.next;
        self.next += 1;
        next
    }

    fn named(&mut self, schema: &'schema Value, pointer: &str) -> Option<Expression> {
        let identity = std::ptr::from_ref(schema);
        if let Some(name) = self.compiled.get(&identity) {
            return Some(reference_expression(name));
        }
        let name = format!("schema_{}", self.next());
        self.compiled.insert(identity, name.clone());
        let Some(expression) = self.lower(schema, pointer) else {
            self.compiled.remove(&identity);
            return None;
        };
        self.grammar.add_rule(GrammarRule::new(&name, expression));
        Some(reference_expression(&name))
    }

    fn lower(&mut self, schema: &'schema Value, pointer: &str) -> Option<Expression> {
        match schema {
            Value::Bool(true) => return Some(reference_expression("value")),
            Value::Object(_) => {}
            _ => return None,
        }
        let combination = schema["anyOf"]
            .as_array()
            .or_else(|| schema["oneOf"].as_array());
        let constants = finite_constants(schema).or_else(|| {
            let branches = combination?;
            branches.iter().try_fold(Vec::new(), |mut values, branch| {
                values.extend(finite_constants(branch)?);
                Some(values)
            })
        });
        if let Some(constants) = constants {
            let entries: Vec<_> = constants
                .into_iter()
                .filter(|value| super::validation::accepts_constant(self.root, schema, value))
                .map(constant)
                .collect();
            return (!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries));
        }
        if let Some(name) = schema["$ref"].as_str()
            && let Some(target) = reference(self.root, name)
        {
            if schema
                .as_object()?
                .keys()
                .any(|key| !reference_keyword(key))
            {
                self.report(
                    pointer,
                    "$ref",
                    "Sibling reference assertions are checked during document validation.",
                );
            }
            return self.named(target, name.strip_prefix('#')?);
        }
        for keyword in ["anyOf", "oneOf"] {
            if let Some(branches) = schema[keyword].as_array() {
                if schema
                    .as_object()?
                    .keys()
                    .any(|key| key != keyword && !reference_keyword(key))
                {
                    self.report(pointer, keyword, "Sibling assertions on alternatives are checked during document validation.");
                }
                if keyword == "oneOf" {
                    self.report(
                        pointer,
                        keyword,
                        "Exclusive branch selection is checked during document validation.",
                    );
                }
                let location = pointer_child(pointer, keyword);
                let entries: Vec<_> = branches
                    .iter()
                    .enumerate()
                    .filter_map(|(index, child)| {
                        self.named(child, &pointer_child(&location, &index.to_string()))
                    })
                    .collect();
                return (!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries));
            }
        }
        let default_types = ["object", "array", "string", "number", "boolean", "null"];
        let declared = schema.get("type").map(types);
        let names = declared.map_or_else(
            || default_types.to_vec(),
            |types| types.iter().filter_map(Value::as_str).collect(),
        );
        let expressions: Vec<_> = names
            .into_iter()
            .filter_map(|name| match name {
                "object" => self.object(schema, pointer),
                "array" => self.array(schema, pointer),
                "string" => Some(self.string(schema, pointer)),
                "integer" | "number" => self.number(name, schema, pointer),
                "boolean" | "null" => Some(reference_expression(name)),
                _ => None,
            })
            .collect();
        (!expressions.is_empty()).then(|| EXPRESSION.choice_unordered(expressions))
    }

    fn object(&mut self, schema: &'schema Value, pointer: &str) -> Option<Expression> {
        let required: HashSet<_> = schema["required"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        let fields = schema["properties"].as_object();
        let mut keys: Vec<_> = fields
            .into_iter()
            .flat_map(serde_json::Map::keys)
            .map(String::as_str)
            .collect();
        if let Some(names) = schema["required"].as_array() {
            for key in names.iter().filter_map(Value::as_str) {
                if !keys.contains(&key) {
                    keys.push(key);
                }
            }
        }
        let additional = schema
            .get("additionalProperties")
            .unwrap_or(&Value::Bool(true));
        if keys.is_empty() {
            let body = match additional {
                Value::Bool(false) => EXPRESSION.empty(),
                Value::Bool(true) => return Some(reference_expression("object")),
                _ => self
                    .named(additional, &pointer_child(pointer, "additionalProperties"))
                    .map_or_else(
                        || EXPRESSION.empty(),
                        |item| {
                            bounded_tail(
                                member(reference_expression("string"), item),
                                0,
                                None,
                                false,
                            )
                        },
                    ),
            };
            return Some(collection("{", "}", body));
        }
        let additional_member = self
            .named(additional, &pointer_child(pointer, "additionalProperties"))
            .and_then(|child| {
                let lengths: Vec<_> = keys.iter().map(|key| key.encode_utf16().count()).collect();
                if lengths.iter().sum::<usize>() > 1024
                    || lengths.iter().any(|length| *length > 128)
                {
                    self.report(pointer, "additionalProperties", "The decoder bounds declared key spellings for additional properties. Validation checks all permitted names.");
                    return None;
                }
                let name = format!("additional_property_{}", self.next());
                let expression = member(additional_property_name(&keys), child);
                self.grammar.add_rule(GrammarRule::new(&name, expression));
                Some(reference_expression(&name))
            });
        let mut members = Vec::new();
        for key in keys {
            let child = fields
                .and_then(|fields| fields.get(key))
                .unwrap_or(additional);
            let is_required = required.contains(key);
            if let Some(item) = self.named(
                child,
                &pointer_child(&pointer_child(pointer, "properties"), key),
            ) {
                members.push(Member {
                    required: is_required,
                    expression: property(key, item),
                });
            } else if is_required {
                return None;
            }
        }
        let members = ObjectMembers {
            fields: &members,
            additional: additional_member,
            group: self.next(),
        };
        let mut states = HashMap::new();
        let body = if members.fields.len() > 12 {
            self.report(pointer, "properties", "The decoder emits large objects in schema field order. Validation accepts all field orders.");
            self.ordered_members(&members, 0, false, &mut states)
        } else {
            let required = members
                .fields
                .iter()
                .enumerate()
                .filter(|(_, member)| member.required)
                .fold(0, |bits, (index, _)| bits | ((1_usize) << index));
            self.unordered_members(&members, 0, false, required, &mut states)
        };
        Some(collection("{", "}", body))
    }

    fn ordered_members(
        &mut self,
        members: &ObjectMembers<'_>,
        index: usize,
        started: bool,
        states: &mut HashMap<usize, String>,
    ) -> Expression {
        let state = index * 2 + usize::from(started);
        if let Some(name) = states.get(&state) {
            return reference_expression(name);
        }
        let name = format!("object_{}_{index}_{}", members.group, usize::from(started));
        states.insert(state, name.clone());
        let mut alternatives = Vec::new();
        if let Some(additional) = &members.additional {
            alternatives.push(EXPRESSION.seq([
                append_member(additional, started),
                self.ordered_members(members, index, true, states),
            ]));
        }
        if let Some(member) = members.fields.get(index) {
            alternatives.push(EXPRESSION.seq([
                append_member(&member.expression, started),
                self.ordered_members(members, index + 1, true, states),
            ]));
            if !member.required {
                alternatives.push(self.ordered_members(members, index + 1, started, states));
            }
        } else {
            alternatives.push(EXPRESSION.empty());
        }
        self.grammar.add_rule(GrammarRule::new(
            &name,
            EXPRESSION.choice_unordered(alternatives),
        ));
        reference_expression(&name)
    }

    fn unordered_members(
        &mut self,
        members: &ObjectMembers<'_>,
        seen: usize,
        started: bool,
        required: usize,
        states: &mut HashMap<usize, String>,
    ) -> Expression {
        let state = seen * 2 + usize::from(started);
        if let Some(name) = states.get(&state) {
            return reference_expression(name);
        }
        let name = format!("object_{}_{seen}_{}", members.group, usize::from(started));
        states.insert(state, name.clone());
        let mut alternatives = Vec::new();
        if seen & required == required {
            alternatives.push(EXPRESSION.empty());
        }
        if let Some(additional) = &members.additional {
            alternatives.push(EXPRESSION.seq([
                append_member(additional, started),
                self.unordered_members(members, seen, true, required, states),
            ]));
        }
        for (index, member) in members.fields.iter().enumerate() {
            let bit = 1_usize << index;
            if seen & bit == 0 {
                alternatives.push(EXPRESSION.seq([
                    append_member(&member.expression, started),
                    self.unordered_members(members, seen | bit, true, required, states),
                ]));
            }
        }
        self.grammar.add_rule(GrammarRule::new(
            &name,
            EXPRESSION.choice_unordered(alternatives),
        ));
        reference_expression(&name)
    }

    fn array(&mut self, schema: &'schema Value, pointer: &str) -> Option<Expression> {
        let location = pointer_child(pointer, "prefixItems");
        let prefix: Vec<_> = schema["prefixItems"]
            .as_array()
            .into_iter()
            .flatten()
            .enumerate()
            .map(|(index, item)| self.named(item, &pointer_child(&location, &index.to_string())))
            .collect();
        let tail = self.named(
            schema.get("items").unwrap_or(&Value::Bool(true)),
            &pointer_child(pointer, "items"),
        );
        let min = schema["minItems"]
            .as_u64()
            .map_or(Some(0), |value| usize::try_from(value).ok())?;
        let max = schema["maxItems"]
            .as_u64()
            .and_then(|value| usize::try_from(value).ok());
        if max.is_some_and(|max| max < min) {
            return None;
        }
        let mut alternatives = Vec::new();
        let mut members = Vec::new();
        for length in 0..=max.unwrap_or(prefix.len()).min(prefix.len()) {
            if length == prefix.len()
                && let Some(item) = &tail
            {
                let rest = bounded_tail(
                    item.clone(),
                    min.saturating_sub(length),
                    max.map(|limit| limit - length),
                    length != 0,
                );
                alternatives
                    .push(EXPRESSION.seq(members.iter().cloned().chain(std::iter::once(rest))));
                break;
            }
            if length >= min {
                alternatives.push(EXPRESSION.seq(members.clone()));
            }
            let Some(Some(item)) = prefix.get(length) else {
                break;
            };
            members.push(if length == 0 {
                item.clone()
            } else {
                separated(item.clone())
            });
        }
        (!alternatives.is_empty())
            .then(|| collection("[", "]", EXPRESSION.choice_unordered(alternatives)))
    }

    fn string(&mut self, schema: &Value, pointer: &str) -> Expression {
        let pattern = schema.get("pattern");
        if let Some(format) = schema["format"].as_str() {
            if formats(format).is_some() && pattern.is_none() {
                self.report(pointer, "format", "The decoder generates direct character spellings in the supported format. Validation also checks escaped strings.");
                return EXPRESSION.nt(format!("json_format_{}", format.replace('-', "_")));
            }
            self.report(pointer, "format", "The format constraint combined with a pattern is checked during document validation.");
        }
        if let Some(pattern) = pattern.and_then(Value::as_str) {
            if let Some(expression) = lower_string_pattern(pattern) {
                self.report(pointer, "pattern", "The decoder generates direct character spellings. Validation also accepts equivalent JSON escapes.");
                return EXPRESSION.seq([EXPRESSION.term("\""), expression, EXPRESSION.term("\"")]);
            }
            self.report(
                pointer,
                "pattern",
                "This pattern is checked during document validation.",
            );
        }
        reference_expression("string")
    }

    fn number(&mut self, name: &str, schema: &Value, pointer: &str) -> Option<Expression> {
        let limits: Vec<_> = keywords("limit")
            .filter(|keyword| schema.get(keyword).is_some())
            .collect();
        if name == "integer" {
            self.report(pointer, "type", "The decoder uses integer spelling. Validation also accepts equivalent decimal or exponent spellings.");
            if !limits.is_empty() {
                let minimum = schema["minimum"]
                    .as_f64()
                    .unwrap_or(f64::NEG_INFINITY)
                    .ceil()
                    .max(
                        schema["exclusiveMinimum"]
                            .as_f64()
                            .unwrap_or(f64::NEG_INFINITY)
                            .floor()
                            + 1.0,
                    );
                let maximum = schema["maximum"]
                    .as_f64()
                    .unwrap_or(f64::INFINITY)
                    .floor()
                    .min(
                        schema["exclusiveMaximum"]
                            .as_f64()
                            .unwrap_or(f64::INFINITY)
                            .ceil()
                            - 1.0,
                    );
                let safe = |value: f64| value.abs() <= 9_007_199_254_740_991.0;
                if (safe(minimum) || minimum == f64::NEG_INFINITY)
                    && (safe(maximum) || maximum == f64::INFINITY)
                {
                    if minimum > maximum {
                        return None;
                    }
                    return Some(signed_integer_range(integer(minimum), integer(maximum)));
                }
            }
        }
        if name == "number" && !limits.is_empty() {
            match signed_decimal_range(schema) {
                DecimalRange::Allowed(decimal) => {
                    self.report(pointer, "type", "The bounded decoder uses decimal spelling. Validation also accepts equivalent exponent spellings.");
                    return Some(decimal);
                }
                DecimalRange::Empty => return None,
                DecimalRange::Unsupported => {}
            }
        }
        for keyword in limits {
            self.report(
                pointer,
                keyword,
                "This bound is checked during document validation.",
            );
        }
        Some(reference_expression(name))
    }
}

fn integer(value: f64) -> Option<i64> {
    value.is_finite().then(|| {
        format!("{value:.0}")
            .parse()
            .expect("safe integer spelling")
    })
}

fn reference_keyword(keyword: &str) -> bool {
    matches!(keyword, "$ref" | "$defs") || keywords("annotation").any(|name| name == keyword)
}

fn finite_constants(schema: &Value) -> Option<Vec<&Value>> {
    if schema == &Value::Bool(false) {
        Some(Vec::new())
    } else if let Some(value) = schema.get("const") {
        Some(vec![value])
    } else {
        schema["enum"]
            .as_array()
            .map(|values| values.iter().collect())
    }
}
