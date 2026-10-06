//! Requirement I195-GRAMMAR-NATIVE-GRAPHQL: the native merged GraphQL grammar,
//! parity/grammars/native/graphql.lino, which js/scripts/import-native-grammars.mjs
//! imports from the pinned tree-sitter-graphql 0.3.0 grammar, builds the
//! concrete syntax trees of the tree-sitter oracle the native grammar
//! replaced as the default GraphQL parse. parity/fixtures/native-grammars/graphql.json
//! holds the upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-graphql.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::Value;

use super::issue_195_native_grammar_rows::{Rows, cases, parse, rebuilt, source};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/graphql.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/graphql.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/graphql.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-GRAPHQL",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-graphql",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native GraphQL fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

/// The kind of the node each comma leaf is a child of, and whether it is
/// trivia there.
fn commas<'t>(tree: &'t SyntaxTree, parent: Option<&'t str>, found: &mut Vec<(&'t str, bool)>) {
    match tree {
        SyntaxTree::Node { kind, children, .. } => {
            for child in children {
                commas(child, Some(kind), found);
            }
        }
        SyntaxTree::Token { kind, trivia, .. } => {
            if kind.as_deref() == Some("comma") {
                found.push((parent.expect("a comma has a parent"), *trivia));
            }
        }
        other => panic!("an accepted GraphQL tree has no {other:?}"),
    }
}

fn commas_of(parser: &FeatureGrammarParser, text: &str) -> Vec<(String, bool)> {
    let tree = parse(parser, text).expect("accepted");
    let mut found = Vec::new();
    commas(&tree, None, &mut found);
    found
        .into_iter()
        .map(|(kind, trivia)| (kind.to_owned(), trivia))
        .collect()
}

#[test]
fn native_graphql_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native GraphQL grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("source_file"));
    for rule in [
        "document",
        "definition",
        "executable_definition",
        "operation_definition",
        "operation_type",
        "variable_definitions",
        "variable_definition",
        "selection_set",
        "selection",
        "field",
        "alias",
        "arguments",
        "argument",
        "value",
        "variable",
        "string_value",
        "integer_value",
        "float_value",
        "boolean_value",
        "null_value",
        "enumeration_value",
        "list_value",
        "object_value",
        "object_field",
        "fragment_spread",
        "fragment_definition",
        "inline_fragment",
        "type_condition",
        "directives",
        "directive",
        "directive_definition",
        "schema_definition",
        "object_type_definition",
        "enumeration_type_definition",
        "input_object_type_definition",
        "type_extension",
        "named_type",
        "list_type",
        "non_null_type",
        "name",
        "comment",
        "comma",
        "description",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // tree-sitter-graphql has no external scanner.
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
            && !foreign.contains("(graphql ")
            && !foreign.contains("module.exports")
            && !generator.is_match(&foreign)
    );
    observe(
        &["nativeGraphqlGrammarIsCanonicalLinks"],
        "native GraphQL grammar is canonical Links Notation",
    );
}

#[test]
fn native_graphql_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 50);
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
        &["nativeGraphqlTreesMatchOracle"],
        "native GraphQL grammar builds the oracle rows",
    );
}

#[test]
fn native_graphql_grammar_rejects_invalid_input() {
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
        &["nativeGraphqlRejectsInvalidInput"],
        "native GraphQL grammar rejects invalid input",
    );
}

#[test]
fn native_graphql_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    // A comma is an extra, but the token of a variable definition and of an
    // object field that names it: tree-sitter takes an extra token only where
    // the parse state has no action on it.
    let owned = |pairs: &[(&str, bool)]| {
        pairs
            .iter()
            .map(|&(kind, trivia)| (kind.to_owned(), trivia))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        commas_of(
            &parser,
            "query Q($a: Int, $b: Int) { f(o: {a: 1, b: 2}, x: [1, 2]) }"
        ),
        owned(&[
            ("variable_definition", false),
            ("object_field", false),
            ("argument", true),
            ("value", true),
        ])
    );
    assert_eq!(
        commas_of(&parser, ",{ a, b }"),
        owned(&[("selection_set", true), ("field", true)])
    );
    observe(
        &["nativeGraphqlTreesLossless"],
        "native GraphQL trees keep every byte",
    );
}
