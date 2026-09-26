//! The Rust emitter: writes a checked program as Rust with its translation contract.
//!
//! Mirrors `js/src/translation/emit-rust.js`.

use super::diagnostics::{unsupported, Result};
use super::emit_common::Emitted;
use super::ir::Program;

/// Emits a checked program as Rust.
///
/// # Errors
/// On constructs the target cannot express faithfully.
pub fn emit_rust(program: &Program) -> Result<Emitted> {
    let _ = program;
    Err(unsupported("Rust emitter", "not yet ported to Rust", None))
}
