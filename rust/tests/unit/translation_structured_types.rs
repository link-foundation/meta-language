use meta_language::translation::check::check_program;
use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::emit_rocq::emit_rocq;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::structured_type_rules::{
    accept_optional_absence, merge_optional_absence, read_rust_optional_prelude,
    render_rust_compound_type, render_rust_reference_type, wrap_portable_reference_identity,
};
use meta_language::translation::surface::SProgram;
use meta_language::translation::types::{Type, option};
use serde_json::Value;

fn fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../parity/fixtures/translation-structured-types.json"
    ))
    .unwrap()
}

#[test]
fn structural_type_adapters_match_every_independent_identity_fixture() {
    let data = fixture();
    for entry in data["cases"].as_array().unwrap() {
        let ty: Type = serde_json::from_value(entry["type"].clone()).unwrap();
        assert_eq!(
            ty.key(),
            entry["identity"].as_str().unwrap(),
            "{}",
            entry["name"]
        );
    }
    assert_eq!(
        option(option(Type::Bool, "null"), "undefined").key(),
        "option:both:bool"
    );
    assert_eq!(
        option(option(Type::Bool, "null"), "null").key(),
        "option:null:bool"
    );
    for expected in ["null", "undefined", "both"] {
        for actual in ["null", "undefined", "both"] {
            assert_eq!(
                accept_optional_absence(expected.to_owned(), actual.to_owned()),
                expected == "both" || expected == actual
            );
            assert_eq!(
                merge_optional_absence(expected.to_owned(), actual.to_owned()),
                if expected == actual { expected } else { "both" }
            );
        }
    }
    assert_eq!(
        wrap_portable_reference_identity("array:bool".to_owned(), true),
        "reference:array:bool"
    );
    assert_eq!(
        wrap_portable_reference_identity("array:bool".to_owned(), false),
        "array:bool"
    );
    assert_eq!(
        render_rust_compound_type("cell".to_owned(), String::new(), vec!["bool".to_owned()]),
        "std::rc::Rc<std::cell::RefCell<bool>>"
    );
    assert_eq!(
        render_rust_reference_type("Vec<bool>".to_owned(), true),
        "std::rc::Rc<std::cell::RefCell<Vec<bool>>>"
    );
    assert_eq!(
        render_rust_reference_type("Vec<bool>".to_owned(), false),
        "Vec<bool>"
    );
}

#[test]
fn data_traits_inspect_nested_compounds_and_reject_callable_fields() {
    let data = fixture();
    for entry in data["traitCases"].as_array().unwrap() {
        let surface: SProgram = serde_json::from_value(entry["surface"].clone()).unwrap();
        let checked = check_program(&surface).unwrap();
        let result = emit_rust(&checked).unwrap();
        assert_eq!(
            result.text,
            entry["rust"].as_str().unwrap(),
            "{}",
            entry["name"]
        );
        let derive = if entry["totalEquality"].as_bool().unwrap() {
            "#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Box"
        } else {
            "#[derive(Clone, Debug, PartialEq)]\npub enum Box"
        };
        assert!(result.text.contains(derive));
    }
    let surface: SProgram = serde_json::from_value(data["callableData"].clone()).unwrap();
    let checked = check_program(&surface).unwrap();
    let error = emit_rust(&checked).unwrap_err();
    assert!(error.to_string().contains("callable data field"));
}

#[test]
fn pure_targets_refuse_shared_reference_signatures_without_a_state_model() {
    let data = fixture();
    for entry in data["emissionCases"].as_array().unwrap() {
        let name = entry["name"].as_str().unwrap();
        if !name.starts_with("reference ") && !name.contains("cell") {
            continue;
        }
        let surface: SProgram = serde_json::from_value(entry["surface"].clone()).unwrap();
        let checked = check_program(&surface).unwrap();
        for error in [
            emit_lean(&checked).unwrap_err(),
            emit_rocq(&checked).unwrap_err(),
        ] {
            assert!(error.to_string().contains("mutable structural value: shared reference state needs an explicit target state model"), "{name}");
        }
    }
}

#[test]
fn compound_identity_functions_match_the_complete_shared_rust_target_source() {
    let data = fixture();
    for entry in data["emissionCases"].as_array().unwrap() {
        let surface: SProgram = serde_json::from_value(entry["surface"].clone()).unwrap();
        let checked = check_program(&surface).unwrap();
        let result = emit_rust(&checked).unwrap();
        assert_eq!(
            result.text,
            entry["rust"].as_str().unwrap(),
            "{}",
            entry["name"]
        );
    }
    assert_eq!(
        read_rust_optional_prelude(),
        data["optionalPrelude"].as_str().unwrap()
    );
}
