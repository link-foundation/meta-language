//! Deferred original factory results, complete source policies and all codecs.
#[test]
fn terminal_pointer_results_retain_actual_contexts_and_capture_constraints() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/pointer-results.mjs",
        "Rust terminal function pointer result contexts",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-contextual-closure-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
