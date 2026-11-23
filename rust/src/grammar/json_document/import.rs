use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::hash::Hash;
use std::sync::LazyLock;

use serde_json::Value;

use super::super::{
    CharClassItem, ExprBuilder, Grammar, GrammarExpr, GrammarRule, parse_grammar_links,
};
use super::integer_policy::exclusive_integer_boundary;
use super::number_spelling::NumberSpelling;
use super::numeric_ranges::{
    DecimalRange, exclusive_decimal_range, signed_decimal_range, signed_integer_range,
};
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
static PROHIBITED_SCHEMA: Value = Value::Bool(false);

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
const FINITE_SCALAR_TYPES: u8 = 1 << 5 | 1 << 6;
static FINITE_SCALAR_VALUES: [Value; 3] = [Value::Bool(false), Value::Bool(true), Value::Null];
static NESTED_COLLECTION_LIMITS: LazyLock<[usize; 2]> = LazyLock::new(|| {
    let mut limits = keywords("nestedCollectionLimits");
    std::array::from_fn(|_| {
        limits
            .next()
            .expect("nested collection limit")
            .parse()
            .expect("numeric nested collection limit")
    })
});
static FINITE_COLLECTION_LIMITS: LazyLock<[usize; 3]> = LazyLock::new(|| {
    let mut limits = keywords("finiteCollectionLimits");
    std::array::from_fn(|_| {
        limits
            .next()
            .expect("finite collection limit")
            .parse()
            .expect("numeric finite collection limit")
    })
});
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
        numeric_constant_diagnostics: HashSet::new(),
        next: 1,
        remaining_nested_collections: NESTED_COLLECTION_LIMITS[1],
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
        properties: Vec<(&'schema str, CollectionChild<'schema>)>,
        additional: CollectionChild<'schema>,
        required: Vec<&'schema str>,
    },
    Array {
        prefix: Vec<CollectionChild<'schema>>,
        items: CollectionChild<'schema>,
        minimum: u64,
        maximum: Option<u64>,
    },
}

#[derive(Clone, Copy, PartialEq)]
struct ArrayItemLayout<'schema> {
    prefix: &'schema [Value],
    items: &'schema Value,
}

struct ArrayLengthProfile<'schema> {
    minimum: usize,
    maximum: Option<usize>,
    layout: Option<ArrayItemLayout<'schema>>,
}

enum CollectionChild<'schema> {
    Schema(&'schema Value),
    Conjunction(Vec<ConjunctionEntry<'schema>>),
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
    numeric_constant_diagnostics: HashSet<String>,
    next: usize,
    remaining_nested_collections: usize,
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

    fn lower_child(
        &mut self,
        child: CollectionChild<'schema>,
        pointer: &str,
    ) -> Option<Expression> {
        match child {
            CollectionChild::Schema(schema) => self.named(schema, pointer),
            CollectionChild::Conjunction(entries) => self.lower_conjunction(entries, pointer),
        }
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
                (constants.into_iter().map(Cow::Borrowed).collect(), keyword)
            })
            .or_else(|| {
                if reference_has_siblings || combination.is_some() {
                    finite_schema_candidates(
                        self.root,
                        schema,
                        &mut HashSet::new(),
                        &mut 16_384,
                        0,
                        allowed_types,
                    )
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
                        && super::validation::accepts_constant(root, [schema], value)
                })
                .map(|value| self.constant(value.as_ref(), pointer, keyword))
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
                    && !branches_use_disjoint_collection_layouts(
                        self.root,
                        branches,
                        allowed_types,
                    );
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
                    if overlaps
                        && let Some(expression) = self.exclusive_array_length_expression(
                            schema,
                            branches,
                            allowed_types,
                            pointer,
                        )
                    {
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
                if overlaps {
                    // An overlap can affect one JSON instance type without affecting the others.
                    let reachable = branches.iter().fold(0, |types, branch| {
                        types | referenced_type_membership(self.root, branch, allowed_types)
                    });
                    let families: Vec<_> = JSON_TYPES
                        .into_iter()
                        .filter(|&(name, mask)| name != "integer" && mask & reachable != 0)
                        .map(|(_, mask)| mask & allowed_types)
                        .collect();
                    if families.len() > 1 {
                        let entries: Vec<_> = families
                            .into_iter()
                            .filter_map(|mask| self.named_with_types(schema, pointer, mask))
                            .collect();
                        return (!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries));
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
            Value::Number(value) => {
                let number = NumberSpelling::new(&value.to_string());
                if !number.is_zero()
                    && self
                        .numeric_constant_diagnostics
                        .insert(pointer_child(pointer, keyword))
                {
                    self.report(
                        pointer,
                        keyword,
                        "Validation accepts additional equivalent number spellings with shifted decimal points and compensating exponents.",
                    );
                }
                number.constant_expression()
            }
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
            property_schemas(schema)
                .map(|(name, schema)| (name, CollectionChild::Schema(schema)))
                .collect(),
            CollectionChild::Schema(additional_property_schema(schema)),
            required_property_names(schema),
            pointer,
        )
    }

    fn object_with_properties(
        &mut self,
        fields: Vec<(&'schema str, CollectionChild<'schema>)>,
        additional: CollectionChild<'schema>,
        required: impl Iterator<Item = &'schema str>,
        pointer: &str,
    ) -> Option<Expression> {
        let mut keys: Vec<_> = fields.iter().map(|(name, _)| *name).collect();
        let mut fields: HashMap<_, _> = fields.into_iter().collect();
        let mut required_names = HashSet::new();
        for key in required {
            if required_names.insert(key) && !fields.contains_key(key) {
                keys.push(key);
            }
        }
        if keys.is_empty() {
            let body = match additional {
                CollectionChild::Schema(Value::Bool(false)) => EXPRESSION.empty(),
                CollectionChild::Schema(Value::Bool(true)) => {
                    return Some(reference_expression("object"));
                }
                child => self
                    .lower_child(child, &pointer_child(pointer, "additionalProperties"))
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
        let additional =
            self.lower_child(additional, &pointer_child(pointer, "additionalProperties"));
        let additional_member = additional.clone()
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
            let item = match fields.remove(key) {
                Some(child) => self.lower_child(
                    child,
                    &pointer_child(&pointer_child(pointer, "properties"), key),
                ),
                None => additional.clone(),
            };
            let is_required = required_names.contains(key);
            if let Some(item) = item {
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
        self.array_with_length(
            prefix_item_schemas(schema)
                .iter()
                .map(CollectionChild::Schema),
            CollectionChild::Schema(tail_item_schema(schema)),
            min,
            max,
            pointer,
        )
    }

    fn array_with_length(
        &mut self,
        prefix: impl IntoIterator<Item = CollectionChild<'schema>>,
        items: CollectionChild<'schema>,
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
        let prefix: Vec<_> = prefix
            .into_iter()
            .enumerate()
            .map(|(index, item)| {
                self.lower_child(item, &pointer_child(&location, &index.to_string()))
            })
            .collect();
        let tail = self.lower_child(items, &pointer_child(pointer, "items"));
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

    fn combine_child_schemas(
        &mut self,
        schemas: Vec<&'schema Value>,
        pointer: &str,
        mut depth: usize,
    ) -> Option<CollectionChild<'schema>> {
        if let Some(schema) = compatible_child_schema(schemas.iter().copied()) {
            return Some(CollectionChild::Schema(schema));
        }
        let root = self.root;
        let mut work = 16_384;
        for schema in &schemas {
            let Some((candidates, keyword)) = finite_schema_candidates(
                root,
                schema,
                &mut HashSet::new(),
                &mut work,
                0,
                ALL_JSON_TYPES,
            ) else {
                continue;
            };
            if candidates.len() > work / schemas.len() {
                continue;
            }
            let entries: Vec<_> = candidates
                .into_iter()
                .filter(|value| {
                    super::validation::accepts_constant(root, schemas.iter().copied(), value)
                })
                .map(|value| self.constant(value.as_ref(), pointer, keyword))
                .collect();
            if entries.is_empty() {
                return Some(CollectionChild::Schema(&PROHIBITED_SCHEMA));
            }
            return Some(CollectionChild::Conjunction(vec![
                ConjunctionEntry::Expression(EXPRESSION.choice_unordered(entries)),
            ]));
        }
        let mut layers = Vec::new();
        for schema in schemas {
            if schema == &UNRESTRICTED_SCHEMA {
                continue;
            }
            let reference = reference_layers(self.root, schema)?;
            if layers.len() + reference.len() > 128 {
                return None;
            }
            layers.extend(reference);
        }
        let membership = layers.iter().fold(ALL_JSON_TYPES, |types, layer| {
            possible_type_membership(layer, types)
        });
        let collections = membership & (type_membership("object") | type_membership("array"));
        // Recursive definitions can revisit the same collection through different member paths.
        if collections != 0
            && layers.iter().any(|layer| {
                layer.as_object().is_some_and(|fields| {
                    fields.keys().any(|key| {
                        ASSERTION_TYPES
                            .get(key.as_str())
                            .is_some_and(|mask| mask & collections != 0)
                    })
                })
            })
        {
            if depth >= NESTED_COLLECTION_LIMITS[0] || self.remaining_nested_collections == 0 {
                return None;
            }
            self.remaining_nested_collections -= 1;
            depth += 1;
        }
        let entries = self.conjunction_entries(&layers, membership, None, pointer, depth)?;
        if entries.is_empty() {
            return Some(CollectionChild::Schema(&PROHIBITED_SCHEMA));
        }
        Some(CollectionChild::Conjunction(entries))
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
                prefix,
                items,
                minimum,
                maximum,
            } => self.array_with_length(
                prefix,
                items,
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
        let entries = self.conjunction_entries(&layers, allowed_types, None, pointer, 0)?;
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
        pointer: &str,
        depth: usize,
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
                "object" | "array" => ConjunctionEntry::Collection(collection_constraints(
                    layers,
                    mask,
                    combination,
                    pointer,
                    |schemas, location| self.combine_child_schemas(schemas, location, depth),
                )?),
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

    fn exclusive_array_length_expression(
        &mut self,
        schema: &'schema Value,
        branches: &'schema [Value],
        allowed_types: u8,
        pointer: &str,
    ) -> Option<Option<Expression>> {
        let array = type_membership("array");
        let guard = array_length_profile(&[schema], Some("oneOf"))?;
        let mut shared_profile: Option<ArrayLengthProfile<'schema>> = None;
        let mut boundaries = BTreeMap::<usize, i64>::new();
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
            if membership != array {
                return None;
            }
            let profile = array_length_profile(&layers, None)?;
            let minimum = guard.minimum.max(profile.minimum);
            let maximum = guard.maximum.into_iter().chain(profile.maximum).min();
            if maximum.is_some_and(|maximum| minimum > maximum) {
                continue;
            }
            if maximum != Some(0) {
                if shared_profile
                    .as_ref()
                    .is_some_and(|shared| shared.layout != profile.layout)
                {
                    return None;
                }
                shared_profile = Some(profile);
            }
            // Coverage changes at the first included length and the first length after the interval.
            *boundaries.entry(minimum).or_default() += 1;
            if let Some(maximum) = maximum {
                *boundaries.entry(maximum.checked_add(1)?).or_default() -= 1;
            }
        }
        let branch_layout = shared_profile.and_then(|profile| profile.layout);
        if let (Some(guard_layout), Some(branch_layout)) = (guard.layout, branch_layout)
            && guard_layout != branch_layout
        {
            return None;
        }
        let layout = guard.layout.or(branch_layout).unwrap_or(ArrayItemLayout {
            prefix: &[],
            items: &UNRESTRICTED_SCHEMA,
        });
        let mut expressions = Vec::new();
        let mut add_interval = |minimum, maximum| {
            if let Some(expression) = self.array_with_length(
                layout.prefix.iter().map(CollectionChild::Schema),
                CollectionChild::Schema(layout.items),
                minimum,
                maximum,
                pointer,
            ) {
                expressions.push(expression);
            }
        };
        let mut count = 0;
        let mut minimum = 0;
        for (length, change) in boundaries {
            let next_count = count + change;
            if count == 1 && next_count != 1 {
                add_interval(minimum, Some(length - 1));
            }
            if count != 1 && next_count == 1 {
                minimum = length;
            }
            count = next_count;
        }
        if count == 1 {
            add_interval(minimum, None);
        }
        Some((!expressions.is_empty()).then(|| EXPRESSION.choice_unordered(expressions)))
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
        let mut finite_work = 16_384;
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
            if keyword == "oneOf"
                && membership & NUMBER_TYPE == membership
                && let Some((candidates, _)) = finite_schema_candidates(
                    self.root,
                    branch,
                    &mut HashSet::new(),
                    &mut finite_work,
                    0,
                    ALL_JSON_TYPES,
                )
            {
                finite_work = finite_work.checked_sub(candidates.len())?;
                let (_, common) =
                    combine_numeric_constraints(&[schema], NUMBER_TYPE, Some(keyword))?;
                // Distinct values within one branch contribute disjoint point intervals.
                let mut seen = HashSet::new();
                for value in candidates {
                    let Some(number) = value.as_f64() else {
                        continue;
                    };
                    let identity = if number == 0.0 { 0 } else { number.to_bits() };
                    if membership & value_type_membership(&value) != 0
                        && seen.insert(identity)
                        && super::validation::accepts_constant(self.root, [branch, &common], &value)
                    {
                        let point = serde_json::json!({ "minimum": value, "maximum": value });
                        alternatives.push((index, "number", point));
                    }
                }
                continue;
            }
            layers.insert(0, schema);
            let (name, limits) = combine_numeric_constraints(&layers, membership, Some(keyword))?;
            alternatives.push((index, name, limits));
        }
        if keyword == "oneOf" {
            let number_limits: Vec<_> = alternatives
                .iter()
                .filter_map(|(_, name, limits)| (*name == "number").then_some(limits))
                .collect();
            if !number_limits.is_empty() {
                let mixed = number_limits.len() != alternatives.len();
                let mut decimal = match exclusive_decimal_range(number_limits, mixed) {
                    DecimalRange::Unsupported => return None,
                    DecimalRange::Empty => None,
                    DecimalRange::Allowed(expression) => Some(expression),
                };
                if mixed {
                    // Both schema types count for an integer, including zero-padded decimals.
                    let integer = exclusive_integer_range(
                        alternatives
                            .iter()
                            .map(|(_, _, limits)| integer_bounds(limits)),
                    )?
                    .map(|expression| {
                        EXPRESSION.seq([
                            expression,
                            EXPRESSION.opt(EXPRESSION.seq([
                                EXPRESSION.term("."),
                                EXPRESSION.repeat(EXPRESSION.term("0"), 1, None),
                            ])),
                        ])
                    });
                    let entries: Vec<_> = decimal.into_iter().chain(integer).collect();
                    decimal = (!entries.is_empty()).then(|| EXPRESSION.choice_unordered(entries));
                }
                if let Some(expression) = &decimal
                    && !matches!(expression, Expression::NonTerminal(_))
                {
                    self.report(pointer, keyword, "The exclusive number decoder uses decimal spelling. Validation also accepts equivalent exponent spellings.");
                }
                return Some(decimal);
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
        let location = pointer_child(pointer, keyword);
        for (index, branch) in branches.iter().enumerate() {
            if possible_type_membership(branch, allowed_types) == 0 {
                continue;
            }
            let mut layers = reference_layers(self.root, branch)?;
            layers.insert(0, schema);
            let location = pointer_child(&location, &index.to_string());
            let entries =
                self.conjunction_entries(&layers, allowed_types, Some(keyword), &location, 0)?;
            alternatives.push((location, entries));
        }
        let entries: Vec<_> = alternatives
            .into_iter()
            .filter_map(|(location, entries)| self.lower_conjunction(entries, &location))
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

fn prefix_item_schemas(schema: &Value) -> &[Value] {
    schema["prefixItems"].as_array().map_or(&[], Vec::as_slice)
}

fn tail_item_schema(schema: &Value) -> &Value {
    schema.get("items").unwrap_or(&UNRESTRICTED_SCHEMA)
}

fn array_length_bounds(layers: &[&Value]) -> (u64, Option<u64>) {
    let minimum = layers
        .iter()
        .filter_map(|layer| layer["minItems"].as_u64())
        .max()
        .unwrap_or(0);
    let maximum = layers
        .iter()
        .filter_map(|layer| {
            let bound = layer["maxItems"].as_u64();
            if tail_item_schema(layer) == &PROHIBITED_SCHEMA {
                let prefix = prefix_item_schemas(layer).len() as u64;
                return Some(bound.map_or(prefix, |bound| bound.min(prefix)));
            }
            bound
        })
        .min();
    (minimum, maximum)
}

fn array_length_profile<'schema>(
    layers: &[&'schema Value],
    combination: Option<&str>,
) -> Option<ArrayLengthProfile<'schema>> {
    const MAXIMUM_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
    if !collection_keywords_are_supported(
        layers,
        type_membership("array"),
        "arrayAssertion",
        combination,
    ) {
        return None;
    }
    let (minimum, maximum) = array_length_bounds(layers);
    if minimum > MAXIMUM_SAFE_INTEGER
        || maximum.is_some_and(|maximum| maximum >= MAXIMUM_SAFE_INTEGER)
    {
        return None;
    }
    let minimum = usize::try_from(minimum).ok()?;
    let maximum = match maximum {
        Some(maximum) => Some(usize::try_from(maximum).ok()?),
        None => None,
    };
    let mut layout = None;
    for &layer in layers {
        let prefix = prefix_item_schemas(layer);
        // A closed tail already contributes its limit to the length interval.
        let items = match tail_item_schema(layer) {
            Value::Bool(false) => &UNRESTRICTED_SCHEMA,
            items => items,
        };
        if prefix.is_empty() && items == &UNRESTRICTED_SCHEMA {
            continue;
        }
        let candidate = ArrayItemLayout { prefix, items };
        if layout.is_some_and(|layout| layout != candidate) {
            return None;
        }
        layout = Some(candidate);
    }
    Some(ArrayLengthProfile {
        minimum,
        maximum,
        layout,
    })
}

fn compatible_child_schema<'schema>(
    schemas: impl Iterator<Item = &'schema Value>,
) -> Option<&'schema Value> {
    let mut rule = &UNRESTRICTED_SCHEMA;
    let mut agrees = true;
    let mut membership = ALL_JSON_TYPES;
    for candidate in schemas {
        match candidate {
            Value::Bool(false) => return Some(candidate),
            Value::Bool(true) => (),
            _ if rule == &UNRESTRICTED_SCHEMA => rule = candidate,
            _ => agrees &= rule == candidate,
        }
        membership &= declared_type_membership(candidate);
    }
    if membership == 0 {
        return Some(&PROHIBITED_SCHEMA);
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
    pointer: &str,
    mut combine: impl FnMut(Vec<&'schema Value>, &str) -> Option<CollectionChild<'schema>>,
) -> Option<CollectionConstraints<'schema>> {
    if membership == type_membership("object") {
        if !collection_keywords_are_supported(layers, membership, "objectAssertion", combination) {
            return None;
        }
        let additional = combine(
            layers
                .iter()
                .copied()
                .map(additional_property_schema)
                .collect(),
            &pointer_child(pointer, "additionalProperties"),
        )?;
        let mut seen = HashSet::new();
        let keys = layers
            .iter()
            .flat_map(|&layer| property_schemas(layer).map(|(name, _)| name))
            .filter(|&name| seen.insert(name));
        let mut properties = Vec::new();
        let location = pointer_child(pointer, "properties");
        for name in keys {
            // Defaults apply only in the layers that leave this property undeclared.
            let schemas = layers
                .iter()
                .map(|&layer| {
                    layer["properties"]
                        .get(name)
                        .unwrap_or_else(|| additional_property_schema(layer))
                })
                .collect();
            let rule = combine(schemas, &pointer_child(&location, name))?;
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
        let (minimum, maximum) = array_length_bounds(layers);
        let layouts: Vec<_> = layers
            .iter()
            .copied()
            .filter(|layer| {
                maximum.is_none_or(|limit| limit != 0 && limit >= minimum)
                    && (!prefix_item_schemas(layer).is_empty()
                        || !tail_item_schema(layer).is_boolean())
            })
            .collect();
        if layouts.len() < 2 {
            let layout = layouts.first().copied().unwrap_or(&UNRESTRICTED_SCHEMA);
            return Some(CollectionConstraints::Array {
                prefix: prefix_item_schemas(layout)
                    .iter()
                    .map(CollectionChild::Schema)
                    .collect(),
                items: CollectionChild::Schema(tail_item_schema(layout)),
                minimum,
                maximum,
            });
        }
        let prefix_length = layouts
            .iter()
            .map(|&layer| prefix_item_schemas(layer).len())
            .max()
            .unwrap_or(0);
        let prefix_end = maximum
            .and_then(|limit| usize::try_from(limit).ok())
            .map_or(prefix_length, |limit| prefix_length.min(limit));
        let mut prefix = Vec::new();
        let location = pointer_child(pointer, "prefixItems");
        for index in 0..prefix_end {
            let schemas = layouts
                .iter()
                .map(|&layer| {
                    prefix_item_schemas(layer)
                        .get(index)
                        .unwrap_or_else(|| tail_item_schema(layer))
                })
                .collect();
            let item = combine(schemas, &pointer_child(&location, &index.to_string()))?;
            // An array cannot reach any position after a forbidden item.
            if matches!(item, CollectionChild::Schema(Value::Bool(false))) {
                return Some(CollectionConstraints::Array {
                    prefix,
                    items: CollectionChild::Schema(&UNRESTRICTED_SCHEMA),
                    minimum,
                    maximum: Some(index as u64),
                });
            }
            prefix.push(item);
        }
        let items = if maximum.is_some_and(|limit| limit <= prefix_length as u64) {
            CollectionChild::Schema(&UNRESTRICTED_SCHEMA)
        } else {
            combine(
                layouts.iter().copied().map(tail_item_schema).collect(),
                &pointer_child(pointer, "items"),
            )?
        };
        return Some(CollectionConstraints::Array {
            prefix,
            items,
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

fn charge_collection_value(
    value: &Value,
    copies: usize,
    work: &mut usize,
    depth: usize,
) -> Option<()> {
    let size = 1 + value.as_str().map_or(0, str::len);
    *work = work.checked_sub(copies.checked_mul(size)?)?;
    if depth > 128 {
        return None;
    }
    if let Some(items) = value.as_array() {
        for item in items {
            charge_collection_value(item, copies, work, depth + 1)?;
        }
    }
    if let Some(fields) = value.as_object() {
        for (key, item) in fields {
            *work = work.checked_sub(copies.checked_mul(key.len() + 1)?)?;
            charge_collection_value(item, copies, work, depth + 1)?;
        }
    }
    Some(())
}

fn extend_collection_values(
    prefixes: Vec<Value>,
    members: &[Cow<'_, Value>],
    keep_prefixes: bool,
    work: &mut usize,
    mut append: impl FnMut(&Value, &Value) -> Value,
) -> Option<Vec<Value>> {
    let size = prefixes
        .len()
        .checked_mul(members.len().checked_add(usize::from(keep_prefixes))?)?;
    if size > FINITE_COLLECTION_LIMITS[2] {
        return None;
    }
    *work = work.checked_sub(size)?;
    if !members.is_empty() {
        for prefix in &prefixes {
            charge_collection_value(prefix, members.len(), work, 0)?;
        }
        for member in members {
            charge_collection_value(member, prefixes.len(), work, 0)?;
        }
    }
    let mut extended = Vec::with_capacity(size);
    for prefix in &prefixes {
        for member in members {
            extended.push(append(prefix, member.as_ref()));
        }
    }
    if keep_prefixes {
        let mut prefixes = prefixes;
        prefixes.extend(extended);
        return Some(prefixes);
    }
    Some(extended)
}

fn finite_collection_values<'schema>(
    schema: &'schema Value,
    membership: u8,
    work: &mut usize,
    mut collect_member: impl FnMut(&'schema Value, &mut usize) -> Option<Vec<Cow<'schema, Value>>>,
) -> Option<Vec<Value>> {
    let [maximum_properties, maximum_length, maximum_values] = *FINITE_COLLECTION_LIMITS;
    if membership == type_membership("object")
        && schema["additionalProperties"] == PROHIBITED_SCHEMA
    {
        let fields = schema["properties"].as_object();
        let required: HashSet<_> = schema["required"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect();
        if required
            .iter()
            .any(|name| !fields.is_some_and(|fields| fields.contains_key(*name)))
        {
            return Some(Vec::new());
        }
        if fields.is_some_and(|fields| fields.len() > maximum_properties) {
            return None;
        }
        let mut values = vec![Value::Object(serde_json::Map::new())];
        for (name, child) in fields.into_iter().flatten() {
            let members = collect_member(child, work)?;
            let key_work = values
                .len()
                .checked_mul(members.len())?
                .checked_mul(name.len() + 1)?;
            *work = work.checked_sub(key_work)?;
            values = extend_collection_values(
                values,
                &members,
                !required.contains(name.as_str()),
                work,
                |prefix, member| {
                    let mut fields = prefix.as_object().expect("object candidate").clone();
                    fields.insert(name.clone(), member.clone());
                    Value::Object(fields)
                },
            )?;
            if values.is_empty() {
                break;
            }
        }
        return Some(values);
    }
    if membership == type_membership("array") {
        let (minimum, maximum) = array_length_bounds(&[schema]);
        let maximum = maximum?;
        if maximum < minimum {
            return Some(Vec::new());
        }
        if maximum > maximum_length as u64 {
            return None;
        }
        let mut values = vec![Value::Array(Vec::new())];
        let mut candidates = Vec::new();
        let prefix = prefix_item_schemas(schema);
        for length in 0..=maximum {
            if length >= minimum {
                if candidates.len() + values.len() > maximum_values {
                    return None;
                }
                for value in &values {
                    charge_collection_value(value, 1, work, 0)?;
                }
                candidates.extend(values.iter().cloned());
            }
            if length == maximum {
                break;
            }
            let child = prefix
                .get(length as usize)
                .unwrap_or_else(|| tail_item_schema(schema));
            let members = collect_member(child, work)?;
            values = extend_collection_values(values, &members, false, work, |prefix, member| {
                let mut items = prefix.as_array().expect("array candidate").clone();
                items.push(member.clone());
                Value::Array(items)
            })?;
            if values.is_empty() {
                break;
            }
        }
        return Some(candidates);
    }
    None
}

fn finite_schema_candidates<'schema>(
    root: &'schema Value,
    schema: &'schema Value,
    active: &mut HashSet<*const Value>,
    work: &mut usize,
    depth: usize,
    allowed_types: u8,
) -> Option<(Vec<Cow<'schema, Value>>, &'static str)> {
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
        return Some((values.into_iter().map(Cow::Borrowed).collect(), keyword));
    }
    let membership = allowed_types & declared_type_membership(schema);
    if membership == 0 {
        return Some((Vec::new(), "type"));
    }
    if membership & FINITE_SCALAR_TYPES == membership {
        let values = FINITE_SCALAR_VALUES
            .iter()
            .filter(|value| value_type_membership(value) & membership != 0)
            .map(Cow::Borrowed)
            .collect();
        return Some((values, "type"));
    }
    let rules = schema.as_object()?;
    active.insert(identity);
    let result = (|| {
        if let Some(target) = rules
            .get("$ref")
            .and_then(Value::as_str)
            .and_then(|name| reference(root, name))
            && let Some((values, _)) =
                finite_schema_candidates(root, target, active, work, depth + 1, membership)
        {
            return Some((values, "$ref"));
        }
        for keyword in ["anyOf", "oneOf"] {
            let Some(branches) = rules.get(keyword).and_then(Value::as_array) else {
                continue;
            };
            let values = branches.iter().try_fold(Vec::new(), |mut values, child| {
                let (candidates, _) =
                    finite_schema_candidates(root, child, active, work, depth + 1, membership)?;
                *work = work.checked_sub(candidates.len())?;
                values.extend(candidates);
                Some(values)
            });
            if let Some(values) = values {
                return Some((values, keyword));
            }
        }
        let values = finite_collection_values(schema, membership, work, |child, work| {
            let (values, _) =
                finite_schema_candidates(root, child, active, work, depth + 1, ALL_JSON_TYPES)?;
            Some(values)
        })?;
        let keyword = if membership == type_membership("object") {
            "properties"
        } else {
            "items"
        };
        Some((values.into_iter().map(Cow::Owned).collect(), keyword))
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

fn referenced_type_membership(root: &Value, branch: &Value, allowed_types: u8) -> u8 {
    reference_layers(root, branch).map_or_else(
        || possible_type_membership(branch, allowed_types),
        |layers| {
            layers.into_iter().fold(allowed_types, |types, layer| {
                possible_type_membership(layer, types)
            })
        },
    )
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
        let membership = referenced_type_membership(root, branch, allowed_types);
        if seen & membership != 0 {
            return false;
        }
        seen |= membership;
    }
    true
}

fn finite_string_values(schema: &Value) -> Option<Vec<&str>> {
    finite_constants(schema)?
        .into_iter()
        .map(Value::as_str)
        .collect()
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
            let values = finite_string_values(schema["properties"].get(key)?)?;
            Some((key, values))
        })
        .collect()
}

fn required_prefix_tags<'schema>(
    layers: &[&'schema Value],
    minimum: u64,
) -> HashMap<usize, Vec<&'schema str>> {
    let mut tags = HashMap::new();
    let required = usize::try_from(minimum).unwrap_or(usize::MAX);
    for &layer in layers {
        for (position, schema) in prefix_item_schemas(layer).iter().take(required).enumerate() {
            if let Some(values) = finite_string_values(schema) {
                tags.entry(position).or_insert(values);
            }
        }
    }
    tags
}

fn tags_are_disjoint<Key: Eq + Hash>(
    left: &HashMap<Key, Vec<&str>>,
    right: &HashMap<Key, Vec<&str>>,
) -> bool {
    left.iter().any(|(key, values)| {
        right
            .get(key)
            .is_some_and(|other| string_sets_are_disjoint(values, other))
    })
}

fn branches_use_disjoint_collection_layouts(
    root: &Value,
    branches: &[Value],
    allowed_types: u8,
) -> bool {
    let references: Vec<_> = branches
        .iter()
        .map(|branch| reference_layers(root, branch))
        .collect();
    let tags: Vec<_> = references
        .iter()
        .map(|layers| {
            layers
                .as_ref()
                .and_then(|layers| layers.last().copied())
                .map(required_string_tags)
                .unwrap_or_default()
        })
        .collect();
    let arrays: Vec<_> = references
        .iter()
        .map(|layers| {
            let layers = layers.as_ref()?;
            let membership = layers.iter().fold(allowed_types, |types, layer| {
                possible_type_membership(layer, types)
            });
            (membership == type_membership("array")).then(|| {
                let (minimum, maximum) = array_length_bounds(layers);
                (minimum, maximum, required_prefix_tags(layers, minimum))
            })
        })
        .collect();
    branches.iter().enumerate().all(|(index, branch)| {
        branches[..index]
            .iter()
            .enumerate()
            .all(|(other, previous)| {
                branches_use_disjoint_types(root, [branch, previous].into_iter(), allowed_types)
                    || arrays[index]
                        .as_ref()
                        .zip(arrays[other].as_ref())
                        .is_some_and(
                            |(
                                (left_min, left_max, left_tags),
                                (right_min, right_max, right_tags),
                            )| {
                                let minimum = left_min.max(right_min);
                                left_max
                                    .iter()
                                    .chain(right_max)
                                    .any(|maximum| maximum < minimum)
                                    || tags_are_disjoint(left_tags, right_tags)
                            },
                        )
                    || tags_are_disjoint(&tags[index], &tags[other])
            })
    })
}
