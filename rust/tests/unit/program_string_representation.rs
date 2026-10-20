use meta_language::translation::ir::Program;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, create_typed_program_links, decode_link_network_binary,
    decode_typed_program_links, emit_typed_program_links, encode_link_network_binary,
    encode_typed_program_links,
};
use serde_json::{Value, json};

fn original_program() -> Program {
    let packet = create_typed_program_links(
        "/** @param {number} value @returns {number} */\nfunction identity(value) { return value; }\nconsole.log(identity(3));",
        "JavaScript",
    )
    .unwrap();
    decode_typed_program_links(&packet.network, packet.root).unwrap()
}

#[test]
fn explicit_string_representation_preserves_boolean_presence_through_both_native_archives() {
    let original = original_program();
    for flag in [None, Some(false), Some(true)] {
        let mut program = original.clone();
        program.utf16_strings = flag;
        let value = serde_json::to_value(&program).unwrap();
        assert_eq!(value.get("utf16Strings").is_some(), flag.is_some());
        if let Some(flag) = flag {
            assert_eq!(value["utf16Strings"], flag);
        }
        let packet = encode_typed_program_links(&program).unwrap();
        for network in [
            LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
            decode_link_network_binary(
                &encode_link_network_binary(&packet.network, BinaryLinksOptions::default())
                    .unwrap(),
                BinaryLinksOptions::default(),
            )
            .unwrap(),
        ] {
            assert_eq!(
                decode_typed_program_links(&network, packet.root).unwrap(),
                program
            );
            let before = network.to_lino();
            let native = emit_typed_program_links(&network, packet.root, "Rust").unwrap();
            assert_eq!(
                native.encodings.iter().any(|row| row.id == "utf16-strings"),
                flag == Some(true)
            );
            assert!(
                emit_typed_program_links(&network, packet.root, "JavaScript")
                    .unwrap()
                    .code
                    .contains("identity")
            );
            assert_eq!(network.to_lino(), before);
        }
    }
}

#[test]
fn present_program_string_representation_rejects_null_and_every_non_boolean_shape() {
    let original = serde_json::to_value(original_program()).unwrap();
    assert!(original.get("utf16Strings").is_none());
    for invalid in [
        Value::Null,
        json!(0),
        json!(1),
        json!("false"),
        json!([]),
        json!({}),
    ] {
        let mut changed = original.clone();
        changed["utf16Strings"] = invalid;
        assert!(serde_json::from_value::<Program>(changed).is_err());
    }
}
