//! Complete existing methods, mechanically relocated without behavior changes.
use super::{
    Alternative, Ctx, Fall, Give, HashSet, Jump, Lowering, ROOT, Rc, Result, SCtor, SData, SExpr,
    SFn, SItem, SNode, SParam, Site, Stmt, TranslationError, Uses, Var, ctor_row, flat, infinite,
    internal, let_node, list_exits, loop_parts, named, node, renamed, statement_uses, untyped,
    visible,
};

impl Lowering {
    /// A loop is a tail-recursive function of the variables it uses. It
    /// returns the live variables it assigns when it ends, or the function's
    /// result when it returns; `local` are the variables of a for loop's head.
    #[allow(clippy::too_many_lines)]
    pub(super) fn lower_loop(
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
                source_unit_clone_derivation: None,
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
            semantic_ret: None,
            name,
            params: params
                .iter()
                .map(|entry| SParam {
                    name: entry.ir.clone(),
                    optional: false,
                    default_value: None,
                    ty: None,
                    span: Some(place),
                    guard: None,
                    rocq_type: None,
                })
                .collect(),
            ret: None,
            source_signature: None,
            source_primitive_owners: None,
            source_empty_option: None,
            source_type_namespace: None,
            source_local_state: None,
            source_const_function: None,
            source_loop: None,
            body: fn_body,
            span: Some(place),
            generated: true,
            source_pure_unit: None,
        }));
        call_site(self)
    }
}
