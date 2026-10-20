//! Source-derived typed packing is separate from complete owned-model validation.
#[path = "../../../experiments/grammar-outcome-network/native-transport/inverse/native-constructor-storage/mod.rs"]
mod storage;

use serde_json::Value;
use storage::{NativeConstructorSlot, NativeConstructorValue};

static EXISTING_REFERENCE: &str = "constructor static reference";

fn constructor_controls() -> Value {
    meta_language::read_structured_data_lino(include_str!(
        "../../../parity/fixtures/native-constructor-storage.lino"
    ))
    .unwrap()
}

fn instantiate_constructor_control(recipe: &Value) -> Result<NativeConstructorSlot, String> {
    let children = recipe["children"]
        .as_array()
        .unwrap()
        .iter()
        .map(instantiate_constructor_control)
        .collect::<Result<Vec<_>, _>>()?;
    let scalar = if recipe["scalar"].is_null() {
        None
    } else {
        let value = &recipe["scalar"]["value"];
        Some(match recipe["scalar"]["kind"].as_str().unwrap() {
            "Boolean" => NativeConstructorValue::Boolean(value.as_bool().unwrap()),
            "Text" => NativeConstructorValue::Text(value.as_str().unwrap().to_owned()),
            "Size" => NativeConstructorValue::Size(value.as_str().unwrap().parse().unwrap()),
            "SignedInteger" => {
                NativeConstructorValue::SignedInteger(value.as_str().unwrap().parse().unwrap())
            }
            "StructuredValue" => NativeConstructorValue::StructuredValue(value.clone()),
            "ExistingStaticReference" => {
                assert_eq!(value.as_str().unwrap(), EXISTING_REFERENCE);
                NativeConstructorValue::ExistingStaticReference(EXISTING_REFERENCE)
            }
            other => return Err(format!("unknown scalar control {other}")),
        })
    };
    storage::construct_source_slot(
        recipe["reference"].as_str().unwrap(),
        recipe["choice"].as_str().unwrap(),
        children,
        scalar,
    )
}

#[test]
fn every_captured_native_constructor_branch_packs_actual_typed_source_values() {
    let controls = constructor_controls();
    let references = controls["sourceReferences"].as_array().unwrap();
    assert_ne!(references, &Vec::<Value>::new());
    assert_eq!(references.len(), storage::SOURCE_REFERENCES.len());
    for (expected, actual) in references.iter().zip(storage::SOURCE_REFERENCES) {
        assert_eq!(expected.as_str().unwrap(), *actual);
    }
    let cases = controls["cases"].as_array().unwrap();
    assert!(cases.len() > references.len());
    let mut reached = std::collections::BTreeSet::new();
    for case in cases {
        let value = instantiate_constructor_control(&case["recipe"]).unwrap();
        assert_eq!(
            value.type_reference,
            case["sourceReference"].as_str().unwrap()
        );
        assert_eq!(
            value.value.storage_type(),
            case["expectedStorageType"].as_str().unwrap()
        );
        reached.insert(value.type_reference);
        if let NativeConstructorValue::ParseOutcome(outcome) = value.value {
            assert_eq!(
                outcome.ok,
                case["recipe"]["children"][0]["scalar"]["value"]
                    .as_bool()
                    .unwrap()
            );
            assert!(outcome.tree.is_none());
            assert_eq!(outcome.ambiguities, Vec::<meta_language::Ambiguity>::new());
            assert!(outcome.rejection.is_none());
            assert!(outcome.source_planes.is_none());
        }
    }
    assert_eq!(
        reached.into_iter().collect::<Vec<_>>(),
        storage::SOURCE_REFERENCES
    );
}

#[test]
fn typed_storage_retains_original_number_variants_payloads_and_the_actual_static_pointer() {
    let controls = constructor_controls();
    let cases = controls["cases"].as_array().unwrap();
    let json = cases
        .iter()
        .find(|case| case["recipe"]["scalar"]["kind"] == "StructuredValue")
        .unwrap();
    let numbers = [
        serde_json::Number::from(0_u64),
        serde_json::Number::from(-1_i64),
        serde_json::Number::from(1_u64 << 60),
        serde_json::Number::from_f64(-0.0).unwrap(),
        serde_json::Number::from_f64(1.5).unwrap(),
        serde_json::Number::from_f64(f64::from_bits(1)).unwrap(),
    ];
    for original in numbers {
        let result = storage::construct_source_slot(
            json["sourceReference"].as_str().unwrap(),
            "",
            vec![],
            Some(NativeConstructorValue::StructuredValue(Value::Number(
                original.clone(),
            ))),
        )
        .unwrap();
        let NativeConstructorValue::StructuredValue(value) = result.value else {
            panic!("actual JSON storage required")
        };
        let actual = value.as_number().unwrap();
        assert_eq!(actual.is_i64(), original.is_i64());
        assert_eq!(actual.is_u64(), original.is_u64());
        assert_eq!(actual.is_f64(), original.is_f64());
        assert_eq!(actual.as_i64(), original.as_i64());
        assert_eq!(actual.as_u64(), original.as_u64());
        assert_eq!(
            actual.as_f64().unwrap().to_bits(),
            original.as_f64().unwrap().to_bits()
        );
    }
    let reference = cases
        .iter()
        .find(|case| case["recipe"]["scalar"]["kind"] == "ExistingStaticReference")
        .unwrap();
    let result = instantiate_constructor_control(&reference["recipe"]).unwrap();
    let NativeConstructorValue::ExistingStaticReference(actual) = result.value else {
        panic!("existing static reference required")
    };
    assert!(std::ptr::eq(actual.as_ptr(), EXISTING_REFERENCE.as_ptr()));
    assert_eq!(actual.len(), EXISTING_REFERENCE.len());
}

#[test]
fn foreign_reordered_missing_and_extra_source_values_refuse_without_new_lifetimes() {
    let controls = constructor_controls();
    let cases = controls["cases"].as_array().unwrap();
    assert!(storage::construct_source_slot("foreign", "", vec![], None).is_err());
    let text = cases
        .iter()
        .find(|case| case["recipe"]["scalar"]["kind"] == "Text")
        .unwrap();
    assert!(
        storage::construct_source_slot(
            text["sourceReference"].as_str().unwrap(),
            "",
            vec![],
            Some(NativeConstructorValue::Boolean(false))
        )
        .is_err()
    );
    let optional = cases
        .iter()
        .find(|case| case["recipe"]["choice"] == "Some")
        .unwrap();
    let foreign = NativeConstructorSlot {
        type_reference: "foreign",
        value: NativeConstructorValue::Boolean(false),
    };
    assert!(
        storage::construct_source_slot(
            optional["sourceReference"].as_str().unwrap(),
            "Some",
            vec![foreign],
            None
        )
        .is_err()
    );
    assert!(
        storage::construct_source_slot(
            optional["sourceReference"].as_str().unwrap(),
            "Some",
            vec![],
            None
        )
        .is_err()
    );
    let child = instantiate_constructor_control(&text["recipe"]).unwrap();
    assert!(
        storage::construct_source_slot(
            optional["sourceReference"].as_str().unwrap(),
            "None",
            vec![child],
            None
        )
        .is_err()
    );
    let variant = cases
        .iter()
        .find(|case| {
            case["recipe"]["choice"]
                .as_str()
                .unwrap()
                .starts_with("declaration:")
        })
        .unwrap();
    assert!(
        storage::construct_source_slot(
            variant["sourceReference"].as_str().unwrap(),
            "foreign variant",
            vec![],
            None
        )
        .is_err()
    );
    let reference = cases
        .iter()
        .find(|case| case["recipe"]["scalar"]["kind"] == "ExistingStaticReference")
        .unwrap();
    let mut changed = reference["recipe"].clone();
    changed["children"][0]["scalar"]["value"] = Value::String("different source text".to_owned());
    assert!(instantiate_constructor_control(&changed).is_err());
    let oversized = (0..100_001)
        .map(|_| NativeConstructorSlot {
            type_reference: "foreign",
            value: NativeConstructorValue::Boolean(false),
        })
        .collect();
    assert!(
        storage::construct_source_slot(
            text["sourceReference"].as_str().unwrap(),
            "",
            oversized,
            None
        )
        .unwrap_err()
        .contains("budget")
    );
}

#[test]
fn reordered_equal_storage_types_and_duplicate_or_incomplete_native_map_pairs_refuse() {
    let controls = constructor_controls();
    let cases = controls["cases"].as_array().unwrap();
    let record = cases
        .iter()
        .find(|case| case["expectedStorageType"] == "meta_language::Ambiguity")
        .unwrap();
    let mut reordered = record["recipe"].clone();
    reordered["children"].as_array_mut().unwrap().swap(1, 2);
    assert!(
        instantiate_constructor_control(&reordered)
            .unwrap_err()
            .contains("reordered")
    );
    let map = cases
        .iter()
        .find(|case| {
            case["expectedStorageType"]
                .as_str()
                .unwrap()
                .starts_with("indexmap::IndexMap<")
                && case["recipe"]["children"].as_array().unwrap().len() == 2
        })
        .unwrap();
    let mut incomplete = map["recipe"].clone();
    assert!(
        incomplete["children"]
            .as_array_mut()
            .unwrap()
            .pop()
            .is_some()
    );
    assert!(
        instantiate_constructor_control(&incomplete)
            .unwrap_err()
            .contains("missing")
    );
    let mut duplicate = map["recipe"].clone();
    let pair = duplicate["children"].as_array().unwrap().clone();
    duplicate["children"].as_array_mut().unwrap().extend(pair);
    assert!(
        instantiate_constructor_control(&duplicate)
            .unwrap_err()
            .contains("duplicate")
    );
}
