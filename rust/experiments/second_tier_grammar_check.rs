// Throwaway reproduction script verifying the five second-tier programming
// grammar crates wire end-to-end through `LinkNetwork::parse`: root node kind,
// byte-exact round-trip, clean verification, and emitted `LinkType::Syntax`
// links.
//
//   cargo run --example second_tier_grammar_check
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
        "{language:>8}: round_trip={round_trips} clean={clean} syntax_links={syntax} first_kind={root}"
    );
}

fn main() {
    report("php", "<?php\nfunction greet($name) {\n    return \"café \" . $name;\n}\n");
    report("swift", "func greet(_ name: String) -> String {\n    return \"café \\(name)\"\n}\n");
    report(
        "kotlin",
        "fun greet(name: String): String {\n    return \"café $name\"\n}\n",
    );
    report(
        "scala",
        "object Demo {\n  def greet(name: String): String = s\"café $name\"\n}\n",
    );
    report("lua", "local function greet(name)\n  return \"café \" .. name\nend\n");
}
