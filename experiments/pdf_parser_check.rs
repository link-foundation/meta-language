use meta_language::{
    parse_markup_document, render_pdf_document, LinkNetwork, LinkType, ParseConfiguration,
};

fn main() {
    let markdown = "# Title\n\nA paragraph with **bold** and *italic*.\n\n- First item\n- Second **strong** item";
    let doc = parse_markup_document("Markdown", markdown).unwrap();
    let pdf = render_pdf_document(&doc);

    let network = LinkNetwork::parse("pdf", &pdf, ParseConfiguration::default());
    // NOTE: signature is parse(text, language, config)
    let network = LinkNetwork::parse(&pdf, "pdf", ParseConfiguration::default());
    let _ = network;

    let network = LinkNetwork::parse(&pdf, "pdf", ParseConfiguration::default());
    let reconstructed = network.reconstruct_text();
    println!("byte-exact reconstruction: {}", reconstructed == pdf);
    assert_eq!(reconstructed, pdf);

    // Count concept-tagged structure links by term.
    let mut concepts: Vec<String> = network
        .links()
        .filter(|l| l.metadata().link_type() == Some(LinkType::Object))
        .filter_map(|l| l.metadata().term().map(str::to_string))
        .collect();
    concepts.sort();
    println!("structure concept tags: {concepts:?}");

    let concept_points: Vec<String> = network
        .links()
        .filter(|l| l.metadata().link_type() == Some(LinkType::Concept))
        .filter_map(|l| l.metadata().term().map(str::to_string))
        .collect();
    println!("concept points: {concept_points:?}");

    for needle in ["heading", "paragraph", "bullet-list", "list-item", "strong", "emphasis"] {
        assert!(concepts.contains(&needle.to_string()), "missing {needle}");
    }
    println!("ALL OK");
}
