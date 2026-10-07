//! The independent CMake parser is linked only by development tests.
use tree_sitter_language::LanguageFn;
unsafe extern "C" {
    fn tree_sitter_cmake() -> *const ();
}
/// The symbol compiled from the pinned parser and patched scanner.
// SAFETY: build.rs compiles the exact vendored parser declaring this symbol.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_cmake) };
