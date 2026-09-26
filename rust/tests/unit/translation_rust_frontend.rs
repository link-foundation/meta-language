//! The Rust-source frontend: accepted programs parse to the surface objects the JavaScript
//! frontend builds, and rejected programs raise the same errors with the same messages.

use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::rust::parse_rust;
use serde_json::{json, Value};

const MAIN: &str = "fn main() { println!(\"x\"); }";

/// The serialised surface program with every span removed.
fn surface(source: &str) -> Value {
    let program = parse_rust(source).unwrap_or_else(|error| panic!("{source}: {error}"));
    let mut value = serde_json::to_value(program).expect("the surface program serialises");
    strip_spans(&mut value);
    value
}

fn strip_spans(value: &mut Value) {
    match value {
        Value::Object(map) => {
            map.remove("span");
            map.values_mut().for_each(strip_spans);
        }
        Value::Array(items) => items.iter_mut().for_each(strip_spans),
        _ => {}
    }
}

/// The kind and message of the error the source raises.
fn failure(source: &str) -> (ErrorKind, String) {
    match parse_rust(source) {
        Ok(_) => panic!("{source}: expected an error"),
        Err(error) => (error.kind, error.message()),
    }
}

fn u64_type() -> Value {
    json!({ "kind": "fixed", "bits": 64, "signed": false })
}

#[test]
fn minimal_program_prints_its_literal() {
    let program = surface(MAIN);
    assert_eq!(program["language"], "Rust");
    assert_eq!(program["items"], json!([]));
    assert_eq!(
        program["main"]["effects"],
        json!([{ "k": "print", "expr": { "k": "str", "value": "x" }, "style": "rust" }])
    );
}

#[test]
fn spans_are_utf16_offsets() {
    let program =
        parse_rust("fn main() {\n  println!(\"h\u{e9}llo \u{1f389}\"); println!(\"{}\", 1);\n}\n")
            .expect("the program parses");
    let value = serde_json::to_value(program).expect("the surface program serialises");
    // The second print starts after an accented letter and a two-unit emoji: byte 39, UTF-16 unit 36.
    assert_eq!(
        value["main"]["effects"][1]["span"],
        json!({ "start": 36, "end": 53 })
    );
}

#[test]
fn enums_and_functions_parse_to_data_and_fn_items() {
    let source = format!(
        "#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Tree {{ Leaf, Node(Box<Tree>, u64, Box<Tree>) }}\n\
         fn size(t: &Tree) -> u64 {{\n  match t {{ Tree::Leaf => 0, Tree::Node(l, _, r) => 1 + size(l) + size(r) }}\n}}\n{MAIN}"
    );
    let program = surface(&source);
    let items = program["items"].as_array().expect("items");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["k"], "data");
    assert_eq!(items[0]["name"], "Tree");
    assert_eq!(
        items[0]["ctors"][1]["fields"][1]["type"],
        u64_type(),
        "Box<Tree> and u64 fields keep their order"
    );
    assert_eq!(
        items[0]["ctors"][1]["fields"][0]["type"],
        json!({ "kind": "named", "path": ["Tree"] })
    );
    assert_eq!(items[1]["k"], "fn");
    assert_eq!(
        items[1]["params"][0]["type"],
        json!({ "kind": "named", "path": ["Tree"] })
    );
    let rows = &items[1]["body"]["rows"];
    assert_eq!(
        rows[0]["patterns"][0],
        json!({ "k": "ctor", "path": ["Tree", "Leaf"], "args": [] })
    );
    assert_eq!(rows[1]["patterns"][0]["args"][1], json!({ "k": "wild" }));
}

#[test]
fn numeric_patterns_keep_their_decimal_text() {
    let source = format!(
        "fn f(n: i64) -> i64 {{\n  match n {{ -1 => 0, 007 => 7, 3i64 => 3, 123456789012345678901234 => 1, k => k }}\n}}\n{MAIN}"
    );
    let program = surface(&source);
    let rows = &program["items"][0]["body"]["rows"];
    assert_eq!(
        rows[0]["patterns"][0],
        json!({ "k": "numLit", "value": "1", "negative": true })
    );
    assert_eq!(
        rows[1]["patterns"][0],
        json!({ "k": "numLit", "value": "007" })
    );
    assert_eq!(
        rows[2]["patterns"][0],
        json!({ "k": "numLit", "value": "3" })
    );
    assert_eq!(
        rows[3]["patterns"][0],
        json!({ "k": "numLit", "value": "123456789012345678901234" })
    );
    assert_eq!(
        rows[4]["patterns"][0],
        json!({ "k": "bindOrCtor", "name": "k" })
    );
}

#[test]
fn use_groups_resolve_to_absolute_paths() {
    let source = format!(
        "mod m {{\n  pub fn f(n: u64) -> u64 {{ n }}\n  pub fn g(n: u64) -> u64 {{ n }}\n  pub mod k {{ pub fn h(n: u64) -> u64 {{ n }} }}\n}}\n\
         use crate::m::{{f, g as gg, k::{{h}}, }};\nfn z(n: u64) -> u64 {{ f(gg(h(n))) }}\n{MAIN}"
    );
    let program = surface(&source);
    let body = &program["items"][1]["body"];
    assert_eq!(body["fn"]["path"], json!(["crate", "m", "f"]));
    assert_eq!(body["args"][0]["fn"]["path"], json!(["crate", "m", "g"]));
    assert_eq!(
        body["args"][0]["args"][0]["fn"]["path"],
        json!(["crate", "m", "k", "h"])
    );
}

#[test]
fn casts_and_conversions_become_exact_casts() {
    let source =
        format!("fn f(n: u64) -> i128 {{\n  n as i128 - u64::from(3u8) as i128\n}}\n{MAIN}");
    let program = surface(&source);
    let body = &program["items"][0]["body"];
    assert_eq!(body["op"], "sub");
    assert_eq!(body["left"]["k"], "cast");
    assert_eq!(body["left"]["flavor"], "exact");
    assert_eq!(body["right"]["arg"]["to"], u64_type());
    assert_eq!(
        body["right"]["arg"]["arg"],
        json!({ "k": "num", "value": "3", "type": { "kind": "fixed", "bits": 8, "signed": false } })
    );
}

#[test]
fn format_strings_become_concatenations() {
    let program = surface("fn main() {\n  let x = 1u64;\n  println!(\"{} and {x}{{}}\", x);\n}\n");
    let effects = &program["main"]["effects"];
    assert_eq!(effects[0]["k"], "let");
    assert_eq!(effects[0]["name"], "x");
    let print = &effects[1]["expr"];
    assert_eq!(print["op"], "concat");
    assert_eq!(print["right"], json!({ "k": "str", "value": "{}" }));
    assert_eq!(
        print["left"]["left"]["left"],
        json!({ "k": "show", "arg": { "k": "name", "path": ["x"] }, "style": "rust" })
    );
}

#[test]
fn assertions_become_propositions() {
    let program = surface(
        "fn main() {\n  assert!(1 < 2 && !false);\n  assert_eq!(1, 1);\n  assert_ne!(1, 2);\n}\n",
    );
    let effects = &program["main"]["effects"];
    assert_eq!(effects[0]["k"], "assert");
    assert_eq!(effects[0]["prop"]["p"], "and");
    assert_eq!(effects[0]["prop"]["left"]["p"], "lt");
    assert_eq!(
        effects[0]["prop"]["right"],
        json!({ "p": "not", "arg": { "p": "bool", "expr": { "k": "bool", "value": false } } })
    );
    assert_eq!(
        effects[1]["prop"],
        json!({ "p": "eq", "left": { "k": "num", "value": "1" }, "right": { "k": "num", "value": "1" } })
    );
    assert_eq!(effects[2]["prop"]["p"], "ne");
}

#[test]
#[allow(clippy::literal_string_with_formatting_args)] // the sources quote Rust format strings
fn rejected_programs_raise_the_javascript_errors() {
    let cases: &[(&str, ErrorKind, &str)] = &[
        (
            "fn f(n: u64) -> u64 { n }",
            ErrorKind::Syntax,
            "a Rust program needs fn main at 25..25",
        ),
        (
            "fn f(n: u64) -> u64 { let mut s = 0; s } fn main() { println!(\"x\"); }",
            ErrorKind::Unsupported,
            "mutable binding: mutation is outside the portable core at 22..26",
        ),
        (
            "fn f(v: Vec<u64>) -> u64 { 0 } fn main() { println!(\"x\"); }",
            ErrorKind::Unsupported,
            "type Vec<…>: generic types are outside the portable core at 8..11",
        ),
        (
            "fn main() { println!(\"{:?}\", 1u64); }",
            ErrorKind::Unsupported,
            "format spec {:?}: only plain {} Display formatting is portable at 21..27",
        ),
        (
            "use std::collections::HashMap; fn main() { println!(\"x\"); }",
            ErrorKind::Unsupported,
            "use std::collections::HashMap: only items of this crate can be imported; the standard library is outside the portable core at 0..29",
        ),
        (
            "fn f(n: u64) -> u64 { match n { k if k > 2 => 1, _ => 0 } } fn main() { println!(\"x\"); }",
            ErrorKind::Unsupported,
            "match guard: guards are outside the portable pattern language at 32..34",
        ),
        (
            "fn f(n: u64) -> u64 {\n  n < 1 < 2\n}\nfn main() { println!(\"x\"); }\n",
            ErrorKind::Syntax,
            "comparison operators cannot be chained but found \"<\" at 30..31",
        ),
        (
            "fn f(n: u64) -> u64 {\n  n.clone(1)\n}\nfn main() { println!(\"x\"); }\n",
            ErrorKind::Type,
            "clone expects 0 arguments but got 1 at 25..35",
        ),
        (
            "fn main() {\n  println!(\"{} {}\", 1);\n}\n",
            ErrorKind::Syntax,
            "format string has more {} than arguments at 23..30",
        ),
        (
            "fn main() { println!(\"x\"); }\nmod m {\n  fn f(n: u64) -> u64 { n }\n",
            ErrorKind::Syntax,
            "module m is not closed at 65..65",
        ),
        (
            "use super::f;\nfn main() { println!(\"x\"); }",
            ErrorKind::Type,
            "super beyond crate root at 0..12",
        ),
        // The JavaScript frontend loops here: its visibility scan does not stop at the end of input.
        (
            "fn main() { println!(\"x\"); }\npub(crate",
            ErrorKind::Syntax,
            "expected ) in visibility but found end of input at 38..38",
        ),
    ];
    for (source, kind, message) in cases {
        assert_eq!(failure(source), (*kind, (*message).to_owned()), "{source}");
    }
}

#[test]
fn lexer_errors_surface_unchanged() {
    assert_eq!(
        failure("fn f(n: u64) -> u64 {\n  \"abc\n}\nfn main() { println!(\"x\"); }\n"),
        (
            ErrorKind::Syntax,
            "unterminated string literal at 54..60".to_owned()
        )
    );
}
