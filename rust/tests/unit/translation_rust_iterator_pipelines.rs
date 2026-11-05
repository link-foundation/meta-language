//! Deferred original/generated products and every shared pipeline decision.
#[path = "../../src/translation/rust_copy_closure_parameter_rules.rs"]
mod parameters;
#[path = "../../src/translation/rust_iterator_pipeline_rules.rs"]
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
fn lazy_scalar_pipelines_preserve_complete_original_products_and_all_decisions() {
    super::translation_string_array_copy_sort::require_rust_profile();
    assert!(
        cfg!(target_pointer_width = "32") || cfg!(target_pointer_width = "64"),
        "uncovered source pointer width"
    );
    super::translation_rust_records::assert_nominal_translation_product(
        "../experiments/rust-records/iterator-pipelines.mjs",
        "lazy ordinary Rust scalar iterator pipelines",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/rust-iterator-pipeline-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
    let data = meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/rust-iterator-pipelines.lino"
    ))
    .unwrap();
    for row in data["premises"].as_array().unwrap() {
        let args = &row["args"];
        let actual = match row["operation"].as_str().unwrap() {
            "acceptRustIteratorPipelineStage" => {
                json!(rules::accept_rust_iterator_pipeline_stage(text(args, 0)))
            }
            "readRustIteratorPipelineParameterLayers" => json!(
                rules::read_rust_iterator_pipeline_parameter_layers(text(args, 0), number(args, 1))
            ),
            "readRustIteratorPipelineResultContext" => json!(
                rules::read_rust_iterator_pipeline_result_context(text(args, 0))
            ),
            "readRustIteratorPipelineResultState" => json!(
                rules::read_rust_iterator_pipeline_result_state(text(args, 0), text(args, 1))
            ),
            "readRustIteratorPipelineCallableError" => {
                json!(rules::read_rust_iterator_pipeline_callable_error(
                    text(args, 0),
                    number(args, 1),
                    number(args, 2),
                    boolean(args, 3),
                    boolean(args, 4),
                    boolean(args, 5),
                    boolean(args, 6)
                ))
            }
            "planRustIteratorPipelineStage" => {
                json!(rules::plan_rust_iterator_pipeline_stage(text(args, 0)))
            }
            "planRustIteratorPipelineTerminal" => {
                json!(rules::plan_rust_iterator_pipeline_terminal(text(args, 0)))
            }
            "readRustIteratorPipelineInactive" => {
                json!(rules::read_rust_iterator_pipeline_inactive(text(args, 0)))
            }
            "readRustIteratorPipelineConsumerOperation" => json!(
                rules::read_rust_iterator_pipeline_consumer_operation(text(args, 0))
            ),
            "planRustIteratorPipelineCaptures" => json!(
                rules::plan_rust_iterator_pipeline_captures(text(args, 0), number(args, 1))
            ),
            "readRustCopyClosureResultContext" => {
                json!(parameters::read_rust_copy_closure_result_context(
                    boolean(args, 0),
                    boolean(args, 1)
                ))
            }
            "testRustSourceReferenceLifetimeLayer" => json!(
                parameters::test_rust_source_reference_lifetime_layer(text(args, 0))
            ),
            "readRustIteratorPipelineContinuationItem" => json!(
                rules::read_rust_iterator_pipeline_continuation_item(text(args, 0))
            ),
            "readRustIteratorPipelineSeed" => {
                json!(rules::read_rust_iterator_pipeline_seed(text(args, 0)))
            }
            unknown => panic!("uncovered pipeline decision {unknown}"),
        };
        if actual.is_number() {
            assert_eq!(actual.as_f64(), row["expected"].as_f64(), "{row}");
        } else {
            assert_eq!(actual, row["expected"], "{row}");
        }
    }
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
