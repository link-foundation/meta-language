//! The conversion of the projected tree of a native grammar into a link
//! network, as the parent module converts a tree-sitter tree.

use super::{
    ConvertContext, insert_gap_token, is_rocq_language, rocq_identifier_term, span_for_range,
};
use crate::native_grammar_parser::NativeNode;
use crate::{LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType};

/// Converts the projected root of a native grammar below `parent`, with the
/// text outside the root retained as gap tokens beside it, as `convert_root`
/// converts a tree-sitter root.
pub(super) fn convert_native_root(
    network: &mut LinkNetwork,
    parent: LinkId,
    root: &NativeNode,
    context: ConvertContext<'_>,
) -> LinkId {
    insert_gap_token(network, parent, 0, root.start, context);
    let root_id = convert_native_node(network, parent, root, context);
    insert_gap_token(network, parent, root.end, context.source_len, context);
    root_id
}

/// Converts a projected native node as `convert_node_with` converts a
/// tree-sitter node, mirroring `convertGrammarNode` over the native adapter
/// of `js/src/native-grammar-parser.js`.
fn convert_native_node(
    network: &mut LinkNetwork,
    parent: LinkId,
    node: &NativeNode,
    context: ConvertContext<'_>,
) -> LinkId {
    let flags = native_flags(node);
    let start = node.start.min(context.source_len);
    let end = node.end.min(context.source_len);
    let node_id = network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(node.named)
            .with_term(&node.term)
            .with_language(context.language)
            .with_span(span_for_range(context.lines, start, end, context.offset))
            .with_flags(flags),
    );

    if node.children.is_empty() {
        if !node.is_missing && start < end {
            let span = span_for_range(context.lines, start, end, context.offset);
            // Rocq's grammar calls every identifier-like leaf `ident`; its text
            // is also a semantic `identifier` or `primitive_type` leaf.
            let owner = if is_rocq_language(context.language) && node.term == "ident" {
                network.insert_link(
                    [node_id],
                    LinkMetadata::new()
                        .with_link_type(LinkType::Syntax)
                        .with_named(node.named)
                        .with_term(rocq_identifier_term(&context.text[start..end]))
                        .with_language(context.language)
                        .with_span(span)
                        .with_flags(flags),
                )
            } else {
                node_id
            };
            let token = network.insert_link(
                [owner],
                LinkMetadata::new()
                    .with_link_type(LinkType::Token)
                    .with_named(node.named)
                    .with_term(&context.text[start..end])
                    .with_language(context.language)
                    .with_span(span)
                    .with_flags(flags),
            );
            if flags.is_extra() {
                network.attach_trivia(
                    node_id,
                    token,
                    span,
                    context.configuration.trivia_attachment_policy(),
                );
            }
        }
        return node_id;
    }

    let mut covered_until = node.start;
    for (child, field) in &node.children {
        if context.has_synthetic_suffix() && child.start >= context.source_len {
            break;
        }
        insert_gap_token(network, node_id, covered_until, child.start, context);
        let child_id = convert_native_node(network, node_id, child, context);
        if let Some(label) = field {
            network.insert_field(node_id, label, child_id);
        }
        covered_until = covered_until.max(child.end.min(context.source_len));
    }
    insert_gap_token(network, node_id, covered_until, node.end, context);
    node_id
}

pub(super) const fn native_flags(node: &NativeNode) -> LinkFlags {
    let mut flags = LinkFlags::clean();
    if node.is_error {
        flags = flags.with_error();
    }
    if node.has_error || node.is_error || node.is_missing {
        flags = flags.with_containing_error();
    }
    if node.is_missing {
        flags = flags.with_missing();
    }
    if node.is_extra {
        flags = flags.with_extra();
    }
    flags
}
