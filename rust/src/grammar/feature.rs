//! The grammar feature union forms (`docs/grammar/feature-union.md`).
//!
//! One table describes every expression kind, every operation of the scanner
//! and action language and every grammar declaration the feature union adds,
//! and the native listing, links and JSON codecs read and write them from
//! that table, so the serializations stay in step with each other and with
//! `js/src/grammar-feature-forms.js`.

use std::collections::BTreeSet;

use super::{CharClassItem, GrammarExpr};

/// The type of one field of a feature form or an operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldType {
    /// One word out of a fixed list.
    Choice(&'static [&'static str]),
    /// A signed integer of at most 15 digits.
    Integer,
    /// A name: a rule, mode, token, stack, variable, field or attribute.
    Name,
    /// A text value.
    Text,
    /// One grammar expression.
    Expression,
    /// The remaining grammar expressions.
    Expressions,
    /// One condition operation.
    Condition,
    /// One value operation.
    Value,
    /// The remaining condition operations.
    Conditions,
    /// A statement list written as `WORD(...)`; the flag makes it optional.
    Block(&'static str, bool),
}

/// The fields of one form, as `(key, type)` pairs in listing order.
pub type FormFields = &'static [(&'static str, FieldType)];

/// The expression kinds the feature union adds, with their fields in listing order.
pub const FEATURE_EXPRESSION_FORMS: &[(&str, FormFields)] = &[
    (
        "precedence",
        &[
            ("level", FieldType::Integer),
            (
                "associativity",
                FieldType::Choice(&["left", "right", "none"]),
            ),
            ("item", FieldType::Expression),
        ],
    ),
    (
        "namedPrecedence",
        &[
            ("name", FieldType::Name),
            (
                "associativity",
                FieldType::Choice(&["left", "right", "none"]),
            ),
            ("item", FieldType::Expression),
        ],
    ),
    (
        "dynamicPrecedence",
        &[
            ("level", FieldType::Integer),
            ("item", FieldType::Expression),
        ],
    ),
    (
        "lexicalPrecedence",
        &[
            ("level", FieldType::Integer),
            ("item", FieldType::Expression),
        ],
    ),
    ("longest", &[("items", FieldType::Expressions)]),
    ("token", &[("item", FieldType::Expression)]),
    ("immediateToken", &[("item", FieldType::Expression)]),
    (
        "alias",
        &[("name", FieldType::Name), ("item", FieldType::Expression)],
    ),
    ("parameter", &[("name", FieldType::Name)]),
    (
        "predicate",
        &[
            ("item", FieldType::Expression),
            ("condition", FieldType::Condition),
        ],
    ),
    (
        "recover",
        &[
            ("item", FieldType::Expression),
            ("synchronize", FieldType::Expression),
        ],
    ),
    ("missing", &[("item", FieldType::Expression)]),
    (
        "embed",
        &[
            ("language", FieldType::Name),
            ("item", FieldType::Expression),
        ],
    ),
    (
        "expand",
        &[
            ("name", FieldType::Name),
            ("arguments", FieldType::Expressions),
        ],
    ),
];

/// Whether an operation is a statement, a condition or a value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperationCategory {
    /// An operation run for its effect.
    Statement,
    /// An operation that holds or does not.
    Condition,
    /// An operation that computes an integer or a text.
    Value,
}

impl OperationCategory {
    /// The category's word in error messages.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Statement => "statement",
            Self::Condition => "condition",
            Self::Value => "value",
        }
    }
}

const TWO_VALUES: FormFields = &[("left", FieldType::Value), ("right", FieldType::Value)];
const STATEMENT: OperationCategory = OperationCategory::Statement;
const CONDITION: OperationCategory = OperationCategory::Condition;
const VALUE: OperationCategory = OperationCategory::Value;

/// The operation language of external scanners, semantic actions and
/// predicates, with each operation's category and fields in listing order.
pub const OPERATION_FORMS: &[(&str, OperationCategory, FormFields)] = &[
    ("advance", STATEMENT, &[]),
    ("consume", STATEMENT, &[("item", FieldType::Expression)]),
    ("skip", STATEMENT, &[("item", FieldType::Expression)]),
    ("mark", STATEMENT, &[]),
    ("emit", STATEMENT, &[("token", FieldType::Name)]),
    ("fail", STATEMENT, &[]),
    (
        "if",
        STATEMENT,
        &[
            ("condition", FieldType::Condition),
            ("consequent", FieldType::Block("then", false)),
            ("alternative", FieldType::Block("else", true)),
        ],
    ),
    (
        "while",
        STATEMENT,
        &[
            ("condition", FieldType::Condition),
            ("body", FieldType::Block("do", false)),
        ],
    ),
    (
        "push",
        STATEMENT,
        &[("stack", FieldType::Name), ("value", FieldType::Value)],
    ),
    ("pop", STATEMENT, &[("stack", FieldType::Name)]),
    (
        "set",
        STATEMENT,
        &[("variable", FieldType::Name), ("value", FieldType::Value)],
    ),
    ("pushMode", STATEMENT, &[("mode", FieldType::Name)]),
    ("popMode", STATEMENT, &[]),
    ("setMode", STATEMENT, &[("mode", FieldType::Name)]),
    (
        "setAttribute",
        STATEMENT,
        &[("attribute", FieldType::Name), ("value", FieldType::Value)],
    ),
    ("buildNode", STATEMENT, &[("kind", FieldType::Name)]),
    ("valid", CONDITION, &[("token", FieldType::Name)]),
    ("expected", CONDITION, &[("item", FieldType::Expression)]),
    ("next", CONDITION, &[("item", FieldType::Expression)]),
    ("atEnd", CONDITION, &[]),
    ("equal", CONDITION, TWO_VALUES),
    ("less", CONDITION, TWO_VALUES),
    ("greater", CONDITION, TWO_VALUES),
    ("all", CONDITION, &[("conditions", FieldType::Conditions)]),
    ("some", CONDITION, &[("conditions", FieldType::Conditions)]),
    ("not", CONDITION, &[("condition", FieldType::Condition)]),
    ("integer", VALUE, &[("value", FieldType::Integer)]),
    ("text", VALUE, &[("value", FieldType::Text)]),
    ("variable", VALUE, &[("name", FieldType::Name)]),
    ("top", VALUE, &[("stack", FieldType::Name)]),
    ("depth", VALUE, &[("stack", FieldType::Name)]),
    ("column", VALUE, &[]),
    ("matched", VALUE, &[]),
    ("mode", VALUE, &[]),
    (
        "attribute",
        VALUE,
        &[("field", FieldType::Name), ("attribute", FieldType::Name)],
    ),
    (
        "sumOf",
        VALUE,
        &[("field", FieldType::Name), ("attribute", FieldType::Name)],
    ),
    ("fieldText", VALUE, &[("field", FieldType::Name)]),
    ("length", VALUE, &[("value", FieldType::Value)]),
    ("number", VALUE, &[("value", FieldType::Value)]),
    ("add", VALUE, TWO_VALUES),
    ("subtract", VALUE, TWO_VALUES),
    ("multiply", VALUE, TWO_VALUES),
];

/// The ways a grammar may match: generalized (every alternative), PEG (first
/// and greedy) or longest (generalized, ties going to the longer token).
pub const MATCHING_MODES: &[&str] = &["generalized", "peg", "longest"];

/// The steps that settle two parses of one text that end alike, as a
/// grammar's `(settling STEP ...)` declares them.
///
/// In order (the lower repair cost always decides first): `tokens`, the tokens a lexer prefers (higher
/// lexical precedence, longer, a literal over a pattern, earlier), with the
/// lexing that goes with it (keywords, separators, immediate tokens);
/// `precedence`, the shift or reduction an LR parser keeps by precedence and
/// associativity, forking where a declared conflict asks; `dynamic`, the
/// higher dynamic precedence. The last step settles a tie: `first` keeps the
/// parse reached first, `ambiguity` keeps it and reports the ambiguity.
pub const SETTLING_STEPS: &[&str] = &["tokens", "precedence", "dynamic", "first", "ambiguity"];

/// The settling of a grammar that declares none, by its matching.
#[must_use]
pub fn default_settling(matching: &str) -> &'static [&'static str] {
    match matching {
        "peg" => &["first"],
        "longest" => &["tokens", "precedence", "dynamic", "ambiguity"],
        _ => &["dynamic", "ambiguity"],
    }
}

/// Why `steps` is no settling, or `None`: every step is known and given
/// once, and exactly the last one settles a tie.
#[must_use]
pub fn settling_problem<S: AsRef<str>>(steps: &[S]) -> Option<String> {
    if steps.is_empty() {
        return Some("the settling names no step".to_owned());
    }
    for (index, step) in steps.iter().enumerate() {
        let step = step.as_ref();
        if !SETTLING_STEPS.contains(&step) {
            return Some(format!("unknown settling step {step}"));
        }
        if steps.iter().position(|other| other.as_ref() == step) != Some(index) {
            return Some(format!("the settling step {step} is given twice"));
        }
        let tie = step == "first" || step == "ambiguity";
        if tie != (index == steps.len() - 1) {
            return Some("the settling ends with exactly one of first and ambiguity".to_owned());
        }
    }
    None
}

/// The longest integer a form may spell, in decimal digits.
pub const MAX_INTEGER_DIGITS: usize = 15;

/// The fields of the feature expression form `head`, when it is one.
#[must_use]
pub fn feature_expression_fields(head: &str) -> Option<FormFields> {
    FEATURE_EXPRESSION_FORMS
        .iter()
        .find(|(name, _)| *name == head)
        .map(|(_, fields)| *fields)
}

/// The category and fields of the operation `head`, when it is one.
#[must_use]
pub fn operation_form(head: &str) -> Option<(OperationCategory, FormFields)> {
    OPERATION_FORMS
        .iter()
        .find(|(name, _, _)| *name == head)
        .map(|(_, category, fields)| (*category, *fields))
}

/// Checks the text of an integer field: an optional `-` and at most 15 digits.
///
/// # Errors
///
/// Returns the detail of the failure.
pub fn check_integer(text: &str) -> Result<i64, String> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(format!("expected an integer, not {text}"));
    }
    if digits.len() > MAX_INTEGER_DIGITS {
        return Err(format!("integer {text} is too large"));
    }
    text.parse()
        .map_err(|_| format!("expected an integer, not {text}"))
}

/// The value of one field of a feature form or an operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FieldValue {
    /// One word of a [`FieldType::Choice`].
    Word(String),
    /// An integer.
    Integer(i64),
    /// A name.
    Name(String),
    /// A text.
    Text(String),
    /// One expression.
    Expression(GrammarExpr),
    /// A list of expressions.
    Expressions(Vec<GrammarExpr>),
    /// One condition or value.
    Operation(FeatureForm),
    /// A list of conditions.
    Operations(Vec<FeatureForm>),
    /// A statement block; `None` for an absent optional block.
    Block(Option<Vec<FeatureForm>>),
}

/// One feature expression form or one operation: a head word and its field
/// values, in the order of [`FEATURE_EXPRESSION_FORMS`] or [`OPERATION_FORMS`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FeatureForm {
    /// The form's head word, such as `precedence` or `pushMode`.
    pub head: String,
    /// The field values in table order.
    pub fields: Vec<FieldValue>,
}

/// One operation of the scanner and action language.
pub type Operation = FeatureForm;

impl FeatureForm {
    /// Builds a form from its head and field values.
    #[must_use]
    pub fn new(head: impl Into<String>, fields: Vec<FieldValue>) -> Self {
        Self {
            head: head.into(),
            fields,
        }
    }

    /// The table fields of this form, as an expression form or an operation.
    #[must_use]
    pub fn spec(&self) -> FormFields {
        feature_expression_fields(&self.head)
            .or_else(|| operation_form(&self.head).map(|(_, fields)| fields))
            .unwrap_or(&[])
    }

    /// The value of the field `key`.
    #[must_use]
    pub fn field(&self, key: &str) -> Option<&FieldValue> {
        let index = self.spec().iter().position(|(name, _)| *name == key)?;
        self.fields.get(index)
    }

    /// The expression of the field `key`.
    #[must_use]
    pub fn expression(&self, key: &str) -> Option<&GrammarExpr> {
        match self.field(key)? {
            FieldValue::Expression(expr) => Some(expr),
            _ => None,
        }
    }

    /// The expression list of the field `key`.
    #[must_use]
    pub fn expressions(&self, key: &str) -> &[GrammarExpr] {
        match self.field(key) {
            Some(FieldValue::Expressions(items)) => items,
            _ => &[],
        }
    }

    /// The word, name or text of the field `key`.
    #[must_use]
    pub fn text(&self, key: &str) -> Option<&str> {
        match self.field(key)? {
            FieldValue::Word(value) | FieldValue::Name(value) | FieldValue::Text(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    /// The integer of the field `key`.
    #[must_use]
    pub fn integer(&self, key: &str) -> Option<i64> {
        match self.field(key)? {
            FieldValue::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// The single operation of the field `key`.
    #[must_use]
    pub fn operation(&self, key: &str) -> Option<&Self> {
        match self.field(key)? {
            FieldValue::Operation(operation) => Some(operation),
            _ => None,
        }
    }

    /// The operation list or block of the field `key`; `None` for an absent block.
    #[must_use]
    pub fn operations(&self, key: &str) -> Option<&[Self]> {
        match self.field(key)? {
            FieldValue::Operations(items) | FieldValue::Block(Some(items)) => Some(items),
            _ => None,
        }
    }

    /// Every grammar expression this form holds directly, operations included.
    pub fn for_each_expression<'a>(&'a self, visit: &mut impl FnMut(&'a GrammarExpr)) {
        for value in &self.fields {
            match value {
                FieldValue::Expression(expr) => visit(expr),
                FieldValue::Expressions(items) => items.iter().for_each(&mut *visit),
                FieldValue::Operation(operation) => operation.for_each_expression(visit),
                FieldValue::Operations(items) | FieldValue::Block(Some(items)) => {
                    for operation in items {
                        operation.for_each_expression(visit);
                    }
                }
                _ => {}
            }
        }
    }

    /// Rewrites every grammar expression this form holds, operations included.
    pub fn map_expressions(&mut self, map: &mut impl FnMut(&mut GrammarExpr)) {
        for value in &mut self.fields {
            match value {
                FieldValue::Expression(expr) => map(expr),
                FieldValue::Expressions(items) => items.iter_mut().for_each(&mut *map),
                FieldValue::Operation(operation) => operation.map_expressions(map),
                FieldValue::Operations(items) | FieldValue::Block(Some(items)) => {
                    for operation in items {
                        operation.map_expressions(map);
                    }
                }
                _ => {}
            }
        }
    }
}

/// One item of a byte class.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteClassItem {
    /// One byte.
    Byte(u8),
    /// An inclusive byte range.
    Range(u8, u8),
}

/// One item of a character class that names Unicode properties.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnicodeClassItem {
    /// A single character.
    Char(char),
    /// An inclusive character range.
    Range(char, char),
    /// A Unicode general category such as `Lu` or `L`.
    Category(String),
    /// A Unicode script such as `Greek` or `Han`.
    Script(String),
}

/// A character class: a plain [`GrammarExpr::CharClass`] when every item is a
/// character or a range, otherwise a [`FeatureExpr::UnicodeClass`].
#[must_use]
pub fn class_expression(negated: bool, items: Vec<UnicodeClassItem>) -> GrammarExpr {
    let plain = items
        .iter()
        .map(|item| match item {
            UnicodeClassItem::Char(value) => Some(CharClassItem::Char(*value)),
            UnicodeClassItem::Range(start, end) => Some(CharClassItem::Range(*start, *end)),
            UnicodeClassItem::Category(_) | UnicodeClassItem::Script(_) => None,
        })
        .collect::<Option<Vec<_>>>();
    plain.map_or_else(
        || GrammarExpr::feature(FeatureExpr::UnicodeClass { negated, items }),
        |items| GrammarExpr::CharClass { negated, items },
    )
}

/// An expression of the grammar feature union.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FeatureExpr {
    /// A form of [`FEATURE_EXPRESSION_FORMS`].
    Form(FeatureForm),
    /// A class of bytes, `byteClass` or `notByteClass`.
    ByteClass {
        /// Whether the class is negated.
        negated: bool,
        /// Bytes and byte ranges of the class.
        items: Vec<ByteClassItem>,
    },
    /// A character class with at least one Unicode `category` or `script` item.
    UnicodeClass {
        /// Whether the class is negated.
        negated: bool,
        /// Characters, ranges, categories and scripts of the class.
        items: Vec<UnicodeClassItem>,
    },
    /// A call of a parameterized rule, `ref(NAME, ARGUMENT, ...)`.
    Call {
        /// The called rule.
        name: String,
        /// The arguments, at least one.
        arguments: Vec<GrammarExpr>,
    },
}

impl FeatureExpr {
    /// The head word of this expression in the native listing.
    #[must_use]
    pub fn head(&self) -> &str {
        match self {
            Self::Form(form) => &form.head,
            Self::ByteClass { .. } => "byteClass",
            Self::UnicodeClass { .. } => "charClass",
            Self::Call { .. } => "ref",
        }
    }

    /// Every grammar expression this one holds directly, operations included.
    pub fn for_each_expression<'a>(&'a self, visit: &mut impl FnMut(&'a GrammarExpr)) {
        match self {
            Self::Form(form) => form.for_each_expression(visit),
            Self::Call { arguments, .. } => arguments.iter().for_each(visit),
            Self::ByteClass { .. } | Self::UnicodeClass { .. } => {}
        }
    }

    /// Rewrites every grammar expression this one holds directly.
    pub fn map_expressions(&mut self, map: &mut impl FnMut(&mut GrammarExpr)) {
        match self {
            Self::Form(form) => form.map_expressions(map),
            Self::Call { arguments, .. } => arguments.iter_mut().for_each(map),
            Self::ByteClass { .. } | Self::UnicodeClass { .. } => {}
        }
    }

    /// Adds the rule names this expression references, as the JavaScript
    /// `collectReferences` does: calls and the expressions of `item`,
    /// `items`, `arguments` and `synchronize` fields.
    pub(crate) fn collect_references(&self, names: &mut BTreeSet<String>) {
        match self {
            Self::Call { name, arguments } => {
                names.insert(name.clone());
                for argument in arguments {
                    argument.collect_nonterminals(names);
                }
            }
            Self::Form(form) => {
                for value in &form.fields {
                    match value {
                        FieldValue::Expression(expr) => expr.collect_nonterminals(names),
                        FieldValue::Expressions(items) => {
                            for item in items {
                                item.collect_nonterminals(names);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Self::ByteClass { .. } | Self::UnicodeClass { .. } => {}
        }
    }
}

/// The optional parts of a rule the feature union adds.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RuleAttributes {
    /// The rule's parameters; empty for a rule without.
    pub parameters: Vec<String>,
    /// The channel; a channel other than `default` makes the rule trivia.
    pub channel: Option<String>,
    /// The lexer modes the rule may match in.
    pub modes: Option<Vec<String>>,
    /// The semantic action, a statement list.
    pub action: Option<Vec<Operation>>,
}

impl RuleAttributes {
    /// Whether no attribute is set.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

/// A macro declaration: `macro NAME(P, ...) = EXPRESSION`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarMacro {
    /// The macro name.
    pub name: String,
    /// The macro parameters.
    pub parameters: Vec<String>,
    /// The macro body.
    pub expression: GrammarExpr,
}

/// An external scanner declaration.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarScanner {
    /// The scanner name.
    pub name: String,
    /// The tokens the scanner produces.
    pub tokens: Vec<String>,
    /// The scanner's statements.
    pub operations: Vec<Operation>,
}

/// One entry of a precedence order: a named precedence or a rule, whose
/// productions it ranks.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrecedenceEntry {
    /// The precedence `namedPrecedence` gives this name.
    Name(String),
    /// Every precedence of this rule's productions.
    Rule(String),
}

impl PrecedenceEntry {
    /// Builds an entry from its kind word, `name` or `rule`, and its value.
    #[must_use]
    pub fn from_kind(kind: &str, value: String) -> Option<Self> {
        match kind {
            "name" => Some(Self::Name(value)),
            "rule" => Some(Self::Rule(value)),
            _ => None,
        }
    }

    /// The entry's kind word, `name` or `rule`.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Name(_) => "name",
            Self::Rule(_) => "rule",
        }
    }

    /// The precedence or rule name.
    #[must_use]
    pub fn value(&self) -> &str {
        match self {
            Self::Name(value) | Self::Rule(value) => value,
        }
    }
}

/// The grammar-level declarations of the feature union.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GrammarDeclarations {
    /// `generalized`, `peg` or `longest`, when given.
    pub matching: Option<String>,
    /// The settling steps (see [`SETTLING_STEPS`]), when given.
    pub settling: Option<Vec<String>>,
    /// Names of grammars this one inherits from.
    pub imports: Vec<String>,
    /// The lexer modes beyond `default`.
    pub modes: Vec<String>,
    /// Trivia expressions allowed between any two tokens.
    pub extras: Vec<GrammarExpr>,
    /// Groups of rule names whose ambiguity is expected.
    pub conflicts: Vec<Vec<String>>,
    /// Orders of named precedences and rules, the earlier entry higher.
    pub precedences: Vec<Vec<PrecedenceEntry>>,
    /// Macro declarations.
    pub macros: Vec<GrammarMacro>,
    /// External scanner declarations.
    pub scanners: Vec<GrammarScanner>,
}

impl GrammarDeclarations {
    /// Whether no declaration is given.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// The token names the external scanners produce.
    #[must_use]
    pub fn external_tokens(&self) -> Vec<&str> {
        self.scanners
            .iter()
            .flat_map(|scanner| scanner.tokens.iter().map(String::as_str))
            .collect()
    }

    /// The names of the declaration kinds that are given, in listing order.
    #[must_use]
    pub fn present(&self) -> Vec<&'static str> {
        let mut present = Vec::new();
        if self.matching.is_some() {
            present.push("matching");
        }
        if self.settling.is_some() {
            present.push("settling");
        }
        for (name, empty) in [
            ("imports", self.imports.is_empty()),
            ("modes", self.modes.is_empty()),
            ("extras", self.extras.is_empty()),
            ("conflicts", self.conflicts.is_empty()),
            ("precedences", self.precedences.is_empty()),
            ("macros", self.macros.is_empty()),
            ("scanners", self.scanners.is_empty()),
        ] {
            if !empty {
                present.push(name);
            }
        }
        present
    }
}
