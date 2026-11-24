//! Original inert-binding inputs and every name/owner assertion.
use meta_language::read_inert_module_binding as read_inert_binding;

#[test]
fn shared_numeric_constant_sources_retain_every_declared_name_and_owner() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../src/data/numeric-module-bindings.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 6);
    for case in cases {
        let source = case["source"].as_str().unwrap();
        let name = case["name"].as_str().unwrap().to_owned();
        assert_eq!(read_inert_binding(source), Some((name, 0)), "{source}");
    }
}

#[test]
fn shared_empty_collection_sources_retain_every_declared_name_and_owner() {
    let fixture = meta_language::read_structured_data_lino(include_str!(
        "../../src/data/empty-module-collections.lino"
    ))
    .unwrap();
    let cases = fixture["cases"].as_array().unwrap();
    assert_eq!(cases.len(), 6);
    for case in cases {
        let source = case["source"].as_str().unwrap();
        let name = case["name"].as_str().unwrap().to_owned();
        let owner = u8::try_from(case["owner"].as_u64().unwrap()).unwrap();
        assert_eq!(read_inert_binding(source), Some((name, owner)), "{source}");
    }
}

use meta_language::translation::javascript::ModuleContext;
use meta_language::{
    DeclarationTranslationContext as Context, accept_declaration_owned_emission,
    check_and_emit_declaration_with_ownership as check_and_emit_item_with_ownership,
};
use std::collections::HashMap;

#[test]
fn actual_external_target_metadata_rejects_target_only_mutation() {
    let provider = check_and_emit_item_with_ownership(
        "/** @returns {number} */ export function externalSeed(){return 0;}",
        "JavaScript",
        "Rust",
        &Context::default(),
        &HashMap::new(),
    )
    .expect("the complete external provider checks and emits")
    .expect("the provider is a real checked declaration");
    let mut context = Context::default();
    context.module = ModuleContext {
        externals: provider.signatures,
        ..ModuleContext::default()
    };
    let mut captured = check_and_emit_item_with_ownership(
        "function first(value){return value+externalSeed();}",
        "JavaScript",
        "Rust",
        &context,
        &HashMap::new(),
    )
    .expect("the complete caller checks and emits")
    .expect("the caller is a real checked declaration");
    let names = vec!["first".to_owned()];
    assert!(accept_declaration_owned_emission(&captured, &names));
    let metadata = captured
        .definition_ownership
        .as_mut()
        .expect("normal ownership emission retains actual external targets");
    assert_eq!(metadata.external_targets.len(), 1);
    assert_eq!(metadata.external_targets[0].source, "externalSeed");
    assert_eq!(metadata.external_targets[0].kind, "fn");
    assert_eq!(metadata.external_targets[0].target, "external_seed");
    metadata.external_targets[0].target.push_str("changed");
    assert!(!accept_declaration_owned_emission(&captured, &names));
}
