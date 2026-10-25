//! Deferred original/native scalar String byte counts, codecs and emissions.
#[test]
fn ordinary_string_lengths_retain_utf8_bytes_and_exact_source_width() {
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source usize target width; this fixture selects the 64-bit source model"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/string-byte-lengths.mjs",
        "Rust Unicode scalar String byte lengths",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-string-byte-length-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
