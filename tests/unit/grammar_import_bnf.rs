use meta_language::{import_bnf, GrammarExpr, GrammarFormat, GrammarImportError, GrammarRule};

const ARITHMETIC: &str = include_str!("../fixtures/grammar/bnf/arithmetic.bnf");
const LIST: &str = include_str!("../fixtures/grammar/bnf/list.bnf");
const POSTAL_ADDRESS: &str = include_str!("../fixtures/grammar/bnf/postal-address.bnf");

#[test]
fn imports_bnf_fixtures_with_source_order_and_format() {
    let arithmetic = import_bnf(ARITHMETIC).expect("arithmetic BNF imports");
    assert_eq!(arithmetic.source_format(), Some(GrammarFormat::Bnf));
    assert_eq!(
        arithmetic.rule_names(),
        vec!["expr", "term", "factor", "number", "digit"]
    );
    assert_eq!(arithmetic.start_rule().map(GrammarRule::name), Some("expr"));
    assert!(matches!(
        arithmetic.rule("expr").map(GrammarRule::expr),
        Some(GrammarExpr::Choice {
            ordered: false,
            alternatives,
        }) if alternatives.len() == 3
    ));

    let list = import_bnf(LIST).expect("recursive list BNF imports");
    assert_eq!(list.rule_names(), vec!["list", "item"]);
    assert_rule_expr(
        &list,
        "item",
        GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::Terminal("a".into()),
                GrammarExpr::Terminal("b".into()),
            ],
        },
    );

    let postal = import_bnf(POSTAL_ADDRESS).expect("postal-address BNF imports");
    assert_eq!(
        postal.start_rule().map(GrammarRule::name),
        Some("postal-address")
    );
    assert_eq!(postal.rules().len(), 18);
}

#[test]
fn every_classic_bnf_construct_lowers_to_the_documented_expression_variant() {
    let grammar = import_bnf(
        r#"
<reference> ::= <item>
<item> ::= "i"
<literal> ::= "lit"
<single-literal> ::= 's'
<sequence> ::= "a" "b" "c"
<choice> ::= "a" | "b"
<epsilon> ::= |
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
        "single-literal",
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
    assert_rule_expr(&grammar, "epsilon", GrammarExpr::Empty);
}

#[test]
fn malformed_bnf_reports_parse_error_without_panicking() {
    let error = import_bnf(r#"<expr> ::= "unterminated"#).expect_err("input is malformed");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::Bnf,
            ..
        }
    ));
}

#[test]
fn undefined_nonterminal_is_reported() {
    let error = import_bnf("<expr> ::= <missing>").expect_err("reference is undefined");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::Bnf,
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
