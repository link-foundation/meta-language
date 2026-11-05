//! JavaScript `let`, assignments and loops: the lowering lifts each loop to a
//! generated function, and Rust and JavaScript run those functions as loops.

use std::process::Command;

use meta_language::translation::check::check_program;
use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{SItem, SProgram};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links,
};
use meta_language::{TranslationSupport, translate_program};

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
    assert!(
        !program
            .items
            .iter()
            .any(|item| matches!(item, SItem::Data(_)))
    );
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
        (
            "function f(n) { var x = n; return x; }",
            "var declaration: var bindings are hoisted to the function and shared by its blocks; use let or const at 16..19",
        ),
        (
            "function f(n) { const x = n; x = 1n; return x; }",
            "assignment of constant x: assigning a const binding throws a TypeError; declare it with let at 29..33",
        ),
        (
            "function f(n) { let x; x = n; return x; }",
            "let x without a value: an uninitialised let holds undefined, which is not a portable value; give it an initial value at 16..20",
        ),
        (
            "function f(n) { let [a] = n; return a; }",
            "let destructuring: declare each binding with its own let at 16..20",
        ),
        (
            "function f(n) { x = 1n; let x = n; return x; }",
            "assignment of x before its declaration: the binding is in its temporal dead zone, where assigning it throws a ReferenceError at 16..20",
        ),
        (
            "function f(n) { break; }",
            "break outside a loop or switch at 16..21",
        ),
        (
            "function f(n) { outer: while (true) { break outer; } return n; }",
            "label outer: labels are outside the portable core; a break or continue applies to the innermost loop at 16..21",
        ),
        (
            "function f(n) { for (x of n) {} return n; }",
            "for…of without const or let: declare the loop variable with const or let, so each iteration has its own at 16..21",
        ),
        (
            "function f(n) { for (const x in n) {} return n; }",
            "for…in loop: iteration over the keys of objects is outside the portable core; count with for (let i = …; …; …) at 16..21",
        ),
        (
            "function f(n) { let ml_x = n; return ml_x; }",
            "reserved identifier: ml_x uses the translator's reserved ml_ prefix at 20..24",
        ),
        (
            "function f(n) { while (n > 0n) { if (n === 3n) return n; n--; } }",
            "the function returns a bigint on one path and finishes without a return value, returning undefined, on another; return a value on every path at 54..55",
        ),
        (
            "function f(n) { while (true) { return n; } return 0n; }",
            "unreachable statement: statements after return, throw, break, continue or a complete if are never executed at 43..54",
        ),
        (
            "function f(n) { n <<= 1n; return n; }",
            "<<= assignment: the portable compound assignments are +=, -=, *=, /=, %=, &&= and ||= at 16..22",
        ),
        (
            "function f(n) { for (let i = 0n; i < n; i++) { g = i; } return n; }",
            "assignment of g: only local variables declared with let, and parameters, are assignable at 47..51",
        ),
    ];
    for (source, message) in cases {
        if source == "function f(n) { let ml_x = n; return ml_x; }"
            || source == "function f(n) { let x; x = n; return x; }"
            || source == "function f(n) { while (n > 0n) { if (n === 3n) return n; n--; } }"
        {
            let calls = if source.contains("while") {
                "console.log(f(4n));console.log(f(0n));"
            } else {
                "console.log(f(17));"
            };
            let runnable = format!("{source}\n{calls}");
            let original = Command::new("node")
                .args(["--input-type=module", "-e", &runnable])
                .output()
                .expect("Node.js supplies the original modern JavaScript observation");
            assert!(
                original.status.success(),
                "{}",
                String::from_utf8_lossy(&original.stderr)
            );
            let checked =
                check_program(&parse(&runnable)).expect("modern optional loop source checks");
            let text = emit_javascript(&checked)
                .expect("modern optional loop source emits")
                .text;
            let emitted = Command::new("node")
                .args(["--input-type=module", "-e", &text])
                .output()
                .expect("Node.js runs the actual emitted complete module");
            assert_eq!(emitted.status.code(), original.status.code(), "{source}");
            assert_eq!(emitted.stdout, original.stdout, "{source}");
            assert_eq!(emitted.stderr, original.stderr, "{source}");
            assert_eq!(
                emitted.stdout,
                if source.contains("while") {
                    b"3n\nundefined\n".as_slice()
                } else {
                    b"17\n".as_slice()
                }
            );
            for target in ["Rust", "Lean", "Rocq"] {
                let translated = translate_program(&runnable, "JavaScript", target)
                    .expect("the original modern loop input has a translation descriptor");
                assert_eq!(
                    translated.contract().support,
                    TranslationSupport::SemanticTranslation,
                    "{target}: {source}"
                );
            }
            continue;
        }
        let error = parse_javascript(source).expect_err(source);
        assert_eq!(error.message(), message, "{source}");
    }
    for source in [
        "function read(){let first=NaN,NaN=1;return first;}console.log(read());",
        "function read(){let first=Infinity,Infinity=1;return first;}console.log(read());",
        "function read(){const result=NaN;let first=0,NaN=1;return result;}console.log(read());",
        "function read(){let first=undefined,undefined=1;return first;}console.log(read());",
        "function read(){const result=NaN\nlet first=0,NaN=1;return result;}console.log(read());",
        "function read(NaN){{const result=NaN;let first=0,NaN=1;return result;}}console.log(read(9));",
        "function read(){for(let NaN of [NaN]){console.log(NaN);}return 1;}console.log(read());",
        "function read(){for(const [NaN] of [[NaN]]){console.log(NaN);}return 1;}console.log(read());",
        "function read(){for(let first=NaN,NaN=1;first<2;first++){}return 1;}console.log(read());",
        "function read(NaN){{NaN=2;let first=0,NaN=1;return first;}}console.log(read(9));",
    ] {
        let original = Command::new("node")
            .args(["--input-type=module", "-e", source])
            .output()
            .unwrap();
        assert!(!original.status.success(), "{source}");
        assert!(
            String::from_utf8_lossy(&original.stderr).contains("ReferenceError"),
            "{source}"
        );
        let error = parse_javascript(source).expect_err(source);
        assert_eq!(error.kind, ErrorKind::Unsupported, "{source}");
        assert!(
            error.reason.contains("temporal dead zone"),
            "{error}: {source}"
        );
    }
    let deferred = "function read(){let first=()=>NaN,NaN=1;return first();}console.log(read());";
    assert_eq!(lexical_binding_output(deferred), b"1\n");
    let error =
        parse_javascript(deferred).expect_err("the capture needs an invocation-order proof");
    assert_eq!(error.kind, ErrorKind::Unsupported);
    assert!(error.reason.contains("invocation-order proof"), "{error}");
    let eager = "function read(){const fn=()=>{let first=NaN,NaN=1;return first;};return fn();}console.log(read());";
    let original = Command::new("node")
        .args(["--input-type=module", "-e", eager])
        .output()
        .unwrap();
    assert!(!original.status.success());
    assert!(String::from_utf8_lossy(&original.stderr).contains("ReferenceError"));
    let error = parse_javascript(eager).expect_err("the closure body's own let is uninitialized");
    assert_eq!(error.kind, ErrorKind::Unsupported);
    assert!(error.reason.contains("temporal dead zone"), "{error}");
    assert!(!error.reason.contains("invocation-order proof"), "{error}");
    for source in [
        "function read(){return NaN;}let first=0,NaN=1;console.log(read());",
        "let first=0,NaN=1;function read(){return NaN;}console.log(read());",
        "function read(value=NaN){return value;}let first=0,NaN=1;console.log(read());",
    ] {
        assert_eq!(
            lexical_binding_output(source),
            b"1\n",
            "the actual module source is the oracle"
        );
        let error =
            parse_javascript(source).expect_err("mutable module captures are not represented");
        assert_eq!(error.kind, ErrorKind::Unsupported);
        assert!(
            error
                .reason
                .contains("mutable module binding requires module-capture representation"),
            "{error}: {source}"
        );
    }
    for source in [
        "function read(){let first=0\nInfinity,NaN;return first;}",
        concat!(
            "function read(){let first={",
            "let:2};first.let\nNaN,Infinity;return first.let;}"
        ),
    ] {
        assert!(parse_javascript(source).is_err(), "{source}");
    }
}

fn lexical_binding_output(source: &str) -> Vec<u8> {
    let result = Command::new("node")
        .args(["--input-type=module", "-e", source])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    result.stdout
}

#[test]
fn actual_lexical_owners_preserve_initialized_bindings_shadows_and_order_through_links() {
    for (source, expected) in [
        (
            "function read(){let NaN=1,first=NaN;return first;}console.log(read());",
            "1\n",
        ),
        (
            "function read(){let first=1\nlet second=first+1;return second;}console.log(read());",
            "2\n",
        ),
        (
            "function read(){let first=1\n+2,second=first+1;return second;}console.log(read());",
            "4\n",
        ),
        (
            concat!(
                "function read(){let first={",
                "let:2},second=first.let;return second;}console.log(read());"
            ),
            "2\n",
        ),
        (
            "function read(){let first=[1,2],second={x:3,y:4};return first.length+second.x;}console.log(read());",
            "5\n",
        ),
        (
            "function read(){const [NaN,other]=[1,2];let first=NaN,Infinity=other;return first+Infinity;}console.log(read());",
            "3\n",
        ),
        (
            "function read(){let total=0;{const NaN=3;total=NaN;}let first=0,NaN=1;return total+NaN;}console.log(read());",
            "4\n",
        ),
        (
            "function read(){let reader=(NaN)=>NaN,NaN=1;return reader(3);}console.log(read());",
            "3\n",
        ),
        (
            "function read(){let NaN=1,reader=()=>NaN;return reader();}console.log(read());",
            "1\n",
        ),
        (
            "function read(){let total=0;for(let NaN=0;NaN<2;NaN++){total+=NaN;}let first=0,NaN=3;return total+NaN;}console.log(read());",
            "4\n",
        ),
        (
            "function read(){let total=0;for(const NaN of [1,2]){total+=NaN;}let first=0,NaN=3;return total+NaN;}console.log(read());",
            "6\n",
        ),
        (
            "function read(){let total=0;for(const [x] of [[1],[2]]){total+=x;}return total;}console.log(read());",
            "3\n",
        ),
        (
            "function pair(a,b){console.log(a);return b;}function read(){let first=pair(1,2),second=pair(first,3);return second;}console.log(read());",
            "1\n2\n3\n",
        ),
        (
            "function read(NaN){return NaN;}let first=0,NaN=1;console.log(read(2));",
            "2\n",
        ),
        (
            "const NaN=3;function read(){return NaN;}console.log(read());",
            "3\n",
        ),
    ] {
        assert_eq!(
            lexical_binding_output(source),
            expected.as_bytes(),
            "the original source is the oracle"
        );
        let checked = check_program(&parse(source)).unwrap();
        assert_eq!(
            lexical_binding_output(&emit_javascript(&checked).unwrap().text),
            expected.as_bytes(),
            "{source}"
        );
    }
    let source = "function read(){let NaN=1,first=NaN;return first;}console.log(read()===1);";
    let program = check_program(&parse(source)).unwrap();
    let archive = encode_typed_program_links(&program).unwrap();
    let rust = emit_rust(&program).unwrap().text;
    for network in [
        LinkNetwork::from_lino(&archive.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&archive.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, archive.root).unwrap();
        assert_eq!(restored, program);
        assert_eq!(
            lexical_binding_output(&emit_javascript(&restored).unwrap().text),
            lexical_binding_output(source)
        );
        assert_eq!(emit_rust(&restored).unwrap().text, rust);
    }
}

#[test]
fn literal_bindings_share_recursive_admission_without_spreads_effects_or_mutable_module_captures() {
    for (source, expected) in [
        (
            "const NaN=3;function read(){return NaN;}console.log(read());",
            "3\n",
        ),
        (
            "const NaN=null;function read(){return NaN===null;}console.log(read());",
            "true\n",
        ),
        (
            "const missing=undefined;function read(){return missing===undefined;}console.log(read());",
            "true\n",
        ),
        (
            "const Infinity=false;function read(){return Infinity;}console.log(read());",
            "false\n",
        ),
        (
            "const NaN=\"word\";function read(){return NaN;}console.log(read());",
            "word\n",
        ),
        (
            concat!(
                "const NaN={",
                "value:3};function read(){return NaN.value;}console.log(read());"
            ),
            "3\n",
        ),
        (
            concat!(
                "const NaN={outer:{inner:{",
                "value:7}}};function read(){return NaN.outer.inner.value;}console.log(read());"
            ),
            "7\n",
        ),
        (
            "const NaN=3;function read(NaN){return NaN;}console.log(read(9));",
            "9\n",
        ),
        (
            "function first(){const NaN=3;return NaN;}function second(){return NaN;}console.log(first(),Number.isNaN(second()));",
            "3 true\n",
        ),
    ] {
        assert_eq!(
            lexical_binding_output(source),
            expected.as_bytes(),
            "the unchanged original source independently specifies the result"
        );
        let checked = check_program(&parse(source)).unwrap();
        let archive = encode_typed_program_links(&checked).unwrap();
        let rust = emit_rust(&checked).unwrap().text;
        assert_eq!(
            lexical_binding_output(&emit_javascript(&checked).unwrap().text),
            expected.as_bytes(),
            "{source}"
        );
        for network in [
            LinkNetwork::from_lino(&archive.network.to_lino()).unwrap(),
            decode_link_network_binary(
                &encode_link_network_binary(&archive.network, BinaryLinksOptions::default())
                    .unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            let restored = decode_typed_program_links(&network, archive.root).unwrap();
            assert_eq!(
                restored, checked,
                "both source-free archives preserve the admitted initializer"
            );
            assert_eq!(
                lexical_binding_output(&emit_javascript(&restored).unwrap().text),
                expected.as_bytes(),
                "{source}"
            );
            assert_eq!(emit_rust(&restored).unwrap().text, rust);
        }
    }
    for (source, expected, earlier_construct) in [
        (
            concat!(
                "const base={",
                "value:3};const NaN={...base};function read(){return NaN.value;}console.log(read());"
            ),
            "3\n",
            Some("namespace after a top-level statement"),
        ),
        (
            concat!(
                "const base={",
                "value:3};const NaN={nested:{...base}};function read(){return NaN.nested.value;}console.log(read());"
            ),
            "3\n",
            Some("namespace after a top-level statement"),
        ),
        (
            concat!(
                "const NaN={...{",
                "value:3}};function read(){return NaN.value;}console.log(read());"
            ),
            "3\n",
            None,
        ),
        (
            concat!(
                "const NaN={nested:{...{",
                "value:3}}};function read(){return NaN.nested.value;}console.log(read());"
            ),
            "3\n",
            None,
        ),
        (
            "function create(){console.log(\"init\");return 3;}const NaN=create();function read(){return NaN;}console.log(read());",
            "init\n3\n",
            None,
        ),
        (
            "const base=3;const NaN=base+1;function read(){return NaN;}console.log(read());",
            "4\n",
            None,
        ),
        (
            "const NaN=[3];function read(){return NaN[0];}console.log(read());",
            "3\n",
            None,
        ),
        (
            "let NaN=3;function read(){return NaN;}console.log(read());",
            "3\n",
            None,
        ),
        (
            "let Infinity=4;function read(){return Infinity;}console.log(read());",
            "4\n",
            None,
        ),
        (
            "let undefined=5;function read(){return undefined;}console.log(read());",
            "5\n",
            None,
        ),
    ] {
        assert_eq!(
            lexical_binding_output(source),
            expected.as_bytes(),
            "refusal does not change the original module observations"
        );
        let error = parse_javascript(source)
            .expect_err("the initializer has no admitted literal ownership");
        assert_eq!(error.kind, ErrorKind::Unsupported, "{source}: {error}");
        if let Some(construct) = earlier_construct {
            assert_eq!(
                error.construct.as_deref(),
                Some(construct),
                "{source}: {error}"
            );
        } else {
            assert!(
                error.reason.contains("module binding") || error.reason.contains("module-capture"),
                "{source}: {error}"
            );
        }
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
