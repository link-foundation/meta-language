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

macro_rules! native_oracle_baselines {
    ($(($id:literal, $version:literal, $directory:literal, $archive:literal, $symbol:literal, $entry:ident),)*) => {
        unsafe extern "C" {
            $(fn $entry() -> *const ();)*
        }

        /// Resolves the independent tree-sitter oracle version recorded by the native port.
        #[must_use]
        pub fn baseline(grammar: &str, version: &str) -> Option<LanguageFn> {
            match (grammar, version) {
                // SAFETY: build.rs compiles each archived parser with its declared baseline symbol.
                $(($id, $version) => Some(unsafe { LanguageFn::from_raw($entry) }),)*
                _ => None,
            }
        }
    };
}
include!("../baselines.rs");
