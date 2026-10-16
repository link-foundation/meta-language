//! Imperative JavaScript function bodies to one expression. `let` variables,
//! assignments, loops, `break` and `continue` have a faithful functional
//! reading: an assignment `x = e` binds a new `x` for the statements after
//! it; a statement after which control continues at several places (an `if`
//! that assigns in both branches, a loop that breaks at two points) is joined
//! by a data type whose alternatives carry the variables it assigns, so the
//! statements after it are translated once; and a loop is a tail-recursive
//! function of the variables it uses, lifted to the top level, whose result
//! carries the variables it assigns back to the statements after it, or the
//! value the loop returns from the function. Only the variables a later
//! statement may read are carried. The variables of a block that shadow a
//! variable in scope are renamed, so no later statement sees them.
//!
//! Mirrors `js/src/translation/javascript-lower.js`.

use std::collections::HashSet;
use std::rc::Rc;

use super::flow::{
    Exits, Uses, case_bodies, exhaustive, exits, flat, infinite, list_exits, loop_parts,
    statement_uses,
};
use super::lowering::{
    TagSwitch, default_rows, lower_tag_if, read_value_case_expression,
    read_value_switch_instruction, read_value_switch_position, statement_span, switch_tags,
    tag_condition, tag_row,
};
use super::{
    BinaryOp, CaseTest, JavaScriptParser, ROOT, Result, SCtor, SData, SExpr, SField, SFn, SItem,
    SNode, SParam, SPattern, SPatternNode, SRow, Span, Stmt, Switch, TranslationError, Type, node,
    unsupported, wild,
};

mod loops;
mod top_level;
pub(super) use self::top_level::lower_top_level;

/// A variable in scope: its source name and the name it has in the translation.
#[derive(Clone)]
struct Var {
    source: String,
    ir: String,
    reference: bool,
}

/// The variable a source name refers to in a scope: the latest entry for it.
fn lookup<'a>(scope: &'a [Var], name: &str) -> Option<&'a Var> {
    scope.iter().rev().find(|entry| entry.source == name)
}

/// The variables of a scope, each once, in the order they were declared.
fn visible(scope: &[Var]) -> Vec<Var> {
    scope
        .iter()
        .enumerate()
        .filter(|(index, entry)| {
            scope.iter().rposition(|other| other.source == entry.source) == Some(*index)
        })
        .map(|(_, entry)| entry.clone())
        .collect()
}

/// A copy of a source expression that reads the variables it names in scope.
fn renamed(expr: &SExpr, scope: &[Var]) -> Result<SExpr> {
    let scope = serde_json::Value::Array(scope.iter().map(|entry| serde_json::json!({"source":entry.source,"ir":entry.ir,"reference":entry.reference})).collect());
    super::callables::rename_expression(expr, scope)
}

fn named(name: &str, place: Span) -> SExpr {
    node(
        SNode::Name {
            path: vec![name.to_owned()],
        },
        place,
    )
}

fn internal(message: &str, place: Span) -> TranslationError {
    TranslationError::syntax(message, Some(place))
}

fn union<'a>(sets: impl IntoIterator<Item = &'a HashSet<String>>) -> HashSet<String> {
    sets.into_iter().flatten().cloned().collect()
}

type Fall = Rc<dyn Fn(&mut Lowering, &[Var]) -> Result<SExpr>>;
type Give = Rc<dyn Fn(&mut Lowering, SExpr) -> Result<SExpr>>;
type Jump = Rc<dyn Fn(&mut Lowering) -> Result<SExpr>>;
/// Where a lowered loop is called, given the lowering of what follows it.
type Site = Box<dyn FnOnce(&mut Lowering) -> Result<SExpr>>;

/// What falling off the end of statements, `return`, `break` and
/// `continue` do, and the source names each continuation may read: `live`
/// after falling off the end, `brk_live` after `break`, `cont_live` after
/// `continue`.
#[derive(Clone)]
struct Ctx {
    fall: Fall,
    ret: Give,
    brk: Jump,
    cont: Jump,
    live: Rc<HashSet<String>>,
    brk_live: Rc<HashSet<String>>,
    cont_live: Rc<HashSet<String>>,
}

/// The body of the function `name` of `params` from its statements.
pub(super) fn lower_imperative(
    parser: &mut JavaScriptParser,
    name: &str,
    params: &[SParam],
    statements: &[Stmt],
    place: Span,
) -> Result<SExpr> {
    lower_imperative_with_captures(parser, name, params, statements, place, &[], &[])
}

pub(super) fn lower_imperative_with_captures(
    parser: &mut JavaScriptParser,
    name: &str,
    params: &[SParam],
    statements: &[Stmt],
    place: Span,
    external_cells: &[String],
    external_bindings: &[String],
) -> Result<SExpr> {
    let mut cells = super::callables::mutable_cells(statements);
    cells.extend(external_cells.iter().cloned());
    if !cells.is_empty() {
        parser.reference_semantics = true;
        parser.reference_semantics_forced = true;
    }
    let mut lowering = Lowering {
        function: name.to_owned(),
        renames: 0,
        generated: std::mem::take(&mut parser.generated),
        count: parser.generated_count,
        cells,
    };
    let ctx = Ctx {
        // A function that finishes without returning returns undefined, the unit value.
        fall: Rc::new(move |_, _| Ok(node(SNode::Unit, place))),
        ret: Rc::new(|_, value| Ok(value)),
        brk: Rc::new(move |_| Err(internal("break outside a loop", place))),
        cont: Rc::new(move |_| Err(internal("continue outside a loop", place))),
        live: Rc::default(),
        brk_live: Rc::default(),
        cont_live: Rc::default(),
    };
    let mut captured = HashSet::new();
    let mut scope: Vec<Var> = external_cells
        .iter()
        .chain(external_bindings)
        .filter(|name| {
            captured.insert((*name).clone())
                && !params.iter().any(|parameter| parameter.name == **name)
        })
        .map(|name| Var {
            source: name.clone(),
            ir: name.clone(),
            reference: external_cells.contains(name),
        })
        .collect();
    scope.extend(params.iter().map(|param| Var {
        source: param.name.clone(),
        ir: crate::translation::callable_source_rules::render_callable_cell_parameter_name(
            &param.name,
            lowering.cells.contains(&param.name),
        ),
        reference: lowering.cells.contains(&param.name),
    }));
    let mut body = lowering.seq(&flat(statements), 0, &ctx, &scope)?;
    for parameter in params.iter().rev() {
        let allocation = crate::translation::callable_source_rules::read_callable_cell_lowering(
            "declare",
            lowering.cells.contains(&parameter.name),
        );
        if !allocation.is_empty() {
            let name =
                crate::translation::callable_source_rules::render_callable_cell_parameter_name(
                    &parameter.name,
                    true,
                );
            body = let_node(
                name,
                node(
                    SNode::CellAllocate {
                        value: Box::new(named(&parameter.name, place)),
                        ty: None,
                    },
                    place,
                ),
                body,
                place,
            );
        }
    }
    parser.generated = lowering.generated;
    parser.generated_count = lowering.count;
    Ok(body)
}

struct Lowering {
    function: String,
    renames: usize,
    generated: Vec<SItem>,
    count: usize,
    cells: HashSet<String>,
}

/// A generated data type's alternative, which carries `variables`.
struct Alternative {
    ctor: String,
    variables: Vec<String>,
    place: Span,
}

impl Alternative {
    fn make(&self, values: Vec<SExpr>) -> SExpr {
        node(
            SNode::CtorObject {
                tag: self.ctor.clone(),
                fields: self.variables.iter().cloned().zip(values).collect(),
                frozen: false,
                prototype_definition: None,
            },
            self.place,
        )
    }
}

/// The field of a generated constructor, whose type inference fills in.
fn untyped(name: &str) -> SField {
    SField {
        name: Some(name.to_owned()),
        ty: Type::Literal,
        rocq_type: None,
        span: None,
    }
}

/// The row `ROOT::data::ctor(variables…) => body`.
fn ctor_row(data: &str, ctor: &str, variables: &[String], body: SExpr, place: Span) -> SRow {
    let args = variables
        .iter()
        .map(|name| SPattern {
            node: SPatternNode::Bind { name: name.clone() },
            span: Some(place),
        })
        .collect();
    SRow {
        patterns: vec![SPattern {
            node: SPatternNode::Ctor {
                path: vec![ROOT.to_owned(), data.to_owned(), ctor.to_owned()],
                args,
            },
            span: Some(place),
        }],
        body,
        span: Some(place),
    }
}

fn with_fall(ctx: &Ctx, fall: Fall) -> Ctx {
    Ctx {
        fall,
        ..ctx.clone()
    }
}

/// A continuation that falls off the end in `ctx` with the variables of `scope`.
fn fall_in(ctx: &Ctx, scope: &[Var]) -> Fall {
    let ctx = ctx.clone();
    let scope = scope.to_vec();
    Rc::new(move |lowering, _| (ctx.fall)(lowering, &scope))
}

impl Lowering {
    /// A declaration in scope; one that shadows a variable in scope gets a new name.
    fn declare(&mut self, scope: &[Var], source: &str) -> Vec<Var> {
        let ir = if lookup(scope, source).is_some() {
            self.renames += 1;
            format!("ml_{source}_{}", self.renames)
        } else {
            source.to_owned()
        };
        let mut inner = scope.to_vec();
        inner.push(Var {
            source: source.to_owned(),
            ir,
            reference: self.cells.contains(source),
        });
        inner
    }

    fn generated_name(&mut self, kind: &str) -> String {
        self.count += 1;
        format!("ml_{}_{kind}{}", self.function, self.count)
    }

    /// The statements from `index` on; `ctx` says what falling off their end,
    /// `break`, `continue` and `return` do, and which variables are live there.
    fn seq(
        &mut self,
        list: &Rc<Vec<Stmt>>,
        index: usize,
        ctx: &Ctx,
        scope: &[Var],
    ) -> Result<SExpr> {
        let Some(statement) = list.get(index) else {
            return (ctx.fall)(self, scope);
        };
        let at_end = index + 1 == list.len();
        let exits = exits(statement);
        if !at_end && exits.fall == 0 {
            return Err(unsupported(
                "unreachable statement",
                "statements after return, throw, break, continue or a complete if are never executed",
                statement_span(&list[index + 1]),
            ));
        }
        let rest: Fall = {
            let list = Rc::clone(list);
            let ctx = ctx.clone();
            let scope = scope.to_vec();
            Rc::new(move |lowering, _| lowering.seq(&list, index + 1, &ctx, &scope))
        };
        match statement {
            Stmt::Const { name, value, span } | Stmt::Let { name, value, span } => {
                let value = renamed(value, scope)?;
                let inner = self.declare(scope, name);
                let ir = inner[inner.len() - 1].ir.clone();
                let body = self.seq(list, index + 1, ctx, &inner)?;
                let operation =
                    crate::translation::callable_source_rules::read_callable_cell_lowering(
                        "declare",
                        inner[inner.len() - 1].reference,
                    );
                let value = if operation.is_empty() {
                    value
                } else {
                    node(
                        SNode::CellAllocate {
                            value: Box::new(value),
                            ty: None,
                        },
                        *span,
                    )
                };
                return Ok(let_node(ir, value, body, *span));
            }
            Stmt::Assign { name, value, span } => {
                let ir = lookup(scope, name).map_or_else(|| name.clone(), |var| var.ir.clone());
                let value = renamed(value, scope)?;
                let body = rest(self, scope)?;
                let reference = lookup(scope, name).is_some_and(|entry| entry.reference);
                let operation =
                    crate::translation::callable_source_rules::read_callable_cell_lowering(
                        "assign", reference,
                    );
                if !operation.is_empty() {
                    return Ok(node(
                        SNode::CellWrite {
                            cell: Box::new(named(&ir, *span)),
                            value: Box::new(value),
                            body: Box::new(body),
                        },
                        *span,
                    ));
                }
                return Ok(let_node(ir, value, body, *span));
            }
            Stmt::Return { expr, .. } => {
                let value = renamed(expr, scope)?;
                return (ctx.ret)(self, value);
            }
            Stmt::Throw { message, span } => {
                return Ok(node(
                    SNode::Abort {
                        message: message.clone(),
                    },
                    *span,
                ));
            }
            Stmt::Break { .. } => return (ctx.brk)(self),
            Stmt::Continue { .. } => return (ctx.cont)(self),
            Stmt::Print { expr, style, span } => {
                let expr = renamed(expr, scope)?;
                let body = rest(self, scope)?;
                return Ok(node(
                    SNode::Print {
                        expr: Box::new(expr),
                        style: *style,
                        body: Box::new(body),
                    },
                    *span,
                ));
            }
            // A statement that discards its value still runs it, for the lines
            // it prints and the aborts it may reach.
            Stmt::Expr { expr, span } => {
                let name =
                    self.generated_name(if matches!(expr.node, SNode::ReferenceWrite { .. }) {
                        "written"
                    } else {
                        "ignored"
                    });
                let value = renamed(expr, scope)?;
                let body = rest(self, scope)?;
                return Ok(let_node(name, value, body, *span));
            }
            _ => {}
        }
        if at_end {
            return self.compound(statement, ctx, scope);
        }
        let later = statement_uses(&list[index + 1..]).free;
        let live = Rc::new(union([&later, ctx.live.as_ref()]));
        if exits.fall == 1 {
            let inner = Ctx {
                fall: rest,
                live,
                ..ctx.clone()
            };
            return self.compound(statement, &inner, scope);
        }
        let inner = Ctx {
            live,
            ..ctx.clone()
        };
        self.join(statement, exits, &inner, scope, &rest)
    }

    fn compound(&mut self, statement: &Stmt, ctx: &Ctx, scope: &[Var]) -> Result<SExpr> {
        match statement {
            Stmt::Block { body, .. } => {
                self.seq(&flat(body), 0, &with_fall(ctx, fall_in(ctx, scope)), scope)
            }
            Stmt::If {
                cond,
                then,
                otherwise,
                span,
            } => {
                let cond = renamed(cond, scope)?;
                let block = with_fall(ctx, fall_in(ctx, scope));
                let then = self.seq(&flat(then), 0, &block, scope)?;
                let otherwise = match otherwise {
                    Some(otherwise) => self.seq(&flat(otherwise), 0, &block, scope)?,
                    None => (ctx.fall)(self, scope)?,
                };
                if let Some(test) = tag_condition(&cond) {
                    return lower_tag_if(&test.clone(), then, otherwise, *span);
                }
                Ok(node(
                    SNode::If {
                        cond: Box::new(cond),
                        then: Box::new(then),
                        otherwise: Box::new(otherwise),
                    },
                    *span,
                ))
            }
            Stmt::Switch(switch) => self.lower_switch(switch, ctx, scope),
            Stmt::While { .. } | Stmt::DoWhile { .. } => {
                self.lower_loop(statement, ctx, scope, &HashSet::new())
            }
            Stmt::For { init, .. } => {
                // The head's variables belong to the loop, not to the statements after it.
                let local: HashSet<String> = init
                    .iter()
                    .filter_map(|init| match init {
                        Stmt::Let { name, .. } => Some(name.clone()),
                        _ => None,
                    })
                    .collect();
                let mut headless = statement.clone();
                if let Stmt::For { init, .. } = &mut headless {
                    init.clear();
                }
                let later = statement_uses(&[headless]).free;
                let live = Rc::new(union([&later, ctx.live.as_ref()]));
                let fall: Fall = {
                    let statement = statement.clone();
                    let ctx = ctx.clone();
                    Rc::new(move |lowering, inner| {
                        lowering.lower_loop(&statement, &ctx, inner, &local)
                    })
                };
                let inner = Ctx {
                    fall,
                    live,
                    ..ctx.clone()
                };
                self.seq(&flat(init), 0, &inner, scope)
            }
            _ => Err(TranslationError::syntax(
                "unknown statement",
                statement_span(statement),
            )),
        }
    }

    /// A switch is a match whose rows run the cases; `break` in a case continues after the switch.
    fn lower_switch(&mut self, node: &Switch, ctx: &Ctx, scope: &[Var]) -> Result<SExpr> {
        let scrutinee = renamed(&node.scrutinee, scope)?;
        let inner = Ctx {
            fall: fall_in(ctx, scope),
            brk: {
                let fall = fall_in(ctx, scope);
                Rc::new(move |lowering| fall(lowering, &[]))
            },
            brk_live: Rc::clone(&ctx.live),
            ..ctx.clone()
        };
        if super::surface_rules::accept_java_script_value_switch(
            node.tag.is_some(),
            node.clauses.iter().any(|clause| {
                clause
                    .tests
                    .iter()
                    .any(|test| matches!(test, CaseTest::Value { .. }))
            }),
        ) {
            return self.lower_value_switch(node, ctx, scope, scrutinee, &inner);
        }
        let tag_switch = node.tag.as_ref().map(|(_, data)| TagSwitch {
            subject: scrutinee.simple_name().unwrap_or_default(),
            data,
            covered: switch_tags(node),
        });
        let bodies = case_bodies(node);
        let mut rows = Vec::new();
        let mut defaults = Vec::new();
        for (clause, body) in node.clauses.iter().zip(bodies) {
            let lowered = self.seq(&Rc::new(body), 0, &inner, scope)?;
            for test in &clause.tests {
                let body = lowered.clone();
                match (test, &tag_switch) {
                    (CaseTest::Default { span }, tag_switch) => {
                        defaults = match tag_switch {
                            Some(tag_switch) => default_rows(tag_switch, body, clause.span)?,
                            None => vec![SRow {
                                patterns: vec![wild(*span)],
                                body,
                                span: Some(clause.span),
                            }],
                        };
                    }
                    (CaseTest::Tag { tag, span }, Some(tag_switch)) => {
                        rows.push(tag_row(tag_switch, tag, *span, &body, clause.span)?);
                    }
                    (CaseTest::Tag { .. }, None) => {}
                    (CaseTest::Value { span, .. }, _) => {
                        return Err(TranslationError::syntax(
                            "internal: an unlowered value switch case",
                            Some(*span),
                        ));
                    }
                    (
                        CaseTest::NumLit {
                            value,
                            negative,
                            span,
                        },
                        _,
                    ) => rows.push(literal_row(
                        SPatternNode::NumLit {
                            value: value.clone(),
                            negative: *negative,
                        },
                        *span,
                        body,
                        clause.span,
                    )),
                    (CaseTest::BoolLit { value, span }, _) => rows.push(literal_row(
                        SPatternNode::BoolLit {
                            value: *value,
                            negative: false,
                        },
                        *span,
                        body,
                        clause.span,
                    )),
                }
            }
        }
        rows.extend(defaults);
        if !exhaustive(node) {
            rows.push(SRow {
                patterns: vec![wild(node.span)],
                body: (ctx.fall)(self, scope)?,
                span: Some(node.span),
            });
        }
        Ok(SExpr::new(
            SNode::Match {
                scrutinees: vec![scrutinee.clone()],
                rows,
            },
            Some(node.span),
        ))
    }

    /// Construct existing nodes in the order selected by the generated cursor.
    fn lower_value_switch(
        &mut self,
        switch: &Switch,
        ctx: &Ctx,
        scope: &[Var],
        scrutinee: SExpr,
        inner: &Ctx,
    ) -> Result<SExpr> {
        let bodies = case_bodies(switch);
        let fallback = switch.clauses.iter().position(|clause| {
            clause
                .tests
                .iter()
                .any(|test| matches!(test, CaseTest::Default { .. }))
        });
        let mut variable = String::new();
        let mut otherwise = None;
        let mut condition = None;
        let mut branch = None;
        for step in 0.. {
            let instruction = read_value_switch_instruction(true, step, switch.clauses.len());
            match instruction.as_str() {
                "capture" => variable = self.generated_name("switch"),
                "fallback" => {
                    otherwise = Some((ctx.fall)(self, scope)?);
                    if let Some(index) = fallback {
                        otherwise =
                            Some(self.seq(&Rc::new(bodies[index].clone()), 0, inner, scope)?);
                    }
                }
                "bind" => {
                    return Ok(node(
                        SNode::Let {
                            name: variable,
                            ty: None,
                            value: Box::new(scrutinee),
                            body: Box::new(
                                otherwise.expect("the shared cursor reads its fallback first"),
                            ),
                        },
                        switch.span,
                    ));
                }
                "test" | "body" | "select" => {
                    let index = read_value_switch_position(true, step, switch.clauses.len());
                    let tests: Vec<_> = switch.clauses[index]
                        .tests
                        .iter()
                        .filter(|test| !matches!(test, CaseTest::Default { .. }))
                        .collect();
                    if tests.is_empty() {
                        continue;
                    }
                    match instruction.as_str() {
                        "test" => {
                            let mut comparisons = Vec::new();
                            for test in tests {
                                let (value, span) =
                                    read_value_case_expression(test).ok_or_else(|| {
                                        internal("a tagged value-switch case", switch.span)
                                    })?;
                                comparisons.push(node(
                                    SNode::Binary {
                                        op: BinaryOp::Eq,
                                        left: Box::new(named(&variable, switch.span)),
                                        right: Box::new(renamed(&value, scope)?),
                                        rounding: None,
                                    },
                                    span,
                                ));
                            }
                            condition = comparisons.into_iter().reduce(|left, right| {
                                node(
                                    SNode::Binary {
                                        op: BinaryOp::Or,
                                        left: Box::new(left),
                                        right: Box::new(right),
                                        rounding: None,
                                    },
                                    switch.span,
                                )
                            });
                        }
                        "body" => {
                            branch =
                                Some(self.seq(&Rc::new(bodies[index].clone()), 0, inner, scope)?)
                        }
                        _ => {
                            otherwise =
                                Some(node(
                                    SNode::If {
                                        cond: Box::new(condition.take().expect(
                                            "the shared cursor constructs its tests first",
                                        )),
                                        then: Box::new(
                                            branch.take().expect(
                                                "the shared cursor lowers its branch first",
                                            ),
                                        ),
                                        otherwise: Box::new(
                                            otherwise.take().expect(
                                                "the shared cursor reads its fallback first",
                                            ),
                                        ),
                                    },
                                    switch.span,
                                ));
                        }
                    }
                }
                _ => {
                    return Err(internal(
                        &format!("unknown value-switch instruction {instruction}"),
                        switch.span,
                    ));
                }
            }
        }
        unreachable!("the shared instruction cursor terminates with bind")
    }

    /// A statement that continues at several places, followed by more
    /// statements. When it only falls off its end and assigns one live
    /// variable, it is that variable's new value; otherwise it is a value of a
    /// data type whose alternatives carry the live variables it assigns to
    /// where it continues.
    fn join(
        &mut self,
        statement: &Stmt,
        exits: Exits,
        ctx: &Ctx,
        scope: &[Var],
        rest: &Fall,
    ) -> Result<SExpr> {
        let Some(place) = statement_span(statement) else {
            return Err(TranslationError::syntax("unknown statement empty", None));
        };
        let assigned = statement_uses(std::slice::from_ref(statement)).assigned;
        let none = HashSet::new();
        let live = union([
            ctx.live.as_ref(),
            if exits.brk > 0 {
                ctx.brk_live.as_ref()
            } else {
                &none
            },
            if exits.cont > 0 {
                ctx.cont_live.as_ref()
            } else {
                &none
            },
        ]);
        let carried: Vec<Var> = visible(scope)
            .into_iter()
            .filter(|entry| assigned.contains(&entry.source) && live.contains(&entry.source))
            .collect();
        if exits.brk == 0 && exits.cont == 0 && exits.ret == 0 && carried.len() == 1 {
            let ir = carried[0].ir.clone();
            let fall: Fall = {
                let ir = ir.clone();
                Rc::new(move |_, _| Ok(named(&ir, place)))
            };
            let value = self.compound(statement, &with_fall(ctx, fall), scope)?;
            let body = rest(self, scope)?;
            return Ok(let_node(ir, value, body, place));
        }
        let data = self.generated_name("join");
        let mut ctors = Vec::new();
        let mut rows = Vec::new();
        let names: Vec<String> = carried.iter().map(|entry| entry.ir.clone()).collect();
        let current = {
            let names = names.clone();
            move || {
                names
                    .iter()
                    .map(|name| named(name, place))
                    .collect::<Vec<_>>()
            }
        };
        let mut alternative = |kind: &str, variables: &[String], body: SExpr| {
            let ctor = format!("{data}_{kind}");
            ctors.push(SCtor {
                name: ctor.clone(),
                fields: variables.iter().map(|variable| untyped(variable)).collect(),
            });
            rows.push(ctor_row(&data, &ctor, variables, body, place));
            Rc::new(Alternative {
                ctor,
                variables: variables.to_vec(),
                place,
            })
        };
        let next = alternative("next", &names, rest(self, scope)?);
        let mut inner = ctx.clone();
        inner.fall = {
            let current = current.clone();
            Rc::new(move |_, _| Ok(next.make(current())))
        };
        if exits.ret > 0 {
            let result = format!("ml_result{}", self.count);
            let body = (ctx.ret)(self, named(&result, place))?;
            let returned = alternative("return", &[result], body);
            inner.ret = Rc::new(move |_, value| Ok(returned.make(vec![value])));
        }
        if exits.brk > 0 {
            let body = (ctx.brk)(self)?;
            let broken = alternative("break", &names, body);
            let current = current.clone();
            inner.brk = Rc::new(move |_| Ok(broken.make(current())));
        }
        if exits.cont > 0 {
            let body = (ctx.cont)(self)?;
            let continued = alternative("continue", &names, body);
            inner.cont = Rc::new(move |_| Ok(continued.make(current())));
        }
        self.generated.push(SItem::Data(SData {
            name: data,
            ctors,
            span: Some(place),
            generated: true,
            source_unit_clone_derivation: None,
        }));
        let scrutinee = self.compound(statement, &inner, scope)?;
        Ok(node(
            SNode::Match {
                scrutinees: vec![scrutinee],
                rows,
            },
            place,
        ))
    }
}

fn let_node(name: String, value: SExpr, body: SExpr, place: Span) -> SExpr {
    node(
        SNode::Let {
            name,
            ty: None,
            value: Box::new(value),
            body: Box::new(body),
        },
        place,
    )
}

fn literal_row(pattern: SPatternNode, test: Span, body: SExpr, place: Span) -> SRow {
    SRow {
        patterns: vec![SPattern {
            node: pattern,
            span: Some(test),
        }],
        body,
        span: Some(place),
    }
}
