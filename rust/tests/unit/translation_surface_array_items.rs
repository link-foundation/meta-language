//! Surface flag presence remains separate from Boolean spread behavior.
use meta_language::translation::surface::{SArrayItem, SExpr};
use serde_json::{Value, json};

#[test]
fn omitted_and_explicit_spread_flags_keep_exact_wire_presence_and_boolean_values() {
    let value = SExpr::name("value");
    let omitted = json!({"value": serde_json::to_value(&value).unwrap()});
    let decoded: SArrayItem = serde_json::from_value(omitted.clone()).unwrap();
    assert!(!decoded.spread);
    assert_eq!(decoded.value, value);
    assert_eq!(serde_json::to_value(decoded.clone()).unwrap(), omitted);
    for spread in [false, true] {
        let expected = json!({"spread": spread, "value": serde_json::to_value(&value).unwrap()});
        let explicit: SArrayItem = serde_json::from_value(expected.clone()).unwrap();
        assert_eq!(explicit.spread, spread);
        assert_eq!(serde_json::to_value(explicit).unwrap(), expected);
        assert_eq!(
            serde_json::to_value(SArrayItem::new(value.clone(), spread)).unwrap(),
            expected
        );
    }
    assert_eq!(decoded, SArrayItem::new(value, false));
    assert_eq!(
        serde_json::to_string(&SArrayItem::new(SExpr::name("value"), false)).unwrap(),
        r#"{"spread":false,"value":{"k":"name","path":["value"]}}"#
    );
}

#[test]
fn public_boolean_edits_cannot_hide_a_true_spread_or_erase_an_explicit_false() {
    let mut omitted: SArrayItem =
        serde_json::from_value(json!({"value":{"k":"name","path":["value"]}})).unwrap();
    omitted.spread = true;
    assert_eq!(serde_json::to_value(&omitted).unwrap()["spread"], true);
    let mut explicit = SArrayItem::new(SExpr::name("value"), true);
    explicit.spread = false;
    assert_eq!(serde_json::to_value(&explicit).unwrap()["spread"], false);
    assert!(
        serde_json::to_value(&explicit)
            .unwrap()
            .get("spread")
            .is_some()
    );
}

#[test]
fn present_null_nonboolean_duplicate_and_missing_values_keep_host_refusals() {
    for spread in [Value::Null, json!(0), json!("false"), json!([]), json!({})] {
        assert!(
            serde_json::from_value::<SArrayItem>(
                json!({"spread":spread,"value":{"k":"name","path":["value"]}})
            )
            .is_err()
        );
    }
    assert!(serde_json::from_value::<SArrayItem>(json!({"spread":false})).is_err());
    assert!(
        serde_json::from_str::<SArrayItem>(
            r#"{"spread":false,"spread":true,"value":{"k":"name","path":["value"]}}"#
        )
        .is_err()
    );
}

fn shared_math_arguments(value: &Value) -> usize {
    let mut count = 0;
    if value["k"] == "math" {
        for argument in value["args"].as_array().unwrap() {
            let decoded: SArrayItem = serde_json::from_value(argument.clone()).unwrap();
            assert_eq!(serde_json::to_value(decoded).unwrap(), *argument);
            count += 1;
        }
    }
    match value {
        Value::Array(values) => {
            count += values.iter().map(shared_math_arguments).sum::<usize>();
        }
        Value::Object(fields) => {
            count += fields.values().map(shared_math_arguments).sum::<usize>();
        }
        _ => {}
    }
    count
}

#[test]
fn actual_shared_float_surface_arguments_round_trip_without_inventing_spread_fields() {
    for source in [
        include_str!("../../../parity/fixtures/rust-float-predicates.lino"),
        include_str!("../../../parity/fixtures/rust-float-operations.lino"),
        include_str!("../../../parity/fixtures/rust-float-constants.lino"),
    ] {
        let fixture = meta_language::read_structured_data_lino(source).unwrap();
        let count = fixture["cases"]
            .as_array()
            .unwrap()
            .iter()
            .map(|case| shared_math_arguments(&case["surface"]))
            .sum::<usize>();
        assert!(count > 0);
    }
}
