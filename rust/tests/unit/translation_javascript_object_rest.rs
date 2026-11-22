//! Deferred original-source, shallow identity, order and complete graph products.
use super::translation_rust_records::assert_nominal_translation_product;
use meta_language::translation::{
    check::check_program, emit_javascript::emit_javascript, emit_rust::emit_rust,
    javascript::parse_javascript,
};
#[test]
fn lexical_object_rest_keeps_actual_fields_order_and_fresh_shallow_identity() {
    super::translation_javascript_object_binding_declarations::require_supported_rust_version();
    assert_nominal_translation_product(
        "../experiments/javascript-program/object-rest-bindings.mjs",
        "JavaScript actual closed own-field lexical object rest",
        || {
            meta_language::create_source_definition_program_links(
                "meta-language/source-program/object-rest-program",
            )
            .map_err(|message| meta_language::TypedProgramLinksError { message })
        },
    );
}

#[test]
fn nominal_rest_infers_the_owner_and_keeps_optional_field_presence() {
    use super::issue_195_translation_pairs::run;
    use meta_language::{
        BinaryLinksOptions, LinkNetwork, decode_link_network_binary, decode_typed_program_links,
        encode_link_network_binary, encode_typed_program_links,
    };
    use std::process::Command;

    let source = include_str!("../../../experiments/javascript-program/nominal-object-rest.mjs");
    let program = check_program(&parse_javascript(source).unwrap()).unwrap();
    let packet = encode_typed_program_links(&program).unwrap();
    let options = BinaryLinksOptions::default();
    let bytes = encode_link_network_binary(&packet.network, options).unwrap();
    let javascript = emit_javascript(&program).unwrap();
    let native = emit_rust(&program).unwrap();
    for network in [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(&bytes, options).unwrap(),
    ] {
        let restored = decode_typed_program_links(&network, packet.root).unwrap();
        assert_eq!(emit_javascript(&restored).unwrap().text, javascript.text);
        let restored_native = emit_rust(&restored).unwrap();
        assert_eq!(restored_native.preludes, native.preludes);
        assert_eq!(restored_native.definitions, native.definitions);
    }
    let entry = javascript
        .mappings
        .iter()
        .find(|mapping| mapping.kind == "function" && mapping.source == "compute")
        .unwrap();
    for (source, entry) in [
        (source, "compute"),
        (&javascript.text, entry.target.as_str()),
    ] {
        let result = run(Command::new("node").args([
            "-e",
            &format!("{source}\nif ({entry}() !== true) process.exit(1);"),
        ]));
        assert!(result.ok, "{}", result.stderr);
    }
    for selector in ["value", "$:tag"] {
        let invalid = source.replace(
            "const {child,...rest}",
            &format!("const {{{selector},...rest}}"),
        );
        let error = parse_javascript(&invalid).unwrap_err();
        assert!(
            error
                .reason
                .contains("actual tag and every required constructor field"),
            "{error}"
        );
    }
}
