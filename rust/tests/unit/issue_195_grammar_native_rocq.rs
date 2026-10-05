//! Requirement I195-GRAMMAR-NATIVE-ROCQ: the native merged Rocq grammar,
//! parity/grammars/native/rocq.lino, which js/scripts/import-native-grammars.mjs
//! imports from the pinned tree-sitter-rocq grammar, builds the concrete
//! syntax trees of the tree-sitter oracle the native grammar replaced as the
//! default Rocq parse. parity/fixtures/native-grammars/rocq.json holds the
//! upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-rocq.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::Value;

use super::issue_195_native_grammar_rows::{Rows, cases, leaves, parse, rebuilt, source, text};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/rocq.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/rocq.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/rocq.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-ROCQ",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-rocq",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Rocq fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

#[test]
fn native_rocq_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Rocq grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("source_file"));
    for rule in [
        "source_file",
        "sentence",
        "attributes",
        "require_command",
        "import_command",
        "evaluation_command",
        "theorem_command",
        "definition_command",
        "fixpoint_command",
        "inductive_command",
        "constructor",
        "record_command",
        "section_command",
        "module_command",
        "notation_command",
        "ltac_definition",
        "proof_block",
        "application",
        "lambda_function",
        "let_expression",
        "match_expression",
        "if_expression",
        "list_literal",
        "binder",
        "implicit_binders",
        "tactic_sequence",
        "generic_tactic",
        "intro_pattern",
        "qualified_identifier",
        "identifier",
        "integer",
        "string",
        "comment",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // tree-sitter-rocq has no external scanner; a comment nests through its
    // own rule.
    assert!(!GRAMMAR.lines().any(|line| line.starts_with("(scanner ")));
    assert!(GRAMMAR.trim_end().lines().all(|line| {
        [
            "(grammar ",
            "(extra ",
            "(conflict ",
            "(precedences ",
            "(scanner ",
            "(rule ",
            "(kind ",
        ]
        .iter()
        .any(|prefix| line.starts_with(prefix))
    }));
    // A renamed rule or an aliased kind keeps its tree-sitter name only as a
    // source-name alias.
    let foreign = regex::Regex::new(r" \(source-names(?: \([^()]*\))+\)")
        .expect("the source-names pattern compiles")
        .replace_all(GRAMMAR, "");
    let generator = regex::Regex::new(r"\bgrammar\.js\b").expect("the generator pattern compiles");
    assert!(
        !foreign.contains("tree-sitter")
            && !foreign.contains("(regex ")
            && !foreign.contains("module.exports")
            && !generator.is_match(&foreign)
    );
    observe(
        &["nativeRocqGrammarIsCanonicalLinks"],
        "native Rocq grammar is canonical Links Notation",
    );
}

#[test]
fn native_rocq_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 220);
    for case in matches {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(outcome.ok, "{:?}", source(case));
        assert!(outcome.ambiguities.is_empty(), "{:?}", source(case));
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // The renamed `identifier` rule keeps the oracle kind `ident`.
    assert_eq!(fixture["oracleKinds"]["identifier"], "ident");
    assert_eq!(cases(&fixture, "divergences"), [] as [Value; 0]);
    observe(
        &["nativeRocqTreesMatchOracle"],
        "native Rocq grammar builds the oracle rows",
    );
}

#[test]
fn native_rocq_grammar_rejects_invalid_input() {
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
        &["nativeRocqRejectsInvalidInput"],
        "native Rocq grammar rejects invalid input",
    );
}

#[test]
fn native_rocq_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(
        &parser,
        "(* a (* b *) *)\nDefinition f (x : nat) : nat := S x.\n",
    )
    .expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted Rocq tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (None, "(*"),
            (Some("unnamed_token"), " a "),
            (None, "(*"),
            (Some("unnamed_token"), " b "),
            (None, "*)"),
            (Some("unnamed_token"), " "),
            (None, "*)"),
            (None, "\n"),
            (None, "Definition"),
            (None, " "),
            (Some("identifier"), "f"),
            (None, " "),
            (None, "("),
            (Some("identifier"), "x"),
            (None, " "),
            (None, ":"),
            (None, " "),
            (Some("identifier"), "nat"),
            (None, ")"),
            (None, " "),
            (None, ":"),
            (None, " "),
            (Some("identifier"), "nat"),
            (None, " "),
            (None, ":="),
            (None, " "),
            (Some("identifier"), "S"),
            (None, " "),
            (Some("identifier"), "x"),
            (None, "."),
            (None, "\n"),
        ]
    );
    observe(
        &["nativeRocqTreesLossless"],
        "native Rocq trees keep every byte",
    );
}

#[test]
fn native_rocq_trees_follow_the_oracle_on_a_byte_order_mark_and_an_end_after_a_match() {
    let fixture = fixture();
    let rows = Rows::new(&fixture);
    let parser = parser();
    // The tree-sitter-rocq oracle rows of js/tests/issue-195-grammar-native-rocq.test.js:
    // a tree-sitter lexer skips a byte order mark at the start of the input,
    // and `end` after the `end` of a match is no keyword of the closed match.
    for (source, oracle) in [
        (
            "\u{feff}Check x.",
            serde_json::json!([
                [0, null, "source_file", 1, 3, 11, ""],
                [1, null, "sentence", 1, 3, 11, ""],
                [2, null, "evaluation_command", 1, 3, 10, ""],
                [3, null, "Check", 0, 3, 8, ""],
                [3, null, "ident", 1, 9, 10, ""],
                [2, null, ".", 0, 10, 11, ""]
            ]),
        ),
        (
            "Check match x with y => y end > 0 end.",
            serde_json::json!([
                [0, null, "source_file", 1, 0, 38, ""],
                [1, null, "sentence", 1, 0, 38, ""],
                [2, null, "evaluation_command", 1, 0, 37, ""],
                [3, null, "Check", 0, 0, 5, ""],
                [3, null, "comparison_operation", 1, 6, 37, ""],
                [4, null, "match_expression", 1, 6, 29, ""],
                [5, null, "match", 0, 6, 11, ""],
                [5, null, "case_item", 1, 12, 13, ""],
                [6, null, "ident", 1, 12, 13, ""],
                [5, null, "with", 0, 14, 18, ""],
                [5, null, "match_case", 1, 19, 25, ""],
                [6, null, "pattern_option", 1, 19, 20, ""],
                [7, "pattern", "ident", 1, 19, 20, ""],
                [6, null, "=>", 0, 21, 23, ""],
                [6, "body", "ident", 1, 24, 25, ""],
                [5, null, "end", 0, 26, 29, ""],
                [4, null, ">", 0, 30, 31, ""],
                [4, null, "application", 1, 32, 37, ""],
                [5, null, "number", 1, 32, 33, ""],
                [5, null, "ident", 1, 34, 37, ""],
                [2, null, ".", 0, 37, 38, ""]
            ]),
        ),
    ] {
        let tree = parse(&parser, source).unwrap_or_else(|| panic!("{source:?}"));
        assert_eq!(Value::Array(rows.rows(&tree, source)), oracle, "{source:?}");
    }
    // An unclosed nested comment is no comment.
    let outcome = parser
        .parse_tree(b"(* Outer (* Inner *)", &FeatureParseOptions::default())
        .expect("the parse runs");
    assert!(!outcome.ok);
    observe(
        &["nativeRocqTreesMatchOracle", "nativeRocqRejectsInvalidInput"],
        "native Rocq trees follow the oracle on a byte order mark and an end after a match",
    );
}
