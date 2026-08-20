//! Complexity guard for the parser boundary.
//!
//! Every built-in parser must stay near-linear in input size. A parser that
//! resolves byte offsets to `(row, column)` points by rescanning the source
//! turns a parse into `O(nodes * bytes)`, which is quadratic in file size and
//! made a 39 KB module take over ten seconds (issue #193). Re-identifying the
//! links of an incremental edit by searching the previous network for every
//! reparsed link is quadratic in the same way.
//!
//! The guards below run the same generated source at two sizes and compare the
//! cost per byte. A linear implementation keeps that column flat; a quadratic
//! one multiplies it by the size ratio, so the threshold sits far below the
//! quadratic signal and far above ordinary measurement noise.

use std::time::Duration;
use std::time::Instant;

use meta_language::{ByteRange, LinkNetwork, ParseConfiguration};

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

/// Languages the incremental guard covers.
///
/// One case per parser family is enough — every language shares the one
/// identifier-matching path — and an edit costs a parse plus the match, so the
/// list stays short to keep the suite fast.
const EDIT_CASES: &[ScalingCase] = &[
    ScalingCase {
        language: "rust",
        unit: rust_unit,
    },
    ScalingCase {
        language: "json",
        unit: json_unit,
    },
];

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

/// Time one edit appended to the end of the source.
///
/// Appending shifts no existing span, so every link of the previous network is
/// a candidate to keep its identifier and the match does its full work.
fn edit_duration(source: &str, language: &str) -> Duration {
    let mut network = LinkNetwork::parse(source, language, ParseConfiguration::default());
    let at = source.len();
    let started = Instant::now();
    let applied = network.apply_edit(ByteRange::new(at, at), "\n");
    let elapsed = started.elapsed();
    assert!(applied, "{language} edit must apply");
    elapsed
}

/// Fastest observed nanoseconds spent per source byte.
///
/// The casts only lose precision past 2^53 nanoseconds — about 104 days — and
/// past 2^53 source bytes, neither of which a test input reaches.
#[allow(clippy::cast_precision_loss)]
fn nanos_per_byte(source: &str, language: &str, work: fn(&str, &str) -> Duration) -> f64 {
    let best = (0..SAMPLES)
        .map(|_| work(source, language))
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

fn measure(case: &ScalingCase, work: fn(&str, &str) -> Duration) -> Measurement {
    let small = source_for(case, SMALL_UNITS);
    let large = source_for(case, SMALL_UNITS * SIZE_RATIO);
    let small_per_byte = nanos_per_byte(&small, case.language, work);
    let large_per_byte = nanos_per_byte(&large, case.language, work);
    Measurement {
        small_bytes: small.len(),
        large_bytes: large.len(),
        small_per_byte,
        large_per_byte,
    }
}

/// Asserts that `work` costs no more per byte on the large input than on the
/// small one, retrying to ride out a scheduling hiccup.
fn assert_scales_linearly(cases: &[ScalingCase], work: fn(&str, &str) -> Duration, subject: &str) {
    for case in cases {
        let mut reports = Vec::new();
        let passed = (0..ATTEMPTS).any(|_| {
            let measurement = measure(case, work);
            reports.push(measurement.report(case.language));
            measurement.growth() <= MAX_PER_BYTE_GROWTH
        });
        assert!(
            passed,
            "per-byte {subject} cost grew more than {MAX_PER_BYTE_GROWTH:.1}x when the input grew \
             {SIZE_RATIO}x, which is the signature of super-linear {subject}:\n  {}",
            reports.join("\n  ")
        );
    }
}

/// Parsing must stay near-linear: no parser may reintroduce a per-node rescan
/// of the whole source (issue #193).
#[test]
fn parsers_scale_linearly_with_input_size() {
    assert_scales_linearly(CASES, parse_duration, "parse");
}

/// Applying an edit must stay near-linear: re-identifying the reparsed links
/// may not reintroduce a search over the whole previous network (issue #193).
#[test]
fn incremental_edits_scale_linearly_with_network_size() {
    assert_scales_linearly(EDIT_CASES, edit_duration, "edit");
}
