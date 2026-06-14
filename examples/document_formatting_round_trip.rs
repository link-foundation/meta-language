//! Round-trips a Markdown document through the shared, language-free
//! document-formatting concept layer into HTML and back, demonstrating that a
//! single concept ontology reconstructs bold/italic/heading/list/link in either
//! surface format.

use meta_language::LinkNetwork;

fn main() {
    let mut network = LinkNetwork::self_describing();
    let report = network.seed_common_concept_ontology();
    println!(
        "formatting concepts seeded: {}",
        report.formatting_concepts()
    );

    let markdown =
        "# Title\n\nA paragraph with **bold**, *italic*, and a [Link](https://example.com).\n\n- First\n- Second";

    let html = network
        .translate_markup_document("Markdown", "HTML", markdown)
        .expect("Markdown translates to HTML");
    let restored = network
        .translate_markup_document("HTML", "Markdown", &html)
        .expect("HTML translates back to Markdown");

    println!("--- Markdown ---\n{markdown}\n");
    println!("--- HTML ---\n{html}\n");
    println!("--- Restored Markdown ---\n{restored}\n");
    assert_eq!(restored, markdown, "round trip is lossless for this sample");

    // Both surface syntaxes denote the one shared `strong` concept link.
    let from_markdown = network
        .resolve_document_format("Markdown", "**bold**")
        .expect("Markdown bold");
    let from_html = network
        .resolve_document_format("HTML", "<strong>bold</strong>")
        .expect("HTML bold");
    assert_eq!(from_markdown.link, from_html.link);
    println!(
        "Markdown **bold** and HTML <strong> share concept link {:?}",
        from_markdown.link
    );
}
