//! Integration tests for the WebAssembly demo crate.
//!
//! Kept under `tests/` (not `src/`) to match the repository convention that
//! source trees stay free of test modules.

use meta_language_web::parse_links_notation;

#[test]
fn parses_a_simple_link() {
    let output = parse_links_notation("(1: 1 1)");
    assert!(output.contains("\"ok\":true"), "output: {output}");
    assert!(output.contains("\"kind\":\"link\""));
}

#[test]
fn reports_parse_errors_without_panicking() {
    let output = parse_links_notation("(((");
    assert!(output.contains("\"ok\":false"), "output: {output}");
}

#[test]
fn formats_round_trip_for_multiple_statements() {
    let output = parse_links_notation("(1: 1 1)\n(2: 2 2)");
    assert!(output.contains("\"count\":2"), "output: {output}");
}
