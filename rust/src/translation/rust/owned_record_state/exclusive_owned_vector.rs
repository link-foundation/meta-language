//! Lower Vec push through the actual mutable place and its ordered operand.
use super::super::{SArrayItem, Token, owned_vector_iteration::item_origin};
use super::{CollectionPlace, Result, RustParser, SExpr, SNode, Span, Type, fail, rules};
use crate::translation::rust_exclusive_source_vector_rules as vector_rules;

impl RustParser {
    pub(in super::super) fn read_exclusive_owned_vector_append(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        if !self.exclusive_source_enabled || method.value != "push" {
            return Ok(None);
        }
        let actual = self.origins.origin(receiver, path);
        let Some(Type::Array {
            element,
            reference: false,
        }) = actual.ty.as_ref()
        else {
            return Ok(None);
        };
        let span = receiver.span;
        let owner = actual
            .expanded_source_owner
            .as_ref()
            .or(actual.source_owner.as_ref());
        let selected = owner.and_then(|owner| {
            vector_rules::accept_rust_exclusive_source_vector_receiver(
                "array",
                actual.array_source_model.unwrap_or(""),
                actual.wrappers.len() as f64,
                actual.proven || self.origins.collecting,
                &owner.kind,
                &owner.path.join("::"),
                owner.arguments.len() as f64,
            )
            .then(|| &owner.arguments[0])
        });
        let Some(owner) = selected else {
            return Err(fail(
                "Vec push needs its actual direct standard owned Vec receiver",
                span,
            ));
        };
        let place = super::super::owned_record_loops::place(self, receiver)
            .ok_or_else(|| fail("Vec push needs its actual mutable lexical place", span))?;
        let slot = place.slot.clone();
        let field = place.origin.clone();
        let layouts = self.record_state_layout_indices(&slot.origin, &place.indices, span)?;
        let indices = place
            .indices
            .iter()
            .map(|index| *index as f64)
            .collect::<Vec<_>>();
        let target = rules::read_rust_owned_record_state_field_identity(&slot.root, &indices);
        let mut expected = item_origin(
            self.record_source_type(element, &actual.path),
            owner,
            path,
            span,
            0,
        )?;
        let ownership = self.origins.standard_sum_ownership_facts(&expected);
        if !self.origins.collecting && !matches!(ownership.profile.as_str(), "copy" | "owned") {
            return Err(fail(
                "Vec push needs an actual closed destructor-free element position",
                span,
            ));
        }
        expected.copy_evidence = (ownership.profile == "copy").then_some(());
        self.cursor.expect("(", Some("Vec push"))?;
        let temporaries = self.origins.owned_temporary_frame();
        let previous = self
            .standard_sum_source
            .expected_origin
            .replace(expected.clone());
        let operand = self.read_expected_copy_closure(path, Some(&expected));
        self.standard_sum_source.expected_origin = previous;
        let operand = operand?;
        self.cursor.eat(",");
        let end = self.cursor.expect(")", Some("Vec push"))?;
        let span = Some(Span::new(
            span.map_or(method.start, |value| value.start),
            end.end,
        ));
        let mut supplied = self.origins.origin(&operand, path);
        let transferred = self
            .origins
            .owned_standard_sum_argument_transferred(&operand);
        self.origins
            .argument_with_transfer(&supplied, &expected, operand.span, transferred)?;
        if !self.origins.collecting {
            if ownership.profile == "owned" {
                supplied = self.owned_standard_sum_method_capture(&operand, path)?;
                self.origins
                    .consume_owned_binding(&supplied, operand.span)?;
            }
            if slot.callable != self.owned_record_state.current_callable
                || self.record_state_overlaps(&slot, &target)
                || super::super::owned_record_updates::field_state(
                    &self.origins.snapshot_owned_bindings(),
                    &target,
                ) != "available"
            {
                return Err(fail(
                    "Vec push needs an available place with no overlapping loan",
                    span,
                ));
            }
        }
        let result = if !layouts.is_empty() && slot.source_binder.is_some() {
            self.owned_record_effect_vector_append(
                CollectionPlace {
                    slot,
                    layouts,
                    target,
                },
                field,
                operand,
                expected,
                path,
                span,
            )?
        } else {
            let held = self.fresh_product_binding();
            let prior = self.owned_record_effect_read(slot.clone(), span)?;
            let current = self.project_record_state(&layouts, prior, span);
            let mut append = SExpr::new(
                SNode::Array {
                    items: vec![
                        SArrayItem::new(current, true),
                        SArrayItem::new(super::name(&held), false),
                    ],
                    element: None,
                },
                span,
            );
            let mut result = super::SourceOrigin::with_type(
                actual.ty.expect("actual Vec type"),
                path,
                Vec::new(),
            );
            result.source_owner = actual.source_owner;
            result.expanded_source_owner = actual.expanded_source_owner;
            self.origins.mark(&append, result);
            self.note_borrowed_expression_source_position(&mut append, path)?;
            let value = SExpr::new(
                SNode::Let {
                    name: held,
                    ty: expected.ty.map(Into::into),
                    value: Box::new(operand),
                    body: Box::new(append),
                },
                span,
            );
            self.owned_record_effect_write(slot, layouts, field, target, value, path, span)?
        };
        self.origins
            .finish_owned_expression(&result, path, &temporaries, false);
        Ok(Some(result))
    }
}
