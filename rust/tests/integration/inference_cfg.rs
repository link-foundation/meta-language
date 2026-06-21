use meta_language::{
    evaluate, infer_cfg, Grammar, GrammarOracle, InferenceOptions, PositiveOnlyOracle, SampleConfig,
};

#[test]
fn cfg_inference_reports_d1_metrics_for_small_list_corpus() {
    let examples = ["[]", "[a]", "[a,b]", "[b,a]"]
        .iter()
        .map(|value| (*value).to_string())
        .collect::<Vec<_>>();
    let result = infer_cfg(&examples, &PositiveOnlyOracle, InferenceOptions::default());
    let golden = list_grammar();
    let scores = evaluate(
        &result.grammar,
        &GrammarOracle::new(&golden),
        Some(&golden),
        &[],
        &SampleConfig {
            seed: 9,
            count: 32,
            max_depth: 6,
            repeat_cap: 3,
        },
    )
    .expect("D1 scores are computed");

    assert!(scores.recall >= 0.95, "{scores:?}\n{:#?}", result.grammar);
    assert!(
        scores.precision >= 0.95,
        "{scores:?}\n{:#?}",
        result.grammar
    );
    assert!(scores.f1 >= 0.95, "{scores:?}\n{:#?}", result.grammar);
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
