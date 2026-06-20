//! Round-trips a document through the text PDF profile (issue #84): a Markdown
//! source is translated into a valid single-page PDF and back without losing its
//! heading/paragraph/list and bold/italic structure, and the rendered PDF parses
//! into the same language-free concept tree as its Markdown source.

use meta_language::{parse_markup_document, parse_pdf_document, LinkNetwork, ParseConfiguration};

fn main() {
    let mut network = LinkNetwork::self_describing();
    let report = network.seed_common_concept_ontology();
    println!(
        "formatting concepts seeded: {}",
        report.formatting_concepts()
    );

    let markdown =
        "# Status Report\n\nThe system is **ready** for *launch*.\n\n- First item\n- Second **strong** item";

    // Markdown ⇄ PDF travels through the shared formatting concept layer.
    let pdf = network
        .translate_markup_document("Markdown", "PDF", markdown)
        .expect("Markdown translates to PDF");
    let restored = network
        .translate_markup_document("PDF", "Markdown", &pdf)
        .expect("PDF translates back to Markdown");

    println!("--- Markdown ---\n{markdown}\n");
    println!("--- PDF ({} bytes) ---\n{pdf}", pdf.len());
    println!("--- Restored Markdown ---\n{restored}\n");
    assert_eq!(restored, markdown, "round trip is lossless for this sample");

    // The PDF and Markdown surfaces reach the one shared concept tree.
    assert_eq!(
        parse_pdf_document(&pdf),
        parse_markup_document("Markdown", markdown).expect("Markdown tree"),
        "PDF carries the same concept tree as its Markdown source"
    );

    // Parsing the PDF as a links network is byte-exact; the concept tags are
    // additive and `reconstruct_text()` returns the input verbatim.
    let pdf_network = LinkNetwork::parse(&pdf, "pdf", ParseConfiguration::default());
    assert_eq!(pdf_network.reconstruct_text(), pdf);
    assert!(pdf_network.verify_full_match(None).is_clean());
    println!(
        "PDF parsed losslessly: {} bytes round-trip byte-for-byte",
        pdf.len()
    );

    // `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF.
    let markdown_network = LinkNetwork::parse(markdown, "Markdown", ParseConfiguration::default());
    let reconstructed = markdown_network.reconstruct_text_as("PDF", ParseConfiguration::default());
    assert_eq!(
        parse_pdf_document(&reconstructed),
        parse_markup_document("Markdown", markdown).expect("Markdown tree"),
        "reconstruct_text_as(PDF) preserves the concept tree"
    );
    println!("reconstruct_text_as(\"PDF\") produced a structurally equivalent document");
}
