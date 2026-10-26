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
        let argument = self.origins.origin(&index, path);
        let element = match receiver.ty.as_ref() {
            Some(Type::Array { element, .. }) => Some(element.as_ref()),
            _ => None,
        };
        let error = read_rust_scalar_array_index_receiver_error(
            receiver.ty.as_ref().map_or("", Type::kind),
            element.map_or("", Type::kind),
            receiver.array_source_model.unwrap_or(""),
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
        let owner = argument
            .source_owner
            .as_ref()
            .filter(|owner| owner.kind == "primitive")
            .map_or_else(String::new, |owner| owner.path.join("::"));
        let (bits, signed) = match argument.ty.as_ref() {
            Some(Type::Fixed { bits, signed }) => (f64::from(*bits), *signed),
            _ => (0.0, false),
        };
        let literal = matches!(
            &index.node,
            SNode::Num {
                ty: None,
                negative: false,
                ..
            }
        );
        let error = read_rust_scalar_array_index_operand_error(
            literal,
            argument.ty.as_ref().map_or("", Type::kind),
            bits,
            signed,
            &owner,
            serde_json::Number::from(argument.wrappers.len())
                .as_f64()
                .expect("actual source wrapper count"),
            argument.proven,
        );
        if !error.is_empty() {
            return Err(unsupported(
                "Rust scalar array index",
                &error,
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
                        from: literal.then_some(Type::Fixed {
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
        let origin = element.map_or_else(
            || SourceOrigin {
                copy_evidence: Some(()),
                ..SourceOrigin::value()
            },
            |element| SourceOrigin::with_type(element.clone(), path, Vec::new()),
        );
        self.origins.mark(&result, origin);
        Ok(result)
    }
}
