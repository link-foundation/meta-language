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
        if row["expected"].get("convertedValue").is_some() {
            let native::JavaScriptCallMachineTransition::MachineResourceLimit(machine, _) = &actual
            else {
                panic!("a converted work refusal retains its exact resumable machine")
            };
            let resumed =
                native::step_java_script_call_machine(program.clone(), (**machine).clone());
            assert_eq!(
                encode_java_script_call_machine_transition(&resumed),
                *trace.last().unwrap(),
                "quota retry must not replay any completed native conversion callback"
            );
        }
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
    for row in data["invalidStates"].as_array().unwrap() {
        let program = decode_java_script_call_program(&row["program"]);
        let machine = decode_java_script_call_machine(&row["machine"]);
        assert_eq!(encode_java_script_call_program(&program), row["program"]);
        assert_eq!(encode_java_script_call_machine(&machine), row["machine"]);
        assert_eq!(
            encode_java_script_call_machine(&machine)["heap"],
            row["heap"]
        );
        let actual = native::step_java_script_call_machine(program, machine);
        assert_eq!(
            encode_java_script_call_machine_transition(&actual),
            row["result"],
            "{}",
            row["name"]
        );
    }
    for row in data["errorResumptionRefusals"].as_array().unwrap() {
        let program = decode_java_script_call_program(&row["program"]);
        let machine = decode_java_script_call_machine(&row["machine"]);
        let value = decode_java_script_semantic_value(&row["value"]);
        assert_eq!(encode_java_script_call_program(&program), row["program"]);
        assert_eq!(encode_java_script_call_machine(&machine), row["machine"]);
        assert_eq!(encode_java_script_semantic_value(&value), row["value"]);
        let actual = native::resume_java_script_call_source_error_allocation(
            program,
            machine,
            native::ml::Big::parse(row["realm"].as_str().unwrap()),
            value,
        );
        assert_eq!(
            encode_java_script_call_machine_transition(&actual),
            row["result"],
            "{}",
            row["name"]
        );
    }
}

// Reuse this generated namespace across actual source-module consumers rather
// than compiling another complete generated machine per consumer module.
pub(super) fn assert_source_machine_trace(program_wire:&serde_json::Value,trace:&serde_json::Value){
    use serde_json::Value;
    let mut program=decode_java_script_call_program(program_wire);
    assert_eq!(encode_java_script_call_program(&program),*program_wire);
    let mut actual=decode_java_script_call_machine_transition(&trace["initial"]);
    let initial=encode_java_script_call_machine_transition(&actual);
    assert_eq!(initial,trace["initial"]);assert_eq!(initial["$"],"machineContinue");
    let machine=&initial["machine"];
    let frames=machine["frames"].as_array().unwrap();assert_eq!(frames.len(),1);
    let frame=&frames[0];assert_eq!(frame["position"],0);
    let heap=decode_java_script_ordinary_heap(&machine["heap"]);
    let identity=native::ml::Big::parse(frame["identity"].as_str().unwrap());
    let receiver=decode_java_script_semantic_value(&frame["registers"][0]);
    let arguments=frame["arguments"].as_array().unwrap().iter().map(decode_java_script_semantic_value).collect();
    let begun=native::begin_java_script_call_machine(program.clone(),heap,identity,receiver,arguments);
    assert_eq!(encode_java_script_call_machine_transition(&begun),initial);
    let events=trace["events"].as_array().unwrap();assert!(!events.is_empty()&&events.len()<=4096);
    let budget=trace["budget"].as_f64().unwrap();assert!(budget>=0.0&&budget<=1048576.0&&budget.fract()==0.0);let mut used=0.0;
    for event in events{
        let before=encode_java_script_call_machine_transition(&actual);
        let machine=decode_java_script_call_machine(&before["machine"]);
        actual=match event["operation"].as_str().unwrap(){
            "run"=>{
                assert_eq!(before["$"],"machineContinue");
                let limit=event["steps"].as_f64().unwrap();assert!(limit>0.0&&limit<=4096.0&&limit.fract()==0.0);
                let execution=native::run_java_script_call_machine(program.clone(),machine,limit,Vec::new());
                let wire=encode_java_script_call_execution(&execution);assert_eq!(wire,event["expected"]);used+=wire["steps"].as_f64().unwrap();
                decode_java_script_call_machine_transition(&wire["transition"])
            },
            "resumeBatch"=>{
                assert_eq!(before["$"],"machineResourceLimit");assert_eq!(before["reason"],"execution exhausted its explicit transition budget");assert!(used<budget);
                decode_java_script_call_machine_transition(&serde_json::json!({"$":"machineContinue","machine":before["machine"]}))
            },
            "allocateObject"=>{
                used+=1.0;assert_eq!(before["$"],"machineAllocateObject");
                native::resume_java_script_call_object_allocation(program.clone(),machine,native::ml::Big::parse(event["identity"].as_str().unwrap()))
            },
            "functionNameSymbolDescription"=>{
                used+=1.0;assert_eq!(before["$"],"machineNeedFunctionNameSymbolDescription");assert_eq!(before["object"],event["object"]);assert_eq!(before["key"],event["key"]);assert_eq!(before["prefix"],event["prefix"]);
                let description=decode_java_script_semantic_value(&event["description"]);
                native::resume_java_script_call_function_name_symbol_description(program.clone(),machine,description)
            },
            "primitiveIteratorPrototype"=>{
                used+=1.0;assert_eq!(before["$"],"machineNeedPrimitiveIteratorGet");assert_eq!(before["receiver"],event["receiver"]);assert_eq!(before["key"],event["key"]);
                let prototype=decode_java_script_semantic_value(&event["prototype"]);
                native::resume_java_script_call_primitive_iterator_prototype(program.clone(),machine,prototype)
            },
            "extendConstructorAuthority"=>{
                used+=1.0;assert_eq!(before["$"],"machineNeedConstructorAuthority");assert_eq!(before["constructor"],event["constructor"]);
                let directory=decode_java_script_array_species_directory(&event["directory"]);
                let updated=native::extend_java_script_call_array_authority(program.clone(),machine,directory);
                let wire=encode_java_script_array_authority_update(&updated);
                assert_eq!(wire["$"],"arrayAuthorityUpdate");assert_eq!(wire["error"],"");
                assert_eq!(wire["machine"],before["machine"]);assert_eq!(wire["program"],event["program"]);
                assert_ne!(wire["program"],encode_java_script_call_program(&program));
                program=decode_java_script_call_program(&wire["program"]);
                decode_java_script_call_machine_transition(&serde_json::json!({"$":"machineContinue","machine":wire["machine"]}))
            },
            "external"=>{
                used+=1.0;assert_eq!(before["$"],event["requestKind"]);assert_eq!(before["realm"],event["realm"]);
                let mut resumed=before["machine"].clone();resumed["heap"]=event["response"]["heap"].clone();
                let machine=decode_java_script_call_machine(&resumed);
                let realm=native::ml::Big::parse(event["realm"].as_str().unwrap());
                let value=decode_java_script_semantic_value(&event["response"]["value"]);
                let abrupt=event["response"]["abrupt"].as_bool().unwrap();
                match event["requestKind"].as_str().unwrap(){
                    "machineInvokeHostCapability"|"machineInvokeErrorIntrinsic"=>{
                        assert_eq!(before["callee"]["identity"],event["callee"]);
                        let identity=native::ml::Big::parse(event["callee"].as_str().unwrap());
                        if event["requestKind"]=="machineInvokeHostCapability"{native::resume_java_script_host_capability(program.clone(),machine,realm,identity,value,abrupt)}
                        else{native::resume_java_script_error_intrinsic(program.clone(),machine,realm,identity,value,abrupt)}
                    },
                    "machineAllocateErrorValue"|"machineAllocateSourceError"|"machineAllocateError"=>{
                        assert_eq!(event["callee"],Value::Null);
                        if abrupt{native::resume_java_script_error_allocation_abrupt(program.clone(),machine,realm,value)}
                        else if event["requestKind"]=="machineAllocateErrorValue"{native::resume_java_script_error_value_allocation(program.clone(),machine,realm,value)}
                        else if event["requestKind"]=="machineAllocateSourceError"{native::resume_java_script_call_source_error_allocation(program.clone(),machine,realm,value)}
                        else{native::resume_java_script_call_error_allocation(program.clone(),machine,realm,value)}
                    },
                    other=>panic!("unrepresented external source event {other}"),
                }
            },
            other=>panic!("unrepresented source event {other}"),
        };
        if event["operation"]!="run"{assert_eq!(encode_java_script_call_machine_transition(&actual),event["expected"]);}
    }
    assert!(used<=budget);assert_eq!(trace["used"].as_f64().unwrap(),used);
    assert_eq!(trace["budgetExhausted"],false,"source originals must complete rather than exhaust their resource budget");
    let final_state=encode_java_script_call_machine_transition(&actual);
    assert_eq!(final_state,trace["final"]);
    assert!(["machineCompleted","machineThrow"].contains(&final_state["$"].as_str().unwrap()));
}
