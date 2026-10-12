//! Original Ruby CI inputs retain their default native budgets and oracle trees.
use super::issue_195_native_grammar_rows::{Rows, oracle_rows, parse, parser, rebuilt, source};
use serde_json::Value;

const GRAMMAR: &str = include_str!("../../../parity/grammars/native/ruby.lino");
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/ruby-profile.json");
const CORPUS: &str =
    include_str!("../../../parity/fixtures/native-grammars/ruby-profile-corpus.json");

#[test]
fn fixed_lexical_matching_preserves_original_ruby_default_budgets() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let corpus: Value = serde_json::from_str(CORPUS).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    let language = tree_sitter_ruby::LANGUAGE.into();
    for title in [
        "relational",
        "minus, call, exponential, range",
        "method call with reserved keyword args",
        "method calls with splat argument",
    ] {
        let case = corpus["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["file"] == "expressions.txt" && case["title"] == title)
            .unwrap();
        let source = source(case);
        let (expected, recovers) = oracle_rows(&language, source);
        assert!(!recovers, "{title}");
        let tree = parse(&parser, source)
            .unwrap_or_else(|| panic!("original source must fit its default budget: {title}"));
        assert_eq!(rows.rows(&tree, source), expected, "{title}");
        assert_eq!(rebuilt(&tree), source, "{title}");
    }
}

#[test]
fn fixed_lexical_matching_preserves_neighboring_ruby_token_contexts() {
    let fixture: Value = serde_json::from_str(FIXTURE).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    let language = tree_sitter_ruby::LANGUAGE.into();
    for source in [
        "a == b\n",
        "a -1\n",
        "foo if: true\n",
        "foo(*bar)\n",
        "foo(x, *)\n",
        "foo(**)\n",
    ] {
        let (expected, recovers) = oracle_rows(&language, source);
        assert!(!recovers, "{source}");
        let tree = parse(&parser, source).unwrap();
        assert_eq!(rows.rows(&tree, source), expected, "{source}");
        assert_eq!(rebuilt(&tree), source);
    }
    let invalid = "foo(*\n";
    assert!(oracle_rows(&language, invalid).1);
    assert!(parse(&parser, invalid).is_none());
}
