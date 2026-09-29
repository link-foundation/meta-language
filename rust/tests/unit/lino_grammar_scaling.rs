//! Stack-safety and complexity guard for the built-in `LiNo` grammar, the
//! counterpart of `js/tests/lino-grammar-scaling.test.js`.
//!
//! Each input below made an earlier Links Notation parser overflow the stack or
//! read the document in superlinear time (the regressions relative-meta-logic
//! tracks against links-notation): deep parentheses, long runs of unclosed
//! groups or quotes, deep indentation, long delimiter runs and many failing
//! lines. Every input is parsed at two sizes and the cost per byte compared: a
//! linear parse keeps it flat and a quadratic one multiplies it by the size
//! ratio.

use std::time::{Duration, Instant};

use meta_language::{LinkNetwork, ParseConfiguration};

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

struct ScalingCase {
    name: &'static str,
    units: usize,
    source: fn(usize) -> String,
}

fn deep_parentheses(units: usize) -> String {
    format!("{}a{}\n", "(".repeat(units), ")".repeat(units))
}

fn unclosed_groups(units: usize) -> String {
    format!("{}\n", "(a ".repeat(units))
}

fn deep_indentation(units: usize) -> String {
    let lines: Vec<String> = (0..units)
        .map(|depth| format!("{}a", " ".repeat(depth % (LINO_MAX_DEPTH - 4))))
        .collect();
    format!("{}\n", lines.join("\n"))
}

fn indented_identifiers(units: usize) -> String {
    "id:\n  a\n  b\n    c\n".repeat(units)
}

fn long_delimiter_run(units: usize) -> String {
    format!("{}x{}\n", "\"".repeat(units), "\"".repeat(units))
}

fn unclosed_quotes(units: usize) -> String {
    format!("{}\n", "\"a ".repeat(units))
}

fn comments_and_groups(units: usize) -> String {
    "a: b c # note\n  child (x y)\n".repeat(units)
}

fn failing_indented_lines(units: usize) -> String {
    format!("x\n{}", "  (broken\n".repeat(units))
}

const LINO_SCALING_CASES: &[ScalingCase] = &[
    ScalingCase {
        name: "deep parentheses",
        units: 500,
        source: deep_parentheses,
    },
    ScalingCase {
        name: "unclosed groups",
        units: 500,
        source: unclosed_groups,
    },
    ScalingCase {
        name: "deep indentation",
        units: 500,
        source: deep_indentation,
    },
    ScalingCase {
        name: "indented identifiers",
        units: 250,
        source: indented_identifiers,
    },
    ScalingCase {
        name: "long delimiter run",
        units: 2000,
        source: long_delimiter_run,
    },
    ScalingCase {
        name: "unclosed quotes",
        units: 500,
        source: unclosed_quotes,
    },
    ScalingCase {
        name: "comments and groups",
        units: 250,
        source: comments_and_groups,
    },
    ScalingCase {
        name: "failing indented lines",
        units: 100,
        source: failing_indented_lines,
    },
];

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
    for case in LINO_SCALING_CASES {
        let small = (case.source)(case.units);
        let large = (case.source)(case.units * SIZE_RATIO);
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
            "{}: per-byte parse cost grew more than {MAX_PER_BYTE_GROWTH:.1}x:\n  {}",
            case.name,
            reports.join("\n  ")
        );
    }
}

fn has_error_node(network: &LinkNetwork) -> bool {
    network.links().any(|link| {
        let metadata = link.metadata();
        metadata.term() == Some("ERROR") && metadata.flags().is_error()
    })
}

#[test]
fn links_nested_deeper_than_the_official_limit_become_an_error_not_a_stack_overflow() {
    let too_deep = deep_parentheses(100_000);
    let network = parse(&too_deep);
    assert_eq!(network.reconstruct_text(), too_deep);
    assert!(has_error_node(&network));
    let deepest = deep_parentheses(LINO_MAX_DEPTH);
    assert!(!has_error_node(&parse(&deepest)));
}
