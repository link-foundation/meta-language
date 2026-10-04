//! Requirement I195-GRAMMAR-NATIVE-TSX: the native merged TSX grammar,
//! parity/grammars/native/tsx.lino, which js/scripts/import-native-grammars.mjs
//! imports from the pinned tree-sitter-typescript TSX grammar with its
//! external scanner ported to native scanner links, builds the concrete
//! syntax trees of the tree-sitter oracle the native grammar replaced as the
//! default TSX parse. parity/fixtures/native-grammars/tsx.json holds the
//! upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-tsx.test.js does for JavaScript.

use std::ops::Range;

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, parse, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/tsx.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/tsx.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/tsx.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-TSX",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-tsx",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native TSX fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

// The fixture rows of the `wanted` case in `range`.
fn slice(cases: &[Value], wanted: &str, range: Range<usize>) -> Value {
    let case = cases
        .iter()
        .find(|case| source(case) == wanted)
        .unwrap_or_else(|| panic!("the {wanted:?} case"));
    Value::Array(case["rows"].as_array().expect("rows")[range].to_vec())
}

#[test]
fn native_tsx_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native TSX grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("program"));
    for rule in [
        "program",
        "interface_declaration",
        "type_alias_declaration",
        "enumeration_declaration",
        "abstract_class_declaration",
        "internal_module",
        "module",
        "ambient_declaration",
        "import_alias",
        "type_annotation",
        "generic_type",
        "type_arguments",
        "type_parameters",
        "union_type",
        "intersection_type",
        "conditional_type",
        "mapped_type_clause",
        "template_literal_type",
        "tuple_type",
        "as_expression",
        "satisfies_expression",
        "non_null_expression",
        "instantiation_expression",
        "jsx_element",
        "jsx_self_closing_element",
        "jsx_opening_element",
        "jsx_closing_element",
        "jsx_attribute",
        "jsx_expression",
        "jsx_namespace_name",
        "decorator",
        "arrow_function",
        "call_expression",
        "unary_expression",
        "identifier",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // The external scanner of tree-sitter-typescript is ported to native
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
            "function_signature_automatic_semicolon",
            "error_recovery",
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
        &["nativeTsxGrammarIsCanonicalLinks"],
        "native TSX grammar is canonical Links Notation",
    );
}

#[test]
fn native_tsx_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 150);
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
    // Where a unary operand meets `<`, the generated parser forks at a
    // declared conflict and keeps the parse that reduced the operand:
    // `!g<T>()` calls `!g`, and `await g<T>` instantiates `await g`.
    assert_eq!(
        slice(matches, "!g<T>();", 2..6),
        json!([
            [2, null, "call_expression", 1, 0, 7, ""],
            [3, "function", "unary_expression", 1, 0, 2, ""],
            [4, "operator", "!", 0, 0, 1, ""],
            [4, "argument", "identifier", 1, 1, 2, ""]
        ])
    );
    assert_eq!(
        slice(matches, "await g<T>;", 2..6),
        json!([
            [2, null, "instantiation_expression", 1, 0, 10, ""],
            [3, null, "await_expression", 1, 0, 7, ""],
            [4, null, "await", 0, 0, 5, ""],
            [4, null, "identifier", 1, 6, 7, ""]
        ])
    );
    // TSX reads `<div ...>{b}</div>` as a JSX element.
    assert_eq!(
        slice(matches, "x = <div className=\"a\">{b}</div>;", 5..7),
        json!([
            [3, "right", "jsx_element", 1, 4, 32, ""],
            [4, "open_tag", "jsx_opening_element", 1, 4, 23, ""]
        ])
    );
    observe(
        &["nativeTsxTreesMatchOracle"],
        "native TSX grammar builds the oracle rows",
    );
}

#[test]
fn native_tsx_grammar_rejects_invalid_input() {
    let fixture = fixture();
    let parser = parser();
    assert_eq!(cases(&fixture, "divergences"), [] as [Value; 0]);
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 16);
    // An unclosed JSX element is no TSX.
    assert!(rejections.iter().any(|case| source(case) == "x = <div>;"));
    for case in rejections {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    observe(
        &["nativeTsxRejectsInvalidInput"],
        "native TSX grammar rejects invalid input",
    );
}

#[test]
fn native_tsx_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(&parser, "// a\nconst v = <A.B c={1}>t {y}</A.B>;\n").expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted TSX tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("comment"), "// a"),
            (None, "\n"),
            (None, "const"),
            (None, " "),
            (Some("identifier"), "v"),
            (None, " "),
            (None, "="),
            (None, " "),
            (None, "<"),
            (Some("identifier"), "A"),
            (None, "."),
            (Some("property_identifier"), "B"),
            (None, " "),
            (Some("property_identifier"), "c"),
            (None, "="),
            (None, "{"),
            (Some("number"), "1"),
            (None, "}"),
            (None, ">"),
            (Some("jsx_text"), "t "),
            (None, "{"),
            (Some("identifier"), "y"),
            (None, "}"),
            (None, "</"),
            (Some("identifier"), "A"),
            (None, "."),
            (Some("property_identifier"), "B"),
            (None, ">"),
            (None, ";"),
            (None, "\n"),
        ]
    );
    observe(
        &["nativeTsxTreesLossless"],
        "native TSX trees keep every byte",
    );
}

#[test]
fn pinned_tsx_oracle_gives_the_fixture() {
    // The oracle is a development dependency since the native grammar
    // replaced it as the default TSX parse.
    oracle_agrees(&tree_sitter_typescript::LANGUAGE_TSX.into(), &fixture());
}
