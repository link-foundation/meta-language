use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::ir::Program;
use serde_json::{json, Value};

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
    assert!(emitted
        .text
        .starts_with("-- Translated from Lean by meta-language: portable core, Lean target.\n"));
    assert!(emitted
        .text
        .contains("set_option linter.unusedSimpArgs false\n\n"));
    assert!(emitted
        .text
        .contains("def monus (a : Nat) (b : Nat) : Nat :=\n  (a - b)\n"));
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
fn aborting_natural_division_uses_the_natural_divisor_check() {
    let mut value = monus();
    let body = &mut value["declarations"][0]["body"];
    body["op"] = json!("div");
    body["semantics"] = json!("exact");
    body["rounding"] = json!("trunc");
    body["byZero"] = json!("abort");
    let emitted = emit_lean(&program(value)).expect("the program emits");
    assert!(emitted.text.contains(
        "def ml_nonzero_nat (divisor : Nat) : Nat :=\n  if divisor == 0 then panic! \"division by zero\" else divisor\n\ndef monus"
    ));
    assert!(emitted.text.contains("  (a / (ml_nonzero_nat b))\n"));
    let contract = serde_json::to_value(&emitted).expect("the result serialises");
    assert_eq!(contract["assumptions"][0]["id"], "non-aborting-executions");
    assert_eq!(
        contract["assumptions"][0]["details"],
        json!(["no abort: division by zero"])
    );
}

#[test]
fn rejects_mutual_recursion_and_structured_output() {
    let mut mutual = monus();
    mutual["declarations"][0]["mutual"] = json!(true);
    let error = emit_lean(&program(mutual)).expect_err("mutual recursion is unsupported");
    assert_eq!(
        error.message(),
        "mutual recursion: monus is mutually recursive; the Lean target emits only single recursive definitions at 0..37"
    );

    let mut unit = monus();
    unit["main"]["effects"][0]["expr"]["arg"] =
        json!({ "k": "unit", "type": { "kind": "unit" }, "span": { "start": 5, "end": 7 } });
    let error = emit_lean(&program(unit)).expect_err("printing a unit value is unsupported");
    assert_eq!(
        error.message(),
        "output of structured values: a unit value has no portable textual form at 5..7"
    );
}
