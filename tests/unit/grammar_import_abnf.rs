use meta_language::{
    import_abnf, CharClassItem, GrammarExpr, GrammarFormat, GrammarImportError, GrammarRule,
};

const POSTAL_ADDRESS: &str = include_str!("../fixtures/grammar/abnf/postal-address.abnf");
const NUMERIC_TERMINALS: &str = include_str!("../fixtures/grammar/abnf/numeric-terminals.abnf");
const INCREMENTAL: &str = include_str!("../fixtures/grammar/abnf/incremental.abnf");

#[test]
fn imports_abnf_fixtures_with_source_order_core_rules_and_format() {
    let postal = import_abnf(POSTAL_ADDRESS).expect("postal-address ABNF imports");

    assert_eq!(postal.source_format(), Some(GrammarFormat::Abnf));
    assert_eq!(
        postal.start_rule().map(GrammarRule::name),
        Some("postal-address")
    );
    assert_eq!(
        &postal.rule_names()[..15],
        &[
            "postal-address",
            "name-part",
            "personal-part",
            "first-name",
            "initial",
            "last-name",
            "suffix",
            "street",
            "apt",
            "house-num",
            "street-name",
            "zip-part",
            "town-name",
            "state",
            "zip-code",
        ]
    );
    for core_rule in ["ALPHA", "DIGIT", "CRLF", "SP", "VCHAR"] {
        assert!(
            postal.rule(core_rule).is_some(),
            "missing injected core rule {core_rule}"
        );
    }
    assert!(postal.undefined_nonterminals().is_empty());

    assert_rule_expr(
        &postal,
        "first-name",
        GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::NonTerminal("ALPHA".into()))),
    );
    assert_rule_expr(
        &postal,
        "apt",
        GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::NonTerminal("DIGIT".into())),
            min: 1,
            max: Some(4),
        },
    );
    assert_rule_expr(
        &postal,
        "ALPHA",
        GrammarExpr::CharClass {
            negated: false,
            items: vec![
                CharClassItem::Range('A', 'Z'),
                CharClassItem::Range('a', 'z'),
            ],
        },
    );
}

#[test]
fn every_repetition_form_lowers_to_the_documented_expression_variant() {
    let grammar = import_abnf(
        r#"
zero = *"a"
one = 1*"a"
at-least-two = 2*"a"
at-most-three = *3"a"
between = 2*4"a"
exact = 3"a"
optional = ["a"]
group = ("a")
"#,
    )
    .expect("repetition fixture imports");

    assert_rule_expr(
        &grammar,
        "zero",
        GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::TerminalInsensitive("a".into()))),
    );
    assert_rule_expr(
        &grammar,
        "one",
        GrammarExpr::OneOrMore(Box::new(GrammarExpr::TerminalInsensitive("a".into()))),
    );
    assert_rule_expr(
        &grammar,
        "at-least-two",
        GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::TerminalInsensitive("a".into())),
            min: 2,
            max: None,
        },
    );
    assert_rule_expr(
        &grammar,
        "at-most-three",
        GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::TerminalInsensitive("a".into())),
            min: 0,
            max: Some(3),
        },
    );
    assert_rule_expr(
        &grammar,
        "between",
        GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::TerminalInsensitive("a".into())),
            min: 2,
            max: Some(4),
        },
    );
    assert_rule_expr(
        &grammar,
        "exact",
        GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::TerminalInsensitive("a".into())),
            min: 3,
            max: Some(3),
        },
    );
    assert_rule_expr(
        &grammar,
        "optional",
        GrammarExpr::Optional(Box::new(GrammarExpr::TerminalInsensitive("a".into()))),
    );
    assert_rule_expr(
        &grammar,
        "group",
        GrammarExpr::TerminalInsensitive("a".into()),
    );
}

#[test]
fn numeric_terminals_decode_radices_ranges_and_series() {
    let grammar = import_abnf(NUMERIC_TERMINALS).expect("numeric terminals import");

    assert_rule_expr(&grammar, "hex-a", GrammarExpr::Terminal("A".into()));
    assert_rule_expr(&grammar, "dec-a", GrammarExpr::Terminal("A".into()));
    assert_rule_expr(&grammar, "bin-a", GrammarExpr::Terminal("A".into()));
    assert_rule_expr(&grammar, "hex-digit", GrammarExpr::CharRange('0', '9'));
    assert_rule_expr(&grammar, "dec-digit", GrammarExpr::CharRange('0', '9'));
    assert_rule_expr(&grammar, "bin-digit", GrammarExpr::CharRange('0', '9'));
    assert_rule_expr(&grammar, "abc", GrammarExpr::Terminal("ABC".into()));
}

#[test]
fn case_sensitivity_and_incremental_alternatives_are_preserved() {
    let grammar = import_abnf(INCREMENTAL).expect("incremental alternatives import");

    assert_rule_expr(
        &grammar,
        "method",
        GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::TerminalInsensitive("get".into()),
                GrammarExpr::TerminalInsensitive("post".into()),
                GrammarExpr::Terminal("PATCH".into()),
                GrammarExpr::TerminalInsensitive("delete".into()),
            ],
        },
    );
}

#[test]
fn rule_names_are_resolved_case_insensitively() {
    let grammar = import_abnf(
        r#"
Message = header body
Header = "h"
BODY = "b"
message =/ trailer
Trailer = "t"
"#,
    )
    .expect("case-insensitive rule names import");

    assert_eq!(
        grammar.rule_names(),
        vec!["Message", "Header", "BODY", "Trailer"]
    );
    assert_rule_expr(
        &grammar,
        "Message",
        GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::Sequence(vec![
                    GrammarExpr::NonTerminal("Header".into()),
                    GrammarExpr::NonTerminal("BODY".into()),
                ]),
                GrammarExpr::NonTerminal("Trailer".into()),
            ],
        },
    );
    assert!(grammar.undefined_nonterminals().is_empty());
}

#[test]
fn prose_val_reports_unsupported_without_panicking() {
    let error = import_abnf("token = <implementation-defined token>\n")
        .expect_err("prose-val is unsupported");

    assert!(matches!(
        error,
        GrammarImportError::Unsupported {
            format: GrammarFormat::Abnf,
            ref construct,
        } if construct == "prose-val"
    ));
}

#[test]
fn malformed_abnf_reports_parse_error_without_panicking() {
    let error = import_abnf("expr = \"unterminated\n").expect_err("input is malformed");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::Abnf,
            ..
        }
    ));
}

#[test]
fn undefined_nonterminal_is_reported_after_core_rule_injection() {
    let error = import_abnf("expr = missing\n").expect_err("reference is undefined");

    assert!(matches!(
        error,
        GrammarImportError::Parse {
            format: GrammarFormat::Abnf,
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
