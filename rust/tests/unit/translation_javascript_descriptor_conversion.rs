//! Deferred complete protocol parity. Semantics and model codecs are generated.
#[path = "javascript_descriptor_conversion_codecs.rs"]
mod codecs;
#[path = "../../src/translation/javascript_descriptor_conversion.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use codecs::*;
use meta_language::read_structured_data_lino;
use native::{
    JavaScriptDescriptorConversionState as State,
    JavaScriptDescriptorConversionTransition as Transition,
};
use std::process::Command;

#[test]
fn descriptor_conversion_keeps_every_state_call_completion_and_original_oracle() {
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
            "../experiments/javascript-properties/descriptor-conversion-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let observations = read_structured_data_lino(&producer.stdout).unwrap();
    for row in observations["cases"].as_array().unwrap() {
        let heap = decode_java_script_ordinary_heap(&row["heap"]);
        assert_eq!(
            encode_java_script_ordinary_heap(&heap),
            row["heap"],
            "complete input transport"
        );
        let receiver = decode_java_script_semantic_value(&row["receiver"]);
        assert_eq!(
            encode_java_script_semantic_value(&receiver),
            row["receiver"]
        );
        let mut actual = native::begin_java_script_descriptor_conversion(heap, receiver);
        let trace = row["trace"].as_array().unwrap();
        let steps = row["steps"].as_array().unwrap();
        assert_eq!(trace.len(), steps.len() + 1);
        assert_eq!(
            encode_java_script_descriptor_conversion_transition(&actual),
            trace[0],
            "{}",
            row["name"]
        );
        for (index, step) in steps.iter().enumerate() {
            let Transition::DescriptorCall(state, _, _, _) = actual else {
                panic!("a recorded actual call must remain pending")
            };
            let heap = decode_java_script_ordinary_heap(&step["heap"]);
            assert_eq!(encode_java_script_ordinary_heap(&heap), step["heap"]);
            let completion = decode_java_script_semantic_completion(&step["completion"]);
            assert_eq!(
                encode_java_script_semantic_completion(&completion),
                step["completion"]
            );
            actual = native::resume_java_script_descriptor_conversion(heap, *state, completion);
            assert_eq!(
                encode_java_script_descriptor_conversion_transition(&actual),
                trace[index + 1],
                "complete call continuation {}",
                row["name"]
            );
        }
        for entry in trace {
            assert_eq!(
                encode_java_script_descriptor_conversion_transition(
                    &decode_java_script_descriptor_conversion_transition(entry)
                ),
                *entry,
                "all transition field transports"
            );
        }
        assert_eq!(
            encode_java_script_descriptor_conversion_transition(&actual),
            row["expected"],
            "independent literal result {}",
            row["name"]
        );
    }
    for row in observations["booleans"].as_array().unwrap() {
        let value = decode_java_script_semantic_value(&row["value"]);
        assert_eq!(encode_java_script_semantic_value(&value), row["value"]);
        assert_eq!(
            native::convert_java_script_semantic_boolean(value),
            row["actual"].as_bool().unwrap(),
            "{}",
            row["name"]
        );
    }
    for row in observations["invalid"].as_array().unwrap() {
        let mut state = decode_java_script_descriptor_conversion_state(&row["state"]);
        assert_eq!(
            encode_java_script_descriptor_conversion_state(&state),
            row["state"]
        );
        if let Some(bits) = row["nextBits"].as_str() {
            let bits = u64::from_str_radix(bits, 16).unwrap();
            let State::DescriptorConversion(_, next, _, _) = &mut state;
            *next = f64::from_bits(bits);
            assert_eq!(next.to_bits(), bits);
        }
        let heap = decode_java_script_ordinary_heap(&observations["invalidHeap"]);
        let actual = if row["operation"] == "resume" {
            native::resume_java_script_descriptor_conversion(
                heap,
                state,
                decode_java_script_semantic_completion(&row["completion"]),
            )
        } else {
            native::step_java_script_descriptor_conversion(heap, state)
        };
        assert_eq!(
            encode_java_script_descriptor_conversion_transition(&actual),
            row["result"],
            "{}",
            row["name"]
        );
    }
}
