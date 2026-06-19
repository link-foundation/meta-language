use meta_language::{import_pest, GrammarExpr, GrammarFormat, GrammarImportError, RuleKind};

#[test]
fn imports_arithmetic_pest_fixture() {
    let text = include_str!("../fixtures/grammar/peg/arithmetic.pest");
    let grammar = import_pest(text).expect("arithmetic pest imports");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Peg));
    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("expr")
    );
    assert_eq!(
        grammar.rule_names(),
        vec!["expr", "term", "factor", "number", "WHITESPACE"]
    );
    assert_eq!(
        grammar.rule("number").expect("number rule").kind(),
        RuleKind::Atomic
    );
    assert_eq!(
        grammar.rule("WHITESPACE").expect("whitespace rule").kind(),
        RuleKind::Silent
    );

    let expr = grammar.rule("expr").expect("expr rule").expr();
    assert_eq!(
        expr,
        &GrammarExpr::Sequence(vec![
            GrammarExpr::NonTerminal("term".to_string()),
            GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::Sequence(vec![
                GrammarExpr::Choice {
                    ordered: true,
                    alternatives: vec![
                        GrammarExpr::Terminal("+".to_string()),
                        GrammarExpr::Terminal("-".to_string()),
                    ],
                },
                GrammarExpr::NonTerminal("term".to_string()),
            ]))),
        ])
    );
}

#[test]
fn imports_json_pest_fixture() {
    let text = include_str!("../fixtures/grammar/peg/json.pest");
    let grammar = import_pest(text).expect("json pest imports");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::Peg));
    assert_eq!(
        grammar.start_rule().map(|rule| rule.name.as_str()),
        Some("json")
    );
    assert_eq!(
        grammar.rule("string").expect("string rule").kind(),
        RuleKind::Atomic
    );
    assert_eq!(
        grammar.rule("value").expect("value rule").kind(),
        RuleKind::Silent
    );
    assert_eq!(
        grammar.rule("digit").expect("digit rule").kind(),
        RuleKind::Silent
    );

    assert_eq!(
        grammar.rule("json").expect("json rule").expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::NonTerminal("SOI".to_string()),
            GrammarExpr::NonTerminal("value".to_string()),
            GrammarExpr::NonTerminal("EOI".to_string()),
        ])
    );
    assert_eq!(
        grammar.rule("digit").expect("digit rule").expr(),
        &GrammarExpr::CharRange('0', '9')
    );
    assert_eq!(
        grammar.rule("boolean").expect("boolean rule").expr(),
        &GrammarExpr::Choice {
            ordered: true,
            alternatives: vec![
                GrammarExpr::TerminalInsensitive("true".to_string()),
                GrammarExpr::TerminalInsensitive("false".to_string()),
            ],
        }
    );
    assert_eq!(
        grammar.rule("any_value").expect("any_value rule").expr(),
        &GrammarExpr::AnyChar
    );
}

#[test]
fn lowers_predicates_and_repetition_forms() {
    let text = include_str!("../fixtures/grammar/peg/predicates.pest");
    let grammar = import_pest(text).expect("predicate pest imports");

    assert_eq!(
        grammar.rule("prefix").expect("prefix rule").expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::And(Box::new(GrammarExpr::Terminal("a".to_string()))),
            GrammarExpr::Not(Box::new(GrammarExpr::Terminal("b".to_string()))),
            GrammarExpr::AnyChar,
        ])
    );
    assert_eq!(
        grammar.rule("exact").expect("exact rule").expr(),
        &GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::Terminal("a".to_string())),
            min: 2,
            max: Some(2),
        }
    );
    assert_eq!(
        grammar.rule("min").expect("min rule").expr(),
        &GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::Terminal("b".to_string())),
            min: 2,
            max: None,
        }
    );
    assert_eq!(
        grammar.rule("max").expect("max rule").expr(),
        &GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::Terminal("c".to_string())),
            min: 0,
            max: Some(3),
        }
    );
    assert_eq!(
        grammar.rule("range").expect("range rule").expr(),
        &GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::Terminal("d".to_string())),
            min: 1,
            max: Some(3),
        }
    );
    assert_eq!(
        grammar.rule("optional").expect("optional rule").expr(),
        &GrammarExpr::Optional(Box::new(GrammarExpr::Terminal("e".to_string())))
    );
    assert_eq!(
        grammar.rule("zero").expect("zero rule").expr(),
        &GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::Terminal("f".to_string())))
    );
    assert_eq!(
        grammar.rule("one").expect("one rule").expr(),
        &GrammarExpr::OneOrMore(Box::new(GrammarExpr::Terminal("g".to_string())))
    );
}

#[test]
fn maps_rule_modifiers() {
    let text = r#"
normal = { "n" }
silent = _{ "s" }
atomic = @{ "a" }
compound = ${ "c" }
non_atomic = !{ "x" }
"#;
    let grammar = import_pest(text).expect("modifiers import");

    assert_eq!(
        grammar.rule("normal").expect("normal").kind(),
        RuleKind::Normal
    );
    assert_eq!(
        grammar.rule("silent").expect("silent").kind(),
        RuleKind::Silent
    );
    assert_eq!(
        grammar.rule("atomic").expect("atomic").kind(),
        RuleKind::Atomic
    );
    assert_eq!(
        grammar.rule("compound").expect("compound").kind(),
        RuleKind::Atomic
    );
    assert_eq!(
        grammar.rule("non_atomic").expect("non_atomic").kind(),
        RuleKind::Normal
    );
}

#[test]
fn allows_pest_builtins_as_nonterminals() {
    let grammar = import_pest(r#"start = { SOI ~ ASCII_DIGIT+ ~ EOI }"#).expect("builtins import");

    assert_eq!(
        grammar.rule("start").expect("start rule").expr(),
        &GrammarExpr::Sequence(vec![
            GrammarExpr::NonTerminal("SOI".to_string()),
            GrammarExpr::OneOrMore(Box::new(GrammarExpr::NonTerminal(
                "ASCII_DIGIT".to_string(),
            ))),
            GrammarExpr::NonTerminal("EOI".to_string()),
        ])
    );
}

#[test]
fn malformed_pest_reports_parse_error() {
    let err = import_pest(r#"start = { "unterminated }"#).expect_err("parse error");

    assert!(matches!(
        err,
        GrammarImportError::Parse {
            format: GrammarFormat::Peg,
            ..
        }
    ));
}

#[test]
fn undefined_nonterminal_reports_parse_error() {
    let err = import_pest(r#"start = { missing }"#).expect_err("undefined rule");

    assert!(matches!(
        err,
        GrammarImportError::Parse {
            format: GrammarFormat::Peg,
            ..
        }
    ));
    assert!(err.to_string().contains("missing"));
}

#[test]
fn push_reports_unsupported() {
    let err = import_pest(r#"stack = { PUSH("a") }"#).expect_err("push unsupported");

    assert!(matches!(
        err,
        GrammarImportError::Unsupported {
            format: GrammarFormat::Peg,
            construct
        } if construct == "Push"
    ));
}

#[test]
fn peek_slice_reports_unsupported() {
    let err = import_pest(r#"peek = { PEEK[..] }"#).expect_err("peek slice unsupported");

    assert!(matches!(
        err,
        GrammarImportError::Unsupported {
            format: GrammarFormat::Peg,
            construct
        } if construct == "PeekSlice"
    ));
}
