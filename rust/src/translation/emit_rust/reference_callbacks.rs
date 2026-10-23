//! Reachability transport for the shared read-only callback decisions.
use super::{Decl, Expr, Node, RustEmitter};
use std::collections::BTreeSet;

impl RustEmitter<'_> {
    pub(super) fn read_only_reference_callback(&self, callback: &Expr) -> bool {
        if !matches!(
            callback.node,
            Node::Lambda { .. } | Node::FunctionRef { .. }
        ) {
            return false;
        }
        let mut pending = vec![callback];
        let mut seen = BTreeSet::new();
        let mut functions = BTreeSet::new();
        while let Some(node) = pending.pop() {
            if !seen.insert(std::ptr::from_ref(node)) {
                continue;
            }
            if seen.len() > 100_000 {
                return false;
            }
            let operation = if let Node::Method { op, .. } = &node.node {
                op.as_str()
            } else {
                ""
            };
            let role = crate::translation::reference_method_rules::read_reference_callback_effect(
                node.kind(),
                operation,
            );
            if role == 1.0 {
                return false;
            }
            if role == 2.0 {
                let name = match &node.node {
                    Node::Call { func, .. } | Node::FunctionRef { func } => func,
                    _ => return false,
                };
                if functions.insert(name.clone()) {
                    let Some(Decl::Fn(function)) = self
                        .program
                        .declarations
                        .iter()
                        .find(|entry| entry.full_name() == name)
                    else {
                        return false;
                    };
                    pending.push(&function.body);
                }
            }
            if role == 3.0
                && let Node::Apply { func, .. } = &node.node
                && !matches!(func.node, Node::Lambda { .. } | Node::FunctionRef { .. })
            {
                return false;
            }
            pending.extend(node.children());
        }
        true
    }
}
