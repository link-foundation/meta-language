//! Optional matching caches preserve the unchanged parse memory budget.
use super::issue_195_native_grammar_rows::{Rows, oracle_rows, parser, rebuilt};
use meta_language::FeatureParseOptions;
use serde_json::Value;

const FIXTURE: &str = include_str!("../../../parity/fixtures/lexical-cache-memory.json");

#[test]
fn optional_lexical_cache_reservations_preserve_the_unchanged_memory_limit() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    for entry in fixture["reservations"].as_array().unwrap() {
        assert_eq!(
            meta_language::translation::frontend_rules::accept_lexical_cache_reservation(
                entry["cells"].as_f64().unwrap(),
                entry["count"].as_f64().unwrap(),
                entry["limit"].as_f64().unwrap(),
            ),
            entry["accepted"].as_bool().unwrap(),
            "{entry}"
        );
    }
}

#[test]
fn long_lexical_extras_preserve_the_tree_when_optional_caches_exhaust_a_small_budget() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let parser = parser(fixture["grammar"].as_str().unwrap());
    let source = format!(
        "#{}\nx;",
        "a".repeat(fixture["commentLength"].as_u64().unwrap() as usize)
    );
    let limited = FeatureParseOptions {
        memory_limit: Some(fixture["memoryLimit"].as_u64().unwrap() as usize),
        ..FeatureParseOptions::default()
    };
    let expected = parser
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap();
    let actual = parser.parse_tree(source.as_bytes(), &limited).unwrap();
    assert!(expected.ok);
    assert!(actual.ok, "{:?}", actual.rejection);
    assert_eq!(actual.tree, expected.tree);
    assert_eq!(actual.ambiguities, expected.ambiguities);
    assert_eq!(rebuilt(actual.tree.as_ref().unwrap()), source);
    let exhausted = parser
        .parse_tree(
            source.as_bytes(),
            &FeatureParseOptions {
                memory_limit: Some(0),
                ..FeatureParseOptions::default()
            },
        )
        .unwrap();
    assert_eq!(exhausted.rejection.unwrap().reason, "memoryBudget");
    let invalid = parser
        .parse_tree(&source.as_bytes()[..source.len() - 1], &limited)
        .unwrap();
    assert_eq!(invalid.rejection.unwrap().reason, "syntax");
}

#[test]
fn original_zig_diagnostic_comment_ci_input_fits_the_unchanged_default_memory_budget() {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/native-grammars/zig-corpus.json"
    ))
    .unwrap();
    let selected = corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| {
            case["file"]
                .as_str()
                .unwrap()
                .ends_with("undef_arith_is_illegal.zig")
        })
        .unwrap();
    let source = selected["source"].as_str().unwrap();
    assert_eq!(source.len(), 495499);
    let parser = parser(include_str!("../../../parity/grammars/native/zig.lino"));
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/native-grammars/zig.json"
    ))
    .unwrap();
    let (expected, recovers) = oracle_rows(&tree_sitter_zig::LANGUAGE.into(), source);
    assert!(!recovers);
    let actual = parser
        .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
        .unwrap();
    assert!(actual.ok, "{:?}", actual.rejection);
    assert!(actual.ambiguities.is_empty());
    let tree = actual.tree.unwrap();
    assert_eq!(Rows::new(&fixture).rows(&tree, source), expected);
    assert_eq!(rebuilt(&tree), source);
}
