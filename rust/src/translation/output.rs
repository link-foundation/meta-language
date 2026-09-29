//! Output as a value, for the targets whose programs are pure.
//!
//! Rust and JavaScript print where the source prints; in Lean and Rocq a function that
//! prints, directly or through a function it calls, takes the lines printed
//! before it and returns them, with the lines it prints added in front, paired
//! with its value, and main prints the lines each of its steps adds. The
//! rewriting is target-independent: it yields a portable-core program without
//! `print` nodes, whose pairs are generated data types.
//!
//! Mirrors `js/src/translation/output.js`.

use std::borrow::Cow;
use std::collections::HashSet;

use super::Span;
use super::diagnostics::{Result, unsupported};
use super::ir::{
    Case, Comparison, Ctor, DataDecl, Decl, Effect, Expr, Field, FnDecl, LitValue, Main, Node,
    Param, Pattern, Program, Prop, TheoremDecl,
};
use super::surface::BinaryOp;
use super::types::{BOOL, Type, data};

/// The checked program with its output threaded through, or the program
/// itself when nothing prints below main.
///
/// # Errors
/// When an operand that prints sits in a compound assertion.
pub fn thread_output(program: &Program) -> Result<Cow<'_, Program>> {
    let mut effectful = HashSet::new();
    let mut grew = true;
    while grew {
        grew = false;
        for entry in &program.declarations {
            if let Decl::Fn(function) = entry
                && !effectful.contains(&function.full_name)
                && prints(&function.body, &effectful)
            {
                effectful.insert(function.full_name.clone());
                grew = true;
            }
        }
    }
    // A step of main is a `print` effect itself; what threads is a value that prints.
    let main_prints = program.main.as_ref().is_some_and(|main| {
        main.effects.iter().any(|effect| match effect {
            Effect::Let { value, .. } => prints(value, &effectful),
            Effect::Assert { prop, .. } => prop_prints(prop, &effectful),
            Effect::Print { expr, .. } | Effect::Output { expr, .. } => prints(expr, &effectful),
        })
    });
    if effectful.is_empty() && !main_prints {
        return Ok(Cow::Borrowed(program));
    }
    Threader {
        source: program,
        effectful,
        pairs: Vec::new(),
        count: 0,
    }
    .program()
    .map(Cow::Owned)
}

fn prints(expr: &Expr, effectful: &HashSet<String>) -> bool {
    match &expr.node {
        Node::Print { .. } => true,
        Node::Call { func, .. } if effectful.contains(func) => true,
        _ => expr
            .children()
            .into_iter()
            .any(|child| prints(child, effectful)),
    }
}

fn prop_prints(prop: &Prop, effectful: &HashSet<String>) -> bool {
    match prop {
        Prop::Forall { body, .. } => prop_prints(body, effectful),
        Prop::And { left, right } | Prop::Or { left, right } | Prop::Implies { left, right } => {
            prop_prints(left, effectful) || prop_prints(right, effectful)
        }
        Prop::Not { arg } => prop_prints(arg, effectful),
        Prop::Bool { expr } => prints(expr, effectful),
        other => other.comparison().is_some_and(|(_, comparison)| {
            prints(&comparison.left, effectful) || prints(&comparison.right, effectful)
        }),
    }
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
                    let out = "ml_out";
                    let mut params = function.params.clone();
                    params.push(Param {
                        name: out.to_owned(),
                        ty: Type::Output,
                        guard: None,
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
                Decl::Theorem(theorem) if self.prop_prints(&theorem.prop) => {
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
            ..self.source.clone()
        })
    }

    fn prints(&self, expr: &Expr) -> bool {
        prints(expr, &self.effectful)
    }

    fn prop_prints(&self, prop: &Prop) -> bool {
        prop_prints(prop, &self.effectful)
    }

    fn fresh(&mut self, prefix: &str) -> String {
        self.count += 1;
        format!("ml_{prefix}{}", self.count)
    }

    /// The generated data type pairing the lines printed so far with a value of `ty`.
    fn pair(&mut self, ty: &Type) -> String {
        let key = ty.key();
        if let Some((_, entry)) = self.pairs.iter().find(|(candidate, _)| *candidate == key) {
            return entry.full_name.clone();
        }
        let name = format!("ml_io{}", self.pairs.len() + 1);
        self.pairs.push((
            key,
            DataDecl {
                name: name.clone(),
                ctors: vec![Ctor {
                    name: format!("{name}_mk"),
                    fields: vec![
                        Field {
                            name: "output".to_owned(),
                            ty: Type::Output,
                        },
                        Field {
                            name: "value".to_owned(),
                            ty: ty.clone(),
                        },
                    ],
                }],
                span: None,
                full_name: name.clone(),
                module_path: Vec::new(),
                generated: true,
                output: true,
            },
        ));
        name
    }

    fn pair_type(&mut self, ty: &Type) -> Type {
        data(self.pair(ty))
    }

    fn make(&mut self, out: Expr, value: Expr) -> Expr {
        let name = self.pair(&value.ty);
        Expr::new(
            Node::Ctor {
                ctor: format!("{name}_mk"),
                data: name.clone(),
                args: vec![out, value],
            },
            data(name),
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

    /// `inner` under the matches of `frames`, innermost last.
    fn close(frames: Vec<Frame>, inner: Expr) -> Expr {
        frames.into_iter().rev().fold(inner, |inner, frame| {
            let ty = inner.ty.clone();
            let ctor = format!("{}_mk", frame.data);
            Expr::new(
                Node::Match {
                    scrutinee: Box::new(frame.scrutinee),
                    cases: vec![Case {
                        pattern: Pattern::Ctor {
                            data: frame.data,
                            ctor,
                            binds: vec![Some(frame.out), Some(frame.value)],
                        },
                        body: inner,
                    }],
                },
                ty,
            )
        })
    }

    /// The operands run in order after the lines `out`: the matches taking
    /// apart the pairs of those that print, the lines after them all, and
    /// their values; a value that prints nothing is used as it is.
    fn bind_all(&mut self, list: &[&Expr], mut out: Expr) -> Result<(Vec<Frame>, Expr, Vec<Expr>)> {
        let mut frames = Vec::new();
        let mut values = Vec::new();
        for &operand in list {
            if self.prints(operand) {
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

    /// `e` run after the lines `out`, a variable: its value paired with the lines printed by then.
    fn io(&mut self, e: &Expr, out: Expr) -> Result<Expr> {
        if !self.prints(e) {
            return Ok(self.make(out, e.clone()));
        }
        let ty = self.pair_type(&e.ty);
        let rebuilt = |node: Node| Expr {
            node,
            ty: ty.clone(),
            span: e.span,
        };
        match &e.node {
            Node::Print { text, body } => {
                let (frames, before, text) = self.bind(text, out)?;
                let after = self.fresh("o");
                let value = Expr::new(
                    Node::OutCons {
                        head: Box::new(text),
                        tail: Box::new(before),
                    },
                    Type::Output,
                );
                let body = self.io(body, Expr::var(after.clone(), Type::Output))?;
                let inner = Expr::new(
                    Node::Let {
                        name: after,
                        value: Box::new(value),
                        body: Box::new(body),
                    },
                    ty,
                );
                Ok(Self::close(frames, inner))
            }
            Node::Call { func, args } => {
                let operands: Vec<&Expr> = args.iter().collect();
                let (frames, after, mut args) = self.bind_all(&operands, out)?;
                let inner = if self.effectful.contains(func) {
                    args.push(after);
                    rebuilt(Node::Call {
                        func: func.clone(),
                        args,
                    })
                } else {
                    let call = Expr {
                        node: Node::Call {
                            func: func.clone(),
                            args,
                        },
                        ty: e.ty.clone(),
                        span: e.span,
                    };
                    self.make(after, call)
                };
                Ok(Self::close(frames, inner))
            }
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let (frames, after, cond) = self.bind(cond, out)?;
                let then = self.io(then, after.clone())?;
                let otherwise = self.io(otherwise, after)?;
                let inner = rebuilt(Node::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                });
                Ok(Self::close(frames, inner))
            }
            Node::Let { name, value, body } => {
                let (frames, after, value) = self.bind(value, out)?;
                let body = self.io(body, after)?;
                let inner = rebuilt(Node::Let {
                    name: name.clone(),
                    value: Box::new(value),
                    body: Box::new(body),
                });
                Ok(Self::close(frames, inner))
            }
            Node::Match { scrutinee, cases } => {
                let (frames, after, scrutinee) = self.bind(scrutinee, out)?;
                let mut threaded = Vec::new();
                for kase in cases {
                    threaded.push(Case {
                        pattern: kase.pattern.clone(),
                        body: self.io(&kase.body, after.clone())?,
                    });
                }
                let inner = rebuilt(Node::Match {
                    scrutinee: Box::new(scrutinee),
                    cases: threaded,
                });
                Ok(Self::close(frames, inner))
            }
            // The right operand of `&&` and `||` runs only when the left one does not decide.
            Node::Binary {
                op: op @ (BinaryOp::And | BinaryOp::Or),
                left,
                right,
                ..
            } => {
                let decided = Expr::lit(BOOL, LitValue::Bool(*op == BinaryOp::Or));
                let (then, otherwise) = if *op == BinaryOp::And {
                    ((**right).clone(), decided)
                } else {
                    (decided, (**right).clone())
                };
                let test = Expr::new(
                    Node::If {
                        cond: left.clone(),
                        then: Box::new(then),
                        otherwise: Box::new(otherwise),
                    },
                    BOOL,
                );
                self.io(&test, out)
            }
            // Every other node runs its operands in order, then computes from their values.
            _ => {
                let operands = e.children();
                let (frames, after, values) = self.bind_all(&operands, out)?;
                let mut values = values.into_iter();
                let copy = e.map_children(&mut |_| values.next().expect("an operand value"));
                let inner = self.make(after, copy);
                Ok(Self::close(frames, inner))
            }
        }
    }

    /// The value of `e` alone, for a theorem, which states facts of values.
    fn value(&mut self, e: &Expr) -> Result<Expr> {
        let pair = self.io(e, out_nil())?;
        let frame = self.open(pair, &e.ty);
        let value = Expr::var(frame.value.clone(), e.ty.clone());
        Ok(Self::close(vec![frame], value))
    }

    fn pure_expr(&mut self, e: &Expr) -> Result<Expr> {
        if self.prints(e) {
            self.value(e)
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

    /// A main step whose value prints: the pair of its value and the lines it
    /// prints, a step printing those lines, and the step itself on the value.
    fn effect(&mut self, effect: &Effect) -> Result<Vec<Effect>> {
        Ok(match effect {
            Effect::Let { name, value, span } if self.prints(value) => {
                let (mut steps, value) = self.run(value)?;
                steps.push(Effect::Let {
                    name: name.clone(),
                    value,
                    span: *span,
                });
                steps
            }
            Effect::Print { expr, span } if self.prints(expr) => {
                let (mut steps, expr) = self.run(expr)?;
                steps.push(Effect::Print { expr, span: *span });
                steps
            }
            Effect::Assert { prop, span } if self.prop_prints(prop) => {
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
        let lines = Self::close(vec![lines], lines_var);
        let result = self.open(pair, &e.ty);
        let result_var = Expr::var(result.value.clone(), e.ty.clone());
        let result = Self::close(vec![result], result_var);
        Ok((
            vec![
                Effect::Let {
                    name,
                    value,
                    span: e.span,
                },
                Effect::Output {
                    expr: lines,
                    span: e.span,
                },
            ],
            result,
        ))
    }

    fn lift(&mut self, e: &Expr, steps: &mut Vec<Effect>) -> Result<Expr> {
        if !self.prints(e) {
            return Ok(e.clone());
        }
        let (run, value) = self.run(e)?;
        steps.extend(run);
        Ok(value)
    }

    /// An assertion's operands that print run before it, in order, as steps of main.
    fn hoist(&mut self, prop: &Prop, steps: &mut Vec<Effect>, span: Option<Span>) -> Result<Prop> {
        Ok(match prop {
            Prop::Bool { expr } => Prop::Bool {
                expr: self.lift(expr, steps)?,
            },
            Prop::Not { arg } => Prop::Not {
                arg: Box::new(self.hoist(arg, steps, span)?),
            },
            Prop::And { .. } | Prop::Or { .. } | Prop::Implies { .. } | Prop::Forall { .. } => {
                return Err(unsupported(
                    "output in a compound assertion",
                    "an operand that prints runs only when the assertion evaluates it; assert on a value computed before",
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
