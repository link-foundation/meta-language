//! Future original/native values, complete source definitions and all codecs.
#[test]
fn let_else_returns_preserve_their_real_scopes_and_continuations() {
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/let-else-completions.mjs",
        "Rust let-else completed fallbacks",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/let-else-completion-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
