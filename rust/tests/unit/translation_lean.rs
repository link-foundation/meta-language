//! The Lean frontend: the surface it reads and the errors it reports.
//!
//! Full parity with `js/src/translation/lean.js` is checked by the stage
//! probe over `js/experiments/translation-lean-frontend-cases.mjs`; these
//! tests pin a representative slice through the public API.

use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::lean::{annotate_layout, parse_lean, span};
use meta_language::translation::lexer::{tokenize_source, Source};
use meta_language::translation::{Language, Span};
use serde_json::{json, Value};

fn surface(source: &str) -> Value {
    serde_json::to_value(parse_lean(source).expect("the Lean source parses"))
        .expect("the surface serialises")
}

fn failure(source: &str) -> (ErrorKind, String) {
    let error = parse_lean(source).expect_err("the Lean source is rejected");
    (error.kind, error.message())
}

#[test]
fn definitions_read_binders_types_and_operators() {
    let program = surface("def add (a b : Nat) (c : Int) : Int :=\n  a + b + c\n");
    let name = |text: &str, start: usize| json!({ "k": "name", "path": [text], "span": { "start": start, "end": start + 1 } });
    assert_eq!(
        program,
        json!({
            "language": "Lean",
            "items": [{
                "k": "fn",
                "name": "add",
                "params": [
                    { "name": "a", "type": { "kind": "nat" } },
                    { "name": "b", "type": { "kind": "nat" } },
                    { "name": "c", "type": { "kind": "int" } }
                ],
                "ret": { "kind": "int" },
                "body": {
                    "k": "binary",
                    "op": "add",
                    "left": {
                        "k": "binary",
                        "op": "add",
                        "left": name("a", 41),
                        "right": name("b", 45),
                        "span": { "start": 41, "end": 46 }
                    },
                    "right": name("c", 49),
                    "span": { "start": 41, "end": 50 }
                },
                "span": { "start": 0, "end": 3 }
            }],
            "main": null
        })
    );
}

#[test]
fn theorems_keep_their_steps_and_proof_source() {
    let program = surface("theorem t : 1 + 1 = 2 := by rfl\n");
    let theorem = &program["items"][0];
    assert_eq!(theorem["k"], "theorem");
    assert_eq!(theorem["prop"]["p"], "eq");
    assert_eq!(theorem["prop"]["span"], json!({ "start": 12, "end": 22 }));
    assert_eq!(
        theorem["proof"],
        json!({
            "steps": [{ "t": "compute", "tactic": "rfl" }],
            "source": "by rfl",
            "sourceLanguage": "Lean"
        })
    );
}

#[test]
fn numeral_patterns_are_decimal_text_and_successors_add() {
    let program = surface(
        "def f (n : Nat) : Nat :=\n  match n with\n  | 0 => 0\n  | k + 2 => k\n  | 12345678901234567890123 => 1\n",
    );
    let rows = &program["items"][0]["body"]["rows"];
    assert_eq!(rows[0]["patterns"][0]["k"], "numLit");
    assert_eq!(rows[0]["patterns"][0]["value"], "0");
    assert_eq!(rows[1]["patterns"][0]["k"], "natAdd");
    assert_eq!(rows[1]["patterns"][0]["add"], 2);
    assert_eq!(rows[1]["patterns"][0]["inner"]["name"], "k");
    assert_eq!(rows[2]["patterns"][0]["value"], "12345678901234567890123");
}

#[test]
fn spans_count_utf16_code_units() {
    let program = surface("def s : String := \"𝔸𝔹\"\ndef n : Nat := 1\n");
    assert_eq!(program["items"][0]["body"]["value"], "𝔸𝔹");
    assert_eq!(
        program["items"][0]["body"]["span"],
        json!({ "start": 18, "end": 24 })
    );
    assert_eq!(
        program["items"][1]["body"]["span"],
        json!({ "start": 40, "end": 41 })
    );
}

#[test]
fn errors_carry_their_kind_and_the_javascript_message() {
    let cases = [
        (
            "def f := 1\n",
            ErrorKind::Syntax,
            "expected : in def but found \":=\" at 6..8",
        ),
        (
            "def f (x : Float) : Nat := 1\n",
            ErrorKind::Unsupported,
            "Lean type Float: outside the portable core at 11..16",
        ),
        (
            "def f (n : Nat) : Nat :=\n  sorry\n",
            ErrorKind::Unsupported,
            "sorry: incomplete definitions cannot be translated at 27..32",
        ),
    ];
    for (source, kind, message) in cases {
        assert_eq!(failure(source), (kind, message.to_owned()), "{source}");
    }
}

#[test]
fn layout_marks_lines_columns_and_line_starts() {
    let source = Source::new("def f :=\n  a b\n");
    let mut tokens = tokenize_source(&source, Language::Lean)
        .expect("the source tokenizes")
        .tokens;
    annotate_layout(&mut tokens, source.units());
    let layout: Vec<_> = tokens
        .iter()
        .map(|token| (token.value.as_str(), token.line, token.col, token.first))
        .collect();
    assert_eq!(
        layout[..5],
        [
            ("def", 0, 0, true),
            ("f", 0, 4, false),
            (":=", 0, 6, false),
            ("a", 1, 2, true),
            ("b", 1, 4, false),
        ]
    );
}

#[test]
fn span_reaches_the_next_token_but_not_past_the_end() {
    let source = Source::new("a  b");
    let tokens = tokenize_source(&source, Language::Lean)
        .expect("the source tokenizes")
        .tokens;
    assert_eq!(span(&tokens[0], &tokens[1]), Span::new(0, 3));
    let end = tokens.last().expect("an end token");
    assert_eq!(span(&tokens[1], end), Span::new(3, 4));
}
