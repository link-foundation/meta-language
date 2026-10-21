//! Shared source fixtures; compilation and native observation run in CI only.
use super::issue_195_translation_pairs::{TemporaryDirectory, run};
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust,
    javascript::parse_javascript,
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links, read_structured_data_lino,
};
use serde_json::Value;
use std::{fs, path::Path, process::Command};

fn policy() -> Value {
    read_structured_data_lino(include_str!("../../src/data/reference-identity-rules.lino")).unwrap()
}
fn versions() {
    let node = run(Command::new("node").args(["-p", "process.versions.node"]));
    assert!(node.ok, "{}", node.stderr);
    let parts = node
        .stdout
        .trim()
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 22 && parts[1] >= 15 || parts[0] == 24,
        "uncovered Node {}; supported >=22.15.0 <23 || >=24 <25",
        node.stdout.trim()
    );
    let rust = run(Command::new("rustc").arg("--version"));
    assert!(rust.ok, "{}", rust.stderr);
    let release = rust.stdout.split_whitespace().nth(1).unwrap();
    let parts = release
        .split('.')
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {release}; supported >=1.90.0 <1.100.0"
    );
}
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
const surface=parseJavaScript(process.argv[1]),program=checkProgram(surface);
console.log(JSON.stringify({surface,javascript:emitJavaScript(program).text,rust:emitRust(program).text}));
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
fn archives(program: &meta_language::translation::ir::Program) -> Vec<meta_language::LinkNetwork> {
    let packet = encode_typed_program_links(program).unwrap();
    vec![
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}
fn compiled(source: &str) -> (bool, String, String) {
    let directory = TemporaryDirectory::new("reference-identity");
    let input = directory.0.join("reference-identity.rs");
    let output = directory.0.join(if cfg!(windows) {
        "reference-identity.exe"
    } else {
        "reference-identity"
    });
    fs::write(&input, source).unwrap();
    let built = run(Command::new("rustc")
        .args(["--edition", "2024"])
        .arg(&input)
        .arg("-o")
        .arg(&output));
    assert!(built.ok, "{}", built.stderr);
    let result = run(&mut Command::new(output));
    (result.ok, result.stdout, result.stderr)
}

#[test]
fn reference_identity_keeps_actual_aliases_distinct_allocations_order_and_both_codecs() {
    versions();
    let policy = policy();
    for row in policy["cases"].as_array().unwrap() {
        let source = row["source"].as_str().unwrap();
        let expected = row["expected"].as_str().unwrap();
        assert_eq!(
            observe(source),
            expected,
            "independent Node: {}",
            row["name"]
        );
        let contract = javascript_contract(source);
        let surface = parse_javascript(source).unwrap();
        assert_eq!(serde_json::to_value(&surface).unwrap(), contract["surface"]);
        let program = check_program(&surface).unwrap();
        assert_eq!(observe(&emit_javascript(&program).unwrap().text), expected);
        let rust = emit_rust(&program).unwrap().text;
        assert_eq!(rust, contract["rust"].as_str().unwrap());
        let direct = compiled(&rust);
        assert!(direct.0, "{}", direct.2);
        assert_eq!(direct.1, expected);
        let packet = encode_typed_program_links(&program).unwrap();
        for network in archives(&program) {
            let restored = decode_typed_program_links(&network, packet.root).unwrap();
            encode_typed_program_links(&restored).unwrap();
            assert_eq!(restored, program);
            assert_eq!(observe(&emit_javascript(&restored).unwrap().text), expected);
            let replay = emit_rust(&restored).unwrap().text;
            assert_eq!(replay, rust);
            let observed = compiled(&replay);
            assert!(observed.0, "{}", observed.2);
            assert_eq!(observed.1, expected);
        }
    }
}

#[test]
fn reference_identity_keeps_every_explicit_surface_refusal() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let policy = policy();
    for row in policy["refusals"].as_array().unwrap() {
        let script = r"
import {parseJavaScript} from './js/src/translation/javascript.js';
console.log(JSON.stringify(parseJavaScript(process.argv[1],JSON.parse(process.argv[2]))));
";
        let source = row["source"].as_str().unwrap();
        let output = run(Command::new("node").current_dir(root).args([
            "--input-type=module",
            "-e",
            script,
            source,
            &row["options"].to_string(),
        ]));
        assert!(output.ok, "{}", output.stderr);
        let surface = serde_json::from_str(&output.stdout).unwrap();
        assert!(check_program(&surface).is_err(), "{}", row["name"]);
    }
}

fn change_first_reference_binary(value: &mut Value, mutation: &str) -> bool {
    if value["k"] == "binary" && value["domain"]["reference"] == true {
        match mutation {
            "missing domain" => {
                value.as_object_mut().unwrap().remove("domain");
            }
            "wrong domain" => value["domain"] = serde_json::json!({"kind":"float"}),
            "ordering" => value["op"] = Value::String("lt".to_owned()),
            "wrong result" => value["type"] = serde_json::json!({"kind":"float"}),
            _ => unreachable!(),
        }
        return true;
    }
    if let Some(fields) = value.as_object_mut() {
        for child in fields.values_mut() {
            if change_first_reference_binary(child, mutation) {
                return true;
            }
        }
    } else if let Some(values) = value.as_array_mut() {
        for child in values {
            if change_first_reference_binary(child, mutation) {
                return true;
            }
        }
    }
    false
}

#[test]
fn reference_identity_refuses_forged_domains_operators_and_results_before_encoding() {
    let policy = policy();
    let source = policy["cases"][0]["source"].as_str().unwrap();
    let original = check_program(&parse_javascript(source).unwrap()).unwrap();
    for mutation in ["missing domain", "wrong domain", "ordering", "wrong result"] {
        let mut value = serde_json::to_value(&original).unwrap();
        assert!(change_first_reference_binary(&mut value, mutation));
        let changed = serde_json::from_value(value).unwrap();
        assert!(encode_typed_program_links(&changed).is_err(), "{mutation}");
    }
}
