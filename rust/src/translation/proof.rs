//! Portable proof plans.
//!
//! A source proof keeps its structure: which variable is split by induction or case analysis,
//! and the names of fields and induction hypotheses. It also keeps its hints: definitions to
//! unfold, lemmas and hypotheses to use, and whether arithmetic or computation closes a goal.
//! Each target emits its own tactics from the plan and its kernel re-checks the result, so a
//! proof is never copied as text into a language that cannot check it.
//!
//! Mirrors `js/src/translation/proof.js`.

use std::collections::HashMap;

use super::diagnostics::{type_error, unsupported, Result};
use super::ir::{Binder, Ctor, Expr, Field, Hints, Node, Plan, PlanCase, Proof, Prop, Split};
use super::surface::{SProof, SSplit, SStep};
use super::types::Type;
use super::Language;

/// What a name in a proof refers to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProofName {
    Function(String),
    Theorem(String),
    Other,
}

/// The checker's view of the program, as a proof sees it.
pub trait ProofContext {
    /// Resolves a (dotted) name from the theorem's module.
    ///
    /// # Errors
    /// When the name climbs above the crate root.
    fn lookup(&self, name: &str) -> Result<Option<ProofName>>;

    /// The constructors of a data type.
    fn ctors(&self, data: &str) -> Vec<Ctor>;
}

/// The theorem a proof belongs to, with its binders already hoisted.
pub struct TheoremHead<'a> {
    pub full_name: &'a str,
    pub binders: &'a [Binder],
}

/// Normalises a surface proof into a portable plan.
///
/// # Errors
/// On steps outside the portable proof model, unknown constructors and self-reference.
pub fn normalise_proof(
    proof: &SProof,
    theorem: &TheoremHead<'_>,
    context: &dyn ProofContext,
) -> Result<Proof> {
    let mut hints = Hints::default();
    let locals: HashMap<String, Type> = theorem
        .binders
        .iter()
        .map(|binder| (binder.name.clone(), binder.ty.clone()))
        .collect();
    let normaliser = Normaliser {
        theorem,
        context,
        language: proof.source_language,
    };
    let plan = normaliser.sequence(&proof.steps, &mut hints, &locals)?;
    Ok(Proof {
        plan,
        source: proof.source.clone().unwrap_or_default(),
        source_language: proof.source_language,
    })
}

fn merge_hints(into: &mut Hints, from: &Hints) {
    let merge = |into: &mut Vec<String>, from: &[String]| {
        for value in from {
            if !into.contains(value) {
                into.push(value.clone());
            }
        }
    };
    merge(&mut into.unfold, &from.unfold);
    merge(&mut into.lemmas, &from.lemmas);
    merge(&mut into.hyps, &from.hyps);
    merge(&mut into.library, &from.library);
    into.arith |= from.arith;
    into.compute |= from.compute;
}

struct Normaliser<'a> {
    theorem: &'a TheoremHead<'a>,
    context: &'a dyn ProofContext,
    language: Language,
}

impl Normaliser<'_> {
    fn sequence(
        &self,
        steps: &[SStep],
        hints: &mut Hints,
        locals: &HashMap<String, Type>,
    ) -> Result<Plan> {
        for (index, step) in steps.iter().enumerate() {
            match step {
                SStep::Intro { names } => {
                    // Leading binders are hoisted into the theorem's parameters, where the
                    // target introduces them; later `intro`s have nothing left to name.
                    if names.iter().any(|name| !locals.contains_key(name)) {
                        return Err(unsupported(
                            "intro",
                            "introducing hypotheses beyond the theorem binders is outside the portable proof model",
                            None,
                        ));
                    }
                }
                SStep::Unfold { names, .. } => {
                    for name in names {
                        self.add_rule(hints, name, locals, false)?;
                    }
                }
                SStep::Rewrite { rules, .. } => {
                    for rule in rules {
                        self.add_rule(hints, &rule.name, locals, rule.reverse)?;
                    }
                }
                SStep::Simp { rules, .. } => {
                    for rule in rules {
                        self.add_rule(hints, &rule.name, locals, rule.reverse)?;
                    }
                    hints.compute = true;
                }
                SStep::Compute { .. } => hints.compute = true,
                SStep::Arith { .. } => hints.arith = true,
                SStep::Induction(split) => {
                    return self.split(split, true, &steps[index + 1..], hints, locals)
                }
                SStep::Cases(split) => {
                    return self.split(split, false, &steps[index + 1..], hints, locals)
                }
            }
        }
        Ok(Plan::Close {
            hints: hints.clone(),
        })
    }

    fn add_rule(
        &self,
        hints: &mut Hints,
        name: &str,
        locals: &HashMap<String, Type>,
        reverse: bool,
    ) -> Result<()> {
        let push = |list: &mut Vec<String>, value: &str| {
            if !list.iter().any(|item| item == value) {
                list.push(value.to_owned());
            }
        };
        if locals.contains_key(name) {
            push(&mut hints.hyps, name);
            return Ok(());
        }
        match self.context.lookup(name)? {
            Some(ProofName::Function(full_name)) => {
                push(&mut hints.unfold, &full_name);
                return Ok(());
            }
            Some(ProofName::Theorem(full_name)) => {
                if full_name == self.theorem.full_name {
                    return Err(type_error(format!("{name} uses itself"), None));
                }
                push(&mut hints.lemmas, &full_name);
                return Ok(());
            }
            _ => {}
        }
        // Library facts such as `Nat.mul_add` or `Nat.add_comm`: every target
        // closes arithmetic with its own decision procedure, so the name is kept
        // for provenance only.
        if !hints.library.iter().any(|item| item == name) {
            hints.library.push(if reverse {
                format!("<-{name}")
            } else {
                name.to_owned()
            });
        }
        hints.arith = true;
        Ok(())
    }

    fn split(
        &self,
        step: &SSplit,
        induction: bool,
        trailing: &[SStep],
        hints: &Hints,
        locals: &HashMap<String, Type>,
    ) -> Result<Plan> {
        let tactic = if induction { "induction" } else { "cases" };
        let Some(ty) = locals.get(&step.variable) else {
            return Err(unsupported(
                &format!("{tactic} on {}", step.variable),
                "only theorem binders can be split",
                None,
            ));
        };
        let Some(ctors) = self.constructors_of(ty) else {
            return Err(unsupported(
                &format!("{tactic} on {}", ty.key()),
                "only natural numbers and data types can be split",
                None,
            ));
        };
        // Rocq names cases by position (`as [| k ih]` and one bullet per
        // subgoal, in constructor order); Lean names them by constructor.
        if step.positional && step.cases.len() > ctors.len() {
            return Err(type_error(
                format!(
                    "{tactic} on {} has {} cases but {} has {} constructors",
                    step.variable,
                    step.cases.len(),
                    ty.key(),
                    ctors.len()
                ),
                None,
            ));
        }
        let mut cases = Vec::new();
        for (ctor_index, ctor) in ctors.iter().enumerate() {
            let written = if step.positional {
                step.cases.get(ctor_index)
            } else {
                step.cases
                    .iter()
                    .find(|kase| canonical_ctor(kase.ctor.as_deref(), ty) == ctor.name)
            };
            let recursive: Vec<bool> = ctor
                .fields
                .iter()
                .map(|field| field.ty.key() == ty.key())
                .collect();
            let binds = written.map_or(&[][..], |kase| &kase.binds[..]);
            let (fields, ihs) = assign_binds(binds, ctor, &recursive, induction, self.language)?;
            let mut inner = locals.clone();
            for (field, name) in ctor.fields.iter().zip(&fields) {
                inner.insert(name.clone(), field.ty.clone());
            }
            for ih in &ihs {
                inner.insert(ih.clone(), Type::Hypothesis);
            }
            let mut case_hints = Hints::default();
            merge_hints(&mut case_hints, hints);
            let mut steps: Vec<SStep> = written.map(|kase| kase.steps.clone()).unwrap_or_default();
            steps.extend(trailing.iter().cloned());
            let plan = self.sequence(&steps, &mut case_hints, &inner)?;
            cases.push(PlanCase {
                ctor: ctor.name.clone(),
                fields,
                ihs,
                recursive,
                plan,
            });
        }
        if !step.positional {
            for kase in &step.cases {
                let name = canonical_ctor(kase.ctor.as_deref(), ty);
                if !ctors.iter().any(|ctor| ctor.name == name) {
                    return Err(type_error(
                        format!(
                            "{} has no constructor {}",
                            ty.key(),
                            kase.ctor.as_deref().unwrap_or("undefined")
                        ),
                        None,
                    ));
                }
            }
        }
        let split = Split {
            variable: step.variable.clone(),
            ty: ty.clone(),
            cases,
        };
        Ok(if induction {
            Plan::Induction(split)
        } else {
            Plan::Cases(split)
        })
    }

    fn constructors_of(&self, ty: &Type) -> Option<Vec<Ctor>> {
        match ty {
            Type::Nat => Some(vec![
                Ctor {
                    name: "zero".to_owned(),
                    fields: Vec::new(),
                },
                Ctor {
                    name: "succ".to_owned(),
                    fields: vec![Field {
                        name: "pred".to_owned(),
                        ty: ty.clone(),
                    }],
                },
            ]),
            Type::Data { name } => Some(self.context.ctors(name)),
            _ => None,
        }
    }
}

fn canonical_ctor(name: Option<&str>, ty: &Type) -> String {
    let name = name.unwrap_or("undefined");
    if matches!(ty, Type::Nat) {
        return match name {
            "zero" | "O" | "Nat.zero" => "zero",
            "succ" | "S" | "Nat.succ" => "succ",
            other => other,
        }
        .to_owned();
    }
    name.rsplit('.').next().unwrap_or(name).to_owned()
}

/// Lean lists all fields first and then one hypothesis per recursive field;
/// Rocq's `as` patterns put each hypothesis right after its field.
fn assign_binds(
    binds: &[Option<String>],
    ctor: &Ctor,
    recursive: &[bool],
    induction: bool,
    language: Language,
) -> Result<(Vec<String>, Vec<String>)> {
    let mut fields = Vec::new();
    let mut ihs = Vec::new();
    let mut cursor = 0;
    let mut fresh = 0;
    let mut take = |prefix: &str| {
        fresh += 1;
        let fallback = format!("ml_{prefix}{fresh}");
        let name = binds.get(cursor).cloned().flatten();
        cursor += 1;
        match name {
            Some(name) if !name.is_empty() && name != "_" => name,
            _ => fallback,
        }
    };
    if language == Language::Rocq {
        for &is_recursive in recursive.iter().take(ctor.fields.len()) {
            fields.push(take("x"));
            if induction && is_recursive {
                ihs.push(take("ih"));
            }
        }
    } else {
        for _ in &ctor.fields {
            fields.push(take("x"));
        }
        if induction {
            for &is_recursive in recursive {
                if is_recursive {
                    ihs.push(take("ih"));
                }
            }
        }
    }
    if cursor < binds.len() {
        return Err(type_error(
            format!("{} case names {} variables", ctor.name, binds.len()),
            None,
        ));
    }
    Ok((fields, ihs))
}

/// Functions, in first-use order, that a proposition mentions.
#[must_use]
pub fn prop_functions(prop: &Prop, mut into: Vec<String>) -> Vec<String> {
    fn visit(expr: &Expr, into: &mut Vec<String>) {
        if let Node::Call { func, .. } = &expr.node {
            if !into.contains(func) {
                into.push(func.clone());
            }
        }
        for child in expr.children() {
            visit(child, into);
        }
    }
    fn visit_prop(prop: &Prop, into: &mut Vec<String>) {
        match prop {
            Prop::Forall { body, .. } => visit_prop(body, into),
            Prop::And { left, right }
            | Prop::Or { left, right }
            | Prop::Implies { left, right } => {
                visit_prop(left, into);
                visit_prop(right, into);
            }
            Prop::Not { arg } => visit_prop(arg, into),
            Prop::Bool { expr } => visit(expr, into),
            other => {
                if let Some((_, comparison)) = other.comparison() {
                    visit(&comparison.left, into);
                    visit(&comparison.right, into);
                }
            }
        }
    }
    visit_prop(prop, &mut into);
    into
}
