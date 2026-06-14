//! Cross-format document reconstruction (issue #86).
//!
//! Every ordered pair of document formats round-trips through the shared,
//! language-free formatting concept layer: a document built from the concepts
//! both formats support survives `A → concepts → B → concepts → A`. The
//! per-format [`document_format_profile`] reports, for every cross-format
//! concept, either native support or a documented lossy fallback, and
//! [`LinkNetwork::reconstruct_text_as`] is the entry point that drives the
//! translation.

use std::collections::BTreeMap;

use meta_language::{
    canonical_document_format, document_format_profile, parse_markup_document, BlockNode,
    FormattingDocument, InlineNode, LinkNetwork, ParseConfiguration, CROSS_FORMAT_CONCEPTS,
    DOCUMENT_FORMATS,
};

/// Whether `format` represents `concept` natively (no lossy fallback).
fn supports(format: &str, concept: &str) -> bool {
    document_format_profile(format)
        .expect("known document format")
        .supports_concept(concept)
}

fn wrapped(concept: &str, text: &str) -> InlineNode {
    InlineNode::Wrapped {
        concept: concept.to_string(),
        attributes: BTreeMap::new(),
        children: vec![InlineNode::Text(text.to_string())],
    }
}

fn hyperlink(text: &str, href: &str) -> InlineNode {
    let mut attributes = BTreeMap::new();
    attributes.insert("href".to_string(), href.to_string());
    InlineNode::Wrapped {
        concept: "hyperlink".to_string(),
        attributes,
        children: vec![InlineNode::Text(text.to_string())],
    }
}

/// Builds a sample document using only the concepts that **both** `source` and
/// `target` support natively, so the round-trip exercises no lossy fallbacks.
fn intersection_sample(source: &str, target: &str) -> FormattingDocument {
    let both = |concept: &str| supports(source, concept) && supports(target, concept);

    let mut blocks = Vec::new();

    if both("heading") {
        blocks.push(BlockNode::Heading {
            level: 1,
            children: vec![InlineNode::Text("Status Report".to_string())],
        });
    }

    blocks.push(BlockNode::Paragraph {
        children: paragraph_inline(&both),
    });

    if both("bullet-list") && both("list-item") {
        blocks.push(BlockNode::List {
            concept: "bullet-list".to_string(),
            items: list_items(&both),
        });
    }

    if both("ordered-list") && both("list-item") {
        blocks.push(BlockNode::List {
            concept: "ordered-list".to_string(),
            items: list_items(&both),
        });
    }

    FormattingDocument { blocks }
}

fn paragraph_inline(both: &impl Fn(&str) -> bool) -> Vec<InlineNode> {
    let mut nodes = vec![InlineNode::Text("The system is ".to_string())];
    if both("strong") {
        nodes.push(wrapped("strong", "ready"));
        nodes.push(InlineNode::Text(" for ".to_string()));
    } else {
        nodes.push(InlineNode::Text("ready for ".to_string()));
    }
    if both("emphasis") {
        nodes.push(wrapped("emphasis", "launch"));
    } else {
        nodes.push(InlineNode::Text("launch".to_string()));
    }
    if both("hyperlink") {
        nodes.push(InlineNode::Text(", see ".to_string()));
        nodes.push(hyperlink("docs", "https://example.com"));
    }
    nodes.push(InlineNode::Text(".".to_string()));
    nodes
}

fn list_items(both: &impl Fn(&str) -> bool) -> Vec<Vec<InlineNode>> {
    let mut second = vec![InlineNode::Text("Second ".to_string())];
    if both("strong") {
        second.push(wrapped("strong", "strong"));
        second.push(InlineNode::Text(" item".to_string()));
    } else {
        second.push(InlineNode::Text("strong item".to_string()));
    }
    vec![vec![InlineNode::Text("First item".to_string())], second]
}

#[test]
fn every_ordered_format_pair_round_trips_through_the_concept_layer() {
    let mut network = LinkNetwork::self_describing();
    let _ = network.seed_common_concept_ontology();

    for &source in DOCUMENT_FORMATS {
        for &target in DOCUMENT_FORMATS {
            let built = intersection_sample(source, target);
            let source_text = network.render_markup_document(source, &built);

            // The canonical concept tree as recovered from the source format.
            let source_tree =
                parse_markup_document(source, &source_text).expect("source format parses");
            assert!(
                !source_tree.blocks.is_empty(),
                "{source} -> {target}: sample carries at least one block"
            );

            // A -> concepts -> B
            let target_text = network
                .translate_markup_document(source, target, &source_text)
                .expect("source translates to target");
            let target_tree =
                parse_markup_document(target, &target_text).expect("target format parses");
            assert_eq!(
                source_tree, target_tree,
                "{source} -> {target}: concept tree survives the forward translation"
            );

            // B -> concepts -> A
            let restored_text = network
                .translate_markup_document(target, source, &target_text)
                .expect("target translates back to source");
            let restored_tree =
                parse_markup_document(source, &restored_text).expect("source format reparses");
            assert_eq!(
                source_tree, restored_tree,
                "{source} <-> {target}: concept tree survives the A -> B -> A round trip"
            );
        }
    }
}

#[test]
fn reconstruct_text_as_translates_across_formats() {
    const SAMPLE_MARKDOWN: &str = "# Status Report\n\nThe system is **ready** for *launch*, see [docs](https://example.com).\n\n- First item\n- Second **strong** item";

    let network = LinkNetwork::parse(SAMPLE_MARKDOWN, "Markdown", ParseConfiguration::default());

    // Same-format reconstruction is byte-exact.
    assert_eq!(
        network.reconstruct_text_as("Markdown", ParseConfiguration::default()),
        SAMPLE_MARKDOWN
    );

    // Markdown -> HTML carries the same concept tree.
    let html = network.reconstruct_text_as("HTML", ParseConfiguration::default());
    assert!(html.contains("<h1>Status Report</h1>"));
    assert!(html.contains("<strong>ready</strong>"));
    assert!(html.contains("<em>launch</em>"));
    assert!(html.contains("<a href=\"https://example.com\">docs</a>"));
    assert_eq!(
        parse_markup_document("HTML", &html).expect("HTML parses"),
        parse_markup_document("Markdown", SAMPLE_MARKDOWN).expect("Markdown parses"),
        "HTML reconstruction carries the Markdown source's concept tree"
    );

    // Markdown -> txt degrades through the documented fallbacks: prose survives,
    // markup is dropped.
    let txt = network.reconstruct_text_as("txt", ParseConfiguration::default());
    assert!(txt.contains("Status Report"));
    assert!(txt.contains("The system is ready for launch"));
    assert!(!txt.contains('#'), "txt drops heading markup");
    assert!(!txt.contains("**"), "txt drops bold markup");
    assert!(!txt.contains("https://example.com"), "txt drops the URL");
    assert!(
        txt.contains("- First item"),
        "txt keeps a plain list marker"
    );
}

#[test]
fn reconstruct_text_as_is_byte_exact_without_a_document_source() {
    // A plain natural-language network has no document source language, so a
    // document-format target returns the byte-exact reconstruction.
    let network = LinkNetwork::parse("Hello world", "English", ParseConfiguration::default());
    assert_eq!(
        network.reconstruct_text_as("HTML", ParseConfiguration::default()),
        network.reconstruct_text()
    );
}

#[test]
fn every_format_profile_reports_support_or_a_fallback_for_each_concept() {
    for &format in DOCUMENT_FORMATS {
        let profile = document_format_profile(format).expect("known document format");
        for &concept in CROSS_FORMAT_CONCEPTS {
            let supported = profile.supports_concept(concept);
            let fallback = profile.concept_fallback(concept).is_some();
            assert!(
                supported ^ fallback,
                "{format}: `{concept}` must be either natively supported or have exactly one documented fallback"
            );
        }
    }
}

#[test]
fn txt_profile_flags_every_unsupported_concept_as_a_fallback() {
    let profile = document_format_profile("txt").expect("txt profile");

    assert!(profile.supports_concept("paragraph"));
    for concept in [
        "heading",
        "bullet-list",
        "ordered-list",
        "list-item",
        "strong",
        "emphasis",
        "hyperlink",
    ] {
        assert!(!profile.supports_concept(concept), "txt drops {concept}");
        assert!(
            profile.concept_fallback(concept).is_some(),
            "txt documents a fallback for {concept}"
        );
    }
    assert_eq!(profile.fallbacks().len(), 7);
}

#[test]
fn ordered_list_falls_back_to_bullets_in_markdown() {
    let profile = document_format_profile("Markdown").expect("Markdown profile");
    assert!(!profile.supports_concept("ordered-list"));
    assert!(profile.concept_fallback("ordered-list").is_some());
}

#[test]
fn hyperlink_falls_back_in_binary_document_formats() {
    for format in ["PDF", "DOCX"] {
        let profile = document_format_profile(format).expect("profile");
        assert!(profile.supports_concept("heading"));
        assert!(profile.supports_concept("ordered-list"));
        assert!(
            !profile.supports_concept("hyperlink"),
            "{format} drops links"
        );
        assert!(profile.concept_fallback("hyperlink").is_some());
    }
}

#[test]
fn canonical_document_format_accepts_common_aliases() {
    assert_eq!(canonical_document_format("md"), Some("Markdown"));
    assert_eq!(canonical_document_format("MARKDOWN"), Some("Markdown"));
    assert_eq!(canonical_document_format("htm"), Some("HTML"));
    assert_eq!(canonical_document_format("plain-text"), Some("txt"));
    assert_eq!(canonical_document_format("pdf"), Some("PDF"));
    assert_eq!(canonical_document_format("docx"), Some("DOCX"));
    assert_eq!(canonical_document_format("English"), None);
}
