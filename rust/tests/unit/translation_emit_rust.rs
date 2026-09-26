//! The Rust emitter on crafted portable-core IR, including the two
//! unsupported errors that no source program reaches (the frontends and the
//! checker reject them first). The expected texts and messages are the ones
//! the JavaScript emitter (`js/src/translation/emit-rust.js`) produces on the
//! same IR; the translation-stage parity harness compares the two emitters on
//! the source-driven cases.

use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::Program;
use serde_json::{json, Value};

const HARNESS: &str = "\nfn ml_main() {\n    println!(\"{}\", (crate::q(7i32, 2i32)).to_string());\n}\n\n// Deep recursion in the source is not bounded by a small native stack.\nfn main() {\n    let check = std::env::args().any(|argument| argument == \"--ml-check-theorems\");\n    let worker = std::thread::Builder::new()\n        .stack_size(1 << 28)\n        .spawn(move || { ml_main() })\n        .expect(\"spawn the program thread\");\n    if worker.join().is_err() {\n        std::process::exit(101);\n    }\n}\n";

const HEADER: &str = "// Translated from Rust by meta-language: portable core, Rust target.\n#![allow(unused, unreachable_patterns, non_snake_case, non_camel_case_types)]\n\n";

fn fixed(signed: bool) -> Value {
    json!({ "kind": "fixed", "bits": 32, "signed": signed })
}

/// `fn q(a: i32, b: i32) -> i32 { a / b }` printed from `main` as `q(7, 2)`,
/// with the division's rounding (or none) and the integer signedness chosen.
fn division_program(rounding: Option<&str>, signed: bool) -> Value {
    let int = fixed(signed);
    let mut body = json!({
        "k": "binary", "op": "div",
        "left": { "k": "var", "name": "a", "type": int, "span": { "start": 30, "end": 31 } },
        "right": { "k": "var", "name": "b", "type": int, "span": { "start": 34, "end": 35 } },
        "type": int, "domain": int, "semantics": "checked", "byZero": "abort",
        "span": { "start": 30, "end": 35 }
    });
    if let Some(rounding) = rounding {
        body["rounding"] = json!(rounding);
    }
    json!({
        "schemaVersion": 1,
        "sourceLanguage": "Rust",
        "items": [{ "k": "decl", "fullName": "q" }],
        "main": {
            "effects": [{
                "k": "print",
                "expr": {
                    "k": "toString",
                    "arg": {
                        "k": "call", "fn": "q",
                        "args": [
                            { "k": "lit", "type": int, "value": "7", "span": { "start": 67, "end": 68 } },
                            { "k": "lit", "type": int, "value": "2", "span": { "start": 70, "end": 71 } }
                        ],
                        "type": int, "span": { "start": 65, "end": 72 }
                    },
                    "type": { "kind": "string" },
                    "span": { "start": 65, "end": 72 }
                },
                "span": { "start": 50, "end": 73 }
            }],
            "span": { "start": 38, "end": 40 }
        },
        "declarations": [{
            "k": "fn", "name": "q",
            "params": [
                { "name": "a", "type": int, "guard": null },
                { "name": "b", "type": int, "guard": null }
            ],
            "ret": int,
            "body": body,
            "span": { "start": 0, "end": 38 },
            "fullName": "q", "modulePath": [], "recursive": false, "decreasing": null
        }]
    })
}

fn program(value: Value) -> Program {
    serde_json::from_value(value).expect("crafted IR deserialises")
}

fn error_message(value: Value) -> String {
    emit_rust(&program(value))
        .expect_err("the emitter rejects the program")
        .message()
}

#[test]
fn truncating_division_emits_checked_div_with_its_contract() {
    let emitted = emit_rust(&program(division_program(Some("trunc"), true))).unwrap();
    assert_eq!(
        emitted.text,
        format!(
            "{HEADER}pub fn q(a: i32, b: i32) -> i32 {{\n    a.checked_div(b).expect(\"i32 division overflowed\")\n}}\n{HARNESS}"
        )
    );
    let mut contract = serde_json::to_value(&emitted).unwrap();
    contract.as_object_mut().unwrap().remove("text");
    assert_eq!(
        contract,
        json!({
            "language": "Rust",
            "mappings": [{ "kind": "function", "source": "q", "target": "q", "sourceSpan": { "start": 0, "end": 38 } }],
            "assumptions": [{
                "id": "non-aborting-executions",
                "statement": "the translation agrees with the source on executions that do not abort; the source aborts on machine-integer overflow, checked conversion failure, division by zero or an explicit panic, and the target computes an unspecified value there instead",
                "details": ["no abort: division by zero"]
            }],
            "encodings": [
                { "id": "machine-integer:i32", "statement": "i32 stays a Rust i32; its arithmetic is checked and panics where the source aborts" },
                { "id": "program-output", "statement": "main prints the lines the source program prints, in order, with println!" }
            ],
            "theorems": [],
            "entry": "main"
        })
    );
}

#[test]
fn euclidean_division_emits_checked_div_euclid() {
    let emitted = emit_rust(&program(division_program(Some("euclid"), true))).unwrap();
    assert_eq!(
        emitted.text,
        format!(
            "{HEADER}pub fn q(a: i32, b: i32) -> i32 {{\n    a.checked_div_euclid(b).expect(\"i32 division overflowed\")\n}}\n{HARNESS}"
        )
    );
}

#[test]
fn floor_division_on_signed_machine_integers_is_unsupported() {
    assert_eq!(
        error_message(division_program(Some("floor"), true)),
        "floor division on i32: Rust machine integers divide with truncating or Euclidean rounding only at 30..35"
    );
}

#[test]
fn division_without_a_rounding_on_signed_machine_integers_is_unsupported() {
    assert_eq!(
        error_message(division_program(None, true)),
        "undefined division on i32: Rust machine integers divide with truncating or Euclidean rounding only at 30..35"
    );
}

#[test]
fn floor_division_on_unsigned_machine_integers_is_accepted() {
    let emitted = emit_rust(&program(division_program(Some("floor"), false))).unwrap();
    let encodings: Vec<&str> = emitted
        .encodings
        .iter()
        .map(|encoding| encoding.id.as_str())
        .collect();
    assert_eq!(encodings, ["machine-integer:u32", "program-output"]);
    assert_eq!(emitted.entry.as_deref(), Some("main"));
}

#[test]
fn printing_a_data_value_is_unsupported() {
    let mut value = division_program(Some("trunc"), true);
    value["items"]
        .as_array_mut()
        .unwrap()
        .insert(0, json!({ "k": "decl", "fullName": "Unit" }));
    value["declarations"].as_array_mut().unwrap().insert(
        0,
        json!({
            "k": "data", "name": "Unit", "fullName": "Unit", "modulePath": [],
            "ctors": [{ "name": "mk", "fields": [] }], "span": { "start": 0, "end": 1 }
        }),
    );
    value["main"]["effects"][0]["expr"]["arg"] = json!({
        "k": "ctor", "data": "Unit", "ctor": "mk", "args": [],
        "type": { "kind": "data", "name": "Unit" }, "span": { "start": 3, "end": 9 }
    });
    assert_eq!(
        error_message(value),
        "output of structured values: a data value has no portable textual form at 3..9"
    );
}

#[test]
fn printing_a_unit_value_is_unsupported() {
    let mut value = division_program(Some("trunc"), true);
    value["main"]["effects"][0]["expr"]["arg"] =
        json!({ "k": "unit", "type": { "kind": "unit" }, "span": { "start": 4, "end": 6 } });
    assert_eq!(
        error_message(value),
        "output of structured values: a unit value has no portable textual form at 4..6"
    );
}
