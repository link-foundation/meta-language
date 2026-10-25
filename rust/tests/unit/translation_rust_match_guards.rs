//! Deferred original-source, native and shared definition products.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn match_guards_preserve_order_scope_or_alternatives_and_exhaustiveness() {
    assert_nominal_translation_product(
        "../experiments/rust-records/match-guards.mjs",
        "Rust ordered match guards",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/conditional-chain-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
