//! Requirement I195-GRAMMAR-NATIVE-JAVA: the native merged Java grammar,
//! parity/grammars/native/java.lino, which js/scripts/import-native-grammars.mjs
//! imports from the pinned tree-sitter-java 0.23.5 grammar, builds the
//! concrete syntax trees of the tree-sitter oracle the native grammar
//! replaced as the default Java parse. parity/fixtures/native-grammars/java.json
//! holds the upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-java.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::Value;

use super::issue_195_native_grammar_rows::{Rows, cases, leaves, parse, rebuilt, source, text};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/java.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/java.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/java.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-JAVA",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-java",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Java fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

#[test]
fn native_java_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Java grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("program"));
    for rule in [
        "program",
        "package_declaration",
        "import_declaration",
        "module_declaration",
        "class_declaration",
        "record_declaration",
        "interface_declaration",
        "enumeration_declaration",
        "annotation_type_declaration",
        "class_body",
        "field_declaration",
        "method_declaration",
        "constructor_declaration",
        "formal_parameters",
        "type_parameters",
        "generic_type",
        "array_type",
        "block",
        "local_variable_declaration",
        "if_statement",
        "for_statement",
        "enhanced_for_statement",
        "try_statement",
        "switch_expression",
        "yield_statement",
        "lambda_expression",
        "method_reference",
        "method_invocation",
        "object_creation_expression",
        "binary_expression",
        "instance_of_expression",
        "ternary_expression",
        "cast_expression",
        "annotation",
        "element_value_pair",
        "string_literal",
        "identifier",
        "line_comment",
        "block_comment",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // tree-sitter-java has no external scanner.
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
        &["nativeJavaGrammarIsCanonicalLinks"],
        "native Java grammar is canonical Links Notation",
    );
}

#[test]
fn native_java_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 140);
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
    // The renamed `instance_of_expression` rule keeps the oracle kind
    // `instanceof_expression`.
    assert_eq!(
        fixture["oracleKinds"]["instance_of_expression"],
        "instanceof_expression"
    );
    assert_eq!(cases(&fixture, "divergences"), [] as [Value; 0]);
    observe(
        &["nativeJavaTreesMatchOracle"],
        "native Java grammar builds the oracle rows",
    );
}

#[test]
fn native_java_grammar_rejects_invalid_input() {
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
        &["nativeJavaRejectsInvalidInput"],
        "native Java grammar rejects invalid input",
    );
}

#[test]
fn native_java_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(&parser, "// c\nclass A { /* d */ int x = 1; }\n").expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted Java tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("line_comment"), "// c"),
            (None, "\n"),
            (None, "class"),
            (None, " "),
            (Some("identifier"), "A"),
            (None, " "),
            (None, "{"),
            (None, " "),
            (Some("block_comment"), "/* d */"),
            (None, " "),
            (None, "int"),
            (None, " "),
            (Some("identifier"), "x"),
            (None, " "),
            (None, "="),
            (None, " "),
            (Some("decimal_integer_literal"), "1"),
            (None, ";"),
            (None, " "),
            (None, "}"),
            (None, "\n"),
        ]
    );
    observe(
        &["nativeJavaTreesLossless"],
        "native Java trees keep every byte",
    );
}

#[test]
fn native_java_trees_follow_the_oracle_on_a_generic_type_a_method_reference_and_an_annotation_argument()
 {
    let fixture = fixture();
    let rows = Rows::new(&fixture);
    let parser = parser();
    // As js/tests/issue-195-grammar-native-java.test.js checks: `A<B> c;`
    // forks at the declared conflict of a generic type and a primary
    // expression, and the generic type's dynamic precedence keeps it; `b` of
    // `b::m` is reduced alone to a type and to a primary expression, a
    // declared reduce/reduce conflict the rule defined first wins; and
    // `v = 1` closes an element value pair of precedence 2 against an
    // assignment of 1. The fixture holds the oracle rows of each.
    for wanted in [
        "class A { void f() { A<B> c; } }\n",
        "class A { void f() { a = b::m; } }\n",
        "@A(v = 1) class C {}\n",
    ] {
        let case = cases(&fixture, "matches")
            .iter()
            .find(|case| source(case) == wanted)
            .unwrap_or_else(|| panic!("the fixture holds {wanted:?}"));
        let outcome = parser
            .parse_tree(wanted.as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(outcome.ok, "{wanted:?}");
        assert!(outcome.ambiguities.is_empty(), "{wanted:?}");
        assert_eq!(rows_of(&rows, &parser, case), case["rows"], "{wanted:?}");
    }
    observe(
        &["nativeJavaTreesMatchOracle"],
        "native Java trees follow the oracle on a generic type, a method reference and an annotation argument",
    );
}
