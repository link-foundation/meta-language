// Dumps the whole network an incremental edit produces, plus the time the edit
// took, for a corpus of languages and edit shapes.
//
// `apply_edit` rebuilds identifiers by matching every reparsed link against
// every old link, which is quadratic in network size: a 15 KB source took over
// twenty seconds for a one-character edit. Replacing that scan with an index
// must keep the resulting network identical, link for link, so this probe is
// meant to be run on the commit before the change and on the commit after it:
//
//   cp experiments/incremental_edit_probe.rs examples/
//   cargo run --release --quiet --example incremental_edit_probe > /tmp/edits-after.txt
//   diff /tmp/edits-before.txt /tmp/edits-after.txt
//
// Timings differ between runs, so diff the `--quiet` output with the timing
// lines filtered out (`grep -v '^time '`).
use std::time::Instant;

use meta_language::{ByteRange, LinkNetwork, ParseConfiguration};

struct EditCase {
    language: &'static str,
    source: &'static str,
    /// Text to find in the source; the edit replaces its first occurrence.
    target: &'static str,
    replacement: &'static str,
}

const CASES: &[EditCase] = &[
    EditCase {
        language: "JavaScript",
        source: "let alpha = 1;\nlet beta = alpha + 1;\n",
        target: "alpha",
        replacement: "gamma",
    },
    EditCase {
        language: "JavaScript",
        source: "let alpha = 1;\nlet beta = 2;\n",
        target: "",
        replacement: "let prefix = 0;\n",
    },
    EditCase {
        language: "Rust",
        source: "fn main() {\n    let value = compute(1);\n}\n\nfn compute(x: usize) -> usize {\n    x + 1\n}\n",
        target: "compute(1)",
        replacement: "compute(2)",
    },
    EditCase {
        language: "Rust",
        source: "fn main() {\n    let s = \"héllo\";\n}\n",
        target: "héllo",
        replacement: "wörld",
    },
    EditCase {
        language: "Rust",
        source: "fn a() {}\nfn b() {}\n",
        target: "fn b() {}\n",
        replacement: "",
    },
    EditCase {
        language: "Python",
        source: "def item(value):\n    return value + 1\n",
        target: "+ 1",
        replacement: "* 2",
    },
    EditCase {
        language: "JSON",
        source: "{\n  \"key\": [1, 2, 3],\n  \"other\": null\n}\n",
        target: "null",
        replacement: "true",
    },
    EditCase {
        language: "Lino",
        source: "(item: source target)\n(other: a b)\n",
        target: "source",
        replacement: "origin",
    },
    EditCase {
        language: "Markdown",
        source: "# Title\n\nText here.\n\n```rust\nfn item() {}\n```\n",
        target: "item",
        replacement: "entry",
    },
    EditCase {
        language: "CSV",
        source: "name,value\nfirst,1\nsecond,2\n",
        target: "first",
        replacement: "third",
    },
];

fn edit_range(source: &str, target: &str) -> ByteRange {
    if target.is_empty() {
        return ByteRange::new(0, 0);
    }
    let start = source.find(target).expect("target occurs in the source");
    ByteRange::new(start, start + target.len())
}

fn main() {
    for case in CASES {
        let mut network = LinkNetwork::parse(case.source, case.language, ParseConfiguration::default());
        let range = edit_range(case.source, case.target);
        let started = Instant::now();
        let applied = network.apply_edit(range, case.replacement);
        let elapsed = started.elapsed();

        println!(
            "=== {} {:?} -> {:?} applied={applied} ===",
            case.language, case.target, case.replacement
        );
        println!("time {:.2} ms", elapsed.as_secs_f64() * 1000.0);
        println!("text {:?}", network.reconstruct_text());
        for link in network.links() {
            let span = link.metadata().span();
            println!(
                "{} refs={:?} {:?} named={} term={:?} lang={:?} bytes={:?} start={:?} end={:?}",
                link.id().as_u64(),
                link.references().iter().map(|id| id.as_u64()).collect::<Vec<_>>(),
                link.metadata().link_type(),
                link.metadata().is_named(),
                link.metadata().term(),
                link.metadata().language(),
                span.map(|span| (span.byte_range().start(), span.byte_range().end())),
                span.map(|span| (span.start_point().row(), span.start_point().column())),
                span.map(|span| (span.end_point().row(), span.end_point().column())),
            );
        }
    }
}
