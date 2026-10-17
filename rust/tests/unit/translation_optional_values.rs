//! Actual modern JavaScript optional values retain runtime effects and types.
use meta_language::TranslationSupport;
use meta_language::translate_program;
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::ir::{Decl, Node};
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::types::Type;
use std::process::Command;

fn observe(source: &str) -> std::process::Output {
    Command::new("node")
        .args(["--input-type=module", "-e", source])
        .output()
        .expect("Node.js observes the complete actual module")
}

fn check_runtime(source: &str, expected: &[u8]) {
    let original = observe(source);
    assert!(
        original.status.success(),
        "{}",
        String::from_utf8_lossy(&original.stderr)
    );
    assert_eq!(original.stdout, expected);
    let checked = check_program(&parse_javascript(source).expect("the actual source parses"))
        .expect("the actual source checks");
    let emitted = emit_javascript(&checked).expect("the actual checked module emits");
    let translated = observe(&emitted.text);
    assert_eq!(translated.status.code(), original.status.code());
    assert_eq!(translated.stdout, original.stdout);
    assert_eq!(translated.stderr, original.stderr);
    for target in ["Rust", "Lean", "Rocq"] {
        let translated = translate_program(source, "JavaScript", target)
            .expect("an actual target descriptor exists");
        assert_eq!(
            translated.contract().support,
            TranslationSupport::SemanticTranslation,
            "{target}"
        );
        assert!(!translated.code().is_empty(), "{target}");
    }
}

#[test]
fn nullish_fallbacks_keep_falsy_values_and_only_run_absent_branch_effects() {
    let source = "function fallback(){console.log('fallback');return 7;}function choose(value){return value??fallback();}console.log(choose(0));console.log(choose(null));console.log(choose(undefined));";
    check_runtime(source, b"0\nfallback\n7\nfallback\n7\n");
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let Some(Decl::Fn(function)) = program.declaration("choose") else {
        panic!("the actual optional function is retained");
    };
    assert_eq!(
        function.params[0].ty,
        Type::Option {
            element: Box::new(Type::Float),
            absence: "both".to_owned()
        }
    );
    assert_eq!(function.ret, Type::Float);
    assert!(matches!(function.body.node, Node::Coalesce { .. }));
}

#[test]
fn optional_fields_numeric_coercions_truth_and_type_queries_follow_actual_javascript() {
    check_runtime(
        "function get(item){return item?.value??7;}console.log(get({value:0}));console.log(get(null));console.log(get(undefined));",
        b"0\n7\n7\n",
    );
    let source = "function inspect(value){console.log(`${value===null} ${value===undefined} ${!!value} ${typeof value} ${value+1}`);return value||5;}console.log(inspect(null));console.log(inspect(undefined));console.log(inspect(0));console.log(inspect(2));";
    check_runtime(source, b"true false false object 1\n5\nfalse true false undefined NaN\n5\nfalse false false number 1\n5\nfalse false true number 3\n2\n");
    for (source, kind) in [
        (
            "/** @param {{item:number}|null|undefined} value @returns {string} */function inspect(value){return typeof value;}console.log('%s',inspect(null),inspect(undefined),inspect({item:2}));",
            "record",
        ),
        (
            "/** @param {[number,string]|null|undefined} value @returns {string} */function inspect(value){return typeof value;}console.log('%s',inspect(null),inspect(undefined),inspect([2,'kept']));",
            "tuple",
        ),
        (
            "/** @param {Map<string,number>|null|undefined} value @returns {string} */function inspect(value){return typeof value;}console.log('%s',inspect(null),inspect(undefined),inspect(new Map([['kept',2]])));",
            "map",
        ),
        (
            "/** @param {Set<number>|null|undefined} value @returns {string} */function inspect(value){return typeof value;}console.log('%s',inspect(null),inspect(undefined),inspect(new Set([2])));",
            "set",
        ),
    ] {
        check_runtime(source, b"object undefined object\n");
        let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
        let Some(Decl::Fn(function)) = checked.declaration("inspect") else {
            panic!("the declared optional observer remains owned")
        };
        let Type::Option { element, absence } = &function.params[0].ty else {
            panic!("the original absence alternatives remain represented")
        };
        assert_eq!(element.kind(), kind);
        assert_eq!(absence, "both");
        assert!(
            matches!(&function.body.node, Node::OptionalTypeOf { present, .. } if present == "object")
        );
    }
    check_runtime(
        "function read(value){console.log('read');const n=value?.item??0;return value;}function describe(value){return typeof read(value);}console.log('%s',describe(null));console.log('%s',describe(undefined));console.log('%s',describe({item:2}));",
        b"read\nobject\nread\nundefined\nread\nobject\n",
    );
}

#[test]
fn optional_tuple_context_keeps_arity_spread_heterogeneous_array_and_unknown_type_refusals() {
    let optional = "/** @param {[number,number]|null|undefined} pair @returns {string} */function describe(pair){return typeof pair;}";
    check_runtime(
        &format!("{optional}console.log('%s',describe([2,3]),describe(null),describe(undefined));"),
        b"object object undefined\n",
    );
    let wrong_arity =
        parse_javascript(&format!("{optional}console.log('%s',describe([2]));")).unwrap_err();
    assert!(wrong_arity.message().contains("two different lengths"));
    let spread = parse_javascript(&format!(
        "{optional}console.log('%s',describe([...[2,3]]));"
    ))
    .unwrap_err();
    assert_eq!(
        spread.kind,
        meta_language::translation::diagnostics::ErrorKind::Type
    );
    assert!(spread.message().contains("array") && spread.message().contains("tuple"));
    let array = optional.replace("[number,number]", "number[]");
    let mixed =
        parse_javascript(&format!("{array}console.log('%s',describe([2,'kept']));")).unwrap_err();
    assert_eq!(
        mixed.kind,
        meta_language::translation::diagnostics::ErrorKind::Type
    );
    assert!(mixed.message().contains("number") && mixed.message().contains("string"));
    let unresolved: meta_language::translation::surface::SProgram = serde_json::from_value(serde_json::json!({
        "language":"JavaScript", "items":[{"k":"fn", "name":"unknownType",
        "params":[{"name":"value", "type":{"kind":"option", "element":{"kind":"union", "alternatives":[{"kind":"float"},{"kind":"string"}]}, "absence":"both"}}],
        "ret":{"kind":"string"}, "body":{"k":"typeOf", "arg":{"k":"name", "path":["value"]}}}], "main":null,
    })).unwrap();
    let expected = Type::Option {
        element: Box::new(Type::Union {
            alternatives: vec![Type::Float, Type::String],
        }),
        absence: "both".to_owned(),
    };
    let error = check_program(&unresolved).unwrap_err();
    assert_eq!(
        error.kind,
        meta_language::translation::diagnostics::ErrorKind::Unsupported
    );
    assert_eq!(error.construct, Some(format!("typeof {}", expected.key())));
    assert!(
        error
            .message()
            .contains("type queries require a known JavaScript value type")
    );
}

#[test]
fn structural_optional_type_queries_round_trip_and_execute_as_one_complete_rust_program() {
    use super::issue_195_translation_pairs::{TemporaryDirectory, run};
    use meta_language::translation::emit_rust::emit_rust;
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
        encode_link_network_binary, encode_typed_program_links,
    };
    let source = "/** @param {{item:number}|null|undefined} value @returns {string} */function recordType(value){return typeof value;}\n/** @param {[number,string]|null|undefined} value @returns {string} */function tupleType(value){return typeof value;}\n/** @param {Map<string,number>|null|undefined} value @returns {string} */function mapType(value){return typeof value;}\n/** @param {Set<number>|null|undefined} value @returns {string} */function setType(value){return typeof value;}\nconsole.log('%s',recordType(null),recordType(undefined),recordType({item:2}));\nconsole.log('%s',tupleType(null),tupleType(undefined),tupleType([2,'kept']));\nconsole.log('%s',mapType(null),mapType(undefined),mapType(new Map([['kept',2]])));\nconsole.log('%s',setType(null),setType(undefined),setType(new Set([2])));\nfunction read(value){console.log('read');const n=value?.item??0;return value;}function describe(value){return typeof read(value);}console.log('%s',describe(null));console.log('%s',describe(undefined));console.log('%s',describe({item:2}));";
    let expected: &[u8] = b"object undefined object\nobject undefined object\nobject undefined object\nobject undefined object\nread\nobject\nread\nundefined\nread\nobject\n";
    let original = observe(source);
    assert!(
        original.status.success(),
        "{}",
        String::from_utf8_lossy(&original.stderr)
    );
    assert_eq!(original.stdout, expected);
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let javascript = emit_javascript(&program).unwrap().text;
    let observed = observe(&javascript);
    assert_eq!(observed.status.code(), original.status.code());
    assert_eq!(observed.stdout, original.stdout);
    assert_eq!(observed.stderr, original.stderr);
    let rust = emit_rust(&program).unwrap().text;
    let archive = encode_typed_program_links(&program).unwrap();
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
        assert_eq!(emit_javascript(&restored).unwrap().text, javascript);
        assert_eq!(emit_rust(&restored).unwrap().text, rust);
    }
    let directory = TemporaryDirectory::new("structural_optional_type_queries");
    let file = directory.0.join("program.rs");
    let executable = directory
        .0
        .join(format!("program{}", std::env::consts::EXE_SUFFIX));
    std::fs::write(&file, rust).unwrap();
    let compiled = run(Command::new("rustc")
        .arg("--edition=2024")
        .arg(&file)
        .arg("-o")
        .arg(&executable));
    assert!(compiled.ok, "{}", compiled.stderr);
    let result = run(&mut Command::new(&executable));
    assert!(result.ok, "{}", result.stderr);
    assert_eq!(result.stdout.as_bytes(), original.stdout);
    assert_eq!(result.stderr.as_bytes(), original.stderr);
}
