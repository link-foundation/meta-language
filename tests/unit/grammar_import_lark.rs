use meta_language::{
    import_lark, CharClassItem, GrammarExpr, GrammarFormat, GrammarImportError, RuleKind,
};

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
        vec!["start", "item", "trailer", "WORD", "NUMBER", "WS", "_ignore"]
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
        &GrammarExpr::Capture {
            label: Some("regex".to_string()),
            expr: Box::new(GrammarExpr::Terminal("[a-z]+".to_string())),
        }
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
