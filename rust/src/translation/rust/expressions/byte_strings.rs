//! Actual lexical bytes and source-owner transport over shared definitions.
use super::{
    Result, RustParser, SArrayItem, SExpr, SNode, SourceOrigin, Span, Token, Type, unsupported,
};
use crate::translation::rust_byte_string_expression_rules as rules;
use crate::translation::source_signatures::SourceOwner;
use rules::ml_string::Text;
impl RustParser {
    pub(super) fn byte_string_expression(
        &mut self,
        token: &Token,
        path: &[String],
    ) -> Result<SExpr> {
        let bytes = rules::read_rust_byte_string_expression_bytes(
            &Text::from_text(&token.value),
            &Text::from_text(&token.suffix),
        );
        let range = Some(token.span());
        if bytes.first() != Some(&1.0) {
            return Err(unsupported(
                "Rust byte string expression",
                &rules::read_rust_byte_string_expression_error(
                    bytes.first().copied().unwrap_or(-2.0),
                )
                .to_string(),
                range,
            ));
        }
        let element = Type::Fixed {
            bits: 8,
            signed: false,
        };
        let owner = SourceOwner {
            kind: "primitive".to_owned(),
            path: vec!["u8".to_owned()],
            arguments: Vec::new(),
        };
        let mut items = Vec::new();
        for byte in &bytes[1..] {
            let value = SExpr::new(
                SNode::Num {
                    value: byte.to_string(),
                    ty: Some(element.clone()),
                    negative: false,
                    unit: false,
                },
                range,
            );
            let mut origin = SourceOrigin::with_type(element.clone(), path, Vec::new());
            origin.source_owner = Some(owner.clone());
            self.origins.mark(&value, origin);
            items.push(SArrayItem::new(value, false));
        }
        let length = items.len();
        let result = SExpr::new(
            SNode::Array {
                items,
                element: Some(element.clone()),
            },
            range,
        );
        let mut origin = SourceOrigin::with_type(
            Type::Array {
                element: Box::new(element),
                reference: false,
            },
            path,
            vec!["reference".to_owned()],
        );
        origin.source_lifetimes = vec![Some("static".to_owned())];
        origin.array_source_model = Some("static byte array");
        origin.array_element_copy_evidence = true;
        origin.source_owner = Some(SourceOwner {
            kind: "array".to_owned(),
            path: vec![length.to_string()],
            arguments: vec![owner],
        });
        self.cursor.advance();
        self.origins.mark(&result, origin);
        Ok(result)
    }
    pub(super) fn byte_string_slice_view(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        count: usize,
        range: Option<Span>,
        path: &[String],
    ) -> Option<SExpr> {
        let mut origin = self.origins.origin(receiver, path);
        if !rules::accept_rust_static_byte_array_slice_view(
            &Text::from_text(&method.value),
            count as f64,
            &Text::from_text(origin.array_source_model.unwrap_or("")),
            origin.proven,
            origin.wrappers.len() as f64,
            origin.wrappers.iter().all(|layer| layer == "reference"),
        ) {
            return None;
        }
        origin = origin.static_byte_array_read_view()?;
        let result = SExpr::new(receiver.node.clone(), range);
        self.origins.mark(&result, origin);
        Some(result)
    }
}
