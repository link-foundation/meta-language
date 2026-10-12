//! Independent development oracle with the pinned Agda grammar and corrected scanner state.
use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_agda_layout() -> *const ();
}

/// The generated parser retains every upstream production.
// SAFETY: build.rs compiles the pinned parser under this separate symbol.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_agda_layout) };
