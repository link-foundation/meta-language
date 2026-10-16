// Generated from the complete shared grammar-selection-rules.js. Do not edit.
// Translated from JavaScript by meta-language: portable core, Rust target.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons
)]

pub fn accept_grammar_selection_mode(mode: String) -> bool {
    ((mode == "any-grammar") || (mode == "all-grammar"))
}

pub fn accept_grammar_selection_mapping(mapping: String) -> bool {
    (mapping == "identity")
}

pub fn accept_grammar_selection_count(mode: String, accepted: f64, total: f64) -> bool {
    if ((total <= (0f64)) || (accepted <= (0f64))) {
        false
    } else {
        if (mode == "all-grammar") {
            (accepted == total)
        } else {
            (mode == "any-grammar")
        }
    }
}

pub fn report_grammar_selection_ambiguity(mode: String, accepted: f64) -> bool {
    ((mode == "any-grammar") && (accepted > (1f64)))
}

pub fn read_grammar_tree_equality_fields(kind: String, output: bool) -> String {
    if (output && (kind == "missing")) {
        String::from(
            "type|kind|projectionName|projectionAnonymous|start|end|trivia|field|language|attributes|lexical|literal",
        )
    } else {
        if output {
            String::from(
                "type|kind|projectionName|projectionAnonymous|start|end|trivia|field|language|attributes|lexical",
            )
        } else {
            if (kind == "node") {
                String::from("type|kind|projectionName|projectionAnonymous")
            } else {
                if (kind == "embed") {
                    String::from("type|kind|projectionName|projectionAnonymous|start|end|language")
                } else {
                    String::from("type|kind|projectionName|projectionAnonymous|start|end")
                }
            }
        }
    }
}

pub fn read_grammar_tree_recursion_plan(kind: String, output: bool) -> String {
    if (kind == "embed") {
        String::from("full-embedding")
    } else {
        if (kind == "node") {
            if output {
                String::from("full-children")
            } else {
                String::from("meaningful-children")
            }
        } else {
            String::from("leaf")
        }
    }
}

pub fn read_grammar_selection_equality_fields(alternative: bool) -> String {
    if alternative {
        String::from("language|provenance")
    } else {
        String::from("mode|mapping|start|end|candidates|provenance|ambiguous")
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
