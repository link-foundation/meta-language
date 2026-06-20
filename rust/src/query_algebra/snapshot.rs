use crate::{LinkNetwork, ParseConfiguration};

use super::{LinkRule, LinkRuleRegistry};

/// Expected outcome for a rule snapshot case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkRuleSnapshotExpectation {
    /// The rule must match the source.
    Valid,
    /// The rule must not match the source.
    Invalid,
}

/// One valid/invalid source case for a rule suite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkRuleSnapshotCase {
    name: String,
    source: String,
    language: String,
    expectation: LinkRuleSnapshotExpectation,
}

impl LinkRuleSnapshotCase {
    /// Creates a snapshot case.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        source: impl Into<String>,
        language: impl Into<String>,
        expectation: LinkRuleSnapshotExpectation,
    ) -> Self {
        Self {
            name: name.into(),
            source: source.into(),
            language: language.into(),
            expectation,
        }
    }
}

/// Valid/invalid rule snapshot suite.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkRuleSnapshotSuite {
    rule: LinkRule,
    cases: Vec<LinkRuleSnapshotCase>,
}

impl LinkRuleSnapshotSuite {
    /// Creates a snapshot suite for `rule`.
    #[must_use]
    pub const fn new(rule: LinkRule) -> Self {
        Self {
            rule,
            cases: Vec::new(),
        }
    }

    /// Adds a case.
    #[must_use]
    pub fn with_case(mut self, case: LinkRuleSnapshotCase) -> Self {
        self.cases.push(case);
        self
    }

    /// Runs all cases against freshly parsed sources.
    #[must_use]
    pub fn run(
        &self,
        registry: &LinkRuleRegistry,
        configuration: ParseConfiguration,
    ) -> LinkRuleSnapshotReport {
        let cases = self
            .cases
            .iter()
            .map(|case| {
                let network = LinkNetwork::parse(&case.source, &case.language, configuration);
                let matches = self.rule.matches(&network, registry);
                let has_match = !matches.is_empty();
                let passed = match case.expectation {
                    LinkRuleSnapshotExpectation::Valid => has_match,
                    LinkRuleSnapshotExpectation::Invalid => !has_match,
                };
                LinkRuleSnapshotResult {
                    name: case.name.clone(),
                    expectation: case.expectation,
                    matched: has_match,
                    match_count: matches.len(),
                    passed,
                }
            })
            .collect();
        LinkRuleSnapshotReport { cases }
    }
}

/// Snapshot suite result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkRuleSnapshotReport {
    cases: Vec<LinkRuleSnapshotResult>,
}

impl LinkRuleSnapshotReport {
    /// Returns whether every case passed.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.cases.iter().all(|case| case.passed)
    }

    /// Per-case results.
    #[must_use]
    pub fn cases(&self) -> &[LinkRuleSnapshotResult] {
        &self.cases
    }
}

/// One snapshot case result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinkRuleSnapshotResult {
    name: String,
    expectation: LinkRuleSnapshotExpectation,
    matched: bool,
    match_count: usize,
    passed: bool,
}

impl LinkRuleSnapshotResult {
    /// Case name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Expected rule outcome.
    #[must_use]
    pub const fn expectation(&self) -> LinkRuleSnapshotExpectation {
        self.expectation
    }

    /// Whether the rule matched the parsed source.
    #[must_use]
    pub const fn matched(&self) -> bool {
        self.matched
    }

    /// Number of matches.
    #[must_use]
    pub const fn match_count(&self) -> usize {
        self.match_count
    }

    /// Whether this case passed.
    #[must_use]
    pub const fn passed(&self) -> bool {
        self.passed
    }
}
