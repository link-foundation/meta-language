//! Independent pinned parsers linked only by development tests.
use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_lean() -> *const ();
    fn tree_sitter_rocq() -> *const ();
}

/// The symbol compiled from the pinned Lean parser and layout scanner.
// SAFETY: build.rs compiles the exact vendored parser declaring this symbol.
pub const LEAN: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_lean) };

/// The symbol compiled from the pinned Rocq parser.
// SAFETY: build.rs compiles the exact vendored parser declaring this symbol.
pub const ROCQ: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_rocq) };
