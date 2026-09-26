//! Translation between JavaScript, Rust, Lean and Rocq through a checked
//! portable core.
//!
//! Each frontend reads a source program into the surface AST,
//! the checker resolves names, types and the exact operator semantics, and
//! each emitter writes a target program together with its mapping, the
//! encodings it chose and the assumptions it relies on.
//!
//! Mirrors `js/src/translation/`; both runtimes produce the same surface
//! trees, checked programs, target texts and contracts.

use serde::{Deserialize, Serialize};

pub mod check;
pub mod decimal;
pub mod diagnostics;
pub mod emit_common;
pub mod emit_javascript;
pub mod emit_lean;
pub mod emit_rocq;
pub mod emit_rust;
pub mod ir;
pub mod javascript;
pub mod lean;
pub mod lean_root_names;
pub mod lexer;
pub mod proof;
pub mod rocq;
pub mod rust;
pub mod surface;
pub mod types;

/// A source range in UTF-16 code units, as JavaScript string offsets count.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// The four languages of the translation matrix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    JavaScript,
    Rust,
    Lean,
    Rocq,
}

impl Language {
    pub const ALL: [Self; 4] = [Self::JavaScript, Self::Rust, Self::Lean, Self::Rocq];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JavaScript => "JavaScript",
            Self::Rust => "Rust",
            Self::Lean => "Lean",
            Self::Rocq => "Rocq",
        }
    }

    #[must_use]
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|language| language.as_str() == name)
    }
}
