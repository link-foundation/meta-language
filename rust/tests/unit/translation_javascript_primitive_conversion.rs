//! Deferred complete protocol parity; semantics and transports are generated.
#[path = "javascript_primitive_conversion_codecs.rs"]
mod codecs;
#[path = "../../src/translation/javascript_primitive_conversion.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use codecs::*;
use meta_language::read_structured_data_lino;
use native::JavaScriptPrimitiveConversionTransition as Transition;
use std::process::Command;

#[test]
fn primitive_conversion_keeps_every_state_call_and_original_coercion() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = actual
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-properties/primitive-conversion-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let observations = read_structured_data_lino(&producer.stdout).unwrap();
    for row in observations["cases"].as_array().unwrap() {
        let heap = decode_java_script_ordinary_heap(&row["heap"]);
        assert_eq!(encode_java_script_ordinary_heap(&heap), row["heap"]);
        let input = decode_java_script_semantic_value(&row["input"]);
        assert_eq!(encode_java_script_semantic_value(&input), row["input"]);
        let key = decode_java_script_semantic_value(&row["exoticKey"]);
        assert_eq!(encode_java_script_semantic_value(&key), row["exoticKey"]);
        let mut actual = native::begin_java_script_primitive_conversion(
            heap,
            input,
            key,
            row["hint"].as_str().unwrap(),
        );
        let trace = row["trace"].as_array().unwrap();
        let steps = row["steps"].as_array().unwrap();
        assert_eq!(trace.len(), steps.len() + 1);
        assert_eq!(
            encode_java_script_primitive_conversion_transition(&actual),
            trace[0]
        );
        for (index, step) in steps.iter().enumerate() {
            let heap = decode_java_script_ordinary_heap(&step["heap"]);
            assert_eq!(encode_java_script_ordinary_heap(&heap), step["heap"]);
            actual = match (step["operation"].as_str().unwrap(), actual) {
                ("step", Transition::PrimitiveContinuation(state)) => {
                    native::step_java_script_primitive_conversion(heap, *state)
                }
                ("resume", Transition::PrimitiveCall(state, _, _, _)) => {
                    let completion =
                        decode_java_script_primitive_conversion_completion(&step["completion"]);
                    assert_eq!(
                        encode_java_script_primitive_conversion_completion(&completion),
                        step["completion"]
                    );
                    native::resume_java_script_primitive_conversion(heap, *state, completion)
                }
                _ => panic!("every transition retains its actual operation and pending state"),
            };
            assert_eq!(
                encode_java_script_primitive_conversion_transition(&actual),
                trace[index + 1],
                "{}",
                row["name"]
            );
        }
        for entry in trace {
            assert_eq!(
                encode_java_script_primitive_conversion_transition(
                    &decode_java_script_primitive_conversion_transition(entry)
                ),
                *entry
            );
        }
        assert_eq!(
            encode_java_script_primitive_conversion_transition(&actual),
            row["expected"]
        );
    }
    for row in observations["invalid"].as_array().unwrap() {
        let heap = decode_java_script_ordinary_heap(&observations["invalidHeap"]);
        let state = decode_java_script_primitive_conversion_state(&row["state"]);
        assert_eq!(
            encode_java_script_primitive_conversion_state(&state),
            row["state"]
        );
        let actual = if row["operation"] == "resume" {
            native::resume_java_script_primitive_conversion(
                heap,
                state,
                decode_java_script_primitive_conversion_completion(&row["completion"]),
            )
        } else {
            native::step_java_script_primitive_conversion(heap, state)
        };
        assert_eq!(
            encode_java_script_primitive_conversion_transition(&actual),
            row["result"],
            "{}",
            row["name"]
        );
    }
}
