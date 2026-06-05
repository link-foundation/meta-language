use crate::link_network::LinkId;

/// Match-and-substitute rule over exact link references.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubstitutionRule {
    pattern: Vec<LinkId>,
    replacement: Vec<LinkId>,
}

impl SubstitutionRule {
    /// Creates a rule that replaces links with `pattern` references by
    /// `replacement` references.
    #[must_use]
    pub fn new<const P: usize, const R: usize>(
        pattern: [LinkId; P],
        replacement: [LinkId; R],
    ) -> Self {
        Self {
            pattern: pattern.to_vec(),
            replacement: replacement.to_vec(),
        }
    }

    /// Creates a rule that inserts a new relation link.
    #[must_use]
    pub fn create<const R: usize>(replacement: [LinkId; R]) -> Self {
        Self {
            pattern: Vec::new(),
            replacement: replacement.to_vec(),
        }
    }

    /// Creates a rule that deletes links matching `pattern`.
    #[must_use]
    pub fn delete<const P: usize>(pattern: [LinkId; P]) -> Self {
        Self {
            pattern: pattern.to_vec(),
            replacement: Vec::new(),
        }
    }

    pub(crate) fn pattern(&self) -> &[LinkId] {
        &self.pattern
    }

    pub(crate) fn replacement(&self) -> &[LinkId] {
        &self.replacement
    }
}

/// Result of applying a substitution rule.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SubstitutionReport {
    pub(crate) created: Vec<LinkId>,
    pub(crate) updated: Vec<LinkId>,
    pub(crate) deleted: Vec<LinkId>,
}

impl SubstitutionReport {
    /// Created link ids.
    #[must_use]
    pub fn created(&self) -> &[LinkId] {
        &self.created
    }

    /// Updated link ids.
    #[must_use]
    pub fn updated(&self) -> &[LinkId] {
        &self.updated
    }

    /// Deleted link ids.
    #[must_use]
    pub fn deleted(&self) -> &[LinkId] {
        &self.deleted
    }
}
