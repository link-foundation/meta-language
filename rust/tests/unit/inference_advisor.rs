use meta_language::{
    AdviceSource, ConceptNamingAdvisor, FallbackAdvisor, Grammar, MdlMergeAdvisor, MergeAdvisor,
    MergeCandidate, MergeRequest, MergeScore, NameCandidate, NamingAdvisor, NamingRequest,
};

#[test]
fn concept_naming_advisor_prefers_number_concept_for_integer_yields() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("root")
        .rule("root", expr.nt("number"))
        .build();
    let rule_expr = expr.rep1(expr.char_range('0', '9'));
    let sample_yields = strings(["1", "23", "456"]);
    let request = NamingRequest {
        grammar: &grammar,
        rule_expr: &rule_expr,
        sample_yields: &sample_yields,
    };

    let candidates = ConceptNamingAdvisor.propose_names(&request);

    assert_eq!(
        candidates.first(),
        Some(&NameCandidate {
            name: "number".to_string(),
            concept: Some("grammar.number".to_string()),
            source: AdviceSource::Deterministic,
        })
    );
}

#[test]
fn concept_naming_advisor_falls_back_to_unique_structural_names() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("root")
        .rule("root", expr.nt("seq_2"))
        .rule("seq_2", expr.term("existing"))
        .build();
    let rule_expr = expr.seq([expr.term("a"), expr.term("b")]);
    let request = NamingRequest {
        grammar: &grammar,
        rule_expr: &rule_expr,
        sample_yields: &[],
    };

    let candidates = ConceptNamingAdvisor.propose_names(&request);

    assert_eq!(
        candidates.first(),
        Some(&NameCandidate {
            name: "seq_2_2".to_string(),
            concept: None,
            source: AdviceSource::Deterministic,
        })
    );
}

#[test]
fn mdl_merge_advisor_ranks_description_length_reducing_merge_above_neutral_merge() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("s")
        .rule("s", expr.choice_unordered([expr.nt("a"), expr.nt("b")]))
        .rule("a", expr.term("x"))
        .rule("b", expr.term("x"))
        .build();
    let examples = strings(["x"]);
    let candidates = [MergeCandidate::new("a", "b"), MergeCandidate::new("a", "a")];
    let request = MergeRequest {
        grammar: &grammar,
        candidates: &candidates,
        examples: &examples,
    };

    let scores = MdlMergeAdvisor.rank_merges(&request);

    assert_eq!(scores.len(), candidates.len());
    assert_eq!(scores[0].source, AdviceSource::Deterministic);
    assert_eq!(scores[1].source, AdviceSource::Deterministic);
    assert!(
        scores[0].score > scores[1].score,
        "expected reducing merge to outrank neutral merge: {scores:?}"
    );
    assert_close(scores[1].score, 0.5);
}

#[test]
fn fallback_advisor_rejects_invalid_accelerated_name_and_uses_deterministic_name() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("root")
        .rule("root", expr.empty())
        .build();
    let rule_expr = expr.rep1(expr.char_range('0', '9'));
    let sample_yields = strings(["7", "11"]);
    let request = NamingRequest {
        grammar: &grammar,
        rule_expr: &rule_expr,
        sample_yields: &sample_yields,
    };
    let advisor = FallbackAdvisor::new(Some(InvalidNamingAdvisor), ConceptNamingAdvisor);

    let candidates = advisor.propose_names(&request);

    assert_eq!(candidates[0].name, "number");
    assert_eq!(candidates[0].concept.as_deref(), Some("grammar.number"));
    assert_eq!(candidates[0].source, AdviceSource::Deterministic);
}

#[test]
fn fallback_advisor_uses_deterministic_merge_scores_when_accelerator_returns_wrong_count() {
    let expr = Grammar::expr();
    let grammar = Grammar::builder()
        .start("s")
        .rule("s", expr.choice_unordered([expr.nt("a"), expr.nt("b")]))
        .rule("a", expr.term("x"))
        .rule("b", expr.term("x"))
        .build();
    let examples = strings(["x"]);
    let candidates = [MergeCandidate::new("a", "b")];
    let request = MergeRequest {
        grammar: &grammar,
        candidates: &candidates,
        examples: &examples,
    };
    let advisor = FallbackAdvisor::new(Some(EmptyMergeAdvisor), MdlMergeAdvisor);

    let scores = advisor.rank_merges(&request);

    assert_eq!(scores.len(), candidates.len());
    assert_eq!(scores[0].source, AdviceSource::Deterministic);
}

struct InvalidNamingAdvisor;

impl NamingAdvisor for InvalidNamingAdvisor {
    fn propose_names(&self, _request: &NamingRequest<'_>) -> Vec<NameCandidate> {
        vec![NameCandidate {
            name: "not valid".to_string(),
            concept: Some("missing.concept".to_string()),
            source: AdviceSource::Llm,
        }]
    }
}

struct EmptyMergeAdvisor;

impl MergeAdvisor for EmptyMergeAdvisor {
    fn rank_merges(&self, _request: &MergeRequest<'_>) -> Vec<MergeScore> {
        Vec::new()
    }
}

#[cfg(feature = "llm-assist")]
mod llm_assist {
    use super::*;
    use meta_language::{LlmClient, LlmError, LlmMergeAdvisor, LlmNamingAdvisor};

    #[test]
    fn llm_naming_advisor_falls_back_when_client_errors() {
        let expr = Grammar::expr();
        let grammar = Grammar::builder()
            .start("root")
            .rule("root", expr.empty())
            .build();
        let rule_expr = expr.rep1(expr.char_range('0', '9'));
        let sample_yields = strings(["1", "2"]);
        let request = NamingRequest {
            grammar: &grammar,
            rule_expr: &rule_expr,
            sample_yields: &sample_yields,
        };

        let candidates = LlmNamingAdvisor::new(FailingClient).propose_names(&request);

        assert_eq!(candidates[0].name, "number");
        assert_eq!(candidates[0].source, AdviceSource::Deterministic);
    }

    #[test]
    fn llm_naming_advisor_accepts_valid_grounded_name() {
        let expr = Grammar::expr();
        let grammar = Grammar::builder()
            .start("root")
            .rule("root", expr.empty())
            .build();
        let rule_expr = expr.rep1(expr.char_range('0', '9'));
        let sample_yields = strings(["10", "20"]);
        let request = NamingRequest {
            grammar: &grammar,
            rule_expr: &rule_expr,
            sample_yields: &sample_yields,
        };

        let candidates =
            LlmNamingAdvisor::new(StaticClient("amount|grammar.number")).propose_names(&request);

        assert_eq!(candidates[0].name, "amount");
        assert_eq!(candidates[0].concept.as_deref(), Some("grammar.number"));
        assert_eq!(candidates[0].source, AdviceSource::Llm);
    }

    #[test]
    fn llm_naming_advisor_rejects_non_unique_or_unknown_concept_name() {
        let expr = Grammar::expr();
        let grammar = Grammar::builder()
            .start("root")
            .rule("root", expr.empty())
            .build();
        let rule_expr = expr.rep1(expr.char_range('0', '9'));
        let sample_yields = strings(["10", "20"]);
        let request = NamingRequest {
            grammar: &grammar,
            rule_expr: &rule_expr,
            sample_yields: &sample_yields,
        };

        let candidates =
            LlmNamingAdvisor::new(StaticClient("root|missing.concept")).propose_names(&request);

        assert_eq!(candidates[0].name, "number");
        assert_eq!(candidates[0].source, AdviceSource::Deterministic);
    }

    #[test]
    fn llm_merge_advisor_accepts_valid_ranked_scores() {
        let expr = Grammar::expr();
        let grammar = Grammar::builder()
            .start("s")
            .rule("s", expr.choice_unordered([expr.nt("a"), expr.nt("b")]))
            .rule("a", expr.term("x"))
            .rule("b", expr.term("x"))
            .build();
        let examples = strings(["x"]);
        let candidates = [MergeCandidate::new("a", "b")];
        let request = MergeRequest {
            grammar: &grammar,
            candidates: &candidates,
            examples: &examples,
        };

        let scores = LlmMergeAdvisor::new(StaticClient("0.9")).rank_merges(&request);

        assert_eq!(scores.len(), 1);
        assert_eq!(scores[0].source, AdviceSource::Llm);
        assert_close(scores[0].score, 0.9);
    }

    struct FailingClient;

    impl LlmClient for FailingClient {
        fn complete(&self, _prompt: &str) -> Result<String, LlmError> {
            Err(LlmError::new("offline"))
        }
    }

    struct StaticClient(&'static str);

    impl LlmClient for StaticClient {
        fn complete(&self, _prompt: &str) -> Result<String, LlmError> {
            Ok(self.0.to_string())
        }
    }
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
