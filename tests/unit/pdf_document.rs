//! PDF document-format support: text-profile render/parse round-trips, the
//! cross-format concept bridge, concept-tagged parsing, and `reconstruct_text_as`.

use meta_language::{
    parse_markup_document, parse_pdf_document, pdf_profile_is_recognized, render_pdf_document,
    BlockNode, InlineNode, LinkNetwork, LinkType, ParseConfiguration,
};

const SAMPLE_MARKDOWN: &str =
    "# Status Report\n\nThe system is **ready** for *launch*.\n\n- First item\n- Second **strong** item";

fn pdf_from_markdown() -> String {
    let document = parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown parses");
    render_pdf_document(&document)
}

#[test]
fn rendered_pdf_is_a_valid_self_describing_document() {
    let pdf = pdf_from_markdown();

    assert!(pdf.starts_with("%PDF-1.7\n"), "carries a PDF header");
    assert!(pdf.trim_end().ends_with("%%EOF"), "carries the EOF marker");
    // The cross-reference table and trailer are present and reference the root.
    assert!(pdf.contains("\nxref\n"));
    assert!(pdf.contains("/Root 1 0 R"));
    // Block structure is carried by marked content; bold by the bold font.
    assert!(pdf.contains("/H1 BDC"));
    assert!(pdf.contains("/P BDC"));
    assert!(pdf.contains("/UL BDC"));
    assert!(pdf.contains("/F2 12 Tf\n(ready) Tj"));
}

#[test]
fn pdf_render_and_parse_are_inverses_on_the_concept_tree() {
    let document = parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown parses");
    let pdf = render_pdf_document(&document);
    let restored = parse_pdf_document(&pdf);

    assert_eq!(restored, document, "parse(render(doc)) == doc");

    // The recovered tree carries the founding feature set.
    assert!(matches!(
        restored.blocks[0],
        BlockNode::Heading { level: 1, .. }
    ));
    assert_eq!(restored.blocks[1].concept_id(), "paragraph");
    assert_eq!(restored.blocks[2].concept_id(), "bullet-list");

    let BlockNode::Paragraph { children } = &restored.blocks[1] else {
        panic!("second block is a paragraph");
    };
    assert!(children.iter().any(|node| matches!(
        node,
        InlineNode::Wrapped { concept, .. } if concept == "strong"
    )));
    assert!(children.iter().any(|node| matches!(
        node,
        InlineNode::Wrapped { concept, .. } if concept == "emphasis"
    )));
}

#[test]
fn rendered_pdf_is_a_fixed_point_under_reparsing() {
    let pdf = pdf_from_markdown();
    let document = parse_pdf_document(&pdf);
    assert_eq!(
        render_pdf_document(&document),
        pdf,
        "render(parse(pdf)) == pdf"
    );
}

#[test]
fn markdown_and_pdf_reach_the_same_concept_tree() {
    let pdf = pdf_from_markdown();
    let from_markdown = parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown tree");
    let from_pdf = parse_markup_document("PDF", &pdf).expect("PDF tree");
    assert_eq!(from_markdown, from_pdf, "PDF is format-independent");
}

#[test]
fn markup_translation_bridges_markdown_to_pdf_and_back() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_common_concept_ontology();

    let pdf = network
        .translate_markup_document("Markdown", "PDF", SAMPLE_MARKDOWN)
        .expect("Markdown translates to PDF");
    assert!(pdf_profile_is_recognized(&pdf));

    let restored = network
        .translate_markup_document("PDF", "Markdown", &pdf)
        .expect("PDF translates back to Markdown");
    assert_eq!(restored, SAMPLE_MARKDOWN);
}

#[test]
fn pdf_parser_preserves_bytes_and_tags_the_document_structure() {
    let pdf = pdf_from_markdown();
    let network = LinkNetwork::parse(&pdf, "pdf", ParseConfiguration::default());

    // Lossless: every byte round-trips and the parse is clean.
    assert_eq!(network.reconstruct_text(), pdf);
    assert!(network.verify_full_match(None).is_clean());

    // Concept-tagged structure links exist for the founding feature set.
    let tags = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Object))
        .filter_map(|link| link.metadata().term().map(str::to_string))
        .collect::<Vec<_>>();
    for concept in [
        "heading",
        "paragraph",
        "bullet-list",
        "list-item",
        "strong",
        "emphasis",
    ] {
        assert!(
            tags.iter().any(|tag| tag == concept),
            "missing concept-tagged structure link for {concept}"
        );
    }
}

#[test]
fn reconstruct_text_as_pdf_renders_a_structurally_equivalent_pdf() {
    // A Markdown-sourced network reconstructs as an equivalent PDF.
    let markdown_network =
        LinkNetwork::parse(SAMPLE_MARKDOWN, "Markdown", ParseConfiguration::default());
    let pdf = markdown_network.reconstruct_text_as("PDF", ParseConfiguration::default());
    assert!(pdf_profile_is_recognized(&pdf));
    assert_eq!(
        parse_pdf_document(&pdf),
        parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown tree"),
        "PDF reconstruction carries the same concept tree as its Markdown source"
    );

    // A PDF-sourced network reconstructs byte-for-byte.
    let original = pdf_from_markdown();
    let pdf_network = LinkNetwork::parse(&original, "pdf", ParseConfiguration::default());
    assert_eq!(
        pdf_network.reconstruct_text_as("PDF", ParseConfiguration::default()),
        original
    );
}

#[test]
fn out_of_profile_pdf_parses_to_an_empty_document_without_panicking() {
    // A PDF without the profile's marked content yields no blocks (graceful).
    let bare = "%PDF-1.7\n1 0 obj\n<< /Type /Catalog >>\nendobj\n%%EOF\n";
    assert!(parse_pdf_document(bare).blocks.is_empty());
    assert!(!pdf_profile_is_recognized(bare));

    // It still parses losslessly as a network.
    let network = LinkNetwork::parse(bare, "pdf", ParseConfiguration::default());
    assert_eq!(network.reconstruct_text(), bare);
}
