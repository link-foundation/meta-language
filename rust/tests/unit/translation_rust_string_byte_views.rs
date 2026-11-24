//! Deferred complete byte-view originals, UTF8 columns and shared source policies.
#[path = "../../src/translation/rust_string_byte_kernel.rs"]
mod kernel;
#[path = "../../src/translation/rust_string_byte_view_rules.rs"]
mod rules;
use serde_json::{Value, json};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
fn number(args: &Value, index: usize) -> f64 {
    args[index].as_f64().unwrap()
}
fn boolean(args: &Value, index: usize) -> bool {
    args[index].as_bool().unwrap()
}
#[test]
fn source_byte_views_preserve_utf8_ownership_lifetimes_and_source_templates() {
    super::translation_string_array_copy_sort::require_rust_profile();
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/string-byte-views.mjs",
        "ordinary Rust string byte views",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-string-byte-view-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-string-byte-views.lino"
    ))
    .unwrap();
    for row in data["encodings"].as_array().unwrap() {
        let input = kernel::ml_string::Text::from_text(row["text"].as_str().unwrap());
        let expected = row["bytes"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_f64().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(kernel::encode_rust_string_utf8(&input), expected, "{row}");
    }
    for units in data["invalidKernelUnits"].as_array().unwrap() {
        let input = kernel::ml_string::Text::from_units(
            units
                .as_array()
                .unwrap()
                .iter()
                .map(|value| u16::try_from(value.as_u64().unwrap()).unwrap())
                .collect(),
        );
        let failure =
            std::panic::catch_unwind(|| kernel::encode_rust_string_utf8(&input)).unwrap_err();
        let (class, message, present) = failure
            .downcast_ref::<(String, Vec<u16>, bool)>()
            .expect("the complete source TypeError payload");
        assert_eq!(class, "TypeError");
        assert_eq!(
            String::from_utf16(message).unwrap(),
            "Rust text needs Unicode scalar values"
        );
        assert!(*present);
    }
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let actual = match row["operation"].as_str().unwrap() {
            "acceptRustStringByteViewCandidate" => json!(
                rules::accept_rust_string_byte_view_candidate(text(args, 0), boolean(args, 1))
            ),
            "readRustStringByteViewError" => json!(rules::read_rust_string_byte_view_error(
                text(args, 0),
                number(args, 1),
                text(args, 2),
                text(args, 3),
                number(args, 4),
                boolean(args, 5),
                boolean(args, 6)
            )),
            "readRustStringByteViewStorage" => {
                json!(rules::read_rust_string_byte_view_storage(text(args, 0)))
            }
            "readRustStringByteViewLayers" => {
                json!(rules::read_rust_string_byte_view_layers(text(args, 0)))
            }
            "readRustStringByteIteratorError" => json!(
                rules::read_rust_string_byte_iterator_error(text(args, 0), boolean(args, 1))
            ),
            "acceptRustStringByteKernelUnitRequirement" => {
                json!(rules::accept_rust_string_byte_kernel_unit_requirement(
                    text(args, 0),
                    text(args, 1),
                    text(args, 2),
                    text(args, 3)
                ))
            }
            "renderRustStringByteScalarBridge" => {
                json!(rules::render_rust_string_byte_scalar_bridge(
                    text(args, 0),
                    text(args, 1),
                    boolean(args, 2)
                ))
            }
            "acceptRustStringByteScalarBridge" => json!(
                rules::accept_rust_string_byte_scalar_bridge(text(args, 0), boolean(args, 1))
            ),
            unknown => panic!("uncovered byte-view decision {unknown}"),
        };
        assert_eq!(actual, row["expected"], "{row}");
    }
}
