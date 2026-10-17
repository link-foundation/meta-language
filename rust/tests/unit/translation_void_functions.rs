//! JavaScript functions that return nothing: falling off the end or `return;`
//! returns undefined, the unit value, and a statement that discards a call's
//! value still runs it.

use meta_language::translation::check::check_program;
use meta_language::translation::ir::Decl;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::SItem;
use meta_language::translation::types::{INT, Type};
use meta_language::{TranslationSupport, translate_program};

use super::translation_javascript_frontend::preserves_javascript_translation_behavior;

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
    // translation-void-functions.test.js retains this original source and
    // observes an actual BigInt or undefined, rather than rejecting its join.
    let original = "function f(n) { if (n > 0n) return n; }";
    let surface = parse_javascript(original).unwrap();
    let SItem::Fn(function) = &surface.items[0] else {
        panic!("the original partially returning function remains present")
    };
    let optional = Type::Option {
        element: Box::new(INT),
        absence: "undefined".to_owned(),
    };
    assert_eq!(function.params[0].ty, Some(INT));
    assert_eq!(function.ret, Some(optional.clone()));
    let checked = check_program(&surface).unwrap();
    let Some(Decl::Fn(function)) = checked.declaration("f") else {
        panic!("the original checked function remains present")
    };
    assert_eq!(function.params[0].ty, INT);
    assert_eq!(function.ret, optional);
    assert_eq!(
        function.body.ty,
        Type::Option {
            element: Box::new(INT),
            absence: "undefined".to_owned()
        }
    );
    preserves_javascript_translation_behavior(
        &format!("{original}\nconsole.log(f(1n));\nconsole.log(f(0n));\nconsole.log(f(-1n));\n"),
        "1n\nundefined\nundefined\n",
    );
    preserves_javascript_translation_behavior(
        "function trace() { console.log('trace'); }\nfunction f(n) { if (n > 0n) return n; return trace(); }\nconsole.log(f(1n));\nconsole.log(f(0n));\n",
        "1n\ntrace\nundefined\n",
    );
}
