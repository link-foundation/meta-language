//! Requirement I195-GRAMMAR-NATIVE-REGEX: the native merged Regex grammar,
//! parity/grammars/native/regex.lino, which js/scripts/import-native-grammars.mjs
//! imports from the pinned tree-sitter-regex 0.25.0 grammar, builds the
//! concrete syntax trees of the tree-sitter oracle the native grammar
//! replaced as the default Regex parse. parity/fixtures/native-grammars/regex.json
//! holds the upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-regex.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::Value;

use super::issue_195_native_grammar_rows::{Rows, cases, leaves, parse, rebuilt, source, text};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/regex.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/regex.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/regex.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-REGEX",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-regex",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Regex fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

#[test]
fn native_regex_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Regex grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("pattern"));
    for rule in [
        "pattern",
        "alternation",
        "term",
        "any_character",
        "start_assertion",
        "end_assertion",
        "boundary_assertion",
        "non_boundary_assertion",
        "lookaround_assertion",
        "lookahead_assertion",
        "lookbehind_assertion",
        "pattern_character",
        "character_class",
        "posix_character_class",
        "class_range",
        "class_character",
        "anonymous_capturing_group",
        "named_capturing_group",
        "non_capturing_group",
        "inline_flags_group",
        "flags",
        "zero_or_more",
        "one_or_more",
        "optional",
        "count_quantifier",
        "backreference_escape",
        "named_group_backreference",
        "decimal_escape",
        "character_class_escape",
        "unicode_character_escape",
        "unicode_property_value_expression",
        "control_escape",
        "identity_escape",
        "group_name",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // tree-sitter-regex has no external scanner.
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
        &["nativeRegexGrammarIsCanonicalLinks"],
        "native Regex grammar is canonical Links Notation",
    );
}

#[test]
fn native_regex_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 80);
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
    // A brace or a bracket that opens no quantifier or POSIX class is a
    // character, as ECMAScript reads it, where the oracle's lexer recovers.
    let divergences = cases(&fixture, "divergences");
    assert_eq!(
        divergences.iter().map(source).collect::<Vec<_>>(),
        ["a{", "[[:alpha:]"]
    );
    for case in divergences {
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    observe(
        &["nativeRegexTreesMatchOracle"],
        "native Regex grammar builds the oracle rows",
    );
}

#[test]
fn native_regex_grammar_rejects_invalid_input() {
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
        &["nativeRegexRejectsInvalidInput"],
        "native Regex grammar rejects invalid input",
    );
}

#[test]
fn native_regex_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    // The line break between two terms is an extra of the pattern.
    let tree = parse(&parser, "a\n(?:b)+?").expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted Regex tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("pattern_character"), "a"),
            (None, "\n"),
            (None, "(?:"),
            (Some("pattern_character"), "b"),
            (None, ")"),
            (None, "+"),
            (Some("lazy"), "?"),
        ]
    );
    observe(
        &["nativeRegexTreesLossless"],
        "native Regex trees keep every byte",
    );
}
