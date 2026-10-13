// Generated from js/src/concept-view-rules.js by
// js/scripts/generate-concept-view-rules.mjs. Do not edit by hand.
// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons
)]

pub fn accept_concept_view_shape(role: String, arity: String, defined: bool) -> bool {
    if (role == "concept view graph") {
        ((arity == "5") && defined)
    } else {
        if (role == "concept view sequence") {
            !defined
        } else {
            if (role == "concept view concept") {
                ((arity == "2") && defined)
            } else {
                if (role == "concept view relation") {
                    ((arity == "3") && defined)
                } else {
                    if ((role == "ontology view") || (role == "taxonomy view")) {
                        ((arity == "3") && !defined)
                    } else {
                        ((((((((role == "concept view archive")
                            || (role == "concept view base"))
                            || (role == "concept view definition"))
                            || (role == "concept view provenance"))
                            || (role == "concept view root"))
                            || (role == "concept view root type declaration"))
                            && (arity == "2"))
                            && !defined)
                    }
                }
            }
        }
    }
}

pub fn accept_concept_view_relation(view: String, kind: String) -> bool {
    if (view == "ontology view") {
        ((kind == "association") || (kind == "axiom"))
    } else {
        if (view == "taxonomy view") {
            ((kind == "classification") || (kind == "subsumption"))
        } else {
            false
        }
    }
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
