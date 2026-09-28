//! Expressions, arithmetic, matches, casts and `main`.

use super::{
    block, comparison_operator, indent, own, rename_main, rounding_text, rust_string, snake,
    unsupported, BinaryOp, ByZero, Decimal, Effect, Expr, Flavor, LitValue, Node, Pattern, Result,
    Rounding, RustEmitter, Semantics, Type, UnaryOp,
};

impl RustEmitter<'_> {
    pub(super) fn expr(&mut self, expr: &Expr) -> Result<String> {
        match &expr.node {
            Node::Lit { value } => Ok(self.literal(expr, value)),
            Node::Unit => Ok("()".to_owned()),
            Node::Var { name } => Ok(own(name, &expr.ty)),
            Node::Call { func, args } => {
                let head = format!("crate::{}", self.state.reference(func, "::"));
                let mut texts = Vec::with_capacity(args.len());
                for arg in args {
                    texts.push(self.expr(arg)?);
                }
                Ok(format!("{head}({})", texts.join(", ")))
            }
            Node::Ctor { data, ctor, args } => {
                let program = self.program;
                let entry = program.data(data);
                let declared = entry
                    .ctors
                    .iter()
                    .find(|candidate| candidate.name == *ctor)
                    .expect("the checker resolves every constructor");
                let head = format!("crate::{}", self.state.ctor_ref(data, ctor, "::"));
                if args.is_empty() {
                    return Ok(head);
                }
                let mut texts = Vec::with_capacity(args.len());
                for (arg, field) in args.iter().zip(&declared.fields) {
                    let text = self.expr(arg)?;
                    texts.push(if matches!(field.ty, Type::Data { .. }) {
                        format!("Box::new({text})")
                    } else {
                        text
                    });
                }
                Ok(format!("{head}({})", texts.join(", ")))
            }
            Node::Unary { op, arg, .. } => {
                if *op == UnaryOp::Not {
                    return Ok(format!("!{}", self.receiver(arg)?));
                }
                if matches!(expr.ty, Type::Fixed { .. }) {
                    let text = format!("{}.checked_neg()", self.receiver(arg)?);
                    return Ok(self.checked(&text, &expr.ty, "negation"));
                }
                if expr.ty.is_float() {
                    return Ok(format!("(-{})", self.receiver(arg)?));
                }
                Ok(format!("{}.neg()", self.receiver(arg)?))
            }
            Node::Binary { .. } => self.binary(expr),
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let cond = self.expr(cond)?;
                let then = self.expr(then)?;
                let otherwise = self.expr(otherwise)?;
                Ok(format!(
                    "if {cond} {{\n{}\n}} else {{\n{}\n}}",
                    indent(&then, 1),
                    indent(&otherwise, 1)
                ))
            }
            Node::Let { name, value, body } => {
                let value = self.expr(value)?;
                let body = self.expr(body)?;
                Ok(block(&format!("let {name} = {value};\n{body}")))
            }
            Node::Match { .. } => self.match_expr(expr),
            Node::ToString { arg, console } => {
                match arg.ty {
                    Type::String => return self.expr(arg),
                    Type::Data { .. } | Type::Unit => {
                        return Err(unsupported(
                            "output of structured values",
                            &format!("a {} value has no portable textual form", arg.ty.kind()),
                            arg.span,
                        ));
                    }
                    Type::Float => {
                        self.uses_number = true;
                        let function = if *console { "js_console" } else { "js_number" };
                        return Ok(format!("crate::ml_number::{function}({})", self.expr(arg)?));
                    }
                    _ => {}
                }
                Ok(format!("{}.to_string()", self.receiver(arg)?))
            }
            Node::Cast { .. } => self.cast(expr),
            Node::Abort { message } => Ok(format!("panic!(\"{{}}\", {})", rust_string(message))),
        }
    }

    pub(super) fn literal(&mut self, expr: &Expr, value: &LitValue) -> String {
        let text = value.text();
        match &expr.ty {
            Type::Nat | Type::Int => {
                self.uses_big = true;
                let bound = 1i128 << 126;
                let small = Decimal::parse(&text)
                    .and_then(|value| value.to_i128())
                    .is_some_and(|value| value >= -bound && value < bound);
                if small {
                    format!("crate::ml::Big::from_i128({text})")
                } else {
                    format!("crate::ml::Big::parse(\"{text}\")")
                }
            }
            Type::Fixed { .. } => {
                if text.starts_with('-') {
                    format!("({text}{})", expr.ty.key())
                } else {
                    format!("{text}{}", expr.ty.key())
                }
            }
            Type::Float => {
                self.floats();
                rust_float(&text)
            }
            Type::Bool => text,
            Type::String => format!("String::from({})", rust_string(&text)),
            other => unreachable!("no Rust literal for {}", other.kind()),
        }
    }

    pub(super) fn checked(&mut self, text: &str, ty: &Type, what: &str) -> String {
        let key = ty.key();
        self.state.encode(
            &format!("machine-integer:{key}"),
            &format!("{key} stays a Rust {key}; its arithmetic is checked and panics where the source aborts"),
        );
        format!("{text}.expect(\"{key} {what} overflowed\")")
    }

    pub(super) fn binary(&mut self, expr: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left,
            right,
            semantics,
            ..
        } = &expr.node
        else {
            unreachable!("a binary expression")
        };
        match op {
            BinaryOp::And => Ok(format!("({} && {})", self.expr(left)?, self.expr(right)?)),
            BinaryOp::Or => Ok(format!("({} || {})", self.expr(left)?, self.expr(right)?)),
            BinaryOp::Concat => Ok(format!(
                "format!(\"{{}}{{}}\", {}, {})",
                self.receiver(left)?,
                self.receiver(right)?
            )),
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => Ok(format!(
                "({} {} {})",
                self.borrow(left)?,
                comparison_operator(*op),
                self.borrow(right)?
            )),
            _ if *semantics == Some(Semantics::Ieee) => self.float_arithmetic(*op, left, right),
            _ if matches!(expr.ty, Type::Fixed { .. }) => self.fixed_arithmetic(expr),
            _ => self.big_arithmetic(expr),
        }
    }

    pub(super) fn floats(&mut self) {
        self.state.encode(
            "floats",
            "a JavaScript Number is an f64 with the same IEEE-754 arithmetic; % on f64 is the same truncated remainder, and ml_number::js_number prints a value as JavaScript does",
        );
    }

    /// f64 arithmetic is IEEE-754 binary64 arithmetic, as JavaScript's; Rust's
    /// `%` is fmod, as JavaScript's.
    fn float_arithmetic(&mut self, op: BinaryOp, left: &Expr, right: &Expr) -> Result<String> {
        let operator = match op {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            _ => "%",
        };
        Ok(format!(
            "({} {operator} {})",
            self.expr(left)?,
            self.expr(right)?
        ))
    }

    pub(super) fn fixed_arithmetic(&mut self, expr: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left,
            right,
            rounding,
            ..
        } = &expr.node
        else {
            unreachable!("a binary expression")
        };
        let left = self.receiver(left)?;
        let right = self.expr(right)?;
        let (method, what) = match op {
            BinaryOp::Add => ("checked_add", "addition"),
            BinaryOp::Sub => ("checked_sub", "subtraction"),
            BinaryOp::Mul => ("checked_mul", "multiplication"),
            BinaryOp::Div => ("checked_div", "division"),
            BinaryOp::Rem => ("checked_rem", "remainder"),
            other => unreachable!("no Rust machine-integer operator {other:?}"),
        };
        let division = matches!(op, BinaryOp::Div | BinaryOp::Rem);
        let signed = matches!(expr.ty, Type::Fixed { signed: true, .. });
        let mut method = method.to_owned();
        if division && *rounding == Some(Rounding::Euclid) {
            method.push_str("_euclid");
        } else if division && *rounding != Some(Rounding::Trunc) && signed {
            return Err(unsupported(
                &format!("{} division on {}", rounding_text(*rounding), expr.ty.key()),
                "Rust machine integers divide with truncating or Euclidean rounding only",
                expr.span,
            ));
        }
        if division {
            self.state.abort_to_total("division by zero");
        }
        Ok(self.checked(&format!("{left}.{method}({right})"), &expr.ty, what))
    }

    pub(super) fn big_arithmetic(&mut self, expr: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left: left_expr,
            right: right_expr,
            semantics,
            rounding,
            by_zero,
            ..
        } = &expr.node
        else {
            unreachable!("a binary expression")
        };
        let left = self.receiver(left_expr)?;
        let right = self.borrow(right_expr)?;
        match op {
            BinaryOp::Add => Ok(format!("{left}.add({right})")),
            BinaryOp::Mul => Ok(format!("{left}.mul({right})")),
            BinaryOp::Sub => {
                if *semantics == Some(Semantics::Truncated) {
                    return Ok(format!(
                        "crate::ml::nat_sub({}, {right})",
                        self.borrow(left_expr)?
                    ));
                }
                Ok(format!("{left}.sub({right})"))
            }
            BinaryOp::Div | BinaryOp::Rem => {
                let abort = *by_zero == Some(ByZero::Abort);
                if abort {
                    self.state.abort_to_total("division by zero");
                }
                Ok(format!(
                    "crate::ml::divide({}, {right}, \"{}\", {abort}, {})",
                    self.borrow(left_expr)?,
                    rounding_text(*rounding),
                    *op == BinaryOp::Rem
                ))
            }
            other => unreachable!("no Rust operator {other:?}"),
        }
    }

    pub(super) fn match_expr(&mut self, expr: &Expr) -> Result<String> {
        let Node::Match { scrutinee, cases } = &expr.node else {
            unreachable!("a match expression")
        };
        let fallback = cases
            .iter()
            .find(|kase| matches!(kase.pattern, Pattern::Wild | Pattern::Bind { .. }));
        if let Type::Data { name } = &scrutinee.ty {
            let program = self.program;
            let entry = program.data(name);
            let mut arms = Vec::with_capacity(cases.len());
            for kase in cases {
                let arm = match &kase.pattern {
                    Pattern::Wild => format!("_ => {}", block(&self.expr(&kase.body)?)),
                    Pattern::Bind { name } => {
                        format!("{name} => {}", block(&self.expr(&kase.body)?))
                    }
                    Pattern::Ctor { data, ctor, binds } => {
                        let declared = entry
                            .ctors
                            .iter()
                            .find(|candidate| candidate.name == *ctor)
                            .expect("the checker resolves every constructor");
                        let head = format!("crate::{}", self.state.ctor_ref(data, ctor, "::"));
                        let names: Vec<&str> = binds
                            .iter()
                            .map(|bind| bind.as_deref().unwrap_or("_"))
                            .collect();
                        let mut lines: Vec<String> = binds
                            .iter()
                            .zip(&declared.fields)
                            .filter_map(|(bind, field)| match bind {
                                Some(bind) if matches!(field.ty, Type::Data { .. }) => {
                                    Some(format!("let {bind} = *{bind};"))
                                }
                                _ => None,
                            })
                            .collect();
                        lines.push(self.expr(&kase.body)?);
                        let text = if names.is_empty() {
                            head
                        } else {
                            format!("{head}({})", names.join(", "))
                        };
                        format!("{text} => {}", block(&lines.join("\n")))
                    }
                    Pattern::NatZero | Pattern::NatSucc { .. } => {
                        unreachable!("a natural pattern on a data type")
                    }
                };
                arms.push(arm);
            }
            return Ok(format!(
                "match {} {{\n{}\n}}",
                self.expr(scrutinee)?,
                indent(&arms.join("\n"), 1)
            ));
        }
        let natural = cases
            .iter()
            .any(|kase| matches!(kase.pattern, Pattern::NatZero | Pattern::NatSucc { .. }));
        if scrutinee.ty != Type::Nat && !natural {
            let kase = fallback.expect("a match without constructors has a fallback case");
            let pattern = match &kase.pattern {
                Pattern::Bind { name } => name.as_str(),
                _ => "_",
            };
            let subject = self.expr(scrutinee)?;
            let body = block(&self.expr(&kase.body)?);
            return Ok(format!(
                "match {subject} {{\n{}\n}}",
                indent(&format!("{pattern} => {body}"), 1)
            ));
        }
        let mut lines = Vec::new();
        let subject = if let Some(name) = scrutinee.var_name() {
            name.to_owned()
        } else {
            self.temporaries += 1;
            let subject = format!("__subject{}", self.temporaries);
            let value = self.expr(scrutinee)?;
            lines.push(format!("let {subject} = {value};"));
            subject
        };
        let zero = cases
            .iter()
            .find(|kase| matches!(kase.pattern, Pattern::NatZero));
        let succ = cases.iter().find_map(|kase| match &kase.pattern {
            Pattern::NatSucc { name } => Some((name, &kase.body)),
            _ => None,
        });
        let zero_text = match zero {
            Some(zero) => self.expr(&zero.body)?,
            None => self.fallback_body(fallback, &subject)?,
        };
        let machine = matches!(scrutinee.ty, Type::Fixed { .. });
        // On an unsigned machine integer the successor case holds a value of at least 1, so `- 1` cannot wrap.
        let predecessor = if machine {
            format!("{subject} - 1")
        } else {
            format!("crate::ml::pred(&{subject})")
        };
        let succ_text = match succ {
            Some((name, body)) => format!("let {name} = {predecessor};\n{}", self.expr(body)?),
            None => self.fallback_body(fallback, &subject)?,
        };
        let test = if machine {
            format!("{subject} == 0")
        } else {
            format!("{subject}.is_zero()")
        };
        lines.push(format!(
            "if {test} {{\n{}\n}} else {{\n{}\n}}",
            indent(&zero_text, 1),
            indent(&succ_text, 1)
        ));
        if lines.len() == 1 {
            return Ok(lines.remove(0));
        }
        Ok(block(&lines.join("\n")))
    }

    pub(super) fn fallback_body(
        &mut self,
        fallback: Option<&crate::translation::ir::Case>,
        subject: &str,
    ) -> Result<String> {
        let kase = fallback.expect("a match missing a natural case has a fallback case");
        let mut lines = Vec::new();
        if let Pattern::Bind { name } = &kase.pattern {
            lines.push(format!("let {name} = {subject}.clone();"));
        }
        lines.push(self.expr(&kase.body)?);
        Ok(lines.join("\n"))
    }

    pub(super) fn cast(&mut self, expr: &Expr) -> Result<String> {
        let Node::Cast {
            arg,
            from,
            to,
            flavor,
        } = &expr.node
        else {
            unreachable!("a cast")
        };
        let arg = self.expr(arg)?;
        if matches!(from, Type::Fixed { .. }) && matches!(to, Type::Fixed { .. }) {
            return Ok(format!("{}::from({arg})", to.key()));
        }
        match flavor {
            Flavor::Exact => Ok(arg),
            Flavor::Clamp => Ok(format!("crate::ml::clamp_nat({arg})")),
            Flavor::Checked => {
                self.state.checked_to_total("conversion to a natural");
                Ok(format!("crate::ml::to_nat_checked({arg})"))
            }
        }
    }

    pub(super) fn main(&mut self, main: &crate::translation::ir::Main) -> Result<String> {
        let effects = rename_main(main, &snake, &self.state.local_reserved());
        let mut lines = Vec::with_capacity(effects.len());
        let mut assertion = 0;
        for effect in &effects {
            match effect {
                Effect::Print { expr, .. } => {
                    lines.push(format!("println!(\"{{}}\", {});", self.expr(expr)?));
                }
                Effect::Let { name, value, .. } => {
                    lines.push(format!("let {name} = {};", self.expr(value)?));
                }
                Effect::Assert { prop, .. } => {
                    assertion += 1;
                    lines.push(format!(
                        "assert!({}, \"assertion {assertion}\");",
                        self.prop(prop)?
                    ));
                    self.state
                        .assertion_theorem(&format!("assertion {assertion}"), effect);
                }
            }
        }
        if main.sequential_async {
            self.state.encode(
                "sequential-async",
                "an async function is the function its body computes and await is its call: every call of one is awaited where it is made, so nothing runs concurrently and the output is the same, in the same order",
            );
        }
        self.state.encode(
            "program-output",
            "main prints the lines the source program prints, in order, with println!",
        );
        Ok(format!(
            "fn ml_main() {{\n{}\n}}",
            indent(&lines.join("\n"), 1)
        ))
    }
}

/// A Number's canonical JavaScript text as an `f64` expression.
fn rust_float(value: &str) -> String {
    match value {
        "NaN" => "f64::NAN".to_owned(),
        "Infinity" => "f64::INFINITY".to_owned(),
        "-Infinity" => "f64::NEG_INFINITY".to_owned(),
        _ if value.starts_with('-') => format!("({value}f64)"),
        _ => format!("{value}f64"),
    }
}
