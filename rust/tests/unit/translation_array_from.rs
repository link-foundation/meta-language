//! Array.from source construction and mapper observations, executed only in CI.
use std::fs;
use std::path::Path;
use std::process::Command;

use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_lean::emit_lean;
use meta_language::translation::emit_rocq::emit_rocq;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::javascript::parse_javascript;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, create_typed_program_links, decode_link_network_binary,
    decode_typed_program_links, emit_typed_program_links, encode_link_network_binary,
    encode_typed_program_links,
};
use serde_json::Value;

use super::issue_195_translation_pairs::{TemporaryDirectory, run};

fn observe(source: &str) -> String {
    let output = run(Command::new("node").args(["--input-type=module", "-e", source]));
    assert!(output.ok, "{}", output.stderr);
    output.stdout
}

fn javascript_contract(source: &str) -> Value {
    let script = r"
import {parseJavaScript} from './js/src/translation/javascript.js';
import {checkProgram} from './js/src/translation/check.js';
import {emitJavaScript} from './js/src/translation/emit-javascript.js';
import {emitRust} from './js/src/translation/emit-rust.js';
import {emitLean} from './js/src/translation/emit-lean.js';
import {emitRocq} from './js/src/translation/emit-rocq.js';
const surface=parseJavaScript(process.argv[1]);
const program=checkProgram(surface);
const codes={};
for(const [target,emit] of [['JavaScript',emitJavaScript],['Rust',emitRust],['Lean',emitLean],['Rocq',emitRocq]]){
 try{codes[target]={code:emit(program).text};}catch(error){codes[target]={error:error.reason};}
}
console.log(JSON.stringify({surface,codes}));
";
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let output = run(Command::new("node").current_dir(root).args([
        "--input-type=module",
        "-e",
        script,
        source,
    ]));
    assert!(output.ok, "{}", output.stderr);
    serde_json::from_str(&output.stdout).unwrap()
}

#[test]
fn array_from_keeps_code_points_mapper_arities_effects_and_actual_raw_shape() {
    for (source, expected) in [
        (
            "function copy() { return Array.from('café😀').length; }console.log(copy());",
            "5\n",
        ),
        (
            "function text(){console.log('receiver');return 'café😀';}function probe(){const points=Array.from(text());const zero=Array.from([1,2],()=>7);const one=Array.from([2,4],value=>value*2);const two=Array.from([2,4],(value,index)=>value+index);return points.length+zero[0]+one[1]+two[1];}console.log(probe(),Array.from('').length);",
            "receiver\n25 0\n",
        ),
        (
            "function mappedText(){return Array.from('😀A',value=>value.slice(0,1));}console.log(mappedText().length);",
            "2\n",
        ),
    ] {
        assert_eq!(observe(source), expected);
        let original = javascript_contract(source);
        let surface = parse_javascript(source).unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), original["surface"]);
        let program = check_program(&surface).unwrap();
        // The public encoder runs the same checked-program validator first.
        encode_typed_program_links(&program).unwrap();
        assert_eq!(observe(&emit_javascript(&program).unwrap().text), expected);
        assert_eq!(
            emit_rust(&program).unwrap().text,
            original["codes"]["Rust"]["code"].as_str().unwrap()
        );
        let packet = create_typed_program_links(source, "JavaScript").unwrap();
        for network in [
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_link_network_binary(
                &encode_link_network_binary(&packet.network, BinaryLinksOptions::default())
                    .unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            let decoded = decode_typed_program_links(&network, packet.root).unwrap();
            encode_typed_program_links(&decoded).unwrap();
            assert_eq!(decoded, program);
            assert_eq!(
                observe(
                    &emit_typed_program_links(&network, packet.root, "JavaScript")
                        .unwrap()
                        .code
                ),
                expected
            );
        }
    }
}

#[test]
fn array_from_map_formatter_keeps_every_existing_target_byte() {
    let source = "/** @returns {number[]} */function values(){return Array.from([1,2],(value,index)=>value+index);}";
    let original = javascript_contract(source);
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    encode_typed_program_links(&program).unwrap();
    for (target, emitted) in [
        ("JavaScript", emit_javascript(&program).unwrap().text),
        ("Rust", emit_rust(&program).unwrap().text),
        ("Lean", emit_lean(&program).unwrap().text),
        ("Rocq", emit_rocq(&program).unwrap().text),
    ] {
        assert_eq!(
            emitted,
            original["codes"][target]["code"].as_str().unwrap(),
            "{target}"
        );
    }
}

#[test]
fn emitted_array_from_mapping_runs_with_original_javascript_observations() {
    let source = "function text(){console.log('receiver');return 'café😀';}function probe(){const points=Array.from(text());const zero=Array.from([1,2],()=>7);const one=Array.from([2,4],value=>value*2);const two=Array.from([2,4],(value,index)=>value+index);return points.length+zero[0]+one[1]+two[1];}console.log(probe(),Array.from('').length);";
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let directory = TemporaryDirectory::new("array-from-mapping");
    let file = directory.0.join("program.rs");
    let executable = directory.0.join("program");
    fs::write(&file, emit_rust(&program).unwrap().text).unwrap();
    let compiled = run(Command::new("rustc")
        .args(["--edition=2024"])
        .arg(&file)
        .arg("-o")
        .arg(&executable));
    assert!(compiled.ok, "{}", compiled.stderr);
    let observed = run(&mut Command::new(&executable));
    assert!(observed.ok, "{}", observed.stderr);
    assert_eq!(observed.stdout, observe(source));
}

#[test]
fn original_nullable_array_source_preserves_all_elements_before_and_after_iterable_normalization() {
    let node = run(Command::new("node").args(["-p", "process.versions.node"]));
    assert!(node.ok, "{}", node.stderr);
    let node_version = node.stdout.trim();
    let mut node_parts = node_version.split('.');
    let node_major = node_parts.next().unwrap().parse::<u32>().unwrap();
    let node_minor = node_parts.next().unwrap().parse::<u32>().unwrap();
    assert!(
        node_major == 22 && node_minor >= 15 || node_major == 24,
        "uncovered Node version {node_version}; supported >=22.15.0 <23.0.0 || >=24.0.0 <25.0.0"
    );
    let rust = run(Command::new("rustc").arg("--version"));
    assert!(rust.ok, "{}", rust.stderr);
    let rust_version = rust.stdout.split_whitespace().nth(1).unwrap();
    let mut rust_parts = rust_version.split('.');
    let rust_major = rust_parts.next().unwrap().parse::<u32>().unwrap();
    let rust_minor = rust_parts.next().unwrap().parse::<u32>().unwrap();
    assert!(
        rust_major == 1 && (90..100).contains(&rust_minor),
        "uncovered Rust version {rust_version}; supported >=1.90.0 <1.100.0"
    );
    let source = "function numberFirst() { return [1, null, undefined, -0].join('|'); }
function absenceFirst() { return [null, undefined, 1, -0].join('|'); }
function spreadAbsence() { return [1, ...[null, undefined], -0].join('|'); }
function nullableBigints() { return [1n, null, 3n].join('|'); }
console.log(numberFirst());
console.log(absenceFirst());
console.log(spreadAbsence());
console.log(nullableBigints());";
    let expected = "1|||0\n||1|0\n1|||0\n1||3\n";
    assert_eq!(observe(source), expected);
    let original = javascript_contract(source);
    let surface = parse_javascript(source).unwrap();
    assert_eq!(serde_json::to_value(&surface).unwrap(), original["surface"]);
    let program = check_program(&surface).unwrap();
    encode_typed_program_links(&program).unwrap();
    assert_eq!(observe(&emit_javascript(&program).unwrap().text), expected);
    assert_eq!(
        observe(original["codes"]["JavaScript"]["code"].as_str().unwrap()),
        expected
    );
    let expected_rust = original["codes"]["Rust"]["code"].as_str().unwrap();
    assert_eq!(emit_rust(&program).unwrap().text, expected_rust);
    let packet = create_typed_program_links(source, "JavaScript").unwrap();
    let directory = TemporaryDirectory::new("nullable-spread-archives");
    for (format, network) in [
        (
            "text",
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        ),
        (
            "binary",
            decode_link_network_binary(
                &encode_link_network_binary(&packet.network, BinaryLinksOptions::default())
                    .unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ),
    ] {
        let decoded = decode_typed_program_links(&network, packet.root).unwrap();
        encode_typed_program_links(&decoded).unwrap();
        assert_eq!(decoded, program, "{format}: complete checked program");
        let javascript = emit_typed_program_links(&network, packet.root, "JavaScript")
            .unwrap()
            .code;
        assert_eq!(
            observe(&javascript),
            expected,
            "{format}: complete emitted JavaScript"
        );
        let rust = emit_typed_program_links(&network, packet.root, "Rust")
            .unwrap()
            .code;
        assert_eq!(
            rust, expected_rust,
            "{format}: every JavaScript/native emitted Rust byte"
        );
        let file = directory.0.join(format!("program_{format}.rs"));
        let executable = directory
            .0
            .join(format!("program_{format}{}", std::env::consts::EXE_SUFFIX));
        fs::write(&file, &rust).unwrap();
        let compiled = run(Command::new("rustc")
            .args(["--edition=2024"])
            .arg(&file)
            .arg("-o")
            .arg(&executable));
        assert!(compiled.ok, "{format}: {}", compiled.stderr);
        let observed = run(&mut Command::new(&executable));
        assert!(observed.ok, "{format}: {}", observed.stderr);
        assert_eq!(
            observed.stdout, expected,
            "{format}: independent original fixture output"
        );
    }
}
