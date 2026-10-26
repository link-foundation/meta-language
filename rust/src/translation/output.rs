//! Output and aborts as values, for the targets whose programs are pure.
//!
//! Rust and JavaScript print where the source prints and abort where it
//! aborts; in Lean and Rocq a function that prints or may abort, directly or
//! through a function it calls, takes the lines printed before it and returns
//! them, with the lines it prints added in front, paired with its value, or,
//! when it aborts, with the source's abort message instead of a value; main
//! prints the lines each of its steps adds and stops with the message of the
//! first abort. The rewriting is target-independent: it yields a portable-core
//! program without `print` and `abort` nodes, machine-integer arithmetic or
//! checked conversions, whose pairs are generated data types.
//!
//! Mirrors `js/src/translation/output.js`.

mod callables;
mod evaluation;

use std::borrow::Cow;
use std::collections::HashSet;

mod settle;

use super::Span;
use super::aborts::{cast_message, overflow_message, zero_divisor_message};
use super::diagnostics::{Result, unsupported};
use super::ir::{
    Case, Comparison, Ctor, DataDecl, Decl, Effect, Expr, Field, FnDecl, LitValue, Main, Node,
    Param, Pattern, Program, Prop, TheoremDecl,
};
use super::surface::BinaryOp;
use super::types::{BOOL, INT, NAT, STRING, Type, data};
use settle::{aborts, division, fixed_max, fixed_min, representation, settle_program, total};
/// A step of main is a `print` effect itself; what threads is a value that prints or may abort.
fn effect_operand_any(effect: &Effect, test: &dyn Fn(&Expr) -> bool) -> bool {
    match effect {
        Effect::Let { value, .. } => test(value),
        Effect::Assert { prop, .. } => prop_any(prop, test),
        Effect::Print { expr, .. } | Effect::Output { expr, .. } => test(expr),
        Effect::Unwrap { pair, .. } => test(pair),
    }
}

/// The checked program with its output and aborts threaded through, or the
/// program itself when nothing below main prints or aborts.
///
/// # Errors
/// When an operand that prints or aborts sits in a compound assertion, or a
/// theorem states a fact of a computation that may abort.
pub fn thread_output(source: &Program) -> Result<Cow<'_, Program>> {
    thread_output_for_target(source, "Lean")
}

/// Thread output using the selected pure target's shared callable boundary.
///
/// # Errors
/// On an effectful callable type or an existing output sequencing obligation.
pub fn thread_output_for_target<'a>(source: &'a Program, target: &str) -> Result<Cow<'a, Program>> {
    let program = settle_program(source);
    let callables::CallableOutputAnalysis {
        effectful,
        count,
        span,
    } = callables::analyze(&program);
    if program.declarations.iter().any(|entry| matches!(entry,
        Decl::Fn(function) if function.source_signature.is_some() && effectful.contains(&function.full_name))) {
        return Err(unsupported("owned Rust source signature transformation",
            "effect threading changes the captured source header and requires a distinct signature transformation proof", None));
    }
    let refusal =
        super::callable_emission_rules::read_callable_output_target_refusal(target, count);
    if !refusal.is_empty() {
        return Err(unsupported("effectful function value", &refusal, span));
    }
    let main_effects = program.main.as_ref().is_some_and(|main| {
        main.effects
            .iter()
            .any(|effect| effect_operand_any(effect, &|e| effects(e, &effectful)))
    });
    if effectful.is_empty() && !main_effects {
        return Ok(Cow::Owned(program));
    }
    let can_abort = program.declarations.iter().any(|entry| match entry {
        Decl::Fn(function) => aborting(&function.body),
        Decl::Theorem(theorem) => prop_any(&theorem.prop, &aborting),
        Decl::Data(_) => false,
    }) || program.main.as_ref().is_some_and(|main| {
        main.effects
            .iter()
            .any(|effect| effect_operand_any(effect, &aborting))
    });
    let names = serde_json::to_value(&program).expect("checked output source retains actual names");
    let private_prefix = super::source_identifiers::read_source_identifier_supply(
        super::source_identifiers::collect_source_identifier_texts(&names),
    );
    Threader {
        source: &program,
        effectful,
        can_abort,
        source_exceptions: can_abort && program.source_language == super::Language::JavaScript,
        private_prefix,
        pairs: Vec::new(),
        count: 0,
    }
    .program()
    .map(Cow::Owned)
}

fn effects(expr: &Expr, effectful: &HashSet<String>) -> bool {
    if !super::callable_emission_rules::accept_callable_creation_effect_traversal(expr.kind()) {
        return false;
    }
    match &expr.node {
        Node::Print { .. } => true,
        Node::Call { func, .. } if effectful.contains(func) => true,
        _ if aborts(expr) => true,
        _ => expr
            .children()
            .into_iter()
            .any(|child| effects(child, effectful)),
    }
}

fn aborting(expr: &Expr) -> bool {
    super::callable_emission_rules::accept_callable_creation_effect_traversal(expr.kind())
        && (aborts(expr) || expr.children().into_iter().any(aborting))
}

fn prop_any(prop: &Prop, test: &dyn Fn(&Expr) -> bool) -> bool {
    match prop {
        Prop::Forall { body, .. } => prop_any(body, test),
        Prop::And { left, right } | Prop::Or { left, right } | Prop::Implies { left, right } => {
            prop_any(left, test) || prop_any(right, test)
        }
        Prop::Not { arg } => prop_any(arg, test),
        Prop::Bool { expr } => test(expr),
        other => other
            .comparison()
            .is_some_and(|(_, comparison)| test(&comparison.left) || test(&comparison.right)),
    }
}

fn literal(ty: Type, value: &impl ToString) -> Expr {
    Expr::lit(ty, LitValue::Text(value.to_string()))
}

fn compare(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    let domain = representation(&left.ty);
    Expr::new(
        Node::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
            domain: Some(domain),
            semantics: None,
            rounding: None,
            by_zero: None,
        },
        BOOL,
    )
}

fn logical(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    Expr::new(
        Node::Binary {
            op,
            left: Box::new(left),
            right: Box::new(right),
            domain: None,
            semantics: None,
            rounding: None,
            by_zero: None,
        },
        BOOL,
    )
}

fn let_in(name: String, value: Expr, body: Expr) -> Expr {
    let ty = body.ty.clone();
    Expr::new(
        Node::Let {
            name,
            value: Box::new(value),
            body: Box::new(body),
        },
        ty,
    )
}

const fn out_nil() -> Expr {
    Expr::new(Node::OutNil, Type::Output)
}

/// A pair taken apart by a match: `match scrutinee with mk out value => …`.
struct Frame {
    scrutinee: Expr,
    data: String,
    out: String,
    value: String,
}

struct Threader<'p> {
    source: &'p Program,
    effectful: HashSet<String>,
    can_abort: bool,
    source_exceptions: bool,
    private_prefix: String,
    /// The generated pairs with the keys of their value types, in order.
    pairs: Vec<(String, DataDecl)>,
    count: usize,
}

impl Threader<'_> {
    fn program(mut self) -> Result<Program> {
        let mut declarations = Vec::new();
        for entry in &self.source.declarations {
            let entry = match entry {
                Decl::Fn(function) if self.effectful.contains(&function.full_name) => {
                    let out = format!("{}out", self.private_prefix);
                    let mut params = function.params.clone();
                    params.push(Param {
                        name: out.to_owned(),
                        optional: false,
                        ty: Type::Output,
                        guard: None,
                        default_value: None,
                    });
                    let ret = self.pair_type(&function.ret);
                    let body = self.io(&function.body, Expr::var(out, Type::Output))?;
                    Decl::Fn(FnDecl {
                        params,
                        ret,
                        body,
                        ..function.clone()
                    })
                }
                Decl::Theorem(theorem) if self.prop_effects(&theorem.prop) => {
                    Decl::Theorem(TheoremDecl {
                        prop: self.pure_prop(&theorem.prop)?,
                        ..theorem.clone()
                    })
                }
                other => other.clone(),
            };
            declarations.push(entry);
        }
        let main = match &self.source.main {
            Some(main) => {
                let mut effects = Vec::new();
                for effect in &main.effects {
                    effects.extend(self.effect(effect)?);
                }
                Some(Main {
                    effects,
                    ..main.clone()
                })
            }
            None => None,
        };
        // The pairs come first; a pair of a data type follows it in the emitted order.
        let mut all: Vec<Decl> = self
            .pairs
            .into_iter()
            .map(|(_, entry)| Decl::Data(entry))
            .collect();
        all.extend(declarations);
        Ok(Program {
            declarations: all,
            main,
            output_threaded: true,
            aborts_threaded: self.can_abort,
            ..self.source.clone()
        })
    }

    fn effects(&self, expr: &Expr) -> bool {
        effects(expr, &self.effectful)
    }

    fn prop_effects(&self, prop: &Prop) -> bool {
        prop_any(prop, &|e| effects(e, &self.effectful))
    }

    fn fresh(&mut self, prefix: &str) -> String {
        self.count += 1;
        super::source_identifiers::render_source_generated_identifier(
            &self.private_prefix,
            prefix,
            self.count,
        )
    }

    /// The generated data type pairing the lines printed so far with a value
    /// of `source`, or, when the program may abort, with the message it
    /// aborted with. A machine integer pairs as its representation, which its
    /// value is.
    fn pair(&mut self, source: &Type) -> String {
        let ty = representation(source);
        let key = ty.key();
        if let Some((_, entry)) = self.pairs.iter().find(|(candidate, _)| *candidate == key) {
            return entry.full_name.clone();
        }
        let name = format!("{}io{}", self.private_prefix, self.pairs.len() + 1);
        let mut ctors = vec![Ctor {
            name: format!("{name}_mk"),
            fields: vec![
                Field {
                    name: "output".to_owned(),
                    ty: Type::Output,
                },
                Field {
                    name: "value".to_owned(),
                    ty,
                },
            ],
        }];
        if self.can_abort {
            ctors.push(Ctor {
                name: format!("{name}_abort"),
                fields: vec![
                    Field {
                        name: "output".to_owned(),
                        ty: Type::Output,
                    },
                    Field {
                        name: "message".to_owned(),
                        ty: STRING,
                    },
                ],
            });
        }
        if self.source_exceptions {
            ctors[1].fields.extend([
                Field {
                    name: "className".to_owned(),
                    ty: STRING,
                },
                Field {
                    name: "messagePresent".to_owned(),
                    ty: BOOL,
                },
            ]);
        }
        self.pairs.push((
            key,
            DataDecl {
                name: name.clone(),
                ctors,
                span: None,
                full_name: name.clone(),
                module_path: Vec::new(),
                generated: true,
                output: true,
                source_unit_clone_derivation: None,
                source_field_signatures: None,
            },
        ));
        name
    }

    fn pair_type(&mut self, ty: &Type) -> Type {
        data(self.pair(ty))
    }

    /// The value paired with the lines `out`; `ty` is the value's source type
    /// when its node computes on the representation.
    fn make_as(&mut self, out: Expr, value: Expr, ty: &Type) -> Expr {
        let name = self.pair(ty);
        Expr::new(
            Node::Ctor {
                ctor: format!("{name}_mk"),
                data: name.clone(),
                args: vec![out, value],
                frozen: false,
            },
            data(name),
        )
    }

    fn make(&mut self, out: Expr, value: Expr) -> Expr {
        let ty = value.ty.clone();
        self.make_as(out, value, &ty)
    }

    /// The abort with `message` after the lines `out`, as a pair of a `ty` value.
    fn fail(&mut self, out: Expr, message: Expr, ty: &Type) -> Expr {
        self.fail_exception(
            out,
            message,
            ty,
            literal(STRING, "Error"),
            Expr::lit(BOOL, LitValue::Bool(true)),
        )
    }

    fn fail_exception(
        &mut self,
        out: Expr,
        message: Expr,
        ty: &Type,
        class_name: Expr,
        present: Expr,
    ) -> Expr {
        let name = self.pair(ty);
        let mut args = vec![out, message];
        if self.source_exceptions {
            args.extend([class_name, present]);
        }
        Expr::new(
            Node::Ctor {
                ctor: format!("{name}_abort"),
                data: name.clone(),
                args,
                frozen: false,
            },
            data(name),
        )
    }

    fn reraise(
        &self,
        out: Expr,
        message: Expr,
        pair_type: &Type,
        class_name: Option<Expr>,
        present: Option<Expr>,
    ) -> Expr {
        let entry = match pair_type {
            Type::Data { name } => self
                .pairs
                .iter()
                .find(|(_, entry)| entry.full_name == *name)
                .map(|(_, entry)| entry),
            _ => None,
        };
        let entry = entry.expect("an abort passes on only as a pair");
        let mut args = vec![out, message];
        if self.source_exceptions {
            args.extend([
                class_name.expect("checked exception class"),
                present.expect("checked exception presence"),
            ]);
        }
        Expr::new(
            Node::Ctor {
                data: entry.full_name.clone(),
                ctor: format!("{}_abort", entry.full_name),
                args,
                frozen: false,
            },
            pair_type.clone(),
        )
    }

    /// Takes apart `pair`, a pair of lines and a value of `ty`.
    fn open(&mut self, scrutinee: Expr, ty: &Type) -> Frame {
        let data = self.pair(ty);
        let out = self.fresh("o");
        let value = self.fresh("v");
        Frame {
            scrutinee,
            data,
            out,
            value,
        }
    }

    /// `match pair with mk o v => inner | abort o m => failure(o, m)`; an
    /// abort passes on as the abort of the pair `inner` is unless `failure`
    /// says otherwise.
    fn close_one(
        &mut self,
        frame: Frame,
        inner: Expr,
        failure: Option<&dyn Fn(Expr, Expr) -> Expr>,
    ) -> Expr {
        let ty = inner.ty.clone();
        let mut cases = vec![Case {
            pattern: Pattern::Ctor {
                data: frame.data.clone(),
                ctor: format!("{}_mk", frame.data),
                binds: vec![Some(frame.out), Some(frame.value)],
            },
            body: inner,
        }];
        if self.can_abort {
            let lines = self.fresh("o");
            let message = self.fresh("m");
            let out = Expr::var(lines.clone(), Type::Output);
            let text = Expr::var(message.clone(), STRING);
            let class_name = self
                .source_exceptions
                .then(|| self.fresh("exception_class"));
            let present = self
                .source_exceptions
                .then(|| self.fresh("exception_present"));
            let mut binds = vec![Some(lines), Some(message)];
            let class_value = class_name
                .as_ref()
                .map(|name| Expr::var(name.clone(), STRING));
            let present_value = present.as_ref().map(|name| Expr::var(name.clone(), BOOL));
            if self.source_exceptions {
                binds.extend([class_name, present]);
            }
            let handled = match failure {
                Some(failure) => failure(out, text),
                None => self.reraise(out, text, &ty, class_value, present_value),
            };
            cases.push(Case {
                pattern: Pattern::Ctor {
                    data: frame.data.clone(),
                    ctor: format!("{}_abort", frame.data),
                    binds,
                },
                body: handled,
            });
        }
        Expr::new(
            Node::Match {
                scrutinee: Box::new(frame.scrutinee),
                cases,
            },
            ty,
        )
    }

    /// `inner` under the matches of `frames`, innermost last.
    fn close(&mut self, frames: Vec<Frame>, inner: Expr) -> Expr {
        frames
            .into_iter()
            .rev()
            .fold(inner, |inner, frame| self.close_one(frame, inner, None))
    }

    /// The operands run in order after the lines `out`: the matches taking
    /// apart the pairs of those that print or may abort, the lines after them
    /// all, and their values; a value that neither prints nor aborts is used
    /// as it is.
    fn bind_all(&mut self, list: &[&Expr], mut out: Expr) -> Result<(Vec<Frame>, Expr, Vec<Expr>)> {
        let mut frames = Vec::new();
        let mut values = Vec::new();
        for &operand in list {
            if self.effects(operand) {
                let pair = self.io(operand, out)?;
                let frame = self.open(pair, &operand.ty);
                out = Expr::var(frame.out.clone(), Type::Output);
                values.push(Expr::var(frame.value.clone(), operand.ty.clone()));
                frames.push(frame);
            } else {
                values.push(operand.clone());
            }
        }
        Ok((frames, out, values))
    }

    fn bind(&mut self, operand: &Expr, out: Expr) -> Result<(Vec<Frame>, Expr, Expr)> {
        let (frames, out, mut values) = self.bind_all(&[operand], out)?;
        let value = values.pop().expect("one operand");
        Ok((frames, out, value))
    }

    /// A node whose operands are values, after the lines `out`: when it may
    /// abort, the test of the source's abort condition, its abort message, and
    /// otherwise its value computed on the representation, where it cannot abort.
    fn complete(&mut self, out: Expr, e: Expr) -> Expr {
        if !aborts(&e) {
            return self.make(out, e);
        }
        if let Node::Math { op, args } = &e.node {
            if super::source_exception_rules::accept_linked_java_script_source_exception("math", op)
            {
                let class =
                    super::source_exception_rules::read_java_script_source_exception_class(op);
                return self.fail_exception(
                    out,
                    args[0].clone(),
                    &e.ty,
                    literal(STRING, &class),
                    args[1].clone(),
                );
            }
            let message = super::string_number_rules::read_java_script_number_conversion_failure(
                op,
                args[0].ty.kind(),
            );
            let body = self.fail_exception(
                out,
                literal(STRING, &message),
                &e.ty,
                literal(STRING, "TypeError"),
                Expr::lit(BOOL, LitValue::Bool(true)),
            );
            let ty = body.ty.clone();
            let name = self.fresh("numeric_argument");
            return Expr::new(
                Node::Let {
                    name,
                    value: Box::new(args[0].clone()),
                    body: Box::new(body),
                },
                ty,
            );
        }
        let mut operands: Vec<(String, Expr)> = Vec::new();
        let mut name = |this: &mut Self, value: Expr| -> Expr {
            if matches!(value.node, Node::Var { .. } | Node::Lit { .. }) {
                return value;
            }
            let bound = this.fresh("a");
            let ty = value.ty.clone();
            operands.push((bound.clone(), value));
            Expr::var(bound, ty)
        };
        let source = e.ty.clone();
        let guard = |this: &mut Self, cond: Expr, message: &str, rest: Expr| -> Expr {
            let ty = rest.ty.clone();
            let then = this.fail(out.clone(), literal(STRING, &message), &source);
            Expr::new(
                Node::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    otherwise: Box::new(rest),
                },
                ty,
            )
        };
        let Expr { node, ty, span } = e;
        let result = match node {
            Node::Cast {
                arg,
                from,
                to,
                flavor,
                message,
                order,
            } => {
                let arg = name(self, *arg);
                let zero = literal(representation(&arg.ty), &0);
                let cond = compare(BinaryOp::Lt, arg.clone(), zero);
                let text = cast_message(message.as_deref()).to_owned();
                let node = total(Expr {
                    node: Node::Cast {
                        arg: Box::new(arg),
                        from,
                        to,
                        flavor,
                        message,
                        order,
                    },
                    ty: ty.clone(),
                    span,
                });
                let rest = self.make_as(out.clone(), node, &ty);
                guard(self, cond, &text, rest)
            }
            Node::Unary { op, arg, semantics } => {
                let arg = name(self, *arg);
                let cond = compare(BinaryOp::Eq, arg.clone(), literal(INT, &fixed_min(&ty)));
                let node = total(Expr {
                    node: Node::Unary {
                        op,
                        arg: Box::new(arg),
                        semantics,
                    },
                    ty: ty.clone(),
                    span,
                });
                let rest = self.make_as(out.clone(), node, &ty);
                guard(self, cond, overflow_message("neg"), rest)
            }
            Node::Binary {
                op,
                left,
                right,
                domain,
                semantics,
                rounding,
                by_zero,
            } => {
                let left = name(self, *left);
                let right = name(self, *right);
                let node = total(Expr {
                    node: Node::Binary {
                        op,
                        left: Box::new(left.clone()),
                        right: Box::new(right.clone()),
                        domain,
                        semantics,
                        rounding,
                        by_zero,
                    },
                    ty: ty.clone(),
                    span,
                });
                let rep = representation(&ty);
                if division(op) {
                    let mut rest = self.make_as(out.clone(), node, &ty);
                    if matches!(ty, Type::Fixed { signed: true, .. }) {
                        let overflow = logical(
                            BinaryOp::And,
                            compare(BinaryOp::Eq, left, literal(rep.clone(), &fixed_min(&ty))),
                            compare(BinaryOp::Eq, right.clone(), literal(rep.clone(), &-1)),
                        );
                        rest = guard(self, overflow, overflow_message(op.name()), rest);
                    }
                    let zero = compare(BinaryOp::Eq, right, literal(rep, &0));
                    guard(self, zero, zero_divisor_message(op.name(), &ty), rest)
                } else if op == BinaryOp::Sub && rep == NAT {
                    let cond = compare(BinaryOp::Lt, left, right);
                    let rest = self.make_as(out.clone(), node, &ty);
                    guard(self, cond, overflow_message("sub"), rest)
                } else {
                    // The exact result, range-checked before it is the machine integer's value.
                    let wide = self.fresh("w");
                    let value = Expr::var(wide.clone(), rep.clone());
                    let above = compare(
                        BinaryOp::Gt,
                        value.clone(),
                        literal(rep.clone(), &fixed_max(&ty)),
                    );
                    let outside = if rep == NAT {
                        above
                    } else {
                        let below =
                            compare(BinaryOp::Lt, value.clone(), literal(rep, &fixed_min(&ty)));
                        logical(BinaryOp::Or, below, above)
                    };
                    let rest = self.make_as(out.clone(), value, &ty);
                    let body = guard(self, outside, overflow_message(op.name()), rest);
                    let pair = self.pair_type(&ty);
                    Expr::new(
                        Node::Let {
                            name: wide,
                            value: Box::new(node),
                            body: Box::new(body),
                        },
                        pair,
                    )
                }
            }
            Node::Abort { .. } => unreachable!("an abort runs as an effect"),
            other => unreachable!("{} does not abort", other.kind()),
        };
        operands
            .into_iter()
            .rev()
            .fold(result, |body, (bound, value)| let_in(bound, value, body))
    }

    /// The value of `e` alone, for a theorem, which states facts of values.
    fn value(&mut self, e: &Expr, span: Option<Span>) -> Result<Expr> {
        if self.can_abort {
            return Err(unsupported(
                "a theorem over a computation that may abort",
                "a theorem states a fact of a value, which an abort does not have",
                span,
            ));
        }
        let pair = self.io(e, out_nil())?;
        let frame = self.open(pair, &e.ty);
        let value = Expr::var(frame.value.clone(), e.ty.clone());
        Ok(self.close(vec![frame], value))
    }

    fn pure_expr(&mut self, e: &Expr) -> Result<Expr> {
        if self.effects(e) {
            self.value(e, None)
        } else {
            Ok(e.clone())
        }
    }

    fn pure_prop(&mut self, prop: &Prop) -> Result<Prop> {
        Ok(match prop {
            Prop::Forall { binders, body } => Prop::Forall {
                binders: binders.clone(),
                body: Box::new(self.pure_prop(body)?),
            },
            Prop::And { left, right } => Prop::And {
                left: Box::new(self.pure_prop(left)?),
                right: Box::new(self.pure_prop(right)?),
            },
            Prop::Or { left, right } => Prop::Or {
                left: Box::new(self.pure_prop(left)?),
                right: Box::new(self.pure_prop(right)?),
            },
            Prop::Implies { left, right } => Prop::Implies {
                left: Box::new(self.pure_prop(left)?),
                right: Box::new(self.pure_prop(right)?),
            },
            Prop::Not { arg } => Prop::Not {
                arg: Box::new(self.pure_prop(arg)?),
            },
            Prop::Bool { expr } => Prop::Bool {
                expr: self.pure_expr(expr)?,
            },
            other => {
                let (op, comparison) = other.comparison().expect("a comparison");
                let left = self.pure_expr(&comparison.left)?;
                let right = self.pure_expr(&comparison.right)?;
                Prop::compare(
                    op,
                    Comparison {
                        left,
                        right,
                        ..comparison.clone()
                    },
                )
            }
        })
    }

    /// A main step whose value prints or may abort: the pair of its value and
    /// the lines it prints, a step printing those lines, a step that stops
    /// main with the message of an abort, and the step itself on the value.
    fn effect(&mut self, effect: &Effect) -> Result<Vec<Effect>> {
        Ok(match effect {
            Effect::Let {
                name,
                value,
                constant,
                span,
                source_pure_unit_call,
            } if self.effects(value) => {
                let (mut steps, value) = self.run(value)?;
                steps.push(Effect::Let {
                    name: name.clone(),
                    value,
                    constant: *constant,
                    span: *span,
                    source_pure_unit_call: source_pure_unit_call.clone(),
                });
                steps
            }
            Effect::Print { expr, span } if self.effects(expr) => {
                let (mut steps, expr) = self.run(expr)?;
                steps.push(Effect::Print { expr, span: *span });
                steps
            }
            Effect::Assert { prop, span } if self.prop_effects(prop) => {
                let mut steps = Vec::new();
                let prop = self.hoist(prop, &mut steps, *span)?;
                steps.push(Effect::Assert { prop, span: *span });
                steps
            }
            other => vec![other.clone()],
        })
    }

    fn run(&mut self, e: &Expr) -> Result<(Vec<Effect>, Expr)> {
        let name = self.fresh("run");
        let pair = Expr::var(name.clone(), self.pair_type(&e.ty));
        let value = self.io(e, out_nil())?;
        let lines = self.open(pair.clone(), &e.ty);
        let lines_var = Expr::var(lines.out.clone(), Type::Output);
        let lines = self.close_one(lines, lines_var, Some(&|out, _| out));
        let mut steps = vec![
            Effect::Let {
                name,
                value,
                constant: false,
                span: e.span,
                source_pure_unit_call: None,
            },
            Effect::Output {
                expr: lines,
                span: e.span,
            },
        ];
        if !self.can_abort {
            let result = self.open(pair, &e.ty);
            let result_var = Expr::var(result.value.clone(), e.ty.clone());
            let result = self.close(vec![result], result_var);
            return Ok((steps, result));
        }
        // `unwrap` binds the value of a pair, or stops main with its abort message.
        let value = self.fresh("val");
        let data = self.pair(&e.ty);
        steps.push(Effect::Unwrap {
            name: value.clone(),
            pair,
            ctors: vec![format!("{data}_mk"), format!("{data}_abort")],
            data,
            ty: e.ty.clone(),
            span: e.span,
        });
        Ok((steps, Expr::var(value, e.ty.clone())))
    }

    fn lift(&mut self, e: &Expr, steps: &mut Vec<Effect>) -> Result<Expr> {
        if !self.effects(e) {
            return Ok(e.clone());
        }
        let (run, value) = self.run(e)?;
        steps.extend(run);
        Ok(value)
    }

    /// An assertion's operands that print or may abort run before it, in order, as steps of main.
    fn hoist(&mut self, prop: &Prop, steps: &mut Vec<Effect>, span: Option<Span>) -> Result<Prop> {
        Ok(match prop {
            Prop::Bool { expr } => Prop::Bool {
                expr: self.lift(expr, steps)?,
            },
            Prop::Not { arg } => Prop::Not {
                arg: Box::new(self.hoist(arg, steps, span)?),
            },
            // The left operand always runs, first; the right one only when the left does not decide the assertion.
            Prop::And { left, right } if !self.prop_effects(right) => Prop::And {
                left: Box::new(self.hoist(left, steps, span)?),
                right: right.clone(),
            },
            Prop::Or { left, right } if !self.prop_effects(right) => Prop::Or {
                left: Box::new(self.hoist(left, steps, span)?),
                right: right.clone(),
            },
            Prop::And { .. } | Prop::Or { .. } => {
                return Err(unsupported(
                    "output or an abort in a compound assertion",
                    "an operand that prints or aborts after the first runs only when the assertion evaluates it; assert on a value computed before",
                    span,
                ));
            }
            Prop::Implies { .. } | Prop::Forall { .. } => {
                return Err(unsupported(
                    "output or an abort in a compound assertion",
                    "an operand that prints or aborts runs only when the assertion evaluates it; assert on a value computed before",
                    span,
                ));
            }
            other => {
                let (op, comparison) = other.comparison().expect("a comparison");
                let left = self.lift(&comparison.left, steps)?;
                let right = self.lift(&comparison.right, steps)?;
                Prop::compare(
                    op,
                    Comparison {
                        left,
                        right,
                        ..comparison.clone()
                    },
                )
            }
        })
    }
}
