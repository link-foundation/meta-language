//! Original documented parameters retain presence in the native ABI and links.
use meta_language::translation::check::check_program;
use meta_language::translation::emit_javascript::emit_javascript;
use meta_language::translation::emit_rust::emit_rust;
use meta_language::translation::ir::Decl;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{SItem, SNode, SParam};
use meta_language::translation::types::Type;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkNetwork, create_typed_program_links,
    decode_link_network_binary, decode_typed_program_links, emit_typed_program_links,
    encode_link_network_binary, read_structured_data_lino,
};
use serde_json::Value;
use std::path::Path;
use std::process::Command;

fn fixture() -> Value {
    read_structured_data_lino(include_str!(
        "../../../parity/fixtures/self-translation-bindings.lino"
    ))
    .unwrap()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["name"] == "documented-optional-string")
        .unwrap()
        .clone()
}

fn javascript_output(source: &str) -> String {
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
fn documented_optional_string_retains_its_type_default_and_presence_metadata() {
    let fixture = fixture();
    let source = fixture["source"].as_str().unwrap();
    assert_eq!(source.len(), 99);
    let surface = parse_javascript(source).unwrap();
    let SItem::Fn(function) = &surface.items[0] else {
        panic!("the original source owns a function")
    };
    let parameter = &function.params[0];
    assert!(parameter.optional);
    assert_eq!(
        parameter.ty,
        Some(Type::Option {
            element: Box::new(Type::String),
            absence: "undefined".to_owned()
        })
    );
    let default = parameter.default_value.as_ref().unwrap();
    assert!(matches!(&default.node, SNode::None { absence } if absence == "undefined"));
    assert_eq!(default.span, parameter.span);
    let program = check_program(&surface).unwrap();
    let Decl::Fn(function) = &program.declarations[0] else {
        panic!("the checked function remains owned")
    };
    assert!(function.params[0].optional);
    assert_eq!(Some(&function.params[0].ty), parameter.ty.as_ref());
    assert_eq!(function.params[0].default_value, parameter.default_value);
    assert_eq!(function.ret, function.params[0].ty);
    assert_eq!(function.annotated_ret, Some(Type::String));
    assert_eq!(serde_json::to_value(parameter).unwrap()["optional"], true);
    assert_eq!(
        serde_json::to_value(&function.params[0]).unwrap()["optional"],
        true
    );
    assert!(
        emit_rust(&program)
            .unwrap()
            .text
            .contains("pub fn identity(value: Option<String>) -> Option<String>")
    );
}

#[test]
fn javascript_authored_optional_parameters_reload_without_source_and_keep_every_observation() {
    let fixture = fixture();
    let source = fixture["source"].as_str().unwrap();
    let expected = check_program(&parse_javascript(source).unwrap()).unwrap();
    let script = "import{createTypedProgramLinks}from'./js/src/typed-program-links.js';const a=createTypedProgramLinks(process.argv[1],'JavaScript');process.stdout.write(JSON.stringify({lino:a.network.toLino(),root:Number(a.root.asU64())}));";
    let output = Command::new("node")
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap())
        .args(["--input-type=module", "-e", script, source])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let authored: Value = serde_json::from_slice(&output.stdout).unwrap();
    let authored_root = LinkId::from_u64(authored["root"].as_u64().unwrap());
    let authored_network = LinkNetwork::from_lino(authored["lino"].as_str().unwrap()).unwrap();
    assert_eq!(
        decode_typed_program_links(&authored_network, authored_root).unwrap(),
        expected
    );
    let native = create_typed_program_links(source, "JavaScript").unwrap();
    for (root, network) in [
        (authored_root, authored_network),
        (native.root, native.network),
    ] {
        let binary = encode_link_network_binary(&network, BinaryLinksOptions::default()).unwrap();
        for restored in [
            LinkNetwork::from_lino(&network.to_lino()).unwrap(),
            decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
        ] {
            assert_eq!(
                decode_typed_program_links(&restored, root).unwrap(),
                expected
            );
            emit_typed_program_links(&restored, root, "JavaScript").unwrap();
            let rust = emit_typed_program_links(&restored, root, "Rust")
                .unwrap()
                .code;
            assert!(rust.contains("pub fn identity(value: Option<String>) -> Option<String>"));
        }
    }
    for observation in
        std::iter::once(&fixture).chain(fixture["additionalCalls"].as_array().unwrap())
    {
        let call = observation["call"].as_str().unwrap();
        let expected = observation["expected"].as_str().unwrap();
        let complete = format!("{source}\nconsole.log({call});");
        assert_eq!(javascript_output(&complete), expected);
        let checked = check_program(&parse_javascript(&complete).unwrap()).unwrap();
        let archive = create_typed_program_links(&complete, "JavaScript").unwrap();
        let binary =
            encode_link_network_binary(&archive.network, BinaryLinksOptions::default()).unwrap();
        for network in [
            LinkNetwork::from_lino(&archive.network.to_lino()).unwrap(),
            decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
        ] {
            assert_eq!(
                decode_typed_program_links(&network, archive.root).unwrap(),
                checked
            );
            assert_eq!(
                javascript_output(
                    &emit_typed_program_links(&network, archive.root, "JavaScript")
                        .unwrap()
                        .code
                ),
                expected
            );
        }
    }
}

#[test]
fn runtime_defaults_keep_precedence_and_required_parameter_metadata_stays_absent() {
    let source = "/** @param {number} [value=99] @returns {number} */\nfunction add(value = 2) { return value + 1; }\nconsole.log(add(), add(7));\n";
    let surface = parse_javascript(source).unwrap();
    let SItem::Fn(function) = &surface.items[0] else {
        panic!("the default source owns a function")
    };
    assert!(!function.params[0].optional);
    assert_eq!(function.params[0].ty, Some(Type::Float));
    assert!(
        !serde_json::to_value(&function.params[0])
            .unwrap()
            .as_object()
            .unwrap()
            .contains_key("optional")
    );
    let program = check_program(&surface).unwrap();
    let Decl::Fn(function) = &program.declarations[0] else {
        panic!("the checked default function remains owned")
    };
    assert!(!function.params[0].optional);
    assert!(
        !serde_json::to_value(&function.params[0])
            .unwrap()
            .as_object()
            .unwrap()
            .contains_key("optional")
    );
    assert_eq!(javascript_output(source), "3 8\n");
    assert_eq!(
        javascript_output(&emit_javascript(&program).unwrap().text),
        "3 8\n"
    );
    for (complete, expected) in [
        (format!("{source}\nfunction maybe(present) {{ if (present) return 7; }}\nconsole.log(add(undefined), add(maybe(false)), add(maybe(true)));\n"), "3 8\n3 3 8\n"),
        ("/** @param {bigint} [value] @returns {bigint} */\nfunction identity(value) { return value; }\nconsole.log(identity(7n));\nconsole.log(String(identity(7n)));\nconsole.log(identity());\nconsole.log(String(identity()));\n".to_owned(), "7n\n7\nundefined\nundefined\n"),
    ] {
        assert_eq!(javascript_output(&complete), expected);
        let checked = check_program(&parse_javascript(&complete).unwrap()).unwrap();
        assert_eq!(javascript_output(&emit_javascript(&checked).unwrap().text), expected);
        let archive = create_typed_program_links(&complete, "JavaScript").unwrap();
        let binary = encode_link_network_binary(&archive.network, BinaryLinksOptions::default()).unwrap();
        for network in [LinkNetwork::from_lino(&archive.network.to_lino()).unwrap(), decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap()] {
            assert_eq!(decode_typed_program_links(&network, archive.root).unwrap(), checked);
            assert_eq!(javascript_output(&emit_typed_program_links(&network, archive.root, "JavaScript").unwrap().code), expected);
        }
    }
    let parameter = serde_json::to_value(&surface.items).unwrap()[0]["params"][0].clone();
    for malformed in [
        Value::Null,
        Value::String("true".to_owned()),
        Value::from(1),
    ] {
        let mut changed = parameter.clone();
        changed["optional"] = malformed;
        assert!(serde_json::from_value::<SParam>(changed).is_err());
    }
    let invalid = "/** @param {number} [other] */\nfunction identity(value) { return value; }\n";
    assert!(
        parse_javascript(invalid)
            .unwrap_err()
            .message()
            .contains("other is not a parameter")
    );
}
