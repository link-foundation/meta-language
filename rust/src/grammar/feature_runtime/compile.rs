//! Compiling a loaded grammar's expressions and operations into the typed
//! program the executor interprets, with the checks `load.js` runs over every
//! expression (`checkExpression`) and operation (`checkOperations`).

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use super::load::{LoadResult, refuse};
use super::operations::{Condition, Statement, ValueOp, allowed_in};
use super::program::{
    Associativity, ClassTest, Expr, Matcher, Name, Target, insensitive_expectation,
    literal_expectation, range_expectation,
};
use super::text::{fold_case, unicode_property_matcher};
use crate::grammar::{
    CharClassItem, FeatureExpr, FeatureForm, GrammarExpr, Operation, UnicodeClassItem,
};

/// Compiles the expressions and operations of one grammar.
pub(super) struct Compiler<'a> {
    rule_index: &'a HashMap<String, usize>,
    external: &'a HashMap<String, usize>,
    modes: &'a HashSet<String>,
    node_kinds: HashMap<String, Name>,
    embedded: Vec<String>,
    /// The items a scanner's `expected` asks about, by their keys (see
    /// `expectation_key`), each with its id.
    pub(super) expectations: HashMap<String, usize>,
    /// Whether the compiled literals are requested, as those of rule bodies
    /// are and those of extras are not.
    pub(super) requesting: bool,
}

/// The key of an item a scanner's `expected` asks about: a literal by its
/// text, a rule or an external token by its name.
fn expectation_key(literal: bool, value: &str) -> String {
    if literal {
        format!("literal {value}")
    } else {
        format!("ref {value}")
    }
}

/// The kind `load.js` names an expression by.
fn kind_name(expr: &GrammarExpr) -> &str {
    match expr {
        GrammarExpr::Empty => "empty",
        GrammarExpr::AnyChar => "any",
        GrammarExpr::Terminal(_) => "literal",
        GrammarExpr::TerminalInsensitive(_) => "literalInsensitive",
        GrammarExpr::CharRange(..) => "charRange",
        GrammarExpr::CharClass { .. } => "charClass",
        GrammarExpr::NonTerminal(_) => "ref",
        GrammarExpr::Choice { .. } => "choice",
        GrammarExpr::Sequence(_) => "seq",
        GrammarExpr::Optional(_) => "optional",
        GrammarExpr::ZeroOrMore(_) => "repeat0",
        GrammarExpr::OneOrMore(_) => "repeat1",
        GrammarExpr::And(_) => "and",
        GrammarExpr::Not(_) => "not",
        GrammarExpr::Repeat { .. } => "repeat",
        GrammarExpr::Capture { .. } => "capture",
        GrammarExpr::Feature(feature) => match feature.as_ref() {
            FeatureExpr::Form(form) => &form.head,
            FeatureExpr::ByteClass { .. } => "byteClass",
            FeatureExpr::UnicodeClass { .. } => "charClass",
            FeatureExpr::Call { .. } => "ref",
        },
    }
}

fn boxed(expr: Expr) -> Box<Expr> {
    Box::new(expr)
}

fn terminal(matcher: Matcher, expectation: &str) -> Expr {
    Expr::Terminal {
        matcher,
        expectation: Arc::from(expectation),
        expected: None,
    }
}

impl<'a> Compiler<'a> {
    pub(super) fn new(
        rule_index: &'a HashMap<String, usize>,
        external: &'a HashMap<String, usize>,
        modes: &'a HashSet<String>,
        node_kinds: HashMap<String, Name>,
    ) -> Self {
        Self {
            rule_index,
            external,
            modes,
            node_kinds,
            embedded: Vec::new(),
            expectations: HashMap::new(),
            requesting: false,
        }
    }

    /// The embedded languages the compiled expressions name, in first-use order.
    pub(super) fn into_embedded(self) -> Vec<String> {
        self.embedded
    }

    /// Refuses a mode the grammar does not declare.
    pub(super) fn check_mode(&self, mode: &str, owner: &str) -> LoadResult<()> {
        if self.modes.contains(mode) {
            Ok(())
        } else {
            refuse("declaration", format!("undeclared mode {mode} in {owner}"))
        }
    }

    fn target(&self, name: &str, owner: &str) -> LoadResult<Target> {
        if self.external.contains_key(name) {
            return Ok(Target::External(Arc::from(name)));
        }
        match self.rule_index.get(name) {
            Some(&index) => Ok(Target::Rule(index)),
            None => refuse("reference", format!("undefined rule {name} in {owner}")),
        }
    }

    fn items(&mut self, items: &[GrammarExpr], owner: &str) -> LoadResult<Vec<Expr>> {
        items
            .iter()
            .map(|item| self.expression(item, owner))
            .collect()
    }

    fn item(&mut self, form: &FeatureForm, key: &str, owner: &str) -> LoadResult<Box<Expr>> {
        match form.expression(key) {
            Some(item) => Ok(boxed(self.expression(item, owner)?)),
            None => refuse(
                "declaration",
                format!("{} in {owner} has no {key}", form.head),
            ),
        }
    }

    /// Compiles one expression of `owner`.
    pub(super) fn expression(&mut self, expr: &GrammarExpr, owner: &str) -> LoadResult<Expr> {
        Ok(match expr {
            GrammarExpr::Empty => Expr::Empty,
            GrammarExpr::Terminal(value) => Expr::Terminal {
                matcher: Matcher::Literal(value.as_bytes().to_vec()),
                expectation: literal_expectation(value),
                expected: if self.requesting && !self.expectations.is_empty() {
                    self.expectations
                        .get(&expectation_key(true, value))
                        .copied()
                } else {
                    None
                },
            },
            GrammarExpr::TerminalInsensitive(value) => Expr::Terminal {
                matcher: Matcher::Insensitive {
                    folded: fold_case(value),
                    count: value.chars().count(),
                },
                expectation: insensitive_expectation(value),
                expected: None,
            },
            GrammarExpr::CharRange(start, end) => Expr::Terminal {
                matcher: Matcher::CharRange(u32::from(*start), u32::from(*end)),
                expectation: range_expectation(*start, *end),
                expected: None,
            },
            GrammarExpr::CharClass { negated, items } => terminal(
                Matcher::CharClass {
                    negated: *negated,
                    tests: items
                        .iter()
                        .map(|item| match item {
                            CharClassItem::Char(value) => ClassTest::Char(u32::from(*value)),
                            CharClassItem::Range(low, high) => {
                                ClassTest::Range(u32::from(*low), u32::from(*high))
                            }
                        })
                        .collect(),
                },
                "character class",
            ),
            GrammarExpr::AnyChar => terminal(Matcher::Any, "any character"),
            GrammarExpr::NonTerminal(name) => Expr::Ref(self.target(name, owner)?),
            GrammarExpr::Choice {
                ordered,
                alternatives,
            } => Expr::Choice {
                ordered: *ordered,
                items: self.items(alternatives, owner)?,
            },
            GrammarExpr::Sequence(items) => Expr::Seq(self.items(items, owner)?),
            GrammarExpr::Optional(item) => self.repeat(item, 0, Some(1), owner)?,
            GrammarExpr::ZeroOrMore(item) => self.repeat(item, 0, None, owner)?,
            GrammarExpr::OneOrMore(item) => self.repeat(item, 1, None, owner)?,
            GrammarExpr::Repeat { expr, min, max } => self.repeat(expr, *min, *max, owner)?,
            GrammarExpr::And(item) => Expr::And(boxed(self.expression(item, owner)?)),
            GrammarExpr::Not(item) => Expr::Not(boxed(self.expression(item, owner)?)),
            GrammarExpr::Capture { label, expr } => Expr::Capture {
                label: label.as_deref().map(Arc::from),
                item: boxed(self.expression(expr, owner)?),
            },
            GrammarExpr::Feature(feature) => self.feature(feature, owner)?,
        })
    }

    fn repeat(
        &mut self,
        item: &GrammarExpr,
        min: usize,
        max: Option<usize>,
        owner: &str,
    ) -> LoadResult<Expr> {
        Ok(Expr::Repeat {
            item: boxed(self.expression(item, owner)?),
            min,
            max,
        })
    }

    fn feature(&mut self, feature: &FeatureExpr, owner: &str) -> LoadResult<Expr> {
        match feature {
            FeatureExpr::ByteClass { negated, items } => Ok(terminal(
                Matcher::ByteClass {
                    negated: *negated,
                    items: items.clone(),
                },
                "byte class",
            )),
            FeatureExpr::UnicodeClass { negated, items } => {
                let mut tests = Vec::with_capacity(items.len());
                for item in items {
                    tests.push(match item {
                        UnicodeClassItem::Char(value) => ClassTest::Char(u32::from(*value)),
                        UnicodeClassItem::Range(low, high) => {
                            ClassTest::Range(u32::from(*low), u32::from(*high))
                        }
                        UnicodeClassItem::Category(value) => property(false, value)?,
                        UnicodeClassItem::Script(value) => property(true, value)?,
                    });
                }
                Ok(terminal(
                    Matcher::CharClass {
                        negated: *negated,
                        tests,
                    },
                    "character class",
                ))
            }
            FeatureExpr::Call { name, .. } => refuse(
                "parameter",
                format!("the call of {name} survives loading in {owner}"),
            ),
            FeatureExpr::Form(form) => self.form(form, owner),
        }
    }

    fn level(form: &FeatureForm) -> i64 {
        form.integer("level").unwrap_or(0)
    }

    fn form(&mut self, form: &FeatureForm, owner: &str) -> LoadResult<Expr> {
        Ok(match form.head.as_str() {
            head @ ("precedence" | "namedPrecedence") => Expr::Precedence {
                level: Self::level(form),
                name: (head == "namedPrecedence")
                    .then(|| Arc::from(form.text("name").unwrap_or_default())),
                associativity: match form.text("associativity") {
                    Some("left") => Associativity::Left,
                    Some("right") => Associativity::Right,
                    _ => Associativity::None,
                },
                item: self.item(form, "item", owner)?,
            },
            "dynamicPrecedence" => Expr::DynamicPrecedence {
                level: Self::level(form),
                item: self.item(form, "item", owner)?,
            },
            "lexicalPrecedence" => Expr::LexicalPrecedence {
                level: Self::level(form),
                item: self.item(form, "item", owner)?,
            },
            "longest" => Expr::Longest(self.items(form.expressions("items"), owner)?),
            "token" => Expr::Token(self.item(form, "item", owner)?),
            "immediateToken" => Expr::ImmediateToken(self.item(form, "item", owner)?),
            "alias" => Expr::Alias {
                name: Arc::from(form.text("name").unwrap_or_default()),
                item: self.item(form, "item", owner)?,
            },
            "predicate" => {
                let item = self.item(form, "item", owner)?;
                let Some(condition) = form.operation("condition") else {
                    return refuse(
                        "declaration",
                        format!("predicate in {owner} has no condition"),
                    );
                };
                Expr::Predicate {
                    item,
                    condition: Box::new(self.condition(condition, "predicate", owner, None)?),
                }
            }
            "recover" => {
                let item = self.item(form, "item", owner)?;
                if form.expression("synchronize").is_none() {
                    return refuse(
                        "declaration",
                        format!("recover in {owner} has no synchronization"),
                    );
                }
                Expr::Recover {
                    item,
                    synchronize: self.item(form, "synchronize", owner)?,
                }
            }
            "missing" => {
                let (kind, literal) = match form.expression("item") {
                    Some(GrammarExpr::NonTerminal(name)) => (
                        Some(
                            self.node_kinds
                                .get(name)
                                .cloned()
                                .unwrap_or_else(|| Arc::from(name.as_str())),
                        ),
                        false,
                    ),
                    Some(GrammarExpr::Terminal(value)) => (Some(Arc::from(value.as_str())), true),
                    _ => (None, false),
                };
                Expr::Missing {
                    item: self.item(form, "item", owner)?,
                    kind,
                    literal,
                }
            }
            "embed" => {
                let language = form.text("language").unwrap_or_default().to_owned();
                if !self.embedded.contains(&language) {
                    self.embedded.push(language.clone());
                }
                Expr::Embed {
                    language: Arc::from(language.as_str()),
                    item: self.item(form, "item", owner)?,
                }
            }
            head @ ("expand" | "parameter") => {
                return refuse(
                    "macro",
                    format!(
                        "{head} {} survives loading in {owner}",
                        form.text("name").unwrap_or_default()
                    ),
                );
            }
            head => return refuse("declaration", format!("unknown form {head} in {owner}")),
        })
    }

    fn check_operation(
        &self,
        operation: &Operation,
        context: &str,
        owner: &str,
        tokens: Option<&[String]>,
    ) -> LoadResult<()> {
        let head = operation.head.as_str();
        if !allowed_in(context, head) {
            return refuse(
                "operation",
                format!("{head} cannot run in a {context} ({owner})"),
            );
        }
        if head == "pushMode" || head == "setMode" {
            self.check_mode(operation.text("mode").unwrap_or_default(), owner)?;
        }
        if head == "emit" || head == "valid" {
            let token = operation.text("token").unwrap_or_default();
            if let Some(tokens) = tokens
                && !tokens.iter().any(|item| item == token)
            {
                return refuse(
                    "operation",
                    format!("{head} names {token}, which {owner} does not produce"),
                );
            }
        }
        Ok(())
    }

    fn operation_item(&mut self, operation: &Operation, owner: &str) -> LoadResult<Expr> {
        operation.expression("item").map_or_else(
            || {
                refuse(
                    "operation",
                    format!("{} in {owner} has no item", operation.head),
                )
            },
            |item| self.expression(item, owner),
        )
    }

    /// The key of an item a scanner's `expected` may ask about: a literal, or
    /// a rule or an external token the grammar defines.
    pub(super) fn expectation(&self, item: &GrammarExpr) -> Option<String> {
        match item {
            GrammarExpr::Terminal(value) => Some(expectation_key(true, value)),
            GrammarExpr::NonTerminal(name)
                if self.rule_index.contains_key(name) || self.external.contains_key(name) =>
            {
                Some(expectation_key(false, name))
            }
            _ => None,
        }
    }

    fn name_of(operation: &Operation, key: &str) -> Name {
        Arc::from(operation.text(key).unwrap_or_default())
    }

    /// Compiles a statement list run in `context` (`scanner` or `action`).
    pub(super) fn statements(
        &mut self,
        operations: &[Operation],
        context: &str,
        owner: &str,
        tokens: Option<&[String]>,
    ) -> LoadResult<Vec<Statement>> {
        operations
            .iter()
            .map(|operation| self.statement(operation, context, owner, tokens))
            .collect()
    }

    fn block(
        &mut self,
        operation: &Operation,
        key: &str,
        context: &str,
        owner: &str,
        tokens: Option<&[String]>,
    ) -> LoadResult<Option<Vec<Statement>>> {
        operation
            .operations(key)
            .map(|block| self.statements(block, context, owner, tokens))
            .transpose()
    }

    fn statement(
        &mut self,
        operation: &Operation,
        context: &str,
        owner: &str,
        tokens: Option<&[String]>,
    ) -> LoadResult<Statement> {
        self.check_operation(operation, context, owner, tokens)?;
        let name = |key| Self::name_of(operation, key);
        Ok(match operation.head.as_str() {
            "advance" => Statement::Advance,
            "consume" => Statement::Consume(self.operation_item(operation, owner)?),
            "skip" => Statement::Skip(self.operation_item(operation, owner)?),
            "mark" => Statement::Mark,
            "emit" => Statement::Emit(name("token")),
            "fail" => Statement::Fail,
            "if" => Statement::If(
                self.sub_condition(operation, "condition", context, owner, tokens)?,
                self.block(operation, "consequent", context, owner, tokens)?
                    .unwrap_or_default(),
                self.block(operation, "alternative", context, owner, tokens)?,
            ),
            "while" => Statement::While(
                self.sub_condition(operation, "condition", context, owner, tokens)?,
                self.block(operation, "body", context, owner, tokens)?
                    .unwrap_or_default(),
            ),
            "push" => Statement::Push(
                name("stack"),
                self.sub_value(operation, "value", context, owner)?,
            ),
            "pop" => Statement::Pop(name("stack")),
            "set" => Statement::Set(
                name("variable"),
                self.sub_value(operation, "value", context, owner)?,
            ),
            "pushMode" => Statement::PushMode(name("mode")),
            "popMode" => Statement::PopMode,
            "setMode" => Statement::SetMode(name("mode")),
            "setAttribute" => Statement::SetAttribute(
                operation.text("attribute").unwrap_or_default().to_owned(),
                self.sub_value(operation, "value", context, owner)?,
            ),
            "buildNode" => Statement::BuildNode(name("kind")),
            head => {
                return refuse("operation", format!("{head} is not a statement ({owner})"));
            }
        })
    }

    fn sub_condition(
        &mut self,
        operation: &Operation,
        key: &str,
        context: &str,
        owner: &str,
        tokens: Option<&[String]>,
    ) -> LoadResult<Condition> {
        operation.operation(key).map_or_else(
            || {
                refuse(
                    "operation",
                    format!("{} in {owner} has no {key}", operation.head),
                )
            },
            |condition| self.condition(condition, context, owner, tokens),
        )
    }

    fn condition(
        &mut self,
        operation: &Operation,
        context: &str,
        owner: &str,
        tokens: Option<&[String]>,
    ) -> LoadResult<Condition> {
        self.check_operation(operation, context, owner, tokens)?;
        let pair = |this: &mut Self| -> LoadResult<(ValueOp, ValueOp)> {
            Ok((
                this.sub_value(operation, "left", context, owner)?,
                this.sub_value(operation, "right", context, owner)?,
            ))
        };
        Ok(match operation.head.as_str() {
            "valid" => Condition::Valid(Self::name_of(operation, "token")),
            "expected" => {
                let Some(item) = operation.expression("item") else {
                    return refuse("operation", format!("expected in {owner} has no item"));
                };
                if let GrammarExpr::NonTerminal(name) = item
                    && self.expectation(item).is_none()
                {
                    return refuse(
                        "reference",
                        format!("expected names undefined rule {name} in {owner}"),
                    );
                }
                let Some(key) = self.expectation(item) else {
                    return refuse(
                        "operation",
                        format!(
                            "expected asks about a {}, not a literal or a rule, in {owner}",
                            kind_name(item)
                        ),
                    );
                };
                Condition::Expected(self.expectations[&key])
            }
            "next" => Condition::Next(self.operation_item(operation, owner)?),
            "atEnd" => Condition::AtEnd,
            "equal" => {
                let (left, right) = pair(self)?;
                Condition::Equal(left, right)
            }
            "less" => {
                let (left, right) = pair(self)?;
                Condition::Less(left, right)
            }
            "greater" => {
                let (left, right) = pair(self)?;
                Condition::Greater(left, right)
            }
            head @ ("all" | "some") => {
                let items = operation
                    .operations("conditions")
                    .unwrap_or_default()
                    .iter()
                    .map(|item| self.condition(item, context, owner, tokens))
                    .collect::<LoadResult<Vec<_>>>()?;
                if head == "all" {
                    Condition::All(items)
                } else {
                    Condition::Some(items)
                }
            }
            "not" => Condition::Not(Box::new(self.sub_condition(
                operation,
                "condition",
                context,
                owner,
                tokens,
            )?)),
            head => return refuse("operation", format!("{head} is not a condition ({owner})")),
        })
    }

    fn sub_value(
        &mut self,
        operation: &Operation,
        key: &str,
        context: &str,
        owner: &str,
    ) -> LoadResult<ValueOp> {
        operation.operation(key).map_or_else(
            || {
                refuse(
                    "operation",
                    format!("{} in {owner} has no {key}", operation.head),
                )
            },
            |value| self.value(value, context, owner),
        )
    }

    fn value(&mut self, operation: &Operation, context: &str, owner: &str) -> LoadResult<ValueOp> {
        self.check_operation(operation, context, owner, None)?;
        let text = |key| operation.text(key).unwrap_or_default().to_owned();
        let name = |key| Self::name_of(operation, key);
        let boxed_value = |this: &mut Self, key| -> LoadResult<Box<ValueOp>> {
            Ok(Box::new(this.sub_value(operation, key, context, owner)?))
        };
        Ok(match operation.head.as_str() {
            "integer" => ValueOp::Integer(operation.integer("value").unwrap_or(0)),
            "text" => ValueOp::Text(text("value")),
            "variable" => ValueOp::Variable(name("name")),
            "top" => ValueOp::Top(name("stack")),
            "depth" => ValueOp::Depth(name("stack")),
            "column" => ValueOp::Column,
            "matched" => ValueOp::Matched,
            "mode" => ValueOp::Mode,
            "attribute" => ValueOp::Attribute(text("field"), text("attribute")),
            "sumOf" => ValueOp::SumOf(text("field"), text("attribute")),
            "fieldText" => ValueOp::FieldText(text("field")),
            "length" => ValueOp::Length(boxed_value(self, "value")?),
            "number" => ValueOp::Number(boxed_value(self, "value")?),
            "add" => ValueOp::Add(boxed_value(self, "left")?, boxed_value(self, "right")?),
            "subtract" => {
                ValueOp::Subtract(boxed_value(self, "left")?, boxed_value(self, "right")?)
            }
            "multiply" => {
                ValueOp::Multiply(boxed_value(self, "left")?, boxed_value(self, "right")?)
            }
            head => return refuse("operation", format!("{head} is not a value ({owner})")),
        })
    }
}

fn property(script: bool, value: &str) -> LoadResult<ClassTest> {
    unicode_property_matcher(script, value).map_or_else(
        || {
            refuse(
                "pattern",
                format!(
                    "unknown Unicode {} {value}",
                    if script { "script" } else { "category" }
                ),
            )
        },
        |pattern| Ok(ClassTest::Property(pattern)),
    )
}
