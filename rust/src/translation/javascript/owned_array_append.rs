//! Actual lexical allocations and occurrences supplied to shared construction rules.
//! Mirrors the source transport in `owned-array-append.js`; admission is generated.

use super::callables::number;
use super::{CaseTest, HashMap, HashSet, ROOT, SArrayItem, SExpr, SFn, SNode, SParam, Span, Stmt};
use crate::translation::owned_array_append_rules as rules;
use serde_json::Value;

pub(super) type Readers = HashMap<String, HashSet<usize>>;
type Scope = HashMap<String, Option<usize>>;

#[derive(Clone, Copy, Default)]
struct Use<'a> {
    context: &'a str,
    operation: &'a str,
    reader: bool,
    captured: bool,
}

fn text<'a>(value: &'a Value, field: &str) -> &'a str {
    value.get(field).and_then(Value::as_str).unwrap_or("")
}

/// Walk the actual serialized expression fields without constructing replacement ASTs.
fn names(
    value: &Value,
    readers: &Readers,
    locals: &HashSet<String>,
    usage: Use<'_>,
    parameter: Option<&str>,
    record: &mut impl FnMut(&str, bool),
) {
    if let Value::Array(values) = value {
        for value in values {
            names(value, readers, locals, usage, parameter, record);
        }
        return;
    }
    let Some(fields) = value.as_object() else {
        return;
    };
    let kind = text(value, "k");
    if kind == "name"
        && let Some(path) = value.get("path").and_then(Value::as_array)
        && path.len() == 1
    {
        record(
            path[0]
                .as_str()
                .expect("a source name retains its actual string"),
            !usage.captured
                && rules::accept_owned_array_read(usage.context, usage.operation, usage.reader),
        );
        return;
    }
    let ordinary = Use {
        captured: usage.captured,
        ..Use::default()
    };
    if kind == "app" {
        names(&value["fn"], readers, locals, ordinary, parameter, record);
        let target = value["fn"]["path"]
            .as_array()
            .filter(|path| text(&value["fn"], "k") == "name" && path.len() == 2 && path[0] == ROOT)
            .and_then(|path| path[1].as_str());
        if let Some(arguments) = value.get("args").and_then(Value::as_array) {
            for (index, argument) in arguments.iter().enumerate() {
                let reader = target.is_some_and(|target| {
                    !locals.contains(target)
                        && readers
                            .get(target)
                            .is_some_and(|slots| slots.contains(&index))
                });
                names(
                    argument,
                    readers,
                    locals,
                    Use {
                        context: "argument",
                        reader,
                        ..ordinary
                    },
                    parameter,
                    record,
                );
            }
        }
        return;
    }
    if kind == "let" && parameter.is_some() {
        names(
            &value["value"],
            readers,
            locals,
            ordinary,
            parameter,
            record,
        );
        let name = text(value, "name");
        if Some(name) == parameter {
            record(name, false);
        }
        let mut inner = locals.clone();
        inner.insert(name.to_owned());
        names(&value["body"], readers, &inner, ordinary, parameter, record);
        return;
    }
    if kind == "index" {
        names(
            &value["object"],
            readers,
            locals,
            Use {
                context: "index-receiver",
                ..ordinary
            },
            parameter,
            record,
        );
        names(
            &value["index"],
            readers,
            locals,
            ordinary,
            parameter,
            record,
        );
        return;
    }
    if kind == "field" {
        let context = if text(value, "field") == "length" {
            "length-receiver"
        } else {
            ""
        };
        names(
            &value["object"],
            readers,
            locals,
            Use {
                context,
                ..ordinary
            },
            parameter,
            record,
        );
        return;
    }
    if kind == "method" || kind == "stringTest" {
        names(
            &value["object"],
            readers,
            locals,
            Use {
                context: "method-receiver",
                operation: text(value, "op"),
                ..ordinary
            },
            parameter,
            record,
        );
        if let Some(arguments) = value.get("args").and_then(Value::as_array) {
            for argument in arguments {
                names(argument, readers, locals, ordinary, parameter, record);
            }
        } else {
            names(
                &value["search"],
                readers,
                locals,
                ordinary,
                parameter,
                record,
            );
        }
        return;
    }
    for (field, child) in fields {
        if !["span", "type", "ret", "from", "to"].contains(&field.as_str()) {
            names(
                child,
                readers,
                locals,
                Use {
                    captured: usage.captured || kind == "lambda",
                    ..Use::default()
                },
                parameter,
                record,
            );
        }
    }
}

/// Inspect actual source occurrences before loop lowering introduces helper calls.
fn source_names(
    statements: &[Stmt],
    readers: &Readers,
    locals: &mut HashSet<String>,
    parameter: &str,
    record: &mut impl FnMut(&str, bool),
) {
    let expression = |value: &SExpr, locals: &HashSet<String>, record: &mut _| {
        let value =
            serde_json::to_value(value).expect("an actual reader expression retains its fields");
        names(
            &value,
            readers,
            locals,
            Use::default(),
            Some(parameter),
            record,
        );
    };
    for statement in statements {
        match statement {
            Stmt::Const { name, value, .. } | Stmt::Let { name, value, .. } => {
                expression(value, locals, record);
                if name == parameter {
                    record(name, false);
                }
                locals.insert(name.clone());
            }
            Stmt::Assign { name, value, .. } => {
                expression(value, locals, record);
                if name == parameter {
                    record(name, false);
                }
            }
            Stmt::Block { body, inline, .. } => {
                if *inline {
                    source_names(body, readers, locals, parameter, record);
                } else {
                    source_names(body, readers, &mut locals.clone(), parameter, record);
                }
            }
            Stmt::If {
                cond,
                then,
                otherwise,
                ..
            } => {
                expression(cond, locals, record);
                source_names(then, readers, &mut locals.clone(), parameter, record);
                if let Some(otherwise) = otherwise {
                    source_names(otherwise, readers, &mut locals.clone(), parameter, record);
                }
            }
            Stmt::While { cond, body, .. } | Stmt::DoWhile { cond, body, .. } => {
                expression(cond, locals, record);
                source_names(body, readers, &mut locals.clone(), parameter, record);
            }
            Stmt::For {
                init,
                cond,
                body,
                update,
                ..
            } => {
                let mut inner = locals.clone();
                source_names(init, readers, &mut inner, parameter, record);
                if let Some(cond) = cond {
                    expression(cond, &inner, record);
                }
                source_names(body, readers, &mut inner.clone(), parameter, record);
                source_names(update, readers, &mut inner, parameter, record);
            }
            Stmt::Switch(switch) => {
                expression(&switch.scrutinee, locals, record);
                for clause in &switch.clauses {
                    for test in &clause.tests {
                        if let CaseTest::Value { value, .. } = test {
                            expression(value, locals, record);
                        }
                    }
                    source_names(
                        &clause.body,
                        readers,
                        &mut locals.clone(),
                        parameter,
                        record,
                    );
                }
            }
            Stmt::Return { expr, .. } | Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } => {
                expression(expr, locals, record);
            }
            Stmt::Break { .. } | Stmt::Continue { .. } | Stmt::Throw { .. } | Stmt::Empty => {}
        }
    }
}

pub(super) fn read_owned_array_reader_parameters(
    declaration: &SFn,
    readers: &Readers,
    statements: &[Stmt],
) -> HashSet<usize> {
    let mut admitted = HashSet::new();
    if !rules::accept_owned_array_reader_result(
        declaration.ret.as_ref().map_or("", super::Type::kind),
    ) {
        return admitted;
    }
    let locals: HashSet<String> = declaration
        .params
        .iter()
        .map(|parameter| parameter.name.clone())
        .collect();
    for (index, parameter) in declaration.params.iter().enumerate() {
        let mut safe = true;
        source_names(
            statements,
            readers,
            &mut locals.clone(),
            &parameter.name,
            &mut |name, allowed| {
                if name == parameter.name && !allowed {
                    safe = false;
                }
            },
        );
        if safe {
            admitted.insert(index);
        }
    }
    admitted
}

struct Escape {
    position: f64,
    loops: Vec<f64>,
}

struct Allocation {
    name: String,
    loops: Vec<f64>,
    escapes: Vec<Escape>,
}

struct Append {
    statement: *const Stmt,
    allocation: usize,
    loops: Vec<f64>,
    position: f64,
    operation_span: Option<Span>,
}

#[derive(Default)]
struct Occurrences {
    allocations: Vec<Allocation>,
    appends: Vec<Append>,
    position: usize,
    next_loop: usize,
}

impl Occurrences {
    fn advance(&mut self) -> f64 {
        let position = number(self.position);
        self.position += 1;
        position
    }

    fn escape(&mut self, allocation: usize, loops: &[f64]) {
        let position = self.advance();
        self.allocations[allocation].escapes.push(Escape {
            position,
            loops: loops.to_vec(),
        });
    }

    fn read(&mut self, expression: &SExpr, scope: &Scope, loops: &[f64], readers: &Readers) {
        let value = serde_json::to_value(expression)
            .expect("actual source expressions retain their fields");
        let locals = scope.keys().cloned().collect();
        names(
            &value,
            readers,
            &locals,
            Use::default(),
            None,
            &mut |name, allowed| {
                if !allowed && let Some(Some(allocation)) = scope.get(name) {
                    self.escape(*allocation, loops);
                }
            },
        );
    }

    /// Source statements supply lexical owners, actual loop ancestry and occurrence order.
    fn list(&mut self, statements: &[Stmt], scope: &mut Scope, loops: &[f64], readers: &Readers) {
        for statement in statements {
            match statement {
                Stmt::Const { name, value, .. } | Stmt::Let { name, value, .. } => {
                    self.read(value, scope, loops, readers);
                    let allocation = if matches!(value.node, SNode::Array { .. }) {
                        let id = self.allocations.len();
                        self.allocations.push(Allocation {
                            name: name.clone(),
                            loops: loops.to_vec(),
                            escapes: Vec::new(),
                        });
                        self.advance();
                        Some(id)
                    } else {
                        None
                    };
                    scope.insert(name.clone(), allocation);
                }
                Stmt::Expr { expr, .. } if append_shape(expr) => {
                    let SNode::Method { object, args, .. } = &expr.node else {
                        unreachable!("the generated shape names an actual source method")
                    };
                    let allocation = object
                        .simple_name()
                        .and_then(|name| scope.get(name))
                        .copied()
                        .flatten();
                    for argument in args.iter().flatten() {
                        self.read(argument, scope, loops, readers);
                    }
                    if let Some(allocation) = allocation {
                        let position = self.advance();
                        self.appends.push(Append {
                            statement: std::ptr::from_ref(statement),
                            allocation,
                            loops: loops.to_vec(),
                            position,
                            operation_span: object
                                .span
                                .zip(
                                    args.first()
                                        .and_then(|argument| argument.as_ref())
                                        .and_then(|argument| argument.span),
                                )
                                .map(|(object, argument)| Span::new(object.end, argument.start)),
                        });
                    }
                }
                Stmt::Assign { name, value, .. } => {
                    self.read(value, scope, loops, readers);
                    if let Some(Some(allocation)) = scope.get(name) {
                        self.escape(*allocation, loops);
                    }
                }
                Stmt::Return { .. }
                | Stmt::Throw { .. }
                | Stmt::Break { .. }
                | Stmt::Continue { .. } => break,
                Stmt::Block { body, inline, .. } => {
                    if *inline {
                        self.list(body, scope, loops, readers);
                    } else {
                        self.list(body, &mut scope.clone(), loops, readers);
                    }
                }
                Stmt::If {
                    cond,
                    then,
                    otherwise,
                    ..
                } => {
                    self.read(cond, scope, loops, readers);
                    self.list(then, &mut scope.clone(), loops, readers);
                    if let Some(otherwise) = otherwise {
                        self.list(otherwise, &mut scope.clone(), loops, readers);
                    }
                }
                Stmt::While { cond, body, .. } | Stmt::DoWhile { cond, body, .. } => {
                    let mut nested = loops.to_vec();
                    nested.push(number(self.next_loop));
                    self.next_loop += 1;
                    let mut inner = scope.clone();
                    self.read(cond, &inner, &nested, readers);
                    self.list(body, &mut inner, &nested, readers);
                }
                Stmt::For {
                    init,
                    cond,
                    update,
                    body,
                    ..
                } => {
                    let mut nested = loops.to_vec();
                    nested.push(number(self.next_loop));
                    self.next_loop += 1;
                    let mut inner = scope.clone();
                    self.list(init, &mut inner, &nested, readers);
                    if let Some(cond) = cond {
                        self.read(cond, &inner, &nested, readers);
                    }
                    self.list(body, &mut inner, &nested, readers);
                    self.list(update, &mut inner, &nested, readers);
                }
                Stmt::Switch(switch) => {
                    self.read(&switch.scrutinee, scope, loops, readers);
                    for clause in &switch.clauses {
                        for test in &clause.tests {
                            if let CaseTest::Value { value, .. } = test {
                                self.read(value, scope, loops, readers);
                            }
                        }
                        self.list(&clause.body, &mut scope.clone(), loops, readers);
                    }
                }
                Stmt::Expr { expr, .. } | Stmt::Print { expr, .. } => {
                    self.read(expr, scope, loops, readers);
                }
                Stmt::Empty => {}
            }
        }
    }
}

fn append_shape(expression: &SExpr) -> bool {
    let (operation, object, count) = match &expression.node {
        SNode::Method { op, object, args } => (op.as_str(), Some(object.as_ref()), args.len()),
        _ => ("", None, 0),
    };
    let path = match object.map(|object| &object.node) {
        Some(SNode::Name { path }) => path.len(),
        _ => 0,
    };
    rules::accept_owned_array_append_shape(
        "expr",
        expression.node.kind(),
        operation,
        object.map_or("", |object| object.node.kind()),
        number(path),
        number(count),
    )
}

pub(super) fn lower_owned_array_appends(
    statements: &mut [Stmt],
    parameters: &[SParam],
    readers: &Readers,
) -> Vec<Span> {
    let mut occurrences = Occurrences::default();
    let mut scope = parameters
        .iter()
        .map(|parameter| (parameter.name.clone(), None))
        .collect();
    occurrences.list(statements, &mut scope, &[], readers);
    let mut selected = HashMap::new();
    let mut operations = Vec::new();
    for append in &occurrences.appends {
        let allocation = &occurrences.allocations[append.allocation];
        let mut loops = Vec::new();
        let mut starts = Vec::new();
        let mut counts = Vec::new();
        let mut positions = Vec::new();
        for escape in &allocation.escapes {
            starts.push(number(loops.len()));
            counts.push(number(escape.loops.len()));
            loops.extend(&escape.loops);
            positions.push(escape.position);
        }
        if rules::accept_owned_array_append_uses(
            &allocation.loops,
            &append.loops,
            &loops,
            &starts,
            &counts,
            &positions,
            append.position,
        ) {
            selected.insert(append.statement, allocation.name.clone());
            operations.extend(append.operation_span);
        }
    }
    replace(statements, &selected);
    operations
}

/// Replace only actual admitted statement owners, retaining every child and span.
fn replace(statements: &mut [Stmt], selected: &HashMap<*const Stmt, String>) {
    for statement in statements {
        if let Some(name) = selected.get(&std::ptr::from_ref::<Stmt>(statement)) {
            let original = std::mem::replace(statement, Stmt::Empty);
            let Stmt::Expr { expr, span } = original else {
                unreachable!("an append owner is an expression statement")
            };
            let place = expr.span;
            let SNode::Method { object, args, .. } = expr.node else {
                unreachable!("the admitted owner retains its method")
            };
            let mut items = vec![SArrayItem::new(*object, true)];
            items.extend(args.into_iter().map(|argument| {
                SArrayItem::without_spread(
                    argument.expect("parsed source arguments retain their actual expressions"),
                )
            }));
            *statement = Stmt::Assign {
                name: name.clone(),
                value: SExpr::new(
                    SNode::Array {
                        items,
                        element: None,
                    },
                    place,
                ),
                span,
            };
            continue;
        }
        match statement {
            Stmt::Block { body, .. } | Stmt::While { body, .. } | Stmt::DoWhile { body, .. } => {
                replace(body, selected);
            }
            Stmt::If {
                then, otherwise, ..
            } => {
                replace(then, selected);
                if let Some(otherwise) = otherwise {
                    replace(otherwise, selected);
                }
            }
            Stmt::For {
                init, body, update, ..
            } => {
                replace(init, selected);
                replace(body, selected);
                replace(update, selected);
            }
            Stmt::Switch(switch) => {
                for clause in &mut switch.clauses {
                    replace(&mut clause.body, selected);
                }
            }
            _ => {}
        }
    }
}
