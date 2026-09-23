use meta_language::{ByteRange, LinkNetwork, LinkType, ParseConfiguration};

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
