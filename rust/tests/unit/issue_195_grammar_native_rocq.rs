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
    assert!(cases(&fixture, "divergences").is_empty());
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
