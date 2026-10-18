//! Complete shared callback profiles and native execution; run only in CI.
use std::fmt::Write as _;
use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::process::{Command, Stdio};

use meta_language::translation::{
    check::check_program,
    diagnostics::{Result as TranslationResult, TranslationError},
    emit_common::Emitted,
    emit_javascript::emit_javascript,
    emit_lean::emit_lean,
    emit_rocq::emit_rocq,
    emit_rust::{RustEmissionOptions, emit_rust_with_array_consumption},
    ir::{Decl, Node, Program},
    javascript::parse_javascript,
    surface::{SItem, SProgram},
    types::Type,
};
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, decode_link_network_binary,
    decode_typed_program_links, encode_link_network_binary, encode_typed_program_links,
    read_structured_data_lino,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

const FIXTURE: &str = include_str!("../../../parity/fixtures/array-callbacks.lino");

fn fixture() -> Value {
    let value = read_structured_data_lino(FIXTURE).unwrap();
    assert_eq!(value["schemaVersion"], 1);
    assert_eq!(value["model"], "complete array callback profiles 1");
    assert_eq!(value["cases"].as_array().unwrap().len(), 18);
    value
}

fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .fold(String::new(), |mut text, byte| {
            write!(text, "{byte:02x}").expect("writing into a String succeeds");
            text
        })
}

fn diagnostic(error: &TranslationError) -> Value {
    json!({
        "kind": error.kind.as_str(), "reason": error.reason, "span": error.span,
        "details": error.construct.as_ref().map_or_else(|| json!({}), |value| json!({"construct":value})),
    })
}

fn target(program: &Program, language: &str) -> TranslationResult<Emitted> {
    match language {
        "JavaScript" => emit_javascript(program),
        "Rust" => emit_rust_with_array_consumption(program, RustEmissionOptions::default()),
        "Lean" => emit_lean(program),
        "Rocq" => emit_rocq(program),
        _ => panic!("the complete fixture uses only its four actual target emitters"),
    }
}

fn target_record(program: &Program, language: &str) -> Value {
    match target(program, language) {
        Ok(emitted) => {
            let mut value = serde_json::to_value(&emitted).unwrap();
            value["preludes"] = json!(emitted.preludes);
            value["definitions"] = json!(emitted.definitions);
            json!({"emitted":value})
        }
        Err(error) => json!({"diagnostic":diagnostic(&error)}),
    }
}

fn owners(program: &Program) -> Value {
    json!(
        program
            .declarations
            .iter()
            .map(|owner| json!({
                "kind":owner.kind(), "name":owner.name(), "fullName":owner.full_name(),
                "modulePath":owner.module_path(),
            }))
            .collect::<Vec<_>>()
    )
}

fn archive_bytes(value: &Value) -> Vec<u8> {
    let text = value.as_str().unwrap();
    assert_eq!(text.len() % 2, 0);
    text.as_bytes()
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn check_targets(program: &Program, profile: &Value) {
    assert_eq!(owners(program), profile["checkedOwners"]);
    for language in ["JavaScript", "Rust", "Lean", "Rocq"] {
        assert_eq!(
            target_record(program, language),
            profile["targets"][language],
            "{language}"
        );
    }
}

#[test]
fn complete_callback_profiles_preserve_actual_frontends_targets_owners_and_both_archives() {
    let fixture = fixture();
    let mut observations = Vec::new();
    let mut expected_observations = Vec::new();
    let mut expected_surfaces = Vec::new();
    let mut admitted = 0;
    let mut refused = 0;
    let mut allocations = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let source = row["programSource"].as_str().unwrap();
        assert_eq!(digest(source.as_bytes()), row["programSourceSha256"]);
        assert_eq!(
            digest(row["source"].as_str().unwrap().as_bytes()),
            row["sourceSha256"]
        );
        let native_surface = parse_javascript(source).unwrap();
        observations.push(json!({"source":source,"readSurface":true}));
        expected_observations.push(row["expectedOutput"].clone());
        expected_surfaces.push(Some(serde_json::to_value(native_surface).unwrap()));
        for profile in row["profiles"].as_array().unwrap() {
            let surface: SProgram = serde_json::from_value(profile["surface"].clone()).unwrap();
            assert_eq!(serde_json::to_value(&surface).unwrap(), profile["surface"]);
            if let Some(expected) = profile.get("checkDiagnostic") {
                assert_eq!(diagnostic(&check_program(&surface).unwrap_err()), *expected);
                assert_eq!(profile["referenceSemantics"], true);
                refused += 1;
                continue;
            }
            let program = check_program(&surface).unwrap();
            let source = surface
                .items
                .iter()
                .find_map(|item| match item {
                    SItem::Fn(function) if function.name == "run" => Some(function),
                    _ => None,
                })
                .unwrap();
            let Some(Decl::Fn(function)) = program.declaration("run") else {
                panic!("the actual callback declaration retains its source owner");
            };
            assert!(source.body.span.is_some());
            let method = if let Node::ReferenceAllocate { value } = &function.body.node {
                allocations.push(row["id"].as_str().unwrap());
                value.as_ref()
            } else {
                &function.body
            };
            assert!(matches!(method.node, Node::Method { .. }));
            assert_eq!(method.span, source.body.span, "{}: inner method", row["id"]);
            assert_eq!(
                function.body.span, source.body.span,
                "{}: result",
                row["id"]
            );
            check_targets(&program, profile);
            assert_eq!(profile["observedOutput"], row["expectedOutput"]);
            let javascript = emit_javascript(&program).unwrap();
            observations.push(json!({"source":javascript.text,"readSurface":false}));
            expected_observations.push(row["expectedOutput"].clone());
            expected_surfaces.push(None);
            let archive = &profile["checkedArchive"];
            let text = archive["text"].as_str().unwrap();
            let binary = archive_bytes(&archive["binaryHex"]);
            assert_eq!(digest(text.as_bytes()), archive["textSha256"]);
            assert_eq!(digest(&binary), archive["binarySha256"]);
            let root = LinkId::from_u64(archive["root"].as_u64().unwrap());
            let native = encode_typed_program_links(&program).unwrap();
            let native_binary =
                encode_link_network_binary(&native.network, BinaryLinksOptions::default()).unwrap();
            for (network, root) in [
                (LinkNetwork::from_lino(text).unwrap(), root),
                (
                    decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
                    root,
                ),
                (
                    LinkNetwork::from_lino(&native.network.to_lino()).unwrap(),
                    native.root,
                ),
                (
                    decode_link_network_binary(&native_binary, BinaryLinksOptions::default())
                        .unwrap(),
                    native.root,
                ),
            ] {
                let restored = decode_typed_program_links(&network, root).unwrap();
                assert_eq!(restored, program);
                encode_typed_program_links(&restored).unwrap();
                check_targets(&restored, profile);
            }
            admitted += 1;
        }
    }
    assert_eq!((admitted, refused), (18, 9));
    assert_eq!(
        allocations,
        [
            "array-callback-map-reference-value-parameters",
            "array-callback-filter-reference-value-parameters",
        ]
    );
    let script = r"
import {readFileSync} from 'node:fs';
import {parseJavaScript} from './js/src/translation/javascript.js';
import {observeArrayCallbackProgram} from './js/scripts/generate-array-callback-fixtures.mjs';
const rows=JSON.parse(readFileSync(0,'utf8'));
process.stdout.write(JSON.stringify(rows.map(row=>({
  observation:observeArrayCallbackProgram(row.source),
  surface:row.readSurface?parseJavaScript(row.source):null,
}))));
";
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut child = Command::new("node")
        .current_dir(root)
        .args(["--input-type=module", "-e", script])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&observations).unwrap())
        .unwrap();
    let observed = child.wait_with_output().unwrap();
    assert!(
        observed.status.success(),
        "{}",
        String::from_utf8_lossy(&observed.stderr)
    );
    let observed: Vec<Value> = serde_json::from_slice(&observed.stdout).unwrap();
    assert_eq!(observed.len(), observations.len());
    for ((actual, expected), surface) in observed
        .iter()
        .zip(expected_observations)
        .zip(expected_surfaces)
    {
        assert_eq!(actual["observation"], expected);
        if let Some(surface) = surface {
            assert_eq!(
                actual["surface"], surface,
                "complete default frontend records, without forced-reference normalization"
            );
        } else {
            assert!(actual["surface"].is_null());
        }
    }
}

fn expected_rust_value(ty: &Type, value: &Value) -> String {
    match ty {
        Type::Float => {
            assert_eq!(value["kind"], "number");
            format!(
                "f64::from_bits({}u64)",
                value["value"].as_f64().unwrap().to_bits()
            )
        }
        Type::Bool => {
            assert_eq!(value["kind"], "boolean");
            value["value"].as_bool().unwrap().to_string()
        }
        Type::Unit => {
            assert_eq!(value["kind"], "undefined");
            "()".to_owned()
        }
        Type::Array { element, .. } => {
            assert_eq!(value["kind"], "array");
            format!(
                "vec![{}]",
                value["items"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|value| expected_rust_value(element, value))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
        Type::Option { element, .. } => {
            if value["kind"] == "undefined" {
                "None".to_owned()
            } else {
                format!("Some({})", expected_rust_value(element, value))
            }
        }
        _ => panic!(
            "the independent fixture results are exactly Number, Boolean, Unit or numeric arrays/options"
        ),
    }
}

#[test]
fn every_admitted_callback_rust_target_observes_the_independent_values_and_effect_traces_in_ci() {
    let fixture = fixture();
    let mut executed = 0;
    let mut target_refusals = 0;
    for row in fixture["cases"].as_array().unwrap() {
        for profile in row["profiles"].as_array().unwrap() {
            if profile.get("checkDiagnostic").is_some() {
                continue;
            }
            let surface: SProgram = serde_json::from_value(profile["surface"].clone()).unwrap();
            let program = check_program(&surface).unwrap();
            let emitted = match target(&program, "Rust") {
                Ok(emitted) => emitted,
                Err(error) => {
                    assert_eq!(
                        json!({"diagnostic":diagnostic(&error)}),
                        profile["targets"]["Rust"]
                    );
                    target_refusals += 1;
                    continue;
                }
            };
            let Some(Decl::Fn(probe)) = program.declaration("probe") else {
                panic!("the complete original program retains its zero-argument probe");
            };
            assert_eq!(probe.params.as_slice(), &[]);
            let actual = if probe.ret.is_reference() {
                "&*actual.borrow()"
            } else {
                "&actual"
            };
            let expected = expected_rust_value(&probe.ret, &row["expectedValue"]);
            let main = format!(
                "fn main() {{ let actual = probe(); assert_eq!({actual}, &({expected})); }}"
            );
            let source = format!(
                "{}\n\n{}\n\n{main}\n",
                emitted.preludes.join("\n\n"),
                emitted.definitions.join("\n\n")
            );
            let directory = TemporaryDirectory::new("complete-array-callback-profile");
            let file = directory.0.join("program.rs");
            let executable = directory.0.join("program");
            fs::write(&file, source).unwrap();
            let compiled = run(Command::new("rustc")
                .args(["--edition=2024"])
                .arg(&file)
                .arg("-o")
                .arg(&executable));
            assert!(compiled.ok, "{}: {}", row["id"], compiled.stderr);
            let observed = run(&mut Command::new(&executable));
            assert!(observed.ok, "{}: {}", row["id"], observed.stderr);
            let mut trace = String::new();
            for index in row["expectedTrace"].as_array().unwrap() {
                writeln!(trace, "{}", index.as_u64().unwrap()).unwrap();
            }
            assert_eq!(observed.stdout, trace, "{}", row["id"]);
            executed += 1;
        }
    }
    assert!(
        executed >= 9,
        "all original false profiles must execute their actual Rust targets"
    );
    assert_eq!(executed + target_refusals, 18);
}

#[test]
fn callback_required_operands_refuse_and_optional_reduce_initial_keeps_its_original_presence() {
    let fixture = fixture();
    for row in fixture["cases"].as_array().unwrap().iter().take(9) {
        let profile = &row["profiles"][0];
        assert_eq!(profile["referenceSemantics"], false);
        let mut value = profile["surface"].clone();
        let owner = value["items"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|item| item["k"] == "fn" && item["name"] == "run")
            .unwrap();
        assert_eq!(owner["body"]["k"], "method");
        owner["body"]["args"][0] = Value::Null;
        let surface: SProgram = serde_json::from_value(value).unwrap();
        let error = check_program(&surface).unwrap_err();
        assert_eq!(error.kind.as_str(), "type");
        assert_eq!(
            error.reason,
            format!(
                ".{}() requires a function",
                row["operation"].as_str().unwrap()
            )
        );
    }
    let reduce = fixture["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["operation"] == "reduce" && row.get("referenceProfiles").is_none())
        .unwrap();
    let mut absent = reduce["profiles"][0]["surface"].clone();
    let mut optional = absent.clone();
    for (value, keep_slot) in [(&mut absent, false), (&mut optional, true)] {
        let owner = value["items"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|item| item["k"] == "fn" && item["name"] == "run")
            .unwrap();
        let args = owner["body"]["args"].as_array_mut().unwrap();
        assert_eq!(args.len(), 2);
        if keep_slot {
            args[1] = Value::Null;
        } else {
            args.truncate(1);
        }
    }
    let absent: SProgram = serde_json::from_value(absent).unwrap();
    let optional: SProgram = serde_json::from_value(optional).unwrap();
    assert_eq!(
        check_program(&optional).unwrap(),
        check_program(&absent).unwrap()
    );
    let original =
        check_program(&parse_javascript(fixture["cases"][0]["source"].as_str().unwrap()).unwrap())
            .unwrap();
    let packet = encode_typed_program_links(&original).unwrap();
    let binary =
        encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
    let mut refusals = 0;
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(restored, original);
        for edit in 0..5 {
            let mut changed = restored.clone();
            let Some(Decl::Fn(owner)) = changed
                .declarations
                .iter_mut()
                .find(|owner| owner.name() == "run")
            else {
                panic!("the original complete map owner exists");
            };
            let Node::Method { op, args, .. } = &mut owner.body.node else {
                panic!("the original map remains an actual checked method");
            };
            match edit {
                0 => *op = "unrepresented".to_owned(),
                1 => args[0] = None,
                2 | 3 => {
                    let Type::Function { params, ret } = &mut args[0].as_mut().unwrap().ty else {
                        panic!("the original callback retains its function type");
                    };
                    if edit == 2 {
                        params.push(Type::Bool);
                    } else {
                        **ret = Type::Bool;
                    }
                }
                4 => owner.body.ty = Type::Float,
                _ => unreachable!(),
            }
            // The native encoder unconditionally projects and checks the full
            // program before encoding; tuple-free inputs use the same boundary.
            assert!(encode_typed_program_links(&changed).is_err());
            refusals += 1;
        }
    }
    assert_eq!(refusals, 10);
}

#[test]
fn reference_flat_map_retains_nested_aliases_fresh_results_and_callback_order_through_archives() {
    let sources = [
        concat!(
            "/** @param {number[][]} values */ function run(values){return values.flatMap((value,index)=>{console.log(index);return [value,value];});}",
            "\nfunction probe(){const item=[7];return run([item,item]);}"
        ),
        concat!(
            "/** @param {number[]} values */ function run(values){return values.flatMap((value,index)=>{console.log(index);return values;});}",
            "\nfunction probe(){return run([4,5]);}"
        ),
    ];
    let script = r"
import {readFileSync} from 'node:fs';
import {runInNewContext} from 'node:vm';
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
function observe(source,nested){
  const trace=[],context={console:{log:value=>trace.push(String(value))}};
  runInNewContext(source,context,{timeout:1000});
  const actual=context.probe();
  const aliases=nested?actual[0]===actual[1]&&actual[0]===actual[2]:true;
  if(nested)actual[0].push(8);
  const values=nested?Array.from(actual,value=>Array.from(value)):Array.from(actual);
  const receiver=nested?[[7]]:[4,5],result=context.run(receiver);
  const fresh=result!==receiver;
  result.push(nested?[9]:9);
  return {trace,aliases,values,fresh,receiverLength:receiver.length};
}
const sources=JSON.parse(readFileSync(0,'utf8'));
process.stdout.write(JSON.stringify(sources.map((source,index)=>{
  const surface=parseJavaScript(source,{referenceSemantics:true}),program=checkProgram(surface);
  const rust=emitRust(program,{arrayConsumption:true});
  return {surface,rust:{emitted:{...rust,preludes:rust.preludes,definitions:rust.definitions}},
    observed:[source,emitJavaScript(program).text].map(text=>observe(text,index===0))};
})));
";
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut child = Command::new("node")
        .current_dir(root)
        .args(["--input-type=module", "-e", script])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(&serde_json::to_vec(&sources).unwrap())
        .unwrap();
    let observed = child.wait_with_output().unwrap();
    assert!(
        observed.status.success(),
        "{}",
        String::from_utf8_lossy(&observed.stderr)
    );
    let observed: Vec<Value> = serde_json::from_slice(&observed.stdout).unwrap();
    assert_eq!(observed.len(), sources.len());
    let expected = [
        json!({"trace":["0","1","0"],"aliases":true,"values":[[7,8],[7,8],[7,8],[7,8]],"fresh":true,"receiverLength":1}),
        json!({"trace":["0","1","0","1"],"aliases":true,"values":[4,5,4,5],"fresh":true,"receiverLength":2}),
    ];
    for (index, row) in observed.iter().enumerate() {
        assert_eq!(row["observed"], json!([expected[index], expected[index]]));
        let surface: SProgram = serde_json::from_value(row["surface"].clone()).unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), row["surface"]);
        let program = check_program(&surface).unwrap();
        let Some(Decl::Fn(function)) = program.declaration("run") else {
            panic!("the source retains its complete flatMap function");
        };
        let Node::Method { op, args, .. } = &function.body.node else {
            panic!("the actual reference result needs no additional allocation node");
        };
        assert_eq!(op, "flatMap");
        let Type::Function { ret, .. } = &args[0].as_ref().unwrap().ty else {
            panic!("the actual callback retains its checked function result");
        };
        assert!(matches!(
            ret.as_ref(),
            Type::Array {
                reference: true,
                ..
            }
        ));
        assert_eq!(&function.body.ty, ret.as_ref());
        assert_eq!(target_record(&program, "Rust"), row["rust"]);
        let packet = encode_typed_program_links(&program).unwrap();
        let binary =
            encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap();
        for network in [
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
        ] {
            let restored = decode_typed_program_links(&network, packet.root).unwrap();
            assert_eq!(restored, program);
            encode_typed_program_links(&restored).unwrap();
            assert_eq!(target_record(&restored, "Rust"), row["rust"]);
            assert_eq!(
                emit_javascript(&restored).unwrap(),
                emit_javascript(&program).unwrap()
            );
        }
        let main = if index == 0 {
            concat!(
                "fn main(){let actual=probe();{let values=actual.borrow();assert_eq!(values.len(),4);",
                "assert!(std::rc::Rc::ptr_eq(&values[0],&values[1]));assert!(std::rc::Rc::ptr_eq(&values[0],&values[2]));",
                "values[0].borrow_mut().push(8f64);}assert!(actual.borrow().iter().all(|value|*value.borrow()==vec![7f64,8f64]));",
                "let receiver=std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new(vec![7f64]))]));",
                "let result=run(receiver.clone());assert!(!std::rc::Rc::ptr_eq(&receiver,&result));",
                "result.borrow_mut().push(std::rc::Rc::new(std::cell::RefCell::new(vec![9f64])));assert_eq!(receiver.borrow().len(),1);}"
            )
        } else {
            concat!(
                "fn main(){let actual=probe();assert_eq!(*actual.borrow(),vec![4f64,5f64,4f64,5f64]);",
                "let receiver=std::rc::Rc::new(std::cell::RefCell::new(vec![4f64,5f64]));let result=run(receiver.clone());",
                "assert!(!std::rc::Rc::ptr_eq(&receiver,&result));result.borrow_mut().push(9f64);assert_eq!(*receiver.borrow(),vec![4f64,5f64]);}"
            )
        };
        let emitted = target(&program, "Rust").unwrap();
        let source = format!(
            "{}\n\n{}\n\n{main}\n",
            emitted.preludes.join("\n\n"),
            emitted.definitions.join("\n\n")
        );
        let directory = TemporaryDirectory::new("reference-flat-map-aliases");
        let file = directory.0.join("program.rs");
        let executable = directory.0.join("program");
        fs::write(&file, source).unwrap();
        let compiled = run(Command::new("rustc")
            .args(["--edition=2024"])
            .arg(&file)
            .arg("-o")
            .arg(&executable));
        assert!(compiled.ok, "{}", compiled.stderr);
        let actual = run(&mut Command::new(&executable));
        assert!(actual.ok, "{}", actual.stderr);
        assert_eq!(
            actual.stdout,
            if index == 0 {
                "0\n1\n0\n"
            } else {
                "0\n1\n0\n1\n"
            }
        );
    }
}
