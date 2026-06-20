use meta_language::{
    evaluate, mdl, run_named_corpus, sample, size_symbols, CharClassItem, EvalError, GoldenCorpus,
    Grammar, GrammarOracle, SampleConfig, ScoringMode, GOLDEN_CORPORA,
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

#[test]
fn public_constructors_defaults_and_error_messages_are_stable() {
    let grammar = one_literal_grammar("letter", "a");
    let oracle = GrammarOracle::new(&grammar);
    let default_config = SampleConfig::default();
    let custom_corpus =
        GoldenCorpus::new("inference-eval:custom", &["a"], custom_single_letter_corpus);

    assert!(oracle.accepts("a"));
    assert_eq!(default_config.count, 256);
    assert_eq!(custom_corpus.name(), "inference-eval:custom");
    assert_eq!(custom_corpus.positives(), &["a"]);
    assert!(GrammarOracle::new(&custom_corpus.golden_grammar()).accepts("a"));

    let messages = [
        (
            EvalError::EmptyGrammar,
            "grammar has no start rule".to_string(),
        ),
        (
            EvalError::EmptyCorpus,
            "corpus-mode recall requires positives".to_string(),
        ),
        (
            EvalError::EmptySample { source: "golden" },
            "golden sampling produced no text".to_string(),
        ),
        (
            EvalError::NonTerminating {
                rule: "expr".to_string(),
            },
            "rule `expr` cannot terminate".to_string(),
        ),
        (
            EvalError::UnknownRule {
                rule: "missing".to_string(),
            },
            "unknown grammar rule `missing`".to_string(),
        ),
        (
            EvalError::InvalidCharRange {
                start: 'z',
                end: 'a',
            },
            "invalid character range 'z'..='a'".to_string(),
        ),
        (
            EvalError::EmptyCharClass,
            "character class has no sampleable member".to_string(),
        ),
        (
            EvalError::InvalidRepeat { min: 3, max: 2 },
            "invalid repetition bounds 3..=2".to_string(),
        ),
        (
            EvalError::CorpusNotFound {
                corpus: "missing".to_string(),
            },
            "unknown corpus `missing`".to_string(),
        ),
    ];

    for (err, expected) in messages {
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn evaluation_reports_empty_and_missing_inputs() {
    let grammar = one_literal_grammar("letter", "a");
    let oracle = GrammarOracle::new(&grammar);

    assert_eq!(
        sample(&Grammar::new(), &metric_config()).expect_err("empty grammar"),
        EvalError::EmptyGrammar
    );
    assert_eq!(
        evaluate(&grammar, &oracle, None, &[], &metric_config()).expect_err("empty positives"),
        EvalError::EmptyCorpus
    );
    assert_eq!(
        evaluate(
            &grammar,
            &oracle,
            Some(&grammar),
            &[],
            &SampleConfig {
                count: 0,
                ..metric_config()
            },
        )
        .expect_err("empty inferred sample"),
        EvalError::EmptySample { source: "inferred" }
    );
    assert_eq!(
        run_named_corpus("inference-eval:missing", &grammar, &metric_config())
            .expect_err("missing corpus"),
        EvalError::CorpusNotFound {
            corpus: "inference-eval:missing".to_string()
        }
    );

    let missing_start = Grammar::builder()
        .start("missing")
        .rule("present", Grammar::expr().term("a"))
        .build();
    assert_eq!(
        sample(&missing_start, &metric_config()).expect_err("missing start rule"),
        EvalError::UnknownRule {
            rule: "missing".to_string()
        }
    );

    let missing_reference = Grammar::builder()
        .start("start")
        .rule("start", Grammar::expr().nt("missing"))
        .build();
    assert_eq!(
        sample(&missing_reference, &metric_config()).expect_err("missing referenced rule"),
        EvalError::UnknownRule {
            rule: "missing".to_string()
        }
    );
}

#[test]
fn sampler_exercises_expression_variants_and_depth_limited_shortest_paths() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("start")
        .rule(
            "start",
            expr.seq([
                expr.and(expr.term("H")),
                expr.not(expr.term("!")),
                expr.terminal_insensitive("HI"),
                expr.char_class(false, [CharClassItem::Range('0', '1')]),
                expr.any(),
                expr.opt(expr.term("?")),
                expr.rep0(expr.term("z")),
                expr.rep1(expr.term("q")),
                expr.repeat(expr.term("r"), 2, Some(3)),
                expr.capture_unlabeled(expr.term("c")),
                expr.nt("tail"),
            ]),
        )
        .rule(
            "tail",
            expr.choice_ordered([expr.term("t"), expr.term("long-tail")]),
        )
        .build();
    let config = SampleConfig {
        seed: 5,
        count: 8,
        max_depth: 0,
        repeat_cap: 3,
    };
    let samples = sample(&grammar, &config).expect("samples");
    let oracle = GrammarOracle::new(&grammar);

    assert!(!samples.is_empty());
    assert!(
        samples.iter().all(|text| oracle.accepts(text)),
        "{samples:?}"
    );
    assert!(samples.iter().all(|text| text.starts_with("HI")));
    assert!(samples.iter().all(|text| text.ends_with("ct")));
}

#[test]
fn sampler_reports_invalid_character_sources_before_expansion() {
    let expr = Grammar::expr();
    let invalid_range = Grammar::builder()
        .start("start")
        .rule("start", expr.rep0(expr.char_range('z', 'a')))
        .build();
    let empty_class = Grammar::builder()
        .start("start")
        .rule(
            "start",
            expr.rep0(expr.char_class(false, Vec::<CharClassItem>::new())),
        )
        .build();
    let negated_class = Grammar::builder()
        .start("start")
        .rule(
            "start",
            expr.char_class(true, [CharClassItem::Char('a'), CharClassItem::Char('b')]),
        )
        .build();
    let config = SampleConfig {
        seed: 1,
        count: 32,
        max_depth: 4,
        repeat_cap: 1,
    };

    assert_eq!(
        sample(&invalid_range, &config).expect_err("invalid range"),
        EvalError::NonTerminating {
            rule: "<repeat>".to_string()
        }
    );
    assert_eq!(
        sample(&empty_class, &config).expect_err("empty class"),
        EvalError::NonTerminating {
            rule: "<repeat>".to_string()
        }
    );
    assert_eq!(
        sample(&negated_class, &config).expect("negated class sample"),
        vec!["c".to_string()]
    );
}

#[test]
fn grammar_oracle_covers_choice_classes_repetition_and_failures() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("start")
        .rule(
            "start",
            expr.choice_ordered([
                expr.seq([
                    expr.terminal_insensitive("ab"),
                    expr.repeat(
                        expr.char_class(true, [CharClassItem::Char('!')]),
                        1,
                        Some(2),
                    ),
                ]),
                expr.term("fallback"),
            ]),
        )
        .build();
    let oracle = GrammarOracle::new(&grammar);

    assert!(oracle.accepts("ABa"));
    assert!(oracle.accepts("abxy"));
    assert!(oracle.accepts("fallback"));
    assert!(!oracle.accepts("AB!"));
    assert!(!oracle.accepts("abxyz"));
    assert!(!GrammarOracle::new(&Grammar::new()).accepts(""));

    let missing_reference = Grammar::builder()
        .start("start")
        .rule("start", expr.nt("missing"))
        .build();
    assert!(!GrammarOracle::new(&missing_reference).accepts(""));

    let recursive = Grammar::builder()
        .start("start")
        .rule("start", expr.seq([expr.nt("start"), expr.term("a")]))
        .build();
    assert!(!GrammarOracle::new(&recursive).accepts("a"));

    let invalid_repeat = Grammar::builder()
        .start("start")
        .rule("start", expr.repeat(expr.term("a"), 2, Some(1)))
        .build();
    assert!(!GrammarOracle::new(&invalid_repeat).accepts("a"));
}

#[test]
fn mdl_and_size_cover_all_expression_symbol_variants() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("start")
        .rule(
            "start",
            expr.choice_unordered([
                expr.empty(),
                expr.seq([
                    expr.terminal_insensitive("A"),
                    expr.char_class(
                        false,
                        [CharClassItem::Char('x'), CharClassItem::Range('0', '9')],
                    ),
                    expr.any(),
                    expr.opt(expr.term("?")),
                    expr.rep0(expr.term("z")),
                    expr.rep1(expr.term("q")),
                    expr.repeat(expr.term("r"), 1, Some(2)),
                    expr.and(expr.term("tail")),
                    expr.not(expr.term("stop")),
                    expr.capture(Some("label"), expr.nt("tail")),
                ]),
            ]),
        )
        .rule("tail", expr.term("tail"))
        .build();
    let symbols = size_symbols(&grammar);
    let bits = mdl(&grammar, &["", "not accepted"]);

    assert!(symbols > 20, "{symbols}");
    let symbols_as_f64 = f64::from(u32::try_from(symbols).expect("test grammar is small"));
    assert!(bits > symbols_as_f64, "{bits}");
}

#[test]
fn corpus_mode_report_shape_can_be_built_from_evaluate_inputs() {
    let inferred = one_literal_grammar("letter", "a");
    let golden = two_literal_grammar("gold", ["a", "b"]);
    let scores = evaluate(
        &inferred,
        &GrammarOracle::new(&golden),
        None,
        &["a", "b"],
        &metric_config(),
    )
    .expect("scores");

    let mode = ScoringMode::Corpus;
    assert!(matches!(mode, ScoringMode::Corpus));
    assert_close(scores.precision, 1.0);
    assert_close(scores.recall, 0.5);
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

fn custom_single_letter_corpus() -> Grammar {
    one_literal_grammar("letter", "a")
}
