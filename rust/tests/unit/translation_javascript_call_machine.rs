//! Deferred complete native machine traces and original source observations.
#[path = "javascript_call_machine_codecs.rs"]
mod codecs;
#[path = "../../src/translation/javascript_call_machine.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use codecs::*;
use meta_language::read_structured_data_lino;
use std::process::Command;
#[test]
fn call_machine_keeps_all_original_calls_states_values_and_heaps() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = actual
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
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
            "../experiments/javascript-properties/call-machine-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let data = read_structured_data_lino(&producer.stdout).unwrap();
    for row in data["programs"].as_array().unwrap() {
        let program = decode_java_script_call_program(&row["program"]);
        assert_eq!(encode_java_script_call_program(&program), row["program"]);
        let heap = decode_java_script_ordinary_heap(&row["heap"]);
        assert_eq!(encode_java_script_ordinary_heap(&heap), row["heap"]);
        let receiver = decode_java_script_semantic_value(&row["receiver"]);
        assert_eq!(
            encode_java_script_semantic_value(&receiver),
            row["receiver"]
        );
        let arguments = row["arguments"]
            .as_array()
            .unwrap()
            .iter()
            .map(decode_java_script_semantic_value)
            .collect::<Vec<_>>();
        for (value, wire) in arguments.iter().zip(row["arguments"].as_array().unwrap()) {
            assert_eq!(encode_java_script_semantic_value(value), *wire);
        }
        let mut actual = native::begin_java_script_call_machine(
            program.clone(),
            heap,
            native::ml::Big::parse(row["entry"].as_str().unwrap()),
            receiver,
            arguments,
        );
        let trace = row["trace"].as_array().unwrap();
        assert!(!trace.is_empty());
        assert_eq!(
            encode_java_script_call_machine_transition(&actual),
            trace[0],
            "{}",
            row["name"]
        );
        for expected in &trace[1..] {
            let native::JavaScriptCallMachineTransition::MachineContinue(machine) = actual else {
                panic!("all intermediate native states must continue exactly as observed")
            };
            actual = native::step_java_script_call_machine(program.clone(), *machine);
            assert_eq!(
                encode_java_script_call_machine_transition(&actual),
                *expected,
                "{}",
                row["name"]
            );
        }
        for expected in trace {
            assert_eq!(
                encode_java_script_call_machine_transition(
                    &decode_java_script_call_machine_transition(expected)
                ),
                *expected,
                "complete transition transport"
            );
        }
        assert!(
            !matches!(
                actual,
                native::JavaScriptCallMachineTransition::MachineContinue(_)
            ),
            "no partial trace may be treated as a terminal result"
        );
    }
    for row in data["invalid"].as_array().unwrap() {
        let program = decode_java_script_call_program(&row["program"]);
        let heap = decode_java_script_ordinary_heap(&data["invalidHeap"]);
        let actual = native::begin_java_script_call_machine(
            program,
            heap,
            native::ml::Big::parse("10"),
            native::JavaScriptSemanticValue::Undefined,
            vec![],
        );
        assert_eq!(
            encode_java_script_call_machine_transition(&actual),
            row["result"],
            "{}",
            row["name"]
        );
    }
}
