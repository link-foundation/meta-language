//! Proven standard Rust Unicode scalar source owners into a byte-count node.
use super::{
    Result, RustParser, SArrayItem, SExpr, SNode, SourceOrigin, Span, Type, rust_fixed_type,
    type_error, unsupported,
};
use crate::translation::rust_string_byte_length_rules as rules;
impl RustParser {
    pub(super) fn read_string_byte_length(
        &mut self,
        receiver: &SExpr,
        count: usize,
        span: Option<Span>,
        path: &[String],
    ) -> Result<Option<SExpr>> {
        let origin = self.origins.origin(receiver, path);
        if origin.array_source_model.is_some()
            || origin.ty != Some(Type::String)
            || !self.ordinary_completion_context()
        {
            return Ok(None);
        }
        if count != 0 {
            return Err(type_error(
                format!("len expects 0 arguments but got {count}"),
                span,
            ));
        }
        let owner = origin.source_owner.clone().unwrap_or_else(|| {
            self.named_source_type_owner(
                &[if origin.text_source_model == Some("string slice") {
                    "str"
                } else {
                    "String"
                }
                .to_owned()],
                Vec::new(),
            )
        });
        let standard = super::read_only_query_rules::accept_rust_read_only_query_owner(
            "string empty",
            &owner.kind,
            &owner.path.join("::"),
        );
        if !self.origins.collecting
            && !super::read_only_query_rules::accept_rust_string_empty_receiver(
                "string",
                origin.text_source_model.unwrap_or(""),
                origin.proven,
                origin.wrappers.iter().all(|layer| layer == "reference"),
                standard,
            )
        {
            return Err(unsupported(
                "Rust String byte length",
                "the receiver needs a proven standard Unicode scalar str or String owner",
                span,
            ));
        }
        let node = SExpr::new(
            SNode::Math {
                op: rules::read_rust_string_byte_length_operation(),
                name: "Rust String byte length".to_owned(),
                args: vec![SArrayItem {
                    value: receiver.clone(),
                    spread: false,
                }],
            },
            span,
        );
        self.origins.mark(
            &node,
            SourceOrigin::with_type(
                rust_fixed_type("usize").expect("existing 64-bit source width"),
                path,
                Vec::new(),
            ),
        );
        Ok(Some(node))
    }
}
