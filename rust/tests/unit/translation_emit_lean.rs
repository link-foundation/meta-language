use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::ir::Program;
use serde_json::{Value, json};

/// The checked IR of `def monus (a b : Nat) : Nat := a - b` with a `main`
/// that prints `monus 3 5`, as the JavaScript checker serialises it.
fn monus() -> Value {
    let nat = json!({ "kind": "nat" });
    let literal = |value: &str, start: u32| json!({ "k": "lit", "type": nat, "value": value, "span": { "start": start, "end": start + 1 } });
    let variable = |name: &str, start: u32| json!({ "k": "var", "name": name, "type": nat, "span": { "start": start, "end": start + 1 } });
    json!({
        "schemaVersion": 1,
        "sourceLanguage": "Lean",
        "items": [{ "k": "decl", "fullName": "monus" }],
        "main": {
            "effects": [{
                "k": "print",
                "expr": {
                    "k": "toString",
                    "arg": {
                        "k": "call",
                        "fn": "monus",
                        "args": [literal("3", 85), literal("5", 87)],
                        "type": nat,
                        "span": { "start": 79, "end": 84 }
                    },
                    "type": { "kind": "string" }
                },
                "span": { "start": 64, "end": 74 }
            }],
            "span": { "start": 37, "end": 40 }
        },
        "declarations": [{
            "k": "fn",
            "name": "monus",
            "params": [
                { "name": "a", "type": nat, "guard": null },
                { "name": "b", "type": nat, "guard": null }
            ],
            "ret": nat,
            "body": {
                "k": "binary",
                "op": "sub",
                "left": variable("a", 31),
                "right": variable("b", 35),
                "type": nat,
                "domain": nat,
                "semantics": "truncated",
                "span": { "start": 31, "end": 36 }
            },
            "span": { "start": 0, "end": 37 },
            "fullName": "monus",
            "modulePath": [],
            "recursive": false,
            "decreasing": null
        }]
    })
}

fn program(value: Value) -> Program {
    serde_json::from_value(value).expect("the IR literal deserialises")
}

#[test]
fn emits_a_function_and_main_with_their_contract() {
    let emitted = emit_lean(&program(monus())).expect("the program emits");
    assert!(
        emitted
            .text
            .starts_with("-- Translated from Lean by meta-language: portable core, Lean target.\n")
    );
    assert!(
        emitted
            .text
            .contains("set_option linter.constructorNameAsVariable false\n\n")
    );
    assert!(
        emitted
            .text
            .contains("def monus (a : Nat) (b : Nat) : Nat :=\n  (a - b)\n")
    );
    assert!(emitted.text.ends_with(
        "def main : IO Unit := do\n  IO.println (toString (monus (3 : Nat) (5 : Nat)))\n"
    ));
    let contract = serde_json::to_value(&emitted).expect("the result serialises");
    assert_eq!(contract["language"], "Lean");
    assert_eq!(
        contract["mappings"],
        json!([{ "kind": "function", "source": "monus", "target": "monus", "sourceSpan": { "start": 0, "end": 37 } }])
    );
    assert_eq!(contract["assumptions"], json!([]));
    assert_eq!(contract["encodings"][0]["id"], "program-output");
    assert_eq!(contract["theorems"], json!([]));
    assert_eq!(contract["entry"], "main");
}

#[test]
fn aborting_natural_division_threads_the_abort_through_the_result() {
    let mut value = monus();
    let body = &mut value["declarations"][0]["body"];
    body["op"] = json!("div");
    body["semantics"] = json!("exact");
    body["rounding"] = json!("trunc");
    body["byZero"] = json!("abort");
    let emitted = emit_lean(&program(value)).expect("the program emits");
    assert!(emitted.text.contains(
        "inductive ml_io1 where\n  | ml_io1_mk (output : List String) (value : Nat) : ml_io1\n  \
         | ml_io1_abort (output : List String) (message : String) : ml_io1\n"
    ));
    assert!(emitted.text.contains(
        "def monus (a : Nat) (b : Nat) (ml_out : List String) : ml_io1 :=\n  \
         (if (b == (0 : Nat)) then (ml_io1.ml_io1_abort ml_out \"Division by zero\") \
         else (ml_io1.ml_io1_mk ml_out (a / b)))\n"
    ));
    assert!(
        emitted
            .text
            .contains("| ml_io2.ml_io2_abort _ ml_m => throw (IO.userError ml_m))\n")
    );
    assert!(!emitted.text.contains("ml_nonzero_nat"));
    let contract = serde_json::to_value(&emitted).expect("the result serialises");
    assert_eq!(contract["assumptions"], json!([]));
    let encodings: Vec<_> = emitted.encodings.iter().map(|e| e.id.as_str()).collect();
    assert_eq!(
        encodings,
        ["program-output", "output-threading", "abort-threading"]
    );
}

#[test]
fn rejects_structured_output() {
    let mut unit = monus();
    unit["main"]["effects"][0]["expr"]["arg"] =
        json!({ "k": "unit", "type": { "kind": "unit" }, "span": { "start": 5, "end": 7 } });
    let error = emit_lean(&program(unit)).expect_err("printing a unit value is unsupported");
    assert_eq!(
        error.message(),
        "output of structured values: a unit value has no portable textual form at 5..7"
    );
}
