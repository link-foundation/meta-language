//! Resolve source storage metadata against the actual parser slot and tokens.
use super::{Result, RustParser, SourceOrigin, Span, TokenKind, unsupported};
use crate::translation::{
    rust_owned_record_loop_rules as rules,
    source_loop_folds::{RustSourceFoldSlot, RustSourceStateTransition, SourceStateBinder},
    source_signatures::SourceTypeSignature,
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

#[cfg(test)]
mod tests {
    use super::super::{SourceProfiles, parse_rust_with_selected_profiles};
    use crate::translation::check::check_program;

    const SOURCE: &str =
        "fn main() { let mut number = 1u16; number = 2u16; println!(\"{}\", number); }";

    #[test]
    fn explicit_region_composition_retains_whole_storage_write_authority() {
        let profiles = SourceProfiles {
            source_version: "1.90.0",
            source_signature: "Rust source function signature 3",
            exclusive_source: "Rust exclusive source loans 1",
            fn_mut_source: "Rust closed source FnMut specializations 1",
            ..SourceProfiles::default()
        };
        let surface = parse_rust_with_selected_profiles(SOURCE, profiles).unwrap();
        let checked = check_program(&surface).unwrap();
        let wire = serde_json::to_string(&checked).unwrap();
        assert!(wire.contains("\"operation\":\"initialize\""));
        assert!(wire.contains("\"operation\":\"write\""));
        let separate = SourceProfiles {
            fn_mut_source: "",
            ..profiles
        };
        assert!(parse_rust_with_selected_profiles(SOURCE, separate).is_err());
    }

    #[test]
    fn owned_text_constructors_publish_closed_string_revisions() {
        let source = r#"fn main() {
            let mut text: String = String::from("discarded");
            text.clear();
            text.push_str("kept");
            println!("{}", text);
        }"#;
        let profiles = SourceProfiles {
            source_version: "1.90.0",
            source_signature: "Rust source function signature 3",
            exclusive_source: "Rust exclusive source loans 1",
            fn_mut_source: "Rust closed source FnMut specializations 1",
            ..SourceProfiles::default()
        };
        let surface = parse_rust_with_selected_profiles(source, profiles).unwrap();
        let checked = check_program(&surface).unwrap();
        let wire = serde_json::to_string(&checked).unwrap();
        assert_eq!(wire.matches("\"operation\":\"write\"").count(), 2);
        assert!(wire.contains("\"op\":\"concat\""));
    }
}
