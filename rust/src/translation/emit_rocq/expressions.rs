//! Expressions, literals, matches, casts and `main`.

use super::*;

impl RocqEmitter<'_> {
    pub(super) fn expr(&mut self, e: &Expr) -> Result<String> {
        match &e.node {
            Node::Lit { value } => self.literal(&e.ty, value),
            Node::Unit => Ok("tt".to_owned()),
            Node::Var { name } => Ok(name.clone()),
            Node::Call { func, args } => {
                let head = match &self.current {
                    Some(current) if current.full_name == *func && current.recursive => {
                        current.name.clone()
                    }
                    _ => self.state.reference(func, "."),
                };
                self.application(head, args)
            }
            Node::Ctor { data, ctor, args } => {
                let head = self.state.ctor_ref(data, ctor, ".");
                self.application(head, args)
            }
            Node::Unary { op, arg, semantics } => {
                if *op == UnaryOp::Not {
                    return Ok(format!("(negb {})", self.expr(arg)?));
                }
                if *semantics == Some(Semantics::Checked) {
                    self.state.checked_to_total("negation");
                }
                if e.ty.is_float() {
                    return Ok(format!("(PrimFloat.opp {})", self.expr(arg)?));
                }
                Ok(format!("(Z.opp {})", self.expr(arg)?))
            }
            Node::Binary { .. } => self.binary(e),
            Node::If {
                cond,
                then,
                otherwise,
            } => {
                let cond = self.expr(cond)?;
                let then = self.expr(then)?;
                Ok(format!(
                    "(if {cond} then {then} else {})",
                    self.expr(otherwise)?
                ))
            }
            Node::Let { name, value, body } => {
                let value = self.expr(value)?;
                Ok(format!("(let {name} := {value} in {})", self.expr(body)?))
            }
            Node::Match { scrutinee, cases } => self.match_expr(scrutinee, cases),
            Node::ToString { arg, console } => self.text_of(arg, *console),
            Node::Cast {
                arg,
                from,
                to,
                flavor,
            } => self.cast(arg, from, to, *flavor),
            Node::Abort { message } => {
                self.state.abort_to_total(message);
                Ok(format!(
                    "(* unreachable under the non-aborting assumption: {} *) {}",
                    message.replace("*)", "* )"),
                    self.inhabitant(&e.ty)?
                ))
            }
        }
    }

    pub(super) fn application(&mut self, head: String, args: &[Expr]) -> Result<String> {
        if args.is_empty() {
            return Ok(head);
        }
        let mut parts = vec![head];
        for arg in args {
            parts.push(self.expr(arg)?);
        }
        Ok(format!("({})", parts.join(" ")))
    }

    pub(super) fn literal(&mut self, ty: &Type, value: &LitValue) -> Result<String> {
        let text = value.text();
        let integer = |text: String| {
            if text.starts_with('-') {
                format!("({text})%Z")
            } else {
                format!("{text}%Z")
            }
        };
        Ok(match ty {
            Type::Nat => format!("{text}%N"),
            Type::Int => integer(text),
            Type::Fixed { signed, .. } => {
                self.state.fixed_to_unbounded(ty);
                if *signed {
                    integer(text)
                } else {
                    format!("{text}%N")
                }
            }
            Type::Bool => text,
            Type::String => format!("\"{}\"%string", text.replace('"', "\"\"")),
            Type::Float => {
                self.floats();
                match text.as_str() {
                    "NaN" => "PrimFloat.nan".to_owned(),
                    "Infinity" => "PrimFloat.infinity".to_owned(),
                    "-Infinity" => "PrimFloat.neg_infinity".to_owned(),
                    _ => text.strip_prefix('-').map_or_else(
                        || format!("{text}%float"),
                        |magnitude| format!("(PrimFloat.opp {magnitude}%float)"),
                    ),
                }
            }
            other => return Err(internal(format!("no Rocq literal for {}", other.kind()))),
        })
    }

    pub(super) fn inhabitant(&self, ty: &Type) -> Result<String> {
        Ok(match ty {
            Type::Nat | Type::Fixed { signed: false, .. } => "0%N".to_owned(),
            Type::Int | Type::Fixed { signed: true, .. } => "0%Z".to_owned(),
            Type::Bool => "false".to_owned(),
            Type::String => "EmptyString".to_owned(),
            Type::Float => "0%float".to_owned(),
            Type::Unit => "tt".to_owned(),
            Type::Data { name } => {
                let entry = self.program.data(name);
                let Some(ctor) = entry
                    .ctors
                    .iter()
                    .find(|candidate| {
                        candidate
                            .fields
                            .iter()
                            .all(|field| !matches!(field.ty, Type::Data { .. }))
                    })
                    .or_else(|| entry.ctors.first())
                else {
                    // JavaScript reads the fields of the missing constructor.
                    return Err(internal(
                        "Cannot read properties of undefined (reading 'fields')".to_owned(),
                    ));
                };
                let mut parts = vec![self.state.ctor_ref(name, &ctor.name, ".")];
                for field in &ctor.fields {
                    parts.push(self.inhabitant(&field.ty)?);
                }
                if parts.len() == 1 {
                    parts.swap_remove(0)
                } else {
                    format!("({})", parts.join(" "))
                }
            }
            other => return Err(internal(format!("no Rocq inhabitant for {}", other.kind()))),
        })
    }

    pub(super) fn binary(&mut self, e: &Expr) -> Result<String> {
        let Node::Binary {
            op,
            left,
            right,
            domain,
            semantics,
            rounding,
            by_zero,
        } = &e.node
        else {
            unreachable!("binary is only called on binary expressions");
        };
        let left = self.expr(left)?;
        let right = self.expr(right)?;
        let domain = domain.as_ref();
        match op {
            BinaryOp::And => Ok(format!("(andb {left} {right})")),
            BinaryOp::Or => Ok(format!("(orb {left} {right})")),
            BinaryOp::Concat => Ok(format!("(String.append {left} {right})")),
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => {
                let domain =
                    domain.ok_or_else(|| internal("comparison without a domain".to_owned()))?;
                self.comparison(*op, domain, &left, &right)
            }
            _ => {
                let domain =
                    domain.ok_or_else(|| internal("arithmetic without a domain".to_owned()))?;
                let module = self.numeric_module(domain)?;
                if *semantics == Some(Semantics::Checked) {
                    self.state.checked_to_total(op_name(*op));
                }
                if *by_zero == Some(ByZero::Abort) {
                    self.state.abort_to_total("division by zero");
                }
                if *semantics == Some(Semantics::Ieee) {
                    if *op == BinaryOp::Rem {
                        self.helpers.insert(Helper::FloatRem);
                        return Ok(format!("(ml_float_rem {left} {right})"));
                    }
                    return Ok(format!("(PrimFloat.{} {left} {right})", op_name(*op)));
                }
                match op {
                    BinaryOp::Add => Ok(format!("({module}.add {left} {right})")),
                    BinaryOp::Mul => Ok(format!("({module}.mul {left} {right})")),
                    // N.sub truncates at zero, which is exactly natural subtraction; a
                    // checked unsigned subtraction agrees with it on non-aborting runs.
                    BinaryOp::Sub => Ok(format!("({module}.sub {left} {right})")),
                    BinaryOp::Div | BinaryOp::Rem => {
                        let division = *op == BinaryOp::Div;
                        if module == "N" {
                            let name = if division { "div" } else { "modulo" };
                            return Ok(format!("(N.{name} {left} {right})"));
                        }
                        match rounding {
                            Some(Rounding::Trunc) => {
                                let name = if division { "quot" } else { "rem" };
                                Ok(format!("(Z.{name} {left} {right})"))
                            }
                            Some(Rounding::Floor) => {
                                let name = if division { "div" } else { "modulo" };
                                Ok(format!("(Z.{name} {left} {right})"))
                            }
                            _ => {
                                self.helpers.insert(Helper::Euclid);
                                let name = if division { "ediv" } else { "emod" };
                                Ok(format!("(ml_Z_{name} {left} {right})"))
                            }
                        }
                    }
                    other => Err(internal(format!("no Rocq arithmetic {}", op_name(*other)))),
                }
            }
        }
    }

    /// A Boolean comparison test.
    pub(super) fn comparison(
        &mut self,
        op: BinaryOp,
        domain: &Type,
        left: &str,
        right: &str,
    ) -> Result<String> {
        let module = match domain {
            Type::Bool => "Bool",
            Type::String => "String",
            other => self.numeric_module(other)?,
        };
        Ok(match op {
            BinaryOp::Eq => format!("({module}.eqb {left} {right})"),
            BinaryOp::Ne => format!("(negb ({module}.eqb {left} {right}))"),
            BinaryOp::Lt => format!("({module}.ltb {left} {right})"),
            BinaryOp::Le => format!("({module}.leb {left} {right})"),
            BinaryOp::Gt => format!("({module}.ltb {right} {left})"),
            _ => format!("({module}.leb {right} {left})"),
        })
    }

    pub(super) fn match_expr(&mut self, scrutinee: &Expr, cases: &[Case]) -> Result<String> {
        if let Some(subject) = scrutinee.var_name() {
            return self.match_on(subject, &scrutinee.ty, cases);
        }
        let name = "ml_scrutinee";
        let value = self.expr(scrutinee)?;
        Ok(format!(
            "(let {name} := {value} in {})",
            self.match_on(name, &scrutinee.ty, cases)?
        ))
    }

    pub(super) fn match_on(&mut self, subject: &str, ty: &Type, cases: &[Case]) -> Result<String> {
        let fallback = || {
            cases
                .iter()
                .find(|kase| matches!(kase.pattern, Pattern::Wild | Pattern::Bind { .. }))
        };
        let Type::Data { name: data_name } = ty else {
            // Natural-number matches test for zero; the successor case binds the predecessor.
            let zero = cases
                .iter()
                .find(|kase| matches!(kase.pattern, Pattern::NatZero));
            let succ = cases
                .iter()
                .find(|kase| matches!(kase.pattern, Pattern::NatSucc { .. }));
            let zero_text = match zero {
                Some(zero) => self.expr(&zero.body)?,
                None => self.bind_fallback(subject, fallback())?,
            };
            let succ_text = match succ {
                Some(Case {
                    pattern: Pattern::NatSucc { name },
                    body,
                }) => format!("(let {name} := N.pred {subject} in {})", self.expr(body)?),
                _ => self.bind_fallback(subject, fallback())?,
            };
            return Ok(format!(
                "(if N.eqb {subject} 0%N then {zero_text} else {succ_text})"
            ));
        };
        let program = self.program;
        let entry = program.data(data_name);
        let mut arms = Vec::new();
        for ctor in &entry.ctors {
            let kase = cases
                .iter()
                .find(|item| {
                    matches!(&item.pattern, Pattern::Ctor { ctor: name, .. } if *name == ctor.name)
                })
                .or_else(fallback)
                .expect("the checker makes every match exhaustive");
            let binds: Vec<&str> = match &kase.pattern {
                Pattern::Ctor { binds, .. } => binds
                    .iter()
                    .map(|bind| bind.as_deref().unwrap_or("_"))
                    .collect(),
                _ => ctor.fields.iter().map(|_| "_").collect(),
            };
            let mut body = self.expr(&kase.body)?;
            if let Pattern::Bind { name } = &kase.pattern {
                body = format!("(let {name} := {subject} in {body})");
            }
            let mut head = vec![self.state.ctor_ref(&entry.full_name, &ctor.name, ".")];
            head.extend(binds.into_iter().map(str::to_owned));
            arms.push(format!("| {} => {body}", head.join(" ")));
        }
        Ok(format!("(match {subject} with {} end)", arms.join(" ")))
    }

    pub(super) fn bind_fallback(&mut self, subject: &str, kase: Option<&Case>) -> Result<String> {
        let kase = kase.expect("the checker makes every match exhaustive");
        let body = self.expr(&kase.body)?;
        Ok(match &kase.pattern {
            Pattern::Bind { name } => format!("(let {name} := {subject} in {body})"),
            _ => body,
        })
    }

    pub(super) fn text_of(&mut self, arg: &Expr, console: bool) -> Result<String> {
        let text = self.expr(arg)?;
        match &arg.ty {
            Type::String => Ok(text),
            Type::Float => {
                self.helpers.insert(Helper::JsNumber);
                if console {
                    self.helpers.insert(Helper::JsConsole);
                    return Ok(format!("(ml_js_console {text})"));
                }
                Ok(format!("(ml_js_number {text})"))
            }
            Type::Bool => {
                self.helpers.insert(Helper::BoolToString);
                Ok(format!("(ml_bool_to_string {text})"))
            }
            Type::Nat => {
                self.helpers.insert(Helper::Digits);
                Ok(format!("(ml_N_to_string {text})"))
            }
            Type::Int => {
                self.helpers.insert(Helper::ZToString);
                Ok(format!("(ml_Z_to_string {text})"))
            }
            Type::Fixed { signed, .. } => {
                self.state.fixed_to_unbounded(&arg.ty);
                if *signed {
                    self.helpers.insert(Helper::ZToString);
                    Ok(format!("(ml_Z_to_string {text})"))
                } else {
                    self.helpers.insert(Helper::Digits);
                    Ok(format!("(ml_N_to_string {text})"))
                }
            }
            other => Err(unsupported(
                "output of structured values",
                &format!("a {} value has no portable textual form", other.kind()),
                arg.span,
            )),
        }
    }

    pub(super) fn cast(
        &mut self,
        arg: &Expr,
        from: &Type,
        to: &Type,
        flavor: Flavor,
    ) -> Result<String> {
        let arg = self.expr(arg)?;
        let module = |ty: &Type| match ty {
            Type::Fixed { signed, .. } => {
                if *signed {
                    "Z"
                } else {
                    "N"
                }
            }
            Type::Nat => "N",
            _ => "Z",
        };
        let (from, to) = (module(from), module(to));
        if flavor == Flavor::Checked {
            self.state.checked_to_total("conversion to a natural");
        }
        if from == to {
            return Ok(arg);
        }
        if from == "N" {
            return Ok(format!("(Z.of_N {arg})"));
        }
        Ok(format!("(Z.to_N {arg})"))
    }

    pub(super) fn main(&mut self, main: &Main) -> Result<String> {
        let effects = rename_main(main, &ident, &self.state.local_reserved());
        let mut assertion = 0;
        let mut theorems: Vec<String> = Vec::new();
        let mut body = String::new();
        for (index, effect) in effects.iter().enumerate() {
            match effect {
                Effect::Print { expr, .. } => {
                    body.push_str(&self.expr(expr)?);
                    body.push_str(" ::\n  ");
                }
                Effect::Let { name, value, .. } => {
                    let value = self.expr(value)?;
                    let _ = write!(body, "let {name} := {value} in\n  ");
                }
                Effect::Assert { prop, .. } => {
                    assertion += 1;
                    let mut lets = String::new();
                    for earlier in &effects[..index] {
                        if let Effect::Let { name, value, .. } = earlier {
                            let value = self.expr(value)?;
                            let _ = write!(lets, "let {name} := {value} in ");
                        }
                    }
                    let name = format!("ml_assertion_{assertion}");
                    let prop = self.prop(prop)?;
                    theorems.push(format!(
                        "Theorem {name} : {lets}{prop}.\nProof. ml_decide. Qed."
                    ));
                    self.helpers.insert(Helper::Decide);
                    self.state.assertion_theorem(&name, effect);
                }
            }
        }
        body.push_str("nil");
        if main.sequential_async {
            self.state.encode(
                "sequential-async",
                "an async function is the function its body computes and await is its call: every call of one is awaited where it is made, so nothing runs concurrently and the output is the same, in the same order",
            );
        }
        self.state.encode(
            "program-output",
            "main is the list of lines the source program prints, in order; evaluating it with vm_compute runs the program",
        );
        theorems.push(format!("Definition main : list string :=\n  {body}."));
        Ok(theorems.join("\n\n"))
    }
}
