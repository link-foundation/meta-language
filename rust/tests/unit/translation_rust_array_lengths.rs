//! Future original/native lengths, complete source, codecs and all emissions.
#[test]
fn ordinary_array_lengths_retain_their_exact_unsigned_source_width() {
    assert!(
        cfg!(target_pointer_width = "64"),
        "uncovered source usize target width; this fixture selects the 64-bit source model"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/array-lengths.mjs",
        "Rust exact unsigned array lengths",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-array-length-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}
