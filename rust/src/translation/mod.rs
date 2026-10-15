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

pub mod aborts;
pub(crate) mod borrowed_call_interface;
pub(crate) mod borrowed_call_interface_rules;
pub mod check;
pub mod decimal;
pub mod diagnostics;
pub mod emit_common;
pub mod emit_javascript;
pub mod emit_lean;
pub mod emit_rocq;
pub mod emit_rust;
pub(crate) mod frontend_rules;
pub mod ir;
pub mod javascript;
mod javascript_line_terminator_rules;
pub mod js_number;
pub mod lean;
pub mod lean_root_names;
pub mod lexer;
pub mod output;
pub(crate) mod owned_local_state;
pub mod primitive_value_rules;
pub mod product_emission_rules;
pub mod proof;
pub mod rocq;
pub(crate) mod runtime_shell;
pub(crate) mod runtime_shell_rules;
pub mod rust;
pub(crate) mod scalar_loop_rules;
pub(crate) mod scoped_binding_emission;
pub(crate) mod scoped_binding_emission_rules;
pub(crate) mod source_const_functions;
pub(crate) mod source_empty_options;
pub(crate) mod source_header_emission;
pub(crate) mod source_header_emission_rules;
pub mod source_loops;
pub(crate) mod source_primitive_owners;
pub(crate) mod source_pure_unit_calls;
pub(crate) mod source_signature_rules;
pub mod source_signatures;
pub(crate) mod source_text_byte_rules;
pub(crate) mod source_text_bytes;
pub(crate) mod source_type_namespaces;
pub(crate) mod source_unit_clone_derivations;
pub mod structured_type_rules;
pub mod surface;
pub(crate) mod type_namespace_rules;
pub(crate) mod typed_program_link_rules;
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
