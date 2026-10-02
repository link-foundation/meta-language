//! Requirement I195-GRAMMAR-NATIVE-JSON: the native merged JSON grammar,
//! parity/grammars/native/json.lino, builds the concrete syntax trees of the
//! tree-sitter-json oracle that still backs the default JSON parse.
//! parity/fixtures/native-grammars/json.json holds the corpus with the oracle
//! rows, which js/scripts/generate-native-grammar-fixtures.mjs generates; this
//! suite projects the Rust executor's trees with
//! `issue_195_native_grammar_rows.rs` and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-json.test.js does for JavaScript.

use meta_language::{
    FeatureParseOptions, SyntaxTree, parse_grammar_links, percent_decode_links_text,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{Rows, cases, leaves, parse, rebuilt, source, text};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/json.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/json.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/json.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-JSON",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-json",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native JSON fixture is JSON")
}

fn parser() -> meta_language::FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

#[test]
fn native_json_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native JSON grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("document"));
    for rule in [
        "document", "object", "pair", "array", "string", "number", "true", "false", "null",
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
        &["nativeJsonGrammarIsCanonicalLinks"],
        "native JSON grammar is canonical Links Notation",
    );
}

#[test]
fn native_json_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 30);
    for case in matches {
        let tree = parse(&parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
        assert_eq!(
            Value::Array(rows.rows(&tree, source(case))),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    observe(
        &["nativeJsonTreesMatchOracle"],
        "native JSON grammar builds the oracle rows",
    );
}

#[test]
fn native_json_grammar_accepts_merged_source_extensions() {
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
                .contains("RFC 8259")
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
            [0, null, "document", 1, 0, 4, ""],
            [1, null, "number", 1, 0, 4, ""]
        ])
    );
    observe(
        &["nativeJsonAcceptsMergedSourceExtensions"],
        "native JSON grammar accepts merged source extensions",
    );
}

#[test]
fn native_json_grammar_rejects_invalid_json() {
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
        &["nativeJsonRejectsInvalidInput"],
        "native JSON grammar rejects invalid JSON",
    );
}

#[test]
fn native_json_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches")
        .iter()
        .chain(cases(&fixture, "divergences"))
    {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(&parser, "\u{feff}[]").expect("a leading byte order mark is accepted");
    let SyntaxTree::Token {
        kind,
        text: leaf,
        start,
        end,
        ..
    } = leaves(&tree)[0]
    else {
        panic!("the byte order mark is a leaf");
    };
    assert_eq!(
        (kind.as_deref(), text(leaf), *start, *end),
        (Some("byte_order_mark"), "\u{feff}", 0, 3)
    );
    assert_eq!(
        percent_decode_links_text("%EF%BB%BF").expect("decodes"),
        "\u{feff}"
    );
    observe(
        &["nativeJsonTreesLossless"],
        "native JSON trees keep every byte",
    );
}
