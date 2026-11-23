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
use super::string_intersection::{
    StringIntersection, intersect_json_string_expressions, select_exclusive_json_string_expressions,
};
use super::tag_policy::string_sets_are_disjoint;
use super::{JsonSchemaIssue, pointer_child};

type Expression = GrammarExpr;
const EXPRESSION: ExprBuilder = ExprBuilder;
static UNRESTRICTED_SCHEMA: Value = Value::Bool(true);

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
static ASSERTION_TYPES: LazyLock<HashMap<&'static str, u8>> = LazyLock::new(|| {
    [
        ("object", "objectAssertion"),
        ("array", "arrayAssertion"),
        ("string", "stringAssertion"),
        ("number", "limit"),
    ]
    .into_iter()
    .flat_map(|(name, category)| {
        keywords(category).map(move |keyword| (keyword, type_membership(name)))
    })
    .collect()
});

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

enum CollectionConstraints<'schema> {
    Object {
        properties: Vec<(&'schema str, &'schema Value)>,
        additional: &'schema Value,
        required: Vec<&'schema str>,
    },
    Array {
        layout: &'schema Value,
        minimum: u64,
        maximum: Option<u64>,
    },
}

enum ConjunctionEntry<'schema> {
    Expression(Expression),
    Number(&'static str, Value),
    Collection(CollectionConstraints<'schema>),
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
                if let Some(expression) = self.referenced_expression(schema, allowed_types, pointer)
                {
                    return expression;
                }
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
                let has_sibling_assertions = schema
                    .as_object()?
                    .keys()
                    .any(|key| key != "type" && key != keyword && !reference_keyword(key));
                let overlaps = keyword == "oneOf"
                    && !branches_use_disjoint_types(self.root, branches.iter(), allowed_types)
                    && !branches_use_disjoint_required_tags(self.root, branches, allowed_types);
                if overlaps || has_sibling_assertions {
                    if let Some(expression) =
                        self.alternative_string_expression(schema, branches, allowed_types, keyword)
                    {
                        return expression;
                    }
                    if let Some(expression) = self.alternative_numeric_expression(
                        schema,
                        branches,
                        allowed_types,
                        keyword,
                        pointer,
                    ) {
                        return expression;
                    }
                    if !overlaps
                        && let Some(expression) = self.alternative_conjunction_expression(
                            schema,
                            branches,
                            allowed_types,
                            keyword,
                            pointer,
                        )
                    {
                        return expression;
                    }
                }
                if has_sibling_assertions {
                    self.report(pointer, keyword, "Sibling assertions on alternatives are checked during document validation.");
                }
                if overlaps {
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
        self.object_with_properties(
            property_schemas(schema).collect(),
            additional_property_schema(schema),
            required_property_names(schema),
            pointer,
        )
    }

    fn object_with_properties(
        &mut self,
        fields: Vec<(&'schema str, &'schema Value)>,
        additional: &'schema Value,
        required: impl Iterator<Item = &'schema str>,
        pointer: &str,
    ) -> Option<Expression> {
        let mut keys: Vec<_> = fields.iter().map(|&(name, _)| name).collect();
        let fields: HashMap<_, _> = fields.into_iter().collect();
        let mut required_names = HashSet::new();
        for key in required {
            if required_names.insert(key) && !fields.contains_key(key) {
                keys.push(key);
            }
        }
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
            let child = fields.get(key).copied().unwrap_or(additional);
            let is_required = required_names.contains(key);
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
        let min = schema["minItems"]
            .as_u64()
            .map_or(Some(0), |value| usize::try_from(value).ok())?;
        let max = schema["maxItems"]
            .as_u64()
            .and_then(|value| usize::try_from(value).ok());
        self.array_with_length(schema, min, max, pointer)
    }

    fn array_with_length(
        &mut self,
        schema: &'schema Value,
        min: usize,
        max: Option<usize>,
        pointer: &str,
    ) -> Option<Expression> {
        if max.is_some_and(|max| max < min) {
            return None;
        }
        if max == Some(0) {
            return Some(collection("[", "]", EXPRESSION.empty()));
        }
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

    fn regular_string_constraints(
        &mut self,
        schema: &Value,
        combination: Option<&str>,
    ) -> Option<Option<Vec<Expression>>> {
        if schema.as_object().is_some_and(|rules| {
            rules.keys().any(|key| {
                Some(key.as_str()) != combination
                    && applies_to_type(key, type_membership("string"))
                    && (key == "$ref"
                        || (!reference_keyword(key)
                            && !matches!(
                                key.as_str(),
                                "type" | "const" | "enum" | "pattern" | "format"
                            )))
            })
        }) {
            return None;
        }
        let mut expressions = Vec::new();
        if let Some(format) = schema["format"].as_str() {
            formats(format)?;
            expressions.push(EXPRESSION.nt(format!("json_format_{}", format.replace('-', "_"))));
        }
        if let Some(pattern) = schema["pattern"].as_str() {
            match lower_json_string_pattern(pattern, |expression| self.pattern_rule(expression))? {
                PatternLowering::Match(expression) => expressions.push(EXPRESSION.seq([
                    EXPRESSION.term("\""),
                    expression,
                    EXPRESSION.term("\""),
                ])),
                PatternLowering::Impossible => return Some(None),
            }
        }
        if let Some(constants) = finite_constants(schema) {
            let strings: Vec<_> = constants
                .into_iter()
                .filter_map(Value::as_str)
                .map(quoted)
                .collect();
            if strings.is_empty() {
                return Some(None);
            }
            expressions.push(EXPRESSION.choice_unordered(strings));
        }
        Some(Some(expressions))
    }

    fn string_branch_product(
        &mut self,
        branches: &[Vec<&Expression>],
    ) -> Option<Option<Expression>> {
        let rule_prefix = format!("json_string_exclusive_{}", self.next());
        match select_exclusive_json_string_expressions(&self.grammar, branches, &rule_prefix)? {
            StringIntersection::Match(expression, rules) => {
                for rule in rules {
                    self.grammar.add_rule(rule);
                }
                Some(Some(expression))
            }
            StringIntersection::Impossible => Some(None),
        }
    }

    fn regular_string_chain(
        &mut self,
        layers: &[&Value],
        combination: Option<&str>,
    ) -> Option<Option<Vec<Expression>>> {
        let mut conjunction = Vec::new();
        for (index, layer) in layers.iter().enumerate() {
            let ignored = if index == 0 {
                combination.or(Some("$ref"))
            } else {
                Some("$ref")
            };
            let Some(expressions) = self.regular_string_constraints(layer, ignored)? else {
                return Some(None);
            };
            conjunction.extend(expressions);
        }
        Some(Some(conjunction))
    }

    fn collection_expression(
        &mut self,
        constraints: CollectionConstraints<'schema>,
        pointer: &str,
    ) -> Option<Expression> {
        match constraints {
            CollectionConstraints::Object {
                properties,
                additional,
                required,
            } => self.object_with_properties(properties, additional, required.into_iter(), pointer),
            CollectionConstraints::Array {
                layout,
                minimum,
                maximum,
            } => self.array_with_length(
                layout,
                usize::try_from(minimum).ok()?,
                maximum.and_then(|max| usize::try_from(max).ok()),
                pointer,
            ),
        }
    }

    fn referenced_expression(
        &mut self,
        schema: &'schema Value,
        allowed_types: u8,
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let layers = reference_layers(self.root, schema)?;
        let entries = self.conjunction_entries(&layers, allowed_types, None)?;
        Some(self.lower_conjunction(entries, pointer))
    }

    fn lower_conjunction(
        &mut self,
        entries: Vec<ConjunctionEntry<'schema>>,
        pointer: &str,
    ) -> Option<Expression> {
        let expressions: Vec<_> = entries
            .into_iter()
            .filter_map(|entry| match entry {
                ConjunctionEntry::Expression(expression) => Some(expression),
                ConjunctionEntry::Number(name, limits) => self.number(name, &limits, pointer),
                ConjunctionEntry::Collection(constraints) => {
                    self.collection_expression(constraints, pointer)
                }
            })
            .collect();
        (!expressions.is_empty()).then(|| EXPRESSION.choice_unordered(expressions))
    }

    fn conjunction_entries(
        &mut self,
        layers: &[&'schema Value],
        allowed_types: u8,
        combination: Option<&str>,
    ) -> Option<Vec<ConjunctionEntry<'schema>>> {
        let membership = layers.iter().fold(allowed_types, |types, layer| {
            possible_type_membership(layer, types)
        });
        if membership & NUMBER_TYPE == FRACTIONAL_TYPE {
            return None;
        }
        let mut entries = Vec::new();
        for (name, mask) in JSON_TYPES {
            if membership & mask != mask || (name == "integer" && membership & FRACTIONAL_TYPE != 0)
            {
                continue;
            }
            let entry = match name {
                "object" | "array" => {
                    ConjunctionEntry::Collection(collection_constraints(layers, mask, combination)?)
                }
                "number" | "integer" => {
                    let (name, limits) = combine_numeric_constraints(layers, mask, combination)?;
                    ConjunctionEntry::Number(name, limits)
                }
                "string" => {
                    let Some(conjunction) = self.regular_string_chain(layers, combination)? else {
                        continue;
                    };
                    let expression = if conjunction.is_empty() {
                        reference_expression(name)
                    } else {
                        let Some(expression) =
                            self.string_branch_product(&[conjunction.iter().collect()])?
                        else {
                            continue;
                        };
                        expression
                    };
                    ConjunctionEntry::Expression(expression)
                }
                _ => {
                    let residual = layers.iter().enumerate().any(|(index, layer)| {
                        layer.as_object().is_some_and(|fields| {
                            fields.keys().any(|key| {
                                active_conjunction_keyword(key, index, combination, mask)
                            })
                        })
                    });
                    if residual {
                        return None;
                    }
                    ConjunctionEntry::Expression(reference_expression(name))
                }
            };
            entries.push(entry);
        }
        Some(entries)
    }

    fn alternative_string_expression(
        &mut self,
        schema: &Value,
        branches: &'schema [Value],
        allowed_types: u8,
        keyword: &str,
    ) -> Option<Option<Expression>> {
        let string = type_membership("string");
        let Some(guard) = self.regular_string_constraints(schema, Some(keyword))? else {
            return Some(None);
        };
        let mut choices = Vec::new();
        for branch in branches {
            if possible_type_membership(branch, allowed_types) == 0 {
                continue;
            }
            let layers = reference_layers(self.root, branch)?;
            let membership = layers.iter().fold(allowed_types, |types, layer| {
                possible_type_membership(layer, types)
            });
            if membership == 0 {
                continue;
            }
            if membership != string {
                return None;
            }
            if let Some(constraints) = self.regular_string_chain(&layers, None)? {
                choices.push(constraints);
            }
        }
        let any_string = reference_expression("string");
        let choices: Vec<_> = choices
            .iter()
            .map(|constraints| {
                if guard.is_empty() && constraints.is_empty() {
                    vec![&any_string]
                } else {
                    guard.iter().chain(constraints).collect()
                }
            })
            .collect();
        if keyword == "oneOf" {
            return self.string_branch_product(&choices);
        }
        // Distribute the common assertions to bound each intersection independently.
        let mut entries = Vec::new();
        for choice in choices {
            if let Some(expression) = self.string_branch_product(&[choice])? {
                entries.push(expression);
            }
        }
        Some((!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries)))
    }

    fn alternative_numeric_expression(
        &mut self,
        schema: &'schema Value,
        branches: &'schema [Value],
        allowed_types: u8,
        keyword: &str,
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let mut alternatives = Vec::new();
        for (index, branch) in branches.iter().enumerate() {
            if possible_type_membership(branch, allowed_types) == 0 {
                continue;
            }
            let mut layers = reference_layers(self.root, branch)?;
            let membership = layers.iter().fold(allowed_types, |types, layer| {
                possible_type_membership(layer, types)
            });
            if membership == 0 {
                continue;
            }
            layers.insert(0, schema);
            let (name, limits) = combine_numeric_constraints(&layers, membership, Some(keyword))?;
            alternatives.push((index, name, limits));
        }
        if keyword == "oneOf" {
            if alternatives.iter().any(|(_, name, _)| *name != "integer") {
                return None;
            }
            let integer = exclusive_integer_range(
                alternatives
                    .iter()
                    .map(|(_, _, limits)| integer_bounds(limits)),
            )?;
            if integer.is_some() {
                self.report(pointer, keyword, "The exclusive integer decoder uses integer spelling. Validation also accepts equivalent decimal or exponent spellings.");
            }
            return Some(integer);
        }
        let location = pointer_child(pointer, keyword);
        let entries: Vec<_> = alternatives
            .into_iter()
            .filter_map(|(index, name, limits)| {
                self.number(name, &limits, &pointer_child(&location, &index.to_string()))
            })
            .collect();
        Some((!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries)))
    }

    fn alternative_conjunction_expression(
        &mut self,
        schema: &'schema Value,
        branches: &'schema [Value],
        allowed_types: u8,
        keyword: &str,
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let mut alternatives = Vec::new();
        for (index, branch) in branches.iter().enumerate() {
            if possible_type_membership(branch, allowed_types) == 0 {
                continue;
            }
            let mut layers = reference_layers(self.root, branch)?;
            layers.insert(0, schema);
            let entries = self.conjunction_entries(&layers, allowed_types, Some(keyword))?;
            alternatives.push((index, entries));
        }
        let location = pointer_child(pointer, keyword);
        let entries: Vec<_> = alternatives
            .into_iter()
            .filter_map(|(index, entries)| {
                self.lower_conjunction(entries, &pointer_child(&location, &index.to_string()))
            })
            .collect();
        Some((!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries)))
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

fn exclusive_integer_range(
    intervals: impl IntoIterator<Item = (f64, f64)>,
) -> Option<Option<Expression>> {
    let mut boundaries = BTreeMap::new();
    let mut coverage = 0.0;
    for (first, last) in intervals {
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

fn applies_to_type(keyword: &str, membership: u8) -> bool {
    ASSERTION_TYPES
        .get(keyword)
        .is_none_or(|&mask| mask & membership != 0)
}

fn active_conjunction_keyword(
    keyword: &str,
    index: usize,
    combination: Option<&str>,
    membership: u8,
) -> bool {
    keyword != "type"
        && !reference_keyword(keyword)
        && !(index == 0 && Some(keyword) == combination)
        && applies_to_type(keyword, membership)
}

fn required_property_names(schema: &Value) -> impl Iterator<Item = &str> {
    schema["required"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
}

fn property_schemas(schema: &Value) -> impl Iterator<Item = (&str, &Value)> {
    schema["properties"]
        .as_object()
        .into_iter()
        .flat_map(|fields| fields.iter().map(|(name, rule)| (name.as_str(), rule)))
}

fn additional_property_schema(schema: &Value) -> &Value {
    schema
        .get("additionalProperties")
        .unwrap_or(&UNRESTRICTED_SCHEMA)
}

fn compatible_property_schema<'schema>(
    schemas: impl Iterator<Item = &'schema Value>,
) -> Option<&'schema Value> {
    let mut rule = &UNRESTRICTED_SCHEMA;
    let mut agrees = true;
    for candidate in schemas {
        match candidate {
            Value::Bool(false) => return Some(candidate),
            Value::Bool(true) => (),
            _ if rule == &UNRESTRICTED_SCHEMA => rule = candidate,
            _ => agrees &= rule == candidate,
        }
    }
    agrees.then_some(rule)
}

fn collection_keywords_are_supported(
    layers: &[&Value],
    membership: u8,
    category: &str,
    combination: Option<&str>,
) -> bool {
    layers.iter().enumerate().all(|(index, layer)| {
        !layer.as_object().is_some_and(|fields| {
            fields.keys().any(|key| {
                active_conjunction_keyword(key, index, combination, membership)
                    && !keywords(category).any(|name| name == key.as_str())
            })
        })
    })
}

fn collection_constraints<'schema>(
    layers: &[&'schema Value],
    membership: u8,
    combination: Option<&str>,
) -> Option<CollectionConstraints<'schema>> {
    if membership == type_membership("object") {
        if !collection_keywords_are_supported(layers, membership, "objectAssertion", combination) {
            return None;
        }
        let additional =
            compatible_property_schema(layers.iter().copied().map(additional_property_schema))?;
        let mut seen = HashSet::new();
        let keys = layers
            .iter()
            .flat_map(|&layer| property_schemas(layer).map(|(name, _)| name))
            .filter(|&name| seen.insert(name));
        let mut properties = Vec::new();
        for name in keys {
            // Defaults apply only in the layers that leave this property undeclared.
            let rule = compatible_property_schema(layers.iter().map(|&layer| {
                layer["properties"]
                    .get(name)
                    .unwrap_or_else(|| additional_property_schema(layer))
            }))?;
            properties.push((name, rule));
        }
        let required = layers
            .iter()
            .flat_map(|&layer| required_property_names(layer))
            .collect();
        return Some(CollectionConstraints::Object {
            properties,
            additional,
            required,
        });
    }
    if membership == type_membership("array") {
        if !collection_keywords_are_supported(layers, membership, "arrayAssertion", combination) {
            return None;
        }
        let maximum = layers
            .iter()
            .filter_map(|layer| {
                let bound = layer["maxItems"].as_u64();
                if layer["items"] == Value::Bool(false) {
                    let prefix = layer["prefixItems"].as_array().map_or(0, Vec::len) as u64;
                    return Some(bound.map_or(prefix, |bound| bound.min(prefix)));
                }
                bound
            })
            .min();
        let mut layouts = layers.iter().copied().filter(|layer| {
            maximum != Some(0)
                && (layer["prefixItems"]
                    .as_array()
                    .is_some_and(|items| !items.is_empty())
                    || layer
                        .get("items")
                        .is_some_and(|schema| !schema.is_boolean()))
        });
        let layout = layouts.next().unwrap_or(&UNRESTRICTED_SCHEMA);
        if layouts.next().is_some() {
            return None;
        }
        let minimum = layers
            .iter()
            .filter_map(|layer| layer["minItems"].as_u64())
            .max()
            .unwrap_or(0);
        return Some(CollectionConstraints::Array {
            layout,
            minimum,
            maximum,
        });
    }
    None
}

fn combine_numeric_constraints(
    layers: &[&Value],
    membership: u8,
    combination: Option<&str>,
) -> Option<(&'static str, Value)> {
    let name = match membership {
        INTEGER_TYPE => "integer",
        NUMBER_TYPE => "number",
        _ => return None,
    };
    let mut limits = serde_json::Map::new();
    for (index, layer) in layers.iter().enumerate() {
        let Some(fields) = layer.as_object() else {
            continue;
        };
        for (key, bound) in fields {
            let key = key.as_str();
            if !active_conjunction_keyword(key, index, combination, membership) {
                continue;
            }
            if !keywords("limit").any(|name| name == key) {
                return None;
            }
            let value = bound.as_f64().filter(|value| value.is_finite())?;
            let lower = matches!(key, "minimum" | "exclusiveMinimum");
            if limits
                .get(key)
                .and_then(Value::as_f64)
                .is_none_or(|previous| {
                    if lower {
                        value > previous
                    } else {
                        value < previous
                    }
                })
            {
                limits.insert(key.to_owned(), bound.clone());
            }
        }
    }
    Some((name, Value::Object(limits)))
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

fn possible_type_membership(schema: &Value, allowed_types: u8) -> u8 {
    let declared = allowed_types & declared_type_membership(schema);
    finite_constants(schema).map_or(declared, |constants| {
        declared
            & constants
                .into_iter()
                .fold(0, |types, value| types | value_type_membership(value))
    })
}

fn reference_layers<'schema>(
    root: &'schema Value,
    mut schema: &'schema Value,
) -> Option<Vec<&'schema Value>> {
    let mut layers = Vec::new();
    let mut visited = HashSet::new();
    while schema.is_boolean() || schema.is_object() {
        layers.push(schema);
        let Some(name) = schema.get("$ref") else {
            return Some(layers);
        };
        let name = name.as_str()?;
        if !visited.insert(name) || layers.len() > 128 {
            return None;
        }
        schema = reference(root, name)?;
    }
    None
}

fn branches_use_disjoint_types<'schema>(
    root: &'schema Value,
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
        let membership = reference_layers(root, branch).map_or_else(
            || possible_type_membership(branch, allowed_types),
            |layers| {
                layers.into_iter().fold(allowed_types, |types, layer| {
                    possible_type_membership(layer, types)
                })
            },
        );
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

fn branches_use_disjoint_required_tags(
    root: &Value,
    branches: &[Value],
    allowed_types: u8,
) -> bool {
    let tags: Vec<_> = branches
        .iter()
        .map(|branch| {
            reference_layers(root, branch)
                .and_then(|layers| layers.last().copied())
                .map(required_string_tags)
                .unwrap_or_default()
        })
        .collect();
    branches.iter().enumerate().all(|(index, branch)| {
        branches[..index]
            .iter()
            .enumerate()
            .all(|(other, previous)| {
                branches_use_disjoint_types(root, [branch, previous].into_iter(), allowed_types)
                    || tags[index].iter().any(|(key, values)| {
                        tags[other].get(key).is_some_and(|other_values| {
                            string_sets_are_disjoint(values, other_values)
                        })
                    })
            })
    })
}
