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
}
