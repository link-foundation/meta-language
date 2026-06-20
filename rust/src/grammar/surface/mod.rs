//! Meta-notation-derived textual surface syntax for grammar authoring.
//!
//! The public parser first runs the input through the existing links-network
//! parse boundary, then lowers the surface tokens into the grammar IR. The
//! `LiNo` helpers reuse the grammar links codec plus [`LinkNetwork::to_lino`] and
//! [`LinkNetwork::from_lino`], so there is still only one canonical network
//! serializer.

use std::error::Error;
use std::fmt;

use crate::grammar::Grammar;
use crate::link_network::{Link, LinkId, LinkNetwork, LinkType};
use crate::rust_codec::{FromLinks, LinksDecoder, LinksEncoder, ToLinks};
use crate::ParseConfiguration;

mod lower;
mod token;
mod write;

const GRAMMAR_SURFACE_LANGUAGE: &str = "grammar-surface";
const GRAMMAR_ROOT_TAG: &str = "grammar::grammar";

/// Error raised while parsing grammar surface text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarSurfaceError {
    /// The delimiter skeleton could not be parsed.
    Skeleton {
        /// Human-readable error message.
        message: String,
    },
    /// A skeleton node had no valid lowering.
    Lowering {
        /// Rule being lowered, when known.
        rule: Option<String>,
        /// Human-readable error message.
        message: String,
    },
    /// A rule referenced a name that no rule defines.
    UndefinedReference {
        /// Rule containing the undefined reference.
        rule: String,
        /// Referenced rule name that was not defined.
        name: String,
    },
}

impl fmt::Display for GrammarSurfaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Skeleton { message } => write!(formatter, "grammar skeleton error: {message}"),
            Self::Lowering { rule, message } => match rule {
                Some(rule) => write!(formatter, "grammar lowering error in {rule}: {message}"),
                None => write!(formatter, "grammar lowering error: {message}"),
            },
            Self::UndefinedReference { rule, name } => {
                write!(formatter, "rule {rule} references undefined rule {name}")
            }
        }
    }
}

impl Error for GrammarSurfaceError {}

/// Parses meta-notation-derived grammar surface text into the grammar IR.
///
/// # Errors
///
/// Returns [`GrammarSurfaceError`] when the delimiter skeleton is malformed,
/// surface nodes cannot be lowered, or non-terminal references are undefined.
pub fn parse_grammar_surface(text: &str) -> Result<Grammar, GrammarSurfaceError> {
    let _skeleton = skeletonise(text);
    let tokens = token::parse_surface_tokens(text)?;
    lower::lower_document(&tokens)
}

/// Lifts a grammar back to canonical surface text.
#[must_use]
pub fn write_grammar_surface(grammar: &Grammar) -> String {
    write::write_surface(grammar)
}

/// Encodes a grammar through the existing links codec and serializes it as `LiNo`.
#[must_use]
pub fn grammar_to_lino(grammar: &Grammar) -> String {
    let mut encoder = LinksEncoder::new();
    let _root = grammar.to_links(&mut encoder);
    encoder.into_network().to_lino()
}

/// Decodes a grammar from `LiNo` text produced by [`grammar_to_lino`].
///
/// # Errors
///
/// Returns [`GrammarSurfaceError`] when the `LiNo` text or grammar links are
/// malformed.
pub fn grammar_from_lino(text: &str) -> Result<Grammar, GrammarSurfaceError> {
    let network = LinkNetwork::from_lino(text).map_err(|error| GrammarSurfaceError::Skeleton {
        message: error.to_string(),
    })?;
    let root = grammar_root(&network).ok_or_else(|| GrammarSurfaceError::Lowering {
        rule: None,
        message: "LiNo network does not contain a grammar root".to_string(),
    })?;
    let mut decoder = LinksDecoder::new(&network);
    Grammar::from_links(&mut decoder, root).map_err(|error| GrammarSurfaceError::Lowering {
        rule: None,
        message: error.to_string(),
    })
}

fn skeletonise(text: &str) -> LinkNetwork {
    LinkNetwork::parse(
        text,
        GRAMMAR_SURFACE_LANGUAGE,
        ParseConfiguration::default(),
    )
}

fn grammar_root(network: &LinkNetwork) -> Option<LinkId> {
    network
        .links()
        .find(|link| {
            link.metadata().link_type() == Some(LinkType::Grammar)
                && link.metadata().term() == Some(GRAMMAR_ROOT_TAG)
        })
        .map(Link::id)
}

fn skeleton_error(message: impl Into<String>) -> GrammarSurfaceError {
    GrammarSurfaceError::Skeleton {
        message: message.into(),
    }
}

fn lowering_error(rule: Option<&str>, message: impl Into<String>) -> GrammarSurfaceError {
    GrammarSurfaceError::Lowering {
        rule: rule.map(str::to_string),
        message: message.into(),
    }
}
