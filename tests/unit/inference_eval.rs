use meta_language::{
    evaluate, mdl, run_named_corpus, sample, size_symbols, EvalError, Grammar, GrammarOracle,
    SampleConfig, GOLDEN_CORPORA,
};

#[test]
fn metrics_identify_equivalent_over_general_over_specific_and_disjoint_grammars() {
    let gold = two_literal_grammar("gold", ["a", "b"]);
    let over_general = three_literal_grammar("over_general", ["a", "b", "c"]);
    let over_specific = one_literal_grammar("over_specific", "a");
    let disjoint = one_literal_grammar("disjoint", "c");
    let config = metric_config();
    let oracle = GrammarOracle(&gold);

    let identical = evaluate(&gold, &oracle, Some(&gold), &[], &config).expect("scores");
    assert_close(identical.precision, 1.0);
    assert_close(identical.recall, 1.0);
    assert_close(identical.f1, 1.0);

    let general = evaluate(&over_general, &oracle, Some(&gold), &[], &config).expect("scores");
    assert!(general.precision < 1.0, "{general:?}");
    assert_close(general.recall, 1.0);
    assert!(general.f1 < 1.0);

    let specific = evaluate(&over_specific, &oracle, Some(&gold), &[], &config).expect("scores");
    assert_close(specific.precision, 1.0);
    assert!(specific.recall < 1.0, "{specific:?}");
    assert!(specific.f1 < 1.0);

    let disjoint_scores = evaluate(&disjoint, &oracle, Some(&gold), &[], &config).expect("scores");
    assert_close(disjoint_scores.precision, 0.0);
    assert_close(disjoint_scores.recall, 0.0);
    assert_close(disjoint_scores.f1, 0.0);
}

#[test]
fn corpus_mode_uses_positive_examples_for_recall() {
    let inferred = one_literal_grammar("letter", "a");
    let gold = two_literal_grammar("gold", ["a", "b"]);
    let oracle = GrammarOracle(&gold);

    let scores = evaluate(
        &inferred,
        &oracle,
        None,
        &["a", "b"],
        &SampleConfig {
            seed: 1,
            count: 8,
            max_depth: 4,
            repeat_cap: 2,
        },
    )
    .expect("scores");

    assert_close(scores.precision, 1.0);
    assert_close(scores.recall, 0.5);
    assert!(scores.f1 > 0.0 && scores.f1 < 1.0);
}

#[test]
fn sampler_is_seeded_deterministic_and_uses_distinct_draw_order() {
    let grammar = three_literal_grammar("letters", ["a", "b", "c"]);
    let first = sample(
        &grammar,
        &SampleConfig {
            seed: 42,
            count: 16,
            max_depth: 4,
            repeat_cap: 2,
        },
    )
    .expect("samples");
    let second = sample(
        &grammar,
        &SampleConfig {
            seed: 42,
            count: 16,
            max_depth: 4,
            repeat_cap: 2,
        },
    )
    .expect("samples");
    let different_seed = sample(
        &grammar,
        &SampleConfig {
            seed: 99,
            count: 16,
            max_depth: 4,
            repeat_cap: 2,
        },
    )
    .expect("samples");

    assert_eq!(first, second);
    assert_ne!(first, different_seed);
    assert!(first
        .iter()
        .all(|sample| ["a", "b", "c"].contains(&sample.as_str())));
}

#[test]
fn sampler_reports_reachable_nonterminating_left_recursion() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("s")
        .rule("s", expr.seq([expr.nt("s"), expr.term("a")]))
        .build();

    let err = sample(&grammar, &metric_config()).expect_err("left recursion is rejected");
    assert_eq!(
        err,
        EvalError::NonTerminating {
            rule: "s".to_string()
        }
    );
}

#[test]
fn grammar_oracle_handles_literals_ranges_repetition_and_lookahead() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("ident")
        .rule(
            "ident",
            expr.seq([
                expr.and(expr.char_range('a', 'z')),
                expr.char_range('a', 'z'),
                expr.rep0(
                    expr.choice_unordered([expr.char_range('a', 'z'), expr.char_range('0', '9')]),
                ),
                expr.not(expr.term("!")),
            ]),
        )
        .build();
    let oracle = GrammarOracle(&grammar);

    assert!(oracle.accepts("a"));
    assert!(oracle.accepts("abc123"));
    assert!(!oracle.accepts("1abc"));
    assert!(!oracle.accepts("abc!"));
}

#[test]
fn mdl_and_size_prefer_smaller_equivalent_grammar() {
    let compact = one_literal_grammar("compact", "a");
    let redundant = two_literal_grammar("redundant", ["a", "a"]);

    assert!(size_symbols(&compact) < size_symbols(&redundant));
    assert!(mdl(&compact, &["a"]) < mdl(&redundant, &["a"]));
}

#[test]
fn golden_corpus_registry_runs_named_smoke_corpora() {
    assert!(GOLDEN_CORPORA.len() >= 2);

    for corpus in GOLDEN_CORPORA {
        let inferred = corpus.golden_grammar();
        let report = run_named_corpus(corpus.name(), &inferred, &metric_config()).expect("report");

        assert_eq!(report.corpus, corpus.name());
        assert_eq!(report.seed, metric_config().seed);
        assert!(report.samples_drawn > 0);
        assert_close(report.scores.f1, 1.0);
    }
}

const fn metric_config() -> SampleConfig {
    SampleConfig {
        seed: 7,
        count: 64,
        max_depth: 6,
        repeat_cap: 4,
    }
}

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= f64::EPSILON,
        "expected {expected}, got {actual}"
    );
}

fn one_literal_grammar(name: &str, terminal: &str) -> Grammar {
    Grammar::builder()
        .start(name)
        .rule(name, Grammar::expr().term(terminal))
        .build()
}

fn two_literal_grammar(name: &str, terminals: [&str; 2]) -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start(name)
        .rule(
            name,
            expr.choice_unordered([expr.term(terminals[0]), expr.term(terminals[1])]),
        )
        .build()
}

fn three_literal_grammar(name: &str, terminals: [&str; 3]) -> Grammar {
    let expr = Grammar::expr();
    Grammar::builder()
        .start(name)
        .rule(
            name,
            expr.choice_unordered([
                expr.term(terminals[0]),
                expr.term(terminals[1]),
                expr.term(terminals[2]),
            ]),
        )
        .build()
}
