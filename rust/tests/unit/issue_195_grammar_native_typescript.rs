//! Requirement I195-GRAMMAR-NATIVE-TYPESCRIPT: the native merged TypeScript grammar,
//! parity/grammars/native/typescript.lino, which js/scripts/import-native-grammars.mjs
//! imports from the pinned tree-sitter-typescript TypeScript grammar with its
//! external scanner ported to native scanner links, builds the concrete
//! syntax trees of the tree-sitter oracle the native grammar replaced as the
//! default TypeScript parse. parity/fixtures/native-grammars/typescript.json holds the
//! upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-typescript.test.js does for JavaScript.

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

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/typescript.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/typescript.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/typescript.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-TYPESCRIPT",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-typescript",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native TypeScript fixture is JSON")
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
fn native_typescript_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native TypeScript grammar reads");
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
        "type_assertion",
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
        &["nativeTypeScriptGrammarIsCanonicalLinks"],
        "native TypeScript grammar is canonical Links Notation",
    );
}

#[test]
fn native_typescript_grammar_builds_the_oracle_rows() {
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
    // TypeScript reads `<string>y` as a type assertion, which TSX reads as
    // JSX.
    assert_eq!(
        slice(matches, "let x = <string>y;", 6..7),
        json!([[3, "value", "type_assertion", 1, 8, 17, ""]])
    );
    observe(
        &["nativeTypeScriptTreesMatchOracle"],
        "native TypeScript grammar builds the oracle rows",
    );
}

#[test]
fn native_typescript_grammar_rejects_invalid_input() {
    let fixture = fixture();
    let parser = parser();
    assert_eq!(cases(&fixture, "divergences"), [] as [Value; 0]);
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 15);
    for case in rejections {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    observe(
        &["nativeTypeScriptRejectsInvalidInput"],
        "native TypeScript grammar rejects invalid input",
    );
}

#[test]
fn native_typescript_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(
        &parser,
        "// a\nlet v: Array<number> = <number[]>w!;\ntype L = `x${T}`;\n",
    )
    .expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted TypeScript tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("comment"), "// a"),
            (None, "\n"),
            (None, "let"),
            (None, " "),
            (Some("identifier"), "v"),
            (None, ":"),
            (None, " "),
            (Some("type_identifier"), "Array"),
            (None, "<"),
            (None, "number"),
            (None, ">"),
            (None, " "),
            (None, "="),
            (None, " "),
            (None, "<"),
            (None, "number"),
            (None, "["),
            (None, "]"),
            (None, ">"),
            (Some("identifier"), "w"),
            (None, "!"),
            (None, ";"),
            (None, "\n"),
            (None, "type"),
            (None, " "),
            (Some("type_identifier"), "L"),
            (None, " "),
            (None, "="),
            (None, " "),
            (None, "`"),
            (Some("string_fragment"), "x"),
            (None, "${"),
            (Some("type_identifier"), "T"),
            (None, "}"),
            (None, "`"),
            (None, ";"),
            (None, "\n"),
        ]
    );
    observe(
        &["nativeTypeScriptTreesLossless"],
        "native TypeScript trees keep every byte",
    );
}

#[test]
fn native_typescript_trees_follow_the_oracle_on_an_identifier_as() {
    let fixture = fixture();
    let rows = Rows::new(&fixture);
    let parser = parser();
    // The tree-sitter-typescript oracle rows of
    // js/tests/issue-195-grammar-native-typescript.test.js: `as` after a
    // keyword and after a line break is an identifier.
    for (source, oracle) in [
        (
            "return as ;",
            json!([
                [0, null, "program", 1, 0, 11, ""],
                [1, null, "return_statement", 1, 0, 11, ""],
                [2, null, "return", 0, 0, 6, ""],
                [2, null, "identifier", 1, 7, 9, ""],
                [2, null, ";", 0, 10, 11, ""]
            ]),
        ),
        (
            "if ( x ) as ( 1 ) ;",
            json!([
                [0, null, "program", 1, 0, 19, ""],
                [1, null, "if_statement", 1, 0, 19, ""],
                [2, null, "if", 0, 0, 2, ""],
                [2, "condition", "parenthesized_expression", 1, 3, 8, ""],
                [3, null, "(", 0, 3, 4, ""],
                [3, null, "identifier", 1, 5, 6, ""],
                [3, null, ")", 0, 7, 8, ""],
                [2, "consequence", "expression_statement", 1, 9, 19, ""],
                [3, null, "call_expression", 1, 9, 17, ""],
                [4, "function", "identifier", 1, 9, 11, ""],
                [4, "arguments", "arguments", 1, 12, 17, ""],
                [5, null, "(", 0, 12, 13, ""],
                [5, null, "number", 1, 14, 15, ""],
                [5, null, ")", 0, 16, 17, ""],
                [3, null, ";", 0, 18, 19, ""]
            ]),
        ),
        (
            "a\nas ( t ) ;",
            json!([
                [0, null, "program", 1, 0, 12, ""],
                [1, null, "expression_statement", 1, 0, 1, ""],
                [2, null, "identifier", 1, 0, 1, ""],
                [1, null, "expression_statement", 1, 2, 12, ""],
                [2, null, "call_expression", 1, 2, 10, ""],
                [3, "function", "identifier", 1, 2, 4, ""],
                [3, "arguments", "arguments", 1, 5, 10, ""],
                [4, null, "(", 0, 5, 6, ""],
                [4, null, "identifier", 1, 7, 8, ""],
                [4, null, ")", 0, 9, 10, ""],
                [2, null, ";", 0, 11, 12, ""]
            ]),
        ),
    ] {
        let tree = parse(&parser, source).unwrap_or_else(|| panic!("{source:?}"));
        assert_eq!(Value::Array(rows.rows(&tree, source)), oracle, "{source:?}");
    }
    observe(
        &["nativeTypeScriptTreesMatchOracle"],
        "native TypeScript trees follow the oracle on an identifier `as` after a keyword and after a line break",
    );
}

#[test]
fn pinned_typescript_oracle_gives_the_fixture() {
    // The oracle is a development dependency since the native grammar
    // replaced it as the default TypeScript parse.
    oracle_agrees(
        &tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
        &fixture(),
    );
}
