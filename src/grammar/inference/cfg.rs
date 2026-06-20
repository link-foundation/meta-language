//! Deterministic positive-only CFG inference over delimiter seed trees.
//!
//! This module is the D5 black-box CFG inference entry point. It consumes the
//! D6 delimiter structural prior, emits the shared grammar IR, and uses the D1
//! grammar oracle/sampler for acceptance checks.

use std::collections::{BTreeMap, BTreeSet};

pub use super::eval::MembershipOracle;
use super::eval::{sample, GrammarOracle, SampleConfig};
use super::prior::{build_structural_prior, ByteSpan, Delimiter, LeafKind, PriorOptions, SeedNode};
use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule};

const ROOT_RULE: &str = "Root";
const DEFAULT_MAX_ITERATIONS: usize = 64;
const DEFAULT_SAMPLE_BUDGET: usize = 256;

/// Options for [`infer_cfg`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InferenceOptions {
    /// Kedavra-style segmentation toggle. The current implementation keeps the
    /// same public behavior while reserving the option for segmented inference.
    pub incremental: bool,
    /// Defensive cap for iterative inference phases.
    pub max_iterations: usize,
    /// Number of candidate strings sampled when a membership oracle is present.
    pub sample_budget: usize,
}

impl Default for InferenceOptions {
    fn default() -> Self {
        Self {
            incremental: false,
            max_iterations: DEFAULT_MAX_ITERATIONS,
            sample_budget: DEFAULT_SAMPLE_BUDGET,
        }
    }
}

/// Counts and acceptance decisions recorded during inference.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InferenceReport {
    /// Number of rules in the emitted grammar.
    pub rules: usize,
    /// Number of delimiter/group bubbles proposed from the structural prior.
    pub bubbles_proposed: usize,
    /// Number of deterministic alternative de-duplications or generalisations accepted.
    pub merges_accepted: usize,
    /// Number of candidate generalisations rejected by the oracle layer.
    pub merges_rejected: usize,
}

/// Inferred grammar plus a compact report for evaluation and benchmarking.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceResult {
    /// Emitted grammar IR.
    pub grammar: Grammar,
    /// Deterministic inference report.
    pub report: InferenceReport,
}

/// Decides whether an inferred generalisation is acceptable.
pub trait Oracle {
    /// Returns `true` when `grammar` accepts every positive example.
    fn accepts_all_positive(&self, grammar: &Grammar, examples: &[String]) -> bool {
        let grammar_oracle = GrammarOracle::new(grammar);
        examples
            .iter()
            .all(|example| grammar_oracle.accepts(example))
    }

    /// Optional black-box membership oracle for rejecting over-generalisation.
    fn membership(&self) -> Option<&dyn MembershipOracle> {
        None
    }
}

/// Positive-only oracle backed by the in-repository grammar recogniser.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PositiveOnlyOracle;

impl PositiveOnlyOracle {
    /// Builds a positive-only oracle.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Oracle for PositiveOnlyOracle {}

/// Infers a deterministic CFG from positive examples.
#[must_use]
pub fn infer_cfg(
    examples: &[String],
    oracle: &dyn Oracle,
    opts: InferenceOptions,
) -> InferenceResult {
    let positives = sorted_unique_examples(examples);
    let mut report = InferenceReport::default();

    if positives.is_empty() {
        let grammar = Grammar::new().with_source_format(GrammarFormat::Inferred);
        return InferenceResult { grammar, report };
    }

    let mut candidate = structured_grammar(&positives, opts, &mut report);
    if !oracle.accepts_all_positive(&candidate, examples)
        || membership_rejects_candidate(&candidate, oracle, opts)
    {
        report.merges_rejected = report.merges_rejected.saturating_add(1);
        candidate = exact_positive_grammar(&positives);
    }

    report.rules = candidate.rules().len();
    InferenceResult {
        grammar: candidate,
        report,
    }
}

fn sorted_unique_examples(examples: &[String]) -> Vec<String> {
    examples
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn structured_grammar(
    examples: &[String],
    opts: InferenceOptions,
    report: &mut InferenceReport,
) -> Grammar {
    let prior = build_structural_prior(examples, PriorOptions::default());
    let mut draft = Draft::default();
    let mut root_alternatives = Vec::new();

    for tree in &prior.trees {
        root_alternatives.push(draft.expr_for_node(&tree.root, &tree.example));
    }

    report.bubbles_proposed = report
        .bubbles_proposed
        .saturating_add(draft.bubbles_proposed);

    let before_root = root_alternatives.len();
    let root_expr = choice_expr(root_alternatives);
    report.merges_accepted = report
        .merges_accepted
        .saturating_add(before_root.saturating_sub(choice_len(&root_expr)));

    let mut grammar = Grammar::new().with_source_format(GrammarFormat::Inferred);
    grammar.add_rule(GrammarRule::new(ROOT_RULE, root_expr));

    for delimiter in [Delimiter::Paren, Delimiter::Curly, Delimiter::Square] {
        let Some(alternatives) = draft.group_alternatives.remove(&delimiter) else {
            continue;
        };
        let before = alternatives.len();
        let rules = rules_for_group(delimiter, alternatives);
        let after = rules.first().map_or(0, |rule| choice_len(rule.expr()));
        report.merges_accepted = report
            .merges_accepted
            .saturating_add(before.saturating_sub(after));
        for rule in rules {
            grammar.add_rule(rule);
        }
    }

    if opts.incremental {
        report.bubbles_proposed = report.bubbles_proposed.saturating_add(examples.len());
    }

    grammar.set_start(ROOT_RULE);
    grammar
}

#[derive(Default)]
struct Draft {
    group_alternatives: BTreeMap<Delimiter, Vec<Vec<GrammarExpr>>>,
    bubbles_proposed: usize,
}

impl Draft {
    fn expr_for_node(&mut self, node: &SeedNode, example: &str) -> GrammarExpr {
        match node {
            SeedNode::Leaf { span, kind } => terminal_for_leaf(example, *span, *kind),
            SeedNode::Group {
                delimiter,
                children,
                span,
            } if *delimiter == Delimiter::Root => {
                seq_expr(self.sequence_for_children(*delimiter, children, *span, example))
            }
            SeedNode::Group {
                delimiter,
                children,
                span,
            } => {
                let inner = self.sequence_for_children(*delimiter, children, *span, example);
                self.group_alternatives
                    .entry(*delimiter)
                    .or_default()
                    .push(inner);
                self.bubbles_proposed = self.bubbles_proposed.saturating_add(1);
                GrammarExpr::non_terminal(group_rule_name(*delimiter))
            }
        }
    }

    fn sequence_for_children(
        &mut self,
        delimiter: Delimiter,
        children: &[SeedNode],
        span: ByteSpan,
        example: &str,
    ) -> Vec<GrammarExpr> {
        let (content_start, content_end) = content_span(delimiter, span);
        let mut cursor = content_start;
        let mut sequence = Vec::new();

        for child in children {
            let child_span = node_span(child);
            push_gap(example, cursor, child_span.start, &mut sequence);
            sequence.push(self.expr_for_node(child, example));
            cursor = child_span.end;
        }

        push_gap(example, cursor, content_end, &mut sequence);
        sequence
    }
}

const fn content_span(delimiter: Delimiter, span: ByteSpan) -> (usize, usize) {
    match delimiter {
        Delimiter::Root => (span.start, span.end),
        Delimiter::Paren | Delimiter::Curly | Delimiter::Square => {
            (span.start.saturating_add(1), span.end.saturating_sub(1))
        }
    }
}

const fn node_span(node: &SeedNode) -> ByteSpan {
    match node {
        SeedNode::Leaf { span, .. } | SeedNode::Group { span, .. } => *span,
    }
}

fn push_gap(example: &str, start: usize, end: usize, sequence: &mut Vec<GrammarExpr>) {
    if start < end {
        sequence.push(GrammarExpr::terminal(&example[start..end]));
    }
}

fn terminal_for_leaf(example: &str, span: ByteSpan, kind: LeafKind) -> GrammarExpr {
    let value = &example[span.start..span.end];
    match kind {
        LeafKind::Backtick => GrammarExpr::terminal_insensitive(value),
        LeafKind::Text | LeafKind::SingleQuote | LeafKind::DoubleQuote => {
            GrammarExpr::terminal(value)
        }
    }
}

fn rules_for_group(delimiter: Delimiter, alternatives: Vec<Vec<GrammarExpr>>) -> Vec<GrammarRule> {
    let name = group_rule_name(delimiter);
    if let Some(list) = infer_comma_list_group(delimiter, &name, &alternatives) {
        return list;
    }

    let inner = choice_expr(alternatives.into_iter().map(seq_expr).collect());
    vec![GrammarRule::new(name, wrap_delimited(delimiter, inner))]
}

fn infer_comma_list_group(
    delimiter: Delimiter,
    name: &str,
    alternatives: &[Vec<GrammarExpr>],
) -> Option<Vec<GrammarRule>> {
    let mut saw_empty = false;
    let mut saw_separator = false;
    let mut item_alternatives = Vec::new();
    let mut separator_alternatives = Vec::new();

    for alternative in alternatives {
        if alternative.is_empty() {
            saw_empty = true;
            continue;
        }

        let parts = split_comma_list(alternative)?;
        saw_separator |= !parts.separators.is_empty();
        item_alternatives.extend(parts.items);
        separator_alternatives.extend(parts.separators);
    }

    if !saw_separator || item_alternatives.is_empty() {
        return None;
    }

    let item_name = format!("{name}_item");
    let items_name = format!("{name}_items");
    let separator = choice_expr(separator_alternatives);
    let content = if saw_empty {
        GrammarExpr::optional(GrammarExpr::non_terminal(&items_name))
    } else {
        GrammarExpr::non_terminal(&items_name)
    };

    let group_rule = GrammarRule::new(name, wrap_delimited(delimiter, content));
    let item_rule = GrammarRule::new(item_name.clone(), choice_expr(item_alternatives));
    let items_rule = GrammarRule::new(
        items_name.clone(),
        GrammarExpr::choice(
            false,
            [
                GrammarExpr::non_terminal(&item_name),
                GrammarExpr::sequence([
                    GrammarExpr::non_terminal(&item_name),
                    separator,
                    GrammarExpr::non_terminal(&items_name),
                ]),
            ],
        ),
    );

    Some(vec![group_rule, item_rule, items_rule])
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ListParts {
    items: Vec<GrammarExpr>,
    separators: Vec<GrammarExpr>,
}

fn split_comma_list(sequence: &[GrammarExpr]) -> Option<ListParts> {
    let mut cursor = 0usize;
    let mut items = Vec::new();
    let mut separators = Vec::new();

    while let Some(comma) = find_comma(sequence, cursor) {
        let item_start = trim_start(sequence, cursor, comma);
        let item_end = trim_end(sequence, item_start, comma);
        if item_start == item_end {
            return None;
        }
        items.push(seq_expr(sequence[item_start..item_end].to_vec()));

        let separator_start = item_end;
        let separator_end = trim_start(sequence, comma.saturating_add(1), sequence.len());
        separators.push(seq_expr(sequence[separator_start..separator_end].to_vec()));
        cursor = separator_end;
    }

    let item_start = trim_start(sequence, cursor, sequence.len());
    let item_end = trim_end(sequence, item_start, sequence.len());
    if item_start == item_end {
        return None;
    }
    items.push(seq_expr(sequence[item_start..item_end].to_vec()));

    Some(ListParts { items, separators })
}

fn find_comma(sequence: &[GrammarExpr], start: usize) -> Option<usize> {
    sequence
        .iter()
        .enumerate()
        .skip(start)
        .find_map(|(index, expr)| is_comma(expr).then_some(index))
}

fn trim_start(sequence: &[GrammarExpr], mut start: usize, end: usize) -> usize {
    while start < end && is_whitespace(&sequence[start]) {
        start += 1;
    }
    start
}

fn trim_end(sequence: &[GrammarExpr], start: usize, mut end: usize) -> usize {
    while start < end && is_whitespace(&sequence[end - 1]) {
        end -= 1;
    }
    end
}

fn is_comma(expr: &GrammarExpr) -> bool {
    matches!(expr, GrammarExpr::Terminal(value) if value == ",")
}

fn is_whitespace(expr: &GrammarExpr) -> bool {
    matches!(expr, GrammarExpr::Terminal(value) if !value.is_empty() && value.chars().all(char::is_whitespace))
}

fn wrap_delimited(delimiter: Delimiter, inner: GrammarExpr) -> GrammarExpr {
    let (open, close) = delimiter_tokens(delimiter);
    let mut items = vec![GrammarExpr::terminal(open)];
    if inner != GrammarExpr::Empty {
        items.push(inner);
    }
    items.push(GrammarExpr::terminal(close));
    GrammarExpr::sequence(items)
}

const fn delimiter_tokens(delimiter: Delimiter) -> (&'static str, &'static str) {
    match delimiter {
        Delimiter::Paren => ("(", ")"),
        Delimiter::Curly => ("{", "}"),
        Delimiter::Square => ("[", "]"),
        Delimiter::Root => ("", ""),
    }
}

fn group_rule_name(delimiter: Delimiter) -> String {
    match delimiter {
        Delimiter::Paren => "n0",
        Delimiter::Curly => "n1",
        Delimiter::Square => "n2",
        Delimiter::Root => ROOT_RULE,
    }
    .to_string()
}

fn choice_expr(alternatives: Vec<GrammarExpr>) -> GrammarExpr {
    let mut unique = BTreeMap::<String, GrammarExpr>::new();
    for alternative in alternatives {
        unique.entry(expr_key(&alternative)).or_insert(alternative);
    }

    match unique.len() {
        0 => GrammarExpr::Empty,
        1 => unique
            .into_values()
            .next()
            .expect("one alternative must be present"),
        _ => GrammarExpr::choice(false, unique.into_values()),
    }
}

fn seq_expr(items: Vec<GrammarExpr>) -> GrammarExpr {
    match items.len() {
        0 => GrammarExpr::Empty,
        1 => items
            .into_iter()
            .next()
            .expect("one sequence item must be present"),
        _ => GrammarExpr::sequence(items),
    }
}

fn choice_len(expr: &GrammarExpr) -> usize {
    match expr {
        GrammarExpr::Choice { alternatives, .. } => alternatives.len(),
        GrammarExpr::Empty => 0,
        _ => 1,
    }
}

fn expr_key(expr: &GrammarExpr) -> String {
    format!("{expr:?}")
}

fn exact_positive_grammar(examples: &[String]) -> Grammar {
    let alternatives = examples.iter().map(|example| {
        if example.is_empty() {
            GrammarExpr::Empty
        } else {
            GrammarExpr::terminal(example)
        }
    });

    Grammar::new()
        .with_source_format(GrammarFormat::Inferred)
        .with_rule(GrammarRule::new(
            ROOT_RULE,
            choice_expr(alternatives.collect()),
        ))
        .with_start(ROOT_RULE)
}

fn membership_rejects_candidate(
    candidate: &Grammar,
    oracle: &dyn Oracle,
    opts: InferenceOptions,
) -> bool {
    let Some(membership) = oracle.membership() else {
        return false;
    };

    let config = SampleConfig {
        count: opts.sample_budget.max(1),
        max_depth: opts.max_iterations.max(1),
        ..SampleConfig::default()
    };

    sample(candidate, &config)
        .is_ok_and(|samples| samples.iter().any(|sample| !membership.accepts(sample)))
}
