//! Requirement I195-GRAMMAR-NATIVE-INI: the native merged INI grammar,
//! parity/grammars/native/ini.lino, builds the concrete syntax trees of the
//! tree-sitter-ini oracle the native grammar replaced as the default INI parse.
//! parity/fixtures/native-grammars/ini.json holds the corpus with the oracle
//! rows, which js/scripts/generate-native-grammar-fixtures.mjs generates; this
//! suite projects the Rust executor's trees with
//! `issue_195_native_grammar_rows.rs` and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-ini.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, parse, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/ini.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/ini.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/ini.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-INI",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-ini",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native INI fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

#[test]
fn native_ini_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native INI grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("document"));
    for rule in ["document", "section", "section_name", "setting", "comment"] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    assert!(GRAMMAR.trim_end().lines().all(|line| {
        ["(grammar ", "(extra ", "(rule "]
            .iter()
            .any(|prefix| line.starts_with(prefix))
    }));
    assert!(!GRAMMAR.contains("tree-sitter") && !GRAMMAR.contains("grammar.js"));
    observe(
        &["nativeIniGrammarIsCanonicalLinks"],
        "native INI grammar is canonical Links Notation",
    );
}

#[test]
fn native_ini_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 50);
    for case in matches {
        let tree = parse(&parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
        assert_eq!(
            Value::Array(rows.rows(&tree, source(case))),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // A comment the oracle parses as an extra is an X row wherever it stands.
    let commented = matches
        .iter()
        .find(|case| source(case) == "[a]\nk=v\n; c\n[b]\n")
        .expect("the commented case");
    let comments: Vec<&Value> = commented["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter(|row| row[2] == "comment")
        .collect();
    assert_eq!(comments, [&json!([1, null, "comment", 1, 8, 12, "X"])]);
    observe(
        &["nativeIniTreesMatchOracle"],
        "native INI grammar builds the oracle rows",
    );
}

#[test]
fn native_ini_grammar_accepts_merged_source_extensions() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let divergences = cases(&fixture, "divergences");
    assert!(divergences.len() >= 2);
    for case in divergences {
        assert!(
            case["reason"]
                .as_str()
                .expect("reason")
                .contains("configparser")
        );
        let tree = parse(&parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
        assert_eq!(
            Value::Array(rows.rows(&tree, source(case))),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    assert_eq!(
        divergences[0]["rows"],
        json!([
            [0, null, "document", 1, 0, 3, ""],
            [1, null, "comment", 1, 0, 3, "X"],
            [2, null, "text", 1, 1, 3, ""]
        ])
    );
    observe(
        &["nativeIniAcceptsMergedSourceExtensions"],
        "native INI grammar accepts merged source extensions",
    );
}

#[test]
fn native_ini_grammar_rejects_invalid_ini() {
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
        &["nativeIniRejectsInvalidInput"],
        "native INI grammar rejects invalid INI",
    );
}

#[test]
fn native_ini_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches")
        .iter()
        .chain(cases(&fixture, "divergences"))
    {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(&parser, " \n; c\nk = v\r\n").expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted INI tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("blank_space"), " "),
            (Some("newline"), "\n"),
            (Some("comment_marker"), ";"),
            (Some("text"), " c"),
            (Some("newline"), "\n"),
            (Some("setting_name"), "k"),
            (None, " "),
            (None, "="),
            (Some("setting_value"), " v\r"),
            (Some("newline"), "\n"),
        ]
    );
    observe(
        &["nativeIniTreesLossless"],
        "native INI trees keep every byte",
    );
}

#[test]
fn pinned_ini_oracle_gives_the_fixture() {
    // The oracle is a development dependency since the native grammar
    // replaced it as the default INI parse.
    oracle_agrees(&tree_sitter_ini::LANGUAGE.into(), &fixture());
}
