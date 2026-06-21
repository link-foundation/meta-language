// Quick check that Ruby source produces grammar-backed syntax links.
use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

fn main() {
    let source = "def greet(name)\n  puts \"Hello, #{name}\"\nend\n";
    let network = LinkNetwork::parse(source, "Ruby", ParseConfiguration::default());

    let syntax = network
        .links()
        .filter(|l| l.metadata().link_type() == Some(LinkType::Syntax)
            && l.metadata().language() == Some("Ruby"))
        .count();
    println!("syntax links: {syntax}");
    println!("round-trips: {}", network.reconstruct_text() == source);
    println!("clean: {}", network.verify_full_match(None).is_clean());

    let mut terms: Vec<_> = network
        .links()
        .filter(|l| l.metadata().link_type() == Some(LinkType::Syntax))
        .filter_map(|l| l.metadata().term())
        .map(str::to_string)
        .collect();
    terms.sort();
    terms.dedup();
    println!("terms: {terms:?}");
}
