use meta_language::translation::diagnostics::ErrorKind;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{
    SEffect, SExpr, SItem, SNode, SPatternNode, SProgram, SPropNode,
};
use meta_language::translation::types::{BOOL, FLOAT, INT, NAT, STRING, Type};
use meta_language::translation::{Language, Span};

const TREE: &str = "/**\n * @typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint, right: Tree }} Tree\n */\n";

fn parse(source: &str) -> SProgram {
    parse_javascript(source).unwrap_or_else(|error| panic!("{}", error.message()))
}

fn rejection(source: &str) -> (ErrorKind, String) {
    let error = parse_javascript(source).expect_err("the program is rejected");
    (error.kind, error.message())
}

fn function_body(program: &SProgram, name: &str) -> SExpr {
    program
        .items
        .iter()
        .find_map(|item| match item {
            SItem::Fn(function) if function.name == name => Some(function.body.clone()),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no function {name}"))
}

#[test]
fn functions_read_their_types_from_jsdoc_and_guards_make_naturals() {
    let program = parse(
        "/**\n * @param {bigint} n\n * @returns {bigint}\n */\nfunction f(n) {\n  if (n < 0n) throw new RangeError('negative');\n  return n === 0n ? 1n : n * f(n - 1n);\n}\nconsole.log(String(f(5n)));\n",
    );
    assert_eq!(program.language, Language::JavaScript);
    let SItem::Fn(function) = &program.items[0] else {
        panic!("a function")
    };
    assert_eq!(function.name, "f");
    assert_eq!(function.params[0].ty, Some(NAT));
    let guard = function.params[0].guard.as_ref().expect("a guard");
    assert_eq!(guard.message, "negative");
    assert!(matches!(function.body.node, SNode::If { .. }));
    let main = program.main.expect("a main");
    assert_eq!(main.span, Some(Span::new(0, 184)));
    assert!(matches!(
        &main.effects[..],
        [SEffect::Print {
            expr: SExpr {
                node: SNode::ToString { .. },
                ..
            },
            ..
        }]
    ));
}

#[test]
fn a_tag_switch_is_a_match_whose_field_reads_are_pattern_variables() {
    let source = format!(
        "{TREE}/**\n * @param {{Tree}} t\n * @returns {{bigint}}\n */\nfunction size(t) {{\n  switch (t.$) {{\n    case 'leaf': return 0n;\n    case 'node': return size(t.left) + 1n + size(t.right);\n  }}\n}}\n"
    );
    let program = parse(&source);
    let SItem::Data(tree) = &program.items[0] else {
        panic!("a data type")
    };
    assert_eq!(tree.name, "Tree");
    assert_eq!(tree.ctors.len(), 2);
    let SNode::Match { rows, .. } = function_body(&program, "size").node else {
        panic!("a match")
    };
    let SPatternNode::Ctor { path, args } = &rows[1].patterns[0].node else {
        panic!("a constructor pattern")
    };
    assert_eq!(path, &["crate", "Tree", "node"]);
    let names: Vec<_> = args
        .iter()
        .map(|arg| match &arg.node {
            SPatternNode::BindOrCtor { name } => name.as_str(),
            _ => "_",
        })
        .collect();
    assert_eq!(names, ["left", "_", "right"]);
}

#[test]
fn a_tag_test_keeps_the_narrowing_it_describes() {
    let source = format!(
        "{TREE}/**\n * @param {{Tree}} t\n * @returns {{boolean}}\n */\nfunction isLeaf(t) {{ return t.$ !== 'node'; }}\n"
    );
    let body = function_body(&parse(&source), "isLeaf");
    let test = body.tag_test.expect("a tag test");
    assert_eq!(test.tag, "node");
    assert_eq!(test.data.name, "Tree");
    assert!(test.negated);
    let json = serde_json::to_value(&body.node).expect("serialisable");
    assert_eq!(json["k"], "match");
}

#[test]
fn assertions_become_propositions_and_strict_imports_allow_equal() {
    let program = parse(
        "import { strict as check } from 'node:assert';\ncheck.equal(1n, 1n);\ncheck(1n < 2n && !false);\n",
    );
    let effects = program.main.expect("a main").effects;
    let SEffect::Assert { prop, .. } = &effects[0] else {
        panic!("an assertion")
    };
    assert!(matches!(&prop.node, SPropNode::Eq(comparison) if comparison.reference));
    let SEffect::Assert { prop, .. } = &effects[1] else {
        panic!("an assertion")
    };
    assert!(matches!(prop.node, SPropNode::And { .. }));
}

#[test]
fn spans_count_utf16_code_units() {
    let program = parse("console.log('😀');\nconsole.log('x');\n");
    let effects = program.main.expect("a main").effects;
    let SEffect::Print { expr, .. } = &effects[1] else {
        panic!("a print")
    };
    assert_eq!(expr.span, Some(Span::new(31, 34)));
}

#[test]
fn number_literals_are_ieee_doubles_kept_as_javascript_prints_them() {
    let program = parse("console.log(1e21 + 0x10);");
    let effects = &program.main.as_ref().expect("a main").effects;
    let SEffect::Print { expr, .. } = &effects[0] else {
        panic!("a print")
    };
    let SNode::Binary { left, right, .. } = &expr.node else {
        panic!("an addition")
    };
    for (operand, text) in [(left, "1e+21"), (right, "16")] {
        let SNode::Num { value, ty, .. } = &operand.node else {
            panic!("a number literal")
        };
        assert_eq!((value.as_str(), ty.as_ref()), (text, Some(&FLOAT)));
    }
}

fn signature(program: &SProgram, name: &str) -> (Vec<Option<Type>>, Option<Type>) {
    fn find(items: &[SItem], name: &str) -> Option<(Vec<Option<Type>>, Option<Type>)> {
        items.iter().find_map(|item| match item {
            SItem::Fn(function) if function.name == name => Some((
                function
                    .params
                    .iter()
                    .map(|param| param.ty.clone())
                    .collect(),
                function.ret.clone(),
            )),
            SItem::Module(module) => find(&module.items, name),
            _ => None,
        })
    }
    find(&program.items, name).unwrap_or_else(|| panic!("no function {name}"))
}

#[test]
fn types_jsdoc_leaves_out_are_inferred_from_the_uses() {
    let program = parse(&format!(
        "{TREE}function answer() {{ return 42; }}\nfunction fact(n) {{ if (n < 0n) throw new RangeError('negative'); return n === 0n ? 1n : n * fact(n - 1n); }}\nfunction label(n) {{ return n + '!'; }}\nfunction pick(flag, a, b) {{ return flag ? a : b; }}\nfunction sum(t) {{ switch (t.$) {{ case 'leaf': return 0n; case 'node': return sum(t.left) + t.value + sum(t.right); }} }}\nconst M = {{ sq(x) {{ return x * x; }}, two(x) {{ return M.sq(x) + M.sq(x); }} }};\nconsole.log(answer());\nconsole.log(label(fact(3n)));\nconsole.log(pick(true, 'a', 'b'));\nconsole.log(M.two(1.5));\n"
    ));
    let tree = Type::Named {
        path: vec!["crate".to_owned(), "Tree".to_owned()],
        span: None,
    };
    let without_spans = |(params, ret): (Vec<Option<Type>>, Option<Type>)| {
        let strip = |ty: Option<Type>| match ty {
            Some(Type::Named { path, .. }) => Some(Type::Named { path, span: None }),
            other => other,
        };
        (
            params.into_iter().map(strip).collect::<Vec<_>>(),
            strip(ret),
        )
    };
    assert_eq!(signature(&program, "answer"), (vec![], Some(FLOAT)));
    assert_eq!(signature(&program, "fact"), (vec![Some(NAT)], Some(INT)));
    assert_eq!(
        signature(&program, "label"),
        (vec![Some(INT)], Some(STRING))
    );
    assert_eq!(
        signature(&program, "pick"),
        (vec![Some(BOOL), Some(STRING), Some(STRING)], Some(STRING))
    );
    assert_eq!(
        without_spans(signature(&program, "sum")),
        (vec![Some(tree)], Some(INT))
    );
    assert_eq!(signature(&program, "sq"), (vec![Some(FLOAT)], Some(FLOAT)));
    assert_eq!(signature(&program, "two"), (vec![Some(FLOAT)], Some(FLOAT)));
}

#[test]
fn uses_that_need_different_types_are_type_errors() {
    let cases = [
        (
            "function id(x) { return x; }\nconsole.log(id(1));\nconsole.log(id('a'));",
            "one value is used as a string and as a number; declare the types of the function with JSDoc at 64..67",
        ),
        (
            "function f(x) { return x + 1n; }\nconsole.log(f(1));",
            "a BigInt is used as a number; JavaScript does not mix BigInt with other types at 23..29",
        ),
        (
            "function f(x) { return !x; }\nconsole.log(f(1));",
            "one value is used as a number and as a boolean; declare the types of the function with JSDoc at 43..44",
        ),
    ];
    for (source, message) in cases {
        assert_eq!(
            rejection(source),
            (ErrorKind::Type, message.to_owned()),
            "{source}"
        );
    }
}

#[test]
fn rejections_name_the_construct_and_its_span() {
    let doc = "/**\n * @param {bigint} n\n * @returns {bigint}\n */\n";
    let cases = [
        (
            format!("{doc}function f(n) {{ var x = n; return x; }}"),
            ErrorKind::Unsupported,
            "var declaration: var bindings are hoisted to the function and shared by its blocks; use let or const at 66..69",
        ),
        (
            format!("{doc}function f(n) {{ return n; return n; }}"),
            ErrorKind::Unsupported,
            "unreachable statement: statements after return, throw or a complete if are never executed at 76..86",
        ),
        (
            "import fs from 'node:fs';".to_owned(),
            ErrorKind::Unsupported,
            "import from 'node:fs': modules other than node:assert are outside the portable core at 0..15",
        ),
        (
            "/** @typedef {{ $: 'a' } | { $: 'a' }} A */\nconsole.log('x');".to_owned(),
            ErrorKind::Type,
            "duplicate tag a in A at 0..43",
        ),
        (
            "const x;".to_owned(),
            ErrorKind::Syntax,
            "expected = in const but found \";\" at 7..8",
        ),
        (
            "console.log(String(0xn));".to_owned(),
            ErrorKind::Syntax,
            "malformed number 0x at 19..21",
        ),
    ];
    for (source, kind, message) in cases {
        assert_eq!(rejection(&source), (kind, message.to_owned()), "{source}");
    }
}

#[test]
fn arrow_functions_and_function_expressions_bound_to_constants_are_functions() {
    let program = parse(
        "const inc = x => x + 1;\n/** @param {number} a @param {number} b @returns {number} */\nconst avg = (a, b) => (a + b) / 2;\nconst fact = function (n) { if (n < 0n) throw new RangeError('negative'); return n === 0n ? 1n : n * fact(n - 1n); };\nconst pow = function pow(b, n) { return n === 0n ? 1n : b * pow(b, n - 1n); };\nexport const greet = (name) => { return 'hello ' + name; };\nconsole.log(inc(avg(1, 2)));\nconsole.log(pow(2n, fact(3n)));\nconsole.log(greet('world'));\n",
    );
    assert_eq!(signature(&program, "inc"), (vec![Some(FLOAT)], Some(FLOAT)));
    assert_eq!(
        signature(&program, "avg"),
        (vec![Some(FLOAT), Some(FLOAT)], Some(FLOAT))
    );
    assert_eq!(signature(&program, "fact"), (vec![Some(NAT)], Some(INT)));
    assert_eq!(
        signature(&program, "pow"),
        (vec![Some(INT), Some(INT)], Some(INT))
    );
    assert_eq!(
        signature(&program, "greet"),
        (vec![Some(STRING)], Some(STRING))
    );
}

#[test]
fn const_functions_javascript_could_not_call_as_translated_are_rejected() {
    let cases = [
        (
            "console.log(1);\nconst f = x => x;",
            "function after a top-level statement: the statements before const f could call it before it is initialised; declare every function first at 16..22",
        ),
        (
            "const f = function g(x) { return x; };",
            "function expression g: its own name is visible only inside it; call it f at 19..20",
        ),
        (
            "const f = function* () {};",
            "generator function: generators are outside the portable core at 0..18",
        ),
        (
            "const f = (...xs) => xs;",
            "rest parameter: functions take a fixed number of arguments at 11..14",
        ),
    ];
    for (source, message) in cases {
        assert_eq!(
            rejection(source),
            (ErrorKind::Unsupported, message.to_owned()),
            "{source}"
        );
    }
}

#[test]
fn an_async_function_whose_every_call_is_awaited_is_the_function_its_body_computes() {
    let program = parse(
        "/** @param {bigint} n @returns {Promise<bigint>} */\nasync function square(n) { return n * n; }\nasync function twice(n) { return await square(await square(n)); }\nconst inc = async (x) => x + 1;\nexport async function again(x) { return inc(x); }\nconsole.log(await twice(2n));\nconsole.log(await again(41));\n",
    );
    assert_eq!(signature(&program, "square"), (vec![Some(INT)], Some(INT)));
    assert_eq!(signature(&program, "twice"), (vec![Some(INT)], Some(INT)));
    assert_eq!(signature(&program, "inc"), (vec![Some(FLOAT)], Some(FLOAT)));
    assert_eq!(
        signature(&program, "again"),
        (vec![Some(FLOAT)], Some(FLOAT))
    );
    assert!(program.main.expect("a main").sequential_async);
    assert!(
        !parse("console.log(1);")
            .main
            .expect("a main")
            .sequential_async
    );
}

#[test]
fn a_promise_the_program_could_observe_is_rejected_where_it_is_made() {
    let cases = [
        (
            "async function f() { return 1; }\nconsole.log(f());",
            "call of async function f without await: the Promise it returns is outside the portable core; await it where it is called at 45..48",
        ),
        (
            "async function f() { return 1; }\nfunction g() { return f(); }\nconsole.log(g());",
            "call of async function f without await: the Promise it returns is outside the portable core; await it where it is called at 55..58",
        ),
        (
            "function f(n) { return await n; }",
            "await outside an async function: await is only valid in async functions and at the top level of a module at 23..28",
        ),
        (
            "async () => 1;",
            "async function: only async functions declared by name or bound to a top-level constant are portable at 0..5",
        ),
    ];
    for (source, message) in cases {
        assert_eq!(
            rejection(source),
            (ErrorKind::Unsupported, message.to_owned()),
            "{source}"
        );
    }
}
