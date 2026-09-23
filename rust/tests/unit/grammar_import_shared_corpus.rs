use meta_language::{
    emit_peggy, grammar_from_lino, grammar_to_lino, import_abnf, import_bnf, import_ebnf,
    import_pest, import_tree_sitter_json, Grammar, GrammarParser,
};
use serde_json::Value;

#[test]
fn shared_importer_corpus_parses_emits_and_round_trips() {
    let fixture: Value = serde_json::from_str(include_str!(
        "../../../parity/fixtures/grammar-importers.json"
    ))
    .expect("shared importer fixture is JSON");
    let cases = fixture["cases"].as_array().expect("cases array");

    for case in cases {
        let format = case["format"].as_str().expect("format");
        let source = case["source"].as_str().expect("source");
        let grammar = import(format, source);
        let expected_start = case["start"].as_str().expect("start");
        assert_eq!(
            grammar.start_rule().map(|rule| rule.name.as_str()),
            Some(expected_start),
            "{format} start rule"
        );
        let expected_rules = case["rules"]
            .as_array()
            .expect("rules")
            .iter()
            .map(|name| name.as_str().expect("rule name"))
            .collect::<Vec<_>>();
        assert_eq!(
            &grammar.rule_names()[..expected_rules.len()],
            expected_rules,
            "{format} rule order"
        );
        assert!(grammar.undefined_nonterminals().is_empty(), "{format}");

        assert_membership(format, &grammar, case, "accepts", true);
        assert_membership(format, &grammar, case, "rejects", false);

        let (peggy, _report) = emit_peggy(&grammar).expect("grammar emits Peggy");
        assert!(peggy.contains(&format!("{expected_start} =")));

        let restored = grammar_from_lino(&grammar_to_lino(&grammar)).expect("LiNo round trip");
        assert_eq!(restored, grammar, "{format} grammar round trip");
        assert_membership(format, &restored, case, "accepts", true);
    }
}

fn import(format: &str, source: &str) -> Grammar {
    match format {
        "abnf" => import_abnf(source).expect("ABNF imports"),
        "bnf" => import_bnf(source).expect("BNF imports"),
        "ebnf" => import_ebnf(source).expect("EBNF imports"),
        "pest" => import_pest(source).expect("pest imports"),
        "tree-sitter-json" => import_tree_sitter_json(source).expect("tree-sitter JSON imports"),
        unexpected => panic!("unexpected shared importer format {unexpected}"),
    }
}

fn assert_membership(format: &str, grammar: &Grammar, case: &Value, key: &str, expected: bool) {
    let parser = GrammarParser::new(grammar.clone());
    for source in case[key].as_array().expect("membership examples") {
        let source = source.as_str().expect("example source");
        assert_eq!(
            parser.accepts(source),
            expected,
            "{format} should {} {source:?}",
            if expected { "accept" } else { "reject" }
        );
    }
}
