#![cfg(test)]

use super::read_capture;
use crate::self_translation::Context;
use crate::self_translation::emission::check_and_emit_item_with_ownership;
use crate::translation::javascript::ModuleContext;
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
    let context = Context {
        module: ModuleContext {
            externals: provider.signatures,
            ..ModuleContext::default()
        },
        ..Context::default()
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
    assert!(read_capture(&captured, &names).is_some());
    let metadata = captured
        .definition_ownership
        .as_mut()
        .expect("normal ownership emission retains actual external targets");
    assert_eq!(metadata.external_targets.len(), 1);
    assert_eq!(metadata.external_targets[0].source, "externalSeed");
    assert_eq!(metadata.external_targets[0].kind, "fn");
    assert_eq!(metadata.external_targets[0].target, "external_seed");
    metadata.external_targets[0].target.push_str("changed");
    assert!(read_capture(&captured, &names).is_none());
}
