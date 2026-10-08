//! Bind checked array method forms to the existing surface array representation.
use super::{JavaScriptParser, Result, SArrayItem, SExpr, SNode, Span, node, unsupported};

impl JavaScriptParser {
    pub(super) fn array_method_expression(
        form: &str,
        args: Vec<SExpr>,
        receiver: Option<SExpr>,
        place: Span,
    ) -> Result<SExpr> {
        if !crate::translation::frontend_rules::accept_array_method_arguments(
            form,
            !args.is_empty(),
            args.len() == 1,
        ) {
            return Err(unsupported(
                "array method arguments",
                "array copying takes an array with no index or mapping arguments",
                Some(place),
            ));
        }
        let items = receiver
            .into_iter()
            .chain(args)
            .map(|value| SArrayItem {
                spread: form != "construct",
                value,
            })
            .collect();
        Ok(node(
            SNode::Array {
                items,
                element: None,
            },
            place,
        ))
    }
}
