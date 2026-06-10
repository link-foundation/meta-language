use meta_language::{Link, LinkNetwork, LinkType, ParseConfiguration};

fn connected_syntax(network: &LinkNetwork, language: &str) -> bool {
    let region_ids = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Region))
        .filter(|link| link.metadata().language() == Some(language))
        .map(Link::id)
        .collect::<Vec<_>>();
    !region_ids.is_empty()
        && network.links().any(|link| {
            link.metadata().link_type() == Some(LinkType::Syntax)
                && link.metadata().language() == Some(language)
                && link
                    .references()
                    .iter()
                    .any(|reference| region_ids.contains(reference))
        })
}

fn main() {
    let md = "# Config\n\n```json\n{\n  \"enabled\": true\n}\n```\n";
    let network = LinkNetwork::parse(md, "Markdown", ParseConfiguration::default());
    println!("markdown round_trip={}", network.reconstruct_text() == md);
    for region in network.embedded_regions() {
        println!("  region language={} span={:?}", region.language(), region.span().byte_range());
    }
    println!("  json connected_syntax={}", connected_syntax(&network, "json"));
}
