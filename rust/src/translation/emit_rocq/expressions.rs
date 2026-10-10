//! Expressions, literals, matches, casts and `main`.

use super::{
    BinaryOp, ByZero, Case, Effect, Expr, Flavor, Helper, LitValue, Main, Node, Pattern, Rc,
    Result, RocqEmitter, Rounding, Semantics, Type, UnaryOp, ident, internal, op_name, rename_main,
    unsupported, unthreaded,
};

impl RocqEmitter<'_> {
    pub(super) fn expr(&mut self, e: &Expr) -> Result<String> {
        match &e.node {
            Node::Lit { value } => self.literal(&e.ty, value),
            Node::Unit => Ok("tt".to_owned()),
            Node::OutNil => Ok("(@nil string)".to_owned()),
            Node::OutCons { head, tail } => {
                Ok(format!("({} :: {})", self.expr(head)?, self.expr(tail)?))
            }
            // Output threading leaves no `print` node.
            Node::Print { .. } => Err(internal("print with its output not threaded".to_owned())),
            Node::Var { name } => Ok(name.clone()),
            Node::Call { func, args } => {
                let member = self.current.as_ref().and_then(|current| {
                    let mutual = current.mutual.as_ref()?;
                    Some((Rc::clone(mutual), mutual.index(func)?))
                });
                if let Some((mutual, index)) = member {
                    let mut parts = Vec::new();
                    for arg in args {
                        parts.push(self.expr(arg)?);
                    }
                    return Ok(mutual.call(index, &parts));
                }
                let head = match &self.current {
                    Some(current)
                        if current.full_name == *func && current.recursive && current.fuel =>
                    {
                        let mut parts = Vec::new();
                        for arg in args {
                            parts.push(self.expr(arg)?);
                        }
                        return Ok(format!("(ml_go ml_fuel {})", parts.join(" ")));
                    }
                    Some(current) if current.full_name == *func && current.general => {
                        let mut parts = Vec::new();
                        for arg in args {
                            parts.push(self.expr(arg)?);
                        }
                        return Ok(match parts.as_slice() {
                            [] => "(ml_rec tt)".to_owned(),
                            [only] => format!("(ml_rec {only})"),
                            _ => format!("(ml_rec ({}))", parts.join(", ")),
                        });
                    }
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
                    unthreaded("a machine-integer negation");
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
            Node::StringMap { op, .. } => Err(crate::translation::diagnostics::unsupported(
                &format!(".{op}()"),
                &crate::translation::frontend_rules::read_string_map_refusal("Rocq", op),
                e.span,
            )),
            Node::StringTest { op, string, search } => {
                let helper =
                    crate::translation::frontend_rules::read_string_test_helper("Rocq", op);
                self.helpers.insert(match helper.as_str() {
                    "stringStartsWith" => Helper::StringStartsWith,
                    "stringEndsWith" => Helper::StringEndsWith,
                    "stringIncludes" => Helper::StringIncludes,
                    _ => unreachable!("checked string predicate has a helper"),
                });
                let object = self.expr(string)?;
                let search = self.expr(search)?;
                Ok(
                    crate::translation::frontend_rules::render_string_test_expression(
                        "Rocq", op, &object, &search,
                    ),
                )
            }
            Node::ToString { arg, console } => self.text_of(arg, *console),
            Node::Cast {
                arg,
                from,
                to,
                flavor,
                ..
            } => self.cast(arg, from, to, *flavor),
            Node::Abort { .. } => unthreaded("an abort"),
            Node::Array { .. } | Node::Append { .. } | Node::Index { .. } | Node::Length { .. } => {
                self.array_expr(e)
            }
            Node::Math { .. } => self.math(e),
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
                self.state
                    .machine_integer(ty, if *signed { "Z" } else { "N" });
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

    pub(super) fn inhabitant(&mut self, ty: &Type) -> Result<String> {
        Ok(match ty {
            Type::Nat | Type::Fixed { signed: false, .. } => "0%N".to_owned(),
            Type::Int | Type::Fixed { signed: true, .. } => "0%Z".to_owned(),
            Type::Bool => "false".to_owned(),
            Type::String => "EmptyString".to_owned(),
            Type::Float => "0%float".to_owned(),
            Type::Unit => "tt".to_owned(),
            Type::Output => "(@nil string)".to_owned(),
            Type::Array { element } => format!("(@nil {})", self.ty(element)?),
            Type::Data { name } => {
                let program = self.program;
                let entry = program.data(name);
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
                // A machine-integer operation, one that may abort, reaches here as its abort test and its value computed on the representation (output.rs).
                if *semantics == Some(Semantics::Checked)
                    || *by_zero == Some(ByZero::Abort)
                    || matches!(e.ty, Type::Fixed { .. })
                {
                    unthreaded(&format!("a machine-integer {}", op_name(*op)));
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
                    // N.sub truncates at zero, which is exactly natural subtraction.
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
                self.state
                    .machine_integer(&arg.ty, if *signed { "Z" } else { "N" });
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
            unthreaded("a checked conversion to a natural");
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
        // A program that may abort is the lines it prints and the message of the abort that stops it, if one does.
        let aborts = self.program.aborts_threaded;
        if aborts {
            self.helpers.insert(Helper::Emit);
        }
        let body = self.build(&effects, 0, &mut assertion, &mut theorems)?;
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
        if self.program.output_threaded {
            self.state.encode(
                "output-threading",
                "a function that prints, directly or through a function it calls, takes the lines printed before it and returns them, with its own in front, paired with its value in a generated ml_io data type; main lists the lines of each step in the order they were printed",
            );
        }
        if aborts {
            self.state.encode(
                "abort-threading",
                "a function that may abort, directly or through a function it calls, returns the source's abort message with the lines printed before it in the generated ml_io data type; main is the pair of the lines printed and Some message when the program aborts, None when it does not",
            );
        }
        let result = if aborts {
            "list string * option string"
        } else {
            "list string"
        };
        theorems.push(format!("Definition main : {result} :=\n  {body}."));
        Ok(theorems.join("\n\n"))
    }

    /// The match on a step's pair: its value bound to the step's name, or the abort's message.
    fn arms(&mut self, effect: &Effect, made: &str, bind: &str, body: &str) -> Result<String> {
        let Effect::Unwrap {
            name,
            pair,
            data,
            ctors,
            ..
        } = effect
        else {
            unreachable!("unwrap effects only");
        };
        let mk = self.state.ctor_ref(data, &ctors[0], ".");
        let abort = self.state.ctor_ref(data, &ctors[1], ".");
        Ok(format!(
            "match {} with {mk} _ {name} => {made} | {abort} _ {bind} => {body} end",
            self.expr(pair)?
        ))
    }

    /// Main's value from the effect at `index` on.
    fn build(
        &mut self,
        effects: &[Effect],
        index: usize,
        assertion: &mut usize,
        theorems: &mut Vec<String>,
    ) -> Result<String> {
        let aborts = self.program.aborts_threaded;
        let Some(effect) = effects.get(index) else {
            return Ok(if aborts { "(nil, None)" } else { "nil" }.to_owned());
        };
        match effect {
            Effect::Print { expr, .. } => {
                let expr = self.expr(expr)?;
                let rest = self.build(effects, index + 1, assertion, theorems)?;
                Ok(if aborts {
                    format!("ml_emit ({expr} :: nil)\n  ({rest})")
                } else {
                    format!("{expr} ::\n  {rest}")
                })
            }
            Effect::Output { expr, .. } => {
                let head = if aborts { "ml_emit" } else { "app" };
                let expr = self.expr(expr)?;
                let rest = self.build(effects, index + 1, assertion, theorems)?;
                Ok(format!("{head} (List.rev {expr})\n  ({rest})"))
            }
            Effect::Let { name, value, .. } => {
                let value = self.expr(value)?;
                let rest = self.build(effects, index + 1, assertion, theorems)?;
                Ok(format!("let {name} := {value} in\n  {rest}"))
            }
            Effect::Unwrap { .. } => {
                let rest = self.build(effects, index + 1, assertion, theorems)?;
                Ok(format!(
                    "({})",
                    self.arms(effect, &rest, "ml_m", "(nil, Some ml_m)")?
                ))
            }
            Effect::Assert { prop, .. } => {
                *assertion += 1;
                // The assertion is stated of the values main computes before it; a run that aborts before it never reaches it.
                let mut statement = self.prop(prop)?;
                for item in effects[..index].iter().rev() {
                    match item {
                        Effect::Let { name, value, .. } => {
                            statement =
                                format!("let {name} := {} in {statement}", self.expr(value)?);
                        }
                        Effect::Unwrap { .. } => {
                            statement = format!("({})", self.arms(item, &statement, "_", "True")?);
                        }
                        _ => {}
                    }
                }
                let name = format!("ml_assertion_{assertion}");
                theorems.push(format!(
                    "Theorem {name} : {statement}.\nProof. ml_decide. Qed."
                ));
                self.helpers.insert(Helper::Decide);
                self.state.assertion_theorem(&name, effect);
                self.build(effects, index + 1, assertion, theorems)
            }
        }
    }
}
