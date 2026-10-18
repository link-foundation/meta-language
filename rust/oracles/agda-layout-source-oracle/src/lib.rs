//! Independent development oracle with the pinned Agda grammar and corrected scanner state.
use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_agda_layout() -> *const ();
    fn tree_sitter_agda_queue_safe() -> *const ();
    fn meta_language_agda_queue_growth_control() -> std::ffi::c_int;
}

/// The generated parser retains every upstream production.
// SAFETY: build.rs compiles the pinned parser under this separate symbol.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_agda_layout) };

/// Retain the original focused oracle semantics with safe wrapped queue growth.
// SAFETY: the same pinned parser is compiled under its separate queue-safe name.
pub const QUEUE_SAFE_LANGUAGE: LanguageFn =
    unsafe { LanguageFn::from_raw(tree_sitter_agda_queue_safe) };

/// Exercise the actual scanner queue across its original one-element capacity.
#[must_use]
pub fn queue_growth_control() -> bool {
    // SAFETY: the generated control owns and releases its local scanner queue.
    unsafe { meta_language_agda_queue_growth_control() == 1 }
}
