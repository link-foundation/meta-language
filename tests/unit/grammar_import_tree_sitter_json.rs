use meta_language::{
    import_tree_sitter_json, CharClassItem, GrammarExpr, GrammarFormat, GrammarImportError,
    RuleKind,
};

#[test]
fn imports_covering_tree_sitter_json_fixture() {
    let grammar = import_tree_sitter_json(include_str!(
        "../fixtures/grammar/tree-sitter/covering.json"
    ))
    .expect("tree-sitter JSON imports");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::TreeSitter));
    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("document")
    );
    assert_eq!(
        grammar.rule_names(),
        vec![
            "document",
            "named",
            "value",
            "identifier",
            "number",
            "string",
            "list_tail",
            "plain_choice",
            "maybe",
            "prec_rule",
            "left_rule",
            "right_rule",
            "dynamic_rule",
            "immediate",
            "reserved_rule",
            "empty",
            "comment",
            "_extras",
        ]
    );

    assert_eq!(
        grammar.rule("document").expect("document").expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::NonTerminal("named".to_string()),
            GrammarExpr::Terminal("=".to_string()),
            GrammarExpr::Capture {
                label: Some("value".to_string()),
                expr: Box::new(GrammarExpr::NonTerminal("value".to_string())),
            },
            GrammarExpr::Optional(Box::new(GrammarExpr::NonTerminal("maybe".to_string()))),
        ])
    );
    assert_eq!(
        grammar.rule("named").expect("named").expr(),
        &GrammarExpr::Capture {
            label: Some("alias:name".to_string()),
            expr: Box::new(GrammarExpr::NonTerminal("identifier".to_string())),
        }
    );
    assert_eq!(
        grammar.rule("identifier").expect("identifier").expr(),
        &GrammarExpr::CharClass {
            negated: false,
            items: vec![
                CharClassItem::Range('a', 'z'),
                CharClassItem::Range('A', 'Z'),
                CharClassItem::Char('_'),
            ],
        }
    );
    assert_eq!(
        grammar.rule("number").expect("number").kind(),
        RuleKind::Token
    );
    assert_eq!(
        grammar.rule("number").expect("number").expr(),
        &GrammarExpr::CharClass {
            negated: false,
            items: vec![CharClassItem::Range('0', '9')],
        }
    );
    assert_eq!(
        grammar.rule("list_tail").expect("list_tail").expr(),
        &GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::Sequence(vec![
            GrammarExpr::Terminal(",".to_string()),
            GrammarExpr::NonTerminal("value".to_string()),
        ])))
    );
    assert_eq!(
        grammar.rule("plain_choice").expect("plain_choice").expr(),
        &GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::Terminal("a".to_string()),
                GrammarExpr::Terminal("b".to_string()),
                GrammarExpr::Empty,
            ],
        }
    );
    assert_eq!(
        grammar.rule("maybe").expect("maybe").expr(),
        &GrammarExpr::OneOrMore(Box::new(GrammarExpr::Terminal("?".to_string())))
    );
    assert_eq!(
        grammar.rule("prec_rule").expect("prec_rule").expr(),
        &GrammarExpr::Capture {
            label: Some("prec=-1".to_string()),
            expr: Box::new(GrammarExpr::Terminal("p".to_string())),
        }
    );
    assert_eq!(
        grammar.rule("left_rule").expect("left_rule").expr(),
        &GrammarExpr::Capture {
            label: Some("prec_left=2".to_string()),
            expr: Box::new(GrammarExpr::Terminal("l".to_string())),
        }
    );
    assert_eq!(
        grammar.rule("right_rule").expect("right_rule").expr(),
        &GrammarExpr::Capture {
            label: Some("prec_right=3".to_string()),
            expr: Box::new(GrammarExpr::Terminal("r".to_string())),
        }
    );
    assert_eq!(
        grammar.rule("dynamic_rule").expect("dynamic_rule").expr(),
        &GrammarExpr::Capture {
            label: Some("prec_dynamic=4".to_string()),
            expr: Box::new(GrammarExpr::Terminal("d".to_string())),
        }
    );
    assert_eq!(
        grammar.rule("immediate").expect("immediate").kind(),
        RuleKind::Token
    );
    assert_eq!(
        grammar.rule("immediate").expect("immediate").expr(),
        &GrammarExpr::Capture {
            label: Some("immediate_token".to_string()),
            expr: Box::new(GrammarExpr::Terminal("!".to_string())),
        }
    );
    assert_eq!(
        grammar.rule("reserved_rule").expect("reserved_rule").expr(),
        &GrammarExpr::Capture {
            label: Some("reserved:word".to_string()),
            expr: Box::new(GrammarExpr::NonTerminal("identifier".to_string())),
        }
    );
    assert_eq!(
        grammar.rule("_extras").expect("extras").expr(),
        &GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::Capture {
                    label: Some("regex".to_string()),
                    expr: Box::new(GrammarExpr::Terminal("\\s".to_string())),
                },
                GrammarExpr::NonTerminal("comment".to_string()),
            ],
        }
    );
}

#[test]
fn imports_real_tree_sitter_json_fixture() {
    let grammar =
        import_tree_sitter_json(include_str!("../fixtures/grammar/tree-sitter/json.json"))
            .expect("real tree-sitter JSON grammar imports");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::TreeSitter));
    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("document")
    );
    assert_eq!(&grammar.rule_names()[..3], ["document", "_value", "object"]);
    assert_eq!(
        grammar.rule("document").expect("document").expr(),
        &GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::NonTerminal("_value".to_string())))
    );
    assert!(grammar.rule("_extras").is_some());
    assert!(
        grammar.undefined_nonterminals().is_empty(),
        "real JSON grammar should resolve internal references"
    );
}

#[test]
fn preserves_first_rule_as_start_symbol() {
    let text = r#"{
  "name": "ordered",
  "rules": {
    "z_start": { "type": "STRING", "value": "z" },
    "a_rule": { "type": "STRING", "value": "a" }
  }
}"#;

    let grammar = import_tree_sitter_json(text).expect("ordered grammar imports");

    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("z_start")
    );
    assert_eq!(grammar.rule_names(), vec!["z_start", "a_rule"]);
}

#[test]
fn malformed_json_reports_parse_error() {
    let error = import_tree_sitter_json(r#"{"rules": {"#).expect_err("parse error");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::TreeSitter,
            ..
        }
    ));
}

#[test]
fn missing_rules_reports_parse_error() {
    let error = import_tree_sitter_json(r#"{"name": "missing"}"#).expect_err("missing rules");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::TreeSitter,
            ..
        }
    ));
}

#[test]
fn unknown_node_type_reports_unsupported_error() {
    let error = import_tree_sitter_json(
        r#"{"name":"unknown","rules":{"start":{"type":"WAT","value":"?"}}}"#,
    )
    .expect_err("unknown type");

    assert!(matches!(
        error,
        GrammarImportError::Unsupported {
            format: GrammarFormat::TreeSitter,
            construct
        } if construct == "WAT"
    ));
}
