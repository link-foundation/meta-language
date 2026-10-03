//! Requirement I195-GRAMMAR-NATIVE-DIFF: the native merged diff grammar,
//! parity/grammars/native/diff.lino, builds the concrete syntax trees of the
//! tree-sitter-diff oracle the native grammar replaced as the default diff parse.
//! parity/fixtures/native-grammars/diff.json holds the corpus with the oracle
//! rows, which js/scripts/generate-native-grammar-fixtures.mjs generates; this
//! suite projects the Rust executor's trees with
//! `issue_195_native_grammar_rows.rs` and compares them with the fixture, as
//! js/tests/issue-195-grammar-native-diff.test.js does for JavaScript.

use meta_language::{
    FeatureGrammarParser, FeatureParseOptions, SyntaxTree, parse_grammar_links,
    render_grammar_links,
};
use serde_json::{Value, json};

use super::issue_195_native_grammar_rows::{
    Rows, cases, leaves, oracle_agrees, parse, rebuilt, source, text,
};
use super::issue_195_observations::{Observation, record};

const FIXTURE_FILE: &str = "parity/fixtures/native-grammars/diff.json";
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/diff.json");
const GRAMMAR: &str = include_str!("../../../parity/grammars/native/diff.lino");

fn observe(assertions: &[&str], test_name: &str) {
    record(&Observation {
        requirement_id: "I195-GRAMMAR-NATIVE-DIFF",
        suffix: "behavior",
        fixture_id: "planned:repository-directive:i195-grammar-native-diff",
        fixture_file: FIXTURE_FILE,
        assertions,
        test_name,
    });
}

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).expect("the native diff fixture is JSON")
}

fn parser() -> FeatureGrammarParser {
    super::issue_195_native_grammar_rows::parser(GRAMMAR)
}

fn rows_of(rows: &Rows, parser: &FeatureGrammarParser, case: &Value) -> Value {
    let tree = parse(parser, source(case)).unwrap_or_else(|| panic!("{:?}", source(case)));
    Value::Array(rows.rows(&tree, source(case)))
}

#[test]
fn native_diff_grammar_is_canonical_links_notation() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the native diff grammar reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.start(), Some("source"));
    for rule in [
        "source", "block", "command", "hunks", "hunk", "changes", "location", "addition",
        "deletion", "context",
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
        &["nativeDiffGrammarIsCanonicalLinks"],
        "native diff grammar is canonical Links Notation",
    );
}

#[test]
fn native_diff_grammar_builds_the_oracle_rows() {
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
    // A git patch is a block: the command, its headers and the hunks of one file.
    let patch = matches
        .iter()
        .find(|case| source(case).starts_with("diff --git a/x b/y\nsimilarity index 90%"))
        .expect("the patch case");
    let outline: Vec<Value> = patch["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .filter(|row| !row[1].is_null() || row[0].as_u64() <= Some(2))
        .map(|row| json!([row[0], row[1], row[2]]))
        .collect();
    assert_eq!(
        outline,
        [
            json!([0, null, "source"]),
            json!([1, null, "block"]),
            json!([2, null, "command"]),
            json!([2, null, "similarity"]),
            json!([2, null, "file_change"]),
            json!([2, null, "file_change"]),
            json!([2, null, "index"]),
            json!([2, null, "old_file"]),
            json!([2, null, "new_file"]),
            json!([2, null, "hunks"]),
            json!([4, "location", "location"]),
            json!([4, "changes", "changes"]),
        ]
    );
    observe(
        &["nativeDiffTreesMatchOracle"],
        "native diff grammar builds the oracle rows",
    );
}

#[test]
fn native_diff_grammar_accepts_merged_source_extensions() {
    let fixture = fixture();
    let parser = parser();
    let rows = Rows::new(&fixture);
    let divergences = cases(&fixture, "divergences");
    assert!(divergences.len() >= 8);
    for case in divergences {
        assert!(
            case["reason"]
                .as_str()
                .expect("reason")
                .contains("tree-sitter-diff 0.1.0")
        );
        assert_eq!(
            rows_of(&rows, &parser, case),
            case["rows"],
            "{:?}",
            source(case)
        );
    }
    // A hunk line that starts with a space is context, keyword or not.
    let context = divergences
        .iter()
        .find(|case| source(case) == " new x\n")
        .expect("the context case");
    assert_eq!(
        context["rows"],
        json!([
            [0, null, "source", 1, 0, 7, ""],
            [1, null, "context", 1, 0, 6, ""]
        ])
    );
    observe(
        &["nativeDiffAcceptsMergedSourceExtensions"],
        "native diff grammar accepts merged source extensions",
    );
}

#[test]
fn native_diff_grammar_rejects_invalid_diffs() {
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
        &["nativeDiffRejectsInvalidInput"],
        "native diff grammar rejects invalid diffs",
    );
}

#[test]
fn native_diff_trees_keep_every_byte() {
    let fixture = fixture();
    let parser = parser();
    for case in cases(&fixture, "matches")
        .iter()
        .chain(cases(&fixture, "divergences"))
    {
        let tree = parse(&parser, source(case)).expect("accepted");
        assert_eq!(rebuilt(&tree), source(case));
    }
    let tree = parse(&parser, " \n--- a b\n@@ -1 +1 @@ f\n-x \r\n").expect("accepted");
    let kinds: Vec<(Option<&str>, &str)> = leaves(&tree)
        .into_iter()
        .map(|leaf| match leaf {
            SyntaxTree::Token {
                kind, text: leaf, ..
            } => (kind.as_deref(), text(leaf)),
            other => panic!("an accepted diff tree has no {other:?}"),
        })
        .collect();
    assert_eq!(
        kinds,
        [
            (Some("newline"), " \n"),
            (None, "---"),
            (None, " "),
            (Some("word"), "a"),
            (None, " "),
            (Some("word"), "b"),
            (Some("newline"), "\n"),
            (None, "@@"),
            (None, " "),
            (Some("linerange"), "-1"),
            (None, " "),
            (Some("linerange"), "+1"),
            (None, " "),
            (None, "@@"),
            (Some("anything"), " f"),
            (Some("newline"), "\n"),
            (None, "-"),
            (Some("anything"), "x "),
            (Some("newline"), "\r\n"),
        ]
    );
    observe(
        &["nativeDiffTreesLossless"],
        "native diff trees keep every byte",
    );
}

#[test]
fn pinned_diff_oracle_gives_the_fixture() {
    // The oracle is a development dependency since the native grammar
    // replaced it as the default Diff parse.
    oracle_agrees(&tree_sitter_diff::LANGUAGE.into(), &fixture());
}
