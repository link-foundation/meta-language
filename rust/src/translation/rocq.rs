//! The Rocq frontend: reads a Rocq program into the surface AST.
//!
//! Mirrors `js/src/translation/rocq.js`.

use super::diagnostics::{unsupported, Result};
use super::surface::SProgram;

/// Reads a Rocq program.
///
/// # Errors
/// On syntax errors and on constructs outside the portable core.
pub fn parse_rocq(source: &str) -> Result<SProgram> {
    let _ = source;
    Err(unsupported("Rocq frontend", "not yet ported to Rust", None))
}
