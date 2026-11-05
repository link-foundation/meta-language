//! Deferred execution of real frontend output by the generated native machine.
#[path = "javascript_call_machine_codecs.rs"]
mod codecs;
#[path = "../../src/translation/javascript_call_machine.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use codecs::*;
use meta_language::read_structured_data_lino;
use std::process::Command;
#[test]
fn actual_source_compiler_output_preserves_every_native_execution_state() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let pieces = actual
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        pieces[0] == 1 && (90..100).contains(&pieces[1]),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-properties/call-source-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let data = read_structured_data_lino(&producer.stdout).unwrap();
    for row in data["rows"].as_array().unwrap() {
        let program = decode_java_script_call_program(&row["program"]);
        assert_eq!(encode_java_script_call_program(&program), row["program"]);
        let heap = decode_java_script_ordinary_heap(&row["heap"]);
        let arguments = row["arguments"]
            .as_array()
            .unwrap()
            .iter()
            .map(decode_java_script_semantic_value)
            .collect::<Vec<_>>();
        let mut actual = native::begin_java_script_call_machine(
            program.clone(),
            heap,
            native::ml::Big::parse(row["entry"].as_str().unwrap()),
            row.get("receiver")
                .map(decode_java_script_semantic_value)
                .unwrap_or(native::JavaScriptSemanticValue::Undefined),
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
                panic!("every intermediate source execution state must continue")
            };
            actual = native::step_java_script_call_machine(program.clone(), *machine);
            assert_eq!(
                encode_java_script_call_machine_transition(&actual),
                *expected,
                "{}",
                row["name"]
            );
        }
        if row["expectedOutcome"] == "allocation" {
            assert!(
                matches!(
                    actual,
                    native::JavaScriptCallMachineTransition::MachineAllocateError(_, _, _, _)
                ),
                "{}",
                row["name"]
            );
        } else if row["expectedOutcome"] == "sourceAllocation" {
            assert!(
                matches!(
                    actual,
                    native::JavaScriptCallMachineTransition::MachineAllocateSourceError(
                        _,
                        _,
                        _,
                        _,
                        _
                    )
                ),
                "{}",
                row["name"]
            );
        } else if row["expected"]["kind"] == "throw" {
            assert!(
                matches!(
                    actual,
                    native::JavaScriptCallMachineTransition::MachineThrow(_, _)
                ),
                "{}",
                row["name"]
            );
        } else {
            assert!(
                matches!(
                    actual,
                    native::JavaScriptCallMachineTransition::MachineCompleted(_, _)
                ),
                "{}",
                row["name"]
            );
        }
    }
}
