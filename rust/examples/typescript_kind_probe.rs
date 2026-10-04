//! Compares the named syntax kinds of TypeScript files under the default
//! (native) grammar and the pinned tree-sitter-typescript oracle, the
//! inventory a downstream projection coverage table counts. Each named kind
//! one side has and the other lacks is printed with the first source it
//! covers, with the parse time of each side.
//!
//!   cargo run --example `typescript_kind_probe` -- <file>...
use std::collections::BTreeMap;
use std::time::Instant;

use meta_language::{LinkNetwork, LinkType, ParseConfiguration};

fn native_kinds(text: &str) -> BTreeMap<String, String> {
    let network = LinkNetwork::parse(text, "typescript", ParseConfiguration::default());
    let mut kinds = BTreeMap::new();
    for link in network.links() {
        let metadata = link.metadata();
        if metadata.link_type() == Some(LinkType::Syntax) && metadata.is_named() {
            let sample = metadata
                .span()
                .map(meta_language::SourceSpan::byte_range)
                .and_then(|range| text.get(range.start()..range.end().min(range.start() + 80)))
                .unwrap_or("")
                .to_owned();
            kinds
                .entry(metadata.term().unwrap_or("?").to_owned())
                .or_insert(sample);
        }
    }
    kinds
}

fn oracle_kinds(text: &str) -> BTreeMap<String, String> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .expect("the oracle loads");
    let tree = parser.parse(text, None).expect("the oracle parses");
    let mut kinds = BTreeMap::new();
    let mut cursor = tree.walk();
    let mut stack = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        if node.is_named() {
            kinds.entry(node.kind().to_owned()).or_insert_with(|| {
                text[node.start_byte()..node.end_byte().min(node.start_byte() + 80)].to_owned()
            });
        }
        stack.extend(node.children(&mut cursor));
    }
    kinds
}

fn main() {
    for file in std::env::args().skip(1) {
        let text = std::fs::read_to_string(&file).expect("a readable file");
        let started = Instant::now();
        let oracle = oracle_kinds(&text);
        let oracle_ms = started.elapsed().as_millis();
        let started = Instant::now();
        let native = native_kinds(&text);
        let native_ms = started.elapsed().as_millis();
        println!(
            "{file}: {} bytes, oracle {oracle_ms} ms, native {native_ms} ms",
            text.len()
        );
        for (kind, sample) in &native {
            if !oracle.contains_key(kind) {
                println!("  native only {kind}: {sample:?}");
            }
        }
        for (kind, sample) in &oracle {
            if !native.contains_key(kind) {
                println!("  oracle only {kind}: {sample:?}");
            }
        }
    }
}
