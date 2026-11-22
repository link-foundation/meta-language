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

    #[test]
    fn projected_string_replacements_preserve_disjoint_sibling_loans() {
        let source = r#"struct Message { body: String, tag: u8 }
            struct Envelope { message: Message, id: u8 }
            fn refs<'a>(tag: &'a u8) -> Vec<&'a u8> { vec![tag] }
            fn main() {
                let mut envelope = Envelope {
                    message: Message { body: String::from("discarded"), tag: 7u8 },
                    id: 8u8,
                };
                let tags = refs(&envelope.message.tag);
                envelope.message.body.clear();
                envelope.message.body.push_str("kept");
                println!("{}", envelope.message.body);
                println!("{}", tags.len() == 1 && envelope.id == 8u8);
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
        assert_eq!(wire.matches("\"operation\":\"clear string\"").count(), 1);
        assert_eq!(
            wire.matches("\"operation\":\"append string text\"").count(),
            1
        );
        let edited = wire.replacen(
            "\"operation\":\"append string text\"",
            "\"operation\":\"clear string\"",
            1,
        );
        assert_ne!(wire, edited);
        crate::translation::source_expression_regions::validate_borrowed_vector_source_regions(
            &serde_json::from_str(&edited).unwrap(),
        )
        .expect_err("changing the retained intrinsic cannot change the meaning of the actual term");
    }

    #[test]
    fn projected_vector_append_keeps_the_actual_selected_field_and_element() {
        let source = r#"struct Buffer { values: Vec<u8>, tag: u8 }
            fn main() {
                let mut buffer = Buffer { values: vec![1u8], tag: 3u8 };
                let value = 7u8;
                buffer.values.push(value);
                println!("{}", buffer.values.len() == 2 && buffer.tag == 3u8);
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
        assert_eq!(
            wire.matches("\"operation\":\"append vector element\"")
                .count(),
            1
        );
        let edited = wire.replacen(
            "\"operation\":\"append vector element\"",
            "\"operation\":\"append string text\"",
            1,
        );
        crate::translation::source_expression_regions::validate_borrowed_vector_source_regions(
            &serde_json::from_str(&edited).unwrap(),
        )
        .expect_err("the actual array and element cannot be interpreted as String operands");
    }

    #[test]
    fn selected_record_field_moves_leave_its_actual_sibling_available() {
        let source = r#"struct Pair { first: String, second: String }
            fn main() {
                let pair = Box::new(Pair {
                    first: String::from("first"),
                    second: String::from("second"),
                });
                let first = pair.first;
                println!("{}", pair.second);
                println!("{}", first);
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
        assert_eq!(
            wire.matches("\"model\":\"Rust source record place 1\"")
                .count(),
            2
        );
        let edited = wire.replacen("\"field\":0,\"value\":", "\"field\":1,\"value\":", 1);
        assert_ne!(wire, edited);
        let edited = serde_json::from_str(&edited).unwrap();
        let error =
            crate::translation::source_expression_regions::validate_borrowed_vector_source_regions(
                &edited,
            )
            .expect_err("the actual bound field must agree with the retained ordinal");
        assert!(error.reason.contains("actual one-constructor projection"));
    }

    #[test]
    fn field_writeback_reinitializes_only_its_selected_moved_subtree() {
        let source = r#"struct Pair { first: String, second: u8 }
            fn refs<'a>(referent: &'a u8) -> Vec<&'a u8> { vec![referent] }
            fn main() {
                let mut pair = Pair { first: String::from("before"), second: 7u8 };
                let view = refs(&pair.second);
                let first = pair.first;
                pair.first = String::from("after");
                println!("{}", view.len() == 1 && pair.second == 7u8);
                println!("{}", pair.first);
                println!("{}", first);
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
        assert_eq!(
            wire.matches("\"model\":\"Rust source place replacement 1\"")
                .count(),
            1
        );
        let edited = wire.replacen(
            "\"places\":[{\"slot\":0,\"fields\":[0]",
            "\"places\":[{\"slot\":0,\"fields\":[1]",
            1,
        );
        assert_ne!(wire, edited);
        let edited = serde_json::from_str(&edited).unwrap();
        crate::translation::source_expression_regions::validate_borrowed_vector_source_regions(
            &edited,
        )
        .expect_err("same-shaped writeback metadata cannot redirect the selected source field");
    }

    #[test]
    fn finite_loop_helpers_publish_only_their_actual_storage_revisions() {
        let source = r#"struct State { count: u8, text: String }
            fn refs<'a>(referent: &'a u8) -> Vec<&'a u8> { vec![referent] }
            fn main() {
                let mut state = State { count: 2u8, text: String::from("kept") };
                while state.count > 0u8 { state.count = state.count - 1u8; }
                let referent = 3u8;
                let view = refs(&referent);
                println!("{}", state.count == 0u8 && view.len() == 1);
            }"#;
        let profiles = SourceProfiles {
            source_version: "1.90.0",
            source_signature: "Rust source function signature 3",
            exclusive_source: "Rust exclusive source loans 1",
            fn_mut_source: "Rust closed source FnMut specializations 1",
            ..SourceProfiles::default()
        };
        let surface = parse_rust_with_selected_profiles(source, profiles).unwrap();
        let mut checked = check_program(&surface).unwrap();
        let transforms: Vec<_> = checked
            .declarations
            .iter_mut()
            .filter_map(|declaration| match declaration {
                crate::translation::ir::Decl::Fn(function) => {
                    function.source_state_transform.as_deref_mut()
                }
                _ => None,
            })
            .collect();
        assert_eq!(transforms.len(), 32);
        for transform in &transforms {
            assert!(transform.control.is_none());
            assert_eq!(
                &source[transform.span.start..transform.span.start + 5],
                "while"
            );
            let [slot] = transform.slots.as_slice() else {
                panic!("one actual state slot")
            };
            assert_eq!(&source[slot.binder.start..slot.binder.end], "state");
        }
        let mut transforms = transforms;
        transforms[0].slots[0]
            .binder
            .identity
            .push_str("_different");
        crate::translation::source_expression_regions::validate_borrowed_vector_source_regions(
            &checked,
        )
        .expect_err("an otherwise same-typed compiler slot must keep its actual storage identity");
    }
}
