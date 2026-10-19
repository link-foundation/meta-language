//! Elm candidate source and scanner operations are shared data, exercised by
//! both runtimes before a catalog promotion or production dependency removal.
use super::issue_195_native_grammar_rows::{
    Rows, cases, oracle_agrees, parse, parser, rebuilt, source,
};
use meta_language::{parse_grammar_links, render_grammar_links};
use serde_json::Value;

const GRAMMAR: &str = include_str!("../../../parity/grammars/candidates/elm/native.lino");
const FIXTURE: &str = include_str!("../../../parity/fixtures/native-grammars/elm-candidate.lino");

#[test]
fn elm_candidate_is_canonical_native_links_with_every_source_scanner_token() {
    let grammar = parse_grammar_links(GRAMMAR).expect("the generated Elm candidate reads");
    assert_eq!(render_grammar_links(&grammar), GRAMMAR);
    assert_eq!(grammar.declarations().external_tokens().len(), 7);
}

#[test]
fn elm_candidate_focused_layout_queues_and_lexical_content_match_independent_oracle() {
    let fixture: Value =
        super::project_data::read_project_data(FIXTURE).expect("Elm candidate fixture");
    oracle_agrees(&tree_sitter_elm::LANGUAGE.into(), &fixture);
    let parser = parser(GRAMMAR);
    let rows = Rows::new(&fixture);
    for case in cases(&fixture, "matches") {
        let tree = parse(&parser, source(case)).expect("the candidate accepts the focused source");
        assert_eq!(
            Value::Array(rows.rows(&tree, source(case))),
            case["rows"],
            "{:?}",
            source(case)
        );
        assert_eq!(rebuilt(&tree), source(case));
    }
    for case in cases(&fixture, "rejections") {
        assert!(parse(&parser, source(case)).is_none(), "{:?}", source(case));
    }
}
