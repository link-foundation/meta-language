//! Complexity guard for the parser boundary.
//!
//! Every built-in parser must stay near-linear in input size. A parser that
//! resolves byte offsets to `(row, column)` points by rescanning the source
//! turns a parse into `O(nodes * bytes)`, which is quadratic in file size and
//! made a 39 KB module take over ten seconds (issue #193).
//!
//! The guard below parses the same generated source at two sizes and compares
//! the cost per byte. A linear parser keeps that column flat; a quadratic one
//! multiplies it by the size ratio, so the threshold sits far below the
//! quadratic signal and far above ordinary measurement noise.

use std::time::Duration;
use std::time::Instant;

use meta_language::{LinkNetwork, ParseConfiguration};

/// Ratio between the large and the small input.
const SIZE_RATIO: usize = 8;

/// Units in the small input; the large input holds `SIZE_RATIO` times as many.
const SMALL_UNITS: usize = 16;

/// Highest tolerated growth of the per-byte cost between the two sizes.
///
/// Linear parsing lands near `1.0`, quadratic parsing near `SIZE_RATIO`
/// (`8.0`). Fixed per-parse overhead — grammar loading, self-description setup
/// — weighs more on the small input, which pushes the measured ratio *down*,
/// so the check only fires on genuine super-linear growth.
const MAX_PER_BYTE_GROWTH: f64 = 3.0;

/// Timing samples per size; the fastest one is used, since noise only ever adds.
const SAMPLES: usize = 3;

/// Retries of the whole comparison before a run is reported as a regression.
const ATTEMPTS: usize = 3;

struct ScalingCase {
    language: &'static str,
    unit: fn(usize) -> String,
}

const CASES: &[ScalingCase] = &[
    ScalingCase {
        language: "rust",
        unit: rust_unit,
    },
    ScalingCase {
        language: "python",
        unit: python_unit,
    },
    ScalingCase {
        language: "json",
        unit: json_unit,
    },
    ScalingCase {
        language: "csv",
        unit: csv_unit,
    },
    ScalingCase {
        language: "lino",
        unit: lino_unit,
    },
    ScalingCase {
        language: "Markdown",
        unit: markdown_unit,
    },
    ScalingCase {
        language: "HTML",
        unit: html_unit,
    },
    ScalingCase {
        language: "English",
        unit: english_unit,
    },
];

fn rust_unit(index: usize) -> String {
    format!(
        "/// Doc comment for item {index}.\n\
         pub fn item_{index}(input: &str) -> usize {{\n\
         \x20   let trimmed = input.trim();\n\
         \x20   if trimmed.is_empty() {{\n\
         \x20       return {index};\n\
         \x20   }}\n\
         \x20   trimmed.len() + {index}\n\
         }}\n\n"
    )
}

fn python_unit(index: usize) -> String {
    format!(
        "def item_{index}(value):\n\
         \x20   \"\"\"Item {index}.\"\"\"\n\
         \x20   total = value + {index}\n\
         \x20   if total > {index}:\n\
         \x20       return total\n\
         \x20   return {index}\n\n"
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

/// Exercises mixed-region detection and the parse of an embedded region.
fn markdown_unit(index: usize) -> String {
    format!(
        "## Section {index}\n\n\
         Paragraph {index} of the document.\n\n\
         ```rust\n\
         pub fn item_{index}() -> usize {{ {index} }}\n\
         ```\n\n"
    )
}

/// Exercises region detection over element bodies and style attributes.
fn html_unit(index: usize) -> String {
    format!(
        "<p style=\"color: rgb({index}, 0, 0)\">Paragraph {index}.</p>\n\
         <span>Item {index}</span>\n"
    )
}

fn english_unit(index: usize) -> String {
    format!("The parser reads sentence number {index} and records its tokens. ")
}

fn source_for(case: &ScalingCase, units: usize) -> String {
    let body: String = (0..units).map(case.unit).collect();
    if case.language == "json" {
        format!("[\n{}\n]\n", body.trim_end_matches(",\n"))
    } else {
        body
    }
}

fn parse_duration(source: &str, language: &str) -> Duration {
    let started = Instant::now();
    let network = LinkNetwork::parse(source, language, ParseConfiguration::default());
    let elapsed = started.elapsed();
    assert_eq!(
        network.reconstruct_text(),
        source,
        "{language} parse must stay lossless"
    );
    elapsed
}

/// Fastest observed nanoseconds spent per source byte.
///
/// The casts only lose precision past 2^53 nanoseconds — about 104 days — and
/// past 2^53 source bytes, neither of which a test input reaches.
#[allow(clippy::cast_precision_loss)]
fn nanos_per_byte(source: &str, language: &str) -> f64 {
    let best = (0..SAMPLES)
        .map(|_| parse_duration(source, language))
        .min()
        .expect("at least one timing sample");
    best.as_nanos() as f64 / source.len() as f64
}

struct Measurement {
    small_bytes: usize,
    large_bytes: usize,
    small_per_byte: f64,
    large_per_byte: f64,
}

impl Measurement {
    fn growth(&self) -> f64 {
        self.large_per_byte / self.small_per_byte
    }

    fn report(&self, language: &str) -> String {
        format!(
            "{language}: {small_bytes} bytes at {small_per_byte:.1} ns/byte, \
             {large_bytes} bytes at {large_per_byte:.1} ns/byte, growth {growth:.2}x",
            small_bytes = self.small_bytes,
            small_per_byte = self.small_per_byte,
            large_bytes = self.large_bytes,
            large_per_byte = self.large_per_byte,
            growth = self.growth(),
        )
    }
}

fn measure(case: &ScalingCase) -> Measurement {
    let small = source_for(case, SMALL_UNITS);
    let large = source_for(case, SMALL_UNITS * SIZE_RATIO);
    let small_per_byte = nanos_per_byte(&small, case.language);
    let large_per_byte = nanos_per_byte(&large, case.language);
    Measurement {
        small_bytes: small.len(),
        large_bytes: large.len(),
        small_per_byte,
        large_per_byte,
    }
}

/// Parsing must stay near-linear: no parser may reintroduce a per-node rescan
/// of the whole source (issue #193).
#[test]
fn parsers_scale_linearly_with_input_size() {
    for case in CASES {
        let mut reports = Vec::new();
        let passed = (0..ATTEMPTS).any(|_| {
            let measurement = measure(case);
            reports.push(measurement.report(case.language));
            measurement.growth() <= MAX_PER_BYTE_GROWTH
        });
        assert!(
            passed,
            "per-byte parse cost grew more than {MAX_PER_BYTE_GROWTH:.1}x when the input grew \
             {SIZE_RATIO}x, which is the signature of a super-linear parse:\n  {}",
            reports.join("\n  ")
        );
    }
}
