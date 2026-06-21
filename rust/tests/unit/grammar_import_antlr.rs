use meta_language::{
    import_antlr, CharClassItem, GrammarExpr, GrammarFormat, GrammarImportError, RuleKind,
};

#[test]
fn imports_arithmetic_antlr_fixture() {
    let grammar =
        import_antlr(include_str!("../fixtures/grammar/antlr/arithmetic.g4")).expect("imports");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Antlr));
    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("expr")
    );
    assert_eq!(
        grammar.rule_names(),
        vec!["expr", "term", "factor", "INT", "ID", "WS"]
    );
    assert_eq!(grammar.rule("expr").expect("expr").kind(), RuleKind::Normal);
    assert_eq!(grammar.rule("INT").expect("INT").kind(), RuleKind::Token);
    assert_eq!(grammar.rule("ID").expect("ID").kind(), RuleKind::Token);
    assert_eq!(grammar.rule("WS").expect("WS").kind(), RuleKind::Token);
    assert_eq!(grammar.rule("WS").expect("WS").doc(), Some("-> skip"));

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
        grammar.rule("ID").expect("ID").expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::CharClass {
                negated: false,
                items: vec![
                    CharClassItem::Range('a', 'z'),
                    CharClassItem::Range('A', 'Z'),
                    CharClassItem::Char('_'),
                ],
            },
            GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::CharClass {
                negated: false,
                items: vec![
                    CharClassItem::Range('a', 'z'),
                    CharClassItem::Range('A', 'Z'),
                    CharClassItem::Char('_'),
                    CharClassItem::Range('0', '9'),
                ],
            })),
        ])
    );
}

#[test]
fn lowers_covering_antlr_constructs() {
    let grammar =
        import_antlr(include_str!("../fixtures/grammar/antlr/covering.g4")).expect("imports");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Antlr));
    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("entry")
    );
    assert_eq!(
        grammar.rule_names(),
        vec![
            "entry",
            "item",
            "literalRange",
            "TOKEN",
            "DIGIT",
            "COMMENT",
            "ACTIONED",
        ]
    );
    assert_eq!(
        grammar.rule("DIGIT").expect("DIGIT").kind(),
        RuleKind::Silent
    );
    assert_eq!(
        grammar.rule("TOKEN").expect("TOKEN").expr(),
        &GrammarExpr::CharClass {
            negated: false,
            items: vec![
                CharClassItem::Range('a', 'z'),
                CharClassItem::Range('0', '9'),
                CharClassItem::Char('_'),
            ],
        }
    );
    assert_eq!(
        grammar.rule("literalRange").expect("literalRange").expr(),
        &GrammarExpr::CharRange('a', 'z')
    );
    assert_eq!(
        grammar.rule("COMMENT").expect("COMMENT").doc(),
        Some("-> channel(HIDDEN)")
    );
    assert_eq!(
        grammar.rule("ACTIONED").expect("ACTIONED").doc(),
        Some("dropped predicate; dropped action; -> type(ID)")
    );

    assert_eq!(
        grammar.rule("entry").expect("entry").expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::Capture {
                label: Some("name".to_string()),
                expr: Box::new(GrammarExpr::NonTerminal("ID".to_string())),
            },
            GrammarExpr::Capture {
                label: Some("values".to_string()),
                expr: Box::new(GrammarExpr::Capture {
                    label: Some("non_greedy".to_string()),
                    expr: Box::new(GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::NonTerminal(
                        "item".to_string(),
                    )))),
                }),
            },
            GrammarExpr::NonTerminal("literalRange".to_string()),
            GrammarExpr::NonTerminal("item".to_string()),
        ])
    );
    assert_eq!(
        grammar.rule("item").expect("item").expr(),
        &GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::AnyChar,
                GrammarExpr::CharClass {
                    negated: true,
                    items: vec![CharClassItem::Char(';')],
                },
                GrammarExpr::Not(Box::new(GrammarExpr::Terminal("x".to_string()))),
            ],
        }
    );
}

#[test]
fn skips_lexer_mode_declarations() {
    let grammar =
        import_antlr(include_str!("../fixtures/grammar/antlr/lexer-mode.g4")).expect("imports");

    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("STRING_TEXT")
    );
    assert_eq!(grammar.rule_names(), vec!["STRING_TEXT"]);
    assert_eq!(
        grammar.rule("STRING_TEXT").expect("STRING_TEXT").expr(),
        &GrammarExpr::OneOrMore(Box::new(GrammarExpr::CharClass {
            negated: true,
            items: vec![CharClassItem::Char('"')],
        }))
    );
}

#[test]
fn unresolved_references_remain_visible_on_imported_grammar() {
    let grammar = import_antlr("grammar Missing; start : missing ;").expect("imports");

    assert_eq!(
        grammar.rule("start").expect("start").expr(),
        &GrammarExpr::NonTerminal("missing".to_string())
    );
    assert!(grammar.undefined_nonterminals().contains("missing"));
}

#[test]
fn skips_inline_comments_before_sequence_boundaries() {
    let grammar = import_antlr(
        "grammar Comments;
         start : 'a' // first branch
             | ('b' /* group end */) // second branch
             ;",
    )
    .expect("imports");

    assert_eq!(
        grammar.rule("start").expect("start").expr(),
        &GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::Terminal("a".to_string()),
                GrammarExpr::Terminal("b".to_string()),
            ],
        }
    );
}

#[test]
fn malformed_antlr_reports_parse_error() {
    let error = import_antlr("grammar Bad; start : ( missing ;").expect_err("parse error");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::Antlr,
            ..
        }
    ));
}

#[test]
fn unsupported_rule_prelude_reports_unsupported_error() {
    let error = import_antlr("grammar Bad; rule locals [int value] : 'x' ;")
        .expect_err("locals unsupported");

    assert!(matches!(
        error,
        GrammarImportError::Unsupported {
            format: GrammarFormat::Antlr,
            construct
        } if construct == "rule prelude locals"
    ));
}
