//! Deferred full normalized-state parity; all heap semantics are generated.
#[path = "../../src/translation/javascript_ordinary_heap.rs"]
mod native;
use super::issue_195_translation_pairs::run;
use meta_language::read_structured_data_lino;
use native::{
    JavaScriptOrdinaryHeap as Heap, JavaScriptOrdinaryObject as Object,
    JavaScriptOrdinaryProperty as Property, JavaScriptSemanticValue as Value,
};
use serde_json::{Value as Data, json};
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
        other => panic!("uncovered semantic value {other}"),
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
fn heap(data: &Data) -> Heap {
    assert_eq!(data["$"], "ordinaryHeap");
    Heap::OrdinaryHeap(
        data["objects"]
            .as_array()
            .unwrap()
            .iter()
            .map(|object| {
                assert_eq!(object["$"], "ordinaryObject");
                Object::OrdinaryObject(
                    native::ml::Big::parse(object["identity"].as_str().unwrap()),
                    Box::new(value(&object["prototype"])),
                    object["extensible"].as_bool().unwrap(),
                    object["callable"].as_bool().unwrap(),
                    object["properties"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|property| {
                            assert_eq!(property["$"], "ordinaryProperty");
                            Property::OrdinaryProperty(
                                Box::new(value(&property["key"])),
                                descriptor(&property["descriptor"]),
                            )
                        })
                        .collect(),
                )
            })
            .collect(),
        data["symbols"]
            .as_array()
            .unwrap()
            .iter()
            .map(|identity| native::ml::Big::parse(identity.as_str().unwrap()))
            .collect(),
    )
}
fn encode_value(value: &Value) -> Data {
    match value {
        Value::Undefined => json!({"$":"undefined"}),
        Value::Null => json!({"$":"null"}),
        Value::Boolean(value) => json!({"$":"boolean","value":value}),
        Value::Number(value) => json!({"$":"number","bits":format!("{:016x}",value.to_bits())}),
        Value::Integer(value) => json!({"$":"integer","value":value.to_string()}),
        Value::Text(units) => {
            json!({"$":"text","units":units.iter().map(|unit| unit.to_string().parse::<serde_json::Number>().unwrap()).collect::<Vec<_>>()})
        }
        Value::Symbol(identity) => json!({"$":"symbol","identity":identity.to_string()}),
        Value::Object(identity, callable) => {
            json!({"$":"object","identity":identity.to_string(),"callable":callable})
        }
    }
}
fn encode_descriptor(data: &Descriptor) -> Data {
    json!({"configurable":data.0,"configurablePresent":data.1,"enumerable":data.2,"enumerablePresent":data.3,"getter":encode_value(&data.4),"getterPresent":data.5,
        "setter":encode_value(&data.6),"setterPresent":data.7,"value":encode_value(&data.8),"valuePresent":data.9,"writable":data.10,"writablePresent":data.11})
}
fn encode_heap(heap: &Heap) -> Data {
    let Heap::OrdinaryHeap(objects, symbols) = heap;
    json!({"$":"ordinaryHeap","symbols":symbols.iter().map(ToString::to_string).collect::<Vec<_>>(),"objects":objects.iter().map(|object|{
        let Object::OrdinaryObject(identity,prototype,extensible,callable,properties)=object;
        json!({"$":"ordinaryObject","identity":identity.to_string(),"prototype":encode_value(prototype),"extensible":extensible,"callable":callable,"properties":properties.iter().map(|property|{
            let Property::OrdinaryProperty(key,descriptor)=property;json!({"$":"ordinaryProperty","key":encode_value(key),"descriptor":encode_descriptor(descriptor)})
        }).collect::<Vec<_>>()})
    }).collect::<Vec<_>>()})
}
fn encode_mutation((error, heap, success): (String, Heap, bool)) -> Data {
    json!({"error":error,"heap":encode_heap(&heap),"success":success})
}

#[test]
fn generated_heap_keeps_complete_states_pending_calls_and_original_oracles() {
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
            "../experiments/javascript-properties/heap-oracle.mjs",
        ]));
    assert!(producer.ok, "{}", producer.stderr);
    let observations = read_structured_data_lino(&producer.stdout).unwrap();
    for row in observations["cases"].as_array().unwrap() {
        let state = heap(&row["heap"]);
        let identity = native::ml::Big::parse(row["identity"].as_str().unwrap());
        let key = value(&row["key"]);
        let receiver = value(&row["receiver"]);
        assert_eq!(
            native::read_java_script_ordinary_heap_error(state.clone()),
            ""
        );
        let actual = match row["operation"].as_str().unwrap() {
            "get" => {
                let (action, arguments, callee, error, receiver, value) =
                    native::plan_java_script_ordinary_get(state, identity, key, receiver);
                json!({"action":action,"arguments":arguments.iter().map(encode_value).collect::<Vec<_>>(),"callee":encode_value(&callee),"error":error,"receiver":encode_value(&receiver),"value":encode_value(&value)})
            }
            "set" => {
                let (action, arguments, callee, error, heap, receiver, success) =
                    native::plan_java_script_ordinary_set(
                        state,
                        identity,
                        key,
                        value(&row["value"]),
                        receiver,
                    );
                json!({"action":action,"arguments":arguments.iter().map(encode_value).collect::<Vec<_>>(),"callee":encode_value(&callee),"error":error,"heap":encode_heap(&heap),"receiver":encode_value(&receiver),"success":success})
            }
            "define" => {
                let (error, heap, success, writes) = native::define_java_script_ordinary_property(
                    state,
                    identity,
                    key,
                    descriptor(&row["descriptor"]),
                );
                json!({"error":error,"heap":encode_heap(&heap),"success":success,"writes":writes})
            }
            "delete" => encode_mutation(native::delete_java_script_ordinary_property(
                state, identity, key,
            )),
            "prevent" => encode_mutation(native::prevent_java_script_ordinary_extensions(
                state, identity,
            )),
            "prototype" => encode_mutation(native::set_java_script_ordinary_prototype(
                state,
                identity,
                value(&row["prototype"]),
            )),
            "insert" => encode_mutation(native::insert_java_script_ordinary_object(
                state,
                identity,
                value(&row["prototype"]),
            )),
            "symbol" => encode_mutation(native::register_java_script_ordinary_symbol(
                state, identity,
            )),
            "keys" => {
                let (error, keys) = native::read_java_script_ordinary_own_keys(state, identity);
                json!({"error":error,"keys":keys.iter().map(encode_value).collect::<Vec<_>>()})
            }
            other => panic!("uncovered heap operation {other}"),
        };
        assert_eq!(
            actual, row["result"],
            "every heap/result field: {}",
            row["name"]
        );
    }
    for row in observations["invalid"].as_array().unwrap() {
        assert_eq!(
            native::read_java_script_ordinary_heap_error(heap(&row["heap"])),
            row["actual"].as_str().unwrap(),
            "{}",
            row["name"]
        );
    }
}
