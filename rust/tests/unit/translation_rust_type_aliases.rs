//! Deferred original Rust oracle, complete definitions and public products.
#[test]
fn ordinary_source_aliases_keep_complete_original_products() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/type-aliases.mjs",
        "ordinary transparent Rust source type aliases",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-type-alias-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
#[test]
fn native_alias_publications_keep_actual_values_and_signatures() {
    super::translation_rust_function_qualifiers::assert_rust_publication_products(
        "../experiments/rust-records/type-alias-publications.mjs",
    );
}
