use super::{
    Data, decode_java_script_array_constructor_fact, decode_java_script_primitive_conversion_state,
    decode_java_script_semantic_value, decode_wire_record_order,
    encode_java_script_array_constructor_fact, encode_java_script_primitive_conversion_state,
    encode_java_script_semantic_value, encode_wire_ordered_record, is_wire_undefined, native,
    wire_number, wire_text, wire_text_units, wire_undefined,
};

pub(in super::super) fn decode_java_script_array_concatenation_state(
    data: &Data,
) -> native::JavaScriptArrayConcatenationState {
    match data["$"].as_str().expect("model constructor") {
        "arrayConcatenationState" => {
            native::JavaScriptArrayConcatenationState::ArrayConcatenationState(
                Box::new(decode_java_script_semantic_value(&data["receiver"])),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["arguments"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_semantic_value)
                        .collect(),
                )),
                Box::new(decode_java_script_semantic_value(&data["spreadableKey"])),
                Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
                data["itemPosition"].as_f64().expect("model Number"),
                Box::new(decode_java_script_semantic_value(&data["item"])),
                data["itemStart"].as_f64().expect("model Number"),
                data["length"].as_f64().expect("model Number"),
                data["index"].as_f64().expect("model Number"),
                data["outputIndex"].as_f64().expect("model Number"),
                data["spread"].as_bool().expect("model Boolean"),
                Box::new(decode_java_script_semantic_value(&data["target"])),
                Box::new(decode_java_script_semantic_value(&data["value"])),
                Box::new(decode_java_script_semantic_value(&data["input"])),
                native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
                data.get("conversion").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_primitive_conversion_state(wire_value))
                    },)
                }),
                data.get("species").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_array_species_state(wire_value))
                    },)
                }),
                data.get("allocationPrototype").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_semantic_value(wire_value))
                    },)
                }),
                data.get("constructor").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_semantic_value(wire_value))
                    },)
                }),
                data.get("constructorFact").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_array_constructor_fact(wire_value))
                    },)
                }),
                data.get("assignment").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_object_spread_state(wire_value))
                    },)
                }),
                data.get("call").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_array_concatenation_call(wire_value))
                    },)
                }),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_concatenation_state(
    value: &native::JavaScriptArrayConcatenationState,
) -> Data {
    match value {
        native::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
        ) => {
            let mut result = json!({"$":"arrayConcatenationState","receiver":encode_java_script_semantic_value(wire_field_0),"arguments":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"spreadableKey":encode_java_script_semantic_value(wire_field_2),"exoticKey":encode_java_script_semantic_value(wire_field_3),"itemPosition":wire_number(*wire_field_4),"item":encode_java_script_semantic_value(wire_field_5),"itemStart":wire_number(*wire_field_6),"length":wire_number(*wire_field_7),"index":wire_number(*wire_field_8),"outputIndex":wire_number(*wire_field_9),"spread":Data::Bool(*wire_field_10),"target":encode_java_script_semantic_value(wire_field_11),"value":encode_java_script_semantic_value(wire_field_12),"input":encode_java_script_semantic_value(wire_field_13),"phase":wire_text(&wire_field_14.0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_15 {
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
            if let Some((wire_present,)) = wire_field_16 {
                fields.insert(
                    "species".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_array_species_state(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_17 {
                fields.insert(
                    "allocationPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_18 {
                fields.insert(
                    "constructor".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_19 {
                fields.insert(
                    "constructorFact".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_array_constructor_fact(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_20 {
                fields.insert(
                    "assignment".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_object_spread_state(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_21 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_array_concatenation_call(wire_present)
                        }
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_array_species_state(
    data: &Data,
) -> native::JavaScriptArraySpeciesState {
    match data["$"].as_str().expect("model constructor") {
        "arraySpeciesState" => native::JavaScriptArraySpeciesState::ArraySpeciesState(
            Box::new(decode_java_script_semantic_value(&data["original"])),
            data["length"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["arrayConstructor"])),
            Box::new(decode_java_script_semantic_value(&data["arrayPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["speciesKey"])),
            Box::new(decode_java_script_semantic_value(&data["candidate"])),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            data.get("call").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_array_species_call(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_species_state(
    value: &native::JavaScriptArraySpeciesState,
) -> Data {
    match value {
        native::JavaScriptArraySpeciesState::ArraySpeciesState(
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
            let mut result = json!({"$":"arraySpeciesState","original":encode_java_script_semantic_value(wire_field_0),"length":wire_number(*wire_field_1),"realm":Data::String(wire_field_2.to_string()),"arrayConstructor":encode_java_script_semantic_value(wire_field_3),"arrayPrototype":encode_java_script_semantic_value(wire_field_4),"speciesKey":encode_java_script_semantic_value(wire_field_5),"candidate":encode_java_script_semantic_value(wire_field_6),"phase":wire_text(&wire_field_7.0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_array_species_call(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_array_species_call(
    data: &Data,
) -> native::JavaScriptArraySpeciesCall {
    match data["$"].as_str().expect("model constructor") {
        "arraySpeciesSelectedCall" => native::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(
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

pub(in super::super) fn encode_java_script_array_species_call(
    value: &native::JavaScriptArraySpeciesCall,
) -> Data {
    match value {
        native::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arraySpeciesSelectedCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_object_spread_state(
    data: &Data,
) -> native::JavaScriptObjectSpreadState {
    match data["$"].as_str().expect("model constructor") {
        "objectSpreadState" => native::JavaScriptObjectSpreadState::ObjectSpreadState(
            Box::new(decode_java_script_semantic_value(&data["target"])),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["excluded"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["keys"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            data["position"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
            Box::new(decode_java_script_semantic_value(&data["callee"])),
            data.get("operation").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(native::ml_string::Text::from_units(wire_text_units(
                        &wire_value,
                    )))
                },)
            }),
            data.get("exoticKey").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
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
                    Some(decode_java_script_object_copy_call(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_object_spread_state(
    value: &native::JavaScriptObjectSpreadState,
) -> Data {
    match value {
        native::JavaScriptObjectSpreadState::ObjectSpreadState(
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
            let mut result = json!({"$":"objectSpreadState","target":encode_java_script_semantic_value(wire_field_0),"input":encode_java_script_semantic_value(wire_field_1),"excluded":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"keys":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"position":wire_number(*wire_field_4),"phase":wire_text(&wire_field_5.0),"value":encode_java_script_semantic_value(wire_field_6),"callee":encode_java_script_semantic_value(wire_field_7)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "operation".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_9 {
                fields.insert(
                    "exoticKey".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_10 {
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
            if let Some((wire_present,)) = wire_field_11 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_object_copy_call(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_array_assignment_state(
    data: &Data,
) -> native::JavaScriptArrayAssignmentState {
    match data["$"].as_str().expect("model constructor") {
        "arrayAssignmentDefinition" => {
            native::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                Box::new(decode_java_script_array_definition_state(
                    &data["definition"],
                )),
                Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
            )
        }
        "arrayAssignmentNumber" => native::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
            Box::new(decode_java_script_array_definition_state(
                &data["definition"],
            )),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
            Box::new(decode_java_script_primitive_conversion_state(
                &data["conversion"],
            )),
        ),
        "arrayAssignmentPrimitive" => {
            native::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                Box::new(decode_java_script_array_definition_state(
                    &data["definition"],
                )),
                Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
                Box::new(decode_java_script_semantic_value(&data["value"])),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_assignment_state(
    value: &native::JavaScriptArrayAssignmentState,
) -> Data {
    match value {
        native::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"arrayAssignmentDefinition","definition":encode_java_script_array_definition_state(wire_field_0),"exoticKey":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arrayAssignmentNumber","definition":encode_java_script_array_definition_state(wire_field_0),"exoticKey":encode_java_script_semantic_value(wire_field_1),"conversion":encode_java_script_primitive_conversion_state(wire_field_2)})
        }
        native::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arrayAssignmentPrimitive","definition":encode_java_script_array_definition_state(wire_field_0),"exoticKey":encode_java_script_semantic_value(wire_field_1),"value":encode_java_script_semantic_value(wire_field_2)})
        }
    }
}

pub(in super::super) fn decode_java_script_array_definition_state(
    data: &Data,
) -> native::JavaScriptArrayDefinitionState {
    match data["$"].as_str().expect("model constructor") {
        "arrayDefinitionState" => native::JavaScriptArrayDefinitionState::ArrayDefinitionState(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
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
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            data["unsignedLength"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["numberLength"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_definition_state(
    value: &native::JavaScriptArrayDefinitionState,
) -> Data {
    match value {
        native::JavaScriptArrayDefinitionState::ArrayDefinitionState(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"arrayDefinitionState","identity":Data::String(wire_field_0.to_string()),"key":encode_java_script_semantic_value(wire_field_1),"descriptor":{ let wire_reference = wire_field_2.borrow(); encode_wire_ordered_record([("configurable", Some(Data::Bool(wire_reference.0.0))),("configurablePresent", Some(Data::Bool(wire_reference.0.1))),("enumerable", Some(Data::Bool(wire_reference.0.2))),("enumerablePresent", Some(Data::Bool(wire_reference.0.3))),("getter", Some(encode_java_script_semantic_value(&wire_reference.0.4))),("getterPresent", Some(Data::Bool(wire_reference.0.5))),("setter", Some(encode_java_script_semantic_value(&wire_reference.0.6))),("setterPresent", Some(Data::Bool(wire_reference.0.7))),("value", Some(encode_java_script_semantic_value(&wire_reference.0.8))),("valuePresent", Some(Data::Bool(wire_reference.0.9))),("writable", Some(Data::Bool(wire_reference.0.10))),("writablePresent", Some(Data::Bool(wire_reference.0.11)))], &wire_reference.1) },"phase":wire_text(&wire_field_3.0),"unsignedLength":wire_number(*wire_field_4),"numberLength":encode_java_script_semantic_value(wire_field_5)})
        }
    }
}

pub(in super::super) fn decode_java_script_object_copy_call(
    data: &Data,
) -> native::JavaScriptObjectCopyCall {
    match data["$"].as_str().expect("model constructor") {
        "objectCopyCall" => native::JavaScriptObjectCopyCall::ObjectCopyCall(
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

pub(in super::super) fn encode_java_script_object_copy_call(
    value: &native::JavaScriptObjectCopyCall,
) -> Data {
    match value {
        native::JavaScriptObjectCopyCall::ObjectCopyCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"objectCopyCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_array_concatenation_call(
    data: &Data,
) -> native::JavaScriptArrayConcatenationCall {
    match data["$"].as_str().expect("model constructor") {
        "arrayConcatenationCall" => {
            native::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(
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

pub(in super::super) fn encode_java_script_array_concatenation_call(
    value: &native::JavaScriptArrayConcatenationCall,
) -> Data {
    match value {
        native::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arrayConcatenationCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_species_construction_state(
    data: &Data,
) -> native::JavaScriptSpeciesConstructionState {
    match data["$"].as_str().expect("model constructor") {
        "speciesConstruction" => native::JavaScriptSpeciesConstructionState::SpeciesConstruction(
            Box::new(decode_java_script_semantic_value(&data["constructor"])),
            Box::new(decode_java_script_array_constructor_fact(&data["fact"])),
            data["length"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            native::ml_string::Text::from_units(wire_text_units(&data["mode"])),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            data.get("call").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_species_construction_call(wire_value))
                },)
            }),
            data.get("model").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(native::ml_string::Text::from_units(wire_text_units(
                        &wire_value,
                    )))
                },)
            }),
            data.get("operation").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_construction_state(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_species_construction_state(
    value: &native::JavaScriptSpeciesConstructionState,
) -> Data {
    match value {
        native::JavaScriptSpeciesConstructionState::SpeciesConstruction(
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
        ) => {
            let mut result = json!({"$":"speciesConstruction","constructor":encode_java_script_semantic_value(wire_field_0),"fact":encode_java_script_array_constructor_fact(wire_field_1),"length":wire_number(*wire_field_2),"phase":wire_text(&wire_field_3.0),"mode":wire_text(&wire_field_4.0),"prototype":encode_java_script_semantic_value(wire_field_5),"receiver":encode_java_script_semantic_value(wire_field_6)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_7 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_species_construction_call(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "model".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_9 {
                fields.insert(
                    "operation".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_construction_state(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_species_construction_call(
    data: &Data,
) -> native::JavaScriptSpeciesConstructionCall {
    match data["$"].as_str().expect("model constructor") {
        "speciesConstructionCall" => {
            native::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(
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

pub(in super::super) fn encode_java_script_species_construction_call(
    value: &native::JavaScriptSpeciesConstructionCall,
) -> Data {
    match value {
        native::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"speciesConstructionCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_construction_state(
    data: &Data,
) -> native::JavaScriptConstructionState {
    match data["$"].as_str().expect("model constructor") {
        "constructionState" => native::JavaScriptConstructionState::ConstructionState(
            Box::new(decode_java_script_semantic_value(&data["target"])),
            Box::new(decode_java_script_semantic_value(&data["newTarget"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            native::ml_string::Text::from_units(wire_text_units(&data["mode"])),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            Box::new(decode_java_script_semantic_value(&data["primitive"])),
            data["arrayIndex"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["allocationKind"])),
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
                    Some(decode_java_script_construction_call(wire_value))
                },)
            }),
            data.get("operation").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_construction_state(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_construction_state(
    value: &native::JavaScriptConstructionState,
) -> Data {
    match value {
        native::JavaScriptConstructionState::ConstructionState(
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
            let mut result = json!({"$":"constructionState","target":encode_java_script_semantic_value(wire_field_0),"newTarget":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"phase":wire_text(&wire_field_3.0),"mode":wire_text(&wire_field_4.0),"prototype":encode_java_script_semantic_value(wire_field_5),"receiver":encode_java_script_semantic_value(wire_field_6),"primitive":encode_java_script_semantic_value(wire_field_7),"arrayIndex":wire_number(*wire_field_8),"allocationKind":wire_text(&wire_field_9.0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_10 {
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
            if let Some((wire_present,)) = wire_field_11 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_construction_call(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_12 {
                fields.insert(
                    "operation".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_construction_state(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_construction_call(
    data: &Data,
) -> native::JavaScriptConstructionCall {
    match data["$"].as_str().expect("model constructor") {
        "constructionCallRecord" => native::JavaScriptConstructionCall::ConstructionCallRecord(
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
            data["constructing"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_semantic_value(&data["newTarget"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_construction_call(
    value: &native::JavaScriptConstructionCall,
) -> Data {
    match value {
        native::JavaScriptConstructionCall::ConstructionCallRecord(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"constructionCallRecord","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"constructing":Data::Bool(*wire_field_3),"newTarget":encode_java_script_semantic_value(wire_field_4)})
        }
    }
}
