use meta_language::{import_ebnf, GrammarExpr, GrammarFormat, GrammarImportError, GrammarRule};

const ARITHMETIC: &str = include_str!("../fixtures/grammar/ebnf/arithmetic.ebnf");
const NUMBER: &str = include_str!("../fixtures/grammar/ebnf/number.ebnf");
const ISO14977_SAMPLE: &str = include_str!("../fixtures/grammar/ebnf/iso14977-sample.ebnf");

#[test]
fn imports_ebnf_fixtures_with_source_order_and_format() {
    let arithmetic = import_ebnf(ARITHMETIC).expect("arithmetic EBNF imports");
    assert_eq!(arithmetic.source_format(), Some(GrammarFormat::Ebnf));
    assert_eq!(
        arithmetic.rule_names(),
        vec!["expr", "term", "factor", "number", "digit"]
    );
    assert_eq!(arithmetic.start_rule().map(GrammarRule::name), Some("expr"));
    assert!(matches!(
        arithmetic.rule("expr").map(GrammarRule::expr),
        Some(GrammarExpr::Sequence(items)) if items.len() == 2
    ));

    let number = import_ebnf(NUMBER).expect("number EBNF imports");
    assert_eq!(number.rule_names(), vec!["integer", "signed", "digit"]);
    assert_rule_expr(
        &number,
        "integer",
        GrammarExpr::Sequence(vec![
            GrammarExpr::NonTerminal("digit".into()),
            GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::NonTerminal("digit".into()))),
        ]),
    );

    let sample = import_ebnf(ISO14977_SAMPLE).expect("ISO 14977 sample imports");
    assert_eq!(sample.start_rule().map(GrammarRule::name), Some("list"));
    assert_eq!(sample.rules().len(), 6);
}

#[test]
fn every_iso_ebnf_construct_lowers_to_the_documented_expression_variant() {
    let grammar = import_ebnf(
        r#"
reference = item ;
item = "i" ;
literal = "lit" ;
single_literal = 's' ;
sequence = "a", "b", "c" ;
choice = "a" | "b" ;
repetition = { item } ;
optional = [ item ] ;
group = ( item ) ;
grouped_empty_choice = ( item | ) ;
empty = ;
"#,
    )
    .expect("operator fixture imports");

    assert_rule_expr(
        &grammar,
        "reference",
        GrammarExpr::NonTerminal("item".into()),
    );
    assert_rule_expr(&grammar, "literal", GrammarExpr::Terminal("lit".into()));
    assert_rule_expr(
        &grammar,
        "single_literal",
        GrammarExpr::Terminal("s".into()),
    );
    assert_rule_expr(
        &grammar,
        "sequence",
        GrammarExpr::Sequence(vec![
            GrammarExpr::Terminal("a".into()),
            GrammarExpr::Terminal("b".into()),
            GrammarExpr::Terminal("c".into()),
        ]),
    );
    assert_rule_expr(
        &grammar,
        "choice",
        GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::Terminal("a".into()),
                GrammarExpr::Terminal("b".into()),
            ],
        },
    );
    assert_rule_expr(
        &grammar,
        "repetition",
        GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::NonTerminal("item".into()))),
    );
    assert_rule_expr(
        &grammar,
        "optional",
        GrammarExpr::Optional(Box::new(GrammarExpr::NonTerminal("item".into()))),
    );
    assert_rule_expr(&grammar, "group", GrammarExpr::NonTerminal("item".into()));
    assert_rule_expr(
        &grammar,
        "grouped_empty_choice",
        GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![GrammarExpr::NonTerminal("item".into()), GrammarExpr::Empty],
        },
    );
    assert_rule_expr(&grammar, "empty", GrammarExpr::Empty);
}

#[test]
fn unsupported_special_sequence_reports_unsupported_without_panicking() {
    let error = import_ebnf("token = ? implementation-defined token ? ;")
        .expect_err("special sequence is unsupported");

    assert!(matches!(
        error,
        GrammarImportError::Unsupported {
            format: GrammarFormat::Ebnf,
            ref construct,
        } if construct.contains("special sequence")
    ));
}

#[test]
fn inline_regex_reports_unsupported_without_panicking() {
    let error = import_ebnf("digit = #'[0-9]' ;").expect_err("regex is unsupported");

    assert!(matches!(
        error,
        GrammarImportError::Unsupported {
            format: GrammarFormat::Ebnf,
            ref construct,
        } if construct.contains("regex")
    ));
}

#[test]
fn malformed_ebnf_reports_parse_error_without_panicking() {
    let error = import_ebnf(r#"expr = "unterminated ;"#).expect_err("input is malformed");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::Ebnf,
            ..
        }
    ));
}

#[test]
fn undefined_nonterminal_is_reported() {
    let error = import_ebnf("expr = missing ;").expect_err("reference is undefined");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::Ebnf,
            ref message,
        } if message.contains("missing")
    ));
}

fn assert_rule_expr(grammar: &meta_language::Grammar, rule: &str, expected: GrammarExpr) {
    assert_eq!(
        grammar.rule(rule).map(|rule| rule.expr().clone()),
        Some(expected),
        "unexpected expression for {rule}"
    );
}
