//! Raw wire facets preserve presence independently of checked representation.
use meta_language::translation::check::check_program;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{SExpr, SProgram, SurfaceRawMetadata};
use meta_language::translation::surface_raw_metadata_rules::{
    accept_shared_surface_reference_kind, accept_surface_raw_metadata_field,
    select_surface_reference_semantics,
};
use meta_language::translation::types::Type;
use meta_language::{LinkNetwork, decode_typed_program_links, encode_typed_program_links};
use serde_json::{Value, json};

const IDENTITY: &str =
    "/** @param {boolean} value @returns {boolean} */ function identity(value) { return value; }";

#[test]
fn omitted_null_false_and_true_raw_metadata_keep_their_exact_shapes() {
    for metadata in [
        None,
        Some(Value::Null),
        Some(json!(false)),
        Some(json!(true)),
    ] {
        let mut expression = json!({"k":"name","path":["value"]});
        let mut program = json!({"language":"JavaScript","items":[],"main":null});
        if let Some(value) = &metadata {
            expression["parenthesized"] = value.clone();
            program["referenceSemantics"] = value.clone();
        }
        let parsed: SExpr = serde_json::from_value(expression.clone()).unwrap();
        assert_eq!(parsed.parenthesized.is_some(), metadata.is_some());
        assert_eq!(serde_json::to_value(parsed.clone()).unwrap(), expression);
        assert_eq!(
            parsed
                .parenthesized
                .as_ref()
                .and_then(SurfaceRawMetadata::as_boolean),
            metadata.as_ref().and_then(Value::as_bool)
        );
        let surface: SProgram = serde_json::from_value(program.clone()).unwrap();
        assert_eq!(surface.reference_semantics.is_some(), metadata.is_some());
        assert_eq!(serde_json::to_value(&surface).unwrap(), program);
        let checked = check_program(&surface).unwrap();
        let selected = metadata.as_ref().and_then(Value::as_bool) == Some(true);
        assert_eq!(checked.reference_semantics, selected);
        assert_eq!(
            serde_json::to_value(checked)
                .unwrap()
                .get("referenceSemantics"),
            selected.then_some(&json!(true))
        );
    }
    assert_eq!(SurfaceRawMetadata::null().as_boolean(), None);
    assert_eq!(SurfaceRawMetadata::from(false).as_boolean(), Some(false));
}

#[test]
fn invalid_present_metadata_and_duplicate_known_fields_still_refuse() {
    for value in [json!(0), json!("true"), json!([]), json!({})] {
        assert!(
            serde_json::from_value::<SExpr>(json!({
                "k":"name","path":["value"],"parenthesized":value
            }))
            .is_err()
        );
        assert!(
            serde_json::from_value::<SProgram>(json!({
                "language":"JavaScript","items":[],"main":null,"referenceSemantics":value
            }))
            .is_err()
        );
    }
    assert!(
        serde_json::from_str::<SExpr>(
            r#"{"k":"name","path":["value"],"parenthesized":null,"parenthesized":false}"#
        )
        .is_err()
    );
    assert!(serde_json::from_str::<SProgram>(
        r#"{"language":"JavaScript","items":[],"main":null,"referenceSemantics":null,"referenceSemantics":false}"#
    ).is_err());
}

#[test]
fn actual_grouping_is_marked_and_selected_scalar_programs_survive_source_free_replay() {
    let plain = parse_javascript(IDENTITY).unwrap();
    let grouped =
        parse_javascript(&IDENTITY.replace("return value;", "return ((value));")).unwrap();
    let plain_json = serde_json::to_value(&plain).unwrap();
    let grouped_json = serde_json::to_value(&grouped).unwrap();
    assert!(
        plain_json["items"][0]["body"]
            .get("parenthesized")
            .is_none()
    );
    assert_eq!(grouped_json["items"][0]["body"]["parenthesized"], true);
    let mut selected = grouped;
    selected.reference_semantics = Some(true.into());
    let checked = check_program(&selected).unwrap();
    assert!(checked.reference_semantics);
    let packet = encode_typed_program_links(&checked).unwrap();
    let reloaded = LinkNetwork::from_lino(&packet.network.to_lino()).unwrap();
    assert_eq!(
        decode_typed_program_links(&reloaded, packet.root).unwrap(),
        checked
    );
}

#[test]
fn existing_structural_reference_flags_retain_all_four_recursive_type_shapes() {
    let mut nested = json!({"kind":"bool"});
    for kind in ["array", "record", "map", "set"] {
        nested = match kind {
            "record" => {
                json!({"kind":kind,"reference":true,"fields":[{"name":"value","type":nested}]})
            }
            "map" => json!({"kind":kind,"reference":true,"key":{"kind":"bool"},"value":nested}),
            _ => json!({"kind":kind,"reference":true,"element":nested}),
        };
        let ty: Type = serde_json::from_value(nested.clone()).unwrap();
        assert!(ty.is_reference());
        assert_eq!(serde_json::to_value(ty).unwrap(), nested);
        assert!(accept_shared_surface_reference_kind(kind));
    }
    assert!(accept_surface_raw_metadata_field(
        "expression",
        "parenthesized"
    ));
    assert!(accept_surface_raw_metadata_field(
        "program",
        "referenceSemantics"
    ));
    assert!(!accept_surface_raw_metadata_field(
        "type",
        "referenceSemantics"
    ));
    assert!(!accept_surface_raw_metadata_field(
        "program",
        "parenthesized"
    ));
    assert!(select_surface_reference_semantics(true, false, false));
    assert!(select_surface_reference_semantics(false, true, true));
    assert!(!select_surface_reference_semantics(false, true, false));
}
