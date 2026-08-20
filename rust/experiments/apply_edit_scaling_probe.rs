// Measures how the cost of one incremental edit grows with network size.
//
// `apply_edit` reparses the edited source and then has to decide which
// reparsed links keep their previous identifier. Searching every old link for
// every new one is `O(old * new)`, which shows up here as a per-byte column
// that keeps climbing as the input doubles (issue #193). An indexed match keeps
// that column flat.
//
//   cp experiments/apply_edit_scaling_probe.rs examples/
//   cargo run --release --quiet --example apply_edit_scaling_probe
//   rm examples/apply_edit_scaling_probe.rs
use std::time::Duration;
use std::time::Instant;

use meta_language::{ByteRange, LinkNetwork, ParseConfiguration};

const UNIT_COUNTS: &[usize] = &[16, 32, 64, 128, 256];
const SAMPLES: usize = 3;

fn rust_unit(index: usize) -> String {
    format!(
        "/// Doc comment for item {index}.\n\
         pub fn item_{index}(input: &str) -> usize {{\n\
         \x20   let trimmed = input.trim();\n\
         \x20   trimmed.len() + {index}\n\
         }}\n\n"
    )
}

fn json_unit(index: usize) -> String {
    format!("    {{ \"name\": \"item {index}\", \"value\": {index} }},\n")
}

fn english_unit(index: usize) -> String {
    format!("The parser reads sentence number {index} and records its tokens. ")
}

fn source_for(language: &str, unit: fn(usize) -> String, units: usize) -> String {
    let body: String = (0..units).map(unit).collect();
    if language == "json" {
        format!("[\n{}\n]\n", body.trim_end_matches(",\n"))
    } else {
        body
    }
}

/// Time one edit at the very end of the source, where no span shifts, so the
/// measurement is dominated by the identifier match rather than by the reparse.
fn edit_duration(source: &str, language: &str) -> Duration {
    let mut network = LinkNetwork::parse(source, language, ParseConfiguration::default());
    let at = source.len();
    let started = Instant::now();
    let applied = network.apply_edit(ByteRange::new(at, at), "\n");
    let elapsed = started.elapsed();
    assert!(applied, "{language} edit applies");
    elapsed
}

fn main() {
    let cases: &[(&str, fn(usize) -> String)] = &[
        ("rust", rust_unit),
        ("json", json_unit),
        ("English", english_unit),
    ];

    for (language, unit) in cases {
        println!("=== {language} ===");
        println!("{:>8}  {:>10}  {:>12}", "bytes", "millis", "ns/byte");
        for units in UNIT_COUNTS {
            let source = source_for(language, *unit, *units);
            let best = (0..SAMPLES)
                .map(|_| edit_duration(&source, language))
                .min()
                .expect("at least one sample");
            println!(
                "{:>8}  {:>10.1}  {:>12.1}",
                source.len(),
                best.as_secs_f64() * 1000.0,
                best.as_nanos() as f64 / source.len() as f64,
            );
        }
    }
}
