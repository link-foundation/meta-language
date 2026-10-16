//! Original JavaScript binding sources and source-free transport parity, CI only.
use std::path::Path;
use std::process::Command;

use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust,
    javascript::parse_javascript, surface::SProgram,
};
use meta_language::{
    BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
    encode_link_network_binary, encode_typed_program_links,
};
use serde_json::{Value, json};

fn fixture() -> Value {
    super::project_data::read_project_data(include_str!(
        "../../../parity/fixtures/javascript-array-bindings.lino"
    ))
    .expect("the original binding sources are shared inspectable data")
}

fn observe(source: &str) -> String {
    let output = Command::new("node")
        .args(["--input-type=module", "-e", source])
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
fn original_array_binding_sources_keep_output_and_both_source_free_archives() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let script = "import {parseJavaScript} from './js/src/translation/javascript.js';import {checkProgram} from './js/src/translation/check.js';import {emitRust} from './js/src/translation/emit-rust.js';const surface=parseJavaScript(process.argv[1]);process.stdout.write(JSON.stringify({surface,rust:emitRust(checkProgram(surface)).text}));";
    let fixture = fixture();
    let accepted = fixture["accepted"].as_array().unwrap();
    assert_eq!(accepted.len(), 8);
    for entry in accepted {
        let source = entry["source"].as_str().unwrap();
        let expected = entry["expected"].as_str().unwrap();
        assert_eq!(
            observe(source),
            expected,
            "independent original source {}",
            entry["id"]
        );
        let output = Command::new("node")
            .current_dir(root)
            .args(["--input-type=module", "-e", script, source])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let javascript: Value = serde_json::from_slice(&output.stdout).unwrap();
        let surface = parse_javascript(source).expect("the complete actual source parses");
        assert_eq!(
            serde_json::to_value(&surface).unwrap(),
            javascript["surface"],
            "actual parsed source {}",
            entry["id"]
        );
        let program = check_program(&surface).expect("the original source checks");
        assert_eq!(observe(&emit_javascript(&program).unwrap().text), expected);
        let rust = emit_rust(&program).unwrap().text;
        assert_eq!(
            rust,
            javascript["rust"].as_str().unwrap(),
            "complete target code {}",
            entry["id"]
        );
        let saved = encode_typed_program_links(&program).unwrap();
        let text = LinkNetwork::from_lino(&saved.network.to_lino()).unwrap();
        let binary = decode_link_network_binary(
            &encode_link_network_binary(&saved.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap();
        for network in [text, binary] {
            let restored = decode_typed_program_links(&network, saved.root).unwrap();
            assert_eq!(observe(&emit_javascript(&restored).unwrap().text), expected);
            assert_eq!(emit_rust(&restored).unwrap().text, rust);
        }
    }
}

#[test]
fn every_original_binding_refusal_remains_a_refusal() {
    let fixture = fixture();
    let refused = fixture["refused"].as_array().unwrap();
    assert_eq!(refused.len(), 13);
    for entry in refused {
        let source = entry[1].as_str().unwrap();
        assert!(
            parse_javascript(source)
                .and_then(|program| check_program(&program))
                .is_err(),
            "original refusal {}",
            entry[0]
        );
    }
}

fn raw_program(start: Value) -> Value {
    json!({"language":"JavaScript","items":[{"k":"fn","name":"suffix","params":[{"name":"values","type":{"kind":"array","element":{"kind":"float"}}}],"ret":{"kind":"array","element":{"kind":"float"}},"body":{"k":"arrayBindingRest","object":{"k":"name","path":["values"]},"start":start}}],"main":null})
}

#[test]
fn raw_binding_transport_preserves_presence_and_only_validates_the_selected_literal() {
    let valid = json!({"k":"num","value":"0"});
    for start in [
        json!({"k":"num","value":""}),
        json!({"k":"num","value":" \t\n"}),
        json!({"k":"num","value":"0x1"}),
        json!({"k":"num","value":"1e0"}),
        json!({"k":"num","value":"-0"}),
    ] {
        let raw = raw_program(start);
        let program: SProgram = serde_json::from_value(raw.clone()).unwrap();
        assert_eq!(
            serde_json::to_value(&program).unwrap()["items"][0]["body"]["start"],
            raw["items"][0]["body"]["start"]
        );
        check_program(&program)
            .expect("the validated raw Number receives its internal Float annotation");
    }
    for start in [
        json!({"k":"num","value":"0","type":{"kind":"float"}}),
        json!({"k":"num","value":"0","unit":false}),
        json!({"k":"num","value":"0","negative":null}),
        json!({"k":"num","value":false}),
        json!({"k":"num","value":"1_0"}),
        json!({"k":"num","value":"-0x1"}),
        json!({"k":"num","value":"NaN"}),
        json!({"k":"num","value":"0.5"}),
        json!({"k":"num","value":"-1"}),
        json!({"k":"num","value":"9007199254740992"}),
    ] {
        let program: SProgram = serde_json::from_value(raw_program(start)).unwrap();
        assert!(check_program(&program).is_err());
    }
    for (name, value) in [
        ("extra", json!(false)),
        ("block", json!(false)),
        ("tagTest", Value::Null),
        ("parenthesized", Value::Null),
    ] {
        let mut raw = raw_program(valid.clone());
        raw["items"][0]["body"][name] = value;
        let refused = serde_json::from_value::<SProgram>(raw)
            .map_or(true, |program| check_program(&program).is_err());
        assert!(
            refused,
            "the actual present field {name} must not be erased"
        );
    }
}
