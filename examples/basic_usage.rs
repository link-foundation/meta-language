use meta_language::{LinkNetwork, ParseConfiguration};

fn main() {
    let network = LinkNetwork::parse("alpha beta", "plain-text", ParseConfiguration::default());
    let report = network.verify_full_match(None);

    println!("links: {}", network.len());
    println!("clean: {}", report.is_clean());
}
