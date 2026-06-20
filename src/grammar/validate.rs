//! Semantic validation for authored grammar IR values.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::ops::Range;

use super::{Grammar, GrammarExpr, GrammarRule};

/// Where a grammar diagnostic applies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleSpan {
    /// Rule receiving the diagnostic.
    pub rule: String,
    /// Byte range in grammar surface source when available.
    pub span: Option<Range<usize>>,
}

/// Diagnostic severity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// The grammar is semantically invalid or unsafe to execute.
    Error,
    /// The grammar is accepted, but the finding is likely an authoring mistake.
    Warning,
}

/// Classes of authoring defects detected by [`validate`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticKind {
    /// A non-terminal reference whose name no rule defines.
    UndefinedNonTerminal {
        /// Referenced rule name that is missing.
        name: String,
        /// Rule containing the missing reference.
        referenced_in: String,
    },
    /// A rule can reach itself before consuming input.
    LeftRecursion {
        /// Rule chain proving the recursion, ending with the first rule again.
        cycle: Vec<String>,
    },
    /// A rule is not reachable from the grammar start symbol.
    UnreachableRule {
        /// Unreachable rule name.
        name: String,
    },
    /// A nullable repetition or suspicious nullable rule body was found.
    NullableRepetition {
        /// Rule containing the nullable construct.
        rule: String,
        /// Human-readable detail about the nullable construct.
        detail: String,
    },
    /// More than one rule uses the same name.
    DuplicateRule {
        /// Duplicated rule name.
        name: String,
    },
    /// A labelled capture has no semantic consumer in the grammar IR.
    UnusedCapture {
        /// Rule containing the labelled capture.
        rule: String,
        /// Capture label.
        label: String,
    },
}

/// One grammar validation finding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarDiagnostic {
    /// Structured class of the finding.
    pub kind: DiagnosticKind,
    /// Whether callers should treat the finding as blocking.
    pub severity: Severity,
    /// Friendly, fix-suggesting diagnostic text.
    pub message: String,
    /// Source rule location for the finding.
    pub location: RuleSpan,
}

impl GrammarDiagnostic {
    /// Returns true when this diagnostic should fail validation.
    #[must_use]
    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }
}

impl fmt::Display for GrammarDiagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for GrammarDiagnostic {}

impl Grammar {
    /// Runs semantic grammar validation.
    #[must_use]
    pub fn validate(&self) -> Vec<GrammarDiagnostic> {
        validate(self)
    }
}

/// Runs every grammar validation checker and returns deterministic diagnostics.
#[must_use]
pub fn validate(grammar: &Grammar) -> Vec<GrammarDiagnostic> {
    let defined_names = defined_rule_names(grammar);
    let nullability = compute_nullability(grammar);

    let mut diagnostics = Vec::new();
    diagnostics.extend(check_duplicate_rules(grammar));
    diagnostics.extend(check_undefined_nonterminals(grammar, &defined_names));
    diagnostics.extend(check_left_recursion(grammar, &nullability));
    diagnostics.extend(check_unreachable_rules(grammar));
    diagnostics.extend(check_nullable_repetition(grammar, &nullability));
    diagnostics.extend(check_unused_captures(grammar));

    sort_diagnostics(grammar, &mut diagnostics);
    diagnostics
}

fn check_duplicate_rules(grammar: &Grammar) -> Vec<GrammarDiagnostic> {
    let mut counts = BTreeMap::<String, usize>::new();
    for rule in grammar.rules() {
        *counts.entry(rule.name.clone()).or_default() += 1;
    }

    counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(name, _)| {
            let message = format!(
                "rule `{name}` is defined more than once; merge alternatives into one rule or rename the duplicate."
            );
            GrammarDiagnostic {
                kind: DiagnosticKind::DuplicateRule { name: name.clone() },
                severity: Severity::Error,
                message,
                location: rule_location(name),
            }
        })
        .collect()
}

fn check_undefined_nonterminals(
    grammar: &Grammar,
    defined_names: &[String],
) -> Vec<GrammarDiagnostic> {
    let defined = defined_names
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut diagnostics = Vec::new();

    for rule in grammar.rules() {
        for name in collect_nonterminals(rule.expr()) {
            if defined.contains(name.as_str()) {
                continue;
            }

            let suggestion = nearest_rule_name(&name, defined_names);
            let message = suggestion.map_or_else(
                || {
                    format!(
                        "rule `{}` references undefined non-terminal `{name}`; define it or fix the spelling.",
                        rule.name()
                    )
                },
                |candidate| {
                    format!(
                        "rule `{}` references undefined non-terminal `{name}`; did you mean `{candidate}`? Define it or fix the spelling.",
                        rule.name()
                    )
                },
            );
            diagnostics.push(GrammarDiagnostic {
                kind: DiagnosticKind::UndefinedNonTerminal {
                    name,
                    referenced_in: rule.name.clone(),
                },
                severity: Severity::Error,
                message,
                location: rule_location(rule.name()),
            });
        }
    }

    diagnostics
}

fn check_left_recursion(
    grammar: &Grammar,
    nullability: &BTreeMap<String, bool>,
) -> Vec<GrammarDiagnostic> {
    let graph = left_reference_graph(grammar, nullability);
    let mut diagnostics = Vec::new();
    let mut seen_cycles = BTreeSet::new();

    for rule in grammar.rules() {
        let mut path = vec![rule.name.clone()];
        let mut seen_rules = BTreeSet::from([rule.name.clone()]);
        collect_left_cycles(
            rule.name(),
            rule.name(),
            &graph,
            &mut seen_rules,
            &mut path,
            &mut seen_cycles,
            &mut diagnostics,
        );
    }

    diagnostics
}

fn collect_left_cycles(
    target: &str,
    current: &str,
    graph: &BTreeMap<String, BTreeSet<String>>,
    seen_rules: &mut BTreeSet<String>,
    path: &mut Vec<String>,
    seen_cycles: &mut BTreeSet<String>,
    diagnostics: &mut Vec<GrammarDiagnostic>,
) {
    let Some(next_rules) = graph.get(current) else {
        return;
    };

    for next in next_rules {
        if next == target {
            let mut cycle = path.clone();
            cycle.push(target.to_string());
            if seen_cycles.insert(canonical_cycle_key(&cycle)) {
                let cycle_text = cycle.join(" -> ");
                diagnostics.push(GrammarDiagnostic {
                    kind: DiagnosticKind::LeftRecursion {
                        cycle: cycle.clone(),
                    },
                    severity: Severity::Error,
                    message: format!(
                        "rule `{target}` is left-recursive (`{cycle_text}`); a recursive-descent/PEG parser may not terminate. Rewrite using repetition or factor the common prefix."
                    ),
                    location: rule_location(target),
                });
            }
        } else if graph.contains_key(next) && seen_rules.insert(next.clone()) {
            path.push(next.clone());
            collect_left_cycles(
                target,
                next,
                graph,
                seen_rules,
                path,
                seen_cycles,
                diagnostics,
            );
            path.pop();
            seen_rules.remove(next);
        }
    }
}

fn check_unreachable_rules(grammar: &Grammar) -> Vec<GrammarDiagnostic> {
    let Some(start_rule) = grammar.start_rule() else {
        return Vec::new();
    };

    let defined = grammar
        .rules()
        .iter()
        .map(GrammarRule::name)
        .collect::<BTreeSet<_>>();
    let mut reachable = BTreeSet::new();
    let mut stack = vec![start_rule.name.clone()];

    while let Some(name) = stack.pop() {
        if !reachable.insert(name.clone()) {
            continue;
        }

        if let Some(rule) = grammar.rule(&name) {
            for reference in collect_nonterminals(rule.expr()) {
                if defined.contains(reference.as_str()) {
                    stack.push(reference);
                }
            }
        }
    }

    grammar
        .rules()
        .iter()
        .filter(|rule| !reachable.contains(rule.name()))
        .map(|rule| GrammarDiagnostic {
            kind: DiagnosticKind::UnreachableRule {
                name: rule.name.clone(),
            },
            severity: Severity::Warning,
            message: format!(
                "rule `{}` is not reachable from start rule `{}`; remove it or reference it from a reachable rule.",
                rule.name(),
                start_rule.name()
            ),
            location: rule_location(rule.name()),
        })
        .collect()
}

fn check_nullable_repetition(
    grammar: &Grammar,
    nullability: &BTreeMap<String, bool>,
) -> Vec<GrammarDiagnostic> {
    let suspicious_nullable = compute_suspicious_nullability(grammar, nullability);
    let mut diagnostics = Vec::new();

    for rule in grammar.rules() {
        if suspicious_nullable
            .get(rule.name())
            .copied()
            .unwrap_or(false)
        {
            diagnostics.push(GrammarDiagnostic {
                kind: DiagnosticKind::NullableRepetition {
                    rule: rule.name.clone(),
                    detail: "rule body is nullable".to_string(),
                },
                severity: Severity::Warning,
                message: format!(
                    "rule `{}` can match empty; add a required terminal/non-terminal or document the rule as intentional.",
                    rule.name()
                ),
                location: rule_location(rule.name()),
            });
        }
        collect_nullable_repetitions(rule.name(), rule.expr(), nullability, &mut diagnostics);
    }

    diagnostics
}

fn collect_nullable_repetitions(
    rule_name: &str,
    expr: &GrammarExpr,
    nullability: &BTreeMap<String, bool>,
    diagnostics: &mut Vec<GrammarDiagnostic>,
) {
    match expr {
        GrammarExpr::ZeroOrMore(inner) => {
            push_nullable_repetition(
                rule_name,
                "zero-or-more repetition",
                inner,
                nullability,
                diagnostics,
            );
            collect_nullable_repetitions(rule_name, inner, nullability, diagnostics);
        }
        GrammarExpr::OneOrMore(inner) => {
            push_nullable_repetition(
                rule_name,
                "one-or-more repetition",
                inner,
                nullability,
                diagnostics,
            );
            collect_nullable_repetitions(rule_name, inner, nullability, diagnostics);
        }
        GrammarExpr::Repeat { expr: inner, .. } => {
            push_nullable_repetition(
                rule_name,
                "counted repetition",
                inner,
                nullability,
                diagnostics,
            );
            collect_nullable_repetitions(rule_name, inner, nullability, diagnostics);
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_nullable_repetitions(rule_name, alternative, nullability, diagnostics);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_nullable_repetitions(rule_name, item, nullability, diagnostics);
            }
        }
        GrammarExpr::Optional(inner)
        | GrammarExpr::And(inner)
        | GrammarExpr::Not(inner)
        | GrammarExpr::Capture { expr: inner, .. } => {
            collect_nullable_repetitions(rule_name, inner, nullability, diagnostics);
        }
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => {}
    }
}

fn push_nullable_repetition(
    rule_name: &str,
    repetition: &str,
    inner: &GrammarExpr,
    nullability: &BTreeMap<String, bool>,
    diagnostics: &mut Vec<GrammarDiagnostic>,
) {
    if !expr_is_nullable(inner, nullability) {
        return;
    }

    let detail = format!("{repetition} has nullable inner expression `{inner}`");
    diagnostics.push(GrammarDiagnostic {
        kind: DiagnosticKind::NullableRepetition {
            rule: rule_name.to_string(),
            detail: detail.clone(),
        },
        severity: Severity::Warning,
        message: format!(
            "rule `{rule_name}` uses a {detail}; make the repeated expression consume input or move the optional part outside the repetition."
        ),
        location: rule_location(rule_name),
    });
}

fn check_unused_captures(grammar: &Grammar) -> Vec<GrammarDiagnostic> {
    let mut diagnostics = Vec::new();
    for rule in grammar.rules() {
        collect_unused_captures(rule.name(), rule.expr(), &mut diagnostics);
    }
    diagnostics
}

fn collect_unused_captures(
    rule_name: &str,
    expr: &GrammarExpr,
    diagnostics: &mut Vec<GrammarDiagnostic>,
) {
    match expr {
        GrammarExpr::Capture {
            label: Some(label),
            expr,
        } => {
            diagnostics.push(GrammarDiagnostic {
                kind: DiagnosticKind::UnusedCapture {
                    rule: rule_name.to_string(),
                    label: label.clone(),
                },
                severity: Severity::Warning,
                message: format!(
                    "capture label `{label}` in rule `{rule_name}` is not used by grammar semantics; remove the label or wire it to a consumer."
                ),
                location: rule_location(rule_name),
            });
            collect_unused_captures(rule_name, expr, diagnostics);
        }
        GrammarExpr::Capture { label: None, expr }
        | GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Repeat { expr, .. } => collect_unused_captures(rule_name, expr, diagnostics),
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_unused_captures(rule_name, alternative, diagnostics);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_unused_captures(rule_name, item, diagnostics);
            }
        }
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar
        | GrammarExpr::NonTerminal(_) => {}
    }
}

fn left_reference_graph(
    grammar: &Grammar,
    nullability: &BTreeMap<String, bool>,
) -> BTreeMap<String, BTreeSet<String>> {
    let defined = grammar
        .rules()
        .iter()
        .map(GrammarRule::name)
        .collect::<BTreeSet<_>>();
    let mut graph = BTreeMap::<String, BTreeSet<String>>::new();

    for rule in grammar.rules() {
        let mut references = BTreeSet::new();
        collect_left_references(rule.expr(), nullability, &mut references);
        graph.entry(rule.name.clone()).or_default().extend(
            references
                .into_iter()
                .filter(|name| defined.contains(name.as_str())),
        );
    }

    graph
}

fn collect_left_references(
    expr: &GrammarExpr,
    nullability: &BTreeMap<String, bool>,
    references: &mut BTreeSet<String>,
) {
    match expr {
        GrammarExpr::NonTerminal(name) => {
            references.insert(name.clone());
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_left_references(alternative, nullability, references);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_left_references(item, nullability, references);
                if !expr_is_nullable(item, nullability) {
                    break;
                }
            }
        }
        GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Capture { expr, .. }
        | GrammarExpr::Repeat { expr, .. } => {
            collect_left_references(expr, nullability, references)
        }
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => {}
    }
}

fn compute_nullability(grammar: &Grammar) -> BTreeMap<String, bool> {
    let mut nullability = grammar
        .rules()
        .iter()
        .map(|rule| (rule.name.clone(), false))
        .collect::<BTreeMap<_, _>>();

    loop {
        let mut changed = false;
        for rule in grammar.rules() {
            if !expr_is_nullable(rule.expr(), &nullability) {
                continue;
            }

            let entry = nullability.entry(rule.name.clone()).or_insert(false);
            if !*entry {
                *entry = true;
                changed = true;
            }
        }

        if !changed {
            return nullability;
        }
    }
}

fn expr_is_nullable(expr: &GrammarExpr, nullability: &BTreeMap<String, bool>) -> bool {
    match expr {
        GrammarExpr::Empty => true,
        GrammarExpr::Terminal(value) | GrammarExpr::TerminalInsensitive(value) => value.is_empty(),
        GrammarExpr::CharRange(_, _) | GrammarExpr::CharClass { .. } | GrammarExpr::AnyChar => {
            false
        }
        GrammarExpr::NonTerminal(name) => nullability.get(name).copied().unwrap_or(false),
        GrammarExpr::Choice { alternatives, .. } => alternatives
            .iter()
            .any(|alternative| expr_is_nullable(alternative, nullability)),
        GrammarExpr::Sequence(items) => {
            items.iter().all(|item| expr_is_nullable(item, nullability))
        }
        GrammarExpr::Optional(_)
        | GrammarExpr::ZeroOrMore(_)
        | GrammarExpr::And(_)
        | GrammarExpr::Not(_) => true,
        GrammarExpr::OneOrMore(expr) | GrammarExpr::Capture { expr, .. } => {
            expr_is_nullable(expr, nullability)
        }
        GrammarExpr::Repeat { expr, min, .. } => *min == 0 || expr_is_nullable(expr, nullability),
    }
}

fn compute_suspicious_nullability(
    grammar: &Grammar,
    nullability: &BTreeMap<String, bool>,
) -> BTreeMap<String, bool> {
    let mut suspicious = grammar
        .rules()
        .iter()
        .map(|rule| (rule.name.clone(), false))
        .collect::<BTreeMap<_, _>>();

    loop {
        let mut changed = false;
        for rule in grammar.rules() {
            if !expr_is_suspicious_nullable(rule.expr(), nullability, &suspicious) {
                continue;
            }

            let entry = suspicious.entry(rule.name.clone()).or_insert(false);
            if !*entry {
                *entry = true;
                changed = true;
            }
        }

        if !changed {
            return suspicious;
        }
    }
}

fn expr_is_suspicious_nullable(
    expr: &GrammarExpr,
    nullability: &BTreeMap<String, bool>,
    suspicious: &BTreeMap<String, bool>,
) -> bool {
    match expr {
        GrammarExpr::Empty => true,
        GrammarExpr::Terminal(value) | GrammarExpr::TerminalInsensitive(value) => value.is_empty(),
        GrammarExpr::CharRange(_, _) | GrammarExpr::CharClass { .. } | GrammarExpr::AnyChar => {
            false
        }
        GrammarExpr::NonTerminal(name) => suspicious.get(name).copied().unwrap_or(false),
        GrammarExpr::Choice { alternatives, .. } => alternatives
            .iter()
            .any(|alternative| expr_is_suspicious_nullable(alternative, nullability, suspicious)),
        GrammarExpr::Sequence(items) => {
            items.is_empty()
                || (items.iter().all(|item| expr_is_nullable(item, nullability))
                    && items
                        .iter()
                        .any(|item| expr_is_suspicious_nullable(item, nullability, suspicious)))
        }
        GrammarExpr::Optional(_) | GrammarExpr::And(_) | GrammarExpr::Not(_) => true,
        GrammarExpr::ZeroOrMore(expr) | GrammarExpr::OneOrMore(expr) => {
            expr_is_nullable(expr, nullability)
        }
        GrammarExpr::Repeat { expr, .. } => expr_is_nullable(expr, nullability),
        GrammarExpr::Capture { expr, .. } => {
            expr_is_suspicious_nullable(expr, nullability, suspicious)
        }
    }
}

fn collect_nonterminals(expr: &GrammarExpr) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    collect_nonterminals_into(expr, &mut names);
    names
}

fn collect_nonterminals_into(expr: &GrammarExpr, names: &mut BTreeSet<String>) {
    match expr {
        GrammarExpr::NonTerminal(name) => {
            names.insert(name.clone());
        }
        GrammarExpr::Choice { alternatives, .. } => {
            for alternative in alternatives {
                collect_nonterminals_into(alternative, names);
            }
        }
        GrammarExpr::Sequence(items) => {
            for item in items {
                collect_nonterminals_into(item, names);
            }
        }
        GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Capture { expr, .. }
        | GrammarExpr::Repeat { expr, .. } => collect_nonterminals_into(expr, names),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => {}
    }
}

fn defined_rule_names(grammar: &Grammar) -> Vec<String> {
    let mut names = Vec::new();
    let mut seen = BTreeSet::new();
    for rule in grammar.rules() {
        if seen.insert(rule.name.clone()) {
            names.push(rule.name.clone());
        }
    }
    names
}

fn nearest_rule_name<'a>(name: &str, candidates: &'a [String]) -> Option<&'a str> {
    let mut best = None;

    for candidate in candidates {
        let distance = levenshtein(name, candidate);
        if distance > 2 {
            continue;
        }

        let should_replace = best.map_or(true, |(best_name, best_distance)| {
            distance < best_distance
                || (distance == best_distance && candidate.as_str() < best_name)
        });
        if should_replace {
            best = Some((candidate.as_str(), distance));
        }
    }

    best.map(|(candidate, _)| candidate)
}

fn levenshtein(left: &str, right: &str) -> usize {
    let left_chars = left.chars().collect::<Vec<_>>();
    let right_chars = right.chars().collect::<Vec<_>>();
    let mut previous = (0..=right_chars.len()).collect::<Vec<_>>();
    let mut current = vec![0; right_chars.len() + 1];

    for (left_index, left_char) in left_chars.iter().enumerate() {
        current[0] = left_index + 1;
        for (right_index, right_char) in right_chars.iter().enumerate() {
            let substitution_cost = usize::from(left_char != right_char);
            current[right_index + 1] = (previous[right_index + 1] + 1)
                .min(current[right_index] + 1)
                .min(previous[right_index] + substitution_cost);
        }
        previous.clone_from(&current);
    }

    previous[right_chars.len()]
}

fn canonical_cycle_key(cycle: &[String]) -> String {
    let nodes = &cycle[..cycle.len().saturating_sub(1)];
    if nodes.is_empty() {
        return String::new();
    }

    let mut best = None;
    for start in 0..nodes.len() {
        let rotation = (0..nodes.len())
            .map(|offset| nodes[(start + offset) % nodes.len()].as_str())
            .collect::<Vec<_>>()
            .join("\0");
        if best
            .as_ref()
            .map_or(true, |candidate| rotation < *candidate)
        {
            best = Some(rotation);
        }
    }
    best.unwrap_or_default()
}

fn sort_diagnostics(grammar: &Grammar, diagnostics: &mut [GrammarDiagnostic]) {
    let indices = grammar
        .rules()
        .iter()
        .enumerate()
        .rev()
        .map(|(index, rule)| (rule.name.clone(), index))
        .collect::<BTreeMap<_, _>>();

    diagnostics.sort_by(|left, right| {
        let left_span = left
            .location
            .span
            .as_ref()
            .map_or(usize::MAX, |span| span.start);
        let right_span = right
            .location
            .span
            .as_ref()
            .map_or(usize::MAX, |span| span.start);
        let left_index = indices
            .get(&left.location.rule)
            .copied()
            .unwrap_or(usize::MAX);
        let right_index = indices
            .get(&right.location.rule)
            .copied()
            .unwrap_or(usize::MAX);

        left_span
            .cmp(&right_span)
            .then_with(|| left_index.cmp(&right_index))
            .then_with(|| left.location.rule.cmp(&right.location.rule))
            .then_with(|| severity_rank(left.severity).cmp(&severity_rank(right.severity)))
            .then_with(|| kind_rank(&left.kind).cmp(&kind_rank(&right.kind)))
            .then_with(|| left.message.cmp(&right.message))
    });
}

const fn severity_rank(severity: Severity) -> usize {
    match severity {
        Severity::Error => 0,
        Severity::Warning => 1,
    }
}

const fn kind_rank(kind: &DiagnosticKind) -> usize {
    match kind {
        DiagnosticKind::DuplicateRule { .. } => 0,
        DiagnosticKind::UndefinedNonTerminal { .. } => 1,
        DiagnosticKind::LeftRecursion { .. } => 2,
        DiagnosticKind::UnreachableRule { .. } => 3,
        DiagnosticKind::NullableRepetition { .. } => 4,
        DiagnosticKind::UnusedCapture { .. } => 5,
    }
}

fn rule_location(rule: impl Into<String>) -> RuleSpan {
    RuleSpan {
        rule: rule.into(),
        span: None,
    }
}
