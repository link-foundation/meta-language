//! Extract only the shared exact literal-message canonical panic form.
use super::super::super::completion_rules::accept_rust_completion_panic_arguments;
use super::{SExpr, SNode};

pub(super) fn read_rust_completion_panic_message(
    model: &str,
    macro_name: &str,
    arguments: &[SExpr],
) -> Option<String> {
    let [first, second] = arguments else {
        return None;
    };
    let SNode::Str { value: template } = &first.node else {
        return None;
    };
    let SNode::Str { value: message } = &second.node else {
        return None;
    };
    accept_rust_completion_panic_arguments(
        model,
        macro_name,
        first.node.kind(),
        template,
        second.node.kind(),
        "2",
    )
    .then(|| message.clone())
}
