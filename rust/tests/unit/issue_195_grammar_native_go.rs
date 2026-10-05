//! Requirement I195-GRAMMAR-NATIVE-GO: the native merged Go grammar,
//! parity/grammars/native/go.lino, which js/scripts/import-native-grammars.mjs
//! imports from the pinned tree-sitter-go 0.25.0 grammar, builds the
//! concrete syntax trees of the tree-sitter oracle the native grammar
//! replaced as the default Go parse. parity/fixtures/native-grammars/go.json
//! holds the upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-go.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::Value;

use super::issue_195_native_grammar_rows::{Rows, cases, leaves, parse, rebuilt, source, text};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/go.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/go.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/go.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-GO",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-go",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Go fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

#[test]
fn native_go_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Go grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("source_file"));
    for rule in [
        "source_file",
        "package_clause",
        "import_declaration",
        "import_specification",
        "constant_declaration",
        "variable_declaration",
        "function_declaration",
        "method_declaration",
        "type_parameter_list",
        "parameter_list",
        "type_alias",
        "type_declaration",
        "generic_type",
        "pointer_type",
        "array_type",
        "slice_type",
        "structure_type",
        "interface_type",
        "map_type",
        "channel_type",
        "function_type",
        "block",
        "short_variable_declaration",
        "assignment_statement",
        "labeled_statement",
        "if_statement",
        "for_statement",
        "range_clause",
        "expression_switch_statement",
        "type_switch_statement",
        "select_statement",
        "go_statement",
        "defer_statement",
        "send_statement",
        "call_expression",
        "selector_expression",
        "index_expression",
        "slice_expression",
        "type_assertion_expression",
        "type_conversion_expression",
        "composite_literal",
        "function_literal",
        "unary_expression",
        "binary_expression",
        "raw_string_literal",
        "interpreted_string_literal",
        "rune_literal",
        "imaginary_literal",
        "identifier",
        "comment",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // tree-sitter-go has no external scanner.
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
        &["nativeGoGrammarIsCanonicalLinks"],
        "native Go grammar is canonical Links Notation",
    );
}

#[test]
fn native_go_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 100);
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
    // The renamed `short_variable_declaration` rule keeps the oracle kind
    // `short_var_declaration`.
    assert_eq!(
        fixture["oracleKinds"]["short_variable_declaration"],
        "short_var_declaration"
    );
    assert_eq!(cases(&fixture, "divergences"), [] as [Value; 0]);
    observe(
        &["nativeGoTreesMatchOracle"],
        "native Go grammar builds the oracle rows",
    );
}

#[test]
fn native_go_grammar_rejects_invalid_input() {
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
        &["nativeGoRejectsInvalidInput"],
        "native Go grammar rejects invalid input",
    );
}

#[test]
fn native_go_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    // The newline that ends a declaration is a token of the source file, not
    // white space.
    let tree = parse(&parser, "// c\npackage main /* d */\nvar x = 1\n").expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted Go tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("comment"), "// c"),
            (None, "\n"),
            (None, "package"),
            (None, " "),
            (Some("package_identifier"), "main"),
            (None, " "),
            (Some("comment"), "/* d */"),
            (Some("unnamed_token"), "\n"),
            (None, "var"),
            (None, " "),
            (Some("identifier"), "x"),
            (None, " "),
            (None, "="),
            (None, " "),
            (Some("integer_literal"), "1"),
            (Some("unnamed_token"), "\n"),
        ]
    );
    observe(
        &["nativeGoTreesLossless"],
        "native Go trees keep every byte",
    );
}

#[test]
fn native_go_trees_follow_the_oracle_on_a_channel_type_a_type_conversion_a_generic_call_and_a_statement_list()
 {
    let fixture = fixture();
    let rows = Rows::new(&fixture);
    let parser = parser();
    // As js/tests/issue-195-grammar-native-go.test.js checks: `chan<- chan
    // int` is a `chan<-` channel type of `chan int`, where the parse that
    // shifts `<-` goes on to a sibling ending where the other's `<- chan int`
    // does; `<-chan int(c)` converts to a `<-chan int` channel type, the
    // reduce/reduce conflict on `int` its precedence 6 wins; `a[b](c)`
    // converts to the generic type `a[b]`, ahead on the stack's dynamic
    // precedence at `(` and tied with the call where the two merge; and of
    // `x := a\n\ty := b`, the newline is the token that ends the first
    // statement, not white space the other parse skips before the second.
    // The fixture holds the oracle rows of each.
    for wanted in [
        "package main\n\nvar c chan<- chan int\n",
        "package main\n\nvar x = <-chan int(c)\n",
        "package main\n\nvar x = a[b](c)\n",
        "package main\n\nfunc main() {\n\tx := a\n\ty := b\n}\n",
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
        &["nativeGoTreesMatchOracle"],
        "native Go trees follow the oracle on a channel type, a type conversion, a generic call and a statement list",
    );
}
