use meta_language::{
    parse_markup_document, BlockNode, DocumentFormatInstance, InlineNode, LinkNetwork, LinkType,
    NetworkProjection,
};

#[test]
fn seeding_attaches_per_format_syntax_mappings_to_each_concept() {
    let mut network = LinkNetwork::self_describing();
    let report = network.seed_document_formatting_concepts();

    // Inline and block concepts required by issue #83.
    assert_eq!(report.concepts(), 18);
    // Markdown + HTML mapping for every concept.
    assert_eq!(report.syntax_mappings(), report.concepts() * 2);

    assert_eq!(
        network.reconstruct_concept("strong", "Markdown"),
        Some("**{}**")
    );
    assert_eq!(
        network.reconstruct_concept("strong", "HTML"),
        Some("<strong>{}</strong>")
    );
}

#[test]
fn common_concept_ontology_includes_document_formatting_concepts() {
    let mut network = LinkNetwork::self_describing();
    let report = network.seed_common_concept_ontology();

    assert_eq!(report.formatting_concepts(), 18);
    assert!(network.document_formatting_concept("strong").is_some());
    assert!(network.document_formatting_concept("heading").is_some());
}

#[test]
fn markdown_and_html_bold_reach_the_same_strong_concept() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_document_formatting_concepts();

    let markdown = network
        .resolve_document_format("Markdown", "**bold**")
        .expect("Markdown bold resolves");
    let html = network
        .resolve_document_format("HTML", "<strong>bold</strong>")
        .expect("HTML bold resolves");

    assert_eq!(markdown.concept, "strong");
    assert_eq!(html.concept, "strong");
    // Both surface syntaxes reach the one language-free concept link.
    assert_eq!(markdown.link, html.link);
    assert_eq!(markdown.content, "bold");
    assert_eq!(html.content, "bold");

    // The shared link is surfaced as a concept under semantic projection.
    let strong = network
        .document_formatting_concept("strong")
        .expect("strong concept seeded");
    assert_eq!(markdown.link, strong);
    assert!(network
        .projected_links(NetworkProjection::Semantic)
        .any(|link| {
            link.id() == strong && link.metadata().link_type() == Some(LinkType::Concept)
        }));
}

#[test]
fn italic_is_not_mistaken_for_bold() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_document_formatting_concepts();

    let emphasis = network
        .resolve_document_format("Markdown", "*italic*")
        .expect("Markdown italic resolves");
    assert_eq!(emphasis.concept, "emphasis");
    assert_eq!(emphasis.content, "italic");
}

#[test]
fn heading_level_round_trips_across_formats() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_document_formatting_concepts();

    let markdown = network
        .resolve_document_format("Markdown", "### Title")
        .expect("Markdown heading resolves");
    assert_eq!(markdown.concept, "heading");
    assert_eq!(markdown.level, Some(3));
    assert_eq!(markdown.content, "Title");

    let instance = DocumentFormatInstance {
        content: markdown.content,
        level: markdown.level,
        attributes: markdown.attributes,
    };
    assert_eq!(
        network.render_document_format("heading", "HTML", &instance),
        Some("<h3>Title</h3>".to_string())
    );
}

#[test]
fn hyperlink_translates_through_the_concept_layer() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_document_formatting_concepts();

    assert_eq!(
        network.translate_document_format("Markdown", "HTML", "[Link](https://example.com)"),
        Some("<a href=\"https://example.com\">Link</a>".to_string())
    );
    assert_eq!(
        network.translate_document_format(
            "HTML",
            "Markdown",
            "<a href=\"https://example.com\">Link</a>"
        ),
        Some("[Link](https://example.com)".to_string())
    );
}

const SAMPLE_MARKDOWN: &str = "# Heading\n\nA paragraph with **bold**, *italic*, and a [Link](https://example.com).\n\n- First item\n- Second **strong** item";

#[test]
fn markdown_parses_into_the_language_free_concept_layer() {
    let document = parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown parses");

    assert_eq!(document.blocks.len(), 3);
    assert!(matches!(
        document.blocks[0],
        BlockNode::Heading { level: 1, .. }
    ));
    assert_eq!(document.blocks[1].concept_id(), "paragraph");
    assert_eq!(document.blocks[2].concept_id(), "bullet-list");

    let BlockNode::Paragraph { children } = &document.blocks[1] else {
        panic!("second block is a paragraph");
    };
    assert!(children.iter().any(|node| matches!(
        node,
        InlineNode::Wrapped { concept, .. } if concept == "strong"
    )));
    assert!(children.iter().any(|node| matches!(
        node,
        InlineNode::Wrapped { concept, .. } if concept == "hyperlink"
    )));
}

#[test]
fn markdown_round_trips_to_html_and_back_through_one_concept_ontology() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_common_concept_ontology();

    let html = network
        .translate_markup_document("Markdown", "HTML", SAMPLE_MARKDOWN)
        .expect("Markdown translates to HTML");

    assert!(html.contains("<h1>Heading</h1>"));
    assert!(html.contains("<strong>bold</strong>"));
    assert!(html.contains("<em>italic</em>"));
    assert!(html.contains("<a href=\"https://example.com\">Link</a>"));
    assert!(html.contains("<ul><li>First item</li>"));

    // The concept layer is format-independent: parsing the HTML back yields the
    // same tree the Markdown produced.
    let from_markdown = parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown tree");
    let from_html = parse_markup_document("HTML", &html).expect("HTML tree");
    assert_eq!(from_markdown, from_html);

    let restored = network
        .translate_markup_document("HTML", "Markdown", &html)
        .expect("HTML translates back to Markdown");
    assert_eq!(restored, SAMPLE_MARKDOWN);
}
