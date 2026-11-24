//! Resolve source storage metadata against the actual parser slot and tokens.
use super::{Result, RustParser, SExpr, SourceOrigin, Span, TokenKind, unsupported};
use crate::translation::{
    rust_owned_record_loop_rules as rules,
    source_loop_folds::{
        RustSourceFoldSlot, RustSourceStateTransition, SourceFoldSpan, SourceStateBinder,
    },
    source_record_places::{RustSourceRecordPlace, owner_view},
    source_signatures::{SourceOwner, SourceTypeSignature},
    source_state_transforms::{MODEL as FINITE_LOOP_MODEL, RustFiniteLoopStateTransform},
    types::Type,
};

fn fail(reason: &str, span: Option<Span>) -> super::TranslationError {
    unsupported("Rust source state transition", reason, span)
}

impl RustParser {
    pub(super) fn retain_source_region_state(&self) -> bool {
        self.source_regions_composed && !self.source_region_body_closed
    }

    pub(super) fn source_region_contains_borrowed_vector(origin: &SourceOrigin) -> Result<bool> {
        let mut pending = vec![origin];
        let mut inspected = 0;
        while let Some(origin) = pending.pop() {
            inspected += 1;
            if inspected > 65_536 {
                return Err(fail(
                    "the complete source position exceeds its bounded inspection graph",
                    None,
                ));
            }
            if origin.array_element_position.is_some() {
                return Ok(true);
            }
            pending.extend(origin.fields.iter().flatten());
            if let Some(sum) = &origin.standard_sum_source {
                pending.extend(&sum.arguments);
            }
        }
        Ok(false)
    }

    pub(super) fn read_source_region_body_closed(
        &self,
        parameters: &[SourceOrigin],
        result: &SourceOrigin,
    ) -> Result<bool> {
        if !self.source_regions_composed {
            return Ok(false);
        }
        if result
            .wrappers
            .first()
            .is_some_and(|wrapper| wrapper == "reference")
            || Self::source_region_contains_borrowed_vector(result)?
        {
            return Ok(false);
        }
        for origin in parameters {
            if Self::source_region_contains_borrowed_vector(origin)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub(super) fn retain_source_state_binder(
        &self,
        name: &str,
        identity: &str,
        span: Span,
    ) -> Result<SourceStateBinder> {
        let tokens = &self.source_constant_tokens;
        let first = tokens.partition_point(|token| token.start < span.start);
        if !tokens.get(first).is_some_and(|token| {
            token.kind == TokenKind::Identifier && token.value == name && token.span() == span
        }) || !rules::accept_rust_source_state_binder(
            name.encode_utf16().count() as f64,
            identity.encode_utf16().count() as f64,
            span.start as f64,
            span.end as f64,
        ) {
            return Err(fail(
                "a state transition needs its original mutable binder token",
                Some(span),
            ));
        }
        Ok(SourceStateBinder {
            name: name.to_owned(),
            identity: identity.to_owned(),
            start: span.start,
            end: span.end,
        })
    }

    pub(super) fn retain_source_region_position(
        &self,
        origin: &SourceOrigin,
        span: Option<Span>,
    ) -> Result<SourceTypeSignature> {
        let mut position = super::borrowed_vectors::source_type_position(origin)?;
        if let Some(owner) = &origin.expanded_source_owner {
            position.owner = owner.clone();
        }
        self.origins
            .standard_sum_qualified_position(&position, &origin.path, 0)
            .map(|position| *position)
            .ok_or_else(|| fail("a retained place needs its actual closed source type", span))
    }

    pub(super) fn retain_record_field_place(
        &self,
        node: &mut SExpr,
        parent: &SourceOrigin,
        field: &SourceOrigin,
        index: usize,
    ) -> Result<()> {
        if field.array_element_position.is_some() {
            return Ok(());
        }
        let span = node
            .span
            .ok_or_else(|| fail("a retained field needs its actual source span", None))?;
        let root = self.retain_source_region_position(parent, Some(span))?;
        let value = self.retain_source_region_position(field, Some(span))?;
        if owner_view(&root, true).is_some() && owner_view(&value, false).is_some() {
            node.source_record_place = Some(Box::new(RustSourceRecordPlace {
                model: "Rust source record place 1".into(),
                root,
                field: index,
                value,
                span: SourceFoldSpan {
                    start: span.start,
                    end: span.end,
                },
            }));
        }
        Ok(())
    }

    pub(super) fn retain_source_fold_slot(
        &self,
        slot: &super::owned_record_state::Slot,
    ) -> Result<RustSourceFoldSlot> {
        let binder = slot
            .source_binder
            .as_ref()
            .ok_or_else(|| fail("a fold slot needs its original mutable binder token", None))?;
        let span = Span::new(binder.start, binder.end);
        if binder.identity != slot.lexical
            || !self
                .owned_record_state
                .declarations
                .get(&slot.current_name)
                .is_some_and(|actual| {
                    actual.lexical == slot.lexical
                        && actual.root == slot.root
                        && actual.source_binder.as_ref() == Some(binder)
                })
        {
            return Err(fail(
                "a state transition needs its actual admitted mutable lexical slot",
                Some(span),
            ));
        }
        let binder = self.retain_source_state_binder(&binder.name, &binder.identity, span)?;
        let position = self.retain_source_region_position(&slot.origin, Some(span))?;
        Ok(RustSourceFoldSlot { binder, position })
    }

    pub(super) fn retain_finite_loop_state_transform(
        &self,
        slots: &[super::owned_record_state::Slot],
        control_owner: Option<&[String]>,
        path: &[String],
        span: Span,
    ) -> Result<Option<RustFiniteLoopStateTransform>> {
        if !self.retain_source_region_state() {
            return Ok(None);
        }
        let mut unit = SourceOrigin::with_type(super::UNIT, path, Vec::new());
        unit.source_owner = Some(SourceOwner {
            kind: "unit".into(),
            path: Vec::new(),
            arguments: Vec::new(),
        });
        let source_result = if control_owner.is_some() {
            self.standard_sum_source
                .return_origin
                .as_ref()
                .ok_or_else(|| {
                    fail(
                        "a returning loop needs its actual enclosing source result",
                        Some(span),
                    )
                })?
        } else {
            &unit
        };
        let slots = slots
            .iter()
            .map(|slot| self.retain_source_fold_slot(slot))
            .collect::<Result<_>>()?;
        let control = control_owner.map(|owner| Type::Named {
            path: std::iter::once("crate".to_owned())
                .chain(owner.iter().cloned())
                .collect(),
            span: None,
        });
        Ok(Some(RustFiniteLoopStateTransform {
            model: FINITE_LOOP_MODEL.into(),
            slots,
            control,
            result: self.retain_source_region_position(source_result, Some(span))?,
            span: SourceFoldSpan {
                start: span.start,
                end: span.end,
            },
        }))
    }

    pub(super) fn retain_source_state_transition(
        &self,
        slot: &super::owned_record_state::Slot,
        operation: &str,
    ) -> Result<Option<RustSourceStateTransition>> {
        if slot.source_binder.is_none() {
            return Ok(None);
        }
        let RustSourceFoldSlot { binder, position } = self.retain_source_fold_slot(slot)?;
        Ok(Some(RustSourceStateTransition {
            model: "Rust source state transition 1".into(),
            operation: operation.into(),
            binder,
            position,
        }))
    }
}

include!("../../../tests/internal/translation/rust/retained_source_storage.rs");
