//! Deferred complete source ownership originals and shared transition decisions.
#[path = "../../src/translation/rust_owned_binding_rules.rs"]
mod rules;
use serde_json::{Value, json};
fn text(args: &Value, index: usize) -> &str {
    args[index].as_str().unwrap()
}
fn boolean(args: &Value, index: usize) -> bool {
    args[index].as_bool().unwrap()
}
fn strings(args: &Value, index: usize) -> Vec<String> {
    args[index]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap().to_owned())
        .collect()
}
#[test]
fn owned_vector_bindings_preserve_consumption_and_unavailable_reads() {
    super::translation_string_array_copy_sort::require_rust_profile();
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/owned-bindings.mjs",
        "ordinary Rust owned vector binding consumption",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-owned-binding-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-owned-bindings.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let actual = match row["operation"].as_str().unwrap() {
            "mergeRustOwnedLoanBindings" => json!(rules::merge_rust_owned_loan_bindings(
                &strings(args, 0),
                &strings(args, 1)
            )),
            "projectRustOwnedLoanBindings" => json!(rules::project_rust_owned_loan_bindings(
                text(args, 0),
                &strings(args, 1)
            )),
            "readRustOwnedLoanReadError" => {
                json!(rules::read_rust_owned_loan_read_error(&strings(args, 0)))
            }
            "readRustOwnedBorrowJoinError" => json!(rules::read_rust_owned_borrow_join_error(
                boolean(args, 0),
                boolean(args, 1),
                boolean(args, 2),
                boolean(args, 3)
            )),
            "acceptRustOwnedResultTransfer" => json!(rules::accept_rust_owned_result_transfer(
                boolean(args, 0),
                boolean(args, 1),
                boolean(args, 2),
                boolean(args, 3)
            )),
            "acceptRustOwnedBindingTracking" => json!(rules::accept_rust_owned_binding_tracking(
                boolean(args, 0),
                boolean(args, 1)
            )),
            "acceptRustOwnedBindingValue" => json!(rules::accept_rust_owned_binding_value(
                text(args, 0),
                text(args, 1),
                args[2].as_f64().unwrap(),
                boolean(args, 3)
            )),
            "readRustOwnedBindingReadError" => {
                json!(rules::read_rust_owned_binding_read_error(text(args, 0)))
            }
            "consumeRustOwnedBindingState" => {
                json!(rules::consume_rust_owned_binding_state(text(args, 0)))
            }
            "expireRustOwnedBindingState" => {
                json!(rules::expire_rust_owned_binding_state(text(args, 0)))
            }
            "mergeRustOwnedBindingStates" => json!(rules::merge_rust_owned_binding_states(
                text(args, 0),
                text(args, 1),
                boolean(args, 2),
                boolean(args, 3)
            )),
            "acceptRustOwnedBindingGuard" => json!(rules::accept_rust_owned_binding_guard(
                &strings(args, 0),
                &strings(args, 1)
            )),
            unknown => panic!("uncovered ownership decision {unknown}"),
        };
        assert_eq!(actual, row["expected"], "{row}");
    }
}

#[test]
fn retained_named_argument_inputs_have_tracked_consumption_proofs() {
    super::translation_string_array_copy_sort::require_rust_profile();
    for source in [
        include_str!("../../../parity/fixtures/rust-fresh-vector-arguments.lino"),
        include_str!("../../../parity/fixtures/rust-fresh-vector-results.lino"),
        include_str!("../../../parity/fixtures/rust-fresh-vector-call-results.lino"),
    ] {
        let data = meta_language::read_structured_data_lino(source).unwrap();
        for row in data["correctedAssertions"].as_array().unwrap() {
            meta_language::create_rust_source_owner_program_links(row["source"].as_str().unwrap())
                .unwrap();
        }
    }
}
