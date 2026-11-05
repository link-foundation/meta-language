//! Deferred source, definition, codec, emission and native products.
use super::translation_rust_records::assert_nominal_translation_product;
#[test]
fn conditional_chains_preserve_scope_short_circuit_and_success_payloads() {
    assert_nominal_translation_product(
        "../experiments/rust-records/conditional-chains.mjs",
        "Rust 2024 conditional chains",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/conditional-chain-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
