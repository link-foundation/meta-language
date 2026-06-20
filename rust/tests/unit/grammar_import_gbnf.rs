use meta_language::{
    import_gbnf, CharClassItem, GrammarExpr, GrammarFormat, GrammarImportError, RuleKind,
};

#[test]
fn imports_arithmetic_gbnf_fixture() {
    let grammar =
        import_gbnf(include_str!("../fixtures/grammar/gbnf/arithmetic.gbnf")).expect("imports");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Gbnf));
    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("root")
    );
    assert_eq!(
        grammar.rule_names(),
        vec![
            "root",
            "expr",
            "term",
            "factor",
            "number",
            "identifier",
            "not_quote"
        ]
    );
    assert_eq!(grammar.rule("root").expect("root").kind(), RuleKind::Normal);
    assert_eq!(
        grammar.rule("root").expect("root").doc(),
        Some("# arithmetic grammar with counted repetition and character classes")
    );
    assert_eq!(
        grammar.rule("root").expect("root").expr(),
        &GrammarExpr::NonTerminal("expr".to_string())
    );
    assert_eq!(
        grammar.rule("expr").expect("expr").expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::NonTerminal("term".to_string()),
            GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::Sequence(vec![
                GrammarExpr::Choice {
                    ordered: false,
                    alternatives: vec![
                        GrammarExpr::Terminal("+".to_string()),
                        GrammarExpr::Terminal("-".to_string()),
                    ],
                },
                GrammarExpr::NonTerminal("term".to_string()),
            ]))),
        ])
    );
    assert_eq!(
        grammar.rule("number").expect("number").expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::Repeat {
                expr: Box::new(GrammarExpr::CharClass {
                    negated: false,
                    items: vec![CharClassItem::Range('0', '9')],
                }),
                min: 1,
                max: None,
            },
            GrammarExpr::Optional(Box::new(GrammarExpr::Sequence(vec![
                GrammarExpr::Terminal(".".to_string()),
                GrammarExpr::Repeat {
                    expr: Box::new(GrammarExpr::CharClass {
                        negated: false,
                        items: vec![CharClassItem::Range('0', '9')],
                    }),
                    min: 1,
                    max: Some(3),
                },
            ]))),
        ])
    );
    assert_eq!(
        grammar.rule("identifier").expect("identifier").expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::CharClass {
                negated: false,
                items: vec![
                    CharClassItem::Range('a', 'z'),
                    CharClassItem::Range('A', 'Z'),
                    CharClassItem::Char('_'),
                ],
            },
            GrammarExpr::Repeat {
                expr: Box::new(GrammarExpr::CharClass {
                    negated: false,
                    items: vec![
                        CharClassItem::Range('a', 'z'),
                        CharClassItem::Range('A', 'Z'),
                        CharClassItem::Range('0', '9'),
                        CharClassItem::Char('_'),
                    ],
                }),
                min: 0,
                max: None,
            },
        ])
    );
    assert_eq!(
        grammar.rule("not_quote").expect("not_quote").expr(),
        &GrammarExpr::CharClass {
            negated: true,
            items: vec![CharClassItem::Char('"'), CharClassItem::Char('\n')],
        }
    );
    assert!(grammar.undefined_nonterminals().is_empty());
}

#[test]
fn lowers_exact_gbnf_repetition() {
    let grammar = import_gbnf(r#"root ::= "a"{3}"#).expect("imports");

    assert_eq!(
        grammar.rule("root").expect("root").expr(),
        &GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::Terminal("a".to_string())),
            min: 3,
            max: Some(3),
        }
    );
}

#[test]
fn malformed_gbnf_reports_parse_error() {
    let error = import_gbnf("root ::= [unterminated").expect_err("parse error");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::Gbnf,
            ..
        }
    ));
}
