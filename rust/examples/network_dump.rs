//! Prints every link of a parsed network, one per line, for debugging.
//!
//!   cargo run --example `network_dump` -- <language> <source>
use meta_language::{LinkNetwork, ParseConfiguration};

fn main() {
    let arguments = std::env::args().collect::<Vec<_>>();
    let source = arguments[2].replace("\\n", "\n");
    let network = LinkNetwork::parse(&source, &arguments[1], ParseConfiguration::default());
    for link in network.links() {
        let metadata = link.metadata();
        println!(
            "{} {:?} {:?} named={} span={:?} flags={:?} refs={:?}",
            link.id().as_u64(),
            metadata.link_type(),
            metadata.term(),
            metadata.is_named(),
            metadata.span().map(meta_language::SourceSpan::byte_range),
            metadata.flags(),
            link.references()
                .iter()
                .map(|id| id.as_u64())
                .collect::<Vec<_>>()
        );
    }
    println!("reconstruction {:?}", network.reconstruct_text());
    println!("clean {}", network.verify_full_match(None).is_clean());
}
