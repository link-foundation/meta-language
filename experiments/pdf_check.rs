use meta_language::{
    parse_markup_document, parse_pdf_document, render_pdf_document, BlockNode, InlineNode,
    LinkNetwork,
};

fn main() {
    let markdown = "# Title\n\nA paragraph with **bold** text.\n\n- First item\n- Second **strong** item";
    let doc = parse_markup_document("Markdown", markdown).unwrap();
    let pdf = render_pdf_document(&doc);
    println!("=== PDF ({} bytes) ===", pdf.len());
    println!("{pdf}");
    println!("=== Validity checks ===");
    println!("starts with %PDF: {}", pdf.starts_with("%PDF"));
    println!("ends with %%EOF: {}", pdf.trim_end().ends_with("%%EOF"));

    let back = parse_pdf_document(&pdf);
    println!("=== Round-trip concept tree match: {} ===", back == doc);
    assert_eq!(back, doc, "PDF parse(render(doc)) == doc");

    // Cross-format: PDF -> Markdown via the shared ontology
    let mut network = LinkNetwork::self_describing();
    network.seed_common_concept_ontology();
    let md_back = network
        .translate_markup_document("PDF", "Markdown", &pdf)
        .unwrap();
    println!("=== Markdown reconstructed from PDF ===\n{md_back}");
    assert_eq!(md_back, markdown);

    // Inspect tree
    for block in &back.blocks {
        match block {
            BlockNode::Heading { level, children } => {
                println!("Heading L{level}: {children:?}");
            }
            BlockNode::Paragraph { children } => println!("Paragraph: {children:?}"),
            BlockNode::List { concept, items } => {
                println!("List {concept}: {} items", items.len());
                for item in items {
                    let has_strong = item
                        .iter()
                        .any(|n| matches!(n, InlineNode::Wrapped { concept, .. } if concept == "strong"));
                    println!("  item strong={has_strong}: {item:?}");
                }
            }
        }
    }
    println!("ALL OK");
}
