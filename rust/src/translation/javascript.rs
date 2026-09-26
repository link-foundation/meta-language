//! The JavaScript frontend: reads a JavaScript program into the surface AST.
//!
//! Mirrors `js/src/translation/javascript.js`.

use super::diagnostics::{unsupported, Result};
use super::surface::SProgram;

/// Reads a JavaScript program.
///
/// # Errors
/// On syntax errors and on constructs outside the portable core.
pub fn parse_javascript(source: &str) -> Result<SProgram> {
    let _ = source;
    Err(unsupported(
        "JavaScript frontend",
        "not yet ported to Rust",
        None,
    ))
}
