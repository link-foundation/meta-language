use super::{
    Data, decode_java_script_array_assignment_state, decode_java_script_iterator_protocol_state,
    decode_java_script_primitive_conversion_state, decode_java_script_semantic_value,
    decode_java_script_synchronous_iterator_record, encode_java_script_array_assignment_state,
    encode_java_script_iterator_protocol_state, encode_java_script_primitive_conversion_state,
    encode_java_script_semantic_value, encode_java_script_synchronous_iterator_record,
    is_wire_undefined, native, wire_number, wire_text, wire_text_units, wire_undefined,
};

pub(in super::super) fn decode_java_script_array_creation_state(
    data: &Data,
) -> native::JavaScriptArrayCreationState {
    match data["$"].as_str().expect("model constructor") {
        "arrayCreationState" => native::JavaScriptArrayCreationState::ArrayCreationState(
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            Box::new(decode_java_script_semantic_value(&data["arrayPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["iteratorKey"])),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            Box::new(decode_java_script_semantic_value(&data["callback"])),
            Box::new(decode_java_script_semantic_value(&data["thisArgument"])),
            data["mapping"].as_bool().expect("model Boolean"),
            data["iterable"].as_bool().expect("model Boolean"),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            data["index"].as_f64().expect("model Number"),
            data["length"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["method"])),
            Box::new(decode_java_script_semantic_value(&data["arrayLike"])),
            Box::new(decode_java_script_semantic_value(&data["target"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
            Box::new(decode_java_script_semantic_value(&data["lengthInput"])),
            data.get("primitivePrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("record").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_synchronous_iterator_record(wire_value))
                },)
            }),
            data.get("iterator").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_iterator_protocol_state(wire_value))
                },)
            }),
            data.get("conversion").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_primitive_conversion_state(wire_value))
                },)
            }),
            data.get("converted").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("wrapper").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_primitive_wrapper_state(wire_value))
                },)
            }),
            data.get("assignment").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_array_assignment_state(wire_value))
                },)
            }),
            data.get("call").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_array_creation_call(wire_value))
                },)
            }),
            data.get("abrupt").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("errorClass").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(native::ml_string::Text::from_units(wire_text_units(
                        &wire_value,
                    )))
                },)
            }),
            data.get("reason").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(native::ml_string::Text::from_units(wire_text_units(
                        &wire_value,
                    )))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_creation_state(
    value: &native::JavaScriptArrayCreationState,
) -> Data {
    match value {
        native::JavaScriptArrayCreationState::ArrayCreationState(
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
            wire_field_12,
            wire_field_13,
            wire_field_14,
            wire_field_15,
            wire_field_16,
            wire_field_17,
            wire_field_18,
            wire_field_19,
            wire_field_20,
            wire_field_21,
            wire_field_22,
            wire_field_23,
            wire_field_24,
            wire_field_25,
            wire_field_26,
            wire_field_27,
            wire_field_28,
            wire_field_29,
        ) => {
            let mut result = json!({"$":"arrayCreationState","operation":wire_text(&wire_field_0.0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"arrayPrototype":encode_java_script_semantic_value(wire_field_3),"iteratorKey":encode_java_script_semantic_value(wire_field_4),"exoticKey":encode_java_script_semantic_value(wire_field_5),"input":encode_java_script_semantic_value(wire_field_6),"callback":encode_java_script_semantic_value(wire_field_7),"thisArgument":encode_java_script_semantic_value(wire_field_8),"mapping":Data::Bool(*wire_field_9),"iterable":Data::Bool(*wire_field_10),"phase":wire_text(&wire_field_11.0),"index":wire_number(*wire_field_12),"length":wire_number(*wire_field_13),"method":encode_java_script_semantic_value(wire_field_14),"arrayLike":encode_java_script_semantic_value(wire_field_15),"target":encode_java_script_semantic_value(wire_field_16),"value":encode_java_script_semantic_value(wire_field_17),"lengthInput":encode_java_script_semantic_value(wire_field_18)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_19 {
                fields.insert(
                    "primitivePrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_20 {
                fields.insert(
                    "record".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_synchronous_iterator_record(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_21 {
                fields.insert(
                    "iterator".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_iterator_protocol_state(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_22 {
                fields.insert(
                    "conversion".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_primitive_conversion_state(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_23 {
                fields.insert(
                    "converted".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_24 {
                fields.insert(
                    "wrapper".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_primitive_wrapper_state(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_25 {
                fields.insert(
                    "assignment".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_array_assignment_state(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_26 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_array_creation_call(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_27 {
                fields.insert(
                    "abrupt".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_28 {
                fields.insert(
                    "errorClass".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_29 {
                fields.insert(
                    "reason".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_primitive_wrapper_state(
    data: &Data,
) -> native::JavaScriptPrimitiveWrapperState {
    match data["$"].as_str().expect("model constructor") {
        "primitiveWrapperState" => native::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(
            Box::new(decode_java_script_semantic_value(&data["input"])),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_primitive_wrapper_state(
    value: &native::JavaScriptPrimitiveWrapperState,
) -> Data {
    match value {
        native::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"primitiveWrapperState","input":encode_java_script_semantic_value(wire_field_0),"prototype":encode_java_script_semantic_value(wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_array_creation_call(
    data: &Data,
) -> native::JavaScriptArrayCreationCall {
    match data["$"].as_str().expect("model constructor") {
        "arrayCreationCallRecord" => native::JavaScriptArrayCreationCall::ArrayCreationCallRecord(
            Box::new(decode_java_script_semantic_value(&data["callee"])),
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_creation_call(
    value: &native::JavaScriptArrayCreationCall,
) -> Data {
    match value {
        native::JavaScriptArrayCreationCall::ArrayCreationCallRecord(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arrayCreationCallRecord","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_array_creation_fallback(
    data: &Data,
) -> native::JavaScriptArrayCreationFallback {
    match data["$"].as_str().expect("model constructor") {
        "arrayCreationArrayFallback" => {
            native::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(
                Box::new(decode_java_script_semantic_value(&data["prototype"])),
                data["length"].as_f64().expect("model Number"),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_creation_fallback(
    value: &native::JavaScriptArrayCreationFallback,
) -> Data {
    match value {
        native::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"arrayCreationArrayFallback","prototype":encode_java_script_semantic_value(wire_field_0),"length":wire_number(*wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_object_from_entries_error_request(
    data: &Data,
) -> native::JavaScriptObjectFromEntriesErrorRequest {
    match data["$"].as_str().expect("model constructor") {
        "objectFromEntriesErrorRequest" => {
            native::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
                native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
                native::ml_string::Text::from_units(wire_text_units(&data["reason"])),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_object_from_entries_error_request(
    value: &native::JavaScriptObjectFromEntriesErrorRequest,
) -> Data {
    match value {
        native::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"objectFromEntriesErrorRequest","realm":Data::String(wire_field_0.to_string()),"errorClass":wire_text(&wire_field_1.0),"reason":wire_text(&wire_field_2.0)})
        }
    }
}

pub(in super::super) fn decode_java_script_function_bind_state(
    data: &Data,
) -> native::JavaScriptFunctionBindState {
    match data["$"].as_str().expect("model constructor") {
        "functionBindState" => native::JavaScriptFunctionBindState::FunctionBindState(
            Box::new(decode_java_script_semantic_value(&data["target"])),
            Box::new(decode_java_script_semantic_value(&data["boundThis"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            data["length"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["name"])),
            data.get("call").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_function_bind_call(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_function_bind_state(
    value: &native::JavaScriptFunctionBindState,
) -> Data {
    match value {
        native::JavaScriptFunctionBindState::FunctionBindState(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
            wire_field_8,
        ) => {
            let mut result = json!({"$":"functionBindState","target":encode_java_script_semantic_value(wire_field_0),"boundThis":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"phase":wire_text(&wire_field_3.0),"prototype":encode_java_script_semantic_value(wire_field_4),"receiver":encode_java_script_semantic_value(wire_field_5),"length":wire_number(*wire_field_6),"name":encode_java_script_semantic_value(wire_field_7)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_function_bind_call(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_function_bind_call(
    data: &Data,
) -> native::JavaScriptFunctionBindCall {
    match data["$"].as_str().expect("model constructor") {
        "functionBindCall" => native::JavaScriptFunctionBindCall::FunctionBindCall(
            Box::new(decode_java_script_semantic_value(&data["callee"])),
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_function_bind_call(
    value: &native::JavaScriptFunctionBindCall,
) -> Data {
    match value {
        native::JavaScriptFunctionBindCall::FunctionBindCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"functionBindCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_object_from_entries_state(
    data: &Data,
) -> native::JavaScriptObjectFromEntriesState {
    match data["$"].as_str().expect("model constructor") {
        "objectFromEntriesState" => {
            native::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
                Box::new(decode_java_script_semantic_value(&data["input"])),
                Box::new(decode_java_script_semantic_value(&data["prototype"])),
                Box::new(decode_java_script_semantic_value(&data["iteratorKey"])),
                Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
                Box::new(decode_java_script_semantic_value(&data["target"])),
                native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
                Box::new(decode_java_script_semantic_value(&data["entry"])),
                Box::new(decode_java_script_semantic_value(&data["keyInput"])),
                Box::new(decode_java_script_semantic_value(&data["value"])),
                Box::new(decode_java_script_semantic_value(&data["key"])),
                data.get("record").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_synchronous_iterator_record(wire_value))
                    },)
                }),
                data.get("iterator").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_iterator_protocol_state(wire_value))
                    },)
                }),
                data.get("conversion").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_primitive_conversion_state(wire_value))
                    },)
                }),
                data.get("call").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_object_from_entries_call(wire_value))
                    },)
                }),
                data.get("abrupt").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_semantic_value(wire_value))
                    },)
                }),
                data.get("errorClass").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(native::ml_string::Text::from_units(wire_text_units(
                            &wire_value,
                        )))
                    },)
                }),
                data.get("reason").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(native::ml_string::Text::from_units(wire_text_units(
                            &wire_value,
                        )))
                    },)
                }),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_object_from_entries_state(
    value: &native::JavaScriptObjectFromEntriesState,
) -> Data {
    match value {
        native::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
            wire_field_12,
            wire_field_13,
            wire_field_14,
            wire_field_15,
            wire_field_16,
        ) => {
            let mut result = json!({"$":"objectFromEntriesState","input":encode_java_script_semantic_value(wire_field_0),"prototype":encode_java_script_semantic_value(wire_field_1),"iteratorKey":encode_java_script_semantic_value(wire_field_2),"exoticKey":encode_java_script_semantic_value(wire_field_3),"target":encode_java_script_semantic_value(wire_field_4),"phase":wire_text(&wire_field_5.0),"entry":encode_java_script_semantic_value(wire_field_6),"keyInput":encode_java_script_semantic_value(wire_field_7),"value":encode_java_script_semantic_value(wire_field_8),"key":encode_java_script_semantic_value(wire_field_9)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_10 {
                fields.insert(
                    "record".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_synchronous_iterator_record(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_11 {
                fields.insert(
                    "iterator".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_iterator_protocol_state(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_12 {
                fields.insert(
                    "conversion".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_primitive_conversion_state(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_13 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_object_from_entries_call(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_14 {
                fields.insert(
                    "abrupt".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_15 {
                fields.insert(
                    "errorClass".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_16 {
                fields.insert(
                    "reason".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_object_from_entries_call(
    data: &Data,
) -> native::JavaScriptObjectFromEntriesCall {
    match data["$"].as_str().expect("model constructor") {
        "objectFromEntriesCallRecord" => {
            native::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(
                Box::new(decode_java_script_semantic_value(&data["callee"])),
                Box::new(decode_java_script_semantic_value(&data["receiver"])),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["arguments"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_semantic_value)
                        .collect(),
                )),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_object_from_entries_call(
    value: &native::JavaScriptObjectFromEntriesCall,
) -> Data {
    match value {
        native::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"objectFromEntriesCallRecord","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_error_allocation_directory(
    data: &Data,
) -> native::JavaScriptErrorAllocationDirectory {
    match data["$"].as_str().expect("model constructor") {
        "errorAllocationDirectory" => {
            native::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["objects"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_error_allocation_identity)
                        .collect(),
                )),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["symbols"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(|wire_value| {
                            native::ml::Big::parse(
                                wire_value.as_str().expect("model exact integer"),
                            )
                        })
                        .collect(),
                )),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_allocation_directory(
    value: &native::JavaScriptErrorAllocationDirectory,
) -> Data {
    match value {
        native::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"errorAllocationDirectory","objects":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_allocation_identity).collect()) },"symbols":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(|wire_value|Data::String(wire_value.to_string())).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_error_allocation_identity(
    data: &Data,
) -> native::JavaScriptErrorAllocationIdentity {
    match data["$"].as_str().expect("model constructor") {
        "errorAllocationIdentity" => {
            native::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(
                native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
                native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
                data["callable"].as_bool().expect("model Boolean"),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_allocation_identity(
    value: &native::JavaScriptErrorAllocationIdentity,
) -> Data {
    match value {
        native::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorAllocationIdentity","identity":Data::String(wire_field_0.to_string()),"kind":wire_text(&wire_field_1.0),"callable":Data::Bool(*wire_field_2)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_iterator_region(
    data: &Data,
) -> native::JavaScriptCallIteratorRegion {
    match data["$"].as_str().expect("model constructor") {
        "callIteratorRegion" => native::JavaScriptCallIteratorRegion::CallIteratorRegion(
            data["slot"].as_f64().expect("model Number"),
            Box::new(decode_java_script_synchronous_iterator_record(
                &data["record"],
            )),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_iterator_region(
    value: &native::JavaScriptCallIteratorRegion,
) -> Data {
    match value {
        native::JavaScriptCallIteratorRegion::CallIteratorRegion(wire_field_0, wire_field_1) => {
            json!({"$":"callIteratorRegion","slot":wire_number(*wire_field_0),"record":encode_java_script_synchronous_iterator_record(wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_argument_list(
    data: &Data,
) -> native::JavaScriptCallArgumentList {
    match data["$"].as_str().expect("model constructor") {
        "callArgumentList" => native::JavaScriptCallArgumentList::CallArgumentList(
            data["slot"].as_f64().expect("model Number"),
            data["depth"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["values"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_argument_list(
    value: &native::JavaScriptCallArgumentList,
) -> Data {
    match value {
        native::JavaScriptCallArgumentList::CallArgumentList(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"callArgumentList","slot":wire_number(*wire_field_0),"depth":wire_number(*wire_field_1),"values":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}
