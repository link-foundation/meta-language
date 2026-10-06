// Parses a synthetic TypeScript input of `units` lines with the native
// TypeScript grammar and the default parse options (so the default memory
// budget of 2,000,000 memo cells applies) and prints the time, the
// rejection, and the process's peak resident memory. Run it bounded:
//
//   (ulimit -v 6000000; timeout 600 node experiments/run-rust-experiment.mjs \
//     experiments/issue-195-memory/rust-default-budget/main.rs 400 2000 8000)
use std::time::Instant;

use meta_language::{FeatureParseOptions, compile_feature_grammar, parse_grammar_links};

const TYPESCRIPT: &str = include_str!("/tmp/gh-issue-solver-1791163594611/rust/src/data/native-grammars/typescript.lino");

fn peak_mib() -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .unwrap_or_default()
        .lines()
        .find(|line| line.starts_with("VmHWM:"))
        .and_then(|line| line.split_whitespace().nth(1))
        .and_then(|kb| kb.parse::<u64>().ok())
        .unwrap_or(0)
        / 1024
}

fn main() {
    let grammar = parse_grammar_links(TYPESCRIPT).expect("grammar");
    let parser = compile_feature_grammar(&grammar, None, FeatureParseOptions::default()).expect("compile");
    for units in std::env::args().skip(1).map(|n| n.parse::<usize>().expect("units")) {
        let source: String = (0..units)
            .map(|i| format!("export const table{i}: Map<string, Array<number>> = new Map([[\"a\", [1, 2, {i}]]]);\n"))
            .collect();
        let options = FeatureParseOptions { error_recovery: Some(true), accept_recovery: Some(true), ..FeatureParseOptions::default() };
        let start = Instant::now();
        let outcome = parser.parse_tree(source.as_bytes(), &options).expect("runs");
        println!(
            "{{\"units\":{units},\"bytes\":{},\"ok\":{},\"rejection\":{:?},\"seconds\":{:.1},\"peakMiB\":{}}}",
            source.len(),
            outcome.ok,
            outcome.rejection.map(|r| r.to_string()),
            start.elapsed().as_secs_f64(),
            peak_mib()
        );
    }
}
