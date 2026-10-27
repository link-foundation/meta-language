// Generated from actual checked source declarations and target mappings.
// Transport only; no source operation semantics are implemented here.
use super::native;
use serde_json::{Value as Data, json};
fn wire_number(value: f64) -> Data {
    if value.to_bits() == 0x8000_0000_0000_0000 {
        return Data::Number(serde_json::Number::from_f64(value).unwrap());
    }
    Data::Number(value.to_string().parse().expect("finite model Number"))
}
pub(super) fn decode_java_script_ordinary_heap(data: &Data) -> native::JavaScriptOrdinaryHeap {
    match data["$"].as_str().expect("model constructor") {
        "ordinaryHeap" => native::JavaScriptOrdinaryHeap::OrdinaryHeap(
            data["objects"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_ordinary_object)
                .collect(),
            data["symbols"]
                .as_array()
                .expect("model array")
                .iter()
                .map(|wire_value| {
                    native::ml::Big::parse(wire_value.as_str().expect("model exact integer"))
                })
                .collect(),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_ordinary_heap(value: &native::JavaScriptOrdinaryHeap) -> Data {
    match value {
        native::JavaScriptOrdinaryHeap::OrdinaryHeap(wire_field_0, wire_field_1) => {
            json!({"$":"ordinaryHeap","objects":Data::Array(wire_field_0.iter().map(encode_java_script_ordinary_object).collect()),"symbols":Data::Array(wire_field_1.iter().map(|wire_value|Data::String(wire_value.to_string())).collect())})
        }
    }
}
pub(super) fn decode_java_script_ordinary_object(data: &Data) -> native::JavaScriptOrdinaryObject {
    match data["$"].as_str().expect("model constructor") {
        "ordinaryObject" => native::JavaScriptOrdinaryObject::OrdinaryObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            data["properties"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_ordinary_property)
                .collect(),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_ordinary_object(value: &native::JavaScriptOrdinaryObject) -> Data {
    match value {
        native::JavaScriptOrdinaryObject::OrdinaryObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"ordinaryObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":Data::Array(wire_field_4.iter().map(encode_java_script_ordinary_property).collect())})
        }
    }
}
pub(super) fn decode_java_script_semantic_value(data: &Data) -> native::JavaScriptSemanticValue {
    match data["$"].as_str().expect("model constructor") {
        "undefined" => native::JavaScriptSemanticValue::Undefined,
        "null" => native::JavaScriptSemanticValue::Null,
        "boolean" => native::JavaScriptSemanticValue::Boolean(
            data["value"].as_bool().expect("model Boolean"),
        ),
        "number" => native::JavaScriptSemanticValue::Number(f64::from_bits(
            u64::from_str_radix(data["bits"].as_str().expect("model IEEE bits"), 16)
                .expect("complete model IEEE bits"),
        )),
        "integer" => native::JavaScriptSemanticValue::Integer(native::ml::Big::parse(
            data["value"].as_str().expect("model exact integer"),
        )),
        "text" => native::JavaScriptSemanticValue::Text(
            data["units"]
                .as_array()
                .expect("model array")
                .iter()
                .map(|wire_value| wire_value.as_f64().expect("model Number"))
                .collect(),
        ),
        "symbol" => native::JavaScriptSemanticValue::Symbol(native::ml::Big::parse(
            data["identity"].as_str().expect("model exact integer"),
        )),
        "object" => native::JavaScriptSemanticValue::Object(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            data["callable"].as_bool().expect("model Boolean"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_semantic_value(value: &native::JavaScriptSemanticValue) -> Data {
    match value {
        native::JavaScriptSemanticValue::Undefined => json!({"$":"undefined"}),
        native::JavaScriptSemanticValue::Null => json!({"$":"null"}),
        native::JavaScriptSemanticValue::Boolean(wire_field_0) => {
            json!({"$":"boolean","value":Data::Bool(*wire_field_0)})
        }
        native::JavaScriptSemanticValue::Number(wire_field_0) => {
            json!({"$":"number","bits":Data::String(format!("{:016x}",wire_field_0.to_bits()))})
        }
        native::JavaScriptSemanticValue::Integer(wire_field_0) => {
            json!({"$":"integer","value":Data::String(wire_field_0.to_string())})
        }
        native::JavaScriptSemanticValue::Text(wire_field_0) => {
            json!({"$":"text","units":Data::Array(wire_field_0.iter().map(|wire_value|wire_number(*wire_value)).collect())})
        }
        native::JavaScriptSemanticValue::Symbol(wire_field_0) => {
            json!({"$":"symbol","identity":Data::String(wire_field_0.to_string())})
        }
        native::JavaScriptSemanticValue::Object(wire_field_0, wire_field_1) => {
            json!({"$":"object","identity":Data::String(wire_field_0.to_string()),"callable":Data::Bool(*wire_field_1)})
        }
    }
}
pub(super) fn decode_java_script_ordinary_property(
    data: &Data,
) -> native::JavaScriptOrdinaryProperty {
    match data["$"].as_str().expect("model constructor") {
        "ordinaryProperty" => native::JavaScriptOrdinaryProperty::OrdinaryProperty(
            Box::new(decode_java_script_semantic_value(&data["key"])),
            (
                data["descriptor"]["configurable"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["configurablePresent"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["enumerable"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["enumerablePresent"]
                    .as_bool()
                    .expect("model Boolean"),
                decode_java_script_semantic_value(&data["descriptor"]["getter"]),
                data["descriptor"]["getterPresent"]
                    .as_bool()
                    .expect("model Boolean"),
                decode_java_script_semantic_value(&data["descriptor"]["setter"]),
                data["descriptor"]["setterPresent"]
                    .as_bool()
                    .expect("model Boolean"),
                decode_java_script_semantic_value(&data["descriptor"]["value"]),
                data["descriptor"]["valuePresent"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["writable"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["writablePresent"]
                    .as_bool()
                    .expect("model Boolean"),
            ),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_ordinary_property(
    value: &native::JavaScriptOrdinaryProperty,
) -> Data {
    match value {
        native::JavaScriptOrdinaryProperty::OrdinaryProperty(wire_field_0, wire_field_1) => {
            json!({"$":"ordinaryProperty","key":encode_java_script_semantic_value(wire_field_0),"descriptor":json!({"configurable":Data::Bool(wire_field_1.0),"configurablePresent":Data::Bool(wire_field_1.1),"enumerable":Data::Bool(wire_field_1.2),"enumerablePresent":Data::Bool(wire_field_1.3),"getter":encode_java_script_semantic_value(&wire_field_1.4),"getterPresent":Data::Bool(wire_field_1.5),"setter":encode_java_script_semantic_value(&wire_field_1.6),"setterPresent":Data::Bool(wire_field_1.7),"value":encode_java_script_semantic_value(&wire_field_1.8),"valuePresent":Data::Bool(wire_field_1.9),"writable":Data::Bool(wire_field_1.10),"writablePresent":Data::Bool(wire_field_1.11)})})
        }
    }
}
pub(super) fn decode_java_script_descriptor_conversion_state(
    data: &Data,
) -> native::JavaScriptDescriptorConversionState {
    match data["$"].as_str().expect("model constructor") {
        "descriptorConversion" => {
            native::JavaScriptDescriptorConversionState::DescriptorConversion(
                Box::new(decode_java_script_semantic_value(&data["receiver"])),
                data["next"].as_f64().expect("model Number"),
                (
                    data["descriptor"]["configurable"]
                        .as_bool()
                        .expect("model Boolean"),
                    data["descriptor"]["configurablePresent"]
                        .as_bool()
                        .expect("model Boolean"),
                    data["descriptor"]["enumerable"]
                        .as_bool()
                        .expect("model Boolean"),
                    data["descriptor"]["enumerablePresent"]
                        .as_bool()
                        .expect("model Boolean"),
                    decode_java_script_semantic_value(&data["descriptor"]["getter"]),
                    data["descriptor"]["getterPresent"]
                        .as_bool()
                        .expect("model Boolean"),
                    decode_java_script_semantic_value(&data["descriptor"]["setter"]),
                    data["descriptor"]["setterPresent"]
                        .as_bool()
                        .expect("model Boolean"),
                    decode_java_script_semantic_value(&data["descriptor"]["value"]),
                    data["descriptor"]["valuePresent"]
                        .as_bool()
                        .expect("model Boolean"),
                    data["descriptor"]["writable"]
                        .as_bool()
                        .expect("model Boolean"),
                    data["descriptor"]["writablePresent"]
                        .as_bool()
                        .expect("model Boolean"),
                ),
                data["pending"].as_bool().expect("model Boolean"),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_descriptor_conversion_state(
    value: &native::JavaScriptDescriptorConversionState,
) -> Data {
    match value {
        native::JavaScriptDescriptorConversionState::DescriptorConversion(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"descriptorConversion","receiver":encode_java_script_semantic_value(wire_field_0),"next":wire_number(*wire_field_1),"descriptor":json!({"configurable":Data::Bool(wire_field_2.0),"configurablePresent":Data::Bool(wire_field_2.1),"enumerable":Data::Bool(wire_field_2.2),"enumerablePresent":Data::Bool(wire_field_2.3),"getter":encode_java_script_semantic_value(&wire_field_2.4),"getterPresent":Data::Bool(wire_field_2.5),"setter":encode_java_script_semantic_value(&wire_field_2.6),"setterPresent":Data::Bool(wire_field_2.7),"value":encode_java_script_semantic_value(&wire_field_2.8),"valuePresent":Data::Bool(wire_field_2.9),"writable":Data::Bool(wire_field_2.10),"writablePresent":Data::Bool(wire_field_2.11)}),"pending":Data::Bool(*wire_field_3)})
        }
    }
}
pub(super) fn decode_java_script_semantic_completion(
    data: &Data,
) -> native::JavaScriptSemanticCompletion {
    match data["$"].as_str().expect("model constructor") {
        "normalCompletion" => native::JavaScriptSemanticCompletion::NormalCompletion(Box::new(
            decode_java_script_semantic_value(&data["value"]),
        )),
        "throwCompletion" => native::JavaScriptSemanticCompletion::ThrowCompletion(Box::new(
            decode_java_script_semantic_value(&data["value"]),
        )),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_semantic_completion(
    value: &native::JavaScriptSemanticCompletion,
) -> Data {
    match value {
        native::JavaScriptSemanticCompletion::NormalCompletion(wire_field_0) => {
            json!({"$":"normalCompletion","value":encode_java_script_semantic_value(wire_field_0)})
        }
        native::JavaScriptSemanticCompletion::ThrowCompletion(wire_field_0) => {
            json!({"$":"throwCompletion","value":encode_java_script_semantic_value(wire_field_0)})
        }
    }
}
pub(super) fn decode_java_script_descriptor_conversion_transition(
    data: &Data,
) -> native::JavaScriptDescriptorConversionTransition {
    match data["$"].as_str().expect("model constructor") {
        "descriptorContinuation" => {
            native::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(
                decode_java_script_descriptor_conversion_state(&data["state"]),
            ))
        }
        "descriptorCall" => native::JavaScriptDescriptorConversionTransition::DescriptorCall(
            Box::new(decode_java_script_descriptor_conversion_state(
                &data["state"],
            )),
            Box::new(decode_java_script_semantic_value(&data["callee"])),
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            data["arguments"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_semantic_value)
                .collect(),
        ),
        "descriptorCompleted" => {
            native::JavaScriptDescriptorConversionTransition::DescriptorCompleted((
                data["descriptor"]["configurable"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["configurablePresent"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["enumerable"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["enumerablePresent"]
                    .as_bool()
                    .expect("model Boolean"),
                decode_java_script_semantic_value(&data["descriptor"]["getter"]),
                data["descriptor"]["getterPresent"]
                    .as_bool()
                    .expect("model Boolean"),
                decode_java_script_semantic_value(&data["descriptor"]["setter"]),
                data["descriptor"]["setterPresent"]
                    .as_bool()
                    .expect("model Boolean"),
                decode_java_script_semantic_value(&data["descriptor"]["value"]),
                data["descriptor"]["valuePresent"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["writable"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["writablePresent"]
                    .as_bool()
                    .expect("model Boolean"),
            ))
        }
        "descriptorThrow" => native::JavaScriptDescriptorConversionTransition::DescriptorThrow(
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        "descriptorIntrinsicError" => {
            native::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                data["errorClass"].as_str().expect("model text").to_owned(),
                data["reason"].as_str().expect("model text").to_owned(),
            )
        }
        "descriptorInvalid" => native::JavaScriptDescriptorConversionTransition::DescriptorInvalid(
            data["reason"].as_str().expect("model text").to_owned(),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_descriptor_conversion_transition(
    value: &native::JavaScriptDescriptorConversionTransition,
) -> Data {
    match value {
        native::JavaScriptDescriptorConversionTransition::DescriptorContinuation(wire_field_0) => {
            json!({"$":"descriptorContinuation","state":encode_java_script_descriptor_conversion_state(wire_field_0)})
        }
        native::JavaScriptDescriptorConversionTransition::DescriptorCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"descriptorCall","state":encode_java_script_descriptor_conversion_state(wire_field_0),"callee":encode_java_script_semantic_value(wire_field_1),"receiver":encode_java_script_semantic_value(wire_field_2),"arguments":Data::Array(wire_field_3.iter().map(encode_java_script_semantic_value).collect())})
        }
        native::JavaScriptDescriptorConversionTransition::DescriptorCompleted(wire_field_0) => {
            json!({"$":"descriptorCompleted","descriptor":json!({"configurable":Data::Bool(wire_field_0.0),"configurablePresent":Data::Bool(wire_field_0.1),"enumerable":Data::Bool(wire_field_0.2),"enumerablePresent":Data::Bool(wire_field_0.3),"getter":encode_java_script_semantic_value(&wire_field_0.4),"getterPresent":Data::Bool(wire_field_0.5),"setter":encode_java_script_semantic_value(&wire_field_0.6),"setterPresent":Data::Bool(wire_field_0.7),"value":encode_java_script_semantic_value(&wire_field_0.8),"valuePresent":Data::Bool(wire_field_0.9),"writable":Data::Bool(wire_field_0.10),"writablePresent":Data::Bool(wire_field_0.11)})})
        }
        native::JavaScriptDescriptorConversionTransition::DescriptorThrow(wire_field_0) => {
            json!({"$":"descriptorThrow","value":encode_java_script_semantic_value(wire_field_0)})
        }
        native::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"descriptorIntrinsicError","errorClass":Data::String(wire_field_0.to_owned()),"reason":Data::String(wire_field_1.to_owned())})
        }
        native::JavaScriptDescriptorConversionTransition::DescriptorInvalid(wire_field_0) => {
            json!({"$":"descriptorInvalid","reason":Data::String(wire_field_0.to_owned())})
        }
    }
}
