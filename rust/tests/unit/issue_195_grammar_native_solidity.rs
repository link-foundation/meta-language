//! Requirement I195-GRAMMAR-NATIVE-SOLIDITY: the native merged Solidity grammar,
//! parity/grammars/native/solidity.lino, which
//! js/scripts/import-native-grammars.mjs imports from the pinned
//! tree-sitter-solidity 1.2.13 grammar, builds the concrete syntax trees of the
//! tree-sitter oracle the native grammar replaced as the default Solidity
//! parse.
//! parity/fixtures/native-grammars/solidity.json holds the
//! upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-solidity.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::Value;

use super::issue_195_native_grammar_rows::{Rows, cases, parse, rebuilt, source};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/solidity.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/solidity.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/solidity.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-SOLIDITY",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-solidity",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Solidity fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

/// The kind of each named leaf, the kind of the node it is a child of, its
/// text and whether it is trivia there.
fn placed(
    tree: &SyntaxTree,
    text: &str,
    parent: Option<&str>,
    found: &mut Vec<(String, String, String, bool)>,
) {
    match tree {
        SyntaxTree::Node { kind, children, .. } => {
            for child in children {
                placed(child, text, Some(kind), found);
            }
        }
        SyntaxTree::Token {
            kind: Some(kind),
            trivia,
            start,
            end,
            ..
        } => found.push((
            kind.clone(),
            parent.expect("a leaf has a parent").to_owned(),
            text[*start..*end].to_owned(),
            *trivia,
        )),
        SyntaxTree::Token { kind: None, .. } => {}
        other => panic!("an accepted Solidity tree has no {other:?}"),
    }
}

#[test]
fn native_solidity_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Solidity grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("source_file"));
    for rule in [
        "source_file",
        "pragma_directive",
        "import_directive",
        "contract_declaration",
        "interface_declaration",
        "library_declaration",
        "error_declaration",
        "structure_declaration",
        "enumeration_declaration",
        "event_definition",
        "using_directive",
        "assembly_statement",
        "yul_block",
        "yul_function_definition",
        "block_statement",
        "if_statement",
        "for_statement",
        "while_statement",
        "do_while_statement",
        "revert_statement",
        "try_statement",
        "catch_clause",
        "emit_statement",
        "state_variable_declaration",
        "modifier_definition",
        "constructor_definition",
        "function_definition",
        "call_arguments",
        "expression",
        "ternary_expression",
        "binary_expression",
        "call_expression",
        "parenthesized_expression",
        "mapping",
        "primitive_type",
        "identifier",
        "comment",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // tree-sitter-solidity has no external scanner.
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
            && !foreign.contains("module.exports")
            && !generator.is_match(&foreign)
    );
    observe(
        &["nativeSolidityGrammarIsCanonicalLinks"],
        "native Solidity grammar is canonical Links Notation",
    );
}

#[test]
fn native_solidity_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    // Every case of the upstream corpus, which the oracle reads.
    assert!(matches.len() >= 125);
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
    assert_eq!(cases(&fixture, "divergences"), [] as [Value; 0]);
    observe(
        &["nativeSolidityTreesMatchOracle"],
        "native Solidity grammar builds the oracle rows",
    );
}

#[test]
fn native_solidity_grammar_rejects_invalid_input() {
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
        &["nativeSolidityRejectsInvalidInput"],
        "native Solidity grammar rejects invalid input",
    );
}

#[test]
fn native_solidity_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    // A comment is trivia before the node after it, as in the oracle, and a
    // number keeps its digits as a token of the literal.
    let text = "// é\ncontract C { uint x = 1; }\n";
    let mut found = Vec::new();
    placed(
        &parse(&parser, text).expect("accepted"),
        text,
        None,
        &mut found,
    );
    let expected = [
        ("comment", "contract_declaration", "// é", true),
        ("identifier", "contract_declaration", "C", false),
        ("identifier", "state_variable_declaration", "x", false),
        ("unnamed_token", "number_literal", "1", false),
    ]
    .map(|(kind, parent, leaf, trivia)| {
        (kind.to_owned(), parent.to_owned(), leaf.to_owned(), trivia)
    });
    assert_eq!(found, expected);
    observe(
        &["nativeSolidityTreesLossless"],
        "native Solidity trees keep every byte",
    );
}
