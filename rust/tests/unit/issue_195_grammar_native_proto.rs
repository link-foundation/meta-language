//! Requirement I195-GRAMMAR-NATIVE-PROTO: the native merged Protocol Buffers
//! grammar, parity/grammars/native/proto.lino, which
//! js/scripts/import-native-grammars.mjs imports from the pinned
//! tree-sitter-proto 0.6.0 grammar, builds the concrete syntax trees of the
//! tree-sitter oracle the native grammar replaced as the default Protocol
//! Buffers parse. parity/fixtures/native-grammars/proto.json holds the
//! upstream corpus with the oracle rows, which
//! js/scripts/generate-native-grammar-fixtures.mjs generates; this suite
//! projects the Rust executor's trees with `issue_195_native_grammar_rows.rs`
//! and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-proto.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::Value;

use super::issue_195_native_grammar_rows::{Rows, cases, parse, rebuilt, source};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/proto.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/proto.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/proto.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-PROTO",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-proto",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Protocol Buffers fixture is JSON")
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
        other => panic!("an accepted Protocol Buffers tree has no {other:?}"),
    }
}

#[test]
fn native_proto_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Protocol Buffers grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("source_file"));
    for rule in [
        "source_file",
        "syntax",
        "edition",
        "import",
        "package",
        "option",
        "option_name",
        "enumeration",
        "enumeration_body",
        "enumeration_field",
        "message",
        "message_body",
        "extend",
        "group",
        "field",
        "field_options",
        "one_of",
        "one_of_field",
        "map_field",
        "key_type",
        "type",
        "reserved",
        "extensions",
        "ranges",
        "range",
        "reserved_field_names",
        "service",
        "remote_procedure_call",
        "constant",
        "block_literal",
        "extension_name",
        "identifier",
        "full_identifier",
        "boolean",
        "integer_literal",
        "float_literal",
        "string",
        "escape_sequence",
        "comment",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // tree-sitter-proto has no external scanner.
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
            && !foreign.contains("(proto ")
            && !foreign.contains("module.exports")
            && !generator.is_match(&foreign)
    );
    observe(
        &["nativeProtoGrammarIsCanonicalLinks"],
        "native Protocol Buffers grammar is canonical Links Notation",
    );
}

#[test]
fn native_proto_grammar_builds_the_oracle_rows() {
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
        &["nativeProtoTreesMatchOracle"],
        "native Protocol Buffers grammar builds the oracle rows",
    );
}

#[test]
fn native_proto_grammar_rejects_invalid_input() {
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
        &["nativeProtoRejectsInvalidInput"],
        "native Protocol Buffers grammar rejects invalid input",
    );
}

#[test]
fn native_proto_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    // A comment is trivia of the node around it; the escape sequence and the
    // text between quotes are leaves of the string, and adjacent strings are
    // one string, as in the oracle.
    let text = "option a = \"x\\n\" \"y\"; // c\n";
    let mut found = Vec::new();
    placed(
        &parse(&parser, text).expect("accepted"),
        text,
        None,
        &mut found,
    );
    let expected = [
        ("identifier", "option", "a", false),
        ("unnamed_token", "string", "x", false),
        ("escape_sequence", "string", "\\n", false),
        ("unnamed_token", "string", "y", false),
        ("comment", "source_file", "// c", true),
    ]
    .map(|(kind, parent, leaf, trivia)| {
        (kind.to_owned(), parent.to_owned(), leaf.to_owned(), trivia)
    });
    assert_eq!(found, expected);
    observe(
        &["nativeProtoTreesLossless"],
        "native Protocol Buffers trees keep every byte",
    );
}
