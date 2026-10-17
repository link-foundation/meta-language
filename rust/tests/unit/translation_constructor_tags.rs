use meta_language::translation::Language;
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::ir::{Decl, Node, Program};
use meta_language::translation::javascript::parse_javascript;
use serde_json::Value;
use std::process::Command;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

fn rename_tag(value: &mut Value, tag: &str) {
    match value {
        Value::Array(children) => children.iter_mut().for_each(|child| rename_tag(child, tag)),
        Value::Object(fields) => {
            if let Some(Value::Array(ctors)) = fields.get_mut("ctors") {
                for ctor in ctors {
                    if ctor["name"] == "one" {
                        ctor["name"] = Value::String(tag.to_owned());
                    }
                }
            }
            if fields.get("ctor").is_some_and(|name| name == "one") {
                fields.insert("ctor".to_owned(), Value::String(tag.to_owned()));
            }
            for child in fields.values_mut() {
                rename_tag(child, tag);
            }
        }
        _ => {}
    }
}

fn rename_field(value: &mut Value, name: &str) {
    match value {
        Value::Array(children) => children
            .iter_mut()
            .for_each(|child| rename_field(child, name)),
        Value::Object(fields) => {
            if let Some(Value::Array(constructors)) = fields.get_mut("ctors") {
                for ctor in constructors {
                    for field in ctor["fields"].as_array_mut().unwrap() {
                        if field["name"] == "payload" {
                            field["name"] = Value::String(name.to_owned());
                        }
                    }
                }
            }
            for child in fields.values_mut() {
                rename_field(child, name);
            }
        }
        _ => {}
    }
}

#[test]
fn constructor_tag_strings_keep_their_data_value_in_javascript_output() {
    let fixture: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/translation-constructor-tags.lino"
    ))
    .unwrap();
    let source = "/** @typedef {{ $: 'one' } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return { $: 'one' }; }\n/** @param {Tag} value @returns {number} */ function read(value) { if (value.$ === 'one') return 1; return 2; }";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    for tag in fixture["tags"].as_array().unwrap() {
        let tag = tag.as_str().unwrap();
        let mut graph = serde_json::to_value(&checked).unwrap();
        rename_tag(&mut graph, tag);
        let program: Program = serde_json::from_value(graph).unwrap();
        let emitted = emit_javascript(&program).unwrap();
        let script = format!(
            "globalThis.injected = false;\n{}\nconsole.log(JSON.stringify({{ tag: make().$, selected: read(make()), other: read({{ $: 'two' }}), injected: globalThis.injected }}));",
            emitted.text
        );
        let output = Command::new("node")
            .args(["--input-type=module", "-e", &script])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            actual,
            serde_json::json!({ "tag": tag, "selected": 1, "other": 2, "injected": false })
        );
    }
}

#[test]
fn constructor_fields_preserve_exact_own_property_names_in_javascript_output() {
    let fixture: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/translation-constructor-tags.lino"
    ))
    .unwrap();
    let source = "/** @typedef {{ $: 'one', payload: number } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return { $: 'one', payload: 7 }; }\n/** @param {Tag} value @returns {number} */ function read(value) { if (value.$ === 'one') return value.payload; return 2; }";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    for name in fixture["fields"].as_array().unwrap() {
        let name = name.as_str().unwrap();
        let mut graph = serde_json::to_value(&checked).unwrap();
        rename_field(&mut graph, name);
        let program: Program = serde_json::from_value(graph).unwrap();
        let emitted = emit_javascript(&program).unwrap();
        let script = format!(
            "{}\nconst field = {}; const item = make(); const supplied = {{ $: 'one', [field]: 9 }}; console.log(JSON.stringify([Object.hasOwn(item, field), item[field], read(item), read(supplied), Object.getPrototypeOf(item) === Object.prototype]));",
            emitted.text,
            serde_json::to_string(name).unwrap()
        );
        let output = Command::new("node")
            .args(["--input-type=module", "-e", &script])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(actual, serde_json::json!([true, 7, 7, 9, true]), "{name}");
    }
}

#[test]
fn constructor_immutability_preserves_its_source_model_and_checked_marker() {
    let fixture: Value = super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/translation-constructor-tags.lino"
    ))
    .unwrap();
    let source = "/** @typedef {{ $: 'one' } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return { $: 'one' }; }";
    let checked = check_program(&parse_javascript(source).unwrap()).unwrap();
    for model in fixture["models"].as_array().unwrap() {
        let mut program = checked.clone();
        program.source_language = Language::parse(model["language"].as_str().unwrap()).unwrap();
        let make = program
            .declarations
            .iter_mut()
            .find_map(|declaration| match declaration {
                Decl::Fn(function) if function.name == "make" => Some(function),
                _ => None,
            })
            .unwrap();
        let Node::Ctor { frozen, .. } = &mut make.body.node else {
            panic!("the source returns a constructor");
        };
        *frozen = model["frozen"].as_bool().unwrap();
        let serialized = serde_json::to_value(&program).unwrap();
        let restored: Program = serde_json::from_value(serialized).unwrap();
        let emitted = emit_javascript(&restored).unwrap();
        let script = format!(
            "{}\nconst item = make(); const descriptor = Object.getOwnPropertyDescriptor(item, '$'); console.log(JSON.stringify([Object.isFrozen(item), Object.isExtensible(item), descriptor.writable, descriptor.configurable, Reflect.set(item, '$', 'two')]));",
            emitted.text
        );
        let output = Command::new("node")
            .args(["--input-type=module", "-e", &script])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            actual, model["descriptors"],
            "{}:{}",
            model["language"], model["frozen"]
        );
    }
    let explicit = "/** @typedef {{ $: 'one' } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return Object.freeze({ $: 'one' }); }";
    let checked = check_program(&parse_javascript(explicit).unwrap()).unwrap();
    let make = checked
        .declarations
        .iter()
        .find_map(|declaration| match declaration {
            Decl::Fn(function) if function.name == "make" => Some(function),
            _ => None,
        })
        .unwrap();
    assert!(matches!(make.body.node, Node::Ctor { frozen: true, .. }));
    for target in ["Rust", "Lean", "Rocq"] {
        let result = meta_language::translate_program(explicit, "JavaScript", target).unwrap();
        let diagnostic = result.diagnostic().unwrap();
        assert_eq!(diagnostic.kind, "unsupported");
        assert_eq!(
            diagnostic
                .message
                .split_once(": ")
                .map(|(construct, _)| construct),
            Some("explicit data object freeze")
        );
    }
}

fn terminal_tag_guard_contract(source: &str, native_javascript: &str) -> Value {
    let script = r#"
import {format} from 'node:util';
import vm from 'node:vm';
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from './js/src/typed-program-links.js';
import {recheckLinkedTypedProgram} from './js/src/typed-program-checked-projection.js';
import {LinkNetwork} from './js/src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from './js/src/link-network-binary.js';
function observe(source){const output=[];vm.runInNewContext(source,{console:{log:(...values)=>output.push(format(...values))}},{timeout:1000});return output.join('\n')+'\n';}
const surface=parseJavaScript(process.argv[1]),checked=checkProgram(surface);
const javascript=emitJavaScript(checked).text,rust=emitRust(checked,{arrayConsumption:true}).text;
const packet=encodeTypedProgramLinks(checked),archives=[];
for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
 const program=decodeTypedProgramLinks(network,packet.root);
 recheckLinkedTypedProgram(program,{wholeProgram:true});
 archives.push({javascript:emitJavaScript(program).text,rust:emitRust(program,{arrayConsumption:true}).text,output:observe(emitJavaScript(program).text)});
}
console.log(JSON.stringify({surface,javascript,rust,archives,observations:[observe(process.argv[1]),observe(process.argv[2]),observe(javascript)]}));
"#;
    let output = run(Command::new("node")
        .current_dir(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap(),
        )
        .args([
            "--input-type=module",
            "-e",
            script,
            source,
            native_javascript,
        ]));
    assert!(output.ok, "{}", output.stderr);
    serde_json::from_str(&output.stdout).unwrap()
}

#[test]
fn terminal_tag_guard_sequences_retain_all_exclusions_scopes_effects_and_source_free_archives() {
    use meta_language::translation::emit_rust::{
        RustEmissionOptions, emit_rust_with_array_consumption,
    };
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
        encode_link_network_binary, encode_typed_program_links,
    };
    let declarations = "/** @typedef {{ $: 'missing' } | { $: 'bad' } | { $: 'value', code: number, end: number }} GuardState */\n/** @param {number} n @returns {GuardState} */function make(n){if(n===0)return {$:'missing'};if(n===1)return {$:'bad'};return {$:'value',code:3,end:4};}\n";
    for (index, (body, expected)) in [
        ("if(value.$==='missing')return -1;if(value.$==='bad')return -2;return value.code+value.end;", "-1\n-2\n7\n"),
        ("if(value.$==='bad')return -2;if(value.$==='missing')return -1;return value.end+value.code;", "-1\n-2\n7\n"),
        ("if(value.$==='missing'){const value=99;return value;}if(value.$==='bad')return -2;return value.code+value.end;", "99\n-2\n7\n"),
        ("let count=0;if(value.$==='missing'){console.log('missing');return -1;}if(value.$==='bad'){console.log('bad');return -2;}count+=value.code;return count+value.end;", "missing\n-1\nbad\n-2\n7\n"),
        ("if(value.$!=='value')return -1;return value.code+value.end;", "-1\n-1\n7\n"),
        ("if(value.$==='missing'){console.log('missing');return -1;}if(value.$==='bad'){console.log('bad');return -2;}return value.code+value.end;", "make 0\nmissing\n-1\nmake 1\nbad\n-2\nmake 2\n7\n"),
    ].into_iter().enumerate() {
        let declarations = if index == 5 {
            declarations.replace("function make(n){", "function make(n){console.log('make',n);")
        } else {
            declarations.to_owned()
        };
        let source = format!("{declarations}/** @param {{GuardState}} value @returns {{number}} */function read(value){{{body}}}\nconsole.log(read(make(0)));console.log(read(make(1)));console.log(read(make(2)));");
        let surface = parse_javascript(&source).unwrap();
        let program = check_program(&surface).unwrap();
        let javascript = emit_javascript(&program).unwrap().text;
        let rust = emit_rust_with_array_consumption(&program, RustEmissionOptions::default()).unwrap().text;
        let oracle = terminal_tag_guard_contract(&source, &javascript);
        assert_eq!(serde_json::to_value(&surface).unwrap(), oracle["surface"], "case {index}: actual JavaScript source surface");
        assert_eq!(javascript, oracle["javascript"].as_str().unwrap(), "case {index}: JavaScript bytes");
        assert_eq!(rust, oracle["rust"].as_str().unwrap(), "case {index}: Rust bytes");
        for actual in oracle["observations"].as_array().unwrap() {
            assert_eq!(actual.as_str().unwrap(), expected, "case {index}: independent original and both JavaScript checkers");
        }
        for actual in oracle["archives"].as_array().unwrap() {
            assert_eq!(actual["javascript"].as_str().unwrap(), javascript);
            assert_eq!(actual["rust"].as_str().unwrap(), rust);
            assert_eq!(actual["output"].as_str().unwrap(), expected);
        }
        let packet = encode_typed_program_links(&program).unwrap();
        for network in [
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_link_network_binary(&encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(), BinaryLinksOptions::default()).unwrap(),
        ] {
            let restored = decode_typed_program_links(&network, packet.root).unwrap();
            assert_eq!(restored, program, "case {index}: complete source-free graph");
            assert_eq!(emit_javascript(&restored).unwrap().text, javascript);
            assert_eq!(emit_rust_with_array_consumption(&restored, RustEmissionOptions::default()).unwrap().text, rust);
        }
        let directory = TemporaryDirectory::new(&format!("terminal-tag-guard-{index}"));
        let file = directory.0.join("program.rs");
        let executable = directory.0.join(format!("program{}", std::env::consts::EXE_SUFFIX));
        std::fs::write(&file, &rust).unwrap();
        let compiled = run(Command::new("rustc").arg("--edition=2024").arg(&file).arg("-o").arg(&executable));
        assert!(compiled.ok, "case {index}: {}", compiled.stderr);
        let actual = run(&mut Command::new(&executable));
        assert!(actual.ok, "case {index}: {}", actual.stderr);
        assert_eq!(actual.stdout, expected, "case {index}: actual generated Rust execution in CI");
    }
    for body in [
        "if(value.$==='missing')return -1;return value.code;",
        "if(value.$==='missing')return -1;if(value.$==='missing')return -2;return value.code;",
        "if(value.$==='missing'){const ignored=0;}if(value.$==='bad')return -2;return value.code;",
        "if(value.$==='missing')return -1;value=make(0);if(value.$==='bad')return -2;return value.code;",
    ] {
        let source = format!(
            "{declarations}/** @param {{GuardState}} value @returns {{number}} */function read(value){{{body}}}"
        );
        let failure = parse_javascript(&source)
            .and_then(|surface| check_program(&surface))
            .unwrap_err();
        assert!(
            matches!(failure.kind.as_str(), "type" | "unsupported"),
            "{body}: {failure}"
        );
    }
    let different = format!(
        "{declarations}/** @param {{GuardState}} left @param {{GuardState}} right @returns {{number}} */function read(left,right){{if(left.$==='missing')return -1;if(right.$==='bad')return -2;return left.code;}}"
    );
    assert!(
        parse_javascript(&different)
            .and_then(|surface| check_program(&surface))
            .is_err(),
        "separate source bindings cannot share exclusions"
    );
}
