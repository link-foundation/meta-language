use std::fs;
use std::path::PathBuf;

use meta_language::{
    script_language, ByteRange, EmbeddedRegion, LinkNetwork, LinkType, ParseConfiguration,
    RegionDetectionPolicy,
};
use serde_json::Value;

#[test]
fn embedded_grammar_roots_preserve_exact_host_source_boundaries() {
    let html = concat!(
        "<script>const value = \"café\";</script>",
        "<style>.x { color: red; }</style>",
        "<p style=\"color: blue\">text</p>"
    );
    let network = LinkNetwork::parse(html, "HTML", ParseConfiguration::default());

    assert_eq!(network.reconstruct_text(), html);
    assert!(network.verify_full_match(None).is_clean());
    assert_connected_region_bounds(
        &network,
        html,
        "JavaScript",
        "program",
        "const value = \"café\";",
    );
    assert_connected_region_bounds(&network, html, "CSS", "stylesheet", ".x { color: red; }");
    assert_connected_region_bounds(&network, html, "CSS", "stylesheet", "color: blue");

    let invalid = "<script>const = ;</script>";
    let invalid_network = LinkNetwork::parse(invalid, "HTML", ParseConfiguration::default());
    assert_eq!(invalid_network.reconstruct_text(), invalid);
    assert!(!invalid_network.verify_full_match(None).is_clean());
}

fn assert_connected_region_bounds(
    network: &LinkNetwork,
    source: &str,
    language: &str,
    root_term: &str,
    region_source: &str,
) {
    let start = source.find(region_source).expect("region source");
    let range = ByteRange::new(start, start + region_source.len());
    let region = network
        .links()
        .find(|link| {
            link.metadata().link_type() == Some(LinkType::Region)
                && link.metadata().language() == Some(language)
                && link
                    .metadata()
                    .span()
                    .is_some_and(|span| span.byte_range() == range)
        })
        .expect("region with exact boundary");
    assert!(network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Syntax)
            && link.metadata().language() == Some(language)
            && link.metadata().term() == Some(root_term)
            && link.references().contains(&region.id())
            && link
                .metadata()
                .span()
                .is_some_and(|span| span.byte_range() == range)
    }));
    assert!(network.links().any(|link| {
        link.metadata().link_type() == Some(LinkType::Token)
            && link.metadata().language() == Some(language)
            && link.metadata().span().is_some_and(|span| {
                range.start() <= span.byte_range().start() && span.byte_range().end() <= range.end()
            })
    }));
}

fn region_cases() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../parity/fixtures/embedded-region-cases.json");
    serde_json::from_str(&fs::read_to_string(path).expect("region cases are readable"))
        .expect("region cases are valid JSON")
}

#[test]
fn host_grammar_csts_delimit_the_shared_embedded_region_cases() {
    for case in region_cases()["cases"].as_array().expect("cases") {
        let source = case["source"].as_str().expect("source");
        let host = case["host"].as_str().expect("host");
        let policy = match case["policy"].as_str().expect("policy") {
            "NameDriven" => RegionDetectionPolicy::NameDriven,
            "ContentDriven" => RegionDetectionPolicy::ContentDriven,
            _ => RegionDetectionPolicy::Both,
        };
        let expected: Vec<(String, usize, usize)> = case["regions"]
            .as_array()
            .expect("regions")
            .iter()
            .map(|region| {
                let start = usize::try_from(region[1].as_u64().expect("start")).expect("start");
                let end = usize::try_from(region[2].as_u64().expect("end")).expect("end");
                assert_eq!(&source[start..end], region[3].as_str().expect("text"));
                (
                    region[0].as_str().expect("language").to_string(),
                    start,
                    end,
                )
            })
            .collect();
        let network = LinkNetwork::parse(
            source,
            host,
            ParseConfiguration::default().with_region_detection_policy(policy),
        );
        let found: Vec<(String, usize, usize)> = network
            .embedded_regions()
            .iter()
            .map(|region: &EmbeddedRegion| {
                let range = region.span().byte_range();
                (region.language().to_string(), range.start(), range.end())
            })
            .collect();
        assert_eq!(found, expected, "{source:?}");
        assert_eq!(network.reconstruct_text(), source);
    }
}

#[test]
fn script_types_name_the_language_of_their_element_content() {
    assert_eq!(script_language(""), Some("JavaScript"));
    assert_eq!(
        script_language(" Text/JavaScript ; charset=utf-8"),
        Some("JavaScript")
    );
    assert_eq!(script_language("module"), Some("JavaScript"));
    assert_eq!(script_language("importmap"), Some("JSON"));
    assert_eq!(script_language("speculationrules"), Some("JSON"));
    assert_eq!(script_language("application/json"), Some("JSON"));
    assert_eq!(script_language("application/ld+json"), Some("JSON"));
    assert_eq!(script_language("text/typescript"), Some("TypeScript"));
    assert_eq!(script_language("text/x-python"), Some("Python"));
    assert_eq!(script_language("text/x-template"), None);
    assert_eq!(script_language("typescript"), None);
}
