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
    DecimalRange, exclusive_decimal_range, partition_numeric_ranges, signed_decimal_range,
    signed_integer_range,
};
use super::patterns::{PatternLowering, any_json_pattern_character, lower_json_string_pattern};
use super::property_names::additional_property_name;
use super::schema::{Analysis, formats, keywords, reference, types};
use super::string_intersection::{
    StringIntersection, StringProductWork, intersect_json_string_expressions,
    partition_json_string_expressions, select_exclusive_json_string_expressions,
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
        collection_numeric_diagnostics: HashSet::new(),
        next: 1,
        remaining_nested_collections: NESTED_COLLECTION_LIMITS[1],
        collection_string_work: StringProductWork::new(),
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

#[derive(Clone)]
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

struct ArrayMembershipState<'schema> {
    position: usize,
    branches: Vec<usize>,
    transitions: Vec<(CollectionValueVariant<'schema>, usize)>,
    expression: Option<Expression>,
}

#[derive(Clone)]
enum CollectionChild<'schema> {
    Schema(&'schema Value),
    Conjunction(Vec<ConjunctionEntry<'schema>>),
}

#[derive(Clone)]
enum ConjunctionEntry<'schema> {
    Expression(Expression),
    Number(&'static str, Value),
    Collection(CollectionConstraints<'schema>),
}

struct ObjectFieldVariants<'schema> {
    name: &'schema str,
    variants: Vec<CollectionValueVariant<'schema>>,
}

struct CollectionValueVariant<'schema> {
    branches: Vec<usize>,
    use_common_schema: bool,
    child: Option<CollectionChild<'schema>>,
    values: Vec<Cow<'schema, Value>>,
    expression: Option<Expression>,
}

fn select_exclusive_object_fields(
    fields: &[ObjectFieldVariants<'_>],
    branches: &[usize],
    work_per_object: usize,
    remaining_work: &mut usize,
    selected: &mut Vec<usize>,
    selections: &mut Vec<Vec<usize>>,
) -> Option<()> {
    let Some((field, rest)) = fields.split_first() else {
        if branches.len() == 1 {
            *remaining_work = remaining_work.checked_sub(work_per_object)?;
            selections.push(selected.clone());
        }
        return Some(());
    };
    for (index, variant) in field.variants.iter().enumerate() {
        let cost = branches
            .len()
            .checked_mul(variant.branches.len())?
            .checked_add(1)?;
        *remaining_work = remaining_work.checked_sub(cost)?;
        let matching: Vec<_> = branches
            .iter()
            .copied()
            .filter(|branch| variant.branches.contains(branch))
            .collect();
        if matching.is_empty() {
            continue;
        }
        selected.push(index);
        select_exclusive_object_fields(
            rest,
            &matching,
            work_per_object,
            remaining_work,
            selected,
            selections,
        )?;
        selected.pop();
    }
    Some(())
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
    collection_numeric_diagnostics: HashSet<String>,
    next: usize,
    remaining_nested_collections: usize,
    collection_string_work: StringProductWork,
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
                        && let Some(expression) = self.lower_exclusive_array_alternatives(
                            schema,
                            branches,
                            allowed_types,
                            pointer,
                        )
                    {
                        return expression;
                    }
                    if overlaps
                        && let Some(expression) = self.exclusive_object_field_expression(
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
}

include!("import/collection_layout.rs");

impl<'schema> Importer<'schema> {
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
}

include!("import/schema_conjunction.rs");

include!("import/exclusive_alternatives.rs");
include!("import/numeric_alternatives.rs");

impl<'schema> Importer<'schema> {
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

include!("import/schema_constraints.rs");
