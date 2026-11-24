use meta_language::grammar::feature::{FeatureExpr, FeatureForm, FieldValue};
use meta_language::{
    CharClassItem, FeatureParseOptions, GrammarExpr, GrammarFormat, GrammarImportError, RuleKind,
    compile_feature_grammar, emit_lark, import_lark,
};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};

#[test]
fn imports_covering_lark_fixture() {
    let grammar =
        import_lark(include_str!("../fixtures/grammar/lark/covering.lark")).expect("imports");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Lark));
    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("start")
    );
    assert_eq!(
        grammar.rule_names(),
        vec![
            "start", "item", "trailer", "WORD", "NUMBER", "WS", "_ignore"
        ]
    );
    assert_eq!(
        grammar.rule("start").expect("start").kind(),
        RuleKind::Normal
    );
    assert_eq!(grammar.rule("item").expect("item").kind(), RuleKind::Silent);
    assert_eq!(grammar.rule("item").expect("item").doc(), Some("inline"));
    assert_eq!(grammar.rule("WORD").expect("WORD").kind(), RuleKind::Token);
    assert_eq!(
        grammar.rule("_ignore").expect("_ignore").doc(),
        Some("%ignore")
    );

    assert_eq!(
        grammar.rule("start").expect("start").expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::NonTerminal("item".to_string()),
            GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::Sequence(vec![
                GrammarExpr::Terminal(",".to_string()),
                GrammarExpr::NonTerminal("item".to_string()),
            ]))),
            GrammarExpr::Optional(Box::new(GrammarExpr::NonTerminal("trailer".to_string()))),
        ])
    );
    assert_eq!(
        grammar.rule("item").expect("item").expr(),
        &GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::NonTerminal("WORD".to_string()),
                GrammarExpr::Repeat {
                    expr: Box::new(GrammarExpr::NonTerminal("NUMBER".to_string())),
                    min: 2,
                    max: Some(2),
                },
            ],
        }
    );
    assert_eq!(
        grammar.rule("WORD").expect("WORD").expr(),
        &GrammarExpr::CharClass {
            negated: false,
            items: vec![CharClassItem::Range('A', 'Z')],
        }
    );
    assert_eq!(
        grammar.rule("trailer").expect("trailer").expr(),
        &GrammarExpr::feature(FeatureExpr::Form(FeatureForm::new(
            "token",
            vec![FieldValue::Expression(GrammarExpr::one_or_more(
                GrammarExpr::char_class(false, [CharClassItem::Range('a', 'z')]),
            ))],
        )))
    );
    assert!(grammar.undefined_nonterminals().is_empty());
}

#[test]
fn prefers_conventional_start_rule_over_first_lark_rule() {
    let grammar = import_lark(
        r#"
other: "x"
start: other
"#,
    )
    .expect("imports");

    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("start")
    );
}

#[test]
fn malformed_lark_reports_parse_error() {
    let error = import_lark("start: [unterminated").expect_err("parse error");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::Lark,
            ..
        }
    ));
}

#[test]
fn unresolved_lark_import_reports_unsupported_error() {
    let error = import_lark("%import common.WS").expect_err("unsupported import");

    assert!(matches!(
        error,
        GrammarImportError::Unsupported {
            format: GrammarFormat::Lark,
            construct
        } if construct == "%import"
    ));
}

#[test]
fn lark_regex_terminals_match_and_report_unsupported_source_positions() {
    const FIXTURE: &str = "parity/fixtures/lark-regex-terminals.json";
    const TEST: &str = "lark_regex_terminals_match_and_report_unsupported_source_positions";
    let cases: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/lark-regex-terminals.lino"
    ))
    .expect("shared Lark regex fixture");
    let observe = |id: &str, assertion: &str| {
        record(&Observation {
            requirement_id: id,
            suffix: "behavior",
            fixture_id: &id.to_ascii_lowercase(),
            fixture_file: FIXTURE,
            assertions: &[assertion],
            test_name: TEST,
        });
    };
    for (group, id, assertion) in [
        (
            "cases",
            "I195-I233-LARK-REGEX-TERMINALS",
            "regexTerminalsMatchTheirLanguage",
        ),
        (
            "flags",
            "I195-I233-LARK-REGEX-FLAGS",
            "supportedRegexFlagsChangeMatching",
        ),
    ] {
        for case in cases[group].as_array().expect("fixture cases") {
            let source = case["source"].as_str().expect("Lark source");
            let grammar = import_lark(source).expect("regex imports");
            let (emitted, _) = emit_lark(&grammar).expect("regex emits");
            let reimported = import_lark(&emitted).expect("regex reimports");
            for grammar in [&grammar, &reimported] {
                let parser = compile_feature_grammar(grammar, None, FeatureParseOptions::default())
                    .expect("regex compiles");
                for (key, expected) in [("accept", true), ("reject", false)] {
                    for sample in case[key].as_array().expect("sample inputs") {
                        let input = sample.as_str().expect("sample text");
                        let result = parser
                            .parse_tree(input.as_bytes(), &FeatureParseOptions::default())
                            .expect("regex parse outcome");
                        assert_eq!(result.ok, expected, "{source}: {input:?}");
                    }
                }
            }
        }
        observe(id, assertion);
    }

    for case in cases["diagnostics"].as_array().expect("regex diagnostics") {
        let source = case[0].as_str().expect("Lark source");
        let diagnostic = case[1].as_str().expect("diagnostic");
        let error = import_lark(source).expect_err("unsupported or malformed regex");
        assert_eq!(
            error.to_string(),
            format!("lark import parse error: {diagnostic}")
        );
    }
    observe(
        "I195-I233-LARK-REGEX-DIAGNOSTICS",
        "unsupportedRegexReportsSourceByte",
    );
}
