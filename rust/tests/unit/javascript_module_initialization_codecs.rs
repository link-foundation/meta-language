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
fn wire_text_units(value: &Data) -> Vec<u16> {
    let units = if let Some(text) = value.as_str() {
        text.encode_utf16().collect::<Vec<u16>>()
    } else {
        let object = value.as_object().expect("qualified model text units");
        assert_eq!(object.len(), 2, "exact model text keys");
        assert_eq!(
            object.get("textEncoding").and_then(Data::as_str),
            Some("UTF16 code units 1"),
            "model text encoding"
        );
        let input = object
            .get("units")
            .and_then(Data::as_array)
            .expect("model text units");
        assert!(input.len() <= 16777216, "model text unit budget");
        input
            .iter()
            .map(|unit| {
                u16::try_from(unit.as_u64().expect("model UTF16 unit"))
                    .expect("model UTF16 unit range")
            })
            .collect()
    };
    assert!(units.len() <= 16777216, "model text unit budget");
    units
}
fn wire_text(units: &[u16]) -> Data {
    assert!(units.len() <= 16777216, "model text unit budget");
    match String::from_utf16(units) {
        Ok(text) => Data::String(text),
        Err(_) => json!({"textEncoding":"UTF16 code units 1","units":units}),
    }
}
pub(super) fn decode_java_script_module_initialization_plan(
    data: &Data,
) -> native::JavaScriptModuleInitializationPlan {
    match data["$"].as_str().expect("model constructor") {
        "moduleInitializationPlan" => {
            native::JavaScriptModuleInitializationPlan::ModuleInitializationPlan(
                native::ml_string::Text::from_units(wire_text_units(&data["profile"])),
                native::ml_string::Text::from_units(wire_text_units(&data["moduleIdentity"])),
                native::ml_string::Text::from_units(wire_text_units(&data["sourceSha256"])),
                data["bindings"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
                data["names"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| {
                        native::ml_string::Text::from_units(wire_text_units(&wire_value))
                    })
                    .collect(),
                data["initializers"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_module_initializer)
                    .collect(),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_module_initialization_plan(
    value: &native::JavaScriptModuleInitializationPlan,
) -> Data {
    match value {
        native::JavaScriptModuleInitializationPlan::ModuleInitializationPlan(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"moduleInitializationPlan","profile":wire_text(&wire_field_0.0),"moduleIdentity":wire_text(&wire_field_1.0),"sourceSha256":wire_text(&wire_field_2.0),"bindings":Data::Array(wire_field_3.iter().map(|wire_value|wire_number(*wire_value)).collect()),"names":Data::Array(wire_field_4.iter().map(|wire_value|wire_text(&wire_value.0)).collect()),"initializers":Data::Array(wire_field_5.iter().map(encode_java_script_module_initializer).collect())})
        }
    }
}
pub(super) fn decode_java_script_module_initializer(
    data: &Data,
) -> native::JavaScriptModuleInitializer {
    match data["$"].as_str().expect("model constructor") {
        "moduleDataLoad" => native::JavaScriptModuleInitializer::ModuleDataLoad(
            data["binding"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["specifier"])),
            native::ml_string::Text::from_units(wire_text_units(&data["readerOwner"])),
        ),
        "moduleConstAlias" => native::JavaScriptModuleInitializer::ModuleConstAlias(
            data["binding"].as_f64().expect("model Number"),
            data["target"].as_f64().expect("model Number"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_module_initializer(
    value: &native::JavaScriptModuleInitializer,
) -> Data {
    match value {
        native::JavaScriptModuleInitializer::ModuleDataLoad(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"moduleDataLoad","binding":wire_number(*wire_field_0),"specifier":wire_text(&wire_field_1.0),"readerOwner":wire_text(&wire_field_2.0)})
        }
        native::JavaScriptModuleInitializer::ModuleConstAlias(wire_field_0, wire_field_1) => {
            json!({"$":"moduleConstAlias","binding":wire_number(*wire_field_0),"target":wire_number(*wire_field_1)})
        }
    }
}
pub(super) fn decode_java_script_module_initialization_state(
    data: &Data,
) -> native::JavaScriptModuleInitializationState {
    match data["$"].as_str().expect("model constructor") {
        "moduleInitializationState" => {
            native::JavaScriptModuleInitializationState::ModuleInitializationState(
                native::ml_string::Text::from_units(wire_text_units(&data["moduleIdentity"])),
                native::ml_string::Text::from_units(wire_text_units(&data["sourceSha256"])),
                native::ml_string::Text::from_units(wire_text_units(&data["moduleURL"])),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
                native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
                data["position"].as_f64().expect("model Number"),
                data["slots"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_module_binding_slot)
                    .collect(),
                Box::new(decode_java_script_semantic_value(&data["failure"])),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_module_initialization_state(
    value: &native::JavaScriptModuleInitializationState,
) -> Data {
    match value {
        native::JavaScriptModuleInitializationState::ModuleInitializationState(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
        ) => {
            json!({"$":"moduleInitializationState","moduleIdentity":wire_text(&wire_field_0.0),"sourceSha256":wire_text(&wire_field_1.0),"moduleURL":wire_text(&wire_field_2.0),"realm":Data::String(wire_field_3.to_string()),"phase":wire_text(&wire_field_4.0),"position":wire_number(*wire_field_5),"slots":Data::Array(wire_field_6.iter().map(encode_java_script_module_binding_slot).collect()),"failure":encode_java_script_semantic_value(wire_field_7)})
        }
    }
}
pub(super) fn decode_java_script_module_binding_slot(
    data: &Data,
) -> native::JavaScriptModuleBindingSlot {
    match data["$"].as_str().expect("model constructor") {
        "moduleBindingSlot" => native::JavaScriptModuleBindingSlot::ModuleBindingSlot(
            data["binding"].as_f64().expect("model Number"),
            data["initialized"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_module_binding_slot(
    value: &native::JavaScriptModuleBindingSlot,
) -> Data {
    match value {
        native::JavaScriptModuleBindingSlot::ModuleBindingSlot(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"moduleBindingSlot","binding":wire_number(*wire_field_0),"initialized":Data::Bool(*wire_field_1),"value":encode_java_script_semantic_value(wire_field_2)})
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
        "sourceText" => native::JavaScriptSemanticValue::SourceText(
            native::ml_string::Text::from_units(wire_text_units(&data["profile"])),
            native::ml_string::Text::from_units(wire_text_units(&data["text"])),
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
        native::JavaScriptSemanticValue::SourceText(wire_field_0, wire_field_1) => {
            json!({"$":"sourceText","profile":wire_text(&wire_field_0.0),"text":wire_text(&wire_field_1.0)})
        }
        native::JavaScriptSemanticValue::Symbol(wire_field_0) => {
            json!({"$":"symbol","identity":Data::String(wire_field_0.to_string())})
        }
        native::JavaScriptSemanticValue::Object(wire_field_0, wire_field_1) => {
            json!({"$":"object","identity":Data::String(wire_field_0.to_string()),"callable":Data::Bool(*wire_field_1)})
        }
    }
}
pub(super) fn decode_java_script_module_initialization_transition(
    data: &Data,
) -> native::JavaScriptModuleInitializationTransition {
    match data["$"].as_str().expect("model constructor") {
        "moduleContinue" => {
            native::JavaScriptModuleInitializationTransition::ModuleContinue(Box::new(
                decode_java_script_module_initialization_state(&data["state"]),
            ))
        }
        "moduleReadData" => native::JavaScriptModuleInitializationTransition::ModuleReadData(
            Box::new(decode_java_script_module_initialization_state(
                &data["state"],
            )),
            data["binding"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["specifier"])),
            native::ml_string::Text::from_units(wire_text_units(&data["readerOwner"])),
        ),
        "moduleTemporalDeadZone" => {
            native::JavaScriptModuleInitializationTransition::ModuleTemporalDeadZone(
                Box::new(decode_java_script_module_initialization_state(
                    &data["state"],
                )),
                data["binding"].as_f64().expect("model Number"),
                native::ml_string::Text::from_units(wire_text_units(&data["name"])),
            )
        }
        "moduleEvaluated" => {
            native::JavaScriptModuleInitializationTransition::ModuleEvaluated(Box::new(
                decode_java_script_module_initialization_state(&data["state"]),
            ))
        }
        "moduleAbrupt" => native::JavaScriptModuleInitializationTransition::ModuleAbrupt(
            Box::new(decode_java_script_module_initialization_state(
                &data["state"],
            )),
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        "moduleWaiting" => {
            native::JavaScriptModuleInitializationTransition::ModuleWaiting(Box::new(
                decode_java_script_module_initialization_state(&data["state"]),
            ))
        }
        "moduleInvalid" => native::JavaScriptModuleInitializationTransition::ModuleInvalid(
            Box::new(decode_java_script_module_initialization_state(
                &data["state"],
            )),
            native::ml_string::Text::from_units(wire_text_units(&data["reason"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_module_initialization_transition(
    value: &native::JavaScriptModuleInitializationTransition,
) -> Data {
    match value {
        native::JavaScriptModuleInitializationTransition::ModuleContinue(wire_field_0) => {
            json!({"$":"moduleContinue","state":encode_java_script_module_initialization_state(wire_field_0)})
        }
        native::JavaScriptModuleInitializationTransition::ModuleReadData(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"moduleReadData","state":encode_java_script_module_initialization_state(wire_field_0),"binding":wire_number(*wire_field_1),"specifier":wire_text(&wire_field_2.0),"readerOwner":wire_text(&wire_field_3.0)})
        }
        native::JavaScriptModuleInitializationTransition::ModuleTemporalDeadZone(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"moduleTemporalDeadZone","state":encode_java_script_module_initialization_state(wire_field_0),"binding":wire_number(*wire_field_1),"name":wire_text(&wire_field_2.0)})
        }
        native::JavaScriptModuleInitializationTransition::ModuleEvaluated(wire_field_0) => {
            json!({"$":"moduleEvaluated","state":encode_java_script_module_initialization_state(wire_field_0)})
        }
        native::JavaScriptModuleInitializationTransition::ModuleAbrupt(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"moduleAbrupt","state":encode_java_script_module_initialization_state(wire_field_0),"value":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptModuleInitializationTransition::ModuleWaiting(wire_field_0) => {
            json!({"$":"moduleWaiting","state":encode_java_script_module_initialization_state(wire_field_0)})
        }
        native::JavaScriptModuleInitializationTransition::ModuleInvalid(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"moduleInvalid","state":encode_java_script_module_initialization_state(wire_field_0),"reason":wire_text(&wire_field_1.0)})
        }
    }
}
pub(super) fn decode_java_script_module_binding_read(
    data: &Data,
) -> native::JavaScriptModuleBindingRead {
    match data["$"].as_str().expect("model constructor") {
        "moduleBindingValue" => native::JavaScriptModuleBindingRead::ModuleBindingValue(Box::new(
            decode_java_script_semantic_value(&data["value"]),
        )),
        "moduleBindingUninitialized" => {
            native::JavaScriptModuleBindingRead::ModuleBindingUninitialized(
                data["binding"].as_f64().expect("model Number"),
                native::ml_string::Text::from_units(wire_text_units(&data["name"])),
            )
        }
        "moduleBindingFailure" => native::JavaScriptModuleBindingRead::ModuleBindingFailure(
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        "moduleBindingInvalid" => native::JavaScriptModuleBindingRead::ModuleBindingInvalid(
            native::ml_string::Text::from_units(wire_text_units(&data["reason"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_module_binding_read(
    value: &native::JavaScriptModuleBindingRead,
) -> Data {
    match value {
        native::JavaScriptModuleBindingRead::ModuleBindingValue(wire_field_0) => {
            json!({"$":"moduleBindingValue","value":encode_java_script_semantic_value(wire_field_0)})
        }
        native::JavaScriptModuleBindingRead::ModuleBindingUninitialized(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"moduleBindingUninitialized","binding":wire_number(*wire_field_0),"name":wire_text(&wire_field_1.0)})
        }
        native::JavaScriptModuleBindingRead::ModuleBindingFailure(wire_field_0) => {
            json!({"$":"moduleBindingFailure","value":encode_java_script_semantic_value(wire_field_0)})
        }
        native::JavaScriptModuleBindingRead::ModuleBindingInvalid(wire_field_0) => {
            json!({"$":"moduleBindingInvalid","reason":wire_text(&wire_field_0.0)})
        }
    }
}
