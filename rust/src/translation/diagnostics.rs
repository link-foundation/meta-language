//! Errors raised by the portable-core translator.
//!
//! `unsupported` errors are the precise obligations the issue requires: they name the construct, the
//! source span, and the reason no faithful encoding exists yet.
//!
//! Mirrors `js/src/translation/diagnostics.js`; messages are identical.

use std::fmt;

use serde::{Deserialize, Serialize};

use super::Span;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ErrorKind {
    Syntax,
    Type,
    Unsupported,
}

impl ErrorKind {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Syntax => "syntax",
            Self::Type => "type",
            Self::Unsupported => "unsupported",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TranslationError {
    pub kind: ErrorKind,
    /// The message without the span suffix.
    pub reason: String,
    pub span: Option<Span>,
    /// The construct an `unsupported` error names.
    pub construct: Option<String>,
}

impl TranslationError {
    #[must_use]
    pub fn new(kind: ErrorKind, reason: impl Into<String>, span: Option<Span>) -> Self {
        Self {
            kind,
            reason: reason.into(),
            span,
            construct: None,
        }
    }

    #[must_use]
    pub fn syntax(reason: impl Into<String>, span: Option<Span>) -> Self {
        Self::new(ErrorKind::Syntax, reason, span)
    }

    /// The message: the reason followed by the span, as `reason at 3..7`.
    #[must_use]
    pub fn message(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for TranslationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.span {
            Some(span) => write!(formatter, "{} at {}..{}", self.reason, span.start, span.end),
            None => formatter.write_str(&self.reason),
        }
    }
}

impl std::error::Error for TranslationError {}

/// A precise obligation for a construct outside the portable core.
#[must_use]
pub fn unsupported(construct: &str, reason: &str, span: Option<Span>) -> TranslationError {
    TranslationError {
        kind: ErrorKind::Unsupported,
        reason: format!("{construct}: {reason}"),
        span,
        construct: Some(construct.to_owned()),
    }
}

#[must_use]
pub fn type_error(reason: impl Into<String>, span: Option<Span>) -> TranslationError {
    TranslationError::new(ErrorKind::Type, reason, span)
}

pub type Result<T> = std::result::Result<T, TranslationError>;
