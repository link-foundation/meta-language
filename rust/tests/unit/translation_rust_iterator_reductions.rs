//! Deferred source products and every shared reduction/owner decision.
#[path = "../../src/translation/rust_array_element_owner_rules.rs"]
mod owners;
#[path = "../../src/translation/rust_copy_closure_parameter_rules.rs"]
mod parameters;
#[path = "../../src/translation/rust_iterator_reduction_rules.rs"]
mod rules;
#[path = "../../src/translation/rust_type_token_rules.rs"]
mod type_tokens;
use serde_json::{Value, json};
fn string(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
fn number(args: &Value, index: usize) -> f64 {
    args[index].as_f64().unwrap()
}
fn boolean(args: &Value, index: usize) -> bool {
    args[index].as_bool().unwrap()
}
#[test]
fn iterator_reductions_preserve_products_and_every_source_decision() {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "32") || cfg!(target_pointer_width = "64"),
        "uncovered source pointer width"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/iterator-reductions.mjs",
        "ordinary Rust immediate copied scalar iterator reductions",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-iterator-reduction-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-iterator-reductions.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let actual = match row["operation"].as_str().unwrap() {
            "readRustIteratorReductionResultContext" => json!(
                rules::read_rust_iterator_reduction_result_context(string(args, 0))
            ),
            "acceptRustIteratorReductionFusedTerminal" => {
                json!(rules::accept_rust_iterator_reduction_fused_terminal(
                    string(args, 0),
                    number(args, 1)
                ))
            }
            "acceptRustIteratorReductionStart" => {
                json!(rules::accept_rust_iterator_reduction_start(
                    string(args, 0),
                    number(args, 1),
                    boolean(args, 2)
                ))
            }
            "acceptRustIteratorCopyAdapter" => {
                json!(rules::accept_rust_iterator_copy_adapter(string(args, 0)))
            }
            "readRustIteratorReductionOperation" => json!(
                rules::read_rust_iterator_reduction_operation(string(args, 0))
            ),
            "readRustIteratorReductionCallbackArity" => json!(
                rules::read_rust_iterator_reduction_callback_arity(string(args, 0))
            ),
            "readRustIteratorReductionCaptureRoles" => json!(
                rules::read_rust_iterator_reduction_capture_roles(string(args, 0))
            ),
            "readRustIteratorReductionReceiverError" => {
                json!(rules::read_rust_iterator_reduction_receiver_error(
                    boolean(args, 0),
                    boolean(args, 1)
                ))
            }
            "readRustIteratorReductionAccumulatorError" => {
                json!(rules::read_rust_iterator_reduction_accumulator_error(
                    boolean(args, 0),
                    boolean(args, 1),
                    number(args, 2)
                ))
            }
            "readRustIteratorReductionCallbackError" => {
                let parameters = args[4]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| value.as_bool().unwrap())
                    .collect::<Vec<_>>();
                json!(rules::read_rust_iterator_reduction_callback_error(
                    string(args, 0),
                    string(args, 1),
                    number(args, 2),
                    number(args, 3),
                    &parameters,
                    boolean(args, 5)
                ))
            }
            "acceptRustIteratorReductionAdapter" => json!(
                rules::accept_rust_iterator_reduction_adapter(string(args, 0))
            ),
            "readRustIteratorReductionAdapterState" => {
                json!(rules::read_rust_iterator_reduction_adapter_state(
                    string(args, 0),
                    string(args, 1),
                    string(args, 2)
                ))
            }
            "planRustIteratorReductionReceiver" => json!(
                rules::plan_rust_iterator_reduction_receiver(string(args, 0), string(args, 1))
            ),
            "planRustIteratorReductionReverseIndex" => {
                json!(rules::plan_rust_iterator_reduction_reverse_index())
            }
            "readRustIteratorReductionCountResultOwner" => {
                json!(rules::read_rust_iterator_reduction_count_result_owner())
            }
            "planRustIteratorReductionCount" => json!(rules::plan_rust_iterator_reduction_count()),
            "acceptRustIteratorCollectionInferredType" => {
                json!(rules::accept_rust_iterator_collection_inferred_type(
                    string(args, 0),
                    boolean(args, 1)
                ))
            }
            "readRustIteratorCollectionError" => json!(rules::read_rust_iterator_collection_error(
                boolean(args, 0),
                boolean(args, 1),
                string(args, 2),
                string(args, 3),
                string(args, 4),
                string(args, 5),
                number(args, 6),
                boolean(args, 7),
                number(args, 8)
            )),
            "readRustIteratorCountReceiverError" => {
                json!(rules::read_rust_iterator_count_receiver_error(
                    string(args, 0),
                    string(args, 1),
                    boolean(args, 2),
                    boolean(args, 3),
                    boolean(args, 4)
                ))
            }
            "readRustIteratorReductionInitialState" => json!(
                rules::read_rust_iterator_reduction_initial_state(string(args, 0))
            ),
            "readRustIteratorOwnedReceiverError" => {
                json!(rules::read_rust_iterator_owned_receiver_error(
                    string(args, 0),
                    string(args, 1),
                    boolean(args, 2),
                    number(args, 3),
                    boolean(args, 4)
                ))
            }
            unknown => panic!("uncovered shared decision {unknown}"),
        };
        if actual.is_number() {
            assert_eq!(actual.as_f64(), row["expected"].as_f64(), "{row}");
        } else {
            assert_eq!(actual, row["expected"], "{row}");
        }
    }
    for row in data["typeTokens"].as_array().unwrap() {
        assert_eq!(
            json!(type_tokens::read_rust_type_closing_token_parts(
                row["token"].as_str().unwrap()
            )),
            row["expected"]
        );
    }
    for row in data["ownerSelections"].as_array().unwrap() {
        let identities = row["identities"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        let admitted = row["admitted"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_bool().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(
            owners::select_rust_array_element_owner_index(&identities, &admitted),
            row["expected"].as_f64().unwrap()
        );
    }
}

#[test]
fn every_immutable_scalar_parameter_admission() {
    super::translation_string_array_copy_sort::require_rust_profile();
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-iterator-reductions.lino"
    ))
    .unwrap();
    for row in data["parameterPremises"].as_array().unwrap() {
        assert_eq!(
            parameters::accept_rust_copy_closure_parameter_source(
                row["known"].as_bool().unwrap(),
                row["baseCopy"].as_bool().unwrap(),
                row["layers"].as_f64().unwrap(),
                row["reference"].as_bool().unwrap(),
                row["primitiveOwner"].as_bool().unwrap()
            ),
            row["expected"].as_bool().unwrap(),
            "{row}"
        );
    }
}
