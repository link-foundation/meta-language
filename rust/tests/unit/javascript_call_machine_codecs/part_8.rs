use super::{
    Data, decode_java_script_array_assignment_state, decode_java_script_ordinary_property,
    decode_java_script_primitive_conversion_state, decode_java_script_semantic_value,
    decode_wire_record_order, encode_java_script_array_assignment_state,
    encode_java_script_ordinary_property, encode_java_script_primitive_conversion_state,
    encode_java_script_semantic_value, encode_wire_ordered_record, is_wire_undefined, native,
    wire_number, wire_text, wire_text_units, wire_undefined,
};

pub(in super::super) fn encode_java_script_property_definitions_state(
    value: &native::JavaScriptPropertyDefinitionsState,
) -> Data {
    match value {
        native::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
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
        ) => {
            let mut result = json!({"$":"propertyDefinitions","operation":wire_text(&wire_field_0.0),"target":encode_java_script_semantic_value(wire_field_1),"input":encode_java_script_semantic_value(wire_field_2),"prototype":encode_java_script_semantic_value(wire_field_3),"exoticKey":encode_java_script_semantic_value(wire_field_4),"phase":wire_text(&wire_field_5.0),"keys":{ let wire_reference = wire_field_6.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"position":wire_number(*wire_field_7),"descriptors":{ let wire_reference = wire_field_8.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"definitionIndex":wire_number(*wire_field_9)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_10 {
                fields.insert(
                    "conversion".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_descriptor_conversion_state(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_11 {
                fields.insert(
                    "definition".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_property_definition_state(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_12 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_property_definitions_call(wire_present)
                        }
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_descriptor_conversion_state(
    data: &Data,
) -> native::JavaScriptDescriptorConversionState {
    match data["$"].as_str().expect("model constructor") {
        "descriptorConversion" => {
            native::JavaScriptDescriptorConversionState::DescriptorConversion(
                Box::new(decode_java_script_semantic_value(&data["receiver"])),
                data["next"].as_f64().expect("model Number"),
                std::rc::Rc::new(std::cell::RefCell::new((
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
                    decode_wire_record_order(
                        &data["descriptor"],
                        &[
                            "configurable",
                            "configurablePresent",
                            "enumerable",
                            "enumerablePresent",
                            "getter",
                            "getterPresent",
                            "setter",
                            "setterPresent",
                            "value",
                            "valuePresent",
                            "writable",
                            "writablePresent",
                        ],
                    ),
                ))),
                data["pending"].as_bool().expect("model Boolean"),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_descriptor_conversion_state(
    value: &native::JavaScriptDescriptorConversionState,
) -> Data {
    match value {
        native::JavaScriptDescriptorConversionState::DescriptorConversion(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"descriptorConversion","receiver":encode_java_script_semantic_value(wire_field_0),"next":wire_number(*wire_field_1),"descriptor":{ let wire_reference = wire_field_2.borrow(); encode_wire_ordered_record([("configurable", Some(Data::Bool(wire_reference.0.0))),("configurablePresent", Some(Data::Bool(wire_reference.0.1))),("enumerable", Some(Data::Bool(wire_reference.0.2))),("enumerablePresent", Some(Data::Bool(wire_reference.0.3))),("getter", Some(encode_java_script_semantic_value(&wire_reference.0.4))),("getterPresent", Some(Data::Bool(wire_reference.0.5))),("setter", Some(encode_java_script_semantic_value(&wire_reference.0.6))),("setterPresent", Some(Data::Bool(wire_reference.0.7))),("value", Some(encode_java_script_semantic_value(&wire_reference.0.8))),("valuePresent", Some(Data::Bool(wire_reference.0.9))),("writable", Some(Data::Bool(wire_reference.0.10))),("writablePresent", Some(Data::Bool(wire_reference.0.11)))], &wire_reference.1) },"pending":Data::Bool(*wire_field_3)})
        }
    }
}

pub(in super::super) fn decode_java_script_property_definition_state(
    data: &Data,
) -> native::JavaScriptPropertyDefinitionState {
    match data["$"].as_str().expect("model constructor") {
        "propertyDefinitionKey" => {
            native::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                Box::new(decode_java_script_semantic_value(&data["target"])),
                Box::new(decode_java_script_semantic_value(&data["attributes"])),
                Box::new(decode_java_script_primitive_conversion_state(
                    &data["conversion"],
                )),
                data.get("operation").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(native::ml_string::Text::from_units(wire_text_units(
                            &wire_value,
                        )))
                    },)
                }),
                data.get("call").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_property_definition_call(wire_value))
                    },)
                }),
            )
        }
        "propertyDefinitionDescriptor" => {
            native::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                Box::new(decode_java_script_semantic_value(&data["target"])),
                Box::new(decode_java_script_semantic_value(&data["key"])),
                Box::new(decode_java_script_descriptor_conversion_state(
                    &data["conversion"],
                )),
                data.get("exoticKey").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_semantic_value(wire_value))
                    },)
                }),
                data.get("operation").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(native::ml_string::Text::from_units(wire_text_units(
                            &wire_value,
                        )))
                    },)
                }),
                data.get("call").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_property_definition_call(wire_value))
                    },)
                }),
            )
        }
        "propertyDefinitionArray" => {
            native::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                Box::new(decode_java_script_semantic_value(&data["target"])),
                Box::new(decode_java_script_semantic_value(&data["key"])),
                Box::new(decode_java_script_array_assignment_state(
                    &data["assignment"],
                )),
                data.get("operation").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(native::ml_string::Text::from_units(wire_text_units(
                            &wire_value,
                        )))
                    },)
                }),
                data.get("call").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_property_definition_call(wire_value))
                    },)
                }),
            )
        }
        "propertyDefinitionReady" => {
            native::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                Box::new(decode_java_script_semantic_value(&data["target"])),
                Box::new(decode_java_script_semantic_value(&data["key"])),
                std::rc::Rc::new(std::cell::RefCell::new((
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
                    decode_wire_record_order(
                        &data["descriptor"],
                        &[
                            "configurable",
                            "configurablePresent",
                            "enumerable",
                            "enumerablePresent",
                            "getter",
                            "getterPresent",
                            "setter",
                            "setterPresent",
                            "value",
                            "valuePresent",
                            "writable",
                            "writablePresent",
                        ],
                    ),
                ))),
                data.get("exoticKey").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_semantic_value(wire_value))
                    },)
                }),
                data.get("operation").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(native::ml_string::Text::from_units(wire_text_units(
                            &wire_value,
                        )))
                    },)
                }),
                data.get("call").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_property_definition_call(wire_value))
                    },)
                }),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_property_definition_state(
    value: &native::JavaScriptPropertyDefinitionState,
) -> Data {
    match value {
        native::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            let mut result = json!({"$":"propertyDefinitionKey","target":encode_java_script_semantic_value(wire_field_0),"attributes":encode_java_script_semantic_value(wire_field_1),"conversion":encode_java_script_primitive_conversion_state(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "operation".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_property_definition_call(wire_present)
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"propertyDefinitionDescriptor","target":encode_java_script_semantic_value(wire_field_0),"key":encode_java_script_semantic_value(wire_field_1),"conversion":encode_java_script_descriptor_conversion_state(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "exoticKey".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "operation".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_property_definition_call(wire_present)
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            let mut result = json!({"$":"propertyDefinitionArray","target":encode_java_script_semantic_value(wire_field_0),"key":encode_java_script_semantic_value(wire_field_1),"assignment":encode_java_script_array_assignment_state(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "operation".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_property_definition_call(wire_present)
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"propertyDefinitionReady","target":encode_java_script_semantic_value(wire_field_0),"key":encode_java_script_semantic_value(wire_field_1),"descriptor":{ let wire_reference = wire_field_2.borrow(); encode_wire_ordered_record([("configurable", Some(Data::Bool(wire_reference.0.0))),("configurablePresent", Some(Data::Bool(wire_reference.0.1))),("enumerable", Some(Data::Bool(wire_reference.0.2))),("enumerablePresent", Some(Data::Bool(wire_reference.0.3))),("getter", Some(encode_java_script_semantic_value(&wire_reference.0.4))),("getterPresent", Some(Data::Bool(wire_reference.0.5))),("setter", Some(encode_java_script_semantic_value(&wire_reference.0.6))),("setterPresent", Some(Data::Bool(wire_reference.0.7))),("value", Some(encode_java_script_semantic_value(&wire_reference.0.8))),("valuePresent", Some(Data::Bool(wire_reference.0.9))),("writable", Some(Data::Bool(wire_reference.0.10))),("writablePresent", Some(Data::Bool(wire_reference.0.11)))], &wire_reference.1) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "exoticKey".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "operation".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_property_definition_call(wire_present)
                        }
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_property_definition_call(
    data: &Data,
) -> native::JavaScriptPropertyDefinitionCall {
    match data["$"].as_str().expect("model constructor") {
        "propertyDefinitionCallRecord" => {
            native::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(
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

pub(in super::super) fn encode_java_script_property_definition_call(
    value: &native::JavaScriptPropertyDefinitionCall,
) -> Data {
    match value {
        native::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"propertyDefinitionCallRecord","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_property_definitions_call(
    data: &Data,
) -> native::JavaScriptPropertyDefinitionsCall {
    match data["$"].as_str().expect("model constructor") {
        "propertyDefinitionsCallRecord" => {
            native::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(
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

pub(in super::super) fn encode_java_script_property_definitions_call(
    value: &native::JavaScriptPropertyDefinitionsCall,
) -> Data {
    match value {
        native::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"propertyDefinitionsCallRecord","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_property_reflection_state(
    data: &Data,
) -> native::JavaScriptPropertyReflectionState {
    match data["$"].as_str().expect("model constructor") {
        "propertyReflection" => native::JavaScriptPropertyReflectionState::PropertyReflection(
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            Box::new(decode_java_script_semantic_value(&data["key"])),
            Box::new(decode_java_script_semantic_value(&data["objectPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["arrayPrototype"])),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["keys"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            data["position"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["fields"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            data["field"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["target"])),
            Box::new(decode_java_script_semantic_value(&data["descriptor"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_property_reflection_state(
    value: &native::JavaScriptPropertyReflectionState,
) -> Data {
    match value {
        native::JavaScriptPropertyReflectionState::PropertyReflection(
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
            json!({"$":"propertyReflection","operation":wire_text(&wire_field_0.0),"input":encode_java_script_semantic_value(wire_field_1),"key":encode_java_script_semantic_value(wire_field_2),"objectPrototype":encode_java_script_semantic_value(wire_field_3),"arrayPrototype":encode_java_script_semantic_value(wire_field_4),"phase":wire_text(&wire_field_5.0),"keys":{ let wire_reference = wire_field_6.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"position":wire_number(*wire_field_7),"fields":{ let wire_reference = wire_field_8.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"field":wire_number(*wire_field_9),"target":encode_java_script_semantic_value(wire_field_10),"descriptor":encode_java_script_semantic_value(wire_field_11)})
        }
    }
}

pub(in super::super) fn decode_java_script_instance_of_state(
    data: &Data,
) -> native::JavaScriptInstanceOfState {
    match data["$"].as_str().expect("model constructor") {
        "instanceOfState" => native::JavaScriptInstanceOfState::InstanceOfState(
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            Box::new(decode_java_script_semantic_value(&data["left"])),
            Box::new(decode_java_script_semantic_value(&data["right"])),
            Box::new(decode_java_script_semantic_value(&data["key"])),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            Box::new(decode_java_script_semantic_value(&data["cursor"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["visited"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| {
                        native::ml::Big::parse(wire_value.as_str().expect("model exact integer"))
                    })
                    .collect(),
            )),
            data.get("call").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_instance_of_call(wire_value))
                },)
            }),
            data.get("candidate").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("boundPath").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(|wire_value| {
                                native::ml::Big::parse(
                                    wire_value.as_str().expect("model exact integer"),
                                )
                            })
                            .collect(),
                    )))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_instance_of_state(
    value: &native::JavaScriptInstanceOfState,
) -> Data {
    match value {
        native::JavaScriptInstanceOfState::InstanceOfState(
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
        ) => {
            let mut result = json!({"$":"instanceOfState","operation":wire_text(&wire_field_0.0),"left":encode_java_script_semantic_value(wire_field_1),"right":encode_java_script_semantic_value(wire_field_2),"key":encode_java_script_semantic_value(wire_field_3),"phase":wire_text(&wire_field_4.0),"prototype":encode_java_script_semantic_value(wire_field_5),"cursor":encode_java_script_semantic_value(wire_field_6),"visited":{ let wire_reference = wire_field_7.borrow(); Data::Array(wire_reference.iter().map(|wire_value|Data::String(wire_value.to_string())).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_instance_of_call(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_9 {
                fields.insert(
                    "candidate".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_10 {
                fields.insert(
                    "boundPath".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(|wire_value| Data::String(wire_value.to_string()))
                                    .collect(),
                            )
                        }
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_instance_of_call(
    data: &Data,
) -> native::JavaScriptInstanceOfCall {
    match data["$"].as_str().expect("model constructor") {
        "instanceOfCall" => native::JavaScriptInstanceOfCall::InstanceOfCall(
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

pub(in super::super) fn encode_java_script_instance_of_call(
    value: &native::JavaScriptInstanceOfCall,
) -> Data {
    match value {
        native::JavaScriptInstanceOfCall::InstanceOfCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"instanceOfCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_object_integrity_state(
    data: &Data,
) -> native::JavaScriptObjectIntegrityState {
    match data["$"].as_str().expect("model constructor") {
        "objectIntegrity" => native::JavaScriptObjectIntegrityState::ObjectIntegrity(
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["keys"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            data["position"].as_f64().expect("model Number"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_object_integrity_state(
    value: &native::JavaScriptObjectIntegrityState,
) -> Data {
    match value {
        native::JavaScriptObjectIntegrityState::ObjectIntegrity(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"objectIntegrity","operation":wire_text(&wire_field_0.0),"input":encode_java_script_semantic_value(wire_field_1),"phase":wire_text(&wire_field_2.0),"keys":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"position":wire_number(*wire_field_4)})
        }
    }
}

pub(in super::super) fn decode_java_script_number_call_state(
    data: &Data,
) -> native::JavaScriptNumberCallState {
    match data["$"].as_str().expect("model constructor") {
        "numberProtocolState" => native::JavaScriptNumberCallState::NumberProtocolState(Box::new(
            decode_java_script_primitive_conversion_state(&data["state"]),
        )),
        "numberPrimitiveValue" => native::JavaScriptNumberCallState::NumberPrimitiveValue(
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_number_call_state(
    value: &native::JavaScriptNumberCallState,
) -> Data {
    match value {
        native::JavaScriptNumberCallState::NumberProtocolState(wire_field_0) => {
            json!({"$":"numberProtocolState","state":encode_java_script_primitive_conversion_state(wire_field_0)})
        }
        native::JavaScriptNumberCallState::NumberPrimitiveValue(wire_field_0) => {
            json!({"$":"numberPrimitiveValue","value":encode_java_script_semantic_value(wire_field_0)})
        }
    }
}

pub(in super::super) fn decode_java_script_text_call_state(
    data: &Data,
) -> native::JavaScriptTextCallState {
    match data["$"].as_str().expect("model constructor") {
        "textProtocolState" => native::JavaScriptTextCallState::TextProtocolState(Box::new(
            decode_java_script_primitive_conversion_state(&data["state"]),
        )),
        "textPrimitiveValue" => native::JavaScriptTextCallState::TextPrimitiveValue(Box::new(
            decode_java_script_semantic_value(&data["value"]),
        )),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_text_call_state(
    value: &native::JavaScriptTextCallState,
) -> Data {
    match value {
        native::JavaScriptTextCallState::TextProtocolState(wire_field_0) => {
            json!({"$":"textProtocolState","state":encode_java_script_primitive_conversion_state(wire_field_0)})
        }
        native::JavaScriptTextCallState::TextPrimitiveValue(wire_field_0) => {
            json!({"$":"textPrimitiveValue","value":encode_java_script_semantic_value(wire_field_0)})
        }
    }
}
