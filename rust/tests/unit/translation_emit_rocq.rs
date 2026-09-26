use meta_language::translation::emit_rocq::emit_rocq;
use meta_language::translation::ir::Program;

fn program(json: &str) -> Program {
    serde_json::from_str(json).unwrap()
}

const NAT: &str = r#"{"kind":"nat"}"#;
const U64: &str = r#"{"kind":"fixed","bits":64,"signed":false}"#;

#[test]
fn lean_definitions_theorems_and_output() {
    let ir = r#"{"schemaVersion":1,"sourceLanguage":"Lean",
      "items":[{"k":"decl","fullName":"double"},{"k":"decl","fullName":"double_two"}],
      "main":{"effects":[{"k":"print","expr":{"k":"toString","arg":{"k":"call","fn":"double",
        "args":[{"k":"lit","type":NAT,"value":"3"}],"type":NAT},"type":{"kind":"string"}}}]},
      "declarations":[
        {"k":"fn","name":"double","params":[{"name":"n","type":NAT,"guard":null}],"ret":NAT,
         "body":{"k":"binary","op":"add","left":{"k":"var","name":"n","type":NAT},
           "right":{"k":"var","name":"n","type":NAT},"type":NAT,"domain":NAT,"semantics":"exact"},
         "fullName":"double","modulePath":[],"recursive":false,"decreasing":null},
        {"k":"theorem","name":"double_two","binders":[],
         "prop":{"p":"eq","left":{"k":"call","fn":"double","args":[{"k":"lit","type":NAT,"value":"2"}],"type":NAT},
           "right":{"k":"lit","type":NAT,"value":"4"},"domain":NAT},
         "proof":{"plan":{"k":"close","hints":{"unfold":[],"lemmas":[],"hyps":[],"library":[],"arith":false,"compute":true}},
           "source":"by rfl","sourceLanguage":"Lean"},
         "fullName":"double_two","modulePath":[]}]}"#
        .replace("NAT", NAT);
    let emitted = emit_rocq(&program(&ir)).unwrap();
    let text = &emitted.text;
    assert!(text.starts_with(
        "(* Translated from Lean by meta-language: portable core, Rocq target. *)\n\
         From Stdlib Require Import NArith ZArith Lia Recdef String List Ascii.\n\n\
         Fixpoint ml_digits"
    ));
    assert!(text.contains("Definition double (n : N) : N :=\n  (N.add n n).\n"));
    assert!(text.contains(
        "Theorem double_two : ((double 2%N) = 4%N).\nProof.\n  cbn [double]; ml_N_norm; cbv beta iota zeta; \
         first [(vm_compute; reflexivity) | ml_close].\nQed.\n"
    ));
    assert!(text.ends_with(
        "Definition main : list string :=\n  (ml_N_to_string (double 3%N)) ::\n  nil.\n\nEval vm_compute in main.\n"
    ));
    // Helpers the program does not use are left out.
    assert!(!text.contains("ml_Z_to_string"));
    assert!(!text.contains("ml_decide"));
    assert_eq!(emitted.entry.as_deref(), Some("main"));
    assert!(emitted.assumptions.is_empty());
    let encodings: Vec<_> = emitted.encodings.iter().map(|e| e.id.as_str()).collect();
    assert_eq!(encodings, ["program-output"]);
    assert_eq!(emitted.theorems.len(), 1);
    assert_eq!(emitted.theorems[0].target, "double_two");
    assert!(emitted.theorems[0].closed_goal);
    let json = serde_json::to_value(&emitted).unwrap();
    assert_eq!(json["language"], "Rocq");
    assert_eq!(json["mappings"][0]["kind"], "function");
    assert_eq!(json["mappings"][1]["target"], "double_two");
}

#[test]
fn natural_recursion_is_a_function_with_a_measure() {
    let ir = r#"{"schemaVersion":1,"sourceLanguage":"Rust","items":[{"k":"decl","fullName":"sum_to"}],
      "main":{"effects":[{"k":"assert","prop":{"p":"eq",
        "left":{"k":"call","fn":"sum_to","args":[{"k":"lit","type":U64,"value":"3"}],"type":U64},
        "right":{"k":"lit","type":U64,"value":"6"},"domain":U64}}]},
      "declarations":[{"k":"fn","name":"sum_to","params":[{"name":"n","type":U64,"guard":null}],"ret":U64,
        "body":{"k":"match","scrutinee":{"k":"var","name":"n","type":U64},"cases":[
          {"pattern":{"k":"natZero"},"body":{"k":"lit","type":U64,"value":"0"}},
          {"pattern":{"k":"natSucc","name":"ml_p1"},"body":{"k":"binary","op":"add",
            "left":{"k":"var","name":"n","type":U64},
            "right":{"k":"call","fn":"sum_to","args":[{"k":"var","name":"ml_p1","type":U64}],"type":U64},
            "type":U64,"domain":U64,"semantics":"checked"}}],"type":U64},
        "fullName":"sum_to","modulePath":[],"recursive":true,"decreasing":0}]}"#
        .replace("U64", U64);
    let emitted = emit_rocq(&program(&ir)).unwrap();
    assert!(emitted.text.contains(
        "Function sum_to (n : N) {measure N.to_nat n} : N :=\n  \
         (if N.eqb n 0%N then 0%N else (let ml_p1 := N.pred n in (N.add n (sum_to ml_p1)))).\n\
         Proof. all: ml_obligation. Defined.\n"
    ));
    assert!(emitted
        .text
        .contains("Theorem ml_assertion_1 : ((sum_to 3%N) = 6%N).\nProof. ml_decide. Qed.\n"));
    assert!(emitted
        .text
        .contains("Ltac ml_decide := vm_compute; ml_prove."));
    assert!(emitted
        .text
        .contains("Definition main : list string :=\n  nil.\n"));
    let encodings: Vec<_> = emitted.encodings.iter().map(|e| e.id.as_str()).collect();
    assert_eq!(
        encodings,
        ["machine-integer:u64", "nat-recursion", "program-output"]
    );
    assert_eq!(emitted.assumptions.len(), 1);
    assert_eq!(emitted.assumptions[0].id, "non-aborting-executions");
    assert_eq!(
        emitted.assumptions[0].details,
        ["u64 arithmetic stays in range", "checked add does not fail"]
    );
    assert_eq!(emitted.theorems[0].target, "ml_assertion_1");
}

#[test]
fn general_recursion_is_unsupported() {
    let ir = r#"{"schemaVersion":1,"sourceLanguage":"Lean","items":[{"k":"decl","fullName":"up"}],"main":null,
      "declarations":[{"k":"fn","name":"up","params":[{"name":"n","type":NAT,"guard":null}],"ret":NAT,
        "body":{"k":"if","cond":{"k":"binary","op":"gt","left":{"k":"var","name":"n","type":NAT},
          "right":{"k":"lit","type":NAT,"value":"10"},"type":{"kind":"bool"},"domain":NAT},
          "then":{"k":"var","name":"n","type":NAT},
          "else":{"k":"call","fn":"up","args":[{"k":"binary","op":"add","left":{"k":"var","name":"n","type":NAT},
            "right":{"k":"lit","type":NAT,"value":"1"},"type":NAT,"domain":NAT,"semantics":"exact"}],"type":NAT},
          "type":NAT},
        "fullName":"up","modulePath":[],"recursive":true,"decreasing":null}]}"#
        .replace("NAT", NAT);
    let error = emit_rocq(&program(&ir)).unwrap_err();
    assert_eq!(
        error.message(),
        "general recursion: up is not structurally recursive and Rocq requires a termination argument"
    );
}
