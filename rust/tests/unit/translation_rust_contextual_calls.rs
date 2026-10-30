//! Deferred ordinary header discovery, real callback bodies and original products.
#[test]
fn complete_headers_supply_actual_named_computed_and_module_callback_contexts() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/contextual-calls.mjs",
        "Rust actual declaration headers and callback contexts",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-declaration-header-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
