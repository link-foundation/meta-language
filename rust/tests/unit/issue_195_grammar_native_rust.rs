//! Requirement I195-GRAMMAR-NATIVE-RUST: the native merged Rust grammar,
//! parity/grammars/native/rust.lino, which js/scripts/import-native-grammars.mjs
//! imports from the pinned tree-sitter-rust grammar with its external scanner
//! ported to native scanner links, builds the concrete syntax trees of the
//! tree-sitter-rust oracle the native grammar replaced as the default Rust
//! parse. parity/fixtures/native-grammars/rust.json holds the upstream corpus
//! with the oracle rows, which js/scripts/generate-native-grammar-fixtures.mjs
//! generates; this suite projects the Rust executor's trees with
//! `issue_195_native_grammar_rows.rs` and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-rust.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{Rows, cases, leaves, parse, rebuilt, source, text};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/rust.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/rust.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/rust.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-RUST",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-rust",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Rust fixture is JSON")
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
fn native_rust_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Rust grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("source_file"));
    for rule in [
        "source_file",
        "function_item",
        "structure_item",
        "enumeration_item",
        "implementation_item",
        "trait_item",
        "module_item",
        "use_declaration",
        "let_declaration",
        "macro_definition",
        "macro_invocation",
        "attribute_item",
        "match_expression",
        "if_expression",
        "closure_expression",
        "binary_expression",
        "call_expression",
        "await_expression",
        "try_expression",
        "string_literal",
        "raw_string_literal",
        "character_literal",
        "line_comment",
        "block_comment",
        "lifetime",
        "generic_type",
        "pattern",
        "identifier",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // The external scanner of tree-sitter-rust is ported to native scanner links.
    let scanners: Vec<&str> = GRAMMAR
        .lines()
        .filter_map(|line| line.strip_prefix("(scanner "))
        .map(|rest| rest.split(' ').next().unwrap_or_default())
        .collect();
    assert_eq!(
        scanners,
        [
            "strings",
            "raw_strings",
            "floats",
            "block_comments",
            "line_documentation",
            "error_sentinel",
        ]
    );
    assert!(GRAMMAR.trim_end().lines().all(|line| {
        [
            "(grammar ",
            "(extra ",
            "(conflict ",
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
        &["nativeRustGrammarIsCanonicalLinks"],
        "native Rust grammar is canonical Links Notation",
    );
}

#[test]
fn native_rust_grammar_builds_the_oracle_rows() {
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
    // A postfix `.await` and `?` nest as the oracle's precedences settle them.
    assert_eq!(
        case(matches, "x.await?;")["rows"],
        json!([
            [0, null, "source_file", 1, 0, 9, ""],
            [1, null, "expression_statement", 1, 0, 9, ""],
            [2, null, "try_expression", 1, 0, 8, ""],
            [3, null, "await_expression", 1, 0, 7, ""],
            [4, null, "identifier", 1, 0, 1, ""],
            [4, null, ".", 0, 1, 2, ""],
            [4, null, "await", 0, 2, 7, ""],
            [3, null, "?", 0, 7, 8, ""],
            [2, null, ";", 0, 8, 9, ""]
        ])
    );
    // A macro invocation followed by `;` in a block is an expression
    // statement: `_expression_except_range` reduces it at level 1, where a
    // declaration statement of level 0 and an empty statement conflict.
    assert_eq!(
        case(matches, "fn f() { m!(x); }")["rows"],
        json!([
            [0, null, "source_file", 1, 0, 17, ""],
            [1, null, "function_item", 1, 0, 17, ""],
            [2, null, "fn", 0, 0, 2, ""],
            [2, "name", "identifier", 1, 3, 4, ""],
            [2, "parameters", "parameters", 1, 4, 6, ""],
            [3, null, "(", 0, 4, 5, ""],
            [3, null, ")", 0, 5, 6, ""],
            [2, "body", "block", 1, 7, 17, ""],
            [3, null, "{", 0, 7, 8, ""],
            [3, null, "expression_statement", 1, 9, 15, ""],
            [4, null, "macro_invocation", 1, 9, 14, ""],
            [5, "macro", "identifier", 1, 9, 10, ""],
            [5, null, "!", 0, 10, 11, ""],
            [5, null, "token_tree", 1, 11, 14, ""],
            [6, null, "(", 0, 11, 12, ""],
            [6, null, "identifier", 1, 12, 13, ""],
            [6, null, ")", 0, 13, 14, ""],
            [4, null, ";", 0, 14, 15, ""],
            [3, null, "}", 0, 16, 17, ""]
        ])
    );
    observe(
        &["nativeRustTreesMatchOracle"],
        "native Rust grammar builds the oracle rows",
    );
}

#[test]
fn native_rust_grammar_rejects_invalid_input() {
    let fixture = fixture();
    let parser = parser();
    assert_eq!(cases(&fixture, "divergences"), [] as [Value; 0]);
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 30);
    for case in rejections {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    observe(
        &["nativeRustRejectsInvalidInput"],
        "native Rust grammar rejects invalid input",
    );
}

#[test]
fn native_rust_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(
        &parser,
        "/// d\n#[a] fn f<'a>(x: &'a str) /* b /* c */ */ { r#\"s\"#; \"\\n\"; } // e\n",
    )
    .expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted Rust tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (None, "//"),
            (None, "/"),
            (Some("documentation_comment"), " d\n"),
            (None, "#"),
            (None, "["),
            (Some("identifier"), "a"),
            (None, "]"),
            (None, " "),
            (None, "fn"),
            (None, " "),
            (Some("identifier"), "f"),
            (None, "<"),
            (None, "'"),
            (Some("identifier"), "a"),
            (None, ">"),
            (None, "("),
            (Some("identifier"), "x"),
            (None, ":"),
            (None, " "),
            (None, "&"),
            (None, "'"),
            (Some("identifier"), "a"),
            (None, " "),
            (Some("primitive_type"), "str"),
            (None, ")"),
            (None, " "),
            (None, "/*"),
            (Some("unnamed_token"), " b /* c */ "),
            (None, "*/"),
            (None, " "),
            (None, "{"),
            (None, " "),
            (Some("unnamed_token"), "r#\""),
            (Some("string_content"), "s"),
            (Some("unnamed_token"), "\"#"),
            (None, ";"),
            (None, " "),
            (Some("'\""), "\""),
            (Some("escape_sequence"), "\\n"),
            (Some("'\""), "\""),
            (None, ";"),
            (None, " "),
            (None, "}"),
            (None, " "),
            (None, "//"),
            (Some("unnamed_token"), " e"),
            (None, "\n"),
        ]
    );
    observe(
        &["nativeRustTreesLossless"],
        "native Rust trees keep every byte",
    );
}
