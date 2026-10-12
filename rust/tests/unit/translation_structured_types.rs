use meta_language::translation::check::check_program;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::structured_type_rules::{
    accept_optional_absence, merge_optional_absence, read_rust_optional_prelude,
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
