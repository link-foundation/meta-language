// Times the native parse of the generative cases of one language, slowest
// first, from the sources js/experiments/generative-case-timing.mjs writes.
//
//   (cd ../js && node experiments/generative-case-timing.mjs Lean 48 /tmp/lean-cases.json)
//   cp experiments/generative_case_timing.rs examples/
//   cargo run --quiet --example generative_case_timing -- /tmp/lean-cases.json
//   rm examples/generative_case_timing.rs
use std::time::Instant;

use meta_language::{LinkNetwork, ParseConfiguration};
use serde_json::Value;

fn main() {
    let path = std::env::args().nth(1).expect("cases JSON path");
    let cases: Value =
        serde_json::from_str(&std::fs::read_to_string(path).expect("cases")).expect("cases JSON");
    let language = cases["language"].as_str().expect("language");
    let mut timings = Vec::new();
    for case in cases["cases"].as_array().expect("cases") {
        let source = case["source"].as_str().expect("source");
        let started = Instant::now();
        let _ = LinkNetwork::parse(source, language, ParseConfiguration::default());
        timings.push((
            started.elapsed().as_millis(),
            case["label"].as_str().unwrap_or("").to_string(),
            source.len(),
        ));
    }
    timings.sort_by(|a, b| b.0.cmp(&a.0));
    let total: u128 = timings.iter().map(|timing| timing.0).sum();
    println!("{language}: {} parses, {total} ms", timings.len());
    for (ms, label, length) in timings.iter().take(15) {
        println!("{ms} ms  {label}  ({length} bytes)");
    }
}
