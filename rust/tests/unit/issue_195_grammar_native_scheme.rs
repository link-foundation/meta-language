//! Requirement I195-GRAMMAR-NATIVE-SCHEME: the native merged Scheme grammar,
//! parity/grammars/native/scheme.lino, builds the concrete syntax trees of the
//! tree-sitter-scheme oracle the native grammar replaced as the default Scheme parse.
//! parity/fixtures/native-grammars/scheme.json holds the corpus with the
//! oracle rows, which js/scripts/generate-native-grammar-fixtures.mjs
//! generates; this suite projects the Rust executor's trees with
//! `issue_195_native_grammar_rows.rs` and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-scheme.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, parse, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/scheme.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/scheme.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/scheme.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-SCHEME",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-scheme",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Scheme fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

fn case<'a>(cases: &'a [Value], wanted: &str) -> &'a Value {
    cases
        .iter()
        .find(|case| source(case) == wanted)
        .unwrap_or_else(|| panic!("the {wanted:?} case"))
}

#[test]
fn native_scheme_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Scheme grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("program"));
    for rule in [
        "program",
        "comment",
        "directive",
        "block_comment",
        "boolean",
        "number",
        "character",
        "string",
        "escape_sequence",
        "symbol",
        "keyword",
        "list",
        "quote",
        "quasiquote",
        "unquote",
        "unquote_splicing",
        "syntax",
        "quasisyntax",
        "unsyntax",
        "unsyntax_splicing",
        "vector",
        "byte_vector",
        "datum_label",
        "datum_reference",
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
        &["nativeSchemeGrammarIsCanonicalLinks"],
        "native Scheme grammar is canonical Links Notation",
    );
}

#[test]
fn native_scheme_grammar_builds_the_oracle_rows() {
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
    // The longer of a number and a symbol wins, the number on a tie.
    assert_eq!(
        case(matches, "1#a")["rows"],
        json!([
            [0, null, "program", 1, 0, 3, ""],
            [1, null, "number", 1, 0, 2, ""],
            [1, null, "symbol", 1, 2, 3, ""]
        ])
    );
    observe(
        &["nativeSchemeTreesMatchOracle"],
        "native Scheme grammar builds the oracle rows",
    );
}

#[test]
fn native_scheme_grammar_accepts_merged_source_extensions() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let divergences = cases(&fixture, "divergences");
    assert!(divergences.len() >= 20);
    for case in divergences {
        let reason = case["reason"].as_str().expect("reason");
        assert!(
            reason.contains("tree-sitter-scheme 0.24.7") && reason.contains("R7RS small section")
        );
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // A datum label wraps the datum it labels; a reference is one token.
    let labelled = case(divergences, "#12=(a . #12#)");
    assert_eq!(
        labelled["rows"][1],
        json!([1, null, "datum_label", 1, 0, 14, ""])
    );
    assert_eq!(
        labelled["rows"][7],
        json!([3, null, "datum_reference", 1, 9, 13, ""])
    );
    observe(
        &["nativeSchemeAcceptsMergedSourceExtensions"],
        "native Scheme grammar accepts merged source extensions",
    );
}

#[test]
fn native_scheme_grammar_rejects_invalid_input() {
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
        &["nativeSchemeRejectsInvalidInput"],
        "native Scheme grammar rejects invalid input",
    );
}

#[test]
fn native_scheme_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches")
        .iter()
        .chain(cases(&fixture, "divergences"))
    {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(
        &parser,
        "#!r6rs ; c\n(define x #u8(1 \"a\\n\") #| b |# 1abc)",
    )
    .expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted Scheme tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (None, "#!"),
            (Some("directive_name"), "r6rs"),
            (Some("whitespace"), " "),
            (Some("comment_text"), "; c"),
            (Some("whitespace"), "\n"),
            (None, "("),
            (Some("symbol"), "define"),
            (Some("whitespace"), " "),
            (Some("symbol"), "x"),
            (Some("whitespace"), " "),
            (None, "#u8("),
            (Some("number"), "1"),
            (Some("whitespace"), " "),
            (None, "\""),
            (Some("string_text"), "a"),
            (Some("escape_sequence"), "\\n"),
            (None, "\""),
            (None, ")"),
            (Some("whitespace"), " "),
            (None, "#|"),
            (Some("comment_text"), " b "),
            (None, "|#"),
            (Some("whitespace"), " "),
            (Some("symbol"), "1abc"),
            (None, ")"),
        ]
    );
    observe(
        &["nativeSchemeTreesLossless"],
        "native Scheme trees keep every byte",
    );
}

#[test]
fn pinned_scheme_oracle_gives_the_fixture() {
    // The oracle is a development dependency since the native grammar
    // replaced it as the default Scheme parse.
    oracle_agrees(&tree_sitter_scheme::LANGUAGE.into(), &fixture());
}
