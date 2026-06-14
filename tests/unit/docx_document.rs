//! DOCX (OOXML) document-format support: text-profile render/parse round-trips,
//! the cross-format concept bridge, concept-tagged parsing, the binary OPC
//! package round-trip, and `reconstruct_text_as`.

use meta_language::{
    docx_package_is_recognized, docx_profile_is_recognized, parse_docx_document,
    parse_docx_package, parse_markup_document, render_docx_document, render_docx_package,
    BlockNode, InlineNode, LinkNetwork, LinkType, ParseConfiguration,
};

const SAMPLE_MARKDOWN: &str =
    "# Status Report\n\nThe system is **ready** for *launch*.\n\n- First item\n- Second **strong** item";

fn docx_from_markdown() -> String {
    let document = parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown parses");
    render_docx_document(&document)
}

#[test]
fn rendered_docx_is_valid_wordprocessingml() {
    let docx = docx_from_markdown();

    assert!(
        docx.starts_with("<?xml version=\"1.0\""),
        "carries an XML prolog"
    );
    assert!(docx.contains(
        "<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\">"
    ));
    assert!(docx.trim_end().ends_with("</w:document>"));
    // Heading style, bold run property, and list numbering are present.
    assert!(docx.contains("<w:pStyle w:val=\"Heading1\"/>"));
    assert!(docx.contains("<w:rPr><w:b/></w:rPr><w:t xml:space=\"preserve\">ready</w:t>"));
    assert!(docx.contains("<w:numId w:val=\"1\"/>"));
    // Emphasis maps to the italic run property.
    assert!(docx.contains("<w:rPr><w:i/></w:rPr><w:t xml:space=\"preserve\">launch</w:t>"));
}

#[test]
fn docx_render_and_parse_are_inverses_on_the_concept_tree() {
    let document = parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown parses");
    let docx = render_docx_document(&document);
    let restored = parse_docx_document(&docx);

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
fn rendered_docx_is_a_fixed_point_under_reparsing() {
    let docx = docx_from_markdown();
    let document = parse_docx_document(&docx);
    assert_eq!(
        render_docx_document(&document),
        docx,
        "render(parse(docx)) == docx"
    );
}

#[test]
fn markdown_and_docx_reach_the_same_concept_tree() {
    let docx = docx_from_markdown();
    let from_markdown = parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown tree");
    let from_docx = parse_markup_document("DOCX", &docx).expect("DOCX tree");
    assert_eq!(from_markdown, from_docx, "DOCX is format-independent");
}

#[test]
fn markup_translation_bridges_markdown_to_docx_and_back() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_common_concept_ontology();

    let docx = network
        .translate_markup_document("Markdown", "DOCX", SAMPLE_MARKDOWN)
        .expect("Markdown translates to DOCX");
    assert!(docx_profile_is_recognized(&docx));

    let restored = network
        .translate_markup_document("DOCX", "Markdown", &docx)
        .expect("DOCX translates back to Markdown");
    assert_eq!(restored, SAMPLE_MARKDOWN);
}

#[test]
fn docx_parser_preserves_bytes_and_tags_the_document_structure() {
    let docx = docx_from_markdown();
    let network = LinkNetwork::parse(&docx, "docx", ParseConfiguration::default());

    // Lossless: every byte round-trips and the parse is clean.
    assert_eq!(network.reconstruct_text(), docx);
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
fn reconstruct_text_as_docx_renders_a_structurally_equivalent_docx() {
    // A Markdown-sourced network reconstructs as an equivalent DOCX.
    let markdown_network =
        LinkNetwork::parse(SAMPLE_MARKDOWN, "Markdown", ParseConfiguration::default());
    let docx = markdown_network.reconstruct_text_as("DOCX", ParseConfiguration::default());
    assert!(docx_profile_is_recognized(&docx));
    assert_eq!(
        parse_docx_document(&docx),
        parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown tree"),
        "DOCX reconstruction carries the same concept tree as its Markdown source"
    );

    // A DOCX-sourced network reconstructs byte-for-byte.
    let original = docx_from_markdown();
    let docx_network = LinkNetwork::parse(&original, "docx", ParseConfiguration::default());
    assert_eq!(
        docx_network.reconstruct_text_as("DOCX", ParseConfiguration::default()),
        original
    );
}

#[test]
fn docx_opc_package_is_a_valid_zip_carrying_the_same_concept_tree() {
    let document = parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown tree");
    let package = render_docx_package(&document);

    // A real ZIP container: local-file-header signature `PK\x03\x04`.
    assert_eq!(&package[..4], b"PK\x03\x04");
    assert!(docx_package_is_recognized(&package));
    assert_eq!(
        parse_docx_package(&package),
        document,
        "the .docx package round-trips the concept tree"
    );
}

#[test]
fn out_of_profile_docx_parses_to_an_empty_document_without_panicking() {
    // OOXML without recognizable paragraphs yields no blocks (graceful).
    let bare = "<?xml version=\"1.0\"?>\n<w:document><w:body><w:sectPr/></w:body></w:document>\n";
    assert!(parse_docx_document(bare).blocks.is_empty());
    assert!(!docx_profile_is_recognized(bare));

    // It still parses losslessly as a network.
    let network = LinkNetwork::parse(bare, "docx", ParseConfiguration::default());
    assert_eq!(network.reconstruct_text(), bare);

    // A non-ZIP byte blob is not a recognized package.
    assert!(!docx_package_is_recognized(b"not a zip file"));
    assert!(parse_docx_package(b"not a zip file").blocks.is_empty());
}
