//! Deferred complete protocol parity; semantics and transports are generated.
#[path = "javascript_property_definition_codecs.rs"]
mod codecs;
#[path = "../../src/translation/javascript_property_definition.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use codecs::*;
use meta_language::read_structured_data_lino;
use native::JavaScriptPropertyDefinitionTransition as Transition;
use std::process::Command;

#[test]
fn property_definition_keeps_every_state_call_and_original_result() {
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
            "../experiments/javascript-properties/property-definition-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let observations = read_structured_data_lino(&producer.stdout).unwrap();
    for row in observations["cases"].as_array().unwrap() {
        let heap = decode_java_script_ordinary_heap(&row["heap"]);
        assert_eq!(encode_java_script_ordinary_heap(&heap), row["heap"]);
        let target = decode_java_script_semantic_value(&row["target"]);
        let key_input = decode_java_script_semantic_value(&row["keyInput"]);
        let attributes = decode_java_script_semantic_value(&row["attributes"]);
        let exotic_key = decode_java_script_semantic_value(&row["exoticKey"]);
        for (value, name) in [
            (&target, "target"),
            (&key_input, "keyInput"),
            (&attributes, "attributes"),
            (&exotic_key, "exoticKey"),
        ] {
            assert_eq!(encode_java_script_semantic_value(value), row[name]);
        }
        let mut actual = native::begin_java_script_property_definition(
            heap, target, key_input, attributes, exotic_key,
        );
        let trace = row["trace"].as_array().unwrap();
        let steps = row["steps"].as_array().unwrap();
        assert_eq!(trace.len(), steps.len() + 1);
        assert_eq!(
            encode_java_script_property_definition_transition(&actual),
            trace[0]
        );
        for (index, step) in steps.iter().enumerate() {
            let heap = decode_java_script_ordinary_heap(&step["heap"]);
            assert_eq!(encode_java_script_ordinary_heap(&heap), step["heap"]);
            actual = match (step["operation"].as_str().unwrap(), actual) {
                ("step", Transition::DefinitionContinuation(state)) => {
                    native::step_java_script_property_definition(heap, *state)
                }
                ("resume", Transition::DefinitionCall(state, _, _, _)) => {
                    let completion = decode_java_script_semantic_completion(&step["completion"]);
                    assert_eq!(
                        encode_java_script_semantic_completion(&completion),
                        step["completion"]
                    );
                    native::resume_java_script_property_definition(heap, *state, completion)
                }
                _ => panic!("every transition retains its actual operation and pending state"),
            };
            assert_eq!(
                encode_java_script_property_definition_transition(&actual),
                trace[index + 1],
                "{}",
                row["name"]
            );
        }
        for entry in trace {
            assert_eq!(
                encode_java_script_property_definition_transition(
                    &decode_java_script_property_definition_transition(entry)
                ),
                *entry
            );
        }
        assert_eq!(
            encode_java_script_property_definition_transition(&actual),
            row["expected"]
        );
    }
    for row in observations["invalid"].as_array().unwrap() {
        let heap = decode_java_script_ordinary_heap(&observations["invalidHeap"]);
        let state = decode_java_script_property_definition_state(&row["state"]);
        assert_eq!(
            encode_java_script_property_definition_state(&state),
            row["state"]
        );
        let actual = if row["operation"] == "resume" {
            native::resume_java_script_property_definition(
                heap,
                state,
                decode_java_script_semantic_completion(&row["completion"]),
            )
        } else {
            native::step_java_script_property_definition(heap, state)
        };
        assert_eq!(
            encode_java_script_property_definition_transition(&actual),
            row["result"],
            "{}",
            row["name"]
        );
    }
}
