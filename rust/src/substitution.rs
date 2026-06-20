use std::collections::BTreeMap;

use crate::link_network::{Link, LinkId};

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

/// A link reference or named variable in a substitution pattern.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SubstitutionValue {
    /// Exact link reference.
    Link(LinkId),
    /// Variable reference such as `$source`.
    Variable(String),
}

impl SubstitutionValue {
    /// Creates an exact link reference.
    #[must_use]
    pub const fn link(link_id: LinkId) -> Self {
        Self::Link(link_id)
    }

    /// Creates a variable reference. A leading `$` is accepted and stripped.
    #[must_use]
    pub fn variable(name: impl Into<String>) -> Self {
        Self::Variable(normalize_variable_name(name))
    }
}

/// Match-and-substitute rule with link-cli-style variable bindings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariableSubstitutionRule {
    index_variable: Option<String>,
    pattern: Vec<SubstitutionValue>,
    replacement: Vec<SubstitutionValue>,
}

impl VariableSubstitutionRule {
    /// Creates a variable substitution rule.
    #[must_use]
    pub fn new<const P: usize, const R: usize>(
        pattern: [SubstitutionValue; P],
        replacement: [SubstitutionValue; R],
    ) -> Self {
        Self {
            index_variable: None,
            pattern: Vec::from(pattern),
            replacement: Vec::from(replacement),
        }
    }

    /// Binds the matched link id to a variable such as `$index`.
    #[must_use]
    pub fn with_index_variable(mut self, name: impl Into<String>) -> Self {
        self.index_variable = Some(normalize_variable_name(name));
        self
    }

    pub(crate) fn index_variable(&self) -> Option<&str> {
        self.index_variable.as_deref()
    }

    pub(crate) fn pattern(&self) -> &[SubstitutionValue] {
        &self.pattern
    }

    pub(crate) fn replacement(&self) -> &[SubstitutionValue] {
        &self.replacement
    }

    pub(crate) fn match_link(&self, link: &Link) -> Option<SubstitutionBindings> {
        if link.references().len() != self.pattern.len() {
            return None;
        }

        let mut bindings = SubstitutionBindings::default();
        if let Some(index_variable) = self.index_variable() {
            bindings.bind(index_variable, link.id())?;
        }

        for (pattern, reference) in self.pattern.iter().zip(link.references()) {
            match pattern {
                SubstitutionValue::Link(expected) if expected == reference => {}
                SubstitutionValue::Link(_) => return None,
                SubstitutionValue::Variable(name) => {
                    bindings.bind(name, *reference)?;
                }
            }
        }

        Some(bindings)
    }
}

/// Variables bound by one substitution match.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SubstitutionBindings {
    values: BTreeMap<String, LinkId>,
}

impl SubstitutionBindings {
    pub(crate) fn bind(&mut self, name: &str, link_id: LinkId) -> Option<()> {
        match self.values.get(name) {
            Some(existing) if *existing != link_id => None,
            Some(_) => Some(()),
            None => {
                self.values.insert(normalize_variable_name(name), link_id);
                Some(())
            }
        }
    }

    /// Returns the link bound to a variable name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<LinkId> {
        self.values.get(&normalize_variable_name(name)).copied()
    }

    /// Iterates variable bindings in name order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, LinkId)> {
        self.values
            .iter()
            .map(|(name, link_id)| (name.as_str(), *link_id))
    }

    pub(crate) fn resolve_values(&self, values: &[SubstitutionValue]) -> Option<Vec<LinkId>> {
        values
            .iter()
            .map(|value| match value {
                SubstitutionValue::Link(link_id) => Some(*link_id),
                SubstitutionValue::Variable(name) => self.get(name),
            })
            .collect()
    }
}

/// Result of applying a substitution rule.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SubstitutionReport {
    pub(crate) created: Vec<LinkId>,
    pub(crate) updated: Vec<LinkId>,
    pub(crate) deleted: Vec<LinkId>,
    pub(crate) bindings: Vec<SubstitutionBindings>,
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

    /// Variable bindings for each matched substitution.
    #[must_use]
    pub fn bindings(&self) -> &[SubstitutionBindings] {
        &self.bindings
    }
}

fn normalize_variable_name(name: impl Into<String>) -> String {
    name.into().trim_start_matches('$').to_string()
}
