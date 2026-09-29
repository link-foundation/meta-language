//! `console.log` in a function: Rust prints where the source prints; Lean and
//! Rocq thread the lines printed through every function that prints.

use meta_language::{translate_program, TranslationSupport};

const LOUD: &str = "function loud(x) {
  console.log('loud');
  return x * 2n;
}
console.log(loud(5n));
";

fn encodes_output_threading(source: &str, target: &str) -> (bool, String) {
    let translated =
        translate_program(source, "JavaScript", target).expect("translation descriptor");
    let threaded = translated.semantics().is_some_and(|semantics| {
        semantics
            .encodings
            .iter()
            .any(|encoding| encoding.id == "output-threading")
    });
    (threaded, translated.code().to_owned())
}

#[test]
fn a_function_that_prints_is_a_semantic_translation_in_every_target() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(LOUD, "JavaScript", target).expect("translation descriptor");
        assert!(translated.diagnostic().is_none(), "{target}");
        assert_eq!(
            translated.contract().support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
    }
}

#[test]
fn rust_prints_where_the_source_prints() {
    let translated = translate_program(LOUD, "JavaScript", "Rust").expect("translation descriptor");
    let expected = "pub fn loud(x: crate::ml::Big) -> crate::ml::Big {\n    {\n        println!(\"{}\", String::from(\"loud\"));\n        x.mul(&crate::ml::Big::from_i128(2))\n    }\n}";
    assert!(
        translated.code().contains(expected),
        "{}",
        translated.code()
    );
}

#[test]
fn lean_threads_the_lines_printed_through_a_function_that_prints() {
    let (threaded, code) = encodes_output_threading(LOUD, "Lean");
    assert!(threaded, "{code}");
    for expected in [
        "inductive ml_io1 where\n  | ml_io1_mk (output : List String) (value : Int) : ml_io1\n  deriving Inhabited",
        "def loud (x : Int) (ml_out : List String) : ml_io1 :=\n  (let ml_o1 := (\"loud\" :: ml_out);",
        ".reverse do IO.println ml_line",
    ] {
        assert!(code.contains(expected), "{expected}\n{code}");
    }
}

#[test]
fn rocq_threads_the_lines_printed_through_a_function_that_prints() {
    let (threaded, code) = encodes_output_threading(LOUD, "Rocq");
    assert!(threaded, "{code}");
    for expected in [
        "Definition loud (x : Z) (ml_out : list string) : ml_io1 :=\n  (let ml_o1 := (\"loud\"%string :: ml_out) in (ml_io1_mk ml_o1 (Z.mul x 2%Z))).",
        "app (List.rev (match ml_run2 with | ml_io2_mk ml_o7 ml_v8 => ml_o7 end))",
    ] {
        assert!(code.contains(expected), "{expected}\n{code}");
    }
}

#[test]
fn a_program_that_prints_only_in_main_threads_nothing() {
    let (threaded, code) = encodes_output_threading("console.log(1n + 2n);\n", "Lean");
    assert!(!threaded, "{code}");
    assert!(!code.contains("ml_io"), "{code}");
}

#[test]
fn an_assertion_over_a_mutually_recursive_function_runs_when_lean_main_runs() {
    let source = "import assert from 'node:assert/strict';
function isEven(n) { if (n === 0n) return true; return isOdd(n - 1n); }
function isOdd(n) { if (n === 0n) return false; return isEven(n - 1n); }
const odd = isOdd(3n);
assert(odd);
";
    let translated =
        translate_program(source, "JavaScript", "Lean").expect("translation descriptor");
    assert!(translated.diagnostic().is_none());
    assert!(
        translated
            .code()
            .contains("if !odd then throw (IO.userError \"assertion 1 failed\")"),
        "{}",
        translated.code()
    );
}

#[test]
fn an_operand_that_prints_in_a_compound_assertion_is_refused_with_a_reason() {
    let source = "import assert from 'node:assert/strict';
function loud(x) { console.log('loud'); return x; }
assert(loud(true) && true);
";
    let translated =
        translate_program(source, "JavaScript", "Lean").expect("translation descriptor");
    let diagnostic = translated.diagnostic().expect("a diagnostic");
    assert_eq!(diagnostic.kind, "unsupported");
    assert!(
        diagnostic
            .message
            .starts_with("output in a compound assertion: "),
        "{}",
        diagnostic.message
    );
}
