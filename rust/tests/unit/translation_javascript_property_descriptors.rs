//! Deferred native parity; the semantics are the actual source-generated module.
#[path = "../../src/translation/javascript_property_descriptors.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use meta_language::read_structured_data_lino;
use native::JavaScriptSemanticValue as Value;
use serde_json::Value as Data;
use std::process::Command;

type Descriptor = (
    bool,
    bool,
    bool,
    bool,
    Value,
    bool,
    Value,
    bool,
    Value,
    bool,
    bool,
    bool,
);

fn value(data: &Data) -> Value {
    match data["$"].as_str().unwrap() {
        "undefined" => Value::Undefined,
        "null" => Value::Null,
        "boolean" => Value::Boolean(data["value"].as_bool().unwrap()),
        "number" => Value::Number(f64::from_bits(
            u64::from_str_radix(data["bits"].as_str().unwrap(), 16).unwrap(),
        )),
        "integer" => Value::Integer(native::ml::Big::parse(data["value"].as_str().unwrap())),
        "text" => Value::Text(
            data["units"]
                .as_array()
                .unwrap()
                .iter()
                .map(|unit| unit.as_f64().unwrap())
                .collect(),
        ),
        "symbol" => Value::Symbol(native::ml::Big::parse(data["identity"].as_str().unwrap())),
        "object" => Value::Object(
            native::ml::Big::parse(data["identity"].as_str().unwrap()),
            data["callable"].as_bool().unwrap(),
        ),
        other => panic!("uncovered semantic operand {other}"),
    }
}
fn descriptor(data: &Data) -> Descriptor {
    (
        data["configurable"].as_bool().unwrap(),
        data["configurablePresent"].as_bool().unwrap(),
        data["enumerable"].as_bool().unwrap(),
        data["enumerablePresent"].as_bool().unwrap(),
        value(&data["getter"]),
        data["getterPresent"].as_bool().unwrap(),
        value(&data["setter"]),
        data["setterPresent"].as_bool().unwrap(),
        value(&data["value"]),
        data["valuePresent"].as_bool().unwrap(),
        data["writable"].as_bool().unwrap(),
        data["writablePresent"].as_bool().unwrap(),
    )
}
fn assert_value(actual: &Value, expected: &Data) {
    match (actual, value(expected)) {
        (Value::Number(actual), Value::Number(expected)) => assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "complete stored IEEE bits"
        ),
        (actual, expected) => assert_eq!(*actual, expected),
    }
}
fn assert_descriptor(actual: &Descriptor, expected: &Data) {
    assert_eq!(actual.0, expected["configurable"].as_bool().unwrap());
    assert_eq!(actual.1, expected["configurablePresent"].as_bool().unwrap());
    assert_eq!(actual.2, expected["enumerable"].as_bool().unwrap());
    assert_eq!(actual.3, expected["enumerablePresent"].as_bool().unwrap());
    assert_value(&actual.4, &expected["getter"]);
    assert_eq!(actual.5, expected["getterPresent"].as_bool().unwrap());
    assert_value(&actual.6, &expected["setter"]);
    assert_eq!(actual.7, expected["setterPresent"].as_bool().unwrap());
    assert_value(&actual.8, &expected["value"]);
    assert_eq!(actual.9, expected["valuePresent"].as_bool().unwrap());
    assert_eq!(actual.10, expected["writable"].as_bool().unwrap());
    assert_eq!(actual.11, expected["writablePresent"].as_bool().unwrap());
}

#[test]
fn source_generated_property_descriptors_preserve_complete_observations() {
    let version = run(Command::new("rustc").arg("--version"));
    assert!(version.ok, "{}", version.stderr);
    let actual = version.stdout.split_whitespace().nth(1).unwrap();
    let parts = actual
        .split('.')
        .map(|part| part.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    assert!(
        parts[0] == 1 && (90..100).contains(&parts[1]),
        "uncovered Rust {actual}; >=1.90 <1.100"
    );
    let producer = run(Command::new("node")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "--max-old-space-size=352",
            "--max-semi-space-size=1",
            "../experiments/javascript-properties/oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let observations = read_structured_data_lino(&producer.stdout).unwrap();
    for row in observations["cases"].as_array().unwrap() {
        let (actual, disposition, error, writes) = native::plan_java_script_property_definition(
            row["exists"].as_bool().unwrap(),
            row["extensible"].as_bool().unwrap(),
            descriptor(&row["current"]),
            descriptor(&row["proposed"]),
        );
        let expected = &row["result"];
        assert_eq!(
            disposition,
            expected["disposition"].as_str().unwrap(),
            "{}",
            row["name"]
        );
        assert_eq!(
            error,
            expected["error"].as_str().unwrap(),
            "{}",
            row["name"]
        );
        assert_eq!(
            writes,
            expected["writes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|field| field.as_str().unwrap().to_owned())
                .collect::<Vec<_>>()
        );
        assert_descriptor(&actual, &expected["descriptor"]);
    }
    for row in observations["sameValues"].as_array().unwrap() {
        assert_eq!(
            native::test_java_script_semantic_same_value(value(&row["left"]), value(&row["right"])),
            row["actual"].as_bool().unwrap(),
            "{}",
            row["name"]
        );
    }
    for row in observations["invalidValues"].as_array().unwrap() {
        assert_eq!(
            native::read_java_script_semantic_value_error(value(&row["value"])),
            row["actual"].as_str().unwrap(),
            "{}",
            row["name"]
        );
        let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            native::test_java_script_semantic_same_value(value(&row["value"]), Value::Undefined)
        }))
        .expect_err("invalid SameValue operands throw");
        let payload = failure
            .downcast_ref::<(String, Vec<u16>, bool)>()
            .expect("complete source exception");
        assert_eq!(payload.0, "TypeError");
        assert_eq!(
            payload.1,
            "SameValue requires two complete values in the declared semantic model"
                .encode_utf16()
                .collect::<Vec<_>>()
        );
        assert!(payload.2);
    }
    for row in observations["textBoundaries"].as_array().unwrap() {
        let units = vec![65.0; usize::try_from(row["units"].as_u64().unwrap()).unwrap()];
        assert_eq!(
            native::read_java_script_semantic_value_error(Value::Text(units.clone())),
            row["actual"].as_str().unwrap()
        );
        if row["actual"] == "" {
            assert!(native::test_java_script_semantic_same_value(
                Value::Text(units.clone()),
                Value::Text(units)
            ));
        }
    }
}
