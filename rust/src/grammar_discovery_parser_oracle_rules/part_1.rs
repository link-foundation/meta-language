// Generated from complete js/src/grammar-discovery-parser-oracle-rules.js. Do not edit by hand.
/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_discovery_parser_oracle_limit(resource: &str) -> f64 {
    if resource == "queries" {
        256f64
    } else if (resource == "symbols") || (resource == "nesting") {
        128f64
    } else if resource == "steps" {
        50_000f64
    } else if (resource == "memo") || (resource == "source bytes") {
        4096f64
    } else if resource == "memory" {
        8192f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_discovery_parser_oracle_answer(accepted: bool, rejection: &str) -> bool {
    accepted || (rejection == "syntax")
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_discovery_parser_oracle_label(expected: f64, accepted: bool) -> bool {
    (expected == -1f64) || (expected == (if accepted { 1f64 } else { 0f64 }))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_discovery_parser_oracle_training_class(declared: bool, accepted: bool) -> f64 {
    if declared {
        0f64
    } else if accepted {
        1f64
    } else {
        -1f64
    }
}
