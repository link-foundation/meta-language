//! JavaScript functions that return nothing: falling off the end or `return;`
//! returns undefined, the unit value, and a statement that discards a call's
//! value still runs it.

use meta_language::translation::javascript::parse_javascript;
use meta_language::{translate_program, TranslationSupport};

const VOID: &str = "function hello(name) {
  console.log(`hello ${name}`);
}
function report(x) {
  if (x < 0n) return;
  console.log(`${x}`);
}
hello('a');
report(-1n);
";

fn code(source: &str, target: &str) -> String {
    translate_program(source, "JavaScript", target)
        .expect("translation descriptor")
        .code()
        .to_owned()
}

#[test]
fn a_function_that_returns_nothing_is_a_semantic_translation_in_every_target() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(VOID, "JavaScript", target).expect("translation descriptor");
        assert!(translated.diagnostic().is_none(), "{target}");
        assert_eq!(
            translated.contract().support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
    }
}

#[test]
fn a_function_that_finishes_without_a_return_value_returns_the_unit_value() {
    let rust = code(VOID, "Rust");
    assert!(
        rust.contains("pub fn hello(name: String) -> () {"),
        "{rust}"
    );
    assert!(
        rust.contains("pub fn report(x: crate::ml::Big) -> () {"),
        "{rust}"
    );
    let rocq = code(VOID, "Rocq");
    assert!(
        rocq.contains("(let ml_o1 := ((String.append \"hello \"%string name) :: ml_out) in (ml_io1_mk ml_o1 tt))."),
        "{rocq}"
    );
}

#[test]
fn a_call_whose_value_a_statement_discards_still_runs() {
    let rust = code(VOID, "Rust");
    assert!(
        rust.contains("let ml_main_ignored1 = crate::hello(String::from(\"a\"));"),
        "{rust}"
    );
    let lean = code(VOID, "Lean");
    assert!(
        lean.contains("let ml_run3 := (hello \"a\" ([] : List String))"),
        "{lean}"
    );
    let body = code(
        "function f(x) {\n  g(x);\n  return x;\n}\nfunction g(x) {\n  return x + 1n;\n}\nconsole.log(f(1n));\n",
        "Rust",
    );
    assert!(
        body.contains("let ml_f_ignored1 = crate::g(x.clone());"),
        "{body}"
    );
}

#[test]
fn a_function_that_returns_a_value_on_one_path_and_nothing_on_another_is_refused() {
    let error = parse_javascript("function f(n) { if (n > 0n) return n; }").expect_err("refused");
    assert_eq!(
        error.message(),
        "the function returns a bigint on one path and finishes without a return value, returning undefined, on another; return a value on every path at 16..38"
    );
}
