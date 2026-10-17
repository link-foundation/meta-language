//! A nullable annotation's value and its field presence are independent.
use meta_language::translation::check::check_program;
use meta_language::translation::javascript::parse_javascript;
use meta_language::translation::surface::{
    SExpr, SItem, SNode, SProgram, SurfaceBindingTypeAnnotation,
};
use meta_language::translation::types::{BOOL, INT};
use serde_json::{Value, json};

#[test]
fn omitted_null_and_concrete_binding_annotations_retain_exact_wire_shapes() {
    for annotation in [None, Some(Value::Null), Some(json!({"kind":"bool"}))] {
        let mut expected = json!({
            "k":"let", "name":"saved",
            "value":{"k":"name","path":["value"]},
            "body":{"k":"name","path":["saved"]}
        });
        if let Some(value) = &annotation {
            expected["type"] = value.clone();
        }
        let expression: SExpr = serde_json::from_value(expected.clone()).unwrap();
        let SNode::Let { ty, .. } = &expression.node else {
            panic!("the fixture must remain a let expression");
        };
        assert_eq!(ty.is_some(), annotation.is_some());
        assert_eq!(
            ty.as_ref().and_then(SurfaceBindingTypeAnnotation::as_ref),
            annotation
                .as_ref()
                .filter(|value| !value.is_null())
                .map(|_| &BOOL)
        );
        assert_eq!(serde_json::to_value(expression.clone()).unwrap(), expected);
        let actual = serde_json::to_string(&expression).unwrap();
        let expected = match annotation {
            None => {
                r#"{"k":"let","name":"saved","value":{"k":"name","path":["value"]},"body":{"k":"name","path":["saved"]}}"#
            }
            Some(Value::Null) => {
                r#"{"k":"let","name":"saved","type":null,"value":{"k":"name","path":["value"]},"body":{"k":"name","path":["saved"]}}"#
            }
            Some(_) => {
                r#"{"k":"let","name":"saved","type":{"kind":"bool"},"value":{"k":"name","path":["value"]},"body":{"k":"name","path":["saved"]}}"#
            }
        };
        assert_eq!(actual, expected);
    }
    assert_eq!(SurfaceBindingTypeAnnotation::null().into_option(), None);
    assert_eq!(
        SurfaceBindingTypeAnnotation::from(BOOL).into_option(),
        Some(BOOL)
    );
}

#[test]
fn present_invalid_binding_annotations_and_duplicate_fields_still_refuse() {
    for annotation in [json!(false), json!(0), json!("bool"), json!([]), json!({})] {
        assert!(
            serde_json::from_value::<SExpr>(json!({
                "k":"let", "name":"saved", "type":annotation,
                "value":{"k":"name","path":["value"]},
                "body":{"k":"name","path":["saved"]}
            }))
            .is_err()
        );
    }
    assert!(serde_json::from_str::<SExpr>(
        r#"{"k":"let","name":"saved","type":null,"type":{"kind":"bool"},"value":{"k":"name","path":["value"]},"body":{"k":"name","path":["saved"]}}"#
    ).is_err());
}

#[test]
fn annotation_presence_preserves_actual_checker_semantics_and_type_refusals() {
    let mut surface = parse_javascript(
        "/** @param {boolean} value @returns {boolean} */ function identity(value) { const saved = value; return saved; }",
    ).unwrap();
    let original = check_program(&surface).unwrap();
    for annotation in [
        None,
        Some(SurfaceBindingTypeAnnotation::null()),
        Some(BOOL.into()),
    ] {
        let SItem::Fn(function) = &mut surface.items[0] else {
            panic!("the original source must contain its actual function");
        };
        let SNode::Let { ty, .. } = &mut function.body.node else {
            panic!("the original function must retain its actual local binding");
        };
        *ty = annotation;
        assert_eq!(check_program(&surface).unwrap(), original);
    }
    let SItem::Fn(function) = &mut surface.items[0] else {
        unreachable!()
    };
    let SNode::Let { ty, .. } = &mut function.body.node else {
        unreachable!()
    };
    *ty = Some(INT.into());
    assert!(check_program(&surface).is_err());
}

#[test]
fn unchanged_scalar_reverse_surfaces_keep_every_annotation_after_host_round_trip() {
    for source in [
        include_str!("../../../parity/fixtures/rust-scalar-loops.lino"),
        include_str!("../../../parity/fixtures/rust-scalar-loop-declarations.lino"),
    ] {
        let fixture = meta_language::read_structured_data_lino(source).unwrap();
        for case in fixture["cases"].as_array().unwrap() {
            let expected = &case["javaScriptReverseSurface"];
            let surface: SProgram = serde_json::from_value(expected.clone()).unwrap();
            assert_eq!(serde_json::to_value(&surface).unwrap(), *expected);
            check_program(&surface).unwrap();
        }
    }
}
