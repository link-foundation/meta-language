//! The JavaScript emitter on small checked programs, including the crafted
//! programs the checker never produces (its unreachable failure paths).

use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::ir::Program;
use serde_json::{json, Value};

fn nat() -> Value {
    json!({ "kind": "nat" })
}

fn var(name: &str, ty: &Value) -> Value {
    json!({ "k": "var", "name": name, "type": ty })
}

fn function(name: &str, params: &[(&str, Value)], ret: &Value, body: &Value) -> Value {
    let params: Vec<Value> = params
        .iter()
        .map(|(param, ty)| json!({ "name": param, "type": ty, "guard": null }))
        .collect();
    json!({
        "k": "fn",
        "name": name,
        "params": params,
        "ret": ret,
        "body": body,
        "fullName": name,
        "modulePath": [],
        "recursive": false,
        "decreasing": null,
    })
}

fn program(declarations: &[Value], main: &Value) -> Program {
    let items: Vec<Value> = declarations
        .iter()
        .map(|entry| json!({ "k": "decl", "fullName": entry["fullName"] }))
        .collect();
    serde_json::from_value(json!({
        "schemaVersion": 1,
        "sourceLanguage": "Lean",
        "items": items,
        "main": main,
        "declarations": declarations,
    }))
    .expect("a well-formed checked program")
}

fn print_main(expr: &Value) -> Value {
    json!({ "effects": [{ "k": "print", "expr": expr }] })
}

#[test]
fn emits_natural_subtraction_with_its_helper_and_contract() {
    let body = json!({
        "k": "binary",
        "op": "sub",
        "left": var("a", &nat()),
        "right": var("b", &nat()),
        "semantics": "truncated",
        "type": nat(),
    });
    let monus = function("monus", &[("a", nat()), ("b", nat())], &nat(), &body);
    let call = json!({
        "k": "toString",
        "arg": {
            "k": "call",
            "fn": "monus",
            "args": [
                { "k": "lit", "value": "3", "type": nat() },
                { "k": "lit", "value": "5", "type": nat() },
            ],
            "type": nat(),
        },
        "type": { "kind": "string" },
    });
    let emitted = emit_javascript(&program(&[monus], &print_main(&call))).expect("emits");
    assert_eq!(
        emitted.text,
        "// Translated from Lean by meta-language: portable core, JavaScript target.\n\
         'use strict';\n\
         \n\
         function ml_natSub(a, b) {\n  return a > b ? a - b : 0n;\n}\n\
         \n\
         function monus(a, b) {\n  return ml_natSub(a, b);\n}\n\
         \n\
         function main() {\n  console.log(String(monus(3n, 5n)));\n}\n\
         \n\
         main();\n"
    );
    assert_eq!(emitted.entry.as_deref(), Some("main"));
    let encodings: Vec<&str> = emitted
        .encodings
        .iter()
        .map(|encoding| encoding.id.as_str())
        .collect();
    assert_eq!(encodings, ["program-output", "numbers"]);
    assert_eq!(emitted.mappings.len(), 1);
    assert_eq!(emitted.mappings[0].target, "monus");
}

#[test]
fn range_checks_machine_integer_parameters_and_results() {
    let byte = json!({ "kind": "fixed", "bits": 8, "signed": false });
    let body = json!({
        "k": "binary",
        "op": "add",
        "left": var("a", &byte),
        "right": var("new", &byte),
        "semantics": "checked",
        "type": byte,
    });
    let add = function(
        "add",
        &[("a", byte.clone()), ("new", byte.clone())],
        &byte,
        &body,
    );
    let emitted = emit_javascript(&program(&[add], &Value::Null)).expect("emits");
    assert!(emitted.text.contains(
        "function add(a, new_) {\n  ml_fixed(a, 0n, 255n, 'u8 argument a');\n  \
         ml_fixed(new_, 0n, 255n, 'u8 argument new_');\n  \
         return ml_fixed((a + new_), 0n, 255n, 'u8 addition');\n}"
    ));
    assert!(emitted
        .text
        .contains("function ml_fixed(value, min, max, what) {"));
    assert!(!emitted.text.contains("main();"));
    assert_eq!(emitted.entry, None);
}

#[test]
fn refuses_to_print_a_structured_value() {
    let unit = json!({
        "k": "toString",
        "arg": { "k": "unit", "type": { "kind": "unit" }, "span": { "start": 4, "end": 6 } },
        "type": { "kind": "string" },
    });
    let error = emit_javascript(&program(&[], &print_main(&unit))).expect_err("unsupported");
    assert_eq!(
        error.message(),
        "output of structured values: a unit value has no portable textual form at 4..6"
    );
}

#[test]
fn reports_operators_and_domains_without_a_javascript_form() {
    let plus = json!({
        "k": "binary",
        "op": "plus",
        "left": { "k": "lit", "value": "1", "type": nat() },
        "right": { "k": "lit", "value": "2", "type": nat() },
        "type": nat(),
    });
    let error = emit_javascript(&program(&[], &print_main(&plus))).expect_err("no operator");
    assert_eq!(error.message(), "no JavaScript operator plus");
}

#[test]
fn binds_a_computed_match_subject_to_a_temporary() {
    let scrutinee = json!({
        "k": "binary",
        "op": "add",
        "left": var("n", &nat()),
        "right": { "k": "lit", "value": "1", "type": nat() },
        "semantics": "exact",
        "type": nat(),
    });
    let body = json!({
        "k": "match",
        "scrutinee": scrutinee,
        "cases": [
            { "pattern": { "k": "natZero" }, "body": { "k": "lit", "value": "1", "type": nat() } },
            { "pattern": { "k": "bind", "name": "m" }, "body": var("m", &nat()) },
        ],
        "type": nat(),
    });
    let pick = function("pick", &[("n", nat())], &nat(), &body);
    let emitted = emit_javascript(&program(&[pick], &Value::Null)).expect("emits");
    assert!(
        emitted.text.contains("const ml_subject1 = (n + 1n);"),
        "{}",
        emitted.text
    );
}

fn theorem(binder: &Value) -> Value {
    json!({
        "k": "theorem",
        "name": "t",
        "binders": [{ "name": "x", "type": binder }],
        "prop": {
            "p": "eq",
            "left": var("x", binder),
            "right": var("x", binder),
            "domain": binder,
        },
        "proof": {
            "plan": { "k": "close", "hints": {
                "unfold": [], "lemmas": [], "hyps": [], "library": [], "arith": false, "compute": true,
            } },
            "source": "by rfl",
            "sourceLanguage": "Lean",
        },
        "fullName": "t",
        "modulePath": [],
    })
}

#[test]
fn checks_theorems_on_bounded_domains() {
    let byte = json!({ "kind": "fixed", "bits": 8, "signed": true });
    let emitted = emit_javascript(&program(&[theorem(&byte)], &Value::Null)).expect("emits");
    assert!(emitted
        .text
        .contains("function t(x) {\n  return (x === x);\n}"));
    assert!(emitted.text.contains(
        "  if (!(ml_product([ml_small_int]).every((args) => t(...args)))) \
         throw new Error('theorem t fails on a bounded input');"
    ));
    assert!(emitted.text.ends_with(
        "if (process.argv.includes('--ml-check-theorems')) ml_checkTheorems();\nelse main();\n"
    ));
    assert_eq!(emitted.theorems.len(), 1);
    assert!(!emitted.theorems[0].closed_goal);
    assert_eq!(emitted.theorems[0].check.as_deref(), Some("bounded"));

    let unit = json!({ "kind": "unit" });
    let emitted = emit_javascript(&program(&[theorem(&unit)], &Value::Null)).expect("emits");
    assert!(
        emitted.text.contains("ml_product([[null]])"),
        "{}",
        emitted.text
    );

    let literal = json!({ "kind": "literal" });
    let error =
        emit_javascript(&program(&[theorem(&literal)], &Value::Null)).expect_err("no domain");
    assert_eq!(error.message(), "no JavaScript domain for literal");
}
