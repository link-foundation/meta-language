//! DOCX (OOXML) source parser: a byte-exact lossless network plus
//! concept-tagged document-structure links.
//!
//! Parsing starts from [`LinkNetwork::parse_lossless_text`], so every byte of
//! the `word/document.xml` source is preserved as a `Token` leaf and
//! [`LinkNetwork::reconstruct_text`] returns the input verbatim. On top of those
//! leaves this module recovers the document structure of the
//! [OOXML text profile](crate::document_formatting) and interns it as additive
//! `Concept`/`Object` links: one shared concept point per role (`heading`,
//! `paragraph`, `bullet-list`, `list-item`, `strong`, `emphasis`, …) and one
//! structure `Object` link per occurrence, referencing both its parent
//! structure node and the concept it instantiates.
//!
//! The structure links are purely additive tags: they carry no spans, are never
//! read back by reconstruction, and only enrich the network with the formatting
//! concepts required by issue #85 for bold/italic/heading/paragraph/list.

use crate::document_formatting::{parse_docx_document, BlockNode, InlineNode};
use crate::{LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration};

/// Parses DOCX `word/document.xml` `text` into a lossless network enriched with
/// concept-tagged document-structure links.
pub fn parse(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    let mut network = LinkNetwork::parse_lossless_text(text, language, configuration);

    let document = parse_docx_document(text);
    if document.blocks.is_empty() {
        // Out-of-profile OOXML: keep the byte-exact lossless network untagged.
        return network;
    }

    let Some(root) = document_link(&network) else {
        return network;
    };

    for block in &document.blocks {
        annotate_block(&mut network, root, block, language);
    }

    network
}

/// Finds the document root inserted by [`LinkNetwork::parse_lossless_text`].
fn document_link(network: &LinkNetwork) -> Option<LinkId> {
    network
        .links()
        .find(|link| link.metadata().link_type() == Some(LinkType::Document))
        .map(crate::Link::id)
}

fn annotate_block(network: &mut LinkNetwork, parent: LinkId, block: &BlockNode, language: &str) {
    let structure = insert_structure(network, parent, block.concept_id(), language);
    match block {
        BlockNode::Heading { children, .. } | BlockNode::Paragraph { children } => {
            annotate_inline(network, structure, children, language);
        }
        BlockNode::List { items, .. } => {
            for item in items {
                let list_item = insert_structure(network, structure, "list-item", language);
                annotate_inline(network, list_item, item, language);
            }
        }
    }
}

fn annotate_inline(
    network: &mut LinkNetwork,
    parent: LinkId,
    nodes: &[InlineNode],
    language: &str,
) {
    for node in nodes {
        if let InlineNode::Wrapped {
            concept, children, ..
        } = node
        {
            let structure = insert_structure(network, parent, concept, language);
            annotate_inline(network, structure, children, language);
        }
    }
}

/// Interns the shared concept point for `concept` and inserts a structure
/// `Object` link referencing both its `parent` node and that concept point.
fn insert_structure(
    network: &mut LinkNetwork,
    parent: LinkId,
    concept: &str,
    language: &str,
) -> LinkId {
    let concept_point = network.insert_typed_point(concept, LinkType::Concept, None);
    network.insert_link(
        [parent, concept_point],
        LinkMetadata::new()
            .with_link_type(LinkType::Object)
            .with_named(true)
            .with_term(concept)
            .with_language(language),
    )
}
