//! Shared normal-Links controls for the finite serializer-owned false type flags.
use meta_language::translation::types::Type;
use meta_language::{
    BinaryLinksOptions, LinkId, LinkMetadata, LinkNetwork, LinkType, decode_link_network_binary,
    decode_typed_program_links, emit_typed_program_links, encode_link_network_binary,
    encode_typed_program_links, read_structured_data_lino, typed_program_links_schema,
};
use serde_json::{Value, json};
use std::process::Command;

#[test]
fn actual_type_serde_omits_only_false_and_retains_true_identity_and_boolean_admission() {
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/typed-program-neutral-type-defaults.lino"
    ))
    .unwrap();
    assert_eq!(fixture["typeCases"].as_array().unwrap().len(), 5);
    for case in fixture["typeCases"].as_array().unwrap() {
        let field = case["field"].as_str().unwrap();
        let false_type: Type = serde_json::from_value(case["type"].clone()).unwrap();
        assert_eq!(case["type"][field], false);
        let normalized = serde_json::to_value(&false_type).unwrap();
        assert!(normalized.get(field).is_none());
        let mut omitted = case["type"].clone();
        omitted.as_object_mut().unwrap().remove(field);
        assert_eq!(serde_json::from_value::<Type>(omitted).unwrap(), false_type);
        let mut explicit_true = case["type"].clone();
        explicit_true[field] = json!(true);
        let true_type: Type = serde_json::from_value(explicit_true).unwrap();
        assert_eq!(serde_json::to_value(&true_type).unwrap()[field], true);
        assert_ne!(true_type.key(), false_type.key());
        for invalid in [Value::Null, json!(0), json!("false"), json!([]), json!({})] {
            let mut changed = case["type"].clone();
            changed[field] = invalid;
            assert!(serde_json::from_value::<Type>(changed).is_err());
        }
    }
}

#[test]
fn actual_checked_false_type_archives_reload_reencode_and_emit_the_original_observable_program() {
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/typed-program-neutral-type-defaults.lino"
    ))
    .unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let root = LinkId::from_u64(case["archive"]["root"].as_str().unwrap().parse().unwrap());
        let hex = case["archive"]["binaryHex"].as_str().unwrap();
        let bytes: Vec<u8> = (0..hex.len())
            .step_by(2)
            .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).unwrap())
            .collect();
        for network in [
            LinkNetwork::from_lino(case["archive"]["text"].as_str().unwrap()).unwrap(),
            decode_link_network_binary(&bytes, BinaryLinksOptions::default()).unwrap(),
        ] {
            let before = network.to_lino();
            let program = decode_typed_program_links(&network, root).unwrap();
            let saved = encode_typed_program_links(&program).unwrap();
            let binary =
                encode_link_network_binary(&saved.network, BinaryLinksOptions::default()).unwrap();
            for reloaded in [
                LinkNetwork::from_lino(&saved.network.to_lino()).unwrap(),
                decode_link_network_binary(&binary, BinaryLinksOptions::default()).unwrap(),
            ] {
                assert_eq!(
                    decode_typed_program_links(&reloaded, saved.root).unwrap(),
                    program
                );
            }
            let emitted = emit_typed_program_links(&network, root, "JavaScript").unwrap();
            let script = format!(
                "{}\nprocess.stdout.write({});",
                emitted.code,
                case["probe"].as_str().unwrap()
            );
            let output = Command::new("node").args(["-e", &script]).output().unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(
                String::from_utf8(output.stdout).unwrap(),
                case["expected"].as_str().unwrap()
            );
            assert_eq!(network.to_lino(), before);
        }
    }
}

#[test]
fn unmodeled_type_fields_and_non_type_contexts_keep_the_original_strict_unknown_field_gate() {
    let fixture = read_structured_data_lino(include_str!(
        "../../../parity/fixtures/typed-program-neutral-type-defaults.lino"
    ))
    .unwrap();
    let schema = typed_program_links_schema();
    for case in fixture["cases"].as_array().unwrap() {
        let root = LinkId::from_u64(case["archive"]["root"].as_str().unwrap().parse().unwrap());
        assert_eq!(case["negativeTargets"].as_array().unwrap().len(), 6);
        for target in case["negativeTargets"].as_array().unwrap() {
            for value in [false, true] {
                let mut network =
                    LinkNetwork::from_lino(case["archive"]["text"].as_str().unwrap()).unwrap();
                let scalar = network.insert_link(
                    [],
                    LinkMetadata::new()
                        .with_link_type(LinkType::Semantic)
                        .with_named(true)
                        .with_language(schema["language"].as_str().unwrap())
                        .with_term(schema["scalars"]["boolean"].as_str().unwrap())
                        .with_definition(if value { "true" } else { "false" }),
                );
                let field = target["field"].as_str().unwrap();
                let label = schema["fields"][field].as_str().unwrap_or(field);
                network.insert_field(
                    LinkId::from_u64(target["target"].as_str().unwrap().parse().unwrap()),
                    label,
                    scalar,
                );
                let error = decode_typed_program_links(&network, root)
                    .unwrap_err()
                    .to_string();
                assert!(
                    error.contains(&format!("does not yet represent field {field}")),
                    "{}: {error}",
                    target["id"]
                );
            }
        }
    }
}
