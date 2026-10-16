//! Shared JavaScript source decisions retain actual native parsing and execution.

use std::process::Command;

use meta_language::translation::Span;
use meta_language::translation::check::check_program;
use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{SItem, SNode};
use meta_language::translation::types::{FLOAT, Type};

fn observe(source: &str) -> String {
    let result = Command::new("node")
        .args(["--input-type=module", "-e", source])
        .output()
        .expect("execute the actual JavaScript source");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    String::from_utf8(result.stdout).expect("the observation is UTF-8")
}

#[test]
fn native_value_switches_preserve_actual_evaluation_order_and_continuations() {
    let cases = [
        (
            "function subject(){console.log('subject');return 2;}\nfunction candidate(value){console.log('test',value);return value;}\nfunction choose(){switch(subject()){case candidate(1):return 'one';default:return 'none';case candidate(2):return 'two';case candidate(3):return 'three';}}\nconsole.log(choose());",
            "subject\ntest 1\ntest 2\ntwo\n",
        ),
        (
            "function candidate(value){console.log('test',value);return value;}\nfunction choose(value){let result='initial';switch(value){case candidate(1):result='one';break;default:result='none';break;case candidate(2):case candidate(3):result='group';break;}return result;}\nconsole.log(choose(3));console.log(choose(4));",
            "test 1\ntest 2\ntest 3\ngroup\ntest 1\ntest 2\ntest 3\nnone\n",
        ),
        (
            "function candidate(value){console.log('test',value);return value;}\nfunction choose(value){let result='';switch(value){case candidate(1):result+='one';case candidate(2):result+='two';break;default:result='none';}return result;}\nconsole.log(choose(1));",
            "test 1\nonetwo\n",
        ),
        (
            "function numberWeight(value){switch(value){case -0:return 1;case 2.5:return 2;case NaN:return 3;default:return 4;}}\nfunction word(value){switch(value){case 'true':return 5;default:return 6;}}\nconsole.log(numberWeight(0),numberWeight(2.5),numberWeight(NaN),word('true'),word('false'));",
            "1 2 4 5 6\n",
        ),
        (
            "function signed(value){let result='none';switch(value){case -1n:result='negative';break;case value:result='same';break;}return result;}\nconsole.log(`${signed(-1n)} ${signed(2n)}`);",
            "negative same\n",
        ),
    ];
    for (source, expected) in cases {
        assert_eq!(observe(source), expected, "original {source}");
        let surface =
            parse_javascript(source).expect("the native frontend reads the actual program");
        let checked =
            check_program(&surface).expect("the native checker checks the actual program");
        let emitted =
            emit_javascript(&checked).expect("the native emitter renders the actual program");
        assert_eq!(observe(&emitted.text), expected, "native emission {source}");
    }
}

#[test]
fn pure_switch_names_share_the_actual_module_generation_counter() {
    let source = "/** @param {string} value @returns {number} */function first(value){switch(value){case 'a':return 1;default:return 0;}}\n/** @param {string} value @returns {number} */function second(value){switch(value){case value:return 2;default:return 0;}}";
    let surface = parse_javascript(source).expect("both actual pure switches parse");
    let names: Vec<_> = surface
        .items
        .iter()
        .map(|item| {
            let SItem::Fn(function) = item else {
                panic!("an actual function")
            };
            let SNode::Let { name, .. } = &function.body.node else {
                panic!("the discriminant is bound once")
            };
            name.as_str()
        })
        .collect();
    assert_eq!(names, ["ml_switch1", "ml_switch2"]);
    check_program(&surface).expect("both switches retain their actual scalar types");
}

#[test]
fn value_case_syntax_and_incompatible_types_keep_their_distinct_boundaries() {
    for expression in ["-true", "-name"] {
        let source = format!(
            "function choose(value){{switch(value){{case {expression}:return 1;default:return 0;}}}}"
        );
        let error = parse_javascript(&source).expect_err("the original invalid case form refuses");
        let start = source.find(expression).unwrap();
        assert_eq!(error.kind, ErrorKind::Unsupported);
        assert_eq!(
            error.reason,
            "case test: the cases of a value switch are BigInt or boolean literals"
        );
        assert_eq!(error.span, Some(Span::new(start, start + expression.len())));
    }
    let source = "/** @param {bigint} value @returns {bigint} */function choose(value){switch(value){case \"one\":return 1n;default:return 0n;}}";
    let surface =
        parse_javascript(source).expect("case expression parsing preserves its actual type");
    assert_eq!(
        check_program(&surface)
            .expect_err("strict equality cannot mix these declared types")
            .kind,
        ErrorKind::Type
    );
}

#[test]
fn lazy_documentation_aliases_resolve_real_types_and_preserve_reference_errors() {
    for source in [
        "/** @typedef {{ 'a': bigint }} A */\nconsole.log('x');",
        "/** @typedef {{ $ 'a' }} A */\nconsole.log('x');",
        "/** @typedef {bigint} N */\nconsole.log('x');",
        "/** @typedef {{ x: bigint }} A */\nconsole.log('x');",
    ] {
        let surface = parse_javascript(source).expect("the unused alias remains lazy");
        assert!(surface.items.is_empty());
        check_program(&surface).expect("the original program checks");
    }
    let source = "/** @typedef {number} Scalar */\n/** @typedef {Scalar[]} Values */\n/** @typedef {[Scalar,Scalar]} Pair */\n/** @param {Values} values @returns {Scalar} */\nfunction size(values){return values.length;}\n/** @param {Pair} value @returns {Pair} */\nfunction identity(value){return value;}";
    let surface =
        parse_javascript(source).expect("referenced aliases resolve through normal type readers");
    let SItem::Fn(size) = &surface.items[0] else {
        panic!("size")
    };
    assert!(
        matches!(&size.params[0].ty, Some(Type::Array { element, .. }) if element.as_ref() == &FLOAT)
    );
    assert_eq!(size.ret, Some(FLOAT));
    let SItem::Fn(identity) = &surface.items[1] else {
        panic!("identity")
    };
    assert_eq!(
        identity.params[0].ty,
        Some(Type::Tuple {
            elements: vec![FLOAT, FLOAT]
        })
    );
    assert_eq!(identity.ret, identity.params[0].ty);
    check_program(&surface).expect("the resolved aliases check");

    let prefix = "/** @typedef {Loop} Loop */\n";
    let comment = "/** @param {Loop} value @returns {Loop} */";
    let source = format!("{prefix}{comment}\nfunction recursive(value){{return value;}}");
    let surface =
        parse_javascript(&source).expect("an unresolved recursive alias remains a named reference");
    let error =
        check_program(&surface).expect_err("the unresolved type refuses at normal checking");
    assert_eq!(error.kind, ErrorKind::Type);
    assert_eq!(error.reason, "unknown type crate.Loop");
    assert_eq!(
        error.span,
        Some(Span::new(prefix.len(), prefix.len() + comment.len()))
    );

    let prefix = "/** @typedef {unknown} Invalid */\n";
    let comment = "/** @param {Invalid} value @returns {number} */";
    let source = format!("{prefix}{comment}\nfunction invalid(value){{return 1;}}");
    let error = parse_javascript(&source).expect_err("an actually referenced invalid type refuses");
    assert_eq!(error.kind, ErrorKind::Unsupported);
    assert_eq!(
        error.reason,
        "JSDoc type {unknown}: portable types are number, bigint, boolean, string, arrays T[] of them and @typedef data types"
    );
    assert_eq!(
        error.span,
        Some(Span::new(prefix.len(), prefix.len() + comment.len()))
    );

    let prefix = "/** @typedef {number} Value */\n";
    let comment = "/** @typedef {string} Value */";
    let error = parse_javascript(&format!("{prefix}{comment}"))
        .expect_err("duplicate aliases refuse at their declaration");
    assert_eq!(error.kind, ErrorKind::Type);
    assert_eq!(error.reason, "duplicate @typedef Value");
    assert_eq!(
        error.span,
        Some(Span::new(prefix.len(), prefix.len() + comment.len()))
    );
}
