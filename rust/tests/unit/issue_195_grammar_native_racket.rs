//! Requirement I195-GRAMMAR-NATIVE-RACKET: the native merged Racket grammar,
//! parity/grammars/native/racket.lino, builds the concrete syntax trees of the
//! tree-sitter-racket oracle the native grammar replaced as the default Racket parse.
//! parity/fixtures/native-grammars/racket.json holds the corpus with the
//! oracle rows, which js/scripts/generate-native-grammar-fixtures.mjs
//! generates; this suite projects the Rust executor's trees with
//! `issue_195_native_grammar_rows.rs` and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-racket.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, parse, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/racket.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/racket.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/racket.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-RACKET",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-racket",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Racket fixture is JSON")
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
fn native_racket_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Racket grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("program"));
    for rule in [
        "program",
        "dot",
        "comment",
        "block_comment",
        "sexp_comment",
        "boolean",
        "string",
        "byte_string",
        "here_string",
        "here_terminator",
        "here_line",
        "here_end",
        "regex",
        "escape_sequence",
        "number",
        "character",
        "symbol",
        "keyword",
        "box",
        "list",
        "vector",
        "structure",
        "hash",
        "graph",
        "quote",
        "quasiquote",
        "syntax",
        "quasisyntax",
        "unquote",
        "unquote_splicing",
        "unsyntax",
        "unsyntax_splicing",
        "extension",
        "lang_name",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    assert!(GRAMMAR.trim_end().lines().all(|line| {
        ["(grammar ", "(extra ", "(rule "]
            .iter()
            .any(|prefix| line.starts_with(prefix))
    }));
    assert!(!GRAMMAR.contains("tree-sitter") && !GRAMMAR.contains("grammar.js"));
    observe(
        &["nativeRacketGrammarIsCanonicalLinks"],
        "native Racket grammar is canonical Links Notation",
    );
}

#[test]
fn native_racket_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 100);
    for case in matches {
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // A here string ends at the line equal to its terminator; a leading byte
    // order mark has no row.
    assert_eq!(
        case(matches, "#<<A\nB\nA\n#<<B\nA\nB")["rows"],
        json!([
            [0, null, "program", 1, 0, 17, ""],
            [1, null, "here_string", 1, 0, 8, ""],
            [2, null, "#<<", 0, 0, 3, ""],
            [1, null, "here_string", 1, 9, 17, ""],
            [2, null, "#<<", 0, 9, 12, ""]
        ])
    );
    assert_eq!(
        case(matches, "\u{feff}a")["rows"],
        json!([
            [0, null, "program", 1, 3, 4, ""],
            [1, null, "symbol", 1, 3, 4, ""]
        ])
    );
    observe(
        &["nativeRacketTreesMatchOracle"],
        "native Racket grammar builds the oracle rows",
    );
}

#[test]
fn native_racket_grammar_accepts_merged_source_extensions() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let divergences = cases(&fixture, "divergences");
    assert!(divergences.len() >= 20);
    for case in divergences {
        let reason = case["reason"].as_str().expect("reason");
        assert!(
            reason.contains("tree-sitter-racket 0.25.0")
                && reason.contains("Racket Reference section 1.3.")
        );
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // A backslash quotes a line feed in a character and in a symbol.
    assert_eq!(
        case(divergences, "#\\\n")["rows"],
        json!([
            [0, null, "program", 1, 0, 3, ""],
            [1, null, "character", 1, 0, 3, ""]
        ])
    );
    assert_eq!(
        case(divergences, "a\\\nb")["rows"],
        json!([
            [0, null, "program", 1, 0, 4, ""],
            [1, null, "symbol", 1, 0, 4, ""]
        ])
    );
    observe(
        &["nativeRacketAcceptsMergedSourceExtensions"],
        "native Racket grammar accepts merged source extensions",
    );
}

#[test]
fn native_racket_grammar_rejects_invalid_input() {
    let fixture = fixture();
    let parser = parser();
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 20);
    // The oracle accepts the last two, comparing only the first character of
    // a line with the here string terminator.
    let here_strings = [
        json!({"source": "#<<EOF\nab\nEOFx"}),
        json!({"source": "#<<EOF\nabc\nEXX"}),
    ];
    for case in rejections.iter().chain(&here_strings) {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    observe(
        &["nativeRacketRejectsInvalidInput"],
        "native Racket grammar rejects invalid input",
    );
}

#[test]
fn native_racket_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches")
        .iter()
        .chain(cases(&fixture, "divergences"))
    {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(
        &parser,
        "#lang racket ; c\n(define x #hash((a . \"b\\n\")) #| b |# #<<E\nx\nE\n1.5t3)",
    )
    .expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted Racket tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (None, "#lang "),
            (Some("lang_name"), "racket"),
            (Some("whitespace"), " "),
            (Some("comment"), "; c"),
            (Some("whitespace"), "\n"),
            (None, "("),
            (Some("symbol"), "define"),
            (Some("whitespace"), " "),
            (Some("symbol"), "x"),
            (Some("whitespace"), " "),
            (Some("hash_prefix"), "#hash"),
            (None, "("),
            (None, "("),
            (Some("symbol"), "a"),
            (Some("whitespace"), " "),
            (Some("dot"), "."),
            (Some("whitespace"), " "),
            (None, "\""),
            (Some("string_text"), "b"),
            (Some("escape_sequence"), "\\n"),
            (None, "\""),
            (None, ")"),
            (None, ")"),
            (Some("whitespace"), " "),
            (None, "#|"),
            (Some("comment_text"), " b "),
            (None, "|#"),
            (Some("whitespace"), " "),
            (None, "#<<"),
            (Some("here_terminator"), "E"),
            (Some("here_newline"), "\n"),
            (Some("here_line"), "x"),
            (Some("here_newline"), "\n"),
            (Some("here_end"), "E"),
            (Some("whitespace"), "\n"),
            (Some("number"), "1.5t3"),
            (None, ")"),
        ]
    );
    observe(
        &["nativeRacketTreesLossless"],
        "native Racket trees keep every byte",
    );
}

#[test]
fn pinned_racket_oracle_gives_the_fixture() {
    // The oracle is a development dependency since the native grammar
    // replaced it as the default Racket parse.
    oracle_agrees(&tree_sitter_racket::LANGUAGE.into(), &fixture());
}
