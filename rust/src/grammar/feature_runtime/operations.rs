//! The operation language of external scanners, semantic actions and
//! predicates, as `js/src/grammar-runtime/operations.js`: a small,
//! deterministic statement language over the parser state.
//!
//! The operations are data in the grammar, compiled at load into the
//! [`Statement`], [`Condition`] and [`ValueOp`] trees interpreted here.

use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;

use super::executor::Executor;
use super::program::Expr;
use super::results::{Res, content_start};
use super::text::column_of;

/// The largest integer the operation language computes, as `Number.MAX_SAFE_INTEGER`.
const SAFE_INTEGER: i64 = (1 << 53) - 1;

/// A value of the operation language: an integer or a text.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OperationValue {
    /// An integer within the safe range of a JavaScript number.
    Integer(i64),
    /// A text.
    Text(String),
}

/// Why a parse stops before it ends: a resource limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Abort {
    StepLimit,
    NestingTooDeep,
    MemoryBudget,
}

/// Why an operation does not complete.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OpError {
    /// The operation failed: the match it guards fails.
    Failed,
    /// A resource limit ends the parse.
    Abort(Abort),
}

impl From<Abort> for OpError {
    fn from(abort: Abort) -> Self {
        Self::Abort(abort)
    }
}

pub(super) type OpResult<T> = Result<T, OpError>;

/// The immutable parser state threaded through every match: the mode stack,
/// the named stacks (empty ones dropped) and the named variables.
#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) struct StateData {
    pub(super) modes: Vec<Arc<str>>,
    pub(super) stacks: BTreeMap<Arc<str>, Vec<OperationValue>>,
    pub(super) variables: BTreeMap<Arc<str>, OperationValue>,
}

/// A shared, immutable parser state; equal states are interchangeable.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(super) struct State(pub(super) Rc<StateData>);

impl State {
    /// The state every parse starts in: the `default` mode, no stacks and no variables.
    pub(super) fn initial() -> Self {
        Self(Rc::new(StateData {
            modes: vec![Arc::from("default")],
            stacks: BTreeMap::new(),
            variables: BTreeMap::new(),
        }))
    }

    /// The current mode.
    pub(super) fn mode(&self) -> &str {
        self.0.modes.last().map_or("default", |mode| mode)
    }

    /// A mutable working copy for one run of operations.
    pub(super) fn working(&self) -> Working {
        Working {
            modes: self.0.modes.clone(),
            stacks: self.0.stacks.clone(),
            variables: self.0.variables.clone(),
        }
    }
}

/// A mutable working copy of a [`State`].
#[derive(Clone, Debug)]
pub(super) struct Working {
    pub(super) modes: Vec<Arc<str>>,
    pub(super) stacks: BTreeMap<Arc<str>, Vec<OperationValue>>,
    pub(super) variables: BTreeMap<Arc<str>, OperationValue>,
}

impl Working {
    /// Freezes the working copy back into a state.
    pub(super) fn settle(self) -> State {
        let mut stacks = self.stacks;
        stacks.retain(|_, values| !values.is_empty());
        State(Rc::new(StateData {
            modes: self.modes,
            stacks,
            variables: self.variables,
        }))
    }
}

/// A value operation.
#[derive(Debug)]
pub(super) enum ValueOp {
    Integer(i64),
    Text(String),
    Variable(Arc<str>),
    Top(Arc<str>),
    Depth(Arc<str>),
    Column,
    Matched,
    Mode,
    Attribute(String, String),
    SumOf(String, String),
    FieldText(String),
    Length(Box<Self>),
    Number(Box<Self>),
    Add(Box<Self>, Box<Self>),
    Subtract(Box<Self>, Box<Self>),
    Multiply(Box<Self>, Box<Self>),
}

/// A condition operation.
#[derive(Debug)]
pub(super) enum Condition {
    Valid(Arc<str>),
    /// Whether the parse, where the scan started, tries the item of this id
    /// (see `Expectations` in executor.rs).
    Expected(usize),
    Next(Expr),
    AtEnd,
    Equal(ValueOp, ValueOp),
    Less(ValueOp, ValueOp),
    Greater(ValueOp, ValueOp),
    All(Vec<Self>),
    Some(Vec<Self>),
    Not(Box<Self>),
}

/// A statement operation.
#[derive(Debug)]
pub(super) enum Statement {
    Advance,
    Consume(Expr),
    Skip(Expr),
    Mark,
    Emit(Arc<str>),
    Fail,
    If(Condition, Vec<Self>, Option<Vec<Self>>),
    While(Condition, Vec<Self>),
    Push(Arc<str>, ValueOp),
    Pop(Arc<str>),
    Set(Arc<str>, ValueOp),
    PushMode(Arc<str>),
    PopMode,
    SetMode(Arc<str>),
    SetAttribute(String, ValueOp),
    BuildNode(Arc<str>),
}

/// What the operations of one context may read and do. Every operation an
/// executor does not offer in a context is rejected at load, so the default
/// methods are never reached.
pub(super) trait Machine {
    fn state(&mut self) -> &mut Working;
    fn step(&mut self) -> Result<(), Abort>;
    fn requested(&self) -> Option<&str> {
        None
    }
    fn expected(&mut self, _id: usize) -> bool {
        false
    }
    fn column(&mut self) -> usize;
    fn at_end(&mut self) -> bool;
    fn matched(&mut self) -> OpResult<String> {
        Err(OpError::Failed)
    }
    fn lookahead(&mut self, _item: &Expr) -> OpResult<bool> {
        Ok(false)
    }
    fn attribute(&mut self, _field: &str, _name: &str) -> OpResult<OperationValue> {
        Err(OpError::Failed)
    }
    fn attributes(&mut self, _field: &str, _name: &str) -> OpResult<Vec<OperationValue>> {
        Err(OpError::Failed)
    }
    fn field_text(&mut self, _field: &str) -> OpResult<String> {
        Err(OpError::Failed)
    }
    fn advance(&mut self) -> OpResult<()> {
        Err(OpError::Failed)
    }
    fn consume(&mut self, _item: &Expr) -> OpResult<()> {
        Err(OpError::Failed)
    }
    fn skip(&mut self, _item: &Expr) -> OpResult<()> {
        Err(OpError::Failed)
    }
    fn mark(&mut self) {}
    fn set_attribute(&mut self, _name: &str, _value: OperationValue) {}
    fn build_node(&mut self, _kind: &Arc<str>) {}
}

const fn integer(value: &OperationValue) -> OpResult<i64> {
    match value {
        OperationValue::Integer(value) => Ok(*value),
        OperationValue::Text(_) => Err(OpError::Failed),
    }
}

fn checked(value: Option<i64>) -> OpResult<OperationValue> {
    match value {
        Some(value) if (-SAFE_INTEGER..=SAFE_INTEGER).contains(&value) => {
            Ok(OperationValue::Integer(value))
        }
        _ => Err(OpError::Failed),
    }
}

fn to_integer(value: usize) -> OperationValue {
    OperationValue::Integer(i64::try_from(value).unwrap_or(i64::MAX))
}

fn arithmetic(
    left: &ValueOp,
    right: &ValueOp,
    machine: &mut dyn Machine,
    combine: fn(i64, i64) -> Option<i64>,
) -> OpResult<OperationValue> {
    let left = integer(&evaluate_value(left, machine)?)?;
    let right = integer(&evaluate_value(right, machine)?)?;
    checked(combine(left, right))
}

fn is_number_text(text: &str) -> bool {
    let digits = text.strip_prefix('-').unwrap_or(text);
    (1..=15).contains(&digits.len()) && digits.bytes().all(|byte| byte.is_ascii_digit())
}

/// Evaluates a value operation.
pub(super) fn evaluate_value(
    value: &ValueOp,
    machine: &mut dyn Machine,
) -> OpResult<OperationValue> {
    machine.step()?;
    Ok(match value {
        ValueOp::Integer(value) => OperationValue::Integer(*value),
        ValueOp::Text(value) => OperationValue::Text(value.clone()),
        ValueOp::Variable(name) => machine
            .state()
            .variables
            .get(name)
            .cloned()
            .unwrap_or(OperationValue::Integer(0)),
        ValueOp::Top(stack) => machine
            .state()
            .stacks
            .get(stack)
            .and_then(|values| values.last().cloned())
            .unwrap_or(OperationValue::Integer(0)),
        ValueOp::Depth(stack) => to_integer(machine.state().stacks.get(stack).map_or(0, Vec::len)),
        ValueOp::Column => to_integer(machine.column()),
        ValueOp::Matched => OperationValue::Text(machine.matched()?),
        ValueOp::Mode => OperationValue::Text(
            machine
                .state()
                .modes
                .last()
                .map_or_else(String::new, std::string::ToString::to_string),
        ),
        ValueOp::Attribute(field, name) => machine.attribute(field, name)?,
        ValueOp::SumOf(field, name) => {
            let mut sum = OperationValue::Integer(0);
            for item in machine.attributes(field, name)? {
                sum = checked(integer(&sum)?.checked_add(integer(&item)?))?;
            }
            sum
        }
        ValueOp::FieldText(field) => OperationValue::Text(machine.field_text(field)?),
        ValueOp::Length(inner) => match evaluate_value(inner, machine)? {
            OperationValue::Text(text) => to_integer(text.chars().count()),
            OperationValue::Integer(_) => return Err(OpError::Failed),
        },
        ValueOp::Number(inner) => match evaluate_value(inner, machine)? {
            number @ OperationValue::Integer(_) => number,
            OperationValue::Text(text) => {
                if !is_number_text(&text) {
                    return Err(OpError::Failed);
                }
                OperationValue::Integer(text.parse().map_err(|_| OpError::Failed)?)
            }
        },
        ValueOp::Add(left, right) => arithmetic(left, right, machine, i64::checked_add)?,
        ValueOp::Subtract(left, right) => arithmetic(left, right, machine, i64::checked_sub)?,
        ValueOp::Multiply(left, right) => arithmetic(left, right, machine, i64::checked_mul)?,
    })
}

/// Evaluates a condition operation.
pub(super) fn evaluate_condition(
    condition: &Condition,
    machine: &mut dyn Machine,
) -> OpResult<bool> {
    machine.step()?;
    Ok(match condition {
        Condition::Valid(token) => machine.requested() == Some(&**token),
        Condition::Expected(id) => machine.expected(*id),
        Condition::Next(item) => machine.lookahead(item)?,
        Condition::AtEnd => machine.at_end(),
        Condition::Equal(left, right) => {
            let left = evaluate_value(left, machine)?;
            let right = evaluate_value(right, machine)?;
            left == right
        }
        Condition::Less(left, right) => {
            let left = integer(&evaluate_value(left, machine)?)?;
            left < integer(&evaluate_value(right, machine)?)?
        }
        Condition::Greater(left, right) => {
            let left = integer(&evaluate_value(left, machine)?)?;
            left > integer(&evaluate_value(right, machine)?)?
        }
        Condition::All(items) => {
            for item in items {
                if !evaluate_condition(item, machine)? {
                    return Ok(false);
                }
            }
            true
        }
        Condition::Some(items) => {
            for item in items {
                if evaluate_condition(item, machine)? {
                    return Ok(true);
                }
            }
            false
        }
        Condition::Not(inner) => !evaluate_condition(inner, machine)?,
    })
}

/// Runs a statement list: `Some(token)` when an `emit` ends the run, `None`
/// when the list runs out.
pub(super) fn run_statements(
    statements: &[Statement],
    machine: &mut dyn Machine,
) -> OpResult<Option<Arc<str>>> {
    for statement in statements {
        machine.step()?;
        match statement {
            Statement::Advance => machine.advance()?,
            Statement::Consume(item) => machine.consume(item)?,
            Statement::Skip(item) => machine.skip(item)?,
            Statement::Mark => machine.mark(),
            Statement::Emit(token) => return Ok(Some(token.clone())),
            Statement::Fail => return Err(OpError::Failed),
            Statement::If(condition, consequent, alternative) => {
                let branch = if evaluate_condition(condition, machine)? {
                    Some(consequent)
                } else {
                    alternative.as_ref()
                };
                if let Some(branch) = branch
                    && let Some(signal) = run_statements(branch, machine)?
                {
                    return Ok(Some(signal));
                }
            }
            Statement::While(condition, body) => {
                while evaluate_condition(condition, machine)? {
                    if let Some(signal) = run_statements(body, machine)? {
                        return Ok(Some(signal));
                    }
                }
            }
            Statement::Push(stack, value) => {
                let value = evaluate_value(value, machine)?;
                machine
                    .state()
                    .stacks
                    .entry(stack.clone())
                    .or_default()
                    .push(value);
            }
            Statement::Pop(stack) => {
                let popped = machine.state().stacks.get_mut(stack).and_then(Vec::pop);
                if popped.is_none() {
                    return Err(OpError::Failed);
                }
            }
            Statement::Set(variable, value) => {
                let value = evaluate_value(value, machine)?;
                machine.state().variables.insert(variable.clone(), value);
            }
            Statement::PushMode(mode) => machine.state().modes.push(mode.clone()),
            Statement::PopMode => {
                let modes = &mut machine.state().modes;
                if modes.len() <= 1 {
                    return Err(OpError::Failed);
                }
                modes.pop();
            }
            Statement::SetMode(mode) => {
                if let Some(last) = machine.state().modes.last_mut() {
                    *last = mode.clone();
                }
            }
            Statement::SetAttribute(name, value) => {
                let value = evaluate_value(value, machine)?;
                machine.set_attribute(name, value);
            }
            Statement::BuildNode(kind) => machine.build_node(kind),
        }
    }
    Ok(None)
}

/// The operations each context may use; loading rejects any other.
pub(super) fn allowed_in(context: &str, operation: &str) -> bool {
    const COMMON: &[&str] = &[
        "if", "while", "push", "pop", "set", "pushMode", "popMode", "setMode", "atEnd", "equal",
        "less", "greater", "all", "some", "not", "integer", "text", "variable", "top", "depth",
        "column", "mode", "length", "number", "add", "subtract", "multiply",
    ];
    const SCANNER: &[&str] = &[
        "advance", "consume", "skip", "mark", "emit", "fail", "valid", "expected", "next",
        "matched",
    ];
    const ACTION: &[&str] = &[
        "fail",
        "setAttribute",
        "buildNode",
        "matched",
        "attribute",
        "sumOf",
        "fieldText",
    ];
    const PREDICATE: &[&str] = &[
        "atEnd", "equal", "less", "greater", "all", "some", "not", "integer", "text", "variable",
        "top", "depth", "column", "matched", "mode", "length", "number", "add", "subtract",
        "multiply",
    ];
    match context {
        "scanner" => COMMON.contains(&operation) || SCANNER.contains(&operation),
        "action" => COMMON.contains(&operation) || ACTION.contains(&operation),
        _ => PREDICATE.contains(&operation),
    }
}

/// The read-only machine conditions see over one result.
pub(super) struct ValueMachine<'e, 'c> {
    executor: &'e mut Executor<'c>,
    working: Working,
    start: usize,
    end: usize,
}

impl<'e, 'c> ValueMachine<'e, 'c> {
    pub(super) fn new(executor: &'e mut Executor<'c>, result: &Res, from: usize) -> Self {
        Self {
            start: content_start(&result.children, from),
            end: result.end,
            working: result.state.working(),
            executor,
        }
    }
}

impl Machine for ValueMachine<'_, '_> {
    fn state(&mut self) -> &mut Working {
        &mut self.working
    }

    fn step(&mut self) -> Result<(), Abort> {
        self.executor.step()
    }

    fn column(&mut self) -> usize {
        column_of(self.executor.bytes, self.start, self.executor.begin)
    }

    fn at_end(&mut self) -> bool {
        self.end == self.executor.end
    }

    fn matched(&mut self) -> OpResult<String> {
        Ok(self.executor.text(self.start, self.end))
    }
}
