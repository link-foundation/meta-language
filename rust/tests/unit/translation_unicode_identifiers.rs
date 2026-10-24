//! Future paired graph, codec, compiler and original-language controls. CI only.
use super::translation_rust_records::assert_nominal_translation_product;
use meta_language::create_unicode_scalar_program_links;
use meta_language::{
    SOURCE_IDENTIFIER_UNICODE_VERSION, normalize_unicode_text, read_structured_data_lino,
};

#[test]
fn unicode_rust_identifiers_preserve_normalized_names_spelling_spans_and_values() {
    assert_nominal_translation_product(
        "../experiments/unicode-identifiers/rust.mjs",
        "Unicode Rust source identifiers",
        create_unicode_scalar_program_links,
    );
}
#[test]
fn unicode_javascript_identifiers_preserve_distinct_names_and_original_values() {
    assert_nominal_translation_product(
        "../experiments/unicode-identifiers/javascript.mjs",
        "Unicode JavaScript source identifiers",
        create_unicode_scalar_program_links,
    );
}

#[test]
fn escaped_javascript_identifiers_preserve_lexical_roles_original_syntax_and_values() {
    assert_nominal_translation_product(
        "../experiments/unicode-identifiers/escaped-javascript.mjs",
        "escaped JavaScript identifiers",
        meta_language::create_java_script_identifier_program_links,
    );
}

#[test]
fn unicode_lifetimes_preserve_names_intervals_and_character_boundaries() {
    assert_nominal_translation_product(
        "../experiments/unicode-identifiers/rust-lifetimes.mjs",
        "Unicode Rust lifetimes",
        create_unicode_scalar_program_links,
    );
}

#[test]
fn canonical_normalization_retains_exclusions_order_hangul_and_compatibility_distinctions() {
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/unicode-source-scalars.lino"
    ))
    .unwrap();
    assert_eq!(
        SOURCE_IDENTIFIER_UNICODE_VERSION,
        fixture["unicode"].as_str().unwrap()
    );
    for row in fixture["normalizations"].as_array().unwrap() {
        let expected = row["expected"].as_str().unwrap();
        assert_eq!(
            normalize_unicode_text(row["source"].as_str().unwrap()).unwrap(),
            expected
        );
        assert_eq!(normalize_unicode_text(expected).unwrap(), expected);
    }
    for row in fixture["identifierSources"].as_array().unwrap() {
        for language in [
            meta_language::translation::Language::JavaScript,
            meta_language::translation::Language::Rust,
        ] {
            let source = row["source"].as_str().unwrap();
            let tokens = meta_language::translation::lexer::tokenize(source, language)
                .unwrap()
                .tokens;
            let whole = tokens.len() == 2
                && tokens[0].kind == meta_language::translation::lexer::TokenKind::Identifier
                && tokens[0].raw == source;
            let key = if language == meta_language::translation::Language::Rust {
                "Rust"
            } else {
                "JavaScript"
            };
            assert_eq!(whole, row[key].as_bool().unwrap(), "{key}: {source}");
            if whole {
                assert_eq!(
                    (tokens[0].start, tokens[0].end),
                    (0, source.encode_utf16().count())
                );
            }
        }
    }
}
