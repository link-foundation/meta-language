//! Times the Markdown parse of the parse_scaling guard at 16 and 128 units, to
//! tell super-linear parsing from load-induced noise. Copy into rust/examples
//! and run `cargo run --release --example issue_195_markdown_scaling [units...]`.
use std::time::Instant;

use meta_language::{LinkNetwork, ParseConfiguration};

fn unit(index: usize) -> String {
    format!(
        "## Section {index}\n\nParagraph {index} of the document.\n\n```rust\npub fn item_{index}() -> usize {{ {index} }}\n```\n\n"
    )
}

fn main() {
    let sizes: Vec<usize> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let sizes = if sizes.is_empty() { vec![16, 32, 64, 128, 256] } else { sizes };
    for units in sizes {
        let source: String = (0..units).map(unit).collect();
        let best = (0..3)
            .map(|_| {
                let started = Instant::now();
                let network = LinkNetwork::parse(&source, "Markdown", ParseConfiguration::default());
                let elapsed = started.elapsed();
                assert_eq!(network.reconstruct_text(), source);
                elapsed
            })
            .min()
            .unwrap();
        println!(
            "{units:4} units {:6} bytes {:9.1} ns/byte",
            source.len(),
            best.as_nanos() as f64 / source.len() as f64
        );
    }
}
