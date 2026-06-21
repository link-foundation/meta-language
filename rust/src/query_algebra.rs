use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::{Link, LinkId, LinkNetwork, LinkQuery, LinkType, SourceTextPredicateHost};

mod snapshot;
mod syntax;
mod text_pattern;

pub use snapshot::{
    LinkRuleSnapshotCase, LinkRuleSnapshotExpectation, LinkRuleSnapshotReport,
    LinkRuleSnapshotResult, LinkRuleSnapshotSuite,
};

use text_pattern::TextPattern;

/// Composable rule over links.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkRule {
    kind: LinkRuleKind,
}

impl LinkRule {
    /// Wraps an existing structural query as a composable rule.
    #[must_use]
    pub const fn query(query: LinkQuery) -> Self {
        Self {
            kind: LinkRuleKind::Query(query),
        }
    }

    /// Matches links by metadata term/kind.
    #[must_use]
    pub fn kind(kind: impl Into<String>) -> Self {
        Self {
            kind: LinkRuleKind::Kind(kind.into()),
        }
    }

    /// Matches links by link type.
    #[must_use]
    pub const fn link_type(link_type: LinkType) -> Self {
        Self {
            kind: LinkRuleKind::LinkType(link_type),
        }
    }

    /// Captures links selected by `rule`.
    #[must_use]
    pub fn capture(name: impl Into<String>, rule: Self) -> Self {
        Self {
            kind: LinkRuleKind::Capture {
                name: normalize_capture_name(name.into()),
                rule: Box::new(rule),
            },
        }
    }

    /// Captures links whose kind matches `kind`.
    #[must_use]
    pub fn typed_metavariable(name: impl Into<String>, kind: impl Into<String>) -> Self {
        Self {
            kind: LinkRuleKind::TypedMetavariable {
                name: normalize_capture_name(name.into()),
                kind: kind.into(),
            },
        }
    }

    /// Matches `rule` only when selected links are inside `ancestor`.
    #[must_use]
    pub fn inside(rule: Self, ancestor: Self) -> Self {
        Self {
            kind: LinkRuleKind::Inside {
                rule: Box::new(rule),
                ancestor: Box::new(ancestor),
            },
        }
    }

    /// Matches `rule` only when selected links contain a descendant.
    #[must_use]
    pub fn has(rule: Self, descendant: Self) -> Self {
        Self {
            kind: LinkRuleKind::Has {
                rule: Box::new(rule),
                descendant: Box::new(descendant),
            },
        }
    }

    /// Matches `rule` only when selected links precede `following`.
    #[must_use]
    pub fn precedes(rule: Self, following: Self) -> Self {
        Self {
            kind: LinkRuleKind::Precedes {
                rule: Box::new(rule),
                following: Box::new(following),
            },
        }
    }

    /// Matches `rule` only when selected links follow `preceding`.
    #[must_use]
    pub fn follows(rule: Self, preceding: Self) -> Self {
        Self {
            kind: LinkRuleKind::Follows {
                rule: Box::new(rule),
                preceding: Box::new(preceding),
            },
        }
    }

    /// Intersects rules by selected link id.
    #[must_use]
    pub fn all(rules: impl Into<Vec<Self>>) -> Self {
        Self {
            kind: LinkRuleKind::All(rules.into()),
        }
    }

    /// Unions rules by selected link id.
    #[must_use]
    pub fn any(rules: impl Into<Vec<Self>>) -> Self {
        Self {
            kind: LinkRuleKind::Any(rules.into()),
        }
    }

    /// Selects links not selected by `rule`.
    #[must_use]
    pub fn negate(rule: Self) -> Self {
        Self {
            kind: LinkRuleKind::Not(Box::new(rule)),
        }
    }

    /// Refers to a named rule in a [`LinkRuleRegistry`].
    #[must_use]
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            kind: LinkRuleKind::Ref(name.into()),
        }
    }

    /// Matches a parent whose ordered children contain `before ... after`.
    #[must_use]
    pub fn ellipsis_gap(before: Self, after: Self) -> Self {
        Self {
            kind: LinkRuleKind::Ellipsis {
                before: Box::new(before),
                after: Box::new(after),
            },
        }
    }

    /// Matches a full document's plain source text with `{{capture}}` holes.
    pub fn text(pattern: impl Into<String>) -> Result<Self, LinkRuleParseError> {
        Ok(Self {
            kind: LinkRuleKind::Text(TextPattern::parse(pattern.into())?),
        })
    }

    /// Parses the documented rule-algebra S-expression surface.
    pub fn from_sexpression(source: &str) -> Result<Self, LinkRuleParseError> {
        syntax::parse_rule(source)
    }

    /// Returns matches for this rule using `registry` for named sub-rules.
    #[must_use]
    pub fn matches(
        &self,
        network: &LinkNetwork,
        registry: &LinkRuleRegistry,
    ) -> Vec<LinkRuleMatch> {
        let context = RuleContext { network, registry };
        dedupe_matches(self.evaluate(&context, &mut Vec::new()))
    }

    fn evaluate(&self, context: &RuleContext<'_>, stack: &mut Vec<String>) -> Vec<LinkRuleMatch> {
        match &self.kind {
            LinkRuleKind::Query(query) => context
                .network
                .query_matches_with(query, &SourceTextPredicateHost)
                .into_iter()
                .map(|query_match| LinkRuleMatch::from_query_match(&query_match))
                .collect(),
            LinkRuleKind::Kind(kind) => context
                .network
                .links()
                .filter(|link| link.metadata().term() == Some(kind.as_str()))
                .map(|link| LinkRuleMatch::new(link.id()))
                .collect(),
            LinkRuleKind::LinkType(link_type) => context
                .network
                .links()
                .filter(|link| link.metadata().link_type() == Some(*link_type))
                .map(|link| LinkRuleMatch::new(link.id()))
                .collect(),
            LinkRuleKind::Language(language) => context
                .network
                .links()
                .filter(|link| link.metadata().language() == Some(language.as_str()))
                .map(|link| LinkRuleMatch::new(link.id()))
                .collect(),
            LinkRuleKind::Named(named) => context
                .network
                .links()
                .filter(|link| link.metadata().is_named() == *named)
                .map(|link| LinkRuleMatch::new(link.id()))
                .collect(),
            LinkRuleKind::Capture { name, rule } => rule
                .evaluate(context, stack)
                .into_iter()
                .map(|rule_match| {
                    let link_id = rule_match.link_id;
                    rule_match.with_link_capture(name, link_id)
                })
                .collect(),
            LinkRuleKind::TypedMetavariable { name, kind } => context
                .network
                .links()
                .filter(|link| link.metadata().term() == Some(kind.as_str()))
                .map(|link| LinkRuleMatch::new(link.id()).with_link_capture(name, link.id()))
                .collect(),
            LinkRuleKind::Inside { rule, ancestor } => {
                let ancestor_ids = ancestor
                    .evaluate(context, stack)
                    .into_iter()
                    .map(|rule_match| rule_match.link_id)
                    .collect::<BTreeSet<_>>();
                rule.evaluate(context, stack)
                    .into_iter()
                    .filter(|rule_match| {
                        ancestors(context.network, rule_match.link_id)
                            .iter()
                            .any(|ancestor| ancestor_ids.contains(ancestor))
                    })
                    .collect()
            }
            LinkRuleKind::Has { rule, descendant } => {
                let descendants = descendant.evaluate(context, stack);
                rule.evaluate(context, stack)
                    .into_iter()
                    .flat_map(|outer| {
                        descendants
                            .iter()
                            .filter(move |inner| {
                                is_descendant(context.network, inner.link_id, outer.link_id)
                            })
                            .map(move |inner| outer.merge_as(outer.link_id, inner))
                    })
                    .collect()
            }
            LinkRuleKind::Precedes { rule, following } => {
                let following = following.evaluate(context, stack);
                rule.evaluate(context, stack)
                    .into_iter()
                    .flat_map(|left| {
                        following
                            .iter()
                            .filter(move |right| {
                                order_key(context.network, left.link_id)
                                    < order_key(context.network, right.link_id)
                            })
                            .map(move |right| left.merge_as(left.link_id, right))
                    })
                    .collect()
            }
            LinkRuleKind::Follows { rule, preceding } => {
                let preceding = preceding.evaluate(context, stack);
                rule.evaluate(context, stack)
                    .into_iter()
                    .flat_map(|right| {
                        preceding
                            .iter()
                            .filter(move |left| {
                                order_key(context.network, left.link_id)
                                    < order_key(context.network, right.link_id)
                            })
                            .map(move |left| right.merge_as(right.link_id, left))
                    })
                    .collect()
            }
            LinkRuleKind::All(rules) => match rules.split_first() {
                Some((first, rest)) => {
                    rest.iter()
                        .fold(first.evaluate(context, stack), |acc, rule| {
                            let matches = rule.evaluate(context, stack);
                            intersect_matches(acc, &matches)
                        })
                }
                None => Vec::new(),
            },
            LinkRuleKind::Any(rules) => {
                let mut matches = Vec::new();
                for rule in rules {
                    matches.extend(rule.evaluate(context, stack));
                }
                dedupe_matches(matches)
            }
            LinkRuleKind::Not(rule) => {
                let rejected = rule
                    .evaluate(context, stack)
                    .into_iter()
                    .map(|rule_match| rule_match.link_id)
                    .collect::<BTreeSet<_>>();
                context
                    .network
                    .links()
                    .filter(|link| !rejected.contains(&link.id()))
                    .map(|link| LinkRuleMatch::new(link.id()))
                    .collect()
            }
            LinkRuleKind::Ref(name) => {
                if stack.iter().any(|entry| entry == name) {
                    return Vec::new();
                }
                let Some(rule) = context.registry.get(name) else {
                    return Vec::new();
                };
                stack.push(name.clone());
                let matches = rule.evaluate(context, stack);
                stack.pop();
                matches
            }
            LinkRuleKind::Ellipsis { before, after } => {
                ellipsis_matches(context, before, after, stack)
            }
            LinkRuleKind::Text(pattern) => pattern.matches(context.network),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum LinkRuleKind {
    Query(LinkQuery),
    Kind(String),
    LinkType(LinkType),
    Language(String),
    Named(bool),
    Capture {
        name: String,
        rule: Box<LinkRule>,
    },
    TypedMetavariable {
        name: String,
        kind: String,
    },
    Inside {
        rule: Box<LinkRule>,
        ancestor: Box<LinkRule>,
    },
    Has {
        rule: Box<LinkRule>,
        descendant: Box<LinkRule>,
    },
    Precedes {
        rule: Box<LinkRule>,
        following: Box<LinkRule>,
    },
    Follows {
        rule: Box<LinkRule>,
        preceding: Box<LinkRule>,
    },
    All(Vec<LinkRule>),
    Any(Vec<LinkRule>),
    Not(Box<LinkRule>),
    Ref(String),
    Ellipsis {
        before: Box<LinkRule>,
        after: Box<LinkRule>,
    },
    Text(TextPattern),
}

/// Named reusable rule registry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkRuleRegistry {
    rules: BTreeMap<String, LinkRule>,
}

impl LinkRuleRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a registry with `name` bound to `rule`.
    #[must_use]
    pub fn with_rule(mut self, name: impl Into<String>, rule: LinkRule) -> Self {
        self.insert(name, rule);
        self
    }

    /// Inserts or replaces a named reusable rule.
    pub fn insert(&mut self, name: impl Into<String>, rule: LinkRule) {
        self.rules.insert(name.into(), rule);
    }

    /// Looks up a named reusable rule.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&LinkRule> {
        self.rules.get(name)
    }
}

impl std::ops::Not for LinkRule {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            kind: LinkRuleKind::Not(Box::new(self)),
        }
    }
}

/// One rule match.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkRuleMatch {
    link_id: LinkId,
    captures: LinkRuleCaptures,
}

impl LinkRuleMatch {
    const fn new(link_id: LinkId) -> Self {
        Self {
            link_id,
            captures: LinkRuleCaptures { values: Vec::new() },
        }
    }

    fn from_query_match(query_match: &crate::QueryMatch) -> Self {
        let mut captures = LinkRuleCaptures::default();
        for capture in query_match.captures().iter() {
            captures = captures.with_link(capture.name(), capture.link_id());
        }
        Self {
            link_id: query_match.link_id(),
            captures,
        }
    }

    fn with_link_capture(mut self, name: &str, link_id: LinkId) -> Self {
        self.captures = self.captures.with_link(name, link_id);
        self
    }

    fn merge(&self, other: &Self) -> Option<Self> {
        (self.link_id == other.link_id).then(|| Self {
            link_id: self.link_id,
            captures: self.captures.merged(&other.captures),
        })
    }

    fn merge_as(&self, link_id: LinkId, other: &Self) -> Self {
        Self {
            link_id,
            captures: self.captures.merged(&other.captures),
        }
    }

    /// Selected link id.
    #[must_use]
    pub const fn link_id(&self) -> LinkId {
        self.link_id
    }

    /// Capture bindings.
    #[must_use]
    pub const fn captures(&self) -> &LinkRuleCaptures {
        &self.captures
    }
}

/// Ordered captures created by a [`LinkRule`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkRuleCaptures {
    values: Vec<LinkRuleCapture>,
}

impl LinkRuleCaptures {
    fn with_link(mut self, name: &str, link_id: LinkId) -> Self {
        self.values.push(LinkRuleCapture {
            name: normalize_capture_name(name),
            link_ids: vec![link_id],
            text: None,
        });
        self
    }

    fn with_text(mut self, name: &str, text: String, link_ids: Vec<LinkId>) -> Self {
        self.values.push(LinkRuleCapture {
            name: normalize_capture_name(name),
            link_ids,
            text: Some(text),
        });
        self
    }

    fn merged(&self, other: &Self) -> Self {
        let mut values = self.values.clone();
        values.extend(other.values.clone());
        Self { values }
    }

    /// Returns the first captured link id for `name`.
    #[must_use]
    pub fn first(&self, name: &str) -> Option<LinkId> {
        let name = normalize_capture_name(name);
        self.values
            .iter()
            .find(|capture| capture.name == name)
            .and_then(|capture| capture.link_ids.first().copied())
    }

    /// Returns captured text for `name` when the capture came from text matching.
    #[must_use]
    pub fn text(&self, name: &str) -> Option<&str> {
        let name = normalize_capture_name(name);
        self.values
            .iter()
            .find(|capture| capture.name == name)
            .and_then(|capture| capture.text.as_deref())
    }

    /// Iterates capture bindings in match order.
    pub fn iter(&self) -> impl Iterator<Item = &LinkRuleCapture> {
        self.values.iter()
    }
}

/// One rule capture.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkRuleCapture {
    name: String,
    link_ids: Vec<LinkId>,
    text: Option<String>,
}

impl LinkRuleCapture {
    /// Capture name without leading `@`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Captured link ids.
    #[must_use]
    pub fn link_ids(&self) -> &[LinkId] {
        &self.link_ids
    }

    /// Captured text when available.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

/// Traversal ordering for rule matches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraversalStrategy {
    /// Parents before children.
    TopDown,
    /// Children before parents.
    BottomUp,
    /// Only matches that do not contain another match.
    Innermost,
    /// Re-apply a mutable visitor until it reports no changes or reaches the cap.
    Fixpoint { max_iterations: usize },
}

impl TraversalStrategy {
    /// Returns rule matches ordered by this strategy.
    #[must_use]
    pub fn matches(
        self,
        network: &LinkNetwork,
        rule: &LinkRule,
        registry: &LinkRuleRegistry,
    ) -> Vec<LinkRuleMatch> {
        let mut matches = rule.matches(network, registry);
        match self {
            Self::TopDown | Self::Fixpoint { .. } => {
                matches.sort_by_key(|rule_match| {
                    (
                        depth(network, rule_match.link_id),
                        order_key(network, rule_match.link_id),
                    )
                });
            }
            Self::BottomUp => {
                matches.sort_by_key(|rule_match| {
                    (
                        std::cmp::Reverse(depth(network, rule_match.link_id)),
                        order_key(network, rule_match.link_id),
                    )
                });
            }
            Self::Innermost => {
                let all_matches = matches.clone();
                matches.retain(|candidate| {
                    !all_matches.iter().any(|other| {
                        other.link_id != candidate.link_id
                            && is_descendant(network, other.link_id, candidate.link_id)
                    })
                });
                matches.sort_by_key(|rule_match| {
                    (
                        std::cmp::Reverse(depth(network, rule_match.link_id)),
                        order_key(network, rule_match.link_id),
                    )
                });
            }
        }
        matches
    }

    /// Visits matches according to this strategy. `Fixpoint` repeats until the
    /// visitor returns no changes.
    pub fn apply_mut<F>(
        self,
        network: &mut LinkNetwork,
        rule: &LinkRule,
        registry: &LinkRuleRegistry,
        mut visitor: F,
    ) -> TraversalReport
    where
        F: FnMut(&mut LinkNetwork, &LinkRuleMatch) -> bool,
    {
        match self {
            Self::Fixpoint { max_iterations } => {
                let mut report = TraversalReport::default();
                for _ in 0..max_iterations {
                    let matches = Self::TopDown.matches(network, rule, registry);
                    if matches.is_empty() {
                        break;
                    }
                    report.iterations += 1;
                    let mut changed_this_iteration = 0;
                    for rule_match in matches {
                        report.visited += 1;
                        if visitor(network, &rule_match) {
                            report.changed += 1;
                            changed_this_iteration += 1;
                        }
                    }
                    if changed_this_iteration == 0 {
                        break;
                    }
                }
                report
            }
            strategy => {
                let matches = strategy.matches(network, rule, registry);
                let mut report = TraversalReport {
                    iterations: usize::from(!matches.is_empty()),
                    visited: 0,
                    changed: 0,
                };
                for rule_match in matches {
                    report.visited += 1;
                    if visitor(network, &rule_match) {
                        report.changed += 1;
                    }
                }
                report
            }
        }
    }
}

/// Summary from mutable traversal.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TraversalReport {
    iterations: usize,
    visited: usize,
    changed: usize,
}

impl TraversalReport {
    /// Completed traversal iterations.
    #[must_use]
    pub const fn iterations(&self) -> usize {
        self.iterations
    }

    /// Number of visited matches.
    #[must_use]
    pub const fn visited(&self) -> usize {
        self.visited
    }

    /// Number of visitor calls that reported a change.
    #[must_use]
    pub const fn changed(&self) -> usize {
        self.changed
    }
}

/// Error returned while parsing rule-algebra syntax.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkRuleParseError {
    message: String,
}

impl LinkRuleParseError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for LinkRuleParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for LinkRuleParseError {}

struct RuleContext<'a> {
    network: &'a LinkNetwork,
    registry: &'a LinkRuleRegistry,
}

fn ellipsis_matches(
    context: &RuleContext<'_>,
    before: &LinkRule,
    after: &LinkRule,
    stack: &mut Vec<String>,
) -> Vec<LinkRuleMatch> {
    let before_matches = before.evaluate(context, stack);
    let after_matches = after.evaluate(context, stack);
    let before_by_id = matches_by_id(&before_matches);
    let after_by_id = matches_by_id(&after_matches);
    let mut matches = Vec::new();

    for parent in context.network.links() {
        let children = structural_children(context.network, parent.id());
        for (left_index, left) in children.iter().enumerate() {
            let Some(left_matches) = before_by_id.get(left) else {
                continue;
            };
            for right in children.iter().skip(left_index + 1) {
                let Some(right_matches) = after_by_id.get(right) else {
                    continue;
                };
                for left_match in left_matches {
                    for right_match in right_matches {
                        matches.push(left_match.merge_as(parent.id(), right_match));
                    }
                }
            }
        }
    }

    matches
}

fn intersect_matches(left: Vec<LinkRuleMatch>, right: &[LinkRuleMatch]) -> Vec<LinkRuleMatch> {
    let right_by_id = matches_by_id(right);
    left.into_iter()
        .flat_map(|left_match| {
            right_by_id
                .get(&left_match.link_id)
                .into_iter()
                .flatten()
                .filter_map(move |right_match| left_match.merge(right_match))
        })
        .collect()
}

fn matches_by_id(matches: &[LinkRuleMatch]) -> BTreeMap<LinkId, Vec<LinkRuleMatch>> {
    let mut by_id = BTreeMap::<LinkId, Vec<LinkRuleMatch>>::new();
    for rule_match in matches {
        by_id
            .entry(rule_match.link_id)
            .or_default()
            .push(rule_match.clone());
    }
    by_id
}

fn dedupe_matches(matches: Vec<LinkRuleMatch>) -> Vec<LinkRuleMatch> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for rule_match in matches {
        if seen.insert(rule_match.link_id) {
            deduped.push(rule_match);
        }
    }
    deduped
}

fn structural_children(network: &LinkNetwork, parent: LinkId) -> Vec<LinkId> {
    let mut children = network
        .links()
        .filter(|link| link.references().first().copied() == Some(parent))
        .filter(|link| {
            !matches!(
                link.metadata().link_type(),
                Some(LinkType::Field | LinkType::Trivia)
            )
        })
        .map(Link::id)
        .collect::<Vec<_>>();
    children.sort_by_key(|child| order_key(network, *child));
    children
}

fn ancestors(network: &LinkNetwork, link_id: LinkId) -> Vec<LinkId> {
    let mut ancestors = Vec::new();
    let mut visited = BTreeSet::new();
    let mut current = link_id;
    while visited.insert(current) {
        let Some(parent) = network
            .link(current)
            .and_then(|link| link.references().first().copied())
        else {
            break;
        };
        if parent == current {
            break;
        }
        ancestors.push(parent);
        current = parent;
    }
    ancestors
}

fn depth(network: &LinkNetwork, link_id: LinkId) -> usize {
    ancestors(network, link_id).len()
}

fn is_descendant(network: &LinkNetwork, descendant: LinkId, ancestor: LinkId) -> bool {
    ancestors(network, descendant).contains(&ancestor)
}

fn order_key(network: &LinkNetwork, link_id: LinkId) -> (usize, u64) {
    let start = network
        .link(link_id)
        .and_then(|link| link.metadata().span())
        .map_or(usize::MAX, |span| span.byte_range().start());
    (start, link_id.as_u64())
}

fn normalize_capture_name(name: impl AsRef<str>) -> String {
    name.as_ref().trim_start_matches('@').to_string()
}
