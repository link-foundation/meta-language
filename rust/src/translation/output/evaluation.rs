//! Existing output sequencing and generated optional absence guards.
use super::{
    BOOL, BinaryOp, Case, Expr, LitValue, Node, Result, STRING, Threader, Type, literal, logical,
};

impl Threader<'_> {
    /// `e` run after the lines `out`, a variable: its value paired with the
    /// lines printed by then, or its abort.
    pub(super) fn io(&mut self, e: &Expr, out: Expr) -> Result<Expr> {
        if !self.effects(e) {
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
                Ok(self.close(frames, inner))
            }
            Node::Abort { message } => Ok(self.fail(out, literal(STRING, &message), &e.ty)),
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
                Ok(self.close(frames, inner))
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
                Ok(self.close(frames, inner))
            }
            Node::Let { name, value, body } => {
                let (frames, after, value) = self.bind(value, out)?;
                let body = self.io(body, after)?;
                let inner = rebuilt(Node::Let {
                    name: name.clone(),
                    value: Box::new(value),
                    body: Box::new(body),
                });
                Ok(self.close(frames, inner))
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
                Ok(self.close(frames, inner))
            }
            Node::Coalesce { left, right } => {
                let (frames, after, left) = self.bind(left, out)?;
                let name = self.fresh("optional");
                let value = Expr::var(name.clone(), left.ty.clone());
                let Type::Option { absence, .. } = &left.ty else {
                    unreachable!("a checked coalescing value retains its optional type");
                };
                let mut tests = Vec::new();
                for absence in
                    crate::translation::optional_value_rules::read_optional_output_absences(absence)
                {
                    let absent = Expr::new(
                        Node::None {
                            absence: absence.clone(),
                        },
                        crate::translation::types::option(Type::Unit, absence),
                    );
                    tests.push(Expr::new(
                        Node::StrictEqual {
                            left: Box::new(value.clone()),
                            right: Box::new(absent),
                            negate: true,
                        },
                        BOOL,
                    ));
                }
                let cond = tests
                    .into_iter()
                    .reduce(|left, right| logical(BinaryOp::And, left, right))
                    .expect("the generated absence plan tests at least one actual policy");
                let present = Expr::new(
                    Node::UnwrapOption {
                        arg: Box::new(value),
                    },
                    e.ty.clone(),
                );
                let then = self.make(after.clone(), present);
                let otherwise = self.io(right, after)?;
                let body = rebuilt(Node::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    otherwise: Box::new(otherwise),
                });
                let inner = rebuilt(Node::Let {
                    name,
                    value: Box::new(left),
                    body: Box::new(body),
                });
                Ok(self.close(frames, inner))
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
                let inner = self.complete(after, copy);
                Ok(self.close(frames, inner))
            }
        }
    }
}
