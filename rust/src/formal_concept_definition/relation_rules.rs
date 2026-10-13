// Generated from js/src/formal-relation-rules.js by
// js/scripts/generate-formal-relation-rules.mjs. Do not edit by hand.
// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons
)]

pub fn accept_formal_relation_label_identity(
    label_kind: String,
    label_definition: String,
    label_has_term: bool,
    label_has_namespace: bool,
    label_named: bool,
    label_self_point: bool,
) -> bool {
    ((((((label_kind == "Field")
        && (label_definition == "A field label names a relation between links."))
        && label_has_term)
        && !label_has_namespace)
        && label_named)
        && label_self_point)
}

pub fn accept_formal_relation_edge_identity(
    relation_kind: String,
    relation_has_namespace: bool,
    relation_has_term: bool,
    relation_has_definition: bool,
    relation_named: bool,
    relation_arity: f64,
) -> bool {
    ((((((relation_kind == "Field") && !relation_has_namespace) && !relation_has_term)
        && !relation_has_definition)
        && !relation_named)
        && (relation_arity == (3f64)))
}

pub fn accept_formal_relation_identity(
    label_admitted: bool,
    relation_admitted: bool,
    same_label_identity: bool,
) -> bool {
    ((label_admitted && relation_admitted) && same_label_identity)
}

fn ml_main() {}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || ml_main())
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}
