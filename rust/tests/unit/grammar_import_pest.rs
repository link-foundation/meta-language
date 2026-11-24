use meta_language::grammar::{
    feature_runtime::{FeatureParseOptions, GrammarSearchOptions, compile_feature_grammar},
    parse_grammar_links, render_grammar_links,
};
use serde_json::Value;

use super::issue_195_observations::{Observation, record};
use meta_language::{
    GrammarExpr, GrammarFormat, GrammarImportError, RuleKind, emit_pest, import_pest,
};

fn whitespace_between_items() -> GrammarExpr {
    parse_grammar_links(
        "(grammar (start skip))\n(rule skip silent (token (repeat0 (ref WHITESPACE))))",
    )
    .expect("whitespace form decodes")
    .start_rule()
    .expect("skip rule")
    .expr()
    .clone()
}

fn assert_accepts(grammar: &meta_language::Grammar, accepted: &[&str], rejected: &[&str]) {
    let options = FeatureParseOptions::default();
    let parser =
        compile_feature_grammar(grammar, None, options.clone()).expect("imported grammar compiles");
    for input in accepted {
        assert!(
            parser
                .parse_tree(input.as_bytes(), &options)
                .expect("parse outcome")
                .ok,
            "accepted {input:?}"
        );
    }
    for input in rejected {
        assert!(
            !parser
                .parse_tree(input.as_bytes(), &options)
                .expect("parse outcome")
                .ok,
            "rejected {input:?}"
        );
    }
}

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
        vec![
            "expr",
            "term",
            "factor",
            "number",
            "WHITESPACE",
            "ASCII_DIGIT",
            "NEWLINE"
        ]
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
    let term = GrammarExpr::NonTerminal("term".to_string());
    let skip = whitespace_between_items();
    let operator = GrammarExpr::Choice {
        ordered: true,
        alternatives: vec![
            GrammarExpr::Terminal("+".to_string()),
            GrammarExpr::Terminal("-".to_string()),
        ],
    };
    assert_eq!(
        expr,
        &GrammarExpr::Sequence(vec![
            term.clone(),
            skip.clone(),
            GrammarExpr::optional(GrammarExpr::Sequence(vec![
                operator.clone(),
                skip.clone(),
                term.clone(),
                GrammarExpr::zero_or_more(GrammarExpr::Sequence(vec![
                    skip.clone(),
                    operator,
                    skip,
                    term
                ]))
            ])),
        ])
    );
    assert_accepts(&grammar, &["2+3", "12 *\r\n(3 + 4)"], &["1 2", "2+", " 2"]);
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
            whitespace_between_items(),
            GrammarExpr::NonTerminal("value".to_string()),
            whitespace_between_items(),
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
    assert_accepts(
        &grammar,
        &[" null ", "\"a b\"", "\t42.5\n"],
        &["1 2", "true false"],
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
        RuleKind::Token
    );
    assert_eq!(
        grammar.rule("non_atomic").expect("non_atomic").kind(),
        RuleKind::Normal
    );
}

#[test]
fn resolves_pest_builtins_as_nonterminals() {
    let grammar = import_pest(r"start = { SOI ~ ASCII_DIGIT+ ~ EOI }").expect("builtins import");

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
    assert!(grammar.undefined_nonterminals().is_empty());
    assert_accepts(&grammar, &["2024"], &["", "٢", " 2024"]);
}

#[test]
fn pest_built_in_rules_implicit_trivia_and_diagnostics() {
    const FIXTURE: &str = "parity/fixtures/pest-built-ins.lino";
    const TEST: &str = "pest_built_in_rules_implicit_trivia_and_diagnostics";
    let cases: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/pest-built-ins.lino"
    ))
    .expect("shared pest fixture");
    assert_eq!(cases["pestVersion"].as_str(), Some("2.9.2"));
    let observe = |requirement_id: &str, assertions: &[&str]| {
        record(&Observation {
            requirement_id,
            suffix: "behavior",
            fixture_id: &requirement_id.to_ascii_lowercase(),
            fixture_file: FIXTURE,
            assertions,
            test_name: TEST,
        });
    };
    for group in ["builtinCases", "triviaCases"] {
        for case in cases[group].as_array().expect("pest cases") {
            let grammar =
                import_pest(case[0].as_str().expect("pest source")).expect("pest grammar");
            assert!(grammar.undefined_nonterminals().is_empty());
            let restored =
                parse_grammar_links(&render_grammar_links(&grammar)).expect("pest links restore");
            let inputs = |index: usize| {
                case[index]
                    .as_array()
                    .expect("pest inputs")
                    .iter()
                    .map(|source| source.as_str().expect("pest input"))
                    .collect::<Vec<_>>()
            };
            assert_accepts(&restored, &inputs(1), &inputs(2));
            let (emitted, _) = emit_pest(&restored).expect("pest emits");
            let imported = import_pest(&emitted).expect("emitted pest imports");
            assert_accepts(&imported, &inputs(1), &inputs(2));
        }
    }
    let anchor = &cases["anchorSearch"];
    let grammar =
        import_pest(anchor["grammar"].as_str().expect("anchor grammar")).expect("pest imports");
    let parser = compile_feature_grammar(&grammar, None, FeatureParseOptions::default())
        .expect("pest grammar compiles");
    let matches = parser
        .find_all(
            anchor["source"].as_str().expect("anchor search source"),
            &GrammarSearchOptions::default(),
        )
        .expect("pest search");
    let spans: Vec<_> = matches
        .iter()
        .map(|matched| [matched.span.start, matched.span.end])
        .collect();
    assert_eq!(serde_json::json!(spans), anchor["spans"]);
    observe(
        "I195-I234-PEST-BUILT-IN-RULES",
        &["builtInRulesExecute", "anchorsUseAbsolutePositions"],
    );
    observe(
        "I195-I234-PEST-IMPLICIT-TRIVIA",
        &["implicitTriviaTracksAtomicContext"],
    );
    for case in cases["diagnostics"]
        .as_array()
        .expect("pest diagnostic cases")
    {
        let error =
            import_pest(case[0].as_str().expect("pest source")).expect_err("pest import error");
        assert_eq!(
            error.to_string(),
            case[1].as_str().expect("pest diagnostic")
        );
    }
    observe(
        "I195-I234-PEST-RULE-DIAGNOSTICS",
        &["pestRuleDiagnosticsUseExactMessages"],
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
    let err = import_pest(r"start = { missing }").expect_err("undefined rule");

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
    let err = import_pest(r"peek = { PEEK[..] }").expect_err("peek slice unsupported");

    assert!(matches!(
        err,
        GrammarImportError::Unsupported {
            format: GrammarFormat::Peg,
            construct
        } if construct == "PeekSlice"
    ));
}
