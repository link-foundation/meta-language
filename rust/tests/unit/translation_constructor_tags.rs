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

const TAG_CONTEXT_INPUT: &str = "/** @typedef {{ $: 'malformed' } | { $: 'unsupported', end: number, escapeLength: number } | { $: 'scalar', code: number, end: number }} Input */\n";
const TAG_CONTEXT_OUTPUT: &str = "/** @typedef {{ $: 'malformed', at: number, end: number } | { $: 'nonScalar', at: number, end: number } | { $: 'value', code: number }} Output */\n";
const TAG_CONTEXT_MAKE: &str = "/** @param {number} n @returns {Input} */function readInput(n){if(n===0)return {$:'malformed'};if(n===1)return {$:'unsupported',end:7,escapeLength:6};return {$:'scalar',code:65,end:8};}\n";
const TAG_CONTEXT_CONVERT: &str = "/** @param {number} n @returns {Output} */function convert(n){const state=readInput(n);if(state.$==='malformed')return {$:'malformed',at:1,end:2};if(state.$==='unsupported')return {$:'nonScalar',at:3,end:state.end};return {$:'value',code:state.code};}\n";

fn tag_receiver_contract(source: &str, javascript: &str, expression: &str) -> Value {
    let script = r#"
import vm from 'node:vm';
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from './js/src/typed-program-links.js';
import {recheckLinkedTypedProgram} from './js/src/typed-program-checked-projection.js';
import {LinkNetwork} from './js/src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from './js/src/link-network-binary.js';
function observe(source){const lines=[];const result=vm.runInNewContext(source+'\nJSON.stringify('+process.argv[3]+');',{console:{log:(...values)=>lines.push(values.map(String).join(' '))}},{timeout:1000});return {lines,result};}
const surface=parseJavaScript(process.argv[1],{referenceSemantics:false}),checked=checkProgram(surface);
const javascript=emitJavaScript(checked).text,rust=emitRust(checked).text;
const packet=encodeTypedProgramLinks(checked),archives=[];
for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]){
 const replayed=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
 archives.push({javascript:emitJavaScript(replayed).text,rust:emitRust(replayed).text,output:observe(emitJavaScript(replayed).text)});
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
            javascript,
            expression,
        ]));
    assert!(output.ok, "{}", output.stderr);
    serde_json::from_str(&output.stdout).unwrap()
}

fn assert_tag_receiver_contract(source: &str, expression: &str, expected: &Value) {
    use meta_language::translation::emit_rust::emit_rust;
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
        encode_link_network_binary, encode_typed_program_links,
    };
    let surface = parse_javascript(source).unwrap();
    let program = check_program(&surface).unwrap();
    let javascript = emit_javascript(&program).unwrap().text;
    let rust = emit_rust(&program).unwrap().text;
    let oracle = tag_receiver_contract(source, &javascript, expression);
    assert_eq!(
        serde_json::to_value(&surface).unwrap(),
        oracle["surface"],
        "actual complete source surface"
    );
    assert_eq!(javascript, oracle["javascript"].as_str().unwrap());
    assert_eq!(rust, oracle["rust"].as_str().unwrap());
    for observation in oracle["observations"].as_array().unwrap() {
        assert_eq!(
            observation, expected,
            "original and both checked JavaScript outputs"
        );
    }
    for archive in oracle["archives"].as_array().unwrap() {
        assert_eq!(archive["javascript"].as_str().unwrap(), javascript);
        assert_eq!(archive["rust"].as_str().unwrap(), rust);
        assert_eq!(&archive["output"], expected);
    }
    let packet = encode_typed_program_links(&program).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ] {
        let replayed = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(replayed, program, "complete source-free checked program");
        assert_eq!(emit_javascript(&replayed).unwrap().text, javascript);
        assert_eq!(emit_rust(&replayed).unwrap().text, rust);
    }
}

fn tag_receiver_refusal(source: &str) -> Value {
    let failure = parse_javascript(source).unwrap_err();
    let actual = serde_json::json!({"kind":failure.kind.as_str(),"reason":failure.reason,"span":failure.span});
    let script = r#"
import {parseJavaScript} from './js/src/translation/javascript.js';
let caught;try{parseJavaScript(process.argv[1]);}catch(error){caught=error;}
if(!caught)throw new Error('the complete unchanged source must refuse');
console.log(JSON.stringify({kind:caught.kind,reason:caught.reason,span:caught.span}));
"#;
    let output = run(Command::new("node")
        .current_dir(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap(),
        )
        .args(["--input-type=module", "-e", script, source]));
    assert!(output.ok, "{}", output.stderr);
    let oracle: Value = serde_json::from_str(&output.stdout).unwrap();
    assert_eq!(actual, oracle, "actual original JavaScript refusal");
    actual
}

#[test]
fn tag_receiver_owners_retain_distinct_input_and_returned_payloads_in_both_archives() {
    let makers = [
        TAG_CONTEXT_MAKE.to_owned(),
        TAG_CONTEXT_MAKE.replace(
            "function readInput(n){",
            "const readInput = function readInput(n){",
        ),
        TAG_CONTEXT_MAKE.replace("function readInput(n){", "const readInput = (n)=>{"),
    ];
    let expected = serde_json::json!({"lines":[],"result":"[{\"$\":\"malformed\",\"at\":1,\"end\":2},{\"$\":\"nonScalar\",\"at\":3,\"end\":7},{\"$\":\"value\",\"code\":65}]"});
    for declarations in [
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}"),
        format!("{TAG_CONTEXT_OUTPUT}{TAG_CONTEXT_INPUT}"),
    ] {
        for maker in &makers {
            for bodies in [
                format!("{maker}{TAG_CONTEXT_CONVERT}"),
                format!("{TAG_CONTEXT_CONVERT}{maker}"),
            ] {
                let source = format!("{declarations}{bodies}");
                assert_tag_receiver_contract(
                    &source,
                    "[convert(0),convert(1),convert(2)]",
                    &expected,
                );
                let program = check_program(&parse_javascript(&source).unwrap()).unwrap();
                let graph = serde_json::to_value(program).unwrap();
                let declarations = graph["declarations"].as_array().unwrap();
                let input = declarations
                    .iter()
                    .find(|item| item["name"] == "Input")
                    .unwrap();
                let output = declarations
                    .iter()
                    .find(|item| item["name"] == "Output")
                    .unwrap();
                assert_eq!(
                    input["ctors"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|ctor| ctor["name"] == "malformed")
                        .unwrap()["fields"],
                    serde_json::json!([])
                );
                let fields = output["ctors"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|ctor| ctor["name"] == "malformed")
                    .unwrap()["fields"]
                    .as_array()
                    .unwrap();
                assert_eq!(
                    fields
                        .iter()
                        .map(|field| field["name"].as_str().unwrap())
                        .collect::<Vec<_>>(),
                    ["at", "end"]
                );
            }
        }
    }
}

#[test]
fn tag_receiver_parameters_aliases_effects_and_zero_argument_forward_results_keep_their_owners() {
    let source = format!(
        "{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}{TAG_CONTEXT_MAKE}/** @param {{Input}} input @returns {{Output}} */function convertInput(input){{const state=input;if(state.$==='malformed'){{console.log('first');return {{$:'malformed',at:4,end:5}};}}if(state.$==='unsupported'){{console.log('second');return {{$:'nonScalar',at:6,end:state.end}};}}console.log('last');return {{$:'value',code:state.code}};}}\n/** @param {{number}} n @returns {{Output}} */function convert(n){{console.log('argument',n);return convertInput(readInput(n));}}\n"
    );
    assert_tag_receiver_contract(
        &source,
        "[convert(0),convert(1),convert(2)]",
        &serde_json::json!({"lines":["argument 0","first","argument 1","second","argument 2","last"],"result":"[{\"$\":\"malformed\",\"at\":4,\"end\":5},{\"$\":\"nonScalar\",\"at\":6,\"end\":7},{\"$\":\"value\",\"code\":65}]"}),
    );
    let types = "/** @typedef {{ $: 'shared', input: number } | { $: 'first', input: number }} First */\n/** @typedef {{ $: 'shared', output: boolean } | { $: 'second', output: boolean }} Second */\n";
    let maker = "/** @returns {First} */function make(){return {$:'shared',input:3};}\n";
    let inspect = "/** @returns {number} */function inspect(){const one=make();const two=one;const three=two;if(three.$==='shared')return three.input;return three.input;}\n";
    for declaration in [
        maker.to_owned(),
        maker.replace("function make(){", "const make=()=>{"),
    ] {
        for functions in [
            format!("{declaration}{inspect}"),
            format!("{inspect}{declaration}"),
        ] {
            assert_tag_receiver_contract(
                &format!("{types}{functions}"),
                "inspect()",
                &serde_json::json!({"lines":[],"result":"3"}),
            );
        }
    }
}

#[test]
fn tag_receiver_unknown_optional_async_mutable_shadowed_namespace_and_captured_owners_refuse_exactly()
 {
    let sources = [
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}/** @returns {{Output}} */function read(state){{if(state.$==='malformed')return {{$:'malformed',at:1,end:2}};return {{$:'value',code:0}};}}"),
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}/** @param {{Input}} state @returns {{Output}} */function outer(state){{const read=(state)=>{{if(state.$==='malformed')return {{$:'malformed',at:1,end:2}};return {{$:'value',code:0}};}};return read(state);}}"),
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}{TAG_CONTEXT_MAKE}/** @param {{Input}} state @returns {{Output}} */function read(state){{{{let state=readInput(0);if(state.$==='malformed')return {{$:'malformed',at:1,end:2}};return {{$:'value',code:0}};}}}}"),
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}/** @param {{Input|undefined}} state @returns {{Output}} */function read(state){{if(state.$==='malformed')return {{$:'malformed',at:1,end:2}};return {{$:'value',code:0}};}}"),
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}{}{TAG_CONTEXT_CONVERT}",TAG_CONTEXT_MAKE.replace("@returns {Input}","@returns {Input|undefined}")),
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}/** @returns {{Promise<Input>}} */async function readInput(){{return {{$:'malformed'}};}}\n/** @returns {{Promise<number>}} */async function read(){{const state=await readInput();if(state.$==='malformed')return 0;return 1;}}"),
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}{TAG_CONTEXT_MAKE}{}{TAG_CONTEXT_CONVERT}",TAG_CONTEXT_MAKE.replace("@returns {Input}","@returns {Output}").replace("if(n===0)return {$:'malformed'};if(n===1)return {$:'unsupported',end:7,escapeLength:6};return {$:'scalar',code:65,end:8};","return {$:'malformed',at:1,end:2};")),
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}{TAG_CONTEXT_MAKE}/** @returns {{number}} */function read(readInput){{const state=readInput();if(state.$==='malformed')return 0;return 1;}}"),
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}{TAG_CONTEXT_MAKE}const api={{readInput(){{return {{$:'malformed',at:1,end:2}};}}}};\n{}",TAG_CONTEXT_CONVERT.replace("readInput(n)","api.readInput(n)")),
        format!("{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}/** @param {{Input}} state @returns {{number}} */function read(state){{const captured=()=>{{if(state.$==='malformed')return 0;return 1;}};return captured();}}"),
    ];
    for source in &sources {
        let diagnostic = tag_receiver_refusal(source);
        let start = source.find("state.$==='malformed'").unwrap();
        assert_eq!(
            diagnostic,
            serde_json::json!({"kind":"type","reason":"tag malformed is ambiguous between data types","span":{"start":start,"end":start+"state.$==='malformed'".len()}})
        );
    }
    let optional = format!(
        "{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}/** @param {{boolean}} present @returns {{Input}} */function readInput(present){{if(present)return {{$:'malformed'}};}}\n/** @param {{boolean}} present @returns {{number}} */function read(present){{const state=readInput(present);if(state.$==='malformed')return 0;return 1;}}console.log(read(false));"
    );
    let failure = parse_javascript(&optional)
        .and_then(|surface| check_program(&surface))
        .unwrap_err();
    assert!(
        matches!(failure.kind.as_str(), "type" | "unsupported"),
        "an annotation cannot erase actual semantic absence"
    );
    let output = run(Command::new("node").args(["--input-type=module", "-e", &optional]));
    assert!(
        !output.ok,
        "the unchanged original source reads the tag of undefined"
    );
    assert!(output.stderr.contains("TypeError"));
}

#[test]
fn tag_receiver_context_retains_payload_obligations_unique_owners_and_unknown_tag_spans() {
    let ambiguous = "/** @param {Input} state */function unknownResult(state){if(state.$==='malformed')return {$:'malformed',at:1,end:2};return {$:'malformed',at:3,end:4};}";
    let mut sources = vec![format!(
        "{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}{ambiguous}"
    )];
    for convert in [
        TAG_CONTEXT_CONVERT.replace("{$:'malformed',at:1,end:2}", "{$:'malformed'}"),
        TAG_CONTEXT_CONVERT.replace(
            "{$:'malformed',at:1,end:2}",
            "{$:'malformed',at:true,end:2}",
        ),
        TAG_CONTEXT_CONVERT.replace("end:state.end", "end:state.escapeLength+state.code"),
    ] {
        sources.push(format!(
            "{TAG_CONTEXT_INPUT}{TAG_CONTEXT_OUTPUT}{TAG_CONTEXT_MAKE}{convert}"
        ));
    }
    for source in sources {
        let failure = parse_javascript(&source)
            .and_then(|surface| check_program(&surface))
            .unwrap_err();
        assert!(matches!(failure.kind.as_str(), "type" | "unsupported"));
    }
    let unique = format!(
        "{TAG_CONTEXT_INPUT}{TAG_CONTEXT_MAKE}/** @param {{Input}} state @returns {{number}} */function read(state){{if(state.$==='malformed')return -1;if(state.$==='unsupported')return -2;return state.code+state.end;}}\n"
    );
    assert_tag_receiver_contract(
        &unique,
        "[read(readInput(0)),read(readInput(1)),read(readInput(2))]",
        &serde_json::json!({"lines":[],"result":"[-1,-2,73]"}),
    );
    let unknown = format!(
        "{TAG_CONTEXT_INPUT}/** @param {{Input}} state @returns {{boolean}} */function read(state){{return state.$==='absent';}}"
    );
    let start = unknown.find("state.$==='absent'").unwrap();
    assert_eq!(
        tag_receiver_refusal(&unknown),
        serde_json::json!({"kind":"type","reason":"no @typedef has tag absent","span":{"start":start,"end":start+"state.$==='absent'".len()}})
    );
}

#[test]
fn speculative_tag_headers_preserve_ambiguity_and_ordinary_declaration_error_precedence() {
    let types = "/** @typedef {{ $: 'shared', input: number } | { $: 'first', input: number }} First */\n/** @typedef {{ $: 'shared', output: boolean } | { $: 'second', output: boolean }} Second */\n";
    let caller = "function inspect(){const state=make();return state.$==='shared';}\n";
    for (maker, kind, reason) in [
        (
            "/** @param {number} @returns {First} */function make(n){return {$:'shared',input:3};}\n",
            "syntax",
            "@param needs a type and a name",
        ),
        (
            "/** @returns {unknown} */function make(){return {$:'shared',input:3};}\n",
            "unsupported",
            "JSDoc type {unknown}: portable types are number, bigint, boolean, string, arrays T[] of them and @typedef data types",
        ),
    ] {
        let source = format!("{types}{caller}{maker}");
        let start = source.find("state.$==='shared'").unwrap();
        assert_eq!(
            tag_receiver_refusal(&source),
            serde_json::json!({"kind":"type","reason":"tag shared is ambiguous between data types","span":{"start":start,"end":start+"state.$==='shared'".len()}})
        );
        let source = format!("{types}{maker}{caller}");
        assert_eq!(
            tag_receiver_refusal(&source),
            serde_json::json!({"kind":kind,"reason":reason,"span":{"start":types.len(),"end":types.len()+maker.find("*/").unwrap()+2}})
        );
    }
}
