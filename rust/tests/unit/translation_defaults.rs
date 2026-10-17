use meta_language::program_translation::translate_program;
use meta_language::translation::javascript::parse_javascript;

const DEFAULTS: &str = r"function label(name, level = 'grammar-rule', depth = 2) {
  return `${name}:${level}:${depth}`;
}
/**
 * @param {string} term
 * @param {number[]} children
 * @returns {number}
 */
function size(term, children = []) {
  return children.length + (term === '' ? 0 : 1);
}
console.log('%s', label('a'), label('b', 'x'), label('c', 'y', 3), size('ab'), size('', [1, 2]));
";

#[test]
fn omitted_arguments_receive_defaults_in_every_target() {
    for target in ["Rust", "Lean", "Rocq"] {
        let translated = translate_program(DEFAULTS, "JavaScript", target).unwrap();
        assert_eq!(translated.diagnostic(), None, "{target}");
        assert!(translated.semantics().is_some(), "{target}");
    }
    let translated = translate_program(DEFAULTS, "JavaScript", "Rust").unwrap();
    for expected in [
        "pub fn label(name: String, level: String, depth: f64) -> String {",
        "crate::label(String::from(\"a\"), String::from(\"grammar-rule\"), 2f64)",
        "crate::label(String::from(\"b\"), String::from(\"x\"), 2f64)",
        "crate::size(String::from(\"ab\"), Vec::<f64>::new())",
    ] {
        assert!(translated.code().contains(expected), "{expected}");
    }
}

#[test]
fn defaults_refuse_parameter_references_and_required_parameters_after_defaults() {
    for (source, expected) in [
        (
            "function f(a = 1, b) { return b; }",
            "parameter b after a default",
        ),
        (
            "function f(a, b = a) { return b; }",
            "default reading parameter a",
        ),
    ] {
        let error = parse_javascript(source).unwrap_err();
        assert!(error.message().contains(expected), "{error}");
    }
}

#[test]
fn default_calls_resolve_names_outside_the_callers_local_scope() {
    let source = "function base() { return 7; }\nfunction chosen(value = base()) { return value; }\nfunction caller(base) { return chosen(); }\nconsole.log(caller(99));\n";
    let translated = translate_program(source, "JavaScript", "Rust").unwrap();
    assert_eq!(translated.diagnostic(), None);
    assert!(
        translated.code().contains("crate::chosen(crate::base())"),
        "{}",
        translated.code()
    );
}

use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::emit_rocq::emit_rocq;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::emit_rust::{
    RustEmissionOptions, emit_rust_with_array_consumption,
};
use meta_language::translation::rust::parse_rust;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links,
};
use std::fs;
use std::path::Path;
use std::process::Command;

fn javascript_output(code: &str) -> String {
    let output = std::process::Command::new("node")
        .args(["--input-type=module", "-e", code])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn omitted_arguments_fill_trailing_defaults_and_all_emitters_accept_them() {
    let source = "/** @param {number} n @param {number} amount @returns {number} */\nfunction add(n, amount = 2) { return n + amount; }\nconsole.log(add(3), add(3, 7));\n";
    let surface = parse_javascript(source).unwrap();
    let program = check_program(&surface).unwrap();
    assert_eq!(javascript_output(source), "5 10\n");
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        "5 10\n"
    );
    assert!(emit_rust(&program).unwrap().text.contains("2f64"));
    emit_lean(&program).unwrap();
    emit_rocq(&program).unwrap();
}

#[test]
fn defaults_reject_parameter_dependencies_and_nondefault_trailing_parameters() {
    for (source, reason) in [
        (
            "function f(a, b = a) { return b; }",
            "default reading parameter a",
        ),
        (
            "function f(a = 1, b) { return b; }",
            "parameter b after a default",
        ),
    ] {
        assert!(
            parse_javascript(source)
                .unwrap_err()
                .message()
                .contains(reason)
        );
    }
}

#[test]
fn unicode_escape_scalars_match_the_javascript_oracle_and_rust_frontend() {
    let source = "console.log('%s', '\\ud83d\\ude00\\u{e9}\\u00e9');\n";
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    assert_eq!(javascript_output(source), "😀éé\n");
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        "😀éé\n"
    );
    parse_rust("fn main() { println!(\"\\u{1f600}\\u{e9}\"); }").unwrap();
    for source in ["console.log('\\ud83d');", "console.log('\\u{110000}');"] {
        assert!(
            parse_javascript(source)
                .unwrap_err()
                .message()
                .contains("scalar values only")
        );
    }
    assert!(
        parse_javascript("console.log('\\uXYZZ');")
            .unwrap_err()
            .message()
            .contains("malformed unicode escape")
    );
}

#[test]
fn lexical_boundaries_keep_regular_expression_bodies_and_division_distinct() {
    use meta_language::translation::Language;
    use meta_language::translation::diagnostics::ErrorKind;
    use meta_language::translation::lexer::{TokenKind, tokenize};
    let fixtures: serde_json::Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/translation-lexical-boundaries.lino"
    ))
    .unwrap();
    for literal in fixtures["regexLiterals"].as_array().unwrap() {
        let literal = literal.as_str().unwrap();
        let source = format!("function matches(text) {{ return {literal}.test(text); }}");
        let tokens = tokenize(&source, Language::JavaScript).unwrap();
        assert!(
            tokens
                .tokens
                .iter()
                .any(|token| token.kind == TokenKind::RegularExpression && token.raw == literal),
            "{literal}"
        );
        let error = parse_javascript(&source).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Unsupported, "{literal}: {error}");
        assert_eq!(error.construct.as_deref(), Some("regular expression"));
    }
    for source in fixtures["divisionPrograms"].as_array().unwrap() {
        let source = source.as_str().unwrap();
        check_program(&parse_javascript(source).unwrap()).unwrap();
        assert!(
            !tokenize(source, Language::JavaScript)
                .unwrap()
                .tokens
                .iter()
                .any(|token| token.kind == TokenKind::RegularExpression)
        );
    }
    for source in fixtures["malformedRegex"].as_array().unwrap() {
        let error = tokenize(source.as_str().unwrap(), Language::JavaScript).unwrap_err();
        assert_eq!(error.kind, ErrorKind::Syntax);
        assert!(error.reason.contains("unterminated regular expression"));
    }
    for source in fixtures["controlRegexPrograms"].as_array().unwrap() {
        let tokens = tokenize(source.as_str().unwrap(), Language::JavaScript).unwrap();
        assert_eq!(
            tokens
                .tokens
                .iter()
                .filter(|token| token.kind == TokenKind::RegularExpression)
                .count(),
            1
        );
    }
    for source in fixtures["qualifiedDivisionPrograms"].as_array().unwrap() {
        let tokens = tokenize(source.as_str().unwrap(), Language::JavaScript).unwrap();
        assert!(
            !tokens
                .tokens
                .iter()
                .any(|token| token.kind == TokenKind::RegularExpression)
        );
    }
    for fixture in fixtures["templateBoundaries"].as_array().unwrap() {
        let source = fixture["source"].as_str().unwrap();
        let expression = fixture["expression"].as_str().unwrap();
        let tokens = tokenize(
            &format!("{source}; const following = 1;"),
            Language::JavaScript,
        )
        .unwrap();
        let template = &tokens.tokens[0];
        assert_eq!(template.raw, source);
        let substitution = template.parts[0].expression.as_ref().unwrap();
        assert_eq!(substitution.source, expression, "{source}");
        assert_eq!(template.parts[0].text, "head ");
        assert_eq!(template.parts[1].text, " tail");
        let encoded = meta_language::translation::lexer::Source::new(source);
        assert_eq!(
            encoded.slice(
                substitution.offset,
                substitution.offset + expression.encode_utf16().count()
            ),
            expression
        );
        assert!(tokens.tokens.iter().any(|token| token.value == "following"));
    }
    for source in fixtures["malformedTemplates"].as_array().unwrap() {
        assert_eq!(
            tokenize(source.as_str().unwrap(), Language::JavaScript)
                .unwrap_err()
                .kind,
            ErrorKind::Syntax
        );
    }
    let count = fixtures["repeatedRegex"]["count"].as_u64().unwrap();
    let literal = fixtures["repeatedRegex"]["literal"].as_str().unwrap();
    let source = (0..count)
        .map(|index| format!("const r{index} = {literal};"))
        .collect::<Vec<_>>()
        .join("\n");
    let encoded = meta_language::translation::lexer::Source::new(&source);
    let tokens = tokenize(&source, Language::JavaScript).unwrap();
    let literals: Vec<_> = tokens
        .tokens
        .iter()
        .filter(|token| token.kind == TokenKind::RegularExpression)
        .collect();
    assert_eq!(u64::try_from(literals.len()).unwrap(), count);
    for token in literals {
        assert_eq!(token.raw, literal);
        assert_eq!(encoded.slice(token.start, token.end), literal);
    }
    for fixture in fixtures["escapedLiterals"].as_array().unwrap() {
        let source = fixture["source"].as_str().unwrap();
        let expected = format!("{}\n", fixture["value"].as_str().unwrap());
        let program = format!("console.log({source});");
        let emitted =
            emit_javascript(&check_program(&parse_javascript(&program).unwrap()).unwrap())
                .unwrap()
                .text;
        assert_eq!(javascript_output(&program), expected);
        assert_eq!(javascript_output(&emitted), expected);
    }
    for source in fixtures["malformedEscapedLiterals"].as_array().unwrap() {
        assert_eq!(
            tokenize(source.as_str().unwrap(), Language::JavaScript)
                .unwrap_err()
                .kind,
            ErrorKind::Syntax
        );
    }
    for source in fixtures["unsupportedEscapedLiterals"].as_array().unwrap() {
        assert_eq!(
            tokenize(source.as_str().unwrap(), Language::JavaScript)
                .unwrap_err()
                .kind,
            ErrorKind::Unsupported
        );
    }
    let source = fixtures["templateBoundaryProgram"].as_str().unwrap();
    let expected = fixtures["templateBoundaryOutput"].as_str().unwrap();
    assert_eq!(javascript_output(source), expected);
    let emitted = emit_javascript(&check_program(&parse_javascript(source).unwrap()).unwrap())
        .unwrap()
        .text;
    assert_eq!(javascript_output(&emitted), expected);
    let source = (0..count)
        .map(|index| format!("const r{index} = `prefix ${{{literal}}} suffix`;"))
        .collect::<Vec<_>>()
        .join("\n");
    let tokens = tokenize(&source, Language::JavaScript).unwrap();
    let templates: Vec<_> = tokens
        .tokens
        .iter()
        .filter(|token| token.kind == TokenKind::Template)
        .collect();
    assert_eq!(u64::try_from(templates.len()).unwrap(), count);
    for template in templates {
        assert_eq!(
            template.parts[0].expression.as_ref().unwrap().source,
            literal
        );
    }
    let source = fixtures["templateProgram"].as_str().unwrap();
    emit_javascript(&check_program(&parse_javascript(source).unwrap()).unwrap()).unwrap();
    assert!(
        tokenize(r"`\01`", Language::JavaScript)
            .unwrap_err()
            .reason
            .contains("octal")
    );
}

#[test]
fn optional_documentation_names_preserve_types_and_runtime_defaults() {
    let source = "/** @param {number} [value=99] @returns {number} */\nfunction add(value = 2) { return value + 1; }\nconsole.log(add(), add(7));\n";
    let surface = parse_javascript(source).unwrap();
    let program = check_program(&surface).unwrap();
    assert_eq!(javascript_output(source), "3 8\n");
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        "3 8\n"
    );
    let rust = emit_rust(&program).unwrap().text;
    assert!(rust.contains("crate::add(2f64)"), "{rust}");
    assert!(rust.contains("crate::add(7f64)"), "{rust}");
    for target in ["Rust", "Lean", "Rocq"] {
        let translated = translate_program(source, "JavaScript", target).unwrap();
        assert_eq!(translated.diagnostic(), None, "{target}");
    }
    // The retained original source has no runtime initializer. Its documented
    // optional parameter receives undefined, whose numeric observation is NaN.
    let source = "/** @param {number} [value=99] @returns {number} */\nfunction add(value) { return value + 1; }\nconsole.log(add());\n";
    let surface = parse_javascript(source).unwrap();
    let meta_language::translation::surface::SItem::Fn(function) = &surface.items[0] else {
        panic!("the original source owns add");
    };
    assert!(function.params[0].optional);
    assert_eq!(
        function.params[0].ty,
        Some(meta_language::translation::types::Type::Option {
            element: Box::new(meta_language::translation::types::Type::Float),
            absence: "undefined".to_owned(),
        })
    );
    assert!(
        matches!(&function.params[0].default_value.as_ref().unwrap().node,
        meta_language::translation::surface::SNode::None { absence } if absence == "undefined")
    );
    let program = check_program(&surface).unwrap();
    assert_eq!(javascript_output(source), "NaN\n");
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        "NaN\n"
    );
    assert_eq!(
        translate_program(source, "JavaScript", "Rust")
            .unwrap()
            .diagnostic(),
        None
    );
    let source = "/** @param {number} [other] */\nfunction identity(value) { return value; }\n";
    let translated = translate_program(source, "JavaScript", "Rust").unwrap();
    assert!(
        translated
            .diagnostic()
            .unwrap()
            .message
            .contains("other is not a parameter")
    );
}

#[test]
fn type_queries_preserve_bound_value_names_without_discarding_evaluation() {
    let source = "/** @param {number} value @returns {string} */\nfunction numberType(value) { return typeof value; }\n/** @param {bigint} value @returns {string} */\nfunction integerType(value) { return typeof value; }\n/** @param {string} value @returns {string} */\nfunction stringType(value) { return typeof value; }\n/** @param {boolean} value @returns {string} */\nfunction booleanType(value) { return typeof value; }\n/** @param {number[]} value @returns {string} */\nfunction arrayType(value) { return typeof value; }\nconsole.log('%s', numberType(7), integerType(7n), stringType('ready'), booleanType(false), arrayType([]), typeof 7, typeof 7n, typeof false, typeof 'ready');\n";
    let expected = "number bigint string boolean object number bigint boolean string\n";
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    assert_eq!(javascript_output(source), expected);
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        expected
    );
    for target in ["Rust", "Lean", "Rocq"] {
        assert_eq!(
            translate_program(source, "JavaScript", target)
                .unwrap()
                .diagnostic(),
            None
        );
    }
    for (source, reason) in [
        (
            "function fail() { throw new Error('preserve'); } console.log(typeof fail());",
            "typeof operand",
        ),
        ("console.log(typeof [1]);", "typeof operand"),
    ] {
        for target in ["Rust", "Lean", "Rocq"] {
            let translated = translate_program(source, "JavaScript", target).unwrap();
            let Some(diagnostic) = translated.diagnostic() else {
                panic!("{target}: expected {reason} rejection for {source}")
            };
            assert!(
                diagnostic.message.contains(reason),
                "{target}: {source}: {}",
                diagnostic.message
            );
        }
    }
    let source = "function answer() { return 7; } console.log(typeof answer);";
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    assert_eq!(javascript_output(source), "function\n");
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        "function\n"
    );
    for target in ["Rust", "Lean", "Rocq"] {
        assert_eq!(
            translate_program(source, "JavaScript", target)
                .unwrap()
                .diagnostic(),
            None,
            "{target}: the exact original function reference type query"
        );
    }
    let structural = "/** @param {{item:number}} value @returns {string} */\nfunction recordType(value) { return typeof value; }\n/** @param {[number,string]} value @returns {string} */\nfunction tupleType(value) { return typeof value; }\n/** @param {Map<string,number>} value @returns {string} */\nfunction mapType(value) { return typeof value; }\n/** @param {Set<number>} value @returns {string} */\nfunction setType(value) { return typeof value; }\nconsole.log('%s', recordType({item:2}), tupleType([2,'kept']), mapType(new Map([['kept',2]])), setType(new Set([2])));";
    let expected = "object object object object\n";
    let program = check_program(&parse_javascript(structural).unwrap()).unwrap();
    assert_eq!(javascript_output(structural), expected);
    for (name, kind) in [
        ("recordType", "record"),
        ("tupleType", "tuple"),
        ("mapType", "map"),
        ("setType", "set"),
    ] {
        let Some(meta_language::translation::ir::Decl::Fn(function)) = program.declaration(name)
        else {
            panic!("the actual structural function remains declared")
        };
        assert_eq!(function.params[0].ty.kind(), kind);
        assert!(
            matches!(&function.body.node, meta_language::translation::ir::Node::Lit {
            value: meta_language::translation::ir::LitValue::Text(value)
        } if value == "object")
        );
    }
    for emitted in [
        emit_javascript(&program).unwrap(),
        emit_rust(&program).unwrap(),
        emit_lean(&program).unwrap(),
        emit_rocq(&program).unwrap(),
    ] {
        let mut names = emitted
            .mappings
            .iter()
            .filter(|mapping| mapping.kind == "function")
            .map(|mapping| mapping.source.as_str())
            .collect::<Vec<_>>();
        names.sort_unstable();
        assert_eq!(names, ["mapType", "recordType", "setType", "tupleType"]);
        assert!(emitted.text.contains("object"));
        if emitted.language == meta_language::translation::Language::JavaScript {
            assert_eq!(javascript_output(&emitted.text), expected);
        }
    }
}

fn parameter_default_contract(source: &str, native_javascript: &str) -> serde_json::Value {
    let script = r#"
import {format} from 'node:util';
import vm from 'node:vm';
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
function observe(source){
 const lines=[];let error=null;
 try{vm.runInNewContext(source,{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});}
 catch(failure){error=failure.message;}
 return {output:lines.length?lines.join('\n')+'\n':'',error};
}
const surface=parseJavaScript(process.argv[1]),checked=checkProgram(surface);
const javascript=emitJavaScript(checked).text,rust=emitRust(checked,{arrayConsumption:true}).text;
console.log(JSON.stringify({surface,javascript,rust,observations:[observe(process.argv[1]),observe(process.argv[2]),observe(javascript)]}));
"#;
    let result = run(Command::new("node")
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
        .args([
            "--input-type=module",
            "-e",
            script,
            source,
            native_javascript,
        ]));
    assert!(result.ok, "{}", result.stderr);
    serde_json::from_str(&result.stdout).unwrap()
}

fn assert_parameter_default_behavior(source: &str, output: &str, error: Option<&str>, name: &str) {
    let surface = parse_javascript(source).unwrap();
    let program = check_program(&surface).unwrap();
    let javascript = emit_javascript(&program).unwrap().text;
    let rust = emit_rust_with_array_consumption(&program, RustEmissionOptions::default())
        .unwrap()
        .text;
    let contract = parameter_default_contract(source, &javascript);
    assert_eq!(
        serde_json::to_value(&surface).unwrap(),
        contract["surface"],
        "{name}: actual inferred source"
    );
    assert_eq!(
        javascript,
        contract["javascript"].as_str().unwrap(),
        "{name}: JavaScript bytes"
    );
    assert_eq!(
        rust,
        contract["rust"].as_str().unwrap(),
        "{name}: Rust bytes"
    );
    let expected = serde_json::json!({"output": output, "error": error});
    for observation in contract["observations"].as_array().unwrap() {
        assert_eq!(
            observation, &expected,
            "{name}: original and both checked JavaScript executions"
        );
    }
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
        assert_eq!(restored, program, "{name}: source-free checked bindings");
        encode_typed_program_links(&restored).unwrap();
        assert_eq!(
            emit_javascript(&restored).unwrap().text,
            javascript,
            "{name}: source-free JavaScript bytes"
        );
        assert_eq!(
            emit_rust_with_array_consumption(&restored, RustEmissionOptions::default())
                .unwrap()
                .text,
            rust,
            "{name}: source-free Rust bytes"
        );
    }
    let directory = TemporaryDirectory::new(name);
    let file = directory.0.join("program.rs");
    let executable = directory
        .0
        .join(format!("program{}", std::env::consts::EXE_SUFFIX));
    fs::write(&file, &rust).unwrap();
    let compiled = run(Command::new("rustc")
        .arg("--edition=2024")
        .arg(&file)
        .arg("-o")
        .arg(&executable));
    assert!(compiled.ok, "{name}: {}", compiled.stderr);
    let result = run(&mut Command::new(&executable));
    assert_eq!(result.stdout, output, "{name}: actual emitted Rust output");
    if let Some(message) = error {
        assert!(!result.ok, "{name}: the original abort remains observable");
        assert!(result.stderr.contains(message), "{name}: {}", result.stderr);
    } else {
        assert!(result.ok, "{name}: {}", result.stderr);
    }
}

#[test]
fn parameter_defaults_preserve_actual_argument_order_lazy_scope_and_source_free_links() {
    for (name, source, expected) in [
        (
            "default_argument_order",
            "/** @returns {number} */function fallback(){console.log(\"default\");return 2;}\n/** @returns {number} */function later(){console.log(\"later\");return 7;}\nfunction chosen(value=fallback(),other=0){return value+other;}\nconsole.log(chosen(undefined,later()));\n",
            "later\ndefault\n9\n",
        ),
        (
            "default_lazy_scope",
            "/** @returns {number} */function fallback(){console.log(\"default\");return 2;}\n/** @param {boolean} present @returns {number} */function maybe(present){console.log(\"maybe\");if(present)return 7;}\n/** @returns {number} */function later(){console.log(\"later\");return 3;}\n/** @param {number} value @param {number} other @returns {number} */function chosen(value=fallback(),other=0){return value+other;}\nfunction caller(fallback){return chosen(maybe(false),later());}\nconsole.log(chosen(maybe(true),later()));console.log(caller(99));console.log(chosen());\n",
            "maybe\nlater\n10\nmaybe\nlater\ndefault\n5\ndefault\n2\n",
        ),
        (
            "default_deferred_inference",
            "function chosen(value=2,other=0){return value+other;}\nfunction caller(){return chosen(maybe(false),later());}\nfunction maybe(present){console.log(\"maybe\");if(present)return 7;}\nfunction later(){console.log(\"later\");return 3;}\nconsole.log(caller());\n",
            "maybe\nlater\n5\n",
        ),
        (
            "default_pure_literal",
            "/** @returns {number} */function later(){console.log(\"later\");return 7;}\nfunction chosen(value=2,other=0){return value+other;}\nconsole.log(chosen(undefined,later()));\n",
            "later\n9\n",
        ),
    ] {
        assert_parameter_default_behavior(source, expected, None, name);
    }
}

#[test]
fn parameter_default_aborts_and_guards_follow_original_execution_order() {
    for (name, source, expected, message) in [
        (
            "default_argument_abort",
            "/** @returns {number} */function fallback(){console.log(\"default\");return 2;}\n/** @returns {number} */function stopped(){console.log(\"argument\");throw new Error(\"argument stopped\");}\nfunction chosen(value=fallback(),other=0){return value+other;}\nconsole.log(chosen(undefined,stopped()));\n",
            "argument\n",
            "argument stopped",
        ),
        (
            "default_initializer_abort",
            "/** @returns {number} */function first(){console.log(\"first default\");throw new Error(\"default stopped\");}\n/** @returns {number} */function second(){console.log(\"second default\");return 3;}\n/** @returns {number} */function later(){console.log(\"later\");return 7;}\nfunction chosen(value=first(),other=second(),last=0){return value+other+last;}\nconsole.log(chosen(undefined,undefined,later()));\n",
            "later\nfirst default\n",
            "default stopped",
        ),
        (
            "default_natural_guard",
            "function fallback(){console.log(\"default\");return 2n;}\nfunction later(){console.log(\"later\");return -1n;}\nfunction chosen(value=fallback(),other=0n){if(other<0n)throw new RangeError(\"negative\");return value+other;}\nconsole.log(chosen(undefined,later()));\n",
            "later\ndefault\n",
            "negative",
        ),
    ] {
        assert_parameter_default_behavior(source, expected, Some(message), name);
    }
}
