//! Actual source-free replay through the appended shared projection roles.

use meta_language::translation::check::check_program;
use meta_language::translation::ir::Decl;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::types::Type;
use meta_language::{LinkNetwork, decode_typed_program_links, encode_typed_program_links};
use serde_json::json;

#[test]
fn source_free_method_projection_retains_the_actual_omitted_separator_slot() {
    let source = "/** @param {string} value @returns {string[]} */ function split(value){return value.split();}";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    let Decl::Fn(function) = checked.declaration("split").unwrap() else {
        panic!("the actual source function remains declared")
    };
    let value = serde_json::to_value(&function.body).unwrap();
    assert_eq!(value["k"], "method");
    assert_eq!(value["args"].as_array().unwrap().len(), 2);
    assert_eq!(value["args"][0], serde_json::Value::Null);
    assert_eq!(value["args"][1]["value"], "4294967295");
    let packet = encode_typed_program_links(&checked).unwrap();
    let restored = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    assert_eq!(
        decode_typed_program_links(&restored, packet.root).unwrap(),
        checked
    );
}

#[test]
fn optional_result_replay_retains_the_source_annotation_and_expanded_result() {
    let source = "/** @param {number[]} values @returns {number} */ function first(values){return values[0];}";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    let Decl::Fn(function) = checked.declaration("first").unwrap() else {
        panic!("the actual source function remains declared")
    };
    assert_eq!(function.annotated_ret, Some(Type::Float));
    assert_eq!(
        serde_json::to_value(&function.ret).unwrap(),
        json!({
            "kind":"option", "element":{"kind":"float"}, "absence":"undefined"
        })
    );
    assert_eq!(function.body.kind(), "optionalIndex");
    let packet = encode_typed_program_links(&checked).unwrap();
    let restored = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    assert_eq!(
        decode_typed_program_links(&restored, packet.root).unwrap(),
        checked
    );
    let mut edited = checked;
    let Decl::Fn(function) = edited
        .declarations
        .iter_mut()
        .find(|declaration| declaration.full_name() == "first")
        .unwrap()
    else {
        panic!("the actual declaration remains available for the edit")
    };
    function.annotated_ret = Some(Type::Bool);
    assert!(
        encode_typed_program_links(&edited).is_err(),
        "an incompatible retained annotation cannot pass complete rechecking"
    );
}
