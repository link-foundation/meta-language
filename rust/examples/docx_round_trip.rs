//! Round-trips a document through the DOCX (OOXML) profile (issue #85): a
//! Markdown source is translated into OOXML `word/document.xml` and back without
//! losing its heading/paragraph/list and bold structure, the rendered OOXML
//! parses into the same language-free concept tree as its Markdown source, and a
//! complete `.docx` OPC package is assembled and read back.

use meta_language::{
    docx_package_is_recognized, parse_docx_document, parse_docx_package, parse_markup_document,
    render_docx_package, LinkNetwork, ParseConfiguration,
};

fn main() {
    let mut network = LinkNetwork::self_describing();
    let report = network.seed_common_concept_ontology();
    println!(
        "formatting concepts seeded: {}",
        report.formatting_concepts()
    );

    let markdown =
        "# Status Report\n\nThe system is **ready** for *launch*.\n\n- First item\n- Second **strong** item";

    // Markdown ⇄ DOCX travels through the shared formatting concept layer.
    let docx = network
        .translate_markup_document("Markdown", "DOCX", markdown)
        .expect("Markdown translates to DOCX");
    let restored = network
        .translate_markup_document("DOCX", "Markdown", &docx)
        .expect("DOCX translates back to Markdown");

    println!("--- Markdown ---\n{markdown}\n");
    println!("--- DOCX document.xml ({} bytes) ---\n{docx}\n", docx.len());
    println!("--- Restored Markdown ---\n{restored}\n");
    assert_eq!(restored, markdown, "round trip is lossless for this sample");

    // The DOCX and Markdown surfaces reach the one shared concept tree.
    assert_eq!(
        parse_docx_document(&docx),
        parse_markup_document("Markdown", markdown).expect("Markdown tree"),
        "DOCX carries the same concept tree as its Markdown source"
    );

    // Parsing the OOXML as a links network is byte-exact; the concept tags are
    // additive and `reconstruct_text()` returns the input verbatim.
    let docx_network = LinkNetwork::parse(&docx, "docx", ParseConfiguration::default());
    assert_eq!(docx_network.reconstruct_text(), docx);
    assert!(docx_network.verify_full_match(None).is_clean());
    println!(
        "OOXML parsed losslessly: {} bytes round-trip byte-for-byte",
        docx.len()
    );

    // A complete `.docx` OPC package is a valid ZIP that reads back the same tree.
    let package = render_docx_package(&parse_markup_document("Markdown", markdown).expect("tree"));
    assert!(docx_package_is_recognized(&package));
    assert_eq!(
        parse_docx_package(&package),
        parse_markup_document("Markdown", markdown).expect("Markdown tree"),
        "the .docx package carries the same concept tree as its Markdown source"
    );
    println!(
        "assembled a valid .docx OPC package ({} bytes, 6 parts)",
        package.len()
    );
}
