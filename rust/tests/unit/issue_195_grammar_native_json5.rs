//! Requirement I195-GRAMMAR-NATIVE-JSON5: the native merged JSON5 grammar,
//! parity/grammars/native/json5.lino, builds the concrete syntax trees of the
//! tree-sitter-json5-orchard oracle the native grammar replaced as the default JSON5 parse.
//! parity/fixtures/native-grammars/json5.json holds the corpus with the
//! oracle rows, which js/scripts/generate-native-grammar-fixtures.mjs
//! generates; this suite projects the Rust executor's trees with
//! `issue_195_native_grammar_rows.rs` and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-json5.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, parse, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/json5.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/json5.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/json5.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-JSON5",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-json5",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native JSON5 fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

#[test]
fn native_json5_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native JSON5 grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("file"));
    for rule in [
        "file",
        "object",
        "member",
        "identifier",
        "array",
        "string",
        "number",
        "null",
        "true",
        "false",
        "comment",
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
        &["nativeJson5GrammarIsCanonicalLinks"],
        "native JSON5 grammar is canonical Links Notation",
    );
}

#[test]
fn native_json5_grammar_builds_the_oracle_rows() {
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
    // A comment after the value is an extra row of the file.
    let commented = matches
        .iter()
        .find(|case| source(case) == "[1,]//c\n")
        .expect("the commented case");
    assert_eq!(
        commented["rows"].as_array().expect("rows").last(),
        Some(&json!([1, null, "comment", 1, 4, 7, "X"]))
    );
    observe(
        &["nativeJson5TreesMatchOracle"],
        "native JSON5 grammar builds the oracle rows",
    );
}

#[test]
fn native_json5_grammar_accepts_merged_source_extensions() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let divergences = cases(&fixture, "divergences");
    assert!(divergences.len() >= 20);
    for case in divergences {
        let reason = case["reason"].as_str().expect("reason");
        assert!(
            reason.contains("tree-sitter-json5-orchard 0.1.0")
                && reason.contains("JSON5 1.0.0 section")
        );
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // A \u escape in an unquoted name stays inside the identifier.
    let escaped = divergences
        .iter()
        .find(|case| source(case) == "{\\u0061:1}")
        .expect("the escaped name case");
    assert_eq!(
        escaped["rows"][4],
        json!([3, "name", "identifier", 1, 1, 7, ""])
    );
    observe(
        &["nativeJson5AcceptsMergedSourceExtensions"],
        "native JSON5 grammar accepts merged source extensions",
    );
}

#[test]
fn native_json5_grammar_rejects_invalid_input() {
    let fixture = fixture();
    let parser = parser();
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 20);
    for case in rejections {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    observe(
        &["nativeJson5RejectsInvalidInput"],
        "native JSON5 grammar rejects invalid input",
    );
}

#[test]
fn native_json5_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches")
        .iter()
        .chain(cases(&fixture, "divergences"))
    {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(&parser, "\u{feff}{a: 'x', // c\n\"b\":[0x1F,],}").expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .filter_map(|leaf| match leaf {
            SyntaxTree::Token {
                kind,
                trivia,
                text: leaf,
                ..
            } => (kind.is_some() || !trivia).then(|| (kind.as_deref(), text(leaf))),
            other => panic!("an accepted JSON5 tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (None, "{"),
            (Some("identifier"), "a"),
            (None, ":"),
            (Some("string"), "'x'"),
            (None, ","),
            (Some("comment"), "// c"),
            (Some("string"), "\"b\""),
            (None, ":"),
            (None, "["),
            (Some("number"), "0x1F"),
            (None, ","),
            (None, "]"),
            (None, ","),
            (None, "}"),
        ]
    );
    observe(
        &["nativeJson5TreesLossless"],
        "native JSON5 trees keep every byte",
    );
}

#[test]
fn pinned_json5_oracle_gives_the_fixture() {
    // The oracle is a development dependency since the native grammar
    // replaced it as the default JSON5 parse.
    oracle_agrees(&tree_sitter_json5_orchard::LANGUAGE.into(), &fixture());
}
