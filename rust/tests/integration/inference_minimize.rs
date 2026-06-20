use meta_language::{
    evaluate, infer_cfg, mdl_cost, minimize, Grammar, GrammarExpr, GrammarOracle, InferenceOptions,
    MinimizeOptions, PositiveOnlyOracle, SampleConfig,
};

#[test]
fn cfg_inference_output_can_be_minimized_without_losing_d1_scores() {
    let examples = strings(["[]", "[a]", "[a,b]", "[b,a]"]);
    let inferred = infer_cfg(&examples, &PositiveOnlyOracle, InferenceOptions::default());
    let before = mdl_cost(&inferred.grammar, &examples);

    let result = minimize(&inferred.grammar, &examples, MinimizeOptions::default());
    let oracle = GrammarOracle::new(&result.grammar);

    assert!(result.grammar.rules().len() < inferred.grammar.rules().len());
    assert!(result.after.total() <= before.total(), "{result:#?}");
    assert!(examples.iter().all(|example| oracle.accepts(example)));
    assert!(
        result
            .grammar
            .rules()
            .iter()
            .any(|rule| references_rule(rule.expr(), rule.name())),
        "{:#?}",
        result.grammar
    );

    let golden = list_grammar();
    let config = SampleConfig {
        seed: 17,
        count: 64,
        max_depth: 8,
        repeat_cap: 3,
    };
    let before_scores = evaluate(
        &inferred.grammar,
        &GrammarOracle::new(&golden),
        Some(&golden),
        &[],
        &config,
    )
    .expect("before scores");
    let after_scores = evaluate(
        &result.grammar,
        &GrammarOracle::new(&golden),
        Some(&golden),
        &[],
        &config,
    )
    .expect("after scores");

    assert!(
        after_scores.recall >= before_scores.recall,
        "{after_scores:?}"
    );
    assert!(
        after_scores.f1 >= before_scores.f1,
        "before: {before_scores:?}; after: {after_scores:?}"
    );
}

fn list_grammar() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("list")
        .rule(
            "list",
            expr.seq([expr.term("["), expr.opt(expr.nt("items")), expr.term("]")]),
        )
        .rule(
            "items",
            expr.choice_unordered([
                expr.nt("item"),
                expr.seq([expr.nt("item"), expr.term(","), expr.nt("items")]),
            ]),
        )
        .rule(
            "item",
            expr.choice_unordered([expr.term("a"), expr.term("b")]),
        )
        .build()
}

fn strings<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn references_rule(expr: &GrammarExpr, expected: &str) -> bool {
    match expr {
        GrammarExpr::NonTerminal(name) => name == expected,
        GrammarExpr::Choice { alternatives, .. } => alternatives
            .iter()
            .any(|alternative| references_rule(alternative, expected)),
        GrammarExpr::Sequence(items) => items.iter().any(|item| references_rule(item, expected)),
        GrammarExpr::Optional(inner)
        | GrammarExpr::ZeroOrMore(inner)
        | GrammarExpr::OneOrMore(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Repeat { expr: inner, .. }
        | GrammarExpr::Capture { expr: inner, .. } => references_rule(inner, expected),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => false,
    }
}
