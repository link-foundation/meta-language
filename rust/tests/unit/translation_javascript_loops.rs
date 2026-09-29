//! JavaScript `let`, assignments and loops: the lowering lifts each loop to a
//! generated function, and Rust and JavaScript run those functions as loops.

use std::process::Command;

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{SItem, SProgram};
use meta_language::{translate_program, TranslationSupport};

const SUM: &str = "function sum(n) {
  let total = 0n;
  for (let i = 1n; i <= n; i++) {
    total += i;
  }
  return total;
}
console.log(sum(10n));
";

fn parse(source: &str) -> SProgram {
    parse_javascript(source).unwrap_or_else(|error| panic!("{}", error.message()))
}

#[test]
fn a_loop_is_a_generated_function_of_the_variables_it_uses() {
    let program = parse(SUM);
    let lifted = program
        .items
        .iter()
        .find_map(|item| match item {
            SItem::Fn(function) if function.name == "ml_sum_loop1" => Some(function),
            _ => None,
        })
        .expect("the lifted loop");
    assert!(lifted.generated);
    let params: Vec<&str> = lifted
        .params
        .iter()
        .map(|param| param.name.as_str())
        .collect();
    assert_eq!(params, ["n", "total", "i"]);
    // The one variable the loop assigns and the statements after it read is its result: no result type is needed.
    assert!(!program
        .items
        .iter()
        .any(|item| matches!(item, SItem::Data(_))));
}

#[test]
fn a_loop_that_returns_or_leaves_live_variables_returns_a_generated_data_type() {
    let program = parse(
        "function divisor(n) {
  for (let d = 2n; d * d <= n; d++) {
    if (n % d === 0n) return d;
  }
  return n;
}
function gcd(a, b) {
  while (b !== 0n) {
    const r = a % b;
    a = b;
    b = r;
  }
  return a;
}
console.log(divisor(91n) + gcd(1071n, 462n));
",
    );
    let data = |name: &str| {
        program.items.iter().find_map(|item| match item {
            SItem::Data(data) if data.name == name => Some(data),
            _ => None,
        })
    };
    let result = data("ml_divisor_loop1_result").expect("the divisor loop's result type");
    assert!(result.generated);
    let ctors: Vec<&str> = result.ctors.iter().map(|ctor| ctor.name.as_str()).collect();
    assert_eq!(ctors, ["ml_divisor_loop1_done", "ml_divisor_loop1_return"]);
    // gcd's loop assigns a and b, but only a is read after it.
    assert!(data("ml_gcd_loop2_result").is_none());
}

#[test]
fn mutable_bindings_javascript_could_not_run_as_translated_are_rejected() {
    let cases = [
        ("function f(n) { var x = n; return x; }", "var declaration: var bindings are hoisted to the function and shared by its blocks; use let or const at 16..19"),
        ("function f(n) { const x = n; x = 1n; return x; }", "assignment of constant x: assigning a const binding throws a TypeError; declare it with let at 29..33"),
        ("function f(n) { let x; x = n; return x; }", "let x without a value: an uninitialised let holds undefined, which is not a portable value; give it an initial value at 16..20"),
        ("function f(n) { let [a] = n; return a; }", "let destructuring: declare each binding with its own let at 16..20"),
        ("function f(n) { x = 1n; let x = n; return x; }", "assignment of x before its declaration: the binding is in its temporal dead zone, where assigning it throws a ReferenceError at 16..20"),
        ("function f(n) { break; }", "break outside a loop or switch at 16..21"),
        ("function f(n) { outer: while (true) { break outer; } return n; }", "label outer: labels are outside the portable core; a break or continue applies to the innermost loop at 16..21"),
        ("function f(n) { for (const x of n) {} return n; }", "for…of loop: iteration over arrays, strings and objects is outside the portable core; count with for (let i = …; …; …) at 16..21"),
        ("function f(n) { let ml_x = n; return ml_x; }", "reserved identifier: ml_x uses the translator's reserved ml_ prefix at 20..24"),
        ("function f(n) { while (n > 0n) { n--; } }", "missing return: the function can finish without returning and return undefined, which is not a portable value at 9..10"),
        ("function f(n) { while (true) { return n; } return 0n; }", "unreachable statement: statements after return, throw, break, continue or a complete if are never executed at 43..54"),
        ("function f(n) { n <<= 1n; return n; }", "<<= assignment: the portable compound assignments are +=, -=, *=, /=, %=, &&= and ||= at 16..22"),
        ("function f(n) { for (let i = 0n; i < n; i++) { g = i; } return n; }", "assignment of g: only local variables declared with let, and parameters, are assignable at 47..51"),
    ];
    for (source, message) in cases {
        let error = parse_javascript(source).expect_err(source);
        assert_eq!(error.message(), message, "{source}");
    }
}

#[test]
fn a_program_of_loops_is_a_semantic_translation_and_runs_lifted_loops_as_loops() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated =
            translate_program(SUM, "JavaScript", target).expect("translation descriptor");
        assert_eq!(
            translated.contract().support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
    }
    let rust = translate_program(SUM, "JavaScript", "Rust").expect("translation descriptor");
    let head = "pub fn ml_sum_loop1(mut n: ";
    let start = rust.code().find(head).expect("the lifted loop in Rust");
    let signature = &rust.code()[start..];
    assert!(
        signature[..signature.find('\n').expect("a line")].ends_with("-> crate::ml::Big {")
            && signature[signature.find('\n').expect("a line")..].starts_with("\n    loop {"),
        "{signature}"
    );
    // A hundred thousand iterations would overflow the stack as a hundred thousand calls.
    let long = SUM.replace("sum(10n)", "sum(100000n)");
    let checked = check_program(&parse(&long)).expect("the program checks");
    let text = emit_javascript(&checked).expect("the program emits").text;
    assert!(text.contains("for (;;) {"), "{text}");
    let output = Command::new("node")
        .args(["--input-type=module", "-e", &text])
        .output()
        .expect("Node.js available for translated JavaScript");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"5000050000n\n");
}
