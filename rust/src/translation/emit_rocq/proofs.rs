//! Theorems, proof plans, closers and propositions.

use super::*;

/// How many times one proof step rewrites with one hypothesis or lemma.
const REWRITE_BOUND: usize = 8;

impl RocqEmitter<'_> {
    pub(super) fn theorem(&mut self, entry: &Decl, theorem: &TheoremDecl) -> Result<String> {
        let (binders, prop, plan) = rename_theorem(theorem, &ident, &self.state.local_reserved());
        let name = self.state.local_name(&theorem.full_name).to_owned();
        self.state.map(entry, &name);
        let mut statement = format!("Theorem {name}");
        for binder in &binders {
            let ty = self.ty(&binder.ty)?;
            let _ = write!(statement, " ({} : {ty})", binder.name);
        }
        let prop_text = self.prop(&prop)?;
        let _ = write!(statement, " : {prop_text}.");
        let mut functions = prop_functions(&prop, Vec::new());
        for name in collect_hint_functions(&plan) {
            if !functions.contains(name) {
                functions.push(name.clone());
            }
        }
        let script = self.plan(&plan, &functions, binders.is_empty(), 1);
        self.state
            .theorem(&theorem.full_name, &name, binders.is_empty(), false);
        Ok(format!("{statement}\nProof.\n{script}\nQed."))
    }

    pub(super) fn plan(
        &self,
        plan: &Plan,
        functions: &[String],
        closed: bool,
        depth: usize,
    ) -> String {
        let indent = "  ".repeat(depth);
        let (split, induction) = match plan {
            Plan::Close { hints } => {
                return format!("{indent}{}.", self.closer(hints, functions, closed));
            }
            Plan::Induction(split) => (split, true),
            Plan::Cases(split) => (split, false),
        };
        let pattern = if split.ty == Type::Nat {
            let succ = split
                .cases
                .iter()
                .find(|item| item.ctor == "succ")
                .expect("a natural split has a successor case");
            let ih = if induction {
                succ.ihs.first().map_or("undefined", String::as_str)
            } else {
                "_"
            };
            let field = succ.fields.first().map_or("undefined", String::as_str);
            format!(
                "induction {} as [|{field} {ih}] using N.peano_ind.",
                split.variable
            )
        } else {
            let groups: Vec<String> = split
                .cases
                .iter()
                .map(|item| {
                    let mut names: Vec<&str> = Vec::new();
                    for (index, field) in item.fields.iter().enumerate() {
                        names.push(field);
                        if induction && item.recursive.get(index).copied().unwrap_or(false) {
                            let earlier = item.recursive[..index]
                                .iter()
                                .filter(|recursive| **recursive)
                                .count();
                            names.push(item.ihs.get(earlier).map_or("undefined", String::as_str));
                        }
                    }
                    names.join(" ")
                })
                .collect();
            format!(
                "{} {} as [{}].",
                if induction { "induction" } else { "destruct" },
                split.variable,
                groups.join(" | ")
            )
        };
        let mut lines = vec![format!("{indent}{pattern}")];
        for item in &split.cases {
            lines.push(format!("{indent}{{"));
            lines.push(self.plan(&item.plan, functions, false, depth + 1));
            lines.push(format!("{indent}}}"));
        }
        lines.join("\n")
    }

    pub(super) fn closer(&self, hints: &Hints, functions: &[String], closed: bool) -> String {
        let mut steps: Vec<String> = Vec::new();
        let mut unfold_all: Vec<&String> = Vec::new();
        for name in functions.iter().chain(&hints.unfold) {
            if !unfold_all.contains(&name) {
                unfold_all.push(name);
            }
        }
        let plain: Vec<String> = unfold_all
            .iter()
            .filter(|name| !self.nat_functions.contains_key(name.as_str()))
            .map(|name| self.state.reference(name, "."))
            .collect();
        if !plain.is_empty() {
            steps.push(format!("cbn [{}]", plain.join(" ")));
        }
        for name in &unfold_all {
            let Some(&(index, arity)) = self.nat_functions.get(name.as_str()) else {
                continue;
            };
            let reference = self.state.reference(name, ".");
            let args = |value: &str| {
                (0..arity)
                    .map(|position| {
                        if position == index {
                            value.to_owned()
                        } else {
                            format!("?a{position}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            let uses = |value: &str| {
                (0..arity)
                    .map(|position| {
                        if position == index {
                            value.to_owned()
                        } else {
                            format!("a{position}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            steps.push(format!(
                "repeat match goal with |- context [{reference} {}] => rewrite ({reference}_equation {}) end",
                args("(N.succ ?k)"),
                uses("(N.succ k)")
            ));
            steps.push(format!(
                "repeat match goal with |- context [{reference} {}] => rewrite ({reference}_equation {}) end",
                args("0%N"),
                uses("0%N")
            ));
        }
        steps.push("ml_N_norm".to_owned());
        steps.push("cbv beta iota zeta".to_owned());
        let mut alternatives: Vec<String> = Vec::new();
        if closed {
            alternatives.push("(vm_compute; reflexivity)".to_owned());
        }
        alternatives.push("ml_close".to_owned());
        let rewrites: Vec<String> = hints
            .hyps
            .iter()
            .cloned()
            .chain(
                hints
                    .lemmas
                    .iter()
                    .map(|lemma| self.state.reference(lemma, ".")),
            )
            .collect();
        if !rewrites.is_empty() {
            // Each rule rewrites at most REWRITE_BOUND times: an unbounded `?ih`
            // never stops when the rewritten side reappears in the result
            // (`rewrite <- ih` with `ih : f l = l` turns `l` into `f l` forever),
            // so a false obligation must fail instead of searching without end.
            let rules = rewrites
                .iter()
                .map(|rule| format!("{REWRITE_BOUND}?{rule}"))
                .collect::<Vec<_>>()
                .join(", ");
            alternatives.push(format!("(rewrite {rules}; ml_close)"));
            alternatives.push(format!("(rewrite <- {rules}; ml_close)"));
        }
        format!("{}; first [{}]", steps.join("; "), alternatives.join(" | "))
    }

    pub(super) fn prop(&mut self, prop: &Prop) -> Result<String> {
        Ok(match prop {
            Prop::Forall { binders, body } => {
                let mut parts = Vec::new();
                for binder in binders {
                    let ty = self.ty(&binder.ty)?;
                    parts.push(format!("({} : {ty})", binder.name));
                }
                format!("(forall {}, {})", parts.join(" "), self.prop(body)?)
            }
            Prop::And { left, right } => {
                let left = self.prop(left)?;
                format!("({left} /\\ {})", self.prop(right)?)
            }
            Prop::Or { left, right } => {
                let left = self.prop(left)?;
                format!("({left} \\/ {})", self.prop(right)?)
            }
            Prop::Implies { left, right } => {
                let left = self.prop(left)?;
                format!("({left} -> {})", self.prop(right)?)
            }
            Prop::Not { arg } => format!("(~ {})", self.prop(arg)?),
            Prop::Bool { expr } => format!("({} = true)", self.expr(expr)?),
            Prop::Eq(comparison) | Prop::Ne(comparison) if comparison.left.ty.is_float() => {
                self.float_prop(prop)?
            }
            Prop::Lt(comparison)
            | Prop::Le(comparison)
            | Prop::Gt(comparison)
            | Prop::Ge(comparison)
                if comparison.domain.is_float() =>
            {
                self.float_prop(prop)?
            }
            Prop::Eq(comparison) => {
                let left = self.expr(&comparison.left)?;
                format!("({left} = {})", self.expr(&comparison.right)?)
            }
            Prop::Ne(comparison) => {
                let left = self.expr(&comparison.left)?;
                format!("({left} <> {})", self.expr(&comparison.right)?)
            }
            Prop::Lt(comparison)
            | Prop::Le(comparison)
            | Prop::Gt(comparison)
            | Prop::Ge(comparison) => {
                let module = self.numeric_module(&comparison.domain)?;
                let left = self.expr(&comparison.left)?;
                let right = self.expr(&comparison.right)?;
                match prop {
                    Prop::Lt(_) => format!("({module}.lt {left} {right})"),
                    Prop::Le(_) => format!("({module}.le {left} {right})"),
                    Prop::Gt(_) => format!("({module}.lt {right} {left})"),
                    _ => format!("({module}.le {right} {left})"),
                }
            }
        })
    }

    /// Number propositions are the source's Boolean tests computing to true.
    fn float_prop(&mut self, prop: &Prop) -> Result<String> {
        let (op, comparison) = prop.comparison().expect("a comparison");
        let left = self.expr(&comparison.left)?;
        let right = self.expr(&comparison.right)?;
        if comparison.same_value {
            self.helpers.insert(Helper::FloatSame);
            let expected = matches!(prop, Prop::Eq(_));
            return Ok(format!("(ml_float_same {left} {right} = {expected})"));
        }
        let test = self.comparison(op, &Type::Float, &left, &right)?;
        Ok(format!("({test} = true)"))
    }

    pub(super) fn floats(&mut self) {
        self.uses_float = true;
        self.state.encode(
            "floats",
            "a JavaScript Number is a Rocq primitive float, the same IEEE-754 binary64 with the same arithmetic, which the kernel computes with; % is ml_float_rem, the exact truncated remainder, and ml_js_number prints a value as JavaScript does",
        );
    }

    pub(super) fn numeric_module(&mut self, ty: &Type) -> Result<&'static str> {
        match ty {
            Type::Nat => Ok("N"),
            Type::Int => Ok("Z"),
            Type::Fixed { signed, .. } => {
                self.state.fixed_to_unbounded(ty);
                Ok(if *signed { "Z" } else { "N" })
            }
            Type::Float => {
                self.floats();
                Ok("PrimFloat")
            }
            other => Err(internal(format!("not numeric: {}", other.kind()))),
        }
    }
}
