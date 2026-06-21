use meta_language::{mdl, mdl_cost, minimize, Grammar, GrammarOracle, Mdl, MinimizeOptions};

#[test]
fn mdl_cost_splits_d1_total_and_prefers_compact_equivalent_grammar() {
    let examples = strings(["a", "b"]);
    let compact = compact_letters();
    let overfit = overfit_letters();
    let data = examples.iter().map(String::as_str).collect::<Vec<_>>();

    let compact_cost = mdl_cost(&compact, &examples);
    let overfit_cost = mdl_cost(&overfit, &examples);

    assert_close(compact_cost.total(), mdl(&compact, &data));
    assert_close(overfit_cost.total(), mdl(&overfit, &data));
    assert!(compact_cost.grammar_bits > 0.0);
    assert!(compact_cost.data_bits > 0.0);
    assert!(compact_cost.total() < overfit_cost.total());
}

#[test]
fn minimize_merges_identical_rules_and_then_reduces_redundant_indirection() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("s")
        .rule("s", expr.choice_unordered([expr.nt("a"), expr.nt("b")]))
        .rule("a", expr.term("x"))
        .rule("b", expr.term("x"))
        .build();
    let examples = strings(["x"]);

    let result = minimize(&grammar, &examples, MinimizeOptions::default());

    assert!(result.report.merges_applied >= 1, "{:#?}", result.report);
    assert!(result.after.total() < result.before.total(), "{result:#?}");
    assert!(result.grammar.rules().len() < grammar.rules().len());
    assert!(examples
        .iter()
        .all(|example| GrammarOracle::new(&result.grammar).accepts(example)));
}

#[test]
fn minimize_inlines_single_use_nonterminal() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("s")
        .rule("s", expr.seq([expr.nt("letter"), expr.term("!")]))
        .rule("letter", expr.term("a"))
        .build();
    let examples = strings(["a!"]);

    let result = minimize(&grammar, &examples, MinimizeOptions::default());

    assert!(result.report.inlines_applied >= 1, "{:#?}", result.report);
    assert_eq!(result.grammar.rules().len(), 1);
    assert!(GrammarOracle::new(&result.grammar).accepts("a!"));
}

#[test]
fn minimize_factors_common_choice_prefix() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("s")
        .rule(
            "s",
            expr.choice_unordered([
                expr.seq([expr.term("a"), expr.term("b")]),
                expr.seq([expr.term("a"), expr.term("c")]),
            ]),
        )
        .build();
    let examples = strings(["ab", "ac"]);

    let result = minimize(&grammar, &examples, MinimizeOptions::default());

    assert!(
        result.report.factorings_applied >= 1,
        "{:#?}",
        result.report
    );
    assert!(result.after.total() < result.before.total(), "{result:#?}");
    assert!(GrammarOracle::new(&result.grammar).accepts("ab"));
    assert!(GrammarOracle::new(&result.grammar).accepts("ac"));
}

#[test]
fn precision_gate_rejects_merge_that_widens_language() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("s")
        .rule(
            "s",
            expr.choice_unordered([
                expr.seq([expr.nt("left_a"), expr.nt("right_b")]),
                expr.seq([expr.nt("left_c"), expr.nt("right_d")]),
            ]),
        )
        .rule(
            "left_a",
            expr.seq([expr.term("a"), expr.term("x"), expr.term("x")]),
        )
        .rule("right_b", expr.term("b"))
        .rule(
            "left_c",
            expr.seq([expr.term("c"), expr.term("x"), expr.term("x")]),
        )
        .rule("right_d", expr.term("d"))
        .build();
    let examples = strings(["axxb", "cxxd"]);
    let options = MinimizeOptions {
        precision_budget: 0.0,
        sample_budget: 128,
        max_iterations: 16,
    };

    let result = minimize(&grammar, &examples, options);
    let oracle = GrammarOracle::new(&result.grammar);

    assert!(
        result.report.candidates_rejected_by_gate > 0,
        "{:#?}",
        result.report
    );
    assert!(examples.iter().all(|example| oracle.accepts(example)));
    assert!(!oracle.accepts("axxd"), "{:#?}", result.grammar);
    assert!(!oracle.accepts("cxxb"), "{:#?}", result.grammar);
}

#[test]
fn minimize_is_deterministic() {
    let grammar = overfit_letters();
    let examples = strings(["a", "b"]);

    let first = minimize(&grammar, &examples, MinimizeOptions::default());
    let second = minimize(&grammar, &examples, MinimizeOptions::default());

    assert_eq!(first, second);
    assert!(first.after.total() <= first.before.total());
}

#[test]
fn public_defaults_and_mdl_total_are_stable() {
    let options = MinimizeOptions::default();
    let mdl = Mdl {
        grammar_bits: 12.5,
        data_bits: 3.5,
    };

    assert_close(options.precision_budget, 0.0);
    assert_eq!(options.sample_budget, 256);
    assert_eq!(options.max_iterations, 64);
    assert_close(mdl.total(), 16.0);
}

fn compact_letters() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("s")
        .rule("s", expr.choice_unordered([expr.term("a"), expr.term("b")]))
        .build()
}

fn overfit_letters() -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start("s")
        .rule("s", expr.choice_unordered([expr.nt("a"), expr.nt("b")]))
        .rule("a", expr.term("a"))
        .rule("b", expr.term("b"))
        .build()
}

fn strings<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "expected {actual} to be close to {expected}"
    );
}
