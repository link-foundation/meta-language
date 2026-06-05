use crate::link_network::LinkId;
use crate::source::SourceSpan;

/// Verification issue kind for a full-match check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VerificationIssueKind {
    /// A link explicitly marks a parse error.
    ErrorLink,
    /// A link marks source text missing from the parse.
    MissingLink,
    /// A link contains a parse error below it.
    HasErrorLink,
}

/// One verification issue tied to a link.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationIssue {
    link_id: LinkId,
    kind: VerificationIssueKind,
    span: Option<SourceSpan>,
}

impl VerificationIssue {
    pub(crate) const fn new(
        link_id: LinkId,
        kind: VerificationIssueKind,
        span: Option<SourceSpan>,
    ) -> Self {
        Self {
            link_id,
            kind,
            span,
        }
    }

    /// Link that caused the issue.
    #[must_use]
    pub const fn link_id(&self) -> LinkId {
        self.link_id
    }

    /// Issue kind.
    #[must_use]
    pub const fn kind(&self) -> VerificationIssueKind {
        self.kind
    }

    /// Source span attached to the issue link, when available.
    #[must_use]
    pub const fn span(&self) -> Option<SourceSpan> {
        self.span
    }
}

/// Result of verifying that a source region fully matches a language.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationReport {
    issues: Vec<VerificationIssue>,
}

impl VerificationReport {
    pub(crate) fn new(issues: Vec<VerificationIssue>) -> Self {
        Self { issues }
    }

    /// Whether the verified region has no error or missing links.
    #[must_use]
    pub fn is_clean(&self) -> bool {
        self.issues.is_empty()
    }

    /// Verification issues found in the region.
    #[must_use]
    pub fn issues(&self) -> &[VerificationIssue] {
        &self.issues
    }
}
