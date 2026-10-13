//! Theorem properties and their explicitly bounded JavaScript domains.

use super::super::ir::{Prop, TheoremDecl, rename_theorem};
use super::{
    Decl, Helper, JavaScriptEmitter, Result, TheoremCheck, Type, ctor_object, ident, malformed,
};

impl JavaScriptEmitter<'_> {
    pub(super) fn theorem(&mut self, entry: &Decl, theorem: &TheoremDecl) -> Result<String> {
        let (binders, prop, _) = rename_theorem(theorem, &ident, &self.state.local_reserved());
        let name = self.state.local_name(&theorem.full_name).to_owned();
        self.state.map(entry, &name);
        self.state
            .theorem(&theorem.full_name, &name, binders.is_empty(), true);
        self.theorem_checks.push(TheoremCheck {
            reference: self.state.reference(&theorem.full_name, "."),
            binders: binders.clone(),
            source: theorem.full_name.clone(),
        });
        let names: Vec<&str> = binders.iter().map(|binder| binder.name.as_str()).collect();
        Ok(format!(
            "{name}({}) {{\n  return {};\n}}",
            names.join(", "),
            self.prop(&prop)?
        ))
    }

    pub(super) fn theorem_runner(&mut self) -> Result<String> {
        self.state.encode("theorem-properties", "each theorem is an executable property; --ml-check-theorems evaluates it on every input of a bounded domain, and its proof remains checked by the source kernel");
        let mut checks = Vec::new();
        for check in self.theorem_checks.clone() {
            let mut domains = Vec::new();
            for binder in &check.binders {
                domains.push(self.domain(&binder.ty, 3)?);
            }
            let TheoremCheck {
                reference, source, ..
            } = check;
            let call = if domains.is_empty() {
                format!("{reference}()")
            } else {
                format!(
                    "ml_product([{}]).every((args) => {reference}(...args))",
                    domains.join(", ")
                )
            };
            checks.push(format!("  if (!({call})) throw new Error('theorem {source} fails on a bounded input');\n  console.log('theorem {source}: holds on the bounded domain');"));
        }
        Ok(format!(
            "function ml_checkTheorems() {{\n{}\n}}",
            checks.join("\n")
        ))
    }

    /// Values of a type up to a constructor depth, as JavaScript source.
    fn domain(&mut self, ty: &Type, depth: i64) -> Result<String> {
        self.helpers.insert(Helper::Domains);
        let values = match ty {
            Type::Nat => if depth >= 3 { "ml_nat" } else { "ml_small_nat" }.to_owned(),
            Type::Int => if depth >= 3 { "ml_int" } else { "ml_small_int" }.to_owned(),
            Type::Fixed { signed, .. } => if *signed {
                "ml_small_int"
            } else {
                "ml_small_nat"
            }
            .to_owned(),
            Type::Bool => "[false, true]".to_owned(),
            Type::String => "['', 'a', 'ab']".to_owned(),
            Type::Unit => "[null]".to_owned(),
            Type::Data { name } => self.data_domain(name, depth)?,
            other => {
                return Err(malformed(format!(
                    "no JavaScript domain for {}",
                    other.kind()
                )));
            }
        };
        Ok(values)
    }

    fn data_domain(&mut self, name: &str, depth: i64) -> Result<String> {
        let entry = self.program.data(name);
        let mut values = Vec::new();
        for ctor in &entry.ctors {
            let recursive = ctor
                .fields
                .iter()
                .any(|field| matches!(field.ty, Type::Data { .. }));
            if recursive && depth <= 1 {
                continue;
            }
            let mut fields = Vec::new();
            for field in &ctor.fields {
                let field_depth = if matches!(field.ty, Type::Data { .. }) {
                    depth - 1
                } else {
                    1
                };
                fields.push(self.domain(&field.ty, field_depth)?);
            }
            if fields.is_empty() {
                values.push(format!(
                    "[{}]",
                    ctor_object(ctor, &[], self.program.source_language, false)
                ));
            } else {
                let args: Vec<String> = (0..ctor.fields.len())
                    .map(|index| format!("args[{index}]"))
                    .collect();
                values.push(format!(
                    "ml_product([{}]).map((args) => {})",
                    fields.join(", "),
                    ctor_object(ctor, &args, self.program.source_language, false)
                ));
            }
        }
        if values.is_empty() {
            return Ok("[]".to_owned());
        }
        let spread: Vec<String> = values.iter().map(|value| format!("...{value}")).collect();
        Ok(format!("[{}]", spread.join(", ")))
    }

    pub(super) fn prop(&mut self, prop: &Prop) -> Result<String> {
        Ok(match prop {
            Prop::Forall { binders, body } => {
                let mut inner = self.prop(body)?;
                for binder in binders.iter().rev() {
                    let domain = self.domain(&binder.ty, 3)?;
                    inner = format!("ml_forall({domain}, ({}) => {inner})", binder.name);
                }
                inner
            }
            Prop::And { left, right } => {
                format!("({} && {})", self.prop(left)?, self.prop(right)?)
            }
            Prop::Or { left, right } => {
                format!("({} || {})", self.prop(left)?, self.prop(right)?)
            }
            Prop::Implies { left, right } => {
                format!("(!{} || {})", self.prop(left)?, self.prop(right)?)
            }
            Prop::Not { arg } => format!("!{}", self.prop(arg)?),
            Prop::Bool { expr } => self.expr(expr)?,
            Prop::Eq(comparison) | Prop::Ne(comparison) => {
                let left = self.expr(&comparison.left)?;
                let right = self.expr(&comparison.right)?;
                let structured = matches!(comparison.left.ty, Type::Data { .. } | Type::Unit);
                let equal = if structured {
                    self.helpers.insert(Helper::Equal);
                    format!("ml_equal({left}, {right})")
                } else if comparison.same_value {
                    format!("Object.is({left}, {right})")
                } else {
                    format!("({left} === {right})")
                };
                if matches!(prop, Prop::Eq(_)) {
                    equal
                } else {
                    format!("!{equal}")
                }
            }
            Prop::Lt(comparison)
            | Prop::Le(comparison)
            | Prop::Gt(comparison)
            | Prop::Ge(comparison) => {
                let operator = match prop {
                    Prop::Lt(_) => "<",
                    Prop::Le(_) => "<=",
                    Prop::Gt(_) => ">",
                    _ => ">=",
                };
                format!(
                    "({} {operator} {})",
                    self.expr(&comparison.left)?,
                    self.expr(&comparison.right)?
                )
            }
        })
    }
}
