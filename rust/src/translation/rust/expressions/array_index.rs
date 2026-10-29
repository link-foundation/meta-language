//! Source/AST transport for the automatically translated scalar index policy.
use super::{
    Flavor, Options, Result, RustParser, SExpr, SNode, SourceOrigin, Span, Type, unsupported,
};
use crate::translation::rust_array_index_rules::{
    read_rust_scalar_array_index_operand_error, read_rust_scalar_array_index_receiver_error,
};
impl RustParser {
    #[allow(clippy::too_many_lines)] // One source read, with exact casts and retained source facts.
    pub(super) fn scalar_array_index(&mut self, object: SExpr, path: &[String]) -> Result<SExpr> {
        let open = self.cursor.advance();
        let index = self.expr(path, Options::default())?;
        let close = self.cursor.expect("]", Some("array index"))?;
        let range = Some(Span::new(
            object.span.map_or(open.start, |span| span.start),
            close.end,
        ));
        let receiver = self.origins.origin(&object, path);
        let element = match receiver.ty.as_ref() {
            Some(Type::Array { element, .. }) => Some(element.as_ref()),
            _ => None,
        };
        let error = read_rust_scalar_array_index_receiver_error(
            receiver.ty.as_ref().map_or("", Type::kind),
            element.map_or("", Type::kind),
            &crate::translation::rust_byte_string_expression_rules::read_rust_static_byte_array_read_model(&crate::translation::rust_byte_string_expression_rules::ml_string::Text::from_text(receiver.array_source_model.unwrap_or(""))).to_string(),
            receiver.proven,
            receiver
                .wrappers
                .iter()
                .all(|layer| layer == "reference" || layer == "box"),
            receiver.array_element_copy_evidence,
        );
        if !error.is_empty() {
            return Err(unsupported("Rust scalar array index", &error, range));
        }
        let expression_error = self.usize_index_expression(&index, path)?;
        if !expression_error.is_empty() {
            return Err(unsupported(
                "Rust scalar array index",
                &read_rust_scalar_array_index_operand_error(false, "", 0.0, false, "", 0.0, false),
                index.span.or(range),
            ));
        }
        let position_span = index.span;
        let position = SExpr::new(
            SNode::Cast {
                arg: Box::new(SExpr::new(
                    SNode::Cast {
                        arg: Box::new(index),
                        to: Type::Fixed {
                            bits: 64,
                            signed: false,
                        },
                        from: Some(Type::Fixed {
                            bits: 64,
                            signed: false,
                        }),
                        flavor: Flavor::Exact,
                    },
                    position_span,
                )),
                to: Type::Nat,
                from: None,
                flavor: Flavor::Exact,
            },
            position_span,
        );
        let result = SExpr::new(
            SNode::Index {
                object: Box::new(object),
                index: Box::new(position),
                optional: None,
                proven_in_bounds: None,
            },
            range,
        );
        let mut origin = element.map_or_else(
            || SourceOrigin {
                copy_evidence: Some(()),
                ..SourceOrigin::value()
            },
            |element| SourceOrigin::with_type(element.clone(), path, Vec::new()),
        );
        let owner = receiver.pointer_integer_owner();
        if !owner.is_empty() {
            origin.source_owner = Some(crate::translation::source_signatures::SourceOwner {
                kind: "primitive".to_owned(),
                path: vec![owner],
                arguments: Vec::new(),
            });
        }
        self.origins.mark(&result, origin);
        Ok(result)
    }
}
