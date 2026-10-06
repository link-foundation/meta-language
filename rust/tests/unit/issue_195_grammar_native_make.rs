//! Requirement I195-GRAMMAR-NATIVE-MAKE: the native merged Make grammar,
//! parity/grammars/native/make.lino, which
//! js/scripts/import-native-grammars.mjs imports from the pinned
//! tree-sitter-make 1.1.1 grammar, builds the concrete syntax trees of the
//! tree-sitter oracle the native grammar replaced as the default Make parse.
//! parity/fixtures/native-grammars/make.json holds the
//! upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-make.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::Value;

use super::issue_195_native_grammar_rows::{Rows, cases, parse, rebuilt, source};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/make.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/make.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/make.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-MAKE",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-make",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Make fixture is JSON")
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
        other => panic!("an accepted Make tree has no {other:?}"),
    }
}

#[test]
fn native_make_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Make grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("makefile"));
    for rule in [
        "makefile",
        "rule",
        "recipe",
        "recipe_line",
        "search_path_assignment",
        "recipe_prefix_assignment",
        "variable_assignment",
        "shell_assignment",
        "define_directive",
        "include_directive",
        "search_path_directive",
        "export_directive",
        "remove_export_directive",
        "override_directive",
        "remove_definition_directive",
        "private_directive",
        "conditional",
        "else_if_directive",
        "else_directive",
        "if_equal_directive",
        "if_not_equal_directive",
        "if_defined_directive",
        "if_not_defined_directive",
        "variable_reference",
        "substitution_reference",
        "automatic_variable",
        "function_call",
        "arguments",
        "shell_function",
        "list",
        "paths",
        "concatenation",
        "string",
        "archive",
        "shell_text_with_split",
        "text",
        "word",
        "comment",
        "raw_line",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // tree-sitter-make has no external scanner.
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
            && !foreign.contains("(make ")
            && !foreign.contains("module.exports")
            && !generator.is_match(&foreign)
    );
    observe(
        &["nativeMakeGrammarIsCanonicalLinks"],
        "native Make grammar is canonical Links Notation",
    );
}

#[test]
fn native_make_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    // The upstream corpus but the three custom `.RECIPEPREFIX` cases the
    // oracle recovers from.
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
    assert_eq!(cases(&fixture, "divergences"), [] as [Value; 0]);
    observe(
        &["nativeMakeTreesMatchOracle"],
        "native Make grammar builds the oracle rows",
    );
}

#[test]
fn native_make_grammar_rejects_invalid_input() {
    let fixture = fixture();
    let parser = parser();
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 10);
    // A rule whose recipe line lacks the tab, and a target `(` of an unclosed
    // reference: the lexer takes the line break after the colon as a token
    // and the `(` as the literal, as tree-sitter does, before the line is
    // read.
    for text in ["a:\nb\n", "a = $(b\n"] {
        assert!(
            rejections.iter().any(|case| source(case) == text),
            "{text:?}"
        );
    }
    for case in rejections {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    observe(
        &["nativeMakeRejectsInvalidInput"],
        "native Make grammar rejects invalid input",
    );
}

#[test]
fn native_make_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    // A comment is trivia before the node after it, which the rows put in the
    // makefile as the oracle does; the reference and the text after it are
    // one value, and a recipe line keeps its `@` and the shell text, as in
    // the oracle.
    let text = "a = $(b) x\n# c\nall:\n\t@echo $$y\n";
    let mut found = Vec::new();
    placed(
        &parse(&parser, text).expect("accepted"),
        text,
        None,
        &mut found,
    );
    let expected = [
        ("word", "variable_assignment", "a", false),
        ("unnamed_token", "variable_assignment", " ", false),
        ("unnamed_token", "variable_assignment", " ", false),
        ("word", "variable_reference", "b", false),
        ("unnamed_token", "text", " x", false),
        ("unnamed_token", "variable_assignment", "\n", false),
        ("comment", "targets", "# c", true),
        ("word", "targets", "all", false),
        ("unnamed_token", "recipe", "\n", false),
        ("unnamed_token", "shell_text", "echo ", false),
        ("word", "variable_reference", "y", false),
        ("unnamed_token", "recipe", "\n", false),
    ]
    .map(|(kind, parent, leaf, trivia)| {
        (kind.to_owned(), parent.to_owned(), leaf.to_owned(), trivia)
    });
    assert_eq!(found, expected);
    observe(
        &["nativeMakeTreesLossless"],
        "native Make trees keep every byte",
    );
}
