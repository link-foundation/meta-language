//! Top-level `let`, assignments, ifs and loops: main binds each top-level
//! variable the statements before a print declare or assign.

use std::process::Command;

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{SEffect, SItem, SProgram};
use meta_language::{translate_program, TranslationSupport};

const FIBONACCI: &str = "let a = 0n;
let b = 1n;
while (b < 100n) {
  const next = a + b;
  a = b;
  b = next;
}
let label = 'small';
if (a + b > 100n) label = 'large';
console.log(a + b);
console.log(label);
";

fn parse(source: &str) -> SProgram {
    parse_javascript(source).unwrap_or_else(|error| panic!("{}", error.message()))
}

#[test]
fn top_level_statements_are_a_semantic_translation_in_every_target() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(FIBONACCI, "JavaScript", target).expect("translation descriptor");
        assert!(translated.diagnostic().is_none(), "{target}");
        assert_eq!(
            translated.contract().support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
    }
    let checked = check_program(&parse(FIBONACCI)).expect("the program checks");
    let text = emit_javascript(&checked).expect("the program emits").text;
    let output = Command::new("node")
        .args(["--input-type=module", "-e", &text])
        .output()
        .expect("Node.js available for translated JavaScript");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"233n\nlarge\n");
}

#[test]
fn main_binds_each_variable_the_statements_before_a_print_assign() {
    let program = parse(FIBONACCI);
    let data = program
        .items
        .iter()
        .find_map(|item| match item {
            SItem::Data(data) if data.name == "ml_main_top2" => Some(data),
            _ => None,
        })
        .expect("the data type carrying the variables");
    assert!(data.generated);
    let fields: Vec<Vec<&str>> = data
        .ctors
        .iter()
        .map(|ctor| {
            ctor.fields
                .iter()
                .filter_map(|field| field.name.as_deref())
                .collect()
        })
        .collect();
    assert_eq!(fields, [["a", "b", "label"]]);
    let effects: Vec<&str> = program
        .main
        .as_ref()
        .expect("main")
        .effects
        .iter()
        .map(|effect| match effect {
            SEffect::Let { name, .. } => name.as_str(),
            SEffect::Print { .. } => "print",
            SEffect::Assert { .. } => "assert",
        })
        .collect();
    assert_eq!(
        effects,
        ["ml_main_top2_value", "a", "b", "label", "print", "print"]
    );
}

#[test]
fn top_level_statements_javascript_could_not_run_as_translated_are_rejected() {
    let cases = [
        ("let x = 1n;\nif (x > 0n) { return; }\n", "top-level return statement: return leaves a function, and a module has none to leave at 26..32"),
        ("let s = 0n;\nswitch (s) { case 0n: s = 1n; }\n", "top-level switch statement: the top level prints with console.log, binds with const or let, assigns, branches with if, loops and asserts at 12..18"),
        ("let x = 1n;\nconst f = () => 2n;\nconsole.log(x);\n", "function after a top-level statement: the statements before const f could call it before it is initialised; declare every function first at 12..18"),
        ("const c = 1n;\nc = 2n;\n", "assignment of constant c: assigning a const binding throws a TypeError; declare it with let at 14..18"),
    ];
    for (source, message) in cases {
        let error = parse_javascript(source).expect_err(source);
        assert_eq!(error.message(), message, "{source}");
    }
}
