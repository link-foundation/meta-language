//! Advisory naming and merge ranking for grammar inference.
//!
//! Advisors are intentionally narrow: deterministic inference can use the
//! default implementations without a model or network, while optional
//! accelerators can suggest names or merge ordering behind the same traits.

use crate::grammar::{
    grammar_expr_concept_id, CharClassItem, Grammar, GrammarExpr, GrammarRule, RuleKind,
    GRAMMAR_CONCEPTS,
};

use super::minimize::mdl_cost;

const COST_EPSILON: f64 = 1e-9;

const INFERENCE_NAMING_CONCEPTS: &[InferenceNamingConcept] = &[
    InferenceNamingConcept {
        id: "grammar.number",
        name: "number",
    },
    InferenceNamingConcept {
        id: "grammar.digit",
        name: "digit",
    },
    InferenceNamingConcept {
        id: "grammar.letter",
        name: "letter",
    },
    InferenceNamingConcept {
        id: "grammar.identifier",
        name: "identifier",
    },
    InferenceNamingConcept {
        id: "grammar.string",
        name: "string",
    },
    InferenceNamingConcept {
        id: "grammar.boolean",
        name: "boolean",
    },
    InferenceNamingConcept {
        id: "grammar.null",
        name: "null",
    },
    InferenceNamingConcept {
        id: "grammar.value",
        name: "value",
    },
    InferenceNamingConcept {
        id: "grammar.item",
        name: "item",
    },
    InferenceNamingConcept {
        id: "grammar.list",
        name: "list",
    },
    InferenceNamingConcept {
        id: "grammar.name",
        name: "name",
    },
    InferenceNamingConcept {
        id: "grammar.object",
        name: "object",
    },
    InferenceNamingConcept {
        id: "grammar.member",
        name: "member",
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct InferenceNamingConcept {
    id: &'static str,
    name: &'static str,
}

/// Source of an advisory inference decision.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdviceSource {
    /// Deterministic local heuristic with no model or network.
    Deterministic,
    /// Optional LLM-backed accelerator.
    Llm,
}

/// Kind of inference decision recorded for evaluation reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdviceDecisionKind {
    /// A non-terminal naming decision.
    Naming,
    /// A rule-merge ranking or selection decision.
    Merge,
}

/// Provenance record for an advisory inference decision.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdviceDecision {
    /// Decision kind.
    pub kind: AdviceDecisionKind,
    /// Rule or candidate target associated with the decision.
    pub target: String,
    /// Advisor source used for this decision.
    pub source: AdviceSource,
}

impl AdviceDecision {
    /// Builds an advice provenance record.
    #[must_use]
    pub fn new(kind: AdviceDecisionKind, target: impl Into<String>, source: AdviceSource) -> Self {
        Self {
            kind,
            target: target.into(),
            source,
        }
    }
}

/// Request for naming an inferred non-terminal.
#[derive(Clone, Copy, Debug)]
pub struct NamingRequest<'a> {
    /// Grammar context used to avoid name collisions.
    pub grammar: &'a Grammar,
    /// Right-hand-side expression to name.
    pub rule_expr: &'a GrammarExpr,
    /// Example substrings this expression derives.
    pub sample_yields: &'a [String],
}

/// Candidate non-terminal name proposed by a naming advisor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NameCandidate {
    /// Proposed rule name.
    pub name: String,
    /// Grounding concept id, when the name is concept-backed.
    pub concept: Option<String>,
    /// Source that produced this suggestion.
    pub source: AdviceSource,
}

/// Request for ranking candidate rule merges.
#[derive(Clone, Copy, Debug)]
pub struct MergeRequest<'a> {
    /// Grammar to score candidate merges against.
    pub grammar: &'a Grammar,
    /// Candidate rule pairs. The winner name survives; the loser is rewritten.
    pub candidates: &'a [MergeCandidate],
    /// Positive examples used for the data component of the MDL score.
    ///
    /// Passing an empty slice makes the score grammar-size-only.
    pub examples: &'a [String],
}

/// Candidate merge of two named rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MergeCandidate {
    /// Rule name to keep.
    pub winner: String,
    /// Rule name to rewrite and remove.
    pub loser: String,
}

impl MergeCandidate {
    /// Builds a merge candidate from winner and loser rule names.
    #[must_use]
    pub fn new(winner: impl Into<String>, loser: impl Into<String>) -> Self {
        Self {
            winner: winner.into(),
            loser: loser.into(),
        }
    }
}

/// Score for one merge candidate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MergeScore {
    /// Score in `[0.0, 1.0]`; higher means more promising.
    pub score: f64,
    /// Source that produced the score.
    pub source: AdviceSource,
}

/// Proposes human-meaningful names for inferred non-terminals.
pub trait NamingAdvisor {
    /// Returns ranked candidate names, best first.
    fn propose_names(&self, request: &NamingRequest<'_>) -> Vec<NameCandidate>;
}

/// Ranks candidate rule merges during inference and minimization.
pub trait MergeAdvisor {
    /// Returns one score per candidate in request order.
    fn rank_merges(&self, request: &MergeRequest<'_>) -> Vec<MergeScore>;
}

/// Deterministic naming advisor grounded in grammar concepts and stable shapes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConceptNamingAdvisor;

impl NamingAdvisor for ConceptNamingAdvisor {
    fn propose_names(&self, request: &NamingRequest<'_>) -> Vec<NameCandidate> {
        let (base_name, concept) = concept_name_for_request(request).map_or_else(
            || (structural_name(request.rule_expr), None),
            |concept| (concept.name.to_string(), Some(concept.id.to_string())),
        );
        let name = unique_rule_name(&sanitize_identifier(&base_name), request.grammar);

        vec![NameCandidate {
            name,
            concept,
            source: AdviceSource::Deterministic,
        }]
    }
}

/// Deterministic merge advisor using the existing MDL objective.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MdlMergeAdvisor;

impl MergeAdvisor for MdlMergeAdvisor {
    fn rank_merges(&self, request: &MergeRequest<'_>) -> Vec<MergeScore> {
        let baseline = mdl_cost(request.grammar, request.examples).total();

        request
            .candidates
            .iter()
            .map(|candidate| {
                let score =
                    merge_candidate_grammar(request.grammar, candidate).map_or(0.0, |trial| {
                        let delta = mdl_cost(&trial, request.examples).total() - baseline;
                        score_from_delta(delta)
                    });
                MergeScore {
                    score,
                    source: AdviceSource::Deterministic,
                }
            })
            .collect()
    }
}

/// Wraps an optional accelerator with a deterministic fallback advisor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FallbackAdvisor<A, D> {
    accelerator: Option<A>,
    deterministic: D,
}

impl<A, D> FallbackAdvisor<A, D> {
    /// Builds a fallback advisor.
    #[must_use]
    pub const fn new(accelerator: Option<A>, deterministic: D) -> Self {
        Self {
            accelerator,
            deterministic,
        }
    }

    /// Builds a deterministic-only fallback advisor.
    #[must_use]
    pub const fn deterministic(deterministic: D) -> Self {
        Self {
            accelerator: None,
            deterministic,
        }
    }
}

impl<A, D> NamingAdvisor for FallbackAdvisor<A, D>
where
    A: NamingAdvisor,
    D: NamingAdvisor,
{
    fn propose_names(&self, request: &NamingRequest<'_>) -> Vec<NameCandidate> {
        let deterministic = self.deterministic.propose_names(request);
        let Some(accelerator) = &self.accelerator else {
            return deterministic;
        };

        let accelerated = accelerator
            .propose_names(request)
            .into_iter()
            .filter(|candidate| validate_name_candidate(request, candidate))
            .collect::<Vec<_>>();

        if accelerated.is_empty() {
            deterministic
        } else {
            accelerated
        }
    }
}

impl<A, D> MergeAdvisor for FallbackAdvisor<A, D>
where
    A: MergeAdvisor,
    D: MergeAdvisor,
{
    fn rank_merges(&self, request: &MergeRequest<'_>) -> Vec<MergeScore> {
        let deterministic = self.deterministic.rank_merges(request);
        let Some(accelerator) = &self.accelerator else {
            return deterministic;
        };

        validate_merge_scores(accelerator.rank_merges(request), request.candidates.len())
            .unwrap_or(deterministic)
    }
}

#[cfg(feature = "llm-assist")]
mod llm;
#[cfg(feature = "llm-assist")]
pub use llm::{LlmClient, LlmError, LlmMergeAdvisor, LlmNamingAdvisor};

fn concept_name_for_request(request: &NamingRequest<'_>) -> Option<InferenceNamingConcept> {
    concept_from_samples(request.sample_yields).or_else(|| concept_from_expr(request.rule_expr))
}

fn concept_from_samples(samples: &[String]) -> Option<InferenceNamingConcept> {
    if samples.is_empty() || samples.iter().any(String::is_empty) {
        return None;
    }

    if samples.iter().all(|sample| is_integer_text(sample)) {
        return inference_concept("grammar.number");
    }
    if samples.iter().all(|sample| is_digit_text(sample)) {
        return inference_concept("grammar.digit");
    }
    if samples.iter().all(|sample| is_identifier_text(sample)) {
        return inference_concept("grammar.identifier");
    }
    if samples.iter().all(|sample| is_letter_text(sample)) {
        return inference_concept("grammar.letter");
    }

    None
}

fn concept_from_expr(expr: &GrammarExpr) -> Option<InferenceNamingConcept> {
    if is_number_expr(expr) {
        return inference_concept("grammar.number");
    }
    if is_digit_expr(expr) {
        return inference_concept("grammar.digit");
    }
    if is_identifier_expr(expr) {
        return inference_concept("grammar.identifier");
    }
    if is_letter_expr(expr) {
        return inference_concept("grammar.letter");
    }

    None
}

fn inference_concept(id: &str) -> Option<InferenceNamingConcept> {
    INFERENCE_NAMING_CONCEPTS
        .iter()
        .copied()
        .find(|concept| concept.id == id)
}

fn structural_name(expr: &GrammarExpr) -> String {
    match expr {
        GrammarExpr::Empty => "empty".to_string(),
        GrammarExpr::Terminal(_) | GrammarExpr::TerminalInsensitive(_) => "literal".to_string(),
        GrammarExpr::CharRange(_, _) => "char_range".to_string(),
        GrammarExpr::CharClass { .. } => "char_class".to_string(),
        GrammarExpr::AnyChar => "any_char".to_string(),
        GrammarExpr::NonTerminal(name) => sanitize_identifier(name),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => {
            let prefix = if *ordered { "ordered_choice" } else { "choice" };
            format!("{prefix}_{}", alternatives.len())
        }
        GrammarExpr::Sequence(items) => format!("seq_{}", items.len()),
        GrammarExpr::Optional(inner) => format!("{}_opt", structural_stem(inner)),
        GrammarExpr::ZeroOrMore(inner) => format!("{}_star", structural_stem(inner)),
        GrammarExpr::OneOrMore(inner) => format!("{}_plus", structural_stem(inner)),
        GrammarExpr::Repeat { expr, .. } => format!("{}_repeat", structural_stem(expr)),
        GrammarExpr::And(inner) => format!("{}_and", structural_stem(inner)),
        GrammarExpr::Not(inner) => format!("{}_not", structural_stem(inner)),
        GrammarExpr::Capture { label, expr } => label.as_deref().map_or_else(
            || format!("{}_capture", structural_stem(expr)),
            sanitize_identifier,
        ),
    }
}

fn structural_stem(expr: &GrammarExpr) -> String {
    concept_from_expr(expr).map_or_else(
        || match expr {
            GrammarExpr::NonTerminal(name) => sanitize_identifier(name),
            _ => sanitize_identifier(
                grammar_expr_concept_id(expr)
                    .rsplit('.')
                    .next()
                    .unwrap_or("rule"),
            ),
        },
        |concept| concept.name.to_string(),
    )
}

fn is_number_expr(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::OneOrMore(inner) => is_digit_expr(inner),
        GrammarExpr::Repeat { expr, min, max } => *min >= 1 && max.is_none() && is_digit_expr(expr),
        GrammarExpr::Sequence(items) if items.len() == 2 => {
            is_optional_sign(&items[0]) && is_number_expr(&items[1])
        }
        _ => false,
    }
}

fn is_optional_sign(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::Optional(inner) => matches!(
            inner.as_ref(),
            GrammarExpr::Terminal(value) if value == "-" || value == "+"
        ),
        _ => false,
    }
}

fn is_digit_expr(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::CharRange('0', '9') => true,
        GrammarExpr::Terminal(value) => is_digit_text(value),
        GrammarExpr::CharClass { negated, items } => {
            !*negated
                && items.iter().all(|item| {
                    matches!(
                        item,
                        CharClassItem::Range('0', '9') | CharClassItem::Char('0'..='9')
                    )
                })
        }
        _ => false,
    }
}

fn is_letter_expr(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::CharRange('a', 'z') | GrammarExpr::CharRange('A', 'Z') => true,
        GrammarExpr::Terminal(value) => is_letter_text(value),
        GrammarExpr::CharClass { negated, items } => {
            !*negated
                && items.iter().all(|item| {
                    matches!(
                        item,
                        CharClassItem::Range('a', 'z')
                            | CharClassItem::Range('A', 'Z')
                            | CharClassItem::Char('a'..='z' | 'A'..='Z' | '_')
                    )
                })
        }
        _ => false,
    }
}

fn is_identifier_expr(expr: &GrammarExpr) -> bool {
    match expr {
        GrammarExpr::Sequence(items) if items.len() == 2 => {
            is_letter_expr(&items[0])
                && matches!(
                    &items[1],
                    GrammarExpr::ZeroOrMore(inner) if is_identifier_tail_expr(inner)
                )
        }
        _ => false,
    }
}

fn is_identifier_tail_expr(expr: &GrammarExpr) -> bool {
    is_letter_expr(expr)
        || is_digit_expr(expr)
        || matches!(
            expr,
            GrammarExpr::Choice { alternatives, .. }
                if alternatives
                    .iter()
                    .all(|alternative| is_letter_expr(alternative) || is_digit_expr(alternative))
        )
}

fn is_integer_text(text: &str) -> bool {
    let digits = text
        .strip_prefix(['-', '+'])
        .filter(|rest| !rest.is_empty())
        .unwrap_or(text);
    digits.chars().all(|character| character.is_ascii_digit())
}

fn is_digit_text(text: &str) -> bool {
    let mut chars = text.chars();
    chars
        .next()
        .is_some_and(|character| character.is_ascii_digit())
        && chars.next().is_none()
}

fn is_letter_text(text: &str) -> bool {
    let mut chars = text.chars();
    chars
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic())
        && chars.next().is_none()
}

fn is_identifier_text(text: &str) -> bool {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn sanitize_identifier(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        let normalized = if character.is_ascii_alphanumeric() || character == '_' {
            character.to_ascii_lowercase()
        } else {
            '_'
        };

        if output.is_empty() && normalized.is_ascii_digit() {
            output.push('_');
        }
        if normalized == '_' && output.ends_with('_') {
            continue;
        }
        output.push(normalized);
    }

    let trimmed = output.trim_matches('_');
    if trimmed.is_empty() {
        "rule".to_string()
    } else if trimmed
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_digit())
    {
        format!("_{trimmed}")
    } else {
        trimmed.to_string()
    }
}

fn unique_rule_name(base: &str, grammar: &Grammar) -> String {
    if grammar.rule(base).is_none() {
        return base.to_string();
    }

    let mut suffix = 2usize;
    loop {
        let candidate = format!("{base}_{suffix}");
        if grammar.rule(&candidate).is_none() {
            return candidate;
        }
        suffix = suffix.saturating_add(1);
    }
}

fn validate_name_candidate(request: &NamingRequest<'_>, candidate: &NameCandidate) -> bool {
    is_valid_identifier(&candidate.name)
        && request.grammar.rule(&candidate.name).is_none()
        && candidate.concept.as_deref().map_or(true, known_concept_id)
}

fn is_valid_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn known_concept_id(concept: &str) -> bool {
    GRAMMAR_CONCEPTS.iter().any(|known| known.id == concept)
        || INFERENCE_NAMING_CONCEPTS
            .iter()
            .any(|known| known.id == concept)
}

fn validate_merge_scores(scores: Vec<MergeScore>, expected_len: usize) -> Option<Vec<MergeScore>> {
    if scores.len() != expected_len || scores.iter().any(|score| !score.score.is_finite()) {
        return None;
    }

    Some(
        scores
            .into_iter()
            .map(|score| MergeScore {
                score: score.score.clamp(0.0, 1.0),
                source: score.source,
            })
            .collect(),
    )
}

fn merge_candidate_grammar(grammar: &Grammar, candidate: &MergeCandidate) -> Option<Grammar> {
    if candidate.winner == candidate.loser {
        return Some(grammar.clone());
    }

    let winner_rule = grammar.rule(&candidate.winner)?;
    let loser_rule = grammar.rule(&candidate.loser)?;
    if grammar.start() == Some(loser_rule.name())
        || !merge_metadata_compatible(winner_rule, loser_rule)
    {
        return None;
    }

    let winner_name = winner_rule.name().to_string();
    let loser_name = loser_rule.name().to_string();
    let replacement = GrammarExpr::non_terminal(&winner_name);
    let merged_expr = merge_exprs(winner_rule.expr(), loser_rule.expr());

    let mut next = Grammar::new();
    if let Some(source_format) = grammar.source_format() {
        next.set_source_format(source_format);
    }

    for rule in grammar.rules() {
        if rule.name() == loser_name {
            continue;
        }

        let mut next_rule = rule.clone();
        if next_rule.name() == winner_name {
            next_rule.expr = merged_expr.clone();
        }
        next_rule.expr = rewrite_nonterminal_refs(&next_rule.expr, &loser_name, &replacement);
        next.add_rule(next_rule);
    }

    if let Some(start) = grammar.start() {
        next.set_start(start);
    }
    Some(next)
}

fn merge_metadata_compatible(winner: &GrammarRule, loser: &GrammarRule) -> bool {
    winner.kind() == loser.kind()
        && winner.concept() == loser.concept()
        && winner.doc() == loser.doc()
        && winner.kind() == RuleKind::Normal
}

fn merge_exprs(winner: &GrammarExpr, loser: &GrammarExpr) -> GrammarExpr {
    if winner == loser {
        return winner.clone();
    }

    let mut alternatives = Vec::new();
    push_merge_alternative(&mut alternatives, winner);
    push_merge_alternative(&mut alternatives, loser);

    if alternatives.len() == 1 {
        alternatives.remove(0)
    } else {
        GrammarExpr::choice(false, alternatives)
    }
}

fn push_merge_alternative(alternatives: &mut Vec<GrammarExpr>, expr: &GrammarExpr) {
    if let GrammarExpr::Choice {
        ordered: false,
        alternatives: nested,
    } = expr
    {
        for alternative in nested {
            push_merge_alternative(alternatives, alternative);
        }
        return;
    }

    if !alternatives.contains(expr) {
        alternatives.push(expr.clone());
    }
}

fn rewrite_nonterminal_refs(
    expr: &GrammarExpr,
    loser_name: &str,
    replacement: &GrammarExpr,
) -> GrammarExpr {
    match expr {
        GrammarExpr::NonTerminal(name) if name == loser_name => replacement.clone(),
        GrammarExpr::Choice {
            ordered,
            alternatives,
        } => GrammarExpr::choice(
            *ordered,
            alternatives
                .iter()
                .map(|expr| rewrite_nonterminal_refs(expr, loser_name, replacement)),
        ),
        GrammarExpr::Sequence(items) => GrammarExpr::sequence(
            items
                .iter()
                .map(|expr| rewrite_nonterminal_refs(expr, loser_name, replacement)),
        ),
        GrammarExpr::Optional(inner) => {
            GrammarExpr::optional(rewrite_nonterminal_refs(inner, loser_name, replacement))
        }
        GrammarExpr::ZeroOrMore(inner) => {
            GrammarExpr::zero_or_more(rewrite_nonterminal_refs(inner, loser_name, replacement))
        }
        GrammarExpr::OneOrMore(inner) => {
            GrammarExpr::one_or_more(rewrite_nonterminal_refs(inner, loser_name, replacement))
        }
        GrammarExpr::Repeat { expr, min, max } => GrammarExpr::repeat(
            rewrite_nonterminal_refs(expr, loser_name, replacement),
            *min,
            *max,
        ),
        GrammarExpr::And(inner) => {
            GrammarExpr::and(rewrite_nonterminal_refs(inner, loser_name, replacement))
        }
        GrammarExpr::Not(inner) => {
            GrammarExpr::not(rewrite_nonterminal_refs(inner, loser_name, replacement))
        }
        GrammarExpr::Capture { label, expr } => label.as_deref().map_or_else(
            || {
                GrammarExpr::capture_unlabeled(rewrite_nonterminal_refs(
                    expr,
                    loser_name,
                    replacement,
                ))
            },
            |label| {
                GrammarExpr::capture(
                    label,
                    rewrite_nonterminal_refs(expr, loser_name, replacement),
                )
            },
        ),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => expr.clone(),
    }
}

fn score_from_delta(delta: f64) -> f64 {
    if !delta.is_finite() {
        return 0.0;
    }

    let magnitude = delta.abs() / (1.0 + delta.abs());
    if delta < -COST_EPSILON {
        0.5 + magnitude * 0.5
    } else if delta > COST_EPSILON {
        0.5 - magnitude * 0.5
    } else {
        0.5
    }
}
