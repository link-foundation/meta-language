#[cfg(test)]
mod deferred_controls {
    use super::*;
    use crate::translation::source_signatures::SourceOwner;
    fn parser() -> RustParser {
        RustParser::new(
            crate::translation::lexer::tokenize("", crate::translation::Language::Rust)
                .unwrap()
                .tokens,
            super::super::SourceOrigins::new(None, ""),
            super::super::SourceProfiles::default(),
            false,
            None,
        )
    }
    fn primitive(owner: &str) -> SourceOrigin {
        let mut origin = SourceOrigin::with_type(
            super::super::rust_fixed_type(owner).unwrap(),
            &[],
            Vec::new(),
        );
        origin.source_owner = Some(SourceOwner {
            kind: "primitive".into(),
            path: vec![owner.into()],
            arguments: Vec::new(),
        });
        origin
    }
    #[test]
    fn primitive_conversion_certificates_require_the_complete_typed_identity() {
        let parser = parser();
        let mut origin = primitive("u8");
        assert_eq!(
            parser.standard_sum_primitive_question_error(&origin),
            Some("u8".into())
        );
        origin.fields = Some(vec![SourceOrigin::with_type(Type::Bool, &[], Vec::new())]);
        assert!(
            parser
                .standard_sum_primitive_question_error(&origin)
                .is_none()
        );
        let mut wrong = primitive("u8");
        wrong.source_owner.as_mut().unwrap().path = vec!["u16".into()];
        assert!(
            parser
                .standard_sum_primitive_question_error(&wrong)
                .is_none()
        );
    }
    #[test]
    fn no_span_from_projection_never_replaces_the_successful_payload_span_origin() {
        let mut parser = parser();
        let payload = name(vec!["payload".into()], Some(super::super::Span::new(1, 4)));
        parser.origins.mark(&payload, primitive("u32"));
        parser
            .origins
            .scope
            .insert("residual".into(), primitive("u8"));
        let value = parser
            .project_primitive_from(name(vec!["residual".into()], None), "u16", None, &[])
            .unwrap();
        parser.origins.scope.remove("residual");
        assert!(value.span.is_none());
        assert_eq!(
            parser.origins.origin(&payload, &[]).ty,
            super::super::rust_fixed_type("u32")
        );
    }
}
