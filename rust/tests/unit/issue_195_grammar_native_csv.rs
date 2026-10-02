//! Requirement I195-GRAMMAR-NATIVE-CSV: the native merged CSV grammar,
//! parity/grammars/native/csv.lino, builds the concrete syntax trees of the
//! tree-sitter-csv oracle that still backs the default CSV parse.
//! parity/fixtures/native-grammars/csv.json holds the corpus with the oracle
//! rows, which js/scripts/generate-native-grammar-fixtures.mjs generates; this
//! suite projects the Rust executor's trees with
//! `issue_195_native_grammar_rows.rs` and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-csv.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{Rows, cases, leaves, parse, rebuilt, source, text};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/csv.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/csv.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/csv.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-CSV",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-csv",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native CSV fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

#[test]
fn native_csv_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native CSV grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("document"));
    for rule in [
        "document", "row", "field", "number", "float", "boolean", "quoted", "text",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    assert!(GRAMMAR.trim_end().lines().all(|line| {
        ["(grammar ", "(extra ", "(rule "]
            .iter()
            .any(|prefix| line.starts_with(prefix))
    }));
    assert!(!GRAMMAR.contains("tree-sitter") && !GRAMMAR.contains("grammar.js"));
    observe(
        &["nativeCsvGrammarIsCanonicalLinks"],
        "native CSV grammar is canonical Links Notation",
    );
}

#[test]
fn native_csv_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 100);
    for case in matches {
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // A typed field keeps its kind, and a boolean shows its keyword.
    let typed = matches
        .iter()
        .find(|case| source(case) == "1.5,true,0x1F\n")
        .expect("the typed case");
    let outline: Vec<Value> = typed["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter(|row| row[0].as_u64() >= Some(3))
        .map(|row| json!([row[2], row[3], row[4], row[5]]))
        .collect();
    assert_eq!(
        outline,
        [
            json!(["float", 1, 0, 3]),
            json!(["boolean", 1, 4, 8]),
            json!(["true", 0, 4, 8]),
            json!(["number", 1, 9, 13]),
        ]
    );
    observe(
        &["nativeCsvTreesMatchOracle"],
        "native CSV grammar builds the oracle rows",
    );
}

#[test]
fn native_csv_grammar_accepts_merged_source_extensions() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let divergences = cases(&fixture, "divergences");
    assert!(divergences.len() >= 8);
    for case in divergences {
        let reason = case["reason"].as_str().expect("reason");
        assert!(reason.contains("tree-sitter-csv f6bf6e3") && reason.contains("RFC 4180"));
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // An empty last field is a field of empty text.
    let empty = divergences
        .iter()
        .find(|case| source(case) == "a,")
        .expect("the empty field case");
    assert_eq!(
        empty["rows"],
        json!([
            [0, null, "document", 1, 0, 2, ""],
            [1, null, "row", 1, 0, 2, ""],
            [2, null, "field", 1, 0, 1, ""],
            [3, null, "text", 1, 0, 1, ""],
            [2, null, ",", 0, 1, 2, ""],
            [2, null, "field", 1, 2, 2, ""],
            [3, null, "text", 1, 2, 2, ""]
        ])
    );
    observe(
        &["nativeCsvAcceptsMergedSourceExtensions"],
        "native CSV grammar accepts merged source extensions",
    );
}

#[test]
fn native_csv_grammar_rejects_invalid_quoting() {
    let fixture = fixture();
    let parser = parser();
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 10);
    for case in rejections {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    observe(
        &["nativeCsvRejectsInvalidInput"],
        "native CSV grammar rejects invalid quoting",
    );
}

#[test]
fn native_csv_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches")
        .iter()
        .chain(cases(&fixture, "divergences"))
    {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(&parser, " \"a\"\"b\" , 1\r\n\r\ntrue,\n").expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted CSV tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("text"), " \"a\"\"b\""),
            (Some("blank_space"), " "),
            (None, ","),
            (Some("number"), " 1"),
            (Some("newline"), "\r\n\r\n"),
            (None, "true"),
            (None, ","),
            (Some("text"), ""),
            (Some("newline"), "\n"),
        ]
    );
    observe(
        &["nativeCsvTreesLossless"],
        "native CSV trees keep every byte",
    );
}
