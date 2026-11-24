//! Deferred complete owned-string originals and the shared source decisions.
#[path = "../../src/translation/rust_owned_binding_rules.rs"]
mod rules;
#[test]
fn owned_strings_preserve_moves_copies_and_source_loans() {
    super::translation_string_array_copy_sort::require_rust_profile();
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/owned-strings.mjs",
        "ordinary Rust owned string binding consumption",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-owned-binding-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-owned-strings.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let actual = match row["operation"].as_str().unwrap() {
            "acceptRustStringSliceCloneReference" => {
                serde_json::json!(rules::accept_rust_string_slice_clone_reference(
                    args[0].as_str().unwrap(),
                    args[1].as_str().unwrap(),
                    args[2].as_str().unwrap(),
                    args[3].as_f64().unwrap()
                ))
            }
            "acceptRustOwnedMacroArgumentTransfer" => serde_json::json!(
                rules::accept_rust_owned_macro_argument_transfer(args[0].as_str().unwrap())
            ),
            "acceptRustOwnedBindingValue" => {
                serde_json::json!(rules::accept_rust_owned_binding_value(
                    args[0].as_str().unwrap(),
                    args[1].as_str().unwrap(),
                    args[2].as_f64().unwrap(),
                    args[3].as_bool().unwrap()
                ))
            }
            "readRustOwnedStringConstructionError" => {
                serde_json::json!(rules::read_rust_owned_string_construction_error(
                    args[0].as_str().unwrap(),
                    args[1].as_str().unwrap(),
                    args[2].as_f64().unwrap(),
                    args[3].as_bool().unwrap(),
                    args[4].as_bool().unwrap()
                ))
            }
            unknown => panic!("uncovered string ownership decision {unknown}"),
        };
        assert_eq!(actual, row["expected"], "{row}");
    }
}
