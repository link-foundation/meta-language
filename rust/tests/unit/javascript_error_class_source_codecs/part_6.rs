use super::{
    Data, decode_java_script_call_instruction, decode_java_script_semantic_value,
    encode_java_script_call_instruction, encode_java_script_semantic_value, is_wire_undefined,
    native, wire_number, wire_text, wire_text_units, wire_undefined,
};

pub(in super::super) fn decode_java_script_call_source_span(
    data: &Data,
) -> native::JavaScriptCallSourceSpan {
    match data["$"].as_str().expect("model constructor") {
        "callSourceSpan" => native::JavaScriptCallSourceSpan::CallSourceSpan(
            native::ml_string::Text::from_units(wire_text_units(&data["unit"])),
            data["start"].as_f64().expect("model Number"),
            data["end"].as_f64().expect("model Number"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_source_span(
    value: &native::JavaScriptCallSourceSpan,
) -> Data {
    match value {
        native::JavaScriptCallSourceSpan::CallSourceSpan(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"callSourceSpan","unit":wire_text(&wire_field_0.0),"start":wire_number(*wire_field_1),"end":wire_number(*wire_field_2)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_template(
    data: &Data,
) -> native::JavaScriptCallTemplate {
    match data["$"].as_str().expect("model constructor") {
        "callTemplate" => native::JavaScriptCallTemplate::CallTemplate(
            native::ml::Big::parse(data["owner"].as_str().expect("model exact integer")),
            data["ordinal"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            data["parameters"].as_f64().expect("model Number"),
            data["registers"].as_f64().expect("model Number"),
            data["captureCount"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["instructions"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_instruction)
                    .collect(),
            )),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            Box::new(decode_java_script_semantic_value(&data["name"])),
            data["length"].as_f64().expect("model Number"),
            data.get("receiverMode").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(native::ml_string::Text::from_units(wire_text_units(
                        &wire_value,
                    )))
                },)
            }),
            data.get("source").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_call_source_span(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_template(
    value: &native::JavaScriptCallTemplate,
) -> Data {
    match value {
        native::JavaScriptCallTemplate::CallTemplate(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
            wire_field_8,
            wire_field_9,
            wire_field_10,
            wire_field_11,
        ) => {
            let mut result = json!({"$":"callTemplate","owner":Data::String(wire_field_0.to_string()),"ordinal":wire_number(*wire_field_1),"realm":Data::String(wire_field_2.to_string()),"parameters":wire_number(*wire_field_3),"registers":wire_number(*wire_field_4),"captureCount":wire_number(*wire_field_5),"instructions":{ let wire_reference = wire_field_6.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_instruction).collect()) },"prototype":encode_java_script_semantic_value(wire_field_7),"name":encode_java_script_semantic_value(wire_field_8),"length":wire_number(*wire_field_9)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_10 {
                fields.insert(
                    "receiverMode".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_11 {
                fields.insert(
                    "source".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_call_source_span(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_call_source_unit(
    data: &Data,
) -> native::JavaScriptCallSourceUnit {
    match data["$"].as_str().expect("model constructor") {
        "callSourceUnit" => native::JavaScriptCallSourceUnit::CallSourceUnit(
            native::ml_string::Text::from_units(wire_text_units(&data["identifier"])),
            Box::new(decode_java_script_semantic_value(&data["text"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_source_unit(
    value: &native::JavaScriptCallSourceUnit,
) -> Data {
    match value {
        native::JavaScriptCallSourceUnit::CallSourceUnit(wire_field_0, wire_field_1) => {
            json!({"$":"callSourceUnit","identifier":wire_text(&wire_field_0.0),"text":encode_java_script_semantic_value(wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_error_module_binding(
    data: &Data,
) -> native::JavaScriptErrorModuleBinding {
    match data["$"].as_str().expect("model constructor") {
        "errorModuleBindingValue" => native::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_module_binding(
    value: &native::JavaScriptErrorModuleBinding,
) -> Data {
    match value {
        native::JavaScriptErrorModuleBinding::ErrorModuleBindingValue(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"errorModuleBindingValue","identity":wire_number(*wire_field_0),"value":encode_java_script_semantic_value(wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_source_capability_owner(
    data: &Data,
) -> native::JavaScriptSourceCapabilityOwner {
    match data["$"].as_str().expect("model constructor") {
        "sourceCapabilityOwner" => native::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(
            native::ml_string::Text::from_units(wire_text_units(&data["source"])),
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            native::ml_string::Text::from_units(wire_text_units(&data["owner"])),
            data["callable"].as_bool().expect("model Boolean"),
            data["executable"].as_bool().expect("model Boolean"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_source_capability_owner(
    value: &native::JavaScriptSourceCapabilityOwner,
) -> Data {
    match value {
        native::JavaScriptSourceCapabilityOwner::SourceCapabilityOwner(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
        ) => {
            json!({"$":"sourceCapabilityOwner","source":wire_text(&wire_field_0.0),"identity":Data::String(wire_field_1.to_string()),"realm":Data::String(wire_field_2.to_string()),"operation":wire_text(&wire_field_3.0),"owner":wire_text(&wire_field_4.0),"callable":Data::Bool(*wire_field_5),"executable":Data::Bool(*wire_field_6)})
        }
    }
}
