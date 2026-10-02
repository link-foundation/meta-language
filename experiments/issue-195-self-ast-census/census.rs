//! formal-ai's `ast_census` (rust/src/agentic_coding/self_ast.rs at d209aac)
//! run on one source with this checkout's crate, to see what drifted.
//! Usage: copy to rust/examples/self_ast_census.rs, then from rust/ run
//! `cargo run --release --example self_ast_census -- <file.rs>`; the
//! CENSUS_DUMP and CENSUS_TOKENS variables print every link or token.
use std::collections::BTreeMap;

use meta_language::{LinkNetwork, LinkType, NetworkProjection, ParseConfiguration};

fn main() {
    let path = std::env::args().nth(1).expect("source path");
    let source = std::fs::read_to_string(path).expect("source");
    let network = LinkNetwork::parse(&source, "rust", ParseConfiguration::default());
    let mut histogram: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_type: BTreeMap<String, usize> = BTreeMap::new();
    for link in network.links() {
        *by_type.entry(format!("{:?}", link.metadata().link_type())).or_insert(0) += 1;
    }
    for link in network.projected_links(NetworkProjection::AbstractSyntax) {
        let metadata = link.metadata();
        if metadata.link_type() != Some(LinkType::Syntax) || !metadata.is_named() {
            continue;
        }
        if let Some(kind) = metadata.term() {
            *histogram.entry(kind.to_owned()).or_insert(0) += 1;
        }
    }
    println!("text_preserved {}", network.reconstruct_text() == source);
    println!("clean {}", network.verify_full_match(None).is_clean());
    println!("total_link_count {}", network.len());
    println!("named_node_count {}", histogram.values().sum::<usize>());
    println!("distinct_node_kinds {}", histogram.len());
    for (kind, count) in &histogram {
        println!("  {kind} {count}");
    }
    if std::env::var_os("CENSUS_DUMP").is_some() {
        for link in network.links() {
            let t = link.metadata().link_type();
            let point = link.references() == [link.id()];
            if point || !matches!(t, Some(LinkType::Syntax | LinkType::Token | LinkType::Trivia | LinkType::Field)) {
                println!("dump {:?} {:?}", t, link);
            }
        }
    }
    if std::env::var_os("CENSUS_TOKENS").is_some() {
        for link in network.links() {
            if link.metadata().link_type() == Some(LinkType::Token) {
                let span = link.metadata().span().map(|s| (s.byte_range().start(), s.byte_range().end()));
                println!("token {:?} {:?} {:?} refs={}", span, link.metadata().term(), link.metadata().flags(), link.references().len());
            }
        }
    }
    for (link_type, count) in &by_type {
        println!("type {link_type} {count}");
    }
}
