#[cfg(test)]
mod deferred_position_controls {
    use super::*;
    use crate::translation::source_signatures::{
        SourceOwner, SourceSignature, validate_source_signature,
    };
    use crate::translation::types::Type;
    fn signature(model: &str) -> SourceSignature {
        let scalar = Type::Fixed {
            bits: 8,
            signed: false,
        };
        let owner = SourceOwner {
            kind: "primitive".into(),
            path: vec!["u8".into()],
            arguments: vec![],
        };
        let child = SourceTypeSignature {
            ty: scalar.clone(),
            wrappers: vec!["reference".into()],
            lifetimes: vec![Some("static".into())],
            owner: owner.clone(),
            array_element_position: None,
        };
        SourceSignature {
            model: model.into(),
            language: "Rust".into(),
            parameters: vec![],
            result: SourceTypeSignature {
                ty: Type::Array {
                    element: Box::new(scalar),
                    reference: false,
                },
                wrappers: vec![],
                lifetimes: vec![],
                owner: SourceOwner {
                    kind: "library".into(),
                    path: vec!["std".into(), "vec".into(), "Vec".into()],
                    arguments: vec![owner],
                },
                array_element_position: Some(Box::new(child)),
            },
            lifetime_bindings: vec![],
        }
    }
    #[test]
    fn only_the_new_source_profile_accepts_its_complete_child_position() {
        let actual = signature("Rust source function signature 3");
        assert!(validate_source_signature(&actual, &[], &actual.result.ty, "Rust").is_ok());
        for model in [
            "Rust source function signature 1",
            "Rust source function signature 2",
        ] {
            let previous = signature(model);
            assert!(
                validate_source_signature(&previous, &[], &previous.result.ty, "Rust").is_err()
            );
        }
    }
    #[test]
    fn source_child_owners_are_not_inferred_from_the_erased_element() {
        let mut actual = signature("Rust source function signature 3");
        actual
            .result
            .array_element_position
            .as_mut()
            .unwrap()
            .owner
            .path = vec!["u16".into()];
        assert!(validate_source_signature(&actual, &[], &actual.result.ty, "Rust").is_err());
    }
}
