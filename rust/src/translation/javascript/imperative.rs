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

use serde_json::Value;

use super::flow::{
    Exits, Uses, case_bodies, exhaustive, exits, flat, infinite, json, list_exits, local_name,
    loop_parts, statement_uses, walked,
};
use super::lowering::{
    TagSwitch, default_rows, lower_tag_if, statement_span, switch_tags, tag_condition, tag_row,
};
use super::{
    CaseTest, JavaScriptParser, ROOT, Result, SCtor, SData, SExpr, SField, SFn, SItem, SNode,
    SParam, SPattern, SPatternNode, SRow, Span, Stmt, Switch, TranslationError, Type, node,
    unsupported, wild,
};

mod top_level;
pub(super) use self::top_level::lower_top_level;

/// A variable in scope: its source name and the name it has in the translation.
#[derive(Clone)]
struct Var {
    source: String,
    ir: String,
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
    fn visit(value: &mut Value, scope: &[Var]) {
        match value {
            Value::Array(items) => items.iter_mut().for_each(|item| visit(item, scope)),
            Value::Object(map) => {
                if let Some(name) = local_name(map) {
                    let ir =
                        lookup(scope, name).map_or_else(|| name.to_owned(), |var| var.ir.clone());
                    map.insert("path".to_owned(), Value::Array(vec![Value::String(ir)]));
                    return;
                }
                for (key, value) in map.iter_mut() {
                    if walked(key) {
                        visit(value, scope);
                    }
                }
            }
            _ => {}
        }
    }
    let mut value = json(expr);
    visit(&mut value, scope);
    serde_json::from_value(value).map_err(|error| {
        TranslationError::syntax(format!("internal: renamed expression: {error}"), expr.span)
    })
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
    let mut lowering = Lowering {
        function: name.to_owned(),
        renames: 0,
        generated: std::mem::take(&mut parser.generated),
        count: parser.generated_count,
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
    let scope: Vec<Var> = params
        .iter()
        .map(|param| Var {
            source: param.name.clone(),
            ir: param.name.clone(),
        })
        .collect();
    let body = lowering.seq(&flat(statements), 0, &ctx, &scope);
    parser.generated = lowering.generated;
    parser.generated_count = lowering.count;
    body
}

struct Lowering {
    function: String,
    renames: usize,
    generated: Vec<SItem>,
    count: usize,
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
                return Ok(let_node(ir, value, body, *span));
            }
            Stmt::Assign { name, value, span } => {
                let ir = lookup(scope, name).map_or_else(|| name.clone(), |var| var.ir.clone());
                let value = renamed(value, scope)?;
                let body = rest(self, scope)?;
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
                let name = self.generated_name("ignored");
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

    /// A loop is a tail-recursive function of the variables it uses. It
    /// returns the live variables it assigns when it ends, or the function's
    /// result when it returns; `local` are the variables of a for loop's head.
    #[allow(clippy::too_many_lines)]
    fn lower_loop(
        &mut self,
        statement: &Stmt,
        ctx: &Ctx,
        scope: &[Var],
        local: &HashSet<String>,
    ) -> Result<SExpr> {
        let Some(parts) = loop_parts(statement) else {
            return Err(TranslationError::syntax("a loop that is not a loop", None));
        };
        let place = parts.place;
        let whole = match statement {
            Stmt::For { .. } => Stmt::While {
                cond: parts
                    .cond
                    .cloned()
                    .unwrap_or_else(|| SExpr::new(SNode::Bool { value: true }, None)),
                body: parts.body.iter().chain(parts.update).cloned().collect(),
                span: place,
            },
            other => other.clone(),
        };
        let Uses { free, assigned } = statement_uses(&[whole]);
        let params: Vec<Var> = visible(scope)
            .into_iter()
            .filter(|entry| free.contains(&entry.source))
            .collect();
        let results: Vec<Var> = params
            .iter()
            .filter(|entry| {
                assigned.contains(&entry.source)
                    && !local.contains(&entry.source)
                    && ctx.live.contains(&entry.source)
            })
            .cloned()
            .collect();
        let body_exits = list_exits(&flat(parts.body));
        let returns = body_exits.ret > 0;
        let ends = !infinite(parts.cond) || body_exits.brk > 0;
        let name = self.generated_name("loop");
        let call = {
            let name = name.clone();
            let args: Vec<String> = params.iter().map(|entry| entry.ir.clone()).collect();
            Rc::new(move || {
                node(
                    SNode::App {
                        func: Box::new(node(
                            SNode::Name {
                                path: vec![ROOT.to_owned(), name.clone()],
                            },
                            place,
                        )),
                        args: args.iter().map(|arg| named(arg, place)).collect(),
                    },
                    place,
                )
            })
        };
        let loop_scope = params.clone();
        let mut give: Give =
            Rc::new(move |_, _| Err(internal("a loop without a return returned", place)));
        let (finish, call_site): (Jump, Site) = if !returns && ends && results.len() == 1 {
            let result = results[0].ir.clone();
            let finish: Jump = {
                let result = result.clone();
                Rc::new(move |_| Ok(named(&result, place)))
            };
            let call = Rc::clone(&call);
            let ctx = ctx.clone();
            let scope = scope.to_vec();
            let site: Site = Box::new(move |lowering| {
                let body = (ctx.fall)(lowering, &scope)?;
                Ok(let_node(result, call(), body, place))
            });
            (finish, site)
        } else if !ends {
            // The loop only ends by returning from the function, or never.
            give = Rc::new(|_, value| Ok(value));
            let call = Rc::clone(&call);
            let ctx = ctx.clone();
            let site: Site = Box::new(move |lowering| {
                if returns {
                    (ctx.ret)(lowering, call())
                } else {
                    Ok(call())
                }
            });
            (
                Rc::new(move |_| Err(internal("a loop without an end ended", place))),
                site,
            )
        } else {
            let data = format!("{name}_result");
            let mut ctors = Vec::new();
            let mut rows = Vec::new();
            let done = format!("{name}_done");
            let variables: Vec<String> = results.iter().map(|entry| entry.ir.clone()).collect();
            ctors.push(SCtor {
                name: done.clone(),
                fields: variables.iter().map(|variable| untyped(variable)).collect(),
            });
            let alternative = Alternative {
                ctor: done.clone(),
                variables: variables.clone(),
                place,
            };
            let finish: Jump = Rc::new(move |_| {
                let values = alternative
                    .variables
                    .iter()
                    .map(|variable| named(variable, place))
                    .collect();
                Ok(alternative.make(values))
            });
            let body = (ctx.fall)(self, scope)?;
            rows.push(ctor_row(&data, &done, &variables, body, place));
            if returns {
                let value = format!("ml_result{}", self.count);
                let returned = format!("{name}_return");
                ctors.push(SCtor {
                    name: returned.clone(),
                    fields: vec![untyped(&value)],
                });
                let alternative = Alternative {
                    ctor: returned.clone(),
                    variables: vec![value.clone()],
                    place,
                };
                give = Rc::new(move |_, result| Ok(alternative.make(vec![result])));
                let body = (ctx.ret)(self, named(&value, place))?;
                rows.push(ctor_row(&data, &returned, &[value], body, place));
            }
            self.generated.push(SItem::Data(SData {
                name: data,
                ctors,
                span: Some(place),
                generated: true,
            }));
            let call = Rc::clone(&call);
            let site: Site = Box::new(move |_| {
                Ok(node(
                    SNode::Match {
                        scrutinees: vec![call()],
                        rows,
                    },
                    place,
                ))
            });
            (finish, site)
        };
        // Every variable the loop uses is live at its next iteration, the live ones it assigns after it.
        let iterating: Rc<HashSet<String>> =
            Rc::new(params.iter().map(|entry| entry.source.clone()).collect());
        let ending: Rc<HashSet<String>> =
            Rc::new(results.iter().map(|entry| entry.source.clone()).collect());
        let lives = |fall: Fall| Ctx {
            fall,
            ret: Rc::clone(&give),
            brk: Rc::clone(&finish),
            cont: Rc::clone(&finish),
            live: Rc::clone(&iterating),
            brk_live: Rc::clone(&ending),
            cont_live: Rc::clone(&iterating),
        };
        let cond = parts.cond.filter(|_| !infinite(parts.cond));
        // The next iteration: a for loop's update, then the call with the variables' current values.
        let next = Rc::clone(&call);
        let again: Fall = if parts.update.is_empty() {
            Rc::new(move |_, _| Ok(next()))
        } else {
            let call = next;
            let update = Rc::new(parts.update.to_vec());
            let mut ctx = lives(Rc::new(move |_, _| Ok(call())));
            ctx.cont = Rc::clone(&finish);
            Rc::new(move |lowering, inner| lowering.seq(&update, 0, &ctx, inner))
        };
        let test = |lowering: &mut Self, then: SExpr| -> Result<SExpr> {
            let Some(cond) = cond else {
                return Ok(then);
            };
            let cond = renamed(cond, &loop_scope)?;
            let otherwise = finish(lowering)?;
            Ok(node(
                SNode::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                },
                place,
            ))
        };
        let fn_body = if parts.do_while {
            let next: Fall = {
                let call = Rc::clone(&call);
                let cond = cond.cloned();
                let loop_scope = loop_scope.clone();
                let finish = Rc::clone(&finish);
                Rc::new(move |lowering, _| {
                    let then = call();
                    let Some(cond) = &cond else {
                        return Ok(then);
                    };
                    let cond = renamed(cond, &loop_scope)?;
                    let otherwise = finish(lowering)?;
                    Ok(node(
                        SNode::If {
                            cond: Box::new(cond),
                            then: Box::new(then),
                            otherwise: Box::new(otherwise),
                        },
                        place,
                    ))
                })
            };
            let mut body_ctx = lives(Rc::clone(&next));
            body_ctx.cont = Rc::new(move |lowering| next(lowering, &[]));
            self.seq(&flat(parts.body), 0, &body_ctx, &loop_scope)?
        } else {
            let next: Fall = {
                let loop_scope = loop_scope.clone();
                Rc::new(move |lowering, _| again(lowering, &loop_scope))
            };
            let mut body_ctx = lives(Rc::clone(&next));
            body_ctx.cont = Rc::new(move |lowering| next(lowering, &[]));
            let body = self.seq(&flat(parts.body), 0, &body_ctx, &loop_scope)?;
            test(self, body)?
        };
        self.generated.push(SItem::Fn(SFn {
            name,
            params: params
                .iter()
                .map(|entry| SParam {
                    name: entry.ir.clone(),
                    ty: None,
                    span: Some(place),
                    guard: None,
                    rocq_type: None,
                })
                .collect(),
            ret: None,
            body: fn_body,
            span: Some(place),
            generated: true,
        }));
        call_site(self)
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
