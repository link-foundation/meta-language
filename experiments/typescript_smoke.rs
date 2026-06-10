use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

fn main() {
    let source = "const value: number = 1;\n";
    let network = LinkNetwork::parse(source, "TypeScript", ParseConfiguration::default());
    let syntax = network
        .links()
        .filter(|l| l.metadata().link_type() == Some(LinkType::Syntax))
        .count();
    println!("reconstruct == source: {}", network.reconstruct_text() == source);
    println!("clean: {}", network.verify_full_match(None).is_clean());
    println!("syntax links: {syntax}");
    let has_program = network.links().any(|l| {
        l.metadata().link_type() == Some(LinkType::Syntax)
            && l.metadata().term() == Some("program")
    });
    println!("has program node: {has_program}");
}
