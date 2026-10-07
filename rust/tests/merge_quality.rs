//! Requirement I195-MERGE-QUALITY-EVIDENCE: the time and memory of the Rust
//! native executor on the corpus of every native grammar, and the time of the
//! pinned tree-sitter oracle on the same corpus where its grammar crate is a
//! development dependency. A test target of its own, since the counting
//! allocator that measures the peak of live heap bytes is global to the
//! binary. With `MERGE_QUALITY_PRINT` set it prints one `merge-quality` JSON
//! line per grammar, which `js/scripts/build-merge-quality-evidence.mjs
//! --measure` writes to parity/fixtures/merge-quality-measurements.json; the
//! deterministic part of the evidence is checked by
//! `tests/unit/issue_195_merge_quality_evidence.rs`.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, compile_feature_grammar, parse_grammar_links,
};
use serde_json::{Value, json};

#[allow(dead_code)]
#[path = "unit/issue_195_observations.rs"]
mod issue_195_observations;

use issue_195_observations::{Observation, record};

/// The system allocator, counting the live heap bytes and their peak.
struct CountingAllocator;

static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

impl CountingAllocator {
    fn grew(size: usize) {
        let live = LIVE.fetch_add(size, Ordering::Relaxed) + size;
        PEAK.fetch_max(live, Ordering::Relaxed);
    }
}

// SAFETY: every call forwards to `System` with the caller's arguments; the
// counters only observe the sizes.
#[allow(unsafe_code)]
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwarded unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            Self::grew(layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: forwarded unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            Self::grew(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        // SAFETY: forwarded unchanged.
        unsafe { System.dealloc(pointer, layout) };
        LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: forwarded unchanged.
        let moved = unsafe { System.realloc(pointer, layout, new_size) };
        if !moved.is_null() {
            LIVE.fetch_sub(layout.size(), Ordering::Relaxed);
            Self::grew(new_size);
        }
        moved
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

/// The fixture of the requirement; the measurements are its published part.
const FIXTURE_FILE: &str = "parity/fixtures/merge-quality-evidence.json";
const MEASUREMENTS: &str = include_str!("../../parity/fixtures/merge-quality-measurements.json");

/// A native grammar: its id, links, corpus fixture and, where its grammar
/// crate is a development dependency, the oracle.
struct Native {
    id: &'static str,
    links: &'static str,
    fixture: &'static str,
    oracle: Option<fn() -> tree_sitter::Language>,
}

macro_rules! native {
    ($id:literal, $file:literal, $oracle:expr) => {
        Native {
            id: $id,
            links: include_str!(concat!("../../parity/grammars/native/", $file, ".lino")),
            fixture: include_str!(concat!(
                "../../parity/fixtures/native-grammars/",
                $file,
                ".json"
            )),
            oracle: $oracle,
        }
    };
}

fn natives() -> Vec<Native> {
    vec![
        native!("native-c", "c", Some(|| tree_sitter_c::LANGUAGE.into())),
        native!("native-csv", "csv", None),
        native!(
            "native-diff",
            "diff",
            Some(|| tree_sitter_diff::LANGUAGE.into())
        ),
        native!("native-go", "go", Some(|| tree_sitter_go::LANGUAGE.into())),
        native!(
            "native-graphql",
            "graphql",
            Some(|| tree_sitter_graphql::LANGUAGE.into())
        ),
        native!(
            "native-ini",
            "ini",
            Some(|| tree_sitter_ini::LANGUAGE.into())
        ),
        native!(
            "native-java",
            "java",
            Some(|| tree_sitter_java::LANGUAGE.into())
        ),
        native!(
            "native-javascript",
            "javascript",
            Some(|| tree_sitter_javascript::LANGUAGE.into())
        ),
        native!(
            "native-json",
            "json",
            Some(|| tree_sitter_json::LANGUAGE.into())
        ),
        native!(
            "native-json5",
            "json5",
            Some(|| tree_sitter_json5_orchard::LANGUAGE.into())
        ),
        // The Lean oracle is a vendored grammar private to the crate, not a
        // development dependency, so its Rust row measures the native side.
        native!("native-lean", "lean", None),
        native!(
            "native-lua",
            "lua",
            Some(|| tree_sitter_lua::LANGUAGE.into())
        ),
        native!(
            "native-make",
            "make",
            Some(|| tree_sitter_make::LANGUAGE.into())
        ),
        native!(
            "native-proto",
            "proto",
            Some(|| tree_sitter_proto::LANGUAGE.into())
        ),
        native!(
            "native-racket",
            "racket",
            Some(|| tree_sitter_racket::LANGUAGE.into())
        ),
        native!(
            "native-regex",
            "regex",
            Some(|| tree_sitter_regex::LANGUAGE.into())
        ),
        // The Rocq oracle, like the Lean one, is a vendored grammar private to
        // the crate.
        native!("native-rocq", "rocq", None),
        native!("native-rust", "rust", None),
        native!(
            "native-scheme",
            "scheme",
            Some(|| tree_sitter_scheme::LANGUAGE.into())
        ),
        native!(
            "native-solidity",
            "solidity",
            Some(|| tree_sitter_solidity::LANGUAGE.into())
        ),
        native!(
            "native-tsx",
            "tsx",
            Some(|| tree_sitter_typescript::LANGUAGE_TSX.into())
        ),
        native!(
            "native-typescript",
            "typescript",
            Some(|| tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        ),
    ]
}

fn sources<'a>(fixture: &'a Value, key: &str) -> Vec<&'a str> {
    fixture[key]
        .as_array()
        .expect("fixture cases")
        .iter()
        .map(|case| case["source"].as_str().expect("case source"))
        .collect()
}

/// Milliseconds of a pass of `parse` over `sources`, after a pass to warm up.
fn timed(sources: &[&str], mut parse: impl FnMut(&str)) -> f64 {
    for source in sources {
        parse(source);
    }
    let start = Instant::now();
    for source in sources {
        parse(source);
    }
    (start.elapsed().as_secs_f64() * 1000.0 * 1000.0).round() / 1000.0
}

fn native_side(native: &Native, matches: &[&str], rejections: &[&str]) -> Value {
    let base = LIVE.load(Ordering::Relaxed);
    PEAK.store(base, Ordering::Relaxed);
    let grammar = parse_grammar_links(native.links).expect("the native grammar reads");
    let parser: FeatureGrammarParser =
        compile_feature_grammar(&grammar, None, FeatureParseOptions::default())
            .expect("the native grammar compiles");
    let accept = FeatureParseOptions::default();
    let parse_ms = timed(matches, |source| {
        let outcome = parser
            .parse_tree(source.as_bytes(), &accept)
            .expect("the parse runs");
        assert!(outcome.ok, "{} rejects {source:?}", native.id);
    });
    let repair = FeatureParseOptions {
        error_recovery: Some(true),
        ..FeatureParseOptions::default()
    };
    let recover_ms = timed(rejections, |source| {
        let outcome = parser
            .parse_tree(source.as_bytes(), &repair)
            .expect("the recovery runs");
        assert!(outcome.tree.is_some(), "{} repairs {source:?}", native.id);
    });
    let peak = PEAK.load(Ordering::Relaxed).saturating_sub(base);
    json!({ "parseMs": parse_ms, "recoverMs": recover_ms, "peakHeapBytes": peak })
}

fn oracle_side(language: &tree_sitter::Language, matches: &[&str], rejections: &[&str]) -> Value {
    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language).expect("the oracle loads");
    let mut parse = |source: &str| {
        parser.parse(source, None).expect("the oracle parses");
    };
    let parse_ms = timed(matches, &mut parse);
    let recover_ms = timed(rejections, &mut parse);
    json!({ "parseMs": parse_ms, "recoverMs": recover_ms })
}

#[test]
fn issue_195_merge_quality_time_and_memory_are_measured() {
    let print = std::env::var_os("MERGE_QUALITY_PRINT").is_some();
    // `--measure --only` measures one grammar.
    let only = std::env::var("MERGE_QUALITY_ONLY").ok();
    let published: Value = serde_json::from_str(MEASUREMENTS).expect("the measurements are JSON");
    let published = published["grammars"].as_array().expect("measured grammars");
    let natives = natives();
    // `--measure` prints before the measurements are published.
    assert!(
        print
            || published
                .iter()
                .map(|entry| entry["grammar"].as_str().expect("grammar"))
                .collect::<Vec<_>>()
                == natives.iter().map(|native| native.id).collect::<Vec<_>>(),
        "every native grammar is measured"
    );
    for (index, native) in natives.iter().enumerate() {
        if print && only.as_deref().is_some_and(|only| only != native.id) {
            continue;
        }
        // `--measure` measures every grammar; the test measures the smallest
        // corpus again, as the JavaScript test does, and checks the others
        // are published.
        if print || native.id == "native-json" {
            let fixture: Value = serde_json::from_str(native.fixture).expect("the fixture is JSON");
            let matches = sources(&fixture, "matches");
            let rejections = sources(&fixture, "rejections");
            let measured = native_side(native, &matches, &rejections);
            assert!(measured["parseMs"].as_f64() > Some(0.0), "{}", native.id);
            assert!(
                measured["peakHeapBytes"].as_u64() > Some(0),
                "{}",
                native.id
            );
            let oracle = native
                .oracle
                .map(|language| oracle_side(&language(), &matches, &rejections));
            if print {
                println!(
                    "merge-quality {}",
                    json!({ "grammar": native.id, "native": measured, "oracle": oracle })
                );
                continue;
            }
            assert!(
                oracle.is_some_and(|oracle| oracle["parseMs"].as_f64() > Some(0.0)),
                "{}",
                native.id
            );
        }
        // The published measurements of both runtimes have every side.
        let entry = &published[index];
        for (runtime, side, key) in [
            ("javascript", "native", "peakKiB"),
            ("javascript", "oracle", "peakKiB"),
            ("rust", "native", "peakHeapBytes"),
        ] {
            assert!(
                entry[runtime][side]["parseMs"].as_f64() > Some(0.0),
                "{} {runtime} {side}",
                native.id
            );
            assert!(
                entry[runtime][side][key].as_f64().is_some(),
                "{} {runtime} {side}",
                native.id
            );
        }
        assert_eq!(
            entry["rust"]["oracle"].is_null(),
            native.oracle.is_none(),
            "{}",
            native.id
        );
    }
    record(&Observation {
        requirement_id: "I195-MERGE-QUALITY-EVIDENCE",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-merge-quality-evidence",
        fixture_file: FIXTURE_FILE,
        assertions: &["timeAndMemoryMeasured"],
        test_name: "merge quality time and memory are measured",
    });
}
