//! The Lean `main`: the program's output, and its assertions, proved by the
//! kernel or checked when `main` runs.

use std::collections::HashSet;

use super::{LeanEmitter, ident};
use crate::translation::diagnostics::{Result, type_error};
use crate::translation::ir::{Effect, Expr, Main, Node, Prop, rename_main};
use crate::translation::surface::BinaryOp;

impl LeanEmitter<'_> {
    pub(super) fn main(&mut self, main: &Main) -> Result<String> {
        let effects = rename_main(main, &ident, &self.state.local_reserved());
        let mut lines = Vec::new();
        let mut theorems = Vec::new();
        let mut assertion = 0;
        for (index, effect) in effects.iter().enumerate() {
            match effect {
                Effect::Print { expr, .. } => {
                    lines.push(format!("  IO.println {}", self.expr(expr, 1)?));
                }
                Effect::Output { expr, .. } => {
                    lines.push(format!(
                        "  for ml_line in {}.reverse do IO.println ml_line",
                        self.expr(expr, 1)?
                    ));
                }
                Effect::Let { name, value, .. } => {
                    lines.push(format!("  let {name} := {}", self.expr(value, 1)?));
                }
                Effect::Unwrap {
                    name,
                    pair,
                    data,
                    ctors,
                    ..
                } => {
                    // The value of a step, or the source's abort: its message, uncaught, ends main.
                    let made = self.state.ctor_ref(data, &ctors[0], ".");
                    let aborted = self.state.ctor_ref(data, &ctors[1], ".");
                    lines.push(format!(
                        "  let {name} ← (match {} with\n    | {made} _ ml_v => pure ml_v\n    | {aborted} _ ml_m => throw (IO.userError ml_m))",
                        self.expr(pair, 1)?
                    ));
                }
                Effect::Assert { prop, .. }
                    if self.uses_number.prop(prop) || self.on_partial(prop, &effects[..index]) =>
                {
                    // The kernel cannot evaluate Float or unfold a partial def, so the assertion runs where the source's does.
                    assertion += 1;
                    lines.push(format!(
                        "  if !{} then throw (IO.userError \"assertion {assertion} failed\")",
                        self.check(prop)?
                    ));
                    self.state.assertion_theorem_with(
                        &format!("assertion {assertion}"),
                        effect,
                        Some("runtime-assertion"),
                    );
                }
                Effect::Assert { prop, .. } => {
                    assertion += 1;
                    // The assertion is stated of the values main computes before it; a run that aborts before it never reaches it.
                    // Where a run may abort, the statement is a Bool the kernel evaluates: the decided assertion, or true where one aborts.
                    let threaded = self.program.aborts_threaded;
                    let unreached = if threaded { "true" } else { "True" };
                    let mut statement = if threaded {
                        format!("decide ({})", self.prop(prop)?)
                    } else {
                        self.prop(prop)?
                    };
                    for item in effects[..index].iter().rev() {
                        match item {
                            Effect::Let { name, value, .. } => {
                                statement =
                                    format!("let {name} := {}; {statement}", self.expr(value, 1)?);
                            }
                            Effect::Unwrap {
                                name,
                                pair,
                                data,
                                ctors,
                                ..
                            } => {
                                let made = self.state.ctor_ref(data, &ctors[0], ".");
                                let aborted = self.state.ctor_ref(data, &ctors[1], ".");
                                statement = format!(
                                    "(match {} with | {made} _ {name} => {statement} | {aborted} _ _ => {unreached})",
                                    self.expr(pair, 1)?
                                );
                            }
                            _ => {}
                        }
                    }
                    if threaded {
                        statement = format!("({statement}) = true");
                    }
                    let name = format!("ml_assertion_{assertion}");
                    theorems.push(format!(
                        "theorem {name} : {statement} := by\n  try rfl\n  try decide"
                    ));
                    self.state.assertion_theorem(&name, effect);
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
            "main prints the lines the source program prints, in order, with IO.println",
        );
        let body = if lines.is_empty() {
            "  pure ()".to_owned()
        } else {
            lines.join("\n")
        };
        if self.program.output_threaded {
            self.state.encode(
                "output-threading",
                "a function that prints, directly or through a function it calls, takes the lines printed before it and returns them, with its own in front, paired with its value in a generated ml_io data type; main prints the lines of each step in the order they were printed",
            );
        }
        if self.program.aborts_threaded {
            self.state.encode(
                "abort-threading",
                "a function that may abort, directly or through a function it calls, returns the source's abort message with the lines printed before it in the generated ml_io data type; main prints those lines and throws IO.userError with the message, which lean --run reports as \"uncaught exception: <message>\" with exit code 1",
            );
        }
        theorems.push(format!("def main : IO Unit := do\n{body}"));
        Ok(theorems.join("\n\n"))
    }

    /// Whether the proposition, or a main binding it reads, calls a partial def.
    fn on_partial(&self, prop: &Prop, before: &[Effect]) -> bool {
        let mut reads = HashSet::new();
        prop_reads(prop, &mut reads);
        let mut partial = self.uses_partial.prop(prop);
        for effect in before.iter().rev() {
            if partial {
                break;
            }
            let (name, value) = match effect {
                Effect::Let { name, value, .. } => (name, value),
                Effect::Unwrap { name, pair, .. } => (name, pair),
                _ => continue,
            };
            if reads.contains(name.as_str()) {
                partial = self.uses_partial.expr(value);
                expr_reads(value, &mut reads);
            }
        }
        partial
    }

    /// A proposition as a Bool computed at run time.
    fn check(&mut self, prop: &Prop) -> Result<String> {
        Ok(match prop {
            Prop::And { left, right } => {
                format!("({} && {})", self.check(left)?, self.check(right)?)
            }
            Prop::Or { left, right } => {
                format!("({} || {})", self.check(left)?, self.check(right)?)
            }
            Prop::Implies { left, right } => {
                format!("(!{} || {})", self.check(left)?, self.check(right)?)
            }
            Prop::Not { arg } => format!("(!{})", self.check(arg)?),
            Prop::Bool { expr } => self.expr(expr, 1)?,
            Prop::Forall { .. } => {
                return Err(type_error(
                    "no run-time check for a quantified proposition".to_owned(),
                    None,
                ));
            }
            other => {
                let (op, comparison) = other.comparison().expect("a comparison");
                let left = self.expr(&comparison.left, 1)?;
                let right = self.expr(&comparison.right, 1)?;
                let equal = matches!(other, Prop::Eq(_));
                if comparison.same_value {
                    self.helpers.insert("floatSame");
                    let not = if equal { "" } else { "!" };
                    return Ok(format!("({not}ml_float_same {left} {right})"));
                }
                let operator = match op {
                    BinaryOp::Eq => "==",
                    BinaryOp::Ne => "!=",
                    BinaryOp::Lt => "<",
                    BinaryOp::Le => "≤",
                    BinaryOp::Gt => ">",
                    _ => "≥",
                };
                if matches!(op, BinaryOp::Eq | BinaryOp::Ne) {
                    format!("({left} {operator} {right})")
                } else {
                    format!("(decide ({left} {operator} {right}))")
                }
            }
        })
    }
}

fn expr_reads(expr: &Expr, reads: &mut HashSet<String>) {
    if let Node::Var { name } = &expr.node {
        reads.insert(name.clone());
    }
    for child in expr.children() {
        expr_reads(child, reads);
    }
}

fn prop_reads(prop: &Prop, reads: &mut HashSet<String>) {
    match prop {
        Prop::Forall { body, .. } => prop_reads(body, reads),
        Prop::And { left, right } | Prop::Or { left, right } | Prop::Implies { left, right } => {
            prop_reads(left, reads);
            prop_reads(right, reads);
        }
        Prop::Not { arg } => prop_reads(arg, reads),
        Prop::Bool { expr } => expr_reads(expr, reads),
        other => {
            if let Some((_, comparison)) = other.comparison() {
                expr_reads(&comparison.left, reads);
                expr_reads(&comparison.right, reads);
            }
        }
    }
}
