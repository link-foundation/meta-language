//! Stack-safety and complexity guard for the built-in `LiNo` grammar, the
//! counterpart of `js/tests/lino-grammar-scaling.test.js`.
//!
//! Each regression of `parity/fixtures/lino-compatibility-matrix.json` made an
//! earlier Links Notation parser overflow the stack or read the document in
//! superlinear time (the findings relative-meta-logic reported against
//! links-notation): lone carriage returns, deep parentheses, long runs of
//! unclosed groups or quotes, deep indentation, long delimiter runs and many
//! failing lines. Every input is parsed at two sizes and the cost per byte
//! compared: a linear parse keeps it flat and a quadratic one multiplies it by
//! the size ratio.

use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use meta_language::{LinkNetwork, ParseConfiguration};
use serde_json::Value;

use super::issue_195_observations::{LINO_MATRIX_FIXTURE, Observation, record};

/// Ratio between the large and the small input.
const SIZE_RATIO: usize = 8;

/// Highest tolerated growth of the per-byte cost between the two sizes; linear
/// parsing lands near `1.0`, quadratic parsing near `SIZE_RATIO`.
const MAX_PER_BYTE_GROWTH: f64 = 3.0;

/// Timing samples per measurement; the fastest one counts.
const SAMPLES: usize = 3;

/// Measurements tried before a case is reported as superlinear.
const ATTEMPTS: usize = 3;

/// Deepest nesting links-notation 0.22 accepts.
const LINO_MAX_DEPTH: usize = 64;

fn matrix() -> Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(LINO_MATRIX_FIXTURE);
    serde_json::from_str(&fs::read_to_string(path).expect("matrix")).expect("matrix JSON")
}

fn field<'a>(shape: &'a Value, key: &str) -> &'a str {
    shape[key]
        .as_str()
        .unwrap_or_else(|| panic!("{key} is a string"))
}

/// The source of a matrix input shape holding `units` units.
fn shape_source(shape: &Value, units: usize) -> String {
    match field(shape, "kind") {
        "repeat" => format!(
            "{}{}{}",
            field(shape, "head"),
            field(shape, "unit").repeat(units),
            field(shape, "tail")
        ),
        "nest" => format!(
            "{}{}{}{}",
            field(shape, "open").repeat(units),
            field(shape, "middle"),
            field(shape, "close").repeat(units),
            field(shape, "tail")
        ),
        "staircase" => {
            let modulo =
                usize::try_from(shape["modulo"].as_u64().expect("modulo")).expect("modulo fits");
            let lines: Vec<String> = (0..units)
                .map(|depth| format!("{}{}", " ".repeat(depth % modulo), field(shape, "text")))
                .collect();
            format!("{}\n", lines.join("\n"))
        }
        kind => panic!("unknown input shape {kind}"),
    }
}

fn units(value: &Value) -> usize {
    usize::try_from(value["units"].as_u64().expect("units")).expect("units fit")
}

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-LINO-UPSTREAM-REGRESSIONS",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-lino-upstream-regressions",
        fixture_file: LINO_MATRIX_FIXTURE,
        assertions,
        test_name,
    });
}

/// The whole `LiNo` parse: the grammar CST and the links-notation semantics.
fn parse(source: &str) -> LinkNetwork {
    LinkNetwork::parse(source, "LiNo", ParseConfiguration::default())
}

fn parse_duration(source: &str) -> Duration {
    let started = Instant::now();
    let network = parse(source);
    let elapsed = started.elapsed();
    assert_eq!(
        network.reconstruct_text(),
        source,
        "the parse stays lossless"
    );
    elapsed
}

/// Fastest observed nanoseconds spent per source byte.
///
/// The casts only lose precision past 2^53 nanoseconds and past 2^53 source
/// bytes, neither of which a test input reaches.
#[allow(clippy::cast_precision_loss)]
fn nanos_per_byte(source: &str) -> f64 {
    let best = (0..SAMPLES)
        .map(|_| parse_duration(source))
        .min()
        .expect("at least one timing sample");
    best.as_nanos() as f64 / source.len() as f64
}

#[test]
fn lino_grammar_reads_every_regression_input_in_linear_time() {
    let matrix = matrix();
    for case in matrix["regressions"].as_array().expect("regressions") {
        let name = field(case, "name");
        let small = shape_source(&case["shape"], units(case));
        let large = shape_source(&case["shape"], units(case) * SIZE_RATIO);
        let mut reports = Vec::new();
        let passed = (0..ATTEMPTS).any(|_| {
            let small_cost = nanos_per_byte(&small);
            let large_cost = nanos_per_byte(&large);
            let growth = large_cost / small_cost;
            reports.push(format!(
                "{small_cost:.0} ns/byte at {} bytes, {large_cost:.0} ns/byte at {} bytes, \
                 growth {growth:.2}x",
                small.len(),
                large.len(),
            ));
            growth <= MAX_PER_BYTE_GROWTH
        });
        assert!(
            passed,
            "{name}: per-byte parse cost grew more than {MAX_PER_BYTE_GROWTH:.1}x:\n  {}",
            reports.join("\n  ")
        );
    }
    observe(
        &["everyRegressionInputLossless", "everyRegressionInputLinear"],
        "lino_grammar_reads_every_regression_input_in_linear_time",
    );
}

fn has_error_node(network: &LinkNetwork) -> bool {
    network.links().any(|link| {
        let metadata = link.metadata();
        metadata.term() == Some("ERROR") && metadata.flags().is_error()
    })
}

#[test]
fn links_nested_deeper_than_the_official_limit_become_an_error_not_a_stack_overflow() {
    let matrix = matrix();
    assert_eq!(matrix["maximumDepth"].as_u64(), Some(LINO_MAX_DEPTH as u64));
    let excessive = &matrix["excessiveNesting"];
    let too_deep = shape_source(&excessive["shape"], units(excessive));
    let network = parse(&too_deep);
    assert_eq!(network.reconstruct_text(), too_deep);
    assert!(has_error_node(&network));
    let deepest = shape_source(&excessive["shape"], LINO_MAX_DEPTH);
    assert!(!has_error_node(&parse(&deepest)));
    observe(
        &["excessiveNestingIsErrorNode"],
        "links_nested_deeper_than_the_official_limit_become_an_error_not_a_stack_overflow",
    );
}
