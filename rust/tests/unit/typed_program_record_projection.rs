//! Canonical record rechecks retain actual owners and source-free field edits.
use meta_language::translation::check::check_program;
use meta_language::translation::ir::{Decl, Expr, LitValue, Node};
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{SExpr, SNode};
use meta_language::translation::types::Type;
use meta_language::{
    BinaryLinksOptions, LinkNetwork, TypedProgramLinks, decode_link_network_binary,
    decode_typed_program_links, emit_typed_program_links, encode_link_network_binary,
    encode_typed_program_links,
};
use serde_json::{Value, json};
use std::process::Command;

fn marked(entries: Value, marker: Value) -> Value {
    json!({"k":"record", "recordProjection":marker, "entries":entries})
}

fn value() -> Value {
    json!({"k":"num", "value":"5", "type":{"kind":"int"}})
}

#[test]
fn marked_record_surface_deserialization_checks_original_descriptors_before_defaults() {
    let marker = json!("checked ordered record values");
    let original = json!({"k":"record", "entries":[{"name":"a", "value":value()}]});
    let raw: SExpr = serde_json::from_value(original).unwrap();
    let SNode::Record {
        entries,
        record_projection,
        ..
    } = &raw.node
    else {
        panic!("the original raw Record API remains unchanged")
    };
    assert!(!entries[0].spread);
    assert!(record_projection.is_none());
    let canonical: SExpr = serde_json::from_value(marked(
        json!([{"name":"a", "spread":false, "value":value()}]),
        marker.clone(),
    ))
    .unwrap();
    assert_eq!(
        serde_json::to_value(&canonical).unwrap()["recordProjection"],
        marker
    );
    for invalid in [
        json!([{"name":"a", "value":value()}]),
        json!([{"name":"a", "spread":true, "value":value()}]),
        json!([{"name":"a", "spread":null, "value":value()}]),
        json!([{"name":"a", "spread":0, "value":value()}]),
        json!([{"name":"a", "spread":false, "value":value(), "extra":true}]),
        json!([{"name":null, "spread":false, "value":value()}]),
        json!([{"spread":false, "value":value()}]),
        json!([{"name":"a", "spread":false}]),
        json!([{"name":"a", "spread":false, "value":null}]),
        json!([null]),
    ] {
        let failure = serde_json::from_value::<SExpr>(marked(invalid, marker.clone())).unwrap_err();
        assert!(
            failure
                .to_string()
                .contains("checked record projection has a malformed field entry"),
            "{failure}"
        );
    }
    for entries in [
        json!([{"name":"b", "spread":false, "value":value()}, {"name":"a", "spread":false, "value":value()}]),
        json!([{"name":"a", "spread":false, "value":value()}, {"name":"a", "spread":false, "value":value()}]),
    ] {
        let failure = serde_json::from_value::<SExpr>(marked(entries, marker.clone())).unwrap_err();
        assert!(
            failure
                .to_string()
                .contains("checked record projection needs unique fields in UTF16 order"),
            "{failure}"
        );
    }
}

#[test]
fn present_invalid_record_markers_cannot_select_the_raw_surface_path() {
    for marker in [
        Value::Null,
        json!(false),
        json!(true),
        json!(""),
        json!("source record"),
        json!(0),
        json!({}),
    ] {
        let failure = serde_json::from_value::<SExpr>(marked(json!([]), marker)).unwrap_err();
        assert!(
            failure
                .to_string()
                .contains("unknown checked record projection"),
            "{failure}"
        );
    }
}

fn archives(packet: &TypedProgramLinks) -> [LinkNetwork; 2] {
    [
        LinkNetwork::from_lino(&packet.network.to_lino()).unwrap(),
        decode_link_network_binary(
            &encode_link_network_binary(&packet.network, BinaryLinksOptions::default()).unwrap(),
            BinaryLinksOptions::default(),
        )
        .unwrap(),
    ]
}

fn replace_nine(expression: &Expr, edits: &mut usize) -> Expr {
    let mut result = expression.map_children(&mut |child| replace_nine(child, edits));
    if result.ty == Type::Int
        && let Node::Lit {
            value: LitValue::Text(value),
        } = &mut result.node
        && value == "9"
    {
        *value = "17".to_owned();
        *edits += 1;
    }
    result
}

#[test]
fn canonical_record_programs_keep_strict_checks_and_source_free_value_edits() {
    let source = "function make(){return {z:9n,a:2n};}const value=make();console.log(\"%s\",String(value.a));console.log(\"%s\",String(value.z));";
    let mut surface = parse_javascript(source).unwrap();
    surface.reference_semantics = Some(false.into());
    let original = check_program(&surface).unwrap();
    let mut program = original.clone();
    for _ in 0..3 {
        let packet = encode_typed_program_links(&program).unwrap();
        for network in archives(&packet) {
            assert_eq!(
                decode_typed_program_links(&network, packet.root).unwrap(),
                original
            );
        }
        program = decode_typed_program_links(&packet.network, packet.root).unwrap();
    }
    let mut edits = 0;
    for declaration in &mut program.declarations {
        if let Decl::Fn(function) = declaration {
            function.body = replace_nine(&function.body, &mut edits);
        }
    }
    assert_eq!(edits, 1);
    let edited = encode_typed_program_links(&program).unwrap();
    for network in archives(&edited) {
        assert_eq!(
            decode_typed_program_links(&network, edited.root).unwrap(),
            program
        );
        let emitted = emit_typed_program_links(&network, edited.root, "JavaScript").unwrap();
        let observed = Command::new("node")
            .args(["-e", &emitted.code])
            .output()
            .unwrap();
        assert!(
            observed.status.success(),
            "{}",
            String::from_utf8_lossy(&observed.stderr)
        );
        assert_eq!(String::from_utf8(observed.stdout).unwrap(), "2\n17\n");
    }
}
