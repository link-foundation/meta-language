//! Importers for external grammar definition formats.

use std::error::Error;
use std::fmt;

use crate::grammar::GrammarFormat;

mod abnf;
mod bnf;
mod ebnf;
mod pest;

pub use abnf::import_abnf;
pub use bnf::import_bnf;
pub use ebnf::import_ebnf;
pub use pest::import_pest;

/// Error raised while importing an external grammar notation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrammarImportError {
    /// The source text could not be parsed or validated as the requested format.
    Parse {
        /// Grammar format being imported.
        format: GrammarFormat,
        /// Human-readable error message.
        message: String,
    },
    /// The importer parsed the input but cannot lower this construct yet.
    Unsupported {
        /// Grammar format being imported.
        format: GrammarFormat,
        /// Construct name or summary.
        construct: String,
    },
}

impl fmt::Display for GrammarImportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { format, message } => {
                write!(formatter, "{format} import parse error: {message}")
            }
            Self::Unsupported { format, construct } => {
                write!(
                    formatter,
                    "{format} import unsupported construct: {construct}"
                )
            }
        }
    }
}

impl Error for GrammarImportError {}

pub(super) fn parse_error(format: GrammarFormat, message: impl Into<String>) -> GrammarImportError {
    GrammarImportError::Parse {
        format,
        message: message.into(),
    }
}

pub(super) fn unsupported_error(
    format: GrammarFormat,
    construct: impl Into<String>,
) -> GrammarImportError {
    GrammarImportError::Unsupported {
        format,
        construct: construct.into(),
    }
}
