use meta_language::{
    grammar_from_lino, grammar_to_lino, parse_grammar_surface, write_grammar_surface,
    CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule, GrammarSurfaceError,
};

#[test]
fn parses_arithmetic_surface_into_meta_language_grammar() {
    let grammar = parse_grammar_surface(
        r#"
(expr: term (( "+" / "-" ) term)*)
(term: factor (( "*" / "/" ) factor)*)
(factor: number / "(" expr ")")
(number: [0-9]+)
"#,
    )
    .expect("arithmetic surface parses");

    assert_eq!(grammar.source_format(), Some(GrammarFormat::MetaLanguage));
    assert_eq!(
        grammar.rule_names(),
        vec!["expr", "term", "factor", "number"]
    );
    assert_eq!(grammar.start_rule().map(GrammarRule::name), Some("expr"));
    assert!(matches!(
        grammar.rule("expr").map(GrammarRule::expr),
        Some(GrammarExpr::Sequence(items)) if items.len() == 2
    ));
    assert_eq!(
        grammar.rule("number").map(GrammarRule::expr),
        Some(&GrammarExpr::OneOrMore(Box::new(GrammarExpr::CharRange(
            '0', '9'
        ))))
    );
}

#[test]
fn every_surface_form_lowers_to_the_documented_expression_variant() {
    let grammar = parse_grammar_surface(
        r#"
(start: reference)
(reference: item)
(item: "i")
(literal: "lit")
(empty_literal: "")
(escaped_literal: "a""b")
(single_literal: 's')
(insensitive: `lit`)
(range: [a-z])
(quoted_range: ['a' 'z'])
(class: [a b c])
(negated_class: [^ a b])
(any: .)
(sequence: "a" "b" "c")
(ordered: "a" / "b")
(unordered: "a" | "b")
(optional: "a"?)
(zero_or_more: "a"*)
(one_or_more: "a"+)
(bounded: "a"{2,4})
(unbounded: "a"{1,})
(and_predicate: & "a")
(not_predicate: ! "a")
(capture: { label : "a" })
(epsilon: ())
"#,
    )
    .expect("operator fixture parses");

    assert_eq!(grammar.start(), Some("reference"));
    assert_rule_expr(
        &grammar,
        "reference",
        GrammarExpr::NonTerminal("item".into()),
    );
    assert_rule_expr(&grammar, "literal", GrammarExpr::Terminal("lit".into()));
    assert_rule_expr(
        &grammar,
        "empty_literal",
        GrammarExpr::Terminal(String::new()),
    );
    assert_rule_expr(
        &grammar,
        "escaped_literal",
        GrammarExpr::Terminal("a\"b".into()),
    );
    assert_rule_expr(
        &grammar,
        "single_literal",
        GrammarExpr::Terminal("s".into()),
    );
    assert_rule_expr(
        &grammar,
        "insensitive",
        GrammarExpr::TerminalInsensitive("lit".into()),
    );
    assert_rule_expr(&grammar, "range", GrammarExpr::CharRange('a', 'z'));
    assert_rule_expr(&grammar, "quoted_range", GrammarExpr::CharRange('a', 'z'));
    assert_rule_expr(
        &grammar,
        "class",
        GrammarExpr::CharClass {
            negated: false,
            items: vec![
                CharClassItem::Char('a'),
                CharClassItem::Char('b'),
                CharClassItem::Char('c'),
            ],
        },
    );
    assert_rule_expr(
        &grammar,
        "negated_class",
        GrammarExpr::CharClass {
            negated: true,
            items: vec![CharClassItem::Char('a'), CharClassItem::Char('b')],
        },
    );
    assert_rule_expr(&grammar, "any", GrammarExpr::AnyChar);
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
        "ordered",
        GrammarExpr::Choice {
            ordered: true,
            alternatives: vec![
                GrammarExpr::Terminal("a".into()),
                GrammarExpr::Terminal("b".into()),
            ],
        },
    );
    assert_rule_expr(
        &grammar,
        "unordered",
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
        "optional",
        GrammarExpr::Optional(Box::new(GrammarExpr::Terminal("a".into()))),
    );
    assert_rule_expr(
        &grammar,
        "zero_or_more",
        GrammarExpr::ZeroOrMore(Box::new(GrammarExpr::Terminal("a".into()))),
    );
    assert_rule_expr(
        &grammar,
        "one_or_more",
        GrammarExpr::OneOrMore(Box::new(GrammarExpr::Terminal("a".into()))),
    );
    assert_rule_expr(
        &grammar,
        "bounded",
        GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::Terminal("a".into())),
            min: 2,
            max: Some(4),
        },
    );
    assert_rule_expr(
        &grammar,
        "unbounded",
        GrammarExpr::Repeat {
            expr: Box::new(GrammarExpr::Terminal("a".into())),
            min: 1,
            max: None,
        },
    );
    assert_rule_expr(
        &grammar,
        "and_predicate",
        GrammarExpr::And(Box::new(GrammarExpr::Terminal("a".into()))),
    );
    assert_rule_expr(
        &grammar,
        "not_predicate",
        GrammarExpr::Not(Box::new(GrammarExpr::Terminal("a".into()))),
    );
    assert_rule_expr(
        &grammar,
        "capture",
        GrammarExpr::Capture {
            label: Some("label".into()),
            expr: Box::new(GrammarExpr::Terminal("a".into())),
        },
    );
    assert_rule_expr(&grammar, "epsilon", GrammarExpr::Empty);
}

#[test]
fn write_surface_round_trips_back_to_the_same_grammar() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .source_format(GrammarFormat::MetaLanguage)
        .start("list")
        .rule(
            "list",
            expr.seq([
                expr.nt("item"),
                expr.rep0(expr.seq([expr.term(","), expr.nt("item")])),
                expr.opt(expr.term(",")),
            ]),
        )
        .rule(
            "item",
            expr.choice_unordered([expr.term("a"), expr.term("b")]),
        )
        .rule("quote", expr.term("a\"b"))
        .rule("empty", expr.term(""))
        .build();

    let surface = write_grammar_surface(&grammar);
    let reparsed = parse_grammar_surface(&surface).expect("written surface reparses");

    assert_eq!(reparsed, grammar);
}

#[test]
fn grammar_lino_bridge_reuses_links_network_round_trip() {
    let grammar = parse_grammar_surface(
        r#"
(expr: term)
(term: "x")
"#,
    )
    .expect("surface parses");

    let lino = grammar_to_lino(&grammar);
    let restored = grammar_from_lino(&lino).expect("grammar decodes from LiNo");

    assert_eq!(restored, grammar);
}

#[test]
fn malformed_surface_reports_structured_errors_without_panicking() {
    let skeleton_error =
        parse_grammar_surface(r#"(expr: "unterminated)"#).expect_err("quote is unbalanced");
    assert!(matches!(
        skeleton_error,
        GrammarSurfaceError::Skeleton { .. }
    ));

    let lowering_error = parse_grammar_surface("(expr: *)").expect_err("operator is dangling");
    assert!(matches!(
        lowering_error,
        GrammarSurfaceError::Lowering {
            rule: Some(rule),
            ..
        } if rule == "expr"
    ));

    let undefined_error =
        parse_grammar_surface("(expr: missing)").expect_err("missing rule is undefined");
    assert!(matches!(
        undefined_error,
        GrammarSurfaceError::UndefinedReference { rule, name }
            if rule == "expr" && name == "missing"
    ));
}

fn assert_rule_expr(grammar: &Grammar, rule: &str, expected: GrammarExpr) {
    assert_eq!(
        grammar.rule(rule).map(|rule| rule.expr().clone()),
        Some(expected),
        "unexpected expression for {rule}"
    );
}
