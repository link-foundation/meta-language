// Throwaway reproduction script verifying the seven data-exchange grammar
// crates wire end-to-end through `LinkNetwork::parse`: root node kind, byte-exact
// round-trip, clean verification, and emitted `LinkType::Syntax` links.
//
//   cargo run --example data_format_grammar_check
use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

fn report(language: &str, source: &str) {
    let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
    let round_trips = network.reconstruct_text() == source;
    let clean = network.verify_full_match(None).is_clean();
    let syntax = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
        .count();
    let root = network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Syntax))
        .filter_map(|link| link.metadata().term().map(str::to_string))
        .next()
        .unwrap_or_default();
    println!(
        "{language:>10}: round_trip={round_trips} clean={clean} syntax_links={syntax} first_kind={root}"
    );
}

fn main() {
    report("JSON", "{\n  \"name\": \"café\",\n  \"items\": [1, 2, 3]\n}\n");
    report("YAML", "name: café\nitems:\n  - 1\n  - 2\n");
    report("TOML", "title = \"café\"\n\n[owner]\nname = \"Tom\"\n");
    report("XML", "<note lang=\"en\">\n  <body>café</body>\n</note>\n");
    report("INI", "; comment\n[owner]\nname = café\n");
    report("protobuf", "syntax = \"proto3\";\n\nmessage Person {\n  string name = 1;\n}\n");
    report("GraphQL", "type Person {\n  name: String!\n}\n");
}
