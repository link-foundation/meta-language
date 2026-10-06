//! Trivia attachment for [`LinkNetwork`] tokens.

use super::{LinkId, LinkMetadata, LinkNetwork, LinkType};
use crate::configuration::TriviaAttachmentPolicy;
use crate::link_flags::LinkFlags;
use crate::source::SourceSpan;

impl LinkNetwork {
    pub(crate) fn attach_trivia(
        &mut self,
        document: LinkId,
        token: LinkId,
        span: SourceSpan,
        policy: TriviaAttachmentPolicy,
    ) {
        match policy {
            TriviaAttachmentPolicy::ContainmentLink => {
                self.insert_containment_trivia(document, token, span);
            }
            TriviaAttachmentPolicy::TokenLink => {
                self.insert_token_trivia(token, span);
            }
            TriviaAttachmentPolicy::Both => {
                self.insert_containment_trivia(document, token, span);
                self.insert_token_trivia(token, span);
            }
        }
    }

    fn insert_containment_trivia(&mut self, document: LinkId, token: LinkId, span: SourceSpan) {
        self.insert_link(
            [document, token],
            LinkMetadata::new()
                .with_link_type(LinkType::Trivia)
                .with_term("containment trivia")
                .with_span(span)
                .with_flags(LinkFlags::extra()),
        );
    }

    fn insert_token_trivia(&mut self, token: LinkId, span: SourceSpan) {
        self.insert_link(
            [token],
            LinkMetadata::new()
                .with_link_type(LinkType::Trivia)
                .with_term("token trivia")
                .with_span(span)
                .with_flags(LinkFlags::extra()),
        );
    }
}
