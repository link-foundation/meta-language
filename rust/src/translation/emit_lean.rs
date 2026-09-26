//! The Lean emitter: writes a checked program as Lean with its translation contract.
//!
//! Mirrors `js/src/translation/emit-lean.js`.

use super::diagnostics::{unsupported, Result};
use super::emit_common::Emitted;
use super::ir::Program;

/// Emits a checked program as Lean.
///
/// # Errors
/// On constructs the target cannot express faithfully.
pub fn emit_lean(program: &Program) -> Result<Emitted> {
    let _ = program;
    Err(unsupported("Lean emitter", "not yet ported to Rust", None))
}
