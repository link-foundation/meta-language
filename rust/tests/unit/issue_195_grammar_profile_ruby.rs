//! Candidate Ruby grammar and counted scanner operations are shared Links data.
use super::issue_195_native_grammar_rows::{
    Rows, cases, oracle_agrees, oracle_rows, parse, parser, rebuilt, source,
};
use meta_language::{FeatureParseOptions, parse_grammar_links, render_grammar_links};
use serde_json::Value;

const GRAMMAR: &str = include_str!("../../../parity/grammars/native/ruby.lino");
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/ruby-profile.lino");
const CORPUS: &str =
    include_str!("../../../parity/fixtures/native-grammars/ruby-profile-corpus.lino");
const EXPRESSIONS: &str =
    include_str!("../../../parity/fixtures/native-grammars/ruby-expression-contexts.lino");
const HEREDOC_CALLS: &str =
    include_str!("../../../parity/fixtures/native-grammars/ruby-heredoc-call-context-ci.lino");
const METHOD_DECLARATIONS: &str = include_str!(
    "../../../parity/fixtures/native-grammars/ruby-method-declaration-context-ci.lino"
);

#[test]
fn ruby_complete_method_statements_preserve_default_budget_and_shared_contexts() {
    let profile: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    let fixture: Value = super::project_data::read_project_data(METHOD_DECLARATIONS).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&profile);
    let language = tree_sitter_ruby::LANGUAGE.into();
    assert_eq!(cases(&fixture, "cases").len(), 1);
    assert_eq!(source(&cases(&fixture, "cases")[0]).len(), 317);
    assert_eq!(cases(&fixture, "controls").len(), 23);
    for case in cases(&fixture, "cases")
        .iter()
        .chain(cases(&fixture, "controls").iter())
    {
        let (expected, recovers) = oracle_rows(&language, source(case));
        assert!(!recovers, "{:?}", source(case));
        assert_eq!(Value::Array(expected), case["rows"]);
        let tree = parse(&parser, source(case)).expect("native accepts default-budget definition");
        assert_eq!(Value::Array(rows.rows(&tree, source(case))), case["rows"]);
        assert_eq!(rebuilt(&tree), source(case));
    }
}

#[test]
fn ruby_heredoc_call_contexts_match_shared_ci_sources_and_neighboring_controls() {
    let profile: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    let fixture: Value = super::project_data::read_project_data(HEREDOC_CALLS).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&profile);
    let language = tree_sitter_ruby::LANGUAGE.into();
    assert_eq!(cases(&fixture, "cases").len(), 4);
    assert_eq!(cases(&fixture, "controls").len(), 11);
    for case in cases(&fixture, "cases")
        .iter()
        .chain(cases(&fixture, "controls").iter())
    {
        let (expected, recovers) = oracle_rows(&language, source(case));
        assert!(!recovers, "{:?}", source(case));
        assert_eq!(Value::Array(expected), case["rows"]);
        let tree = parse(&parser, source(case)).expect("native accepts unambiguous heredoc call");
        assert_eq!(Value::Array(rows.rows(&tree, source(case))), case["rows"]);
        assert_eq!(rebuilt(&tree), source(case));
    }
}

#[test]
fn ruby_operator_and_call_contexts_match_the_independent_oracle() {
    let profile: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    let fixture: Value = super::project_data::read_project_data(EXPRESSIONS).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&profile);
    let language = tree_sitter_ruby::LANGUAGE.into();
    assert_eq!(cases(&fixture, "matches").len(), 16);
    for case in cases(&fixture, "matches") {
        let (expected, recovers) = oracle_rows(&language, source(case));
        assert!(!recovers, "{:?}", source(case));
        assert_eq!(Value::Array(expected), case["rows"]);
        let tree =
            parse(&parser, source(case)).expect("native accepts recorded expression context");
        assert_eq!(Value::Array(rows.rows(&tree, source(case))), case["rows"]);
        assert_eq!(rebuilt(&tree), source(case));
    }
}

#[test]
fn ruby_profile_focused_trees_match_the_independent_oracle() {
    assert_eq!(
        render_grammar_links(&parse_grammar_links(GRAMMAR).unwrap()),
        GRAMMAR
    );
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    oracle_agrees(&tree_sitter_ruby::LANGUAGE.into(), &fixture);
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("native accepts focused source");
        assert_eq!(Value::Array(rows.rows(&tree, source(case))), case["rows"]);
        assert_eq!(rebuilt(&tree), source(case));
    }
    for case in cases(&fixture, "rejections") {
        assert!(parse(&parser, source(case)).is_none());
        let native = parser
            .parse_tree(
                source(case).as_bytes(),
                &FeatureParseOptions {
                    error_recovery: Some(true),
                    ..FeatureParseOptions::default()
                },
            )
            .unwrap();
        assert_eq!(native.rejection.unwrap().reason, "recovered");
        assert_eq!(rebuilt(&native.tree.unwrap()), source(case));
    }
}

// CI executes the complete pinned corpus; local work uses focused JavaScript.
#[test]
fn ruby_profile_upstream_corpus_matches_the_independent_oracle() {
    let fixture: Value = super::project_data::read_project_data(FIXTURE).unwrap();
    let corpus: Value = super::project_data::read_project_data(CORPUS).unwrap();
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    let language = tree_sitter_ruby::LANGUAGE.into();
    let inputs = corpus["cases"].as_array().unwrap();
    assert!(!inputs.is_empty());
    let mut failures = Vec::new();
    for case in inputs {
        let (expected, recovers) = oracle_rows(&language, source(case));
        let outcome = parser.parse_tree(source(case).as_bytes(), &FeatureParseOptions::default());
        let problem = match outcome {
            Err(error) => Some(format!("executor: {error:?}")),
            Ok(outcome) if recovers => outcome
                .ok
                .then(|| "native accepted invalid input".to_owned()),
            Ok(outcome) if !outcome.ok => {
                Some(format!("native rejection: {:?}", outcome.rejection))
            }
            Ok(outcome) if !outcome.ambiguities.is_empty() => {
                Some("ambiguous native tree".to_owned())
            }
            Ok(outcome) => {
                let tree = outcome.tree.unwrap();
                if rows.rows(&tree, source(case)) != expected {
                    Some("concrete tree differs from oracle".to_owned())
                } else if rebuilt(&tree) != source(case) {
                    Some("source reconstruction differs".to_owned())
                } else {
                    None
                }
            }
        };
        if let Some(problem) = problem {
            failures.push(format!("{}: {}: {problem}", case["file"], case["title"]));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
