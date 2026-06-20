use super::{
    validate_name_candidate, AdviceSource, ConceptNamingAdvisor, MdlMergeAdvisor, MergeAdvisor,
    MergeRequest, MergeScore, NameCandidate, NamingAdvisor, NamingRequest,
    INFERENCE_NAMING_CONCEPTS,
};

/// Provider-agnostic LLM boundary for optional inference acceleration.
pub trait LlmClient: Send + Sync {
    /// Completes one prompt.
    fn complete(&self, prompt: &str) -> Result<String, LlmError>;
}

/// Error returned by an optional LLM client.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LlmError {
    message: String,
}

impl LlmError {
    /// Builds an LLM client error.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Human-readable error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for LlmError {}

/// Optional LLM-backed naming advisor with deterministic fallback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LlmNamingAdvisor<C> {
    client: C,
    fallback: ConceptNamingAdvisor,
}

impl<C> LlmNamingAdvisor<C> {
    /// Builds an LLM naming advisor with the default deterministic fallback.
    #[must_use]
    pub const fn new(client: C) -> Self {
        Self {
            client,
            fallback: ConceptNamingAdvisor,
        }
    }

    /// Builds an LLM naming advisor with an explicit deterministic fallback.
    #[must_use]
    pub const fn with_fallback(client: C, fallback: ConceptNamingAdvisor) -> Self {
        Self { client, fallback }
    }
}

impl<C> NamingAdvisor for LlmNamingAdvisor<C>
where
    C: LlmClient,
{
    fn propose_names(&self, request: &NamingRequest<'_>) -> Vec<NameCandidate> {
        let deterministic = self.fallback.propose_names(request);
        let Ok(response) = self.client.complete(&naming_prompt(request)) else {
            return deterministic;
        };

        let candidates = parse_name_candidates(&response)
            .into_iter()
            .filter(|candidate| validate_name_candidate(request, candidate))
            .collect::<Vec<_>>();

        if candidates.is_empty() {
            deterministic
        } else {
            candidates
        }
    }
}

/// Optional LLM-backed merge advisor with deterministic fallback.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LlmMergeAdvisor<C> {
    client: C,
    fallback: MdlMergeAdvisor,
}

impl<C> LlmMergeAdvisor<C> {
    /// Builds an LLM merge advisor with the default deterministic fallback.
    #[must_use]
    pub const fn new(client: C) -> Self {
        Self {
            client,
            fallback: MdlMergeAdvisor,
        }
    }

    /// Builds an LLM merge advisor with an explicit deterministic fallback.
    #[must_use]
    pub const fn with_fallback(client: C, fallback: MdlMergeAdvisor) -> Self {
        Self { client, fallback }
    }
}

impl<C> MergeAdvisor for LlmMergeAdvisor<C>
where
    C: LlmClient,
{
    fn rank_merges(&self, request: &MergeRequest<'_>) -> Vec<MergeScore> {
        let deterministic = self.fallback.rank_merges(request);
        let Ok(response) = self.client.complete(&merge_prompt(request)) else {
            return deterministic;
        };
        let Some(parsed) = parse_merge_scores(&response, request.candidates.len()) else {
            return deterministic;
        };

        parsed
            .into_iter()
            .zip(deterministic)
            .map(|(score, deterministic)| {
                let mut score = score.clamp(0.0, 1.0);
                if deterministic.score < 0.5 {
                    score = score.min(deterministic.score);
                }
                MergeScore {
                    score,
                    source: AdviceSource::Llm,
                }
            })
            .collect()
    }
}

fn naming_prompt(request: &NamingRequest<'_>) -> String {
    let concepts = INFERENCE_NAMING_CONCEPTS
        .iter()
        .map(|concept| format!("{}={}", concept.id, concept.name))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "Suggest one grammar rule name as name|concept.\nExpression: {}\nSamples: {:?}\nConcepts: {concepts}",
        request.rule_expr, request.sample_yields
    )
}

fn merge_prompt(request: &MergeRequest<'_>) -> String {
    let candidates = request
        .candidates
        .iter()
        .map(|candidate| format!("{}<-{}", candidate.winner, candidate.loser))
        .collect::<Vec<_>>()
        .join(", ");
    format!("Score merge candidates in order with numbers from 0 to 1: {candidates}")
}

fn parse_name_candidates(response: &str) -> Vec<NameCandidate> {
    response.lines().filter_map(parse_name_candidate).collect()
}

fn parse_name_candidate(line: &str) -> Option<NameCandidate> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let (name, concept) = line
        .split_once('|')
        .map_or((line, None), |(name, concept)| {
            let concept = concept.trim();
            let concept = if concept.is_empty() || concept.eq_ignore_ascii_case("none") {
                None
            } else {
                Some(concept.to_string())
            };
            (name, concept)
        });
    let name = name.trim().trim_matches(['"', '\'', '`']);
    (!name.is_empty()).then(|| NameCandidate {
        name: name.to_string(),
        concept,
        source: AdviceSource::Llm,
    })
}

fn parse_merge_scores(response: &str, expected_len: usize) -> Option<Vec<f64>> {
    let scores = response
        .split(|character: char| character.is_ascii_whitespace() || matches!(character, ',' | ';'))
        .filter_map(|token| token.trim().parse::<f64>().ok())
        .collect::<Vec<_>>();

    (scores.len() == expected_len && scores.iter().all(|score| score.is_finite())).then_some(scores)
}
