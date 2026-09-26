//! The Rust frontend: reads a Rust program into the surface AST.
//!
//! Mirrors `js/src/translation/rust.js`.

use super::diagnostics::{unsupported, Result};
use super::surface::SProgram;

/// Reads a Rust program.
///
/// # Errors
/// On syntax errors and on constructs outside the portable core.
pub fn parse_rust(source: &str) -> Result<SProgram> {
    let _ = source;
    Err(unsupported("Rust frontend", "not yet ported to Rust", None))
}
