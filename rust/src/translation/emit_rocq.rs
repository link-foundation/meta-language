//! The Rocq emitter: writes a checked program as Rocq with its translation contract.
//!
//! Mirrors `js/src/translation/emit-rocq.js`.

use super::diagnostics::{unsupported, Result};
use super::emit_common::Emitted;
use super::ir::Program;

/// Emits a checked program as Rocq.
///
/// # Errors
/// On constructs the target cannot express faithfully.
pub fn emit_rocq(program: &Program) -> Result<Emitted> {
    let _ = program;
    Err(unsupported("Rocq emitter", "not yet ported to Rust", None))
}
