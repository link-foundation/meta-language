//! The control flow of imperative JavaScript statements: the variables they
//! share with their context, and the places control leaves them to.
//!
//! Mirrors `js/src/translation/javascript-lower.js`.

use std::collections::HashSet;
use std::rc::Rc;

use serde_json::Value;

use super::lowering::switch_tags;
use super::{CaseTest, SExpr, SNode, Span, Stmt, Switch};

/// Whether statements use `let`, assignments, loops, `break`, `continue` or print.
pub(super) fn imperative(statements: &[Stmt]) -> bool {
    statements.iter().any(|statement| match statement {
        Stmt::Let { .. }
        | Stmt::Assign { .. }
        | Stmt::While { .. }
        | Stmt::DoWhile { .. }
        | Stmt::For { .. }
        | Stmt::Break { .. }
        | Stmt::Continue { .. }
        | Stmt::Print { .. } => true,
        Stmt::Block { body, .. } => imperative(body),
        Stmt::If {
            then, otherwise, ..
        } => imperative(then) || otherwise.as_deref().is_some_and(imperative),
        Stmt::Switch(node) => node.clauses.iter().any(|clause| imperative(&clause.body)),
        _ => false,
    })
}

/// The names statements read or assign (`free`) and assign (`assigned`)
/// without declaring them first, so the ones they share with their context.
#[derive(Default)]
pub(super) struct Uses {
    pub(super) free: HashSet<String>,
    pub(super) assigned: HashSet<String>,
}

pub(super) fn statement_uses(statements: &[Stmt]) -> Uses {
    let mut uses = Uses::default();
    use_list(statements, &mut HashSet::new(), &mut uses);
    uses
}

fn use_list(statements: &[Stmt], scope: &mut HashSet<String>, uses: &mut Uses) {
    for statement in statements {
        use_statement(statement, scope, uses);
    }
}

fn read(expr: &SExpr, scope: &HashSet<String>, uses: &mut Uses) {
    for name in references(expr) {
        if !scope.contains(&name) {
            uses.free.insert(name);
        }
    }
}

fn use_statement(statement: &Stmt, scope: &mut HashSet<String>, uses: &mut Uses) {
    match statement {
        Stmt::Const { name, value, .. } | Stmt::Let { name, value, .. } => {
            read(value, scope, uses);
            scope.insert(name.clone());
        }
        Stmt::Assign { name, value, .. } => {
            read(value, scope, uses);
            if !scope.contains(name) {
                uses.free.insert(name.clone());
                uses.assigned.insert(name.clone());
            }
        }
        Stmt::Return { expr, .. } | Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } => {
            read(expr, scope, uses);
        }
        Stmt::Block { body, inline, .. } => {
            if *inline {
                use_list(body, scope, uses);
            } else {
                use_list(body, &mut scope.clone(), uses);
            }
        }
        Stmt::If {
            cond,
            then,
            otherwise,
            ..
        } => {
            read(cond, scope, uses);
            use_list(then, &mut scope.clone(), uses);
            if let Some(otherwise) = otherwise {
                use_list(otherwise, &mut scope.clone(), uses);
            }
        }
        Stmt::Switch(node) => {
            read(&node.scrutinee, scope, uses);
            for clause in &node.clauses {
                use_list(&clause.body, &mut scope.clone(), uses);
            }
        }
        Stmt::While { cond, body, .. } | Stmt::DoWhile { cond, body, .. } => {
            read(cond, scope, uses);
            use_list(body, &mut scope.clone(), uses);
        }
        Stmt::For {
            init,
            cond,
            update,
            body,
            ..
        } => {
            let mut inner = scope.clone();
            use_list(init, &mut inner, uses);
            if let Some(cond) = cond {
                read(cond, &inner, uses);
            }
            use_list(update, &mut inner, uses);
            use_list(body, &mut inner.clone(), uses);
        }
        Stmt::Throw { .. } | Stmt::Break { .. } | Stmt::Continue { .. } | Stmt::Empty => {}
    }
}

/// An expression as the JSON the JavaScript frontend walks.
pub(super) fn json(expr: &SExpr) -> Value {
    serde_json::to_value(expr).unwrap_or(Value::Null)
}

/// Whether a JSON key holds source that names variables.
pub(super) fn walked(key: &str) -> bool {
    key != "span" && key != "type" && key != "data"
}

/// A single-segment name node's name.
pub(super) fn local_name(map: &serde_json::Map<String, Value>) -> Option<&str> {
    if map.get("k").and_then(Value::as_str) != Some("name") {
        return None;
    }
    match map.get("path").and_then(Value::as_array)?.as_slice() {
        [Value::String(name)] => Some(name),
        _ => None,
    }
}

/// Local variables an expression reads.
fn references(expr: &SExpr) -> Vec<String> {
    fn visit(value: &Value, names: &mut Vec<String>) {
        match value {
            Value::Array(items) => items.iter().for_each(|item| visit(item, names)),
            Value::Object(map) => {
                if let Some(name) = local_name(map) {
                    names.push(name.to_owned());
                }
                for (key, value) in map {
                    if walked(key) {
                        visit(value, names);
                    }
                }
            }
            _ => {}
        }
    }
    let mut names = Vec::new();
    visit(&json(expr), &mut names);
    names
}

/// Statements of a block, with the declarations of `let a, b` inline and empty statements dropped.
pub(super) fn flat(statements: &[Stmt]) -> Rc<Vec<Stmt>> {
    Rc::new(
        statements
            .iter()
            .flat_map(|statement| match statement {
                Stmt::Block {
                    body, inline: true, ..
                } => body.clone(),
                other => vec![other.clone()],
            })
            .filter(|statement| !matches!(statement, Stmt::Empty))
            .collect(),
    )
}

/// How many places control leaves a statement to.
#[derive(Clone, Copy, Default)]
pub(super) struct Exits {
    pub(super) fall: usize,
    pub(super) brk: usize,
    pub(super) cont: usize,
    pub(super) ret: usize,
}

impl Exits {
    pub(super) const fn fall(fall: usize) -> Self {
        Self {
            fall,
            brk: 0,
            cont: 0,
            ret: 0,
        }
    }

    pub(super) const fn sum(self, other: Self) -> Self {
        Self {
            fall: self.fall + other.fall,
            brk: self.brk + other.brk,
            cont: self.cont + other.cont,
            ret: self.ret + other.ret,
        }
    }
}

/// A loop whose condition is always true, which only `break` and `return` leave.
pub(super) fn infinite(cond: Option<&SExpr>) -> bool {
    cond.map_or(true, |cond| {
        matches!(cond.node, SNode::Bool { value: true })
    })
}

/// A loop's condition, body, span, and whether it is a `do … while`.
pub(super) struct LoopParts<'a> {
    pub(super) cond: Option<&'a SExpr>,
    pub(super) body: &'a [Stmt],
    pub(super) update: &'a [Stmt],
    pub(super) place: Span,
    pub(super) do_while: bool,
}

pub(super) fn loop_parts(statement: &Stmt) -> Option<LoopParts<'_>> {
    match statement {
        Stmt::While { cond, body, span } => Some(LoopParts {
            cond: Some(cond),
            body,
            update: &[],
            place: *span,
            do_while: false,
        }),
        Stmt::DoWhile { body, cond, span } => Some(LoopParts {
            cond: Some(cond),
            body,
            update: &[],
            place: *span,
            do_while: true,
        }),
        Stmt::For {
            cond,
            update,
            body,
            span,
            ..
        } => Some(LoopParts {
            cond: cond.as_ref(),
            body,
            update,
            place: *span,
            do_while: false,
        }),
        _ => None,
    }
}

/// How many places control leaves a statement to: falling off its end,
/// `break`, `continue` and `return`.
pub(super) fn exits(statement: &Stmt) -> Exits {
    let nothing = Exits::default();
    match statement {
        Stmt::Return { .. } => Exits { ret: 1, ..nothing },
        Stmt::Throw { .. } => nothing,
        Stmt::Break { .. } => Exits { brk: 1, ..nothing },
        Stmt::Continue { .. } => Exits { cont: 1, ..nothing },
        Stmt::Block { body, .. } => list_exits(&flat(body)),
        Stmt::If {
            then, otherwise, ..
        } => {
            let then = list_exits(&flat(then));
            let otherwise = otherwise
                .as_ref()
                .map_or(Exits::fall(1), |otherwise| list_exits(&flat(otherwise)));
            then.sum(otherwise)
        }
        Stmt::Switch(node) => {
            let bodies = case_bodies(node);
            let count = bodies.len();
            let mut total = Exits::fall(usize::from(!exhaustive(node)));
            for (index, body) in bodies.iter().enumerate() {
                let exits = list_exits(body);
                // `break` leaves the switch, as the last case does by falling off its end.
                let fall = exits.brk + if index + 1 == count { exits.fall } else { 0 };
                total = total.sum(Exits {
                    fall,
                    brk: 0,
                    ..exits
                });
            }
            total
        }
        _ => loop_parts(statement).map_or_else(
            || Exits::fall(1),
            |parts| {
                let body = list_exits(&flat(parts.body));
                Exits {
                    fall: usize::from(!(infinite(parts.cond) && body.brk == 0)),
                    ret: usize::from(body.ret > 0),
                    ..nothing
                }
            },
        ),
    }
}

pub(super) fn list_exits(list: &[Stmt]) -> Exits {
    let mut total = Exits::fall(1);
    for statement in list {
        let exits = exits(statement);
        total = Exits {
            fall: exits.fall,
            ..total.sum(Exits { fall: 0, ..exits })
        };
        if exits.fall == 0 {
            break;
        }
    }
    total
}

/// The statements each case runs: its own, and the next case's when it falls through.
pub(super) fn case_bodies(node: &Switch) -> Vec<Vec<Stmt>> {
    let mut bodies: Vec<Vec<Stmt>> = Vec::new();
    for (index, clause) in node.clauses.iter().enumerate().rev() {
        let mut body = flat(&clause.body).as_ref().clone();
        let falls = index + 1 < node.clauses.len() && list_exits(&body).fall > 0;
        if falls {
            body.extend(bodies[0].iter().cloned());
        }
        bodies.insert(0, body);
    }
    bodies
}

pub(super) fn exhaustive(node: &Switch) -> bool {
    if node.clauses.iter().any(|clause| {
        clause
            .tests
            .iter()
            .any(|test| matches!(test, CaseTest::Default { .. }))
    }) {
        return true;
    }
    let Some((_, data)) = &node.tag else {
        return false;
    };
    let tags = switch_tags(node);
    data.ctors
        .iter()
        .all(|ctor| tags.contains(&ctor.name.as_str()))
}
