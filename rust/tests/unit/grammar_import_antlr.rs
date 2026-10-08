use meta_language::{
    CharClassItem, GrammarExpr, GrammarFormat, GrammarImportError, RuleKind, import_antlr,
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
            "ID",
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
                GrammarExpr::CharClass {
                    negated: true,
                    items: vec![CharClassItem::Char('x')],
                },
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
    let error =
        import_antlr("grammar Bad; rule [int value] : 'x' ;").expect_err("arguments unsupported");

    assert!(matches!(
        error,
        GrammarImportError::Unsupported {
            format: GrammarFormat::Antlr,
            construct
        } if construct == "rule arguments"
    ));
}

#[test]
fn imports_the_lexer_features_the_grammars_v4_grammars_use() {
    use meta_language::grammar::UnicodeClassItem;
    use meta_language::grammar::feature::class_expression;
    use meta_language::{FeatureParseOptions, compile_feature_grammar};

    let grammar =
        import_antlr(include_str!("../fixtures/grammar/antlr/lexer-features.g4")).expect("imports");
    let doc = grammar.rule("doc").expect("doc");
    assert_eq!(doc.doc(), Some("// read up to the end; alternative Items"));
    assert_eq!(
        doc.expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::OneOrMore(Box::new(GrammarExpr::NonTerminal("ITEM".to_string()))),
            GrammarExpr::Not(Box::new(GrammarExpr::AnyChar)),
        ])
    );
    assert_eq!(
        grammar.rule("ITEM").expect("ITEM").expr(),
        &class_expression(
            false,
            vec![
                UnicodeClassItem::Range('A', 'Z'),
                UnicodeClassItem::Category("Nd".to_string()),
                UnicodeClassItem::Script("Greek".to_string()),
            ]
        )
    );
    let channels: Vec<Option<&str>> = grammar
        .rules()
        .iter()
        .map(|rule| rule.attributes.channel.as_deref())
        .collect();
    assert_eq!(channels, vec![None, None, Some("skip"), Some("HIDDEN")]);
    let options = FeatureParseOptions::default();
    let parser = compile_feature_grammar(&grammar, None, options.clone()).expect("compiles");
    let accepts = |text: &str| {
        parser
            .parse_tree(text.as_bytes(), &options)
            .is_ok_and(|outcome| outcome.tree.is_some())
    };
    assert!(accepts("AB 1 α # a comment"));
    assert!(!accepts("AB a"));

    for (source, expected) in [
        ("grammar P; a : [\\p{Emoji}] ;", "Unicode property Emoji"),
        (
            "grammar P; a : [\\P{L}] ;",
            "negated Unicode property in character set",
        ),
    ] {
        let error = import_antlr(source).expect_err(source);
        assert!(
            matches!(&error, GrammarImportError::Unsupported { format: GrammarFormat::Antlr, construct } if construct == expected),
            "{source}: {error}"
        );
    }
    let error = import_antlr("grammar Bad; start : 'x' # ;").expect_err("label");
    assert_eq!(
        error.to_string(),
        "antlr import parse error: expected alternative label at byte 27"
    );
}

#[test]
fn complements_a_set_of_characters_and_matches_one_character_outside_it() {
    use meta_language::grammar::UnicodeClassItem;
    use meta_language::grammar::feature::class_expression;
    use meta_language::{FeatureParseOptions, compile_feature_grammar};

    let grammar =
        import_antlr(include_str!("../fixtures/grammar/antlr/set-complement.g4")).expect("imports");
    let GrammarExpr::Sequence(string) = grammar.rule("STRING").expect("STRING").expr() else {
        panic!("STRING is a sequence");
    };
    assert_eq!(
        string[1],
        GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::Terminal("\"\"".to_string()),
                GrammarExpr::CharClass {
                    negated: true,
                    items: vec![CharClassItem::Char('"')],
                },
            ],
        }))
    );
    assert_eq!(
        grammar.rule("SET").expect("SET").expr(),
        &GrammarExpr::Sequence(vec![
            class_expression(
                true,
                vec![
                    UnicodeClassItem::Char('a'),
                    UnicodeClassItem::Range('b', 'c'),
                    UnicodeClassItem::Char('d'),
                    UnicodeClassItem::Category("Nd".to_string()),
                ]
            ),
            GrammarExpr::Not(Box::new(GrammarExpr::Terminal("xy".to_string()))),
        ])
    );
    let options = FeatureParseOptions::default();
    let parser = compile_feature_grammar(&grammar, None, options.clone()).expect("compiles");
    let accepts = |text: &str| {
        parser
            .parse_tree(text.as_bytes(), &options)
            .is_ok_and(|outcome| outcome.tree.is_some())
    };
    assert!(accepts("\"café, é\"\"x\"\"\""));
    assert!(!accepts("\"open"));
}

#[test]
fn left_recursive_alternatives_climb_by_precedence_with_surrogate_sets_and_numeric_channels() {
    use meta_language::grammar::feature::{FeatureExpr, FeatureForm, FieldValue};
    use meta_language::{FeatureParseOptions, compile_feature_grammar};

    let grammar =
        import_antlr(include_str!("../fixtures/grammar/antlr/precedence.g4")).expect("imports");
    let binary = |operator: &str| {
        GrammarExpr::Sequence(vec![
            GrammarExpr::NonTerminal("expr".to_string()),
            GrammarExpr::Terminal(operator.to_string()),
            GrammarExpr::NonTerminal("expr".to_string()),
        ])
    };
    let precedence = |level: i64, associativity: &str, operator: &str| {
        GrammarExpr::feature(FeatureExpr::Form(FeatureForm::new(
            "precedence",
            vec![
                FieldValue::Integer(level),
                FieldValue::Word(associativity.to_string()),
                FieldValue::Expression(binary(operator)),
            ],
        )))
    };
    assert_eq!(
        grammar.rule("expr").expect("expr").expr(),
        &GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                precedence(4, "right", "^"),
                precedence(3, "left", "*"),
                precedence(2, "left", "+"),
                GrammarExpr::NonTerminal("ID".to_string()),
            ],
        }
    );
    assert_eq!(
        grammar.rule("ID").expect("ID").expr(),
        &GrammarExpr::OneOrMore(Box::new(GrammarExpr::CharClass {
            negated: true,
            items: vec![CharClassItem::Range('\u{0}', '@')],
        }))
    );
    assert_eq!(
        grammar.rule("LONE").expect("LONE").expr(),
        &GrammarExpr::CharClass {
            negated: false,
            items: Vec::new(),
        }
    );
    assert_eq!(
        grammar
            .rule("NL")
            .expect("NL")
            .attributes
            .channel
            .as_deref(),
        Some("2")
    );
    let options = FeatureParseOptions::default();
    let parser = compile_feature_grammar(&grammar, None, options.clone()).expect("compiles");
    let accepts = |text: &str| {
        parser
            .parse_tree(text.as_bytes(), &options)
            .is_ok_and(|outcome| outcome.tree.is_some())
    };
    assert!(accepts("a^b^c*d+é"));
    assert!(!accepts("a+"));

    for (source, message) in [
        (
            "grammar P; e : <assoc=up> e 'x' e | 'y' ;",
            "antlr import unsupported construct: associativity up",
        ),
        (
            "grammar P; e : [] ;",
            "antlr import parse error: character class must not be empty at byte 15",
        ),
    ] {
        let error = import_antlr(source).expect_err(source);
        assert_eq!(error.to_string(), message, "{source}");
    }
}

#[test]
fn case_insensitive_options_match_either_case_and_rule_preludes_join_the_doc() {
    use meta_language::{FeatureParseOptions, compile_feature_grammar};

    let grammar = import_antlr(include_str!(
        "../fixtures/grammar/antlr/case-insensitive.g4"
    ))
    .expect("imports");
    let class = |items: Vec<CharClassItem>| GrammarExpr::CharClass {
        negated: false,
        items,
    };
    assert_eq!(
        grammar.rule("ECHO").expect("ECHO").expr(),
        &GrammarExpr::TerminalInsensitive("echo".to_string())
    );
    assert_eq!(
        grammar.rule("WORD").expect("WORD").expr(),
        &GrammarExpr::Sequence(vec![
            class(vec![
                CharClassItem::Range('a', 'c'),
                CharClassItem::Range('A', 'C')
            ]),
            GrammarExpr::OneOrMore(Box::new(class(vec![
                CharClassItem::Range('x', 'z'),
                CharClassItem::Char('_'),
                CharClassItem::Range('X', 'Z'),
            ]))),
        ])
    );
    assert_eq!(
        grammar.rule("NAME").expect("NAME").expr(),
        &GrammarExpr::OneOrMore(Box::new(class(vec![CharClassItem::Range('a', 'z')])))
    );
    assert_eq!(
        grammar.rule("TAGGED").expect("TAGGED").doc(),
        Some("dropped returns [int count]; dropped locals [int indexBefore = -1]")
    );
    let options = FeatureParseOptions::default();
    let parser = compile_feature_grammar(&grammar, None, options.clone()).expect("compiles");
    let accepts = |text: &str| {
        parser
            .parse_tree(text.as_bytes(), &options)
            .is_ok_and(|outcome| outcome.tree.is_some())
    };
    assert!(accepts("EcHo Bx_Z <abc>"));
    assert!(!accepts("<ABC>"));

    for (source, message) in [
        (
            "grammar P; r throws : 'x' ;",
            "antlr import parse error: expected exception name at byte 20",
        ),
        (
            "grammar P; r locals : 'x' ;",
            "antlr import parse error: expected ':' before rule body at byte 13",
        ),
    ] {
        let error = import_antlr(source).expect_err(source);
        assert_eq!(error.to_string(), message, "{source}");
    }
}

#[test]
fn type_commands_let_a_rule_match_where_the_type_does() {
    use meta_language::{FeatureParseOptions, compile_feature_grammar};

    let grammar =
        import_antlr(include_str!("../fixtures/grammar/antlr/retype.g4")).expect("imports");
    let rule = |name: &str| grammar.rule(name).expect(name);
    assert_eq!(
        rule("NL").expr(),
        &GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::Sequence(vec![
                    GrammarExpr::Optional(Box::new(GrammarExpr::Terminal("\r".to_string()))),
                    GrammarExpr::Terminal("\n".to_string()),
                ]),
                GrammarExpr::NonTerminal("COMMENT".to_string()),
            ],
        }
    );
    assert_eq!(
        rule("NL").doc(),
        Some("also COMMENT, which -> type(NL) retypes")
    );
    assert_eq!(rule("COMMENT").doc(), Some("-> type(NL)"));
    assert_eq!(
        rule("WORD").expr(),
        &GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::NonTerminal("NAME".to_string()),
                GrammarExpr::NonTerminal("NUMBER".to_string()),
            ],
        }
    );
    assert_eq!(rule("WORD").kind(), RuleKind::Token);
    assert_eq!(grammar.rule_names().last(), Some(&"WORD"));
    assert_eq!(
        rule("HIDDEN_NOTE").attributes.channel.as_deref(),
        Some("HIDDEN")
    );
    let options = FeatureParseOptions::default();
    let parser = compile_feature_grammar(&grammar, None, options.clone()).expect("compiles");
    let accepts = |text: &str| {
        parser
            .parse_tree(text.as_bytes(), &options)
            .is_ok_and(|outcome| outcome.tree.is_some())
    };
    assert!(accepts("ab 12 # note\ncd\n"));
    assert!(!accepts("ab ; cd\n"));
}

#[test]
fn braced_unicode_literals_and_ranges_retain_scalar_values() {
    let grammar =
        import_antlr(include_str!("../fixtures/grammar/antlr/braced-unicode.g4")).unwrap();
    assert_eq!(
        grammar.rule("DIGIT").unwrap().expr(),
        &GrammarExpr::CharRange('0', '9')
    );
    assert_eq!(
        grammar.rule("EMOJI").unwrap().expr(),
        &GrammarExpr::Terminal("😀".to_owned())
    );
    assert_eq!(
        grammar.rule("LIMIT").unwrap().expr(),
        &GrammarExpr::Terminal("\u{10ffff}".to_owned())
    );
    assert_eq!(
        grammar.rule("NUL").unwrap().expr(),
        &GrammarExpr::Terminal("\0".to_owned())
    );
    for escape in [
        r"\u{}",
        r"\u{D800}",
        r"\u{110000}",
        r"\u{1234567}",
        r"\u{12",
        r"\u{xyz}",
    ] {
        let source = format!("grammar Invalid; entry: '{escape}';");
        let error = import_antlr(&source).unwrap_err();
        assert!(
            error.to_string().contains("invalid braced unicode escape"),
            "{error}"
        );
    }
}
