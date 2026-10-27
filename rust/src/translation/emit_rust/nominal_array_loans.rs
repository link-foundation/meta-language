//! Actual checked owners and patterns into the shared source loan selection.
use super::{Expr, Node, Pattern, Result, RustEmitter, Type};
use crate::translation::nominal_array_loan_rules as rules;
use crate::translation::rust_record_rules::ml_string::Text;

fn coordinate(value: usize) -> f64 {
    f64::from(
        u32::try_from(value)
            .expect("actual checked constructor coordinates fit the source directory"),
    )
}
impl RustEmitter<'_> {
    pub(super) fn nominal_array_projection_loan(
        &self,
        expression: &Expr,
    ) -> Result<Option<String>> {
        let Node::Match { scrutinee, cases } = &expression.node else {
            return Ok(None);
        };
        let Type::Data { name } = &scrutinee.ty else {
            return Ok(None);
        };
        let entry = self.program.data(name);
        let Some(kase) = cases.first() else {
            return Ok(None);
        };
        let Some(ctor) = entry.ctors.first() else {
            return Ok(None);
        };
        let Pattern::Ctor {
            data,
            ctor: constructor,
            binds,
        } = &kase.pattern
        else {
            return Ok(None);
        };
        let binding = if let Node::Var { name } = &kase.body.node {
            name.as_str()
        } else {
            ""
        };
        let slot = binds
            .iter()
            .position(|candidate| candidate.as_deref() == Some(binding));
        let direct = matches!(kase.body.node, Node::Var { .. })
            && slot
                .and_then(|slot| ctor.fields.get(slot))
                .is_some_and(|field| {
                    matches!(&field.ty, Type::Array { .. }) && !field.ty.is_reference()
                });
        if !rules::accept_nominal_array_projection_loan(
            expression.ty.kind(),
            expression.ty.is_reference(),
            if scrutinee.var_name().is_some() {
                "var"
            } else {
                "other"
            },
            coordinate(entry.ctors.len()),
            coordinate(cases.len()),
            data == &entry.full_name && constructor == &ctor.name,
            coordinate(ctor.fields.len()),
            coordinate(binds.len()),
            slot.map_or(-1.0, coordinate),
            coordinate(
                binds
                    .iter()
                    .filter(|candidate| candidate.as_deref() == Some(binding))
                    .count(),
            ),
            direct,
        ) {
            return Ok(None);
        }
        let head = format!("crate::{}", self.state.ctor_ref(data, constructor, "::"));
        let names = ctor
            .fields
            .iter()
            .map(|field| Text::from_text(&field.name))
            .collect::<Vec<_>>();
        let bindings = binds
            .iter()
            .map(|binding| Text::from_text(binding.as_deref().unwrap_or("_")))
            .collect::<Vec<_>>();
        let pattern = crate::translation::rust_record_rules::render_rust_constructor_fields(
            &Text::from_text(self.program.source_language.as_str()),
            &Text::from_text(&head),
            &names,
            &bindings,
        );
        Ok(Some(rules::render_nominal_array_projection_loan(
            scrutinee
                .var_name()
                .expect("the shared rule selects a direct actual variable"),
            &pattern.to_string(),
            binding,
        )))
    }
}
