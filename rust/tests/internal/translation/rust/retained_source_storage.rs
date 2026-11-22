#[cfg(test)]
mod tests {
    use super::super::{SourceProfiles, parse_rust_with_selected_profiles};
    use crate::translation::check::check_program;

    const SOURCE: &str =
        "fn main() { let mut number = 1u16; number = 2u16; println!(\"{}\", number); }";

    #[test]
    fn composed_source_checks_the_actual_fn_mut_selector_and_version() {
        let profiles = SourceProfiles {
            source_version: "1.90.0",
            source_signature: "Rust source function signature 3",
            exclusive_source: "Rust exclusive source loans 1",
            fn_mut_source: "Rust closed source FnMut specializations 1",
            ..SourceProfiles::default()
        };
        for version in ["1.89.0", "1.100.0"] {
            let error = parse_rust_with_selected_profiles(
                "fn main() {}",
                SourceProfiles {
                    source_version: version,
                    ..profiles
                },
            )
            .unwrap_err();
            assert!(error.reason.contains("from 1.90.0 through 1.99.x"));
        }
        for (exclusive_source, fn_mut_source, diagnostic) in [
            (
                "",
                profiles.fn_mut_source,
                "actual exclusive source state profile",
            ),
            (
                profiles.exclusive_source,
                "unknown profile",
                "actual closed source FnMut specialization model",
            ),
        ] {
            let error = parse_rust_with_selected_profiles(
                "fn main() {}",
                SourceProfiles {
                    exclusive_source,
                    fn_mut_source,
                    ..profiles
                },
            )
            .unwrap_err();
            assert!(error.reason.contains(diagnostic));
        }
    }

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
    fn boxed_string_replacement_retains_its_actual_payload_position() {
        let source = r#"fn main() {
            let mut text = Box::new(String::from("discarded"));
            text.clear();
            text.push_str("kept");
            println!("{}", *text);
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
        assert_eq!(wire.matches("\"slot\":0,\"fields\":[]").count(), 2);
    }

    #[test]
    fn direct_vector_mutations_keep_the_current_source_revision() {
        let source = r#"fn main() {
            let mut values = vec![1u8, 2u8, 3u8];
            values.truncate(2usize);
            let last: Option<u8> = values.pop();
            let number = match last { Some(value) => value, None => 0u8 };
            values.clear();
            println!("{} {}", number, values.len());
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
        for operation in ["truncate", "pop", "clear"] {
            assert_eq!(
                wire.matches(&format!(r#""operation":"{operation}""#))
                    .count(),
                1
            );
        }
    }

    #[test]
    fn projected_vector_mutations_keep_the_slot_and_unchanged_sibling() {
        let source = r#"struct Buffer { values: Box<Vec<u8>>, tag: u8 }
            fn main() {
                let mut buffer = Buffer { values: Box::new(vec![1u8, 2u8]), tag: 3u8 };
                buffer.values.truncate(1usize);
                let last: Option<u8> = buffer.values.pop();
                buffer.values.clear();
                let number = match last { Some(value) => value, None => 0u8 };
                println!("{} {}", number, buffer.tag);
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
        let selected = r#""parameter":0,"slot":0,"fields":[0]"#;
        assert_eq!(wire.matches(selected).count(), 3);
        let forged = wire.replacen(selected, r#""parameter":0,"slot":0,"fields":[1]"#, 1);
        let forged: crate::translation::ir::Program = serde_json::from_str(&forged).unwrap();
        assert!(
            crate::translation::source_expression_regions::validate_borrowed_vector_source_regions(
                &forged
            )
            .is_err()
        );
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
