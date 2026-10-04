//! Requirement I195-GRAMMAR-NATIVE-JAVASCRIPT: the native merged JavaScript
//! grammar, parity/grammars/native/javascript.lino, which
//! js/scripts/import-native-grammars.mjs imports from the pinned
//! tree-sitter-javascript grammar with its external scanner ported to native
//! scanner links, builds the concrete syntax trees of the
//! tree-sitter-javascript oracle the native grammar replaced as the default
//! JavaScript parse. parity/fixtures/native-grammars/javascript.json holds the
//! upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-javascript.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, parse, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/javascript.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/javascript.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/javascript.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-JAVASCRIPT",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-javascript",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native JavaScript fixture is JSON")
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
fn native_javascript_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native JavaScript grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("program"));
    for rule in [
        "program",
        "import_statement",
        "export_statement",
        "function_declaration",
        "generator_function_declaration",
        "class_declaration",
        "class_body",
        "method_definition",
        "field_definition",
        "lexical_declaration",
        "variable_declaration",
        "if_statement",
        "for_statement",
        "for_in_statement",
        "while_statement",
        "try_statement",
        "switch_statement",
        "return_statement",
        "arrow_function",
        "call_expression",
        "member_expression",
        "assignment_expression",
        "binary_expression",
        "ternary_expression",
        "await_expression",
        "template_string",
        "template_substitution",
        "regular_expression",
        "string",
        "number",
        "comment",
        "jsx_element",
        "jsx_self_closing_element",
        "jsx_expression",
        "object_pattern",
        "array_pattern",
        "spread_element",
        "private_property_identifier",
        "optional_chain",
        "identifier",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // The external scanner of tree-sitter-javascript is ported to native
    // scanner links.
    let scanners: Vec<&str> = GRAMMAR
        .lines()
        .filter_map(|line| line.strip_prefix("(scanner "))
        .map(|rest| rest.split(' ').next().unwrap_or_default())
        .collect();
    assert_eq!(
        scanners,
        [
            "automatic_semicolon",
            "template_characters",
            "ternary_question_mark",
            "html_comment",
            "jsx_text",
        ]
    );
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
        &["nativeJavaScriptGrammarIsCanonicalLinks"],
        "native JavaScript grammar is canonical Links Notation",
    );
}

#[test]
fn native_javascript_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 110);
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
    // The scanner inserts a semicolon at a line break, which the default tree
    // leaves out as the oracle does.
    assert_eq!(
        case(matches, "a\nb\n")["rows"],
        json!([
            [0, null, "program", 1, 0, 4, ""],
            [1, null, "expression_statement", 1, 0, 1, ""],
            [2, null, "identifier", 1, 0, 1, ""],
            [1, null, "expression_statement", 1, 2, 3, ""],
            [2, null, "identifier", 1, 2, 3, ""]
        ])
    );
    // The scanner reads the characters of a template up to a substitution.
    assert_eq!(
        case(matches, "x = `a${b}c`;")["rows"],
        json!([
            [0, null, "program", 1, 0, 13, ""],
            [1, null, "expression_statement", 1, 0, 13, ""],
            [2, null, "assignment_expression", 1, 0, 12, ""],
            [3, "left", "identifier", 1, 0, 1, ""],
            [3, null, "=", 0, 2, 3, ""],
            [3, "right", "template_string", 1, 4, 12, ""],
            [4, null, "`", 0, 4, 5, ""],
            [4, null, "string_fragment", 1, 5, 6, ""],
            [4, null, "template_substitution", 1, 6, 10, ""],
            [5, null, "${", 0, 6, 8, ""],
            [5, null, "identifier", 1, 8, 9, ""],
            [5, null, "}", 0, 9, 10, ""],
            [4, null, "string_fragment", 1, 10, 11, ""],
            [4, null, "`", 0, 11, 12, ""],
            [2, null, ";", 0, 12, 13, ""]
        ])
    );
    observe(
        &["nativeJavaScriptTreesMatchOracle"],
        "native JavaScript grammar builds the oracle rows",
    );
}

#[test]
fn native_javascript_grammar_rejects_invalid_input() {
    let fixture = fixture();
    let parser = parser();
    assert_eq!(cases(&fixture, "divergences"), [] as [Value; 0]);
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 20);
    for case in rejections {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    // The zero-width automatic semicolon before a line break is scanned in the
    // parse state the keyword `class` after it is lexed in, so a bare `class`
    // is not read as an identifier.
    for source in ["x\nclass", "\nfunction foo() {}\nclass"] {
        let outcome = parser
            .parse_tree(source.as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{source:?}");
    }
    observe(
        &["nativeJavaScriptRejectsInvalidInput"],
        "native JavaScript grammar rejects invalid input",
    );
}

#[test]
fn native_javascript_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(
        &parser,
        "// a\nconst s = `t${x}` /* b */;\nlet r = /c+/g, j = <p>hi {y}</p>\n",
    )
    .expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted JavaScript tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("comment"), "// a"),
            (None, "\n"),
            (None, "const"),
            (None, " "),
            (Some("identifier"), "s"),
            (None, " "),
            (None, "="),
            (None, " "),
            (None, "`"),
            (Some("string_fragment"), "t"),
            (None, "${"),
            (Some("identifier"), "x"),
            (None, "}"),
            (None, "`"),
            (None, " "),
            (Some("comment"), "/* b */"),
            (None, ";"),
            (None, "\n"),
            (None, "let"),
            (None, " "),
            (Some("identifier"), "r"),
            (None, " "),
            (None, "="),
            (None, " "),
            (None, "/"),
            (Some("regular_expression_pattern"), "c+"),
            (None, "/"),
            (Some("regular_expression_flags"), "g"),
            (None, ","),
            (None, " "),
            (Some("identifier"), "j"),
            (None, " "),
            (None, "="),
            (None, " "),
            (None, "<"),
            (Some("identifier"), "p"),
            (None, ">"),
            (Some("jsx_text"), "hi "),
            (None, "{"),
            (Some("identifier"), "y"),
            (None, "}"),
            (None, "</"),
            (Some("identifier"), "p"),
            (None, ">"),
            (Some("unnamed_token"), ""),
            (None, "\n"),
        ]
    );
    observe(
        &["nativeJavaScriptTreesLossless"],
        "native JavaScript trees keep every byte",
    );
}

#[test]
fn pinned_javascript_oracle_gives_the_fixture() {
    // The oracle is a development dependency since the native grammar
    // replaced it as the default JavaScript parse.
    oracle_agrees(&tree_sitter_javascript::LANGUAGE.into(), &fixture());
}
