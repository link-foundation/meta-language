//! Source-only binding forms; checked terms remain ordinary lets and matches.
use super::{SPattern, Span, Token};

pub(super) enum ProductBindingPattern {
    Name(Token),
    Wildcard(Token),
    Tuple {
        items: Vec<Self>,
        rest: Option<usize>,
        span: Option<Span>,
    },
    Unit {
        span: Option<Span>,
    },
    Nominal(SPattern),
}

impl ProductBindingPattern {
    pub(super) const fn source_span(&self) -> Option<Span> {
        match self {
            Self::Name(token) | Self::Wildcard(token) => Some(token.span()),
            Self::Tuple { span, .. } | Self::Unit { span } => *span,
            Self::Nominal(pattern) => pattern.span,
        }
    }
}
