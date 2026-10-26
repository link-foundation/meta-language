//! Thin owned source projection into the shared primitive vector-copy rules.
use super::{RustParser, SExpr, SNode, SourceOrigin, Span, Token, Type};
use crate::translation::rust_scalar_vector_copy_rules::{
    accept_rust_scalar_vector_copy_owner, accept_rust_scalar_vector_copy_query,
    accept_rust_scalar_vector_copy_receiver, read_rust_scalar_vector_copy_operation,
};
use crate::translation::source_signatures::SourceOwner;
impl RustParser {
    pub(super) fn read_scalar_vector_copy(
        &mut self,
        receiver: &SExpr,
        method: &Token,
        args: usize,
        range: Option<Span>,
        path: &[String],
    ) -> Option<SExpr> {
        let origin = self.origins.origin(receiver, path);
        let layers = u32::try_from(origin.wrappers.len()).map_or(f64::INFINITY, f64::from);
        if !accept_rust_scalar_vector_copy_query(
            self.ordinary_completion_context(),
            &method.value,
            origin.array_source_model.unwrap_or(""),
            layers,
        ) {
            return None;
        }
        let element = match &origin.ty {
            Some(Type::Array { element, .. }) => Some(element.as_ref()),
            _ => None,
        };
        if !accept_rust_scalar_vector_copy_receiver(
            origin.ty.as_ref().map_or("", Type::kind),
            element.map_or("", Type::kind),
            origin.proven,
            origin.wrappers.iter().all(|layer| layer == "reference"),
            origin.array_element_copy_evidence,
            u32::try_from(args).map_or(f64::INFINITY, f64::from),
        ) {
            return None;
        }
        Some(self.build_scalar_vector_copy(receiver, origin, range, path))
    }

    /// Project an admitted source copy without reinterpreting its trait call.
    pub(super) fn build_scalar_vector_copy(
        &mut self,
        receiver: &SExpr,
        origin: SourceOrigin,
        range: Option<Span>,
        path: &[String],
    ) -> SExpr {
        let owner = origin
            .source_owner
            .as_ref()
            .filter(|owner| {
                accept_rust_scalar_vector_copy_owner(
                    &owner.kind,
                    &owner.path.join("::"),
                    u32::try_from(owner.path.len()).map_or(f64::INFINITY, f64::from),
                    u32::try_from(owner.arguments.len()).map_or(f64::INFINITY, f64::from),
                )
            })
            .map(|owner| SourceOwner {
                kind: "library".into(),
                path: vec!["std".into(), "vec".into(), "Vec".into()],
                arguments: vec![owner.arguments[0].clone()],
            });
        let result = SExpr::new(
            SNode::Method {
                object: Box::new(receiver.clone()),
                op: read_rust_scalar_vector_copy_operation(),
                args: Vec::new(),
            },
            range,
        );
        let mut copied = origin;
        copied.path = path.to_vec();
        copied.wrappers.clear();
        copied.fields = None;
        copied.text_source_model = None;
        copied.array_source_model = Some("owned vector");
        copied.source_lifetimes.clear();
        copied.source_owner = owner;
        copied.expanded_source_owner = None;
        copied.fresh_array_value = range;
        copied.fresh_vector_result_contract = false;
        self.origins.mark_vector_constructor(&result, copied);
        result
    }
}
