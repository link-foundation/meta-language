//! Actual record programs, original binding identities and ordered union headers.
//! Native compilation and process observations run in CI, never during editing.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::emit_rocq::emit_rocq;
use meta_language::translation::ir::{Decl, Expr, LitValue, Node, Program};
use meta_language::translation::types::Type;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, TypedProgramLinks, create_typed_program_links,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, encode_typed_program_links,
};
use serde_json::Value;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

const TARGETS: [&str; 4] = ["JavaScript", "Rust", "Lean", "Rocq"];
const DECLARED: &str = "/** @param {{z:bigint,a:bigint}} value @returns {bigint} */function select(value){return value.z;}function choose(){return select({a:2n,z:3n});}console.log(\"%s\",String(choose()));";

fn portable_emission(network: &LinkNetwork, root: LinkId, target: &str) -> Result<String, String> {
    match target {
        "JavaScript" | "Rust" => emit_typed_program_links(network, root, target)
            .map(|emitted| emitted.code)
            .map_err(|failure| failure.to_string()),
        "Lean" | "Rocq" => {
            let program =
                decode_typed_program_links(network, root).map_err(|failure| failure.to_string())?;
            let emitted = if target == "Lean" {
                emit_lean(&program)
            } else {
                emit_rocq(&program)
            };
            emitted
                .map(|emitted| emitted.text)
                .map_err(|failure| failure.to_string())
        }
        _ => unreachable!("the test names every existing standard target explicitly"),
    }
}

fn archives(packet: &TypedProgramLinks) -> [LinkNetwork; 2] {
    [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}

fn javascript_contract(source: &str) -> Value {
    let script = r#"
import {createTypedProgramLinks,decodeTypedProgramLinks,emitTypedProgramLinks} from './js/src/index.js';
import {emitLean} from './js/src/translation/emit-lean.js';
import {emitRocq} from './js/src/translation/emit-rocq.js';
import vm from 'node:vm';
import {format} from 'node:util';
const source=process.argv[1];
const lines=[];
vm.runInNewContext(source,{console:{log:(...values)=>lines.push(format(...values))}},{timeout:1000});
const packet=createTypedProgramLinks(source,'JavaScript');
const codes={};
for(const target of ['JavaScript','Rust','Lean','Rocq']){
  try{
    let code;
    if(target==='Lean'||target==='Rocq'){
      const program=decodeTypedProgramLinks(packet.network,packet.root);
      code=(target==='Lean'?emitLean(program):emitRocq(program)).text;
    }else{
      code=emitTypedProgramLinks(packet.network,packet.root,target).code;
    }
    codes[target]={code};
  }
  catch(error){codes[target]={error:error.message};}
}
console.log(JSON.stringify({lines,codes}));
"#;
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let observed = run(Command::new("node").current_dir(directory).args([
        "--input-type=module",
        "-e",
        script,
        source,
    ]));
    assert!(observed.ok, "{}", observed.stderr);
    serde_json::from_str(&observed.stdout).unwrap()
}

fn observe_javascript(code: &str, expected: &str) {
    let observed = run(Command::new("node").args(["-e", code]));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, expected);
}

fn observe_threaded(program: &Program) -> Value {
    let script = r#"
import {observeThreadedOptionalProgram} from './js/tests/helpers/translation-output-observer.js';
const program=JSON.parse(process.argv[1]);
program.declarations=new Map(program.declarations.map(entry=>[entry.fullName,entry]));
console.log(JSON.stringify(observeThreadedOptionalProgram(program)));
"#;
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let encoded = serde_json::to_string(program).unwrap();
    let observed = run(Command::new("node").current_dir(directory).args([
        "--input-type=module",
        "-e",
        script,
        &encoded,
    ]));
    assert!(observed.ok, "{}", observed.stderr);
    serde_json::from_str(&observed.stdout).unwrap()
}

fn function<'a>(program: &'a Program, name: &str) -> &'a meta_language::translation::ir::FnDecl {
    let Some(Decl::Fn(function)) = program.declaration(name) else {
        panic!("the actual function declaration exists")
    };
    function
}

#[test]
fn complete_record_programs_keep_overwrites_spreads_and_every_target_byte_in_both_archives() {
    let prefix = "function mark(label,value){console.log(label);return value;}";
    let cases = [
        (
            format!(
                "{prefix}function build(){{const entry={{z:mark(\"first\",1n),a:mark(\"second\",2n),z:mark(\"third\",3n)}};return entry;}}const entry=build();console.log(\"%s\",String(entry.z));console.log(\"%s\",String(entry.a));"
            ),
            "first\nsecond\nthird\n3\n2\n",
        ),
        (
            format!(
                "{prefix}function build(){{const base={{b:mark(\"base-b\",2n),a:mark(\"base-a\",1n)}};const entry={{...base,b:mark(\"override-b\",4n)}};return entry;}}const entry=build();console.log(\"%s\",String(entry.a));console.log(\"%s\",String(entry.b));"
            ),
            "base-b\nbase-a\noverride-b\n1\n4\n",
        ),
        (
            format!(
                "{prefix}function copy(base){{return {{...base,b:mark(\"override-b\",4n)}};}}function build(){{const base={{b:mark(\"base-b\",2n),a:mark(\"base-a\",1n)}};return copy(base);}}const entry=build();console.log(\"%s\",String(entry.a));console.log(\"%s\",String(entry.b));"
            ),
            "base-b\nbase-a\noverride-b\n1\n4\n",
        ),
    ];
    for (source, expected) in cases {
        let original = javascript_contract(&source);
        assert_eq!(
            original["lines"]
                .as_array()
                .unwrap()
                .iter()
                .map(|line| line.as_str().unwrap())
                .collect::<Vec<_>>()
                .join("\n")
                + "\n",
            expected
        );
        let packet = create_typed_program_links(&source, "JavaScript").unwrap();
        for network in archives(&packet) {
            let program = decode_typed_program_links(&network, packet.root).unwrap();
            assert!(program.declaration("build").is_some());
            for target in TARGETS {
                let emitted = portable_emission(&network, packet.root, target).unwrap();
                assert_eq!(
                    emitted,
                    original["codes"][target]["code"].as_str().unwrap(),
                    "{target}"
                );
                if target == "JavaScript" {
                    observe_javascript(&emitted, expected);
                }
            }
        }
    }
}

fn replace_three(expression: &Expr, count: &mut usize) -> Expr {
    let mut copy = expression.map_children(&mut |child| replace_three(child, count));
    if copy.ty == Type::Int
        && let Node::Lit {
            value: LitValue::Text(value),
        } = &mut copy.node
        && value == "3"
    {
        *value = "7".to_owned();
        *count += 1;
    }
    copy
}

#[test]
fn declared_record_fields_retain_parameter_and_let_bindings_after_source_free_value_edit() {
    let packet = create_typed_program_links(DECLARED, "JavaScript").unwrap();
    for network in archives(&packet) {
        let mut program = decode_typed_program_links(&network, packet.root).unwrap();
        let Type::Record { fields, .. } = &function(&program, "select").params[0].ty else {
            panic!("actual declared record parameter")
        };
        assert_eq!(
            fields
                .iter()
                .map(|field| (field.name.as_str(), field.ty.kind()))
                .collect::<Vec<_>>(),
            [("a", "int"), ("z", "int")]
        );
        let mut count = 0;
        for declaration in &mut program.declarations {
            if let Decl::Fn(function) = declaration {
                function.body = replace_three(&function.body, &mut count);
            }
        }
        assert_eq!(count, 1);
        let edited = encode_typed_program_links(&program).unwrap();
        let expected = javascript_contract(&DECLARED.replace("z:3n", "z:7n"));
        for restored in archives(&edited) {
            let checked = decode_typed_program_links(&restored, edited.root).unwrap();
            assert_eq!(
                function(&checked, "select").params[0].ty,
                function(&program, "select").params[0].ty
            );
            for target in TARGETS {
                let emitted = portable_emission(&restored, edited.root, target).unwrap();
                assert_eq!(emitted, expected["codes"][target]["code"].as_str().unwrap());
                if target == "JavaScript" {
                    observe_javascript(&emitted, "7\n");
                }
            }
        }
    }
}

#[test]
fn complete_emitted_rust_for_a_declared_record_executes_its_actual_main() {
    let packet = create_typed_program_links(DECLARED, "JavaScript").unwrap();
    let emitted = emit_typed_program_links(&packet.network, packet.root, "Rust").unwrap();
    let directory = TemporaryDirectory::new("declared-record-rust");
    let source = directory.0.join("program.rs");
    let executable = directory.0.join("program");
    fs::write(&source, &emitted.code).unwrap();
    let compiled = run(Command::new("rustc")
        .args(["--edition", "2024"])
        .arg(&source)
        .arg("-o")
        .arg(&executable));
    assert!(compiled.ok, "{}", compiled.stderr);
    assert_eq!(fs::read_to_string(&source).unwrap(), emitted.code);
    let observed = run(&mut Command::new(executable));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, "3\n");
}

#[test]
fn ordered_union_headers_keep_exact_target_refusal_before_the_independent_frozen_body() {
    let source = "/** @typedef {{ $: \"one\" } | { $: \"two\" }} Tag */\n/** @param {bigint | string} unused @returns {Tag} */function make(unused){return Object.freeze({$:\"one\"});}console.log(\"ready\");";
    let expected = javascript_contract(source);
    let packet = create_typed_program_links(source, "JavaScript").unwrap();
    for network in archives(&packet) {
        let program = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(
            function(&program, "make").params[0].ty.key(),
            "union:[int,string]"
        );
        observe_javascript(
            &emit_typed_program_links(&network, packet.root, "JavaScript")
                .unwrap()
                .code,
            "ready\n",
        );
        for target in ["Rust", "Lean", "Rocq"] {
            let message = format!("no {target} type for union");
            assert_eq!(
                expected["codes"][target]["error"].as_str().unwrap(),
                message
            );
            assert_eq!(
                portable_emission(&network, packet.root, target).unwrap_err(),
                message
            );
        }
    }
    let ordinary = create_typed_program_links(
        &source.replace("@param {bigint | string}", "@param {bigint}"),
        "JavaScript",
    )
    .unwrap();
    for target in ["Rust", "Lean", "Rocq"] {
        assert!(
            portable_emission(&ordinary.network, ordinary.root, target)
                .unwrap_err()
                .contains("explicit data object freeze")
        );
    }
}

#[test]
fn nullable_source_type_folds_preserve_duplicate_missing_tokens_and_archive_absence() {
    for (spelling, absence, argument) in [
        ("bigint|null", "null", "null"),
        ("undefined|bigint", "undefined", "undefined"),
        ("bigint|null|null", "both", "null"),
        ("?bigint", "null", "null"),
    ] {
        let source = format!(
            "/** @param {{{spelling}}} unused @returns {{bigint}} */function choose(unused){{return 1n;}}console.log(String(choose({argument}))); "
        );
        let expected = javascript_contract(&source);
        let packet = create_typed_program_links(&source, "JavaScript").unwrap();
        for network in archives(&packet) {
            let program = decode_typed_program_links(&network, packet.root).unwrap();
            let Type::Option {
                absence: actual,
                element,
            } = &function(&program, "choose").params[0].ty
            else {
                panic!("the actual shared nullable source plan constructs Option")
            };
            assert_eq!(actual, absence);
            assert_eq!(element.as_ref(), &Type::Int);
            let emitted = emit_typed_program_links(&network, packet.root, "JavaScript").unwrap();
            assert_eq!(
                emitted.code,
                expected["codes"]["JavaScript"]["code"].as_str().unwrap()
            );
            observe_javascript(&emitted.code, "1\n");
        }
    }
}

fn move_record_calls(expression: &Expr, bindings: &mut HashMap<String, Expr>) -> Expr {
    let mut source = expression.clone();
    if let Node::Let { name, value, .. } = &mut source.node
        && let Node::Call { func, args } = &value.node
        && func == "report"
    {
        let scalar = args[0].clone();
        bindings.insert(name.clone(), value.as_ref().clone());
        **value = scalar;
    }
    let mut copy = source.map_children(&mut |child| move_record_calls(child, bindings));
    if let Node::Record { entries } = &mut copy.node {
        assert_eq!(
            entries
                .iter()
                .map(|entry| entry.name.as_str())
                .collect::<Vec<_>>(),
            ["a", "z"]
        );
        for entry in entries {
            let Node::Var { name } = &entry.value.node else {
                panic!("actual checked binding value")
            };
            entry.value = bindings.get(name).unwrap().clone();
            let Node::Call { func, .. } = &entry.value.node else {
                panic!("actual original call")
            };
            assert_eq!(func, "report");
        }
    }
    copy
}

#[test]
fn source_free_record_descriptor_calls_preserve_actual_targets_and_threaded_output_order() {
    let source = "function report(value){console.log(\"%s\",String(value));return value;}function choose(){return {z:report(2n),a:report(1n)}.z;}console.log(\"%s\",String(choose()));";
    let packet = create_typed_program_links(source, "JavaScript").unwrap();
    let mut program = decode_typed_program_links(&packet.network, packet.root).unwrap();
    let mut bindings = HashMap::new();
    for declaration in &mut program.declarations {
        if let Decl::Fn(function) = declaration
            && function.full_name == "choose"
        {
            function.body = move_record_calls(&function.body, &mut bindings);
        }
    }
    assert_eq!(bindings.len(), 2);
    let edited = encode_typed_program_links(&program).unwrap();
    for network in archives(&edited) {
        let restored = decode_typed_program_links(&network, edited.root).unwrap();
        let threaded = meta_language::translation::output::thread_output(&restored).unwrap();
        assert!(threaded.output_threaded);
        assert_eq!(
            observe_threaded(&threaded),
            serde_json::json!({"lines":["1","2","2"],"error":""})
        );
        for target in TARGETS {
            let emitted = portable_emission(&network, edited.root, target).unwrap();
            assert!(!emitted.is_empty());
            if target == "JavaScript" {
                observe_javascript(&emitted, "1\n2\n2\n");
            }
        }
    }
}
