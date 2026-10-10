//! Requirement I195-MERGE-QUALITY-EVIDENCE: the published comparison of every
//! native grammar with the pinned tree-sitter grammar it was merged from. This
//! suite recomputes the deterministic part, coverage, preserved features,
//! correctness, recovery and shared reuse, in Rust and compares it with
//! parity/fixtures/merge-quality-evidence.json, which
//! `js/scripts/build-merge-quality-evidence.mjs` writes and
//! js/tests/issue-195-merge-quality-evidence.test.js recomputes in
//! JavaScript. The time and memory of the Rust executor are measured by the
//! `merge_quality` test target, `rust/tests/merge_quality.rs`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use meta_language::{RuleKind, native_grammar_concept_reuse, parse_grammar_links};
use serde_json::{Value, json};

use super::issue_195_observations::{Observation, record};

const FIXTURE: &str = "parity/fixtures/merge-quality-evidence.json";
const MEASUREMENTS: &str = "parity/fixtures/merge-quality-measurements.json";
const DOCUMENT: &str = "docs/grammar/merge-quality-evidence.md";
const INVENTORY: &str = "parity/language-grammar-inventory.json";
const RECOVERY: &str = "parity/fixtures/native-recovery.json";
const FEATURE_LINKS: [&str; 5] = ["scanner", "conflict", "precedences", "extra", "kind"];

fn path(file: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(file)
}

fn read(file: &str) -> String {
    std::fs::read_to_string(path(file)).unwrap_or_else(|error| panic!("{file} reads: {error}"))
}

fn read_json(file: &str) -> Value {
    serde_json::from_str(&read(file)).unwrap_or_else(|error| panic!("{file} is JSON: {error}"))
}

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-MERGE-QUALITY-EVIDENCE",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-merge-quality-evidence",
        fixture_file: FIXTURE,
        assertions,
        test_name,
    });
}

fn file_name(links: &str) -> &str {
    links
        .rsplit('/')
        .next()
        .and_then(|name| name.strip_suffix(".lino"))
        .expect("a links file")
}

fn strings(value: &Value) -> BTreeSet<&str> {
    value
        .as_array()
        .expect("a list")
        .iter()
        .map(|entry| entry.as_str().expect("a name"))
        .collect()
}

/// Whether `source` is the upstream test corpus at the pinned revision.
fn is_upstream_corpus(source: &str) -> bool {
    source.contains("/tree/") && source.ends_with("corpus")
}

fn count_of(value: &Value) -> usize {
    value.as_array().expect("a list").len()
}

/// The deterministic merge quality record of the native grammar `id`.
#[allow(clippy::too_many_lines)]
fn record_of(
    id: &str,
    links_file: &str,
    recovery: &Value,
    reuse: &BTreeMap<String, (usize, usize)>,
) -> Value {
    let links = read(links_file);
    let grammar = parse_grammar_links(&links).expect("the native grammar reads");
    let name = file_name(links_file);
    let fixture_file = format!("parity/fixtures/native-grammars/{name}.json");
    let fixture = read_json(&fixture_file);
    let report_file = format!("parity/grammars/merge-reports/{name}.json");
    let report = path(&report_file).exists().then(|| read_json(&report_file));

    let mut rules: BTreeMap<&str, usize> = ["normal", "atomic", "silent", "token"]
        .into_iter()
        .map(|kind| (kind, 0))
        .collect();
    for rule in grammar.rules() {
        *rules.get_mut(rule.kind.as_str()).expect("a rule kind") += 1;
    }
    let oracle_name = |rule: &str| {
        fixture["oracleKinds"][rule]
            .as_str()
            .unwrap_or(rule)
            .to_owned()
    };
    let unseen: BTreeSet<&str> = strings(&fixture["hidden"])
        .union(&strings(&fixture["anonymous"]))
        .copied()
        .collect();
    let visible: Vec<String> = grammar
        .rules()
        .iter()
        .filter(|rule| rule.kind == RuleKind::Normal && !unseen.contains(rule.name()))
        .map(|rule| oracle_name(rule.name()))
        .filter(|name| !name.starts_with('_'))
        .collect();
    let mut kinds = BTreeSet::new();
    let mut fields = BTreeSet::new();
    let mut rows = 0;
    let matches = fixture["matches"].as_array().expect("matches");
    for case in matches {
        let case_rows = case["rows"].as_array().expect("rows");
        rows += case_rows.len();
        for row in case_rows {
            if row[3] == 1 {
                kinds.insert(row[2].as_str().expect("a kind").to_owned());
            }
            if let Some(field) = row[1].as_str() {
                fields.insert(field.to_owned());
            }
        }
    }
    let exercised = visible.iter().filter(|name| kinds.contains(*name)).count();
    let mut heads: BTreeMap<&str, usize> =
        FEATURE_LINKS.into_iter().map(|head| (head, 0)).collect();
    for line in links.lines() {
        if let Some(head) = line
            .strip_prefix('(')
            .and_then(|rest| rest.split_once(' '))
            .map(|(head, _)| head)
            && let Some(count) = heads.get_mut(head)
        {
            *count += 1;
        }
    }
    let language = fixture["language"].as_str().expect("a language");
    let mut categories: BTreeMap<String, usize> = BTreeMap::new();
    for case in recovery["cases"].as_array().expect("recovery cases") {
        if case["language"] == language {
            *categories
                .entry(case["category"].as_str().expect("a category").to_owned())
                .or_default() += 1;
        }
    }
    let rejections = fixture["rejections"].as_array().expect("rejections");
    let recovered = |case: &Value| case["recovered"].as_str().unwrap_or_default().to_owned();
    let repaired = rejections
        .iter()
        .filter(|case| {
            let tree = recovered(case);
            tree.contains("(ERROR@") || tree.contains("(MISSING@")
        })
        .count();
    let nodes = |marker: &str| -> usize {
        rejections
            .iter()
            .map(|case| recovered(case).matches(marker).count())
            .sum()
    };
    let merge_count = |key: &str| report.as_ref().map(|report| count_of(&report[key]));
    let (shared, specific) = reuse[id];
    json!({
        "grammar": id,
        "language": language,
        "links": links_file,
        "fixture": fixture_file,
        "oracle": fixture["oracle"],
        "sources": fixture["sources"],
        "corpus": fixture["corpus"]["url"].as_str().map(Value::from).or_else(|| fixture["sources"]
            .as_array()
            .expect("sources")
            .iter()
            .find(|source| source.as_str().is_some_and(is_upstream_corpus)).cloned()),
        "mergeReport": report.as_ref().map(|_| report_file.clone()),
        "coverage": {
            "rules": rules,
            "visibleRules": visible.len(),
            "exercisedRules": exercised,
            "checkedKinds": kinds.len(),
            "checkedFields": fields.len(),
            "renamed": merge_count("renamed"),
            "expandedWords": merge_count("expandedWords"),
            "approximations": merge_count("approximations"),
            "unsupported": merge_count("unsupported"),
        },
        "features": heads,
        "correctness": {
            "matches": matches.len(),
            "rows": rows,
            "bytes": matches.iter().map(|case| case["source"].as_str().expect("a source").len()).sum::<usize>(),
            "divergences": count_of(&fixture["divergences"]),
            "rejections": rejections.len(),
        },
        "recovery": {
            "repaired": repaired,
            "errorNodes": nodes("(ERROR@"),
            "missingNodes": nodes("(MISSING@"),
            "categories": categories,
        },
        "reuse": { "shared": shared, "specific": specific },
    })
}

fn report() -> Vec<Value> {
    let inventory = read_json(INVENTORY);
    let recovery = read_json(RECOVERY);
    let reuse: BTreeMap<String, (usize, usize)> = native_grammar_concept_reuse()
        .grammars
        .iter()
        .map(|grammar| {
            (
                grammar.grammar.clone(),
                (grammar.shared.len(), grammar.specific.len()),
            )
        })
        .collect();
    // A `BTreeMap` orders the grammars by id, as the JavaScript report does.
    let natives: BTreeMap<&String, &Value> = inventory["nativeGrammars"]
        .as_object()
        .expect("native grammars")
        .iter()
        .collect();
    natives
        .into_iter()
        .map(|(id, entry)| {
            record_of(
                id,
                entry["grammar"].as_str().expect("a links file"),
                &recovery,
                &reuse,
            )
        })
        .collect()
}

#[test]
fn issue_195_merge_quality_evidence_rust_report_equals_the_published_report() {
    let report = report();
    let published = read_json(FIXTURE);
    assert_eq!(published["grammars"].as_array().expect("grammars"), &report);
    assert_eq!(report.len(), 38);
    for record in &report {
        let grammar = &record["grammar"];
        let coverage = &record["coverage"];
        let correctness = &record["correctness"];
        // Every grammar names the sources it merges: a URL, or a vendored copy.
        let sources = record["sources"].as_array().expect("sources");
        assert_ne!(sources.len(), 0, "{grammar}");
        for source in sources {
            let source = source.as_str().expect("a source");
            assert!(
                source.starts_with("https://") || path(source).exists(),
                "{source}"
            );
        }
        // A grammar the importer merges is checked on the upstream test corpus
        // of its source grammar at the pinned revision.
        if !record["mergeReport"].is_null() {
            let corpus = record["corpus"].as_str().expect("an upstream corpus");
            assert!(corpus.starts_with("https://github.com/"), "{grammar}");
        }
        assert!(coverage["exercisedRules"].as_u64() > Some(0), "{grammar}");
        assert!(coverage["checkedKinds"].as_u64() > Some(0), "{grammar}");
        assert!(correctness["matches"].as_u64() >= Some(30), "{grammar}");
        assert!(
            correctness["rows"].as_u64() > correctness["matches"].as_u64(),
            "{grammar}"
        );
        // Every rejection the oracle recovers from is repaired natively.
        assert_eq!(
            record["recovery"]["repaired"], correctness["rejections"],
            "{grammar}"
        );
        // A grammar the importer merged reports no approximation and no unsupported feature.
        if !record["mergeReport"].is_null() {
            assert_eq!(coverage["approximations"], 0, "{grammar}");
            assert_eq!(coverage["unsupported"], 0, "{grammar}");
        }
    }
    observe(
        &[
            "coverageMeasured",
            "correctnessMeasured",
            "recoveryMeasured",
        ],
        "merge quality evidence: the Rust report equals the published report",
    );
}

#[test]
fn issue_195_merge_quality_evidence_comparison_is_published_with_measurements() {
    let report = read_json(FIXTURE);
    let measurements = read_json(MEASUREMENTS);
    let document = read(DOCUMENT);
    let grammars: Vec<&Value> = report["grammars"]
        .as_array()
        .expect("grammars")
        .iter()
        .map(|record| &record["grammar"])
        .collect();
    let measured: Vec<&Value> = measurements["grammars"]
        .as_array()
        .expect("measured grammars")
        .iter()
        .map(|entry| &entry["grammar"])
        .collect();
    assert_eq!(measured, grammars);
    for entry in measurements["grammars"]
        .as_array()
        .expect("measured grammars")
    {
        let grammar = &entry["grammar"];
        for side in ["native", "oracle"] {
            assert!(
                entry["javascript"][side]["parseMs"].as_f64() > Some(0.0),
                "{grammar} {side}"
            );
            assert!(
                entry["javascript"][side]["recoverMs"].as_f64() > Some(0.0),
                "{grammar} {side}"
            );
            assert!(
                entry["javascript"][side]["peakKiB"].is_u64(),
                "{grammar} {side}"
            );
        }
        assert!(
            entry["rust"]["native"]["parseMs"].as_f64() > Some(0.0),
            "{grammar}"
        );
        assert!(
            entry["rust"]["native"]["peakHeapBytes"].as_u64() > Some(0),
            "{grammar}"
        );
    }
    for heading in [
        "## Corpora",
        "## Correctness",
        "## Coverage",
        "## Recovery",
        "## Time and memory",
    ] {
        assert!(document.contains(heading), "{heading}");
    }
    for grammar in grammars {
        let grammar = grammar.as_str().expect("a grammar id");
        assert!(document.contains(&format!("| `{grammar}` |")), "{grammar}");
    }
    observe(
        &["comparisonPublished"],
        "merge quality evidence: the comparison is published with measurements",
    );
}
