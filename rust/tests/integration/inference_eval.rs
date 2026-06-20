use meta_language::{run_corpus, Grammar, SampleConfig, GOLDEN_CORPORA};

#[test]
fn golden_corpus_runner_returns_populated_benchmark_reports() {
    let config = SampleConfig {
        seed: 11,
        count: 32,
        max_depth: 6,
        repeat_cap: 3,
    };

    for corpus in GOLDEN_CORPORA {
        let expr = Grammar::expr();
        let inferred = Grammar::builder()
            .start("text")
            .rule("text", expr.rep1(expr.any()))
            .build();
        let report = run_corpus(corpus, &inferred, &config).expect("benchmark report");

        assert_eq!(report.corpus, corpus.name());
        assert_eq!(report.seed, config.seed);
        assert!(report.samples_drawn > 0);
        assert!(report.scores.precision <= 1.0);
        assert!(report.scores.recall <= 1.0);
        assert!(report.scores.f1 <= 1.0);
        assert!(report.scores.size_symbols > 0);
        assert!(report.scores.mdl_bits > 0.0);
    }
}
