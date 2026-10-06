//! Full-match verification of a [`LinkNetwork`] region.

use super::{Link, LinkNetwork};
use crate::source::ByteRange;
use crate::verification::{VerificationIssue, VerificationIssueKind, VerificationReport};

impl LinkNetwork {
    /// Verifies that the selected region has no error or missing links.
    #[must_use]
    pub fn verify_full_match(&self, region: Option<ByteRange>) -> VerificationReport {
        let issues = self
            .links()
            .filter(|link| link_is_in_region(link, region))
            .filter_map(|link| {
                let flags = link.metadata().flags();
                let kind = if flags.is_error() {
                    VerificationIssueKind::ErrorLink
                } else if flags.is_missing() {
                    VerificationIssueKind::MissingLink
                } else if flags.has_error() {
                    VerificationIssueKind::HasErrorLink
                } else {
                    return None;
                };

                Some(VerificationIssue::new(
                    link.id(),
                    kind,
                    link.metadata().span(),
                ))
            })
            .collect();
        VerificationReport::new(issues)
    }
}

fn link_is_in_region(link: &Link, region: Option<ByteRange>) -> bool {
    let Some(region) = region else {
        return true;
    };
    link.metadata()
        .span()
        .is_some_and(|span| span.byte_range().intersects(region))
}
