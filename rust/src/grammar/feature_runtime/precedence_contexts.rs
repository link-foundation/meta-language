//! Grammar projection for the automatically translated precedence decisions.

use super::program::{Expr, Program, Target};

/// Whole-rule contexts, in the program's rule order, for one source construct.
pub(super) fn complete_rule_contexts(program: &Program, name: usize) -> Vec<bool> {
    let mut parents = vec![Vec::new(); program.rules.len()];
    for (owner, rule) in program.rules.iter().enumerate() {
        let mut pending = vec![&rule.expression];
        while let Some(expression) = pending.pop() {
            let kind = match expression {
                Expr::Ref(_) => "ref",
                Expr::Choice { .. } => "choice",
                Expr::Alias { .. } => "alias",
                Expr::Capture { .. } => "capture",
                Expr::LexicalScope { .. } => "lexicalScope",
                Expr::Precedence { .. } => "precedence",
                Expr::DynamicPrecedence { .. } => "dynamicPrecedence",
                Expr::Repeat {
                    min: 0,
                    max: Some(1),
                    ..
                } => "optional",
                _ => "",
            };
            let form =
                crate::translation::frontend_rules::read_unit_alternative_expression_form(kind);
            match (form, expression) {
                (1.0, Expr::Ref(Target::Rule(index))) => parents[*index].push(owner),
                (2.0, Expr::Choice { items, .. }) => pending.extend(items.iter()),
                (
                    3.0,
                    Expr::Alias { item, .. }
                    | Expr::LexicalScope { item, .. }
                    | Expr::Capture { item, .. }
                    | Expr::Precedence { item, .. }
                    | Expr::DynamicPrecedence { item, .. }
                    | Expr::Repeat { item, .. },
                ) => pending.push(item),
                _ => {}
            }
        }
    }
    let mut reached = vec![false; program.rules.len()];
    let mut pending = vec![name];
    while let Some(current) = pending.pop() {
        if reached[current] {
            continue;
        }
        reached[current] = true;
        pending.extend(parents[current].iter().copied());
    }
    reached
}
