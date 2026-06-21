use meta_language::{
    infer_cfg, infer_cfg_with_advisors, AdviceDecisionKind, AdviceSource, InferenceOptions,
    MergeAdvisor, MergeRequest, MergeScore, NameCandidate, NamingAdvisor, NamingRequest,
    PositiveOnlyOracle,
};

#[test]
fn cfg_inference_records_deterministic_advice_sources() {
    let examples = strings(["[]", "[a]", "[a,b]"]);

    let first = infer_cfg(&examples, &PositiveOnlyOracle, InferenceOptions::default());
    let second = infer_cfg(&examples, &PositiveOnlyOracle, InferenceOptions::default());

    assert_eq!(first, second);
    assert!(
        !first.report.advice.is_empty(),
        "expected inference report to record advisor provenance"
    );
    assert!(first
        .report
        .advice
        .iter()
        .all(|decision| decision.source == AdviceSource::Deterministic));
}

#[test]
fn cfg_inference_records_custom_advisor_sources() {
    let examples = strings(["[]", "[a]", "[a,a]"]);

    let deterministic = infer_cfg(&examples, &PositiveOnlyOracle, InferenceOptions::default());
    let advised = infer_cfg_with_advisors(
        &examples,
        &PositiveOnlyOracle,
        InferenceOptions::default(),
        &LlmSourceNamingAdvisor,
        &LlmSourceMergeAdvisor,
    );

    assert_eq!(advised.grammar, deterministic.grammar);
    assert!(advised.report.advice.iter().any(|decision| {
        decision.kind == AdviceDecisionKind::Naming && decision.source == AdviceSource::Llm
    }));
    assert!(advised.report.advice.iter().any(|decision| {
        decision.kind == AdviceDecisionKind::Merge && decision.source == AdviceSource::Llm
    }));
}

#[cfg(feature = "llm-assist")]
#[test]
fn cfg_inference_with_failing_llm_client_matches_deterministic_result() {
    use meta_language::{
        ConceptNamingAdvisor, FallbackAdvisor, LlmMergeAdvisor, LlmNamingAdvisor, MdlMergeAdvisor,
    };

    let examples = strings(["[]", "[a]", "[a,a]"]);
    let deterministic = infer_cfg(&examples, &PositiveOnlyOracle, InferenceOptions::default());
    let naming = FallbackAdvisor::new(
        Some(LlmNamingAdvisor::new(FailingClient)),
        ConceptNamingAdvisor,
    );
    let merge = FallbackAdvisor::new(Some(LlmMergeAdvisor::new(FailingClient)), MdlMergeAdvisor);

    let assisted = infer_cfg_with_advisors(
        &examples,
        &PositiveOnlyOracle,
        InferenceOptions::default(),
        &naming,
        &merge,
    );

    assert_eq!(assisted, deterministic);
}

#[cfg(feature = "llm-assist")]
struct FailingClient;

#[cfg(feature = "llm-assist")]
impl meta_language::LlmClient for FailingClient {
    fn complete(&self, _prompt: &str) -> Result<String, meta_language::LlmError> {
        Err(meta_language::LlmError::new("offline"))
    }
}

struct LlmSourceNamingAdvisor;

impl NamingAdvisor for LlmSourceNamingAdvisor {
    fn propose_names(&self, _request: &NamingRequest<'_>) -> Vec<NameCandidate> {
        vec![NameCandidate {
            name: "llm_name".to_string(),
            concept: None,
            source: AdviceSource::Llm,
        }]
    }
}

struct LlmSourceMergeAdvisor;

impl MergeAdvisor for LlmSourceMergeAdvisor {
    fn rank_merges(&self, request: &MergeRequest<'_>) -> Vec<MergeScore> {
        request
            .candidates
            .iter()
            .map(|_| MergeScore {
                score: 0.75,
                source: AdviceSource::Llm,
            })
            .collect()
    }
}

fn strings<const N: usize>(values: [&str; N]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
