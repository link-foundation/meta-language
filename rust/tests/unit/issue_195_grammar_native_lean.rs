//! Requirement I195-GRAMMAR-NATIVE-LEAN: the native merged Lean grammar,
//! parity/grammars/native/lean.lino, which js/scripts/import-native-grammars.mjs
//! imports from the pinned tree-sitter-lean4 grammar with its external layout
//! scanner ported to a native scanner link, builds the concrete syntax trees
//! of the tree-sitter oracle the native grammar replaced as the default Lean
//! parse. parity/fixtures/native-grammars/lean.json holds the upstream corpus
//! with the oracle rows, which js/scripts/generate-native-grammar-fixtures.mjs
//! generates; this suite projects the Rust executor's trees with
//! `issue_195_native_grammar_rows.rs` and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-lean.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{Rows, cases, leaves, parse, rebuilt, source, text};
use super::issue_195_observations::{Observation, record};

#[test]
fn native_lean_explicit_application_arguments_are_left_associated() {
    let source = "#check @plant Tree rain\n";
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let parser = super::issue_195_native_grammar_rows::parser(GRAMMAR);
    let tree = parse(&parser, source).unwrap();
    let rows = Rows::new(&fixture).rows(&tree, source);
    let spans = |kind: &str| {
        rows.iter()
            .filter(|row| row[2] == kind)
            .map(|row| json!([row[4], row[5]]))
            .collect::<Vec<_>>()
    };
    assert_eq!(spans("application"), vec![json!([7, 23]), json!([7, 18])]);
    assert_eq!(spans("explicit"), vec![json!([7, 13])]);
    assert_eq!(rebuilt(&tree), source);
}

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/lean.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/lean.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/lean.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-LEAN",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-lean",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native Lean fixture is JSON")
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
fn native_lean_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native Lean grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("module"));
    for rule in [
        "module",
        "import",
        "command",
        "set_option",
        "namespace",
        "section",
        "open",
        "variable",
        "universe",
        "hash_command",
        "declaration",
        "attributes",
        "definition",
        "instance_declaration",
        "structure",
        "inductive",
        "constructor",
        "explicit_binder",
        "implicit_binder",
        "instance_binder",
        "application",
        "projection",
        "explicit",
        "fun",
        "quantifier",
        "by",
        "tactic",
        "match",
        "match_arm",
        "do",
        "do_let",
        "anonymous_constructor",
        "structure_instance",
        "string",
        "identifier",
        "comment",
    ] {
        assert!(grammar.rule(rule).is_some(), "{rule}");
    }
    // The external layout scanner of tree-sitter-lean4 is ported to a native
    // scanner link.
    let scanners: Vec<&str> = GRAMMAR
        .lines()
        .filter_map(|line| line.strip_prefix("(scanner "))
        .map(|rest| rest.split(' ').next().unwrap_or_default())
        .collect();
    assert_eq!(scanners, ["layout"]);
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
        &["nativeLeanGrammarIsCanonicalLinks"],
        "native Lean grammar is canonical Links Notation",
    );
}

#[test]
fn native_lean_grammar_builds_the_oracle_rows() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let matches = cases(&fixture, "matches");
    assert!(matches.len() >= 240);
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
    // `pp` is a name of level 0 right that one parse ends before a projection
    // `.all` and the other after `all`; right associativity keeps the dotted
    // name.
    assert_eq!(
        Value::Array(
            case(matches, "set_option pp.all true\n")["rows"]
                .as_array()
                .expect("rows")[3..6]
                .to_vec()
        ),
        json!([
            [2, "name", "identifier", 1, 11, 13, ""],
            [2, "name", ".", 0, 13, 14, ""],
            [2, "name", "identifier", 1, 14, 17, ""]
        ])
    );
    observe(
        &["nativeLeanTreesMatchOracle"],
        "native Lean grammar builds the oracle rows",
    );
}

#[test]
fn native_lean_grammar_accepts_merged_source_extensions() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let divergences = cases(&fixture, "divergences");
    assert_eq!(divergences.len(), 1);
    for case in divergences {
        let reason = case["reason"].as_str().expect("reason");
        assert!(
            reason.contains("tree-sitter-lean4 0.3.0")
                && reason.contains("Theorem Proving in Lean 4 section")
        );
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // `#check @foo.bar` projects `bar` out of the explicit `@foo`.
    let explicit = divergences[0]["rows"].as_array().expect("rows");
    assert_eq!(
        Value::Array(explicit[explicit.len() - 7..].to_vec()),
        json!([
            [1, null, "hash_command", 1, 56, 71, ""],
            [2, null, "projection", 1, 63, 71, ""],
            [3, "term", "explicit", 1, 63, 67, ""],
            [4, null, "@", 0, 63, 64, ""],
            [4, null, "identifier", 1, 64, 67, ""],
            [3, null, ".", 0, 67, 68, ""],
            [3, "name", "identifier", 1, 68, 71, ""]
        ])
    );
    observe(
        &["nativeLeanAcceptsMergedSourceExtensions"],
        "native Lean grammar accepts merged source extensions",
    );
}

#[test]
fn native_lean_grammar_rejects_invalid_input() {
    let fixture = fixture();
    let parser = parser();
    let rejections = cases(&fixture, "rejections");
    assert!(rejections.len() >= 40);
    for case in rejections {
        let outcome = parser
            .parse_tree(source(case).as_bytes(), &FeatureParseOptions::default())
            .expect("the parse runs");
        assert!(!outcome.ok, "{:?}", source(case));
        assert!(outcome.rejection.is_some(), "{:?}", source(case));
    }
    observe(
        &["nativeLeanRejectsInvalidInput"],
        "native Lean grammar rejects invalid input",
    );
}

#[test]
fn native_lean_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches")
        .iter()
        .chain(cases(&fixture, "divergences"))
    {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(&parser, "-- a\ndef f (x : Nat) : Nat := x.succ /- b -/\n").expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted Lean tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("comment"), "-- a"),
            (None, "\n"),
            (None, "def"),
            (None, " "),
            (Some("identifier"), "f"),
            (None, " "),
            (None, "("),
            (Some("identifier"), "x"),
            (None, " "),
            (None, ":"),
            (None, " "),
            (Some("identifier"), "Nat"),
            (None, ")"),
            (None, " "),
            (None, ":"),
            (None, " "),
            (Some("identifier"), "Nat"),
            (None, " "),
            (None, ":="),
            (Some("unnamed_token"), " "),
            (Some("identifier"), "x"),
            (None, "."),
            (Some("identifier"), "succ"),
            (None, " "),
            (Some("comment"), "/- b -/"),
            (Some("unnamed_token"), ""),
            (None, "\n"),
        ]
    );
    observe(
        &["nativeLeanTreesLossless"],
        "native Lean trees keep every byte",
    );
}
