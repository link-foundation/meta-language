//! The JavaScript emitter: writes a checked program as JavaScript with its translation contract.
//!
//! Mirrors `js/src/translation/emit-javascript.js`.

use super::diagnostics::{unsupported, Result};
use super::emit_common::Emitted;
use super::ir::Program;

/// Emits a checked program as JavaScript.
///
/// # Errors
/// On constructs the target cannot express faithfully.
pub fn emit_javascript(program: &Program) -> Result<Emitted> {
    let _ = program;
    Err(unsupported(
        "JavaScript emitter",
        "not yet ported to Rust",
        None,
    ))
}
