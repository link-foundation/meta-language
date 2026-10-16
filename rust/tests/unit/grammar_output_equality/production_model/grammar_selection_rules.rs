// Generated from complete owned Rust CST declarations. Do not edit by hand.

// Generated from the complete shared grammar-selection-rules.js. Do not edit.
#![allow(
    unused,
    unreachable_patterns,
    non_snake_case,
    non_camel_case_types,
    invalid_nan_comparisons
)]

/// Execute the complete checked source algorithm.
#[must_use]
pub fn accept_grammar_selection_mode(mode: &str) -> bool {
    (mode == "any-grammar") || (mode == "all-grammar")
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn accept_grammar_selection_mapping(mapping: &str) -> bool {
    mapping == "identity"
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn accept_grammar_selection_count(mode: &str, accepted: f64, total: f64) -> bool {
    if (total <= 0f64) || (accepted <= 0f64) {
        false
    } else if mode == "all-grammar" {
        (accepted).partial_cmp(&(total)) == Some(std::cmp::Ordering::Equal)
    } else {
        mode == "any-grammar"
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn report_grammar_selection_ambiguity(mode: &str, accepted: f64) -> bool {
    (mode == "any-grammar") && (accepted > 1f64)
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_grammar_tree_equality_fields(kind: &str, output: bool) -> String {
    if output && (kind == "missing") {
        String::from(
            "type|kind|projectionName|projectionAnonymous|start|end|trivia|field|language|attributes|lexical|literal",
        )
    } else if output {
        String::from(
            "type|kind|projectionName|projectionAnonymous|start|end|trivia|field|language|attributes|lexical",
        )
    } else if kind == "node" {
        String::from("type|kind|projectionName|projectionAnonymous")
    } else if kind == "embed" {
        String::from("type|kind|projectionName|projectionAnonymous|start|end|language")
    } else {
        String::from("type|kind|projectionName|projectionAnonymous|start|end")
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_grammar_tree_recursion_plan(kind: &str, output: bool) -> String {
    if kind == "embed" {
        String::from("full-embedding")
    } else if kind == "node" {
        if output {
            String::from("full-children")
        } else {
            String::from("meaningful-children")
        }
    } else {
        String::from("leaf")
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_grammar_selection_equality_fields(alternative: bool) -> String {
    if alternative {
        String::from("language|provenance")
    } else {
        String::from("mode|mapping|start|end|candidates|provenance|ambiguous")
    }
}

/// Execute the complete checked source algorithm.
#[must_use]
pub fn read_grammar_source_default_matching(source_format: &str) -> String {
    if source_format == "peg" {
        String::from("peg")
    } else {
        String::from("generalized")
    }
}

// Bind every copied declaration without relaxing native lints.
const _: fn() = || {
    let _ = self::accept_grammar_selection_mode;
    let _ = self::accept_grammar_selection_mapping;
    let _ = self::accept_grammar_selection_count;
    let _ = self::report_grammar_selection_ambiguity;
    let _ = self::read_grammar_tree_equality_fields;
    let _ = self::read_grammar_tree_recursion_plan;
    let _ = self::read_grammar_selection_equality_fields;
    let _ = self::read_grammar_source_default_matching;
};
