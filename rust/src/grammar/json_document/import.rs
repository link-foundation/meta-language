use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::sync::LazyLock;

use serde_json::Value;

use super::super::{
    CharClassItem, ExprBuilder, Grammar, GrammarExpr, GrammarRule, parse_grammar_links,
};
use super::integer_policy::exclusive_integer_boundary;
use super::number_spelling::NumberSpelling;
use super::numeric_ranges::{DecimalRange, signed_decimal_range, signed_integer_range};
use super::patterns::{PatternLowering, any_json_pattern_character, lower_json_string_pattern};
use super::property_names::additional_property_name;
use super::schema::{Analysis, formats, keywords, reference, types};
use super::string_intersection::{StringIntersection, intersect_json_string_expressions};
use super::tag_policy::string_sets_are_disjoint;
use super::{JsonSchemaIssue, pointer_child};

type Expression = GrammarExpr;
const EXPRESSION: ExprBuilder = ExprBuilder;

const INTEGER_TYPE: u8 = 1 << 3;
const FRACTIONAL_TYPE: u8 = 1 << 4;
const NUMBER_TYPE: u8 = INTEGER_TYPE | FRACTIONAL_TYPE;
const JSON_TYPES: [(&str, u8); 7] = [
    ("object", 1),
    ("array", 1 << 1),
    ("string", 1 << 2),
    ("number", NUMBER_TYPE),
    ("integer", INTEGER_TYPE),
    ("boolean", 1 << 5),
    ("null", 1 << 6),
];
const ALL_JSON_TYPES: u8 = (1 << 7) - 1;

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
        pattern_rules: Vec::new(),
    };
    importer.grammar.add_rule(GrammarRule::new(
        "json_pattern_character",
        any_json_pattern_character(),
    ));
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
    compiled: HashMap<(*const Value, u8), String>,
    diagnostics: Vec<JsonSchemaIssue>,
    next: usize,
    pattern_rules: Vec<GrammarExpr>,
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
        self.named_with_types(schema, pointer, ALL_JSON_TYPES)
    }

    fn named_with_types(
        &mut self,
        schema: &'schema Value,
        pointer: &str,
        allowed_types: u8,
    ) -> Option<Expression> {
        let membership = allowed_types & declared_type_membership(schema);
        if membership == 0 {
            return None;
        }
        let identity = (std::ptr::from_ref(schema), membership);
        if let Some(name) = self.compiled.get(&identity) {
            return Some(reference_expression(name));
        }
        let name = format!("schema_{}", self.next());
        self.compiled.insert(identity, name.clone());
        let Some(expression) = self.lower(schema, pointer, membership) else {
            self.compiled.remove(&identity);
            return None;
        };
        self.grammar.add_rule(GrammarRule::new(&name, expression));
        Some(reference_expression(&name))
    }

    fn lower(
        &mut self,
        schema: &'schema Value,
        pointer: &str,
        allowed_types: u8,
    ) -> Option<Expression> {
        match schema {
            Value::Bool(true) if allowed_types == ALL_JSON_TYPES => {
                return Some(reference_expression("value"));
            }
            Value::Bool(true) | Value::Object(_) => {}
            _ => return None,
        }
        let combination = schema["anyOf"]
            .as_array()
            .or_else(|| schema["oneOf"].as_array());
        let reference_has_siblings = schema.get("$ref").is_some()
            && schema
                .as_object()?
                .keys()
                .any(|key| !reference_keyword(key));
        let constants = finite_constants(schema).or_else(|| {
            let branches = combination?;
            branches.iter().try_fold(Vec::new(), |mut values, branch| {
                values.extend(finite_constants(branch)?);
                Some(values)
            })
        });
        let candidates = constants
            .map(|constants| {
                let keyword = ["const", "enum", "anyOf", "oneOf"]
                    .into_iter()
                    .find(|key| schema.get(*key).is_some())
                    .expect("each finite schema belongs to an assertion");
                (constants, keyword)
            })
            .or_else(|| {
                if reference_has_siblings || combination.is_some() {
                    finite_schema_candidates(self.root, schema, &mut HashSet::new(), &mut 16_384, 0)
                } else {
                    None
                }
            });
        if let Some((constants, keyword)) = candidates {
            let root = self.root;
            let entries: Vec<_> = constants
                .into_iter()
                .filter(|value| {
                    allowed_types & value_type_membership(value) != 0
                        && super::validation::accepts_constant(root, schema, value)
                })
                .map(|value| self.constant(value, pointer, keyword))
                .collect();
            return (!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries));
        }
        if let Some(name) = schema["$ref"].as_str()
            && let Some(target) = reference(self.root, name)
        {
            if schema
                .as_object()?
                .keys()
                .any(|key| key != "type" && !reference_keyword(key))
            {
                self.report(
                    pointer,
                    "$ref",
                    "Sibling reference assertions are checked during document validation.",
                );
            }
            return self.named_with_types(target, name.strip_prefix('#')?, allowed_types);
        }
        for keyword in ["anyOf", "oneOf"] {
            if let Some(branches) = schema[keyword].as_array() {
                if schema
                    .as_object()?
                    .keys()
                    .any(|key| key != "type" && key != keyword && !reference_keyword(key))
                {
                    self.report(pointer, keyword, "Sibling assertions on alternatives are checked during document validation.");
                }
                if keyword == "oneOf"
                    && !branches_use_disjoint_types(branches.iter(), allowed_types)
                    && !branches_use_disjoint_required_tags(branches, allowed_types)
                {
                    if let Some(integer) = exclusive_integer_range(branches) {
                        if integer.is_some() {
                            self.report(pointer, keyword, "The exclusive integer decoder uses integer spelling. Validation also accepts equivalent decimal or exponent spellings.");
                        }
                        return integer;
                    }
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
                        self.named_with_types(
                            child,
                            &pointer_child(&location, &index.to_string()),
                            allowed_types,
                        )
                    })
                    .collect();
                return (!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries));
            }
        }
        let expressions: Vec<_> = JSON_TYPES
            .into_iter()
            .filter(|&(name, membership)| {
                allowed_types & membership == membership
                    && (name != "integer" || allowed_types & FRACTIONAL_TYPE == 0)
            })
            .filter_map(|(name, _)| match name {
                "object" => self.object(schema, pointer),
                "array" => self.array(schema, pointer),
                "string" => self.string(schema, pointer),
                "integer" | "number" => self.number(name, schema, pointer),
                "boolean" | "null" => Some(reference_expression(name)),
                _ => None,
            })
            .collect();
        (!expressions.is_empty()).then(|| EXPRESSION.choice_unordered(expressions))
    }

    fn constant(&mut self, value: &Value, pointer: &str, keyword: &str) -> Expression {
        match value {
            Value::String(value) => quoted(value),
            Value::Array(items) => collection(
                "[",
                "]",
                EXPRESSION.seq(items.iter().enumerate().map(|(index, item)| {
                    let entry = self.constant(item, pointer, keyword);
                    if index == 0 { entry } else { separated(entry) }
                })),
            ),
            Value::Object(fields) => {
                let members: Vec<_> = fields
                    .iter()
                    .map(|(name, item)| Member {
                        required: true,
                        expression: property(name, self.constant(item, pointer, keyword)),
                    })
                    .collect();
                self.object_members(&members, None, pointer, keyword)
            }
            _ => EXPRESSION.term(value.to_string()),
        }
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
        Some(self.object_members(&members, additional_member, pointer, "properties"))
    }

    fn object_members(
        &mut self,
        fields: &[Member],
        additional: Option<Expression>,
        pointer: &str,
        keyword: &str,
    ) -> Expression {
        let members = ObjectMembers {
            fields,
            additional,
            group: self.next(),
        };
        let mut states = HashMap::new();
        let body = if members.fields.len() > 12 {
            self.report(pointer, keyword, "The decoder emits large objects in schema field order. Validation accepts all field orders.");
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
        collection("{", "}", body)
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

    fn pattern_rule(&mut self, expression: GrammarExpr) -> GrammarExpr {
        if let Some(index) = self
            .pattern_rules
            .iter()
            .position(|entry| expression == *entry)
        {
            return EXPRESSION.nt(format!("json_pattern_part_{}", index + 1));
        }
        self.pattern_rules.push(expression.clone());
        let name = format!("json_pattern_part_{}", self.pattern_rules.len());
        self.grammar.add_rule(GrammarRule::new(&name, expression));
        EXPRESSION.nt(name)
    }

    fn string(&mut self, schema: &Value, pointer: &str) -> Option<Expression> {
        let format = schema["format"]
            .as_str()
            .filter(|name| formats(name).is_some())
            .map(|name| EXPRESSION.nt(format!("json_format_{}", name.replace('-', "_"))));
        if let Some(pattern) = schema.get("pattern").and_then(Value::as_str) {
            match lower_json_string_pattern(pattern, |expression| self.pattern_rule(expression)) {
                Some(PatternLowering::Match(expression)) => {
                    let pattern =
                        EXPRESSION.seq([EXPRESSION.term("\""), expression, EXPRESSION.term("\"")]);
                    let Some(format) = &format else {
                        return Some(pattern);
                    };
                    let rule_prefix = format!("json_string_intersection_{}", self.next());
                    match intersect_json_string_expressions(&self.grammar, format, &pattern, &rule_prefix) {
                        Some(StringIntersection::Match(expression, rules)) => {
                            for rule in rules {
                                self.grammar.add_rule(rule);
                            }
                            return Some(expression);
                        }
                        Some(StringIntersection::Impossible) => return None,
                        None => self.report(pointer, "pattern", "The format and pattern intersection exceeds the decoder work bound. The pattern is checked during document validation."),
                    }
                }
                Some(PatternLowering::Impossible) => return None,
                None => self.report(
                    pointer,
                    "pattern",
                    "This pattern is checked during document validation.",
                ),
            }
        }
        Some(format.unwrap_or_else(|| reference_expression("string")))
    }

    fn number(&mut self, name: &str, schema: &Value, pointer: &str) -> Option<Expression> {
        let limits: Vec<_> = keywords("limit")
            .filter(|keyword| schema.get(keyword).is_some())
            .collect();
        if name == "integer" {
            self.report(pointer, "type", "The decoder uses integer spelling. Validation also accepts equivalent decimal or exponent spellings.");
            if !limits.is_empty() {
                let (minimum, maximum) = integer_bounds(schema);
                if (safe_integer_bound(minimum) || minimum == f64::NEG_INFINITY)
                    && (safe_integer_bound(maximum) || maximum == f64::INFINITY)
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

fn safe_integer_bound(value: f64) -> bool {
    value.abs() <= 9_007_199_254_740_991.0
}

fn integer_bounds(schema: &Value) -> (f64, f64) {
    let number = |key: &str, fallback| schema[key].as_f64().unwrap_or(fallback);
    let minimum = number("minimum", f64::NEG_INFINITY)
        .ceil()
        .max(number("exclusiveMinimum", f64::NEG_INFINITY).floor() + 1.0);
    let maximum = number("maximum", f64::INFINITY)
        .floor()
        .min(number("exclusiveMaximum", f64::INFINITY).ceil() - 1.0);
    (minimum, maximum)
}

fn exclusive_integer_range(branches: &[Value]) -> Option<Option<Expression>> {
    let mut boundaries = BTreeMap::new();
    let mut coverage = 0.0;
    for branch in branches {
        if branch == &Value::Bool(false) {
            continue;
        }
        if !matches!(types(branch.get("type")?), [value] if value.as_str() == Some("integer"))
            || branch.get("$ref").is_some()
            || branch.as_object()?.keys().any(|key| {
                key != "type"
                    && !reference_keyword(key)
                    && !keywords("limit").any(|name| name == key.as_str())
            })
        {
            return None;
        }
        let (first, last) = integer_bounds(branch);
        if first > last {
            continue;
        }
        if first == f64::NEG_INFINITY {
            coverage += 1.0;
        } else {
            if !safe_integer_bound(first) || !safe_integer_bound(first - 1.0) {
                return None;
            }
            *boundaries.entry(integer(first)?).or_insert(0.0) += 1.0;
        }
        if last != f64::INFINITY {
            if !safe_integer_bound(last) || !safe_integer_bound(last + 1.0) {
                return None;
            }
            *boundaries.entry(integer(last + 1.0)?).or_insert(0.0) -= 1.0;
        }
    }
    let mut expressions = Vec::new();
    let mut first = None;
    for (point, change) in boundaries {
        let next = coverage + change;
        let boundary = exclusive_integer_boundary(coverage, next);
        if boundary < 0.0 {
            expressions.push(signed_integer_range(first, Some(point - 1)));
        }
        if boundary > 0.0 {
            first = Some(point);
        }
        coverage = next;
    }
    if coverage == 1.0 {
        expressions.push(signed_integer_range(first, None));
    }
    Some((!expressions.is_empty()).then(|| EXPRESSION.choice_unordered(expressions)))
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

fn finite_schema_candidates<'schema>(
    root: &'schema Value,
    schema: &'schema Value,
    active: &mut HashSet<*const Value>,
    work: &mut usize,
    depth: usize,
) -> Option<(Vec<&'schema Value>, &'static str)> {
    *work = work.checked_sub(1)?;
    let identity = std::ptr::from_ref(schema);
    if depth > 128 || active.contains(&identity) {
        return None;
    }
    if let Some(values) = finite_constants(schema) {
        let keyword = if schema.get("const").is_some() {
            "const"
        } else {
            "enum"
        };
        return Some((values, keyword));
    }
    let rules = schema.as_object()?;
    active.insert(identity);
    let result = (|| {
        if let Some(target) = rules
            .get("$ref")
            .and_then(Value::as_str)
            .and_then(|name| reference(root, name))
            && let Some((values, _)) =
                finite_schema_candidates(root, target, active, work, depth + 1)
        {
            return Some((values, "$ref"));
        }
        for keyword in ["anyOf", "oneOf"] {
            let Some(branches) = rules.get(keyword).and_then(Value::as_array) else {
                continue;
            };
            let values = branches.iter().try_fold(Vec::new(), |mut values, child| {
                let (candidates, _) =
                    finite_schema_candidates(root, child, active, work, depth + 1)?;
                *work = work.checked_sub(candidates.len())?;
                values.extend(candidates);
                Some(values)
            });
            if let Some(values) = values {
                return Some((values, keyword));
            }
        }
        None
    })();
    active.remove(&identity);
    result
}

fn declared_type_membership(schema: &Value) -> u8 {
    schema.get("type").map_or(ALL_JSON_TYPES, |declared| {
        types(declared)
            .iter()
            .filter_map(Value::as_str)
            .fold(0, |membership, name| membership | type_membership(name))
    })
}

fn type_membership(name: &str) -> u8 {
    JSON_TYPES
        .iter()
        .find_map(|&(keyword, membership)| (keyword == name).then_some(membership))
        .unwrap_or(0)
}

fn value_type_membership(value: &Value) -> u8 {
    type_membership(match value {
        Value::Array(_) => "array",
        Value::Bool(_) => "boolean",
        Value::Null => "null",
        Value::Number(number) => {
            return if NumberSpelling::new(&number.to_string()).is_integer() {
                INTEGER_TYPE
            } else {
                FRACTIONAL_TYPE
            };
        }
        Value::Object(_) => "object",
        Value::String(_) => "string",
    })
}

fn branches_use_disjoint_types<'schema>(
    branches: impl ExactSizeIterator<Item = &'schema Value>,
    allowed_types: u8,
) -> bool {
    if branches.len() < 2 {
        return true;
    }
    let mut seen = 0;
    for branch in branches {
        if !branch.is_object() && !branch.is_boolean() {
            return false;
        }
        let declared = allowed_types & declared_type_membership(branch);
        let membership = finite_constants(branch).map_or(declared, |constants| {
            declared
                & constants
                    .into_iter()
                    .fold(0, |types, value| types | value_type_membership(value))
        });
        if seen & membership != 0 {
            return false;
        }
        seen |= membership;
    }
    true
}

fn required_string_tags(schema: &Value) -> HashMap<&str, Vec<&str>> {
    if !matches!(schema.get("type").map(types), Some([value]) if value.as_str() == Some("object"))
        || ["$ref", "anyOf", "oneOf"]
            .iter()
            .any(|key| schema.get(*key).is_some())
    {
        return HashMap::new();
    }
    let Some(required) = schema["required"].as_array() else {
        return HashMap::new();
    };
    required
        .iter()
        .filter_map(|key| {
            let key = key.as_str()?;
            let values = finite_constants(schema["properties"].get(key)?)?
                .into_iter()
                .map(Value::as_str)
                .collect::<Option<Vec<_>>>()?;
            Some((key, values))
        })
        .collect()
}

fn branches_use_disjoint_required_tags(branches: &[Value], allowed_types: u8) -> bool {
    let tags: Vec<_> = branches.iter().map(required_string_tags).collect();
    branches.iter().enumerate().all(|(index, branch)| {
        branches[..index]
            .iter()
            .enumerate()
            .all(|(other, previous)| {
                branches_use_disjoint_types([branch, previous].into_iter(), allowed_types)
                    || tags[index].iter().any(|(key, values)| {
                        tags[other].get(key).is_some_and(|other_values| {
                            string_sets_are_disjoint(values, other_values)
                        })
                    })
            })
    })
}
