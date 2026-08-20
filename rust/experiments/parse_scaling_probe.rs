// Measures how the cost of a parse grows with input size, per language.
//
// A parser that resolves byte offsets by rescanning the source is
// `O(nodes * bytes)`, which shows up here as a per-byte column that keeps
// climbing as the input doubles (issue #193). A linear parser keeps that
// column flat.
//
//   cp experiments/parse_scaling_probe.rs examples/
//   cargo run --release --quiet --example parse_scaling_probe
//   rm examples/parse_scaling_probe.rs
use std::time::Duration;
use std::time::Instant;

use meta_language::{LinkNetwork, ParseConfiguration};

const UNIT_COUNTS: &[usize] = &[16, 32, 64, 128, 256, 512];
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

fn python_unit(index: usize) -> String {
    format!(
        "def item_{index}(value):\n\
         \x20   total = value + {index}\n\
         \x20   return total\n\n"
    )
}

fn json_unit(index: usize) -> String {
    format!("    {{ \"name\": \"item {index}\", \"value\": {index} }},\n")
}

fn csv_unit(index: usize) -> String {
    format!("item {index},{index},\"quoted, value {index}\"\n")
}

fn lino_unit(index: usize) -> String {
    format!("(item{index}: source{index} target{index})\n")
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

fn parse_duration(source: &str, language: &str) -> Duration {
    let started = Instant::now();
    let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
    let elapsed = started.elapsed();
    assert_eq!(network.reconstruct_text(), source, "{language} stays lossless");
    elapsed
}

fn main() {
    let cases: &[(&str, fn(usize) -> String)] = &[
        ("rust", rust_unit),
        ("python", python_unit),
        ("json", json_unit),
        ("csv", csv_unit),
        ("lino", lino_unit),
        ("English", english_unit),
    ];

    for (language, unit) in cases {
        println!("=== {language} ===");
        println!("{:>8}  {:>10}  {:>12}", "bytes", "millis", "ns/byte");
        for units in UNIT_COUNTS {
            let source = source_for(language, *unit, *units);
            let best = (0..SAMPLES)
                .map(|_| parse_duration(&source, language))
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
