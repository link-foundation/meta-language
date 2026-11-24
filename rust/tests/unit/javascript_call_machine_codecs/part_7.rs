use super::{
    Data, decode_java_script_array_constructor_fact, decode_java_script_array_species_state,
    decode_java_script_descriptor_conversion_state, decode_java_script_object_spread_state,
    decode_java_script_ordinary_property, decode_java_script_primitive_conversion_state,
    decode_java_script_property_definition_state, decode_java_script_property_definitions_call,
    decode_java_script_semantic_value, encode_java_script_array_constructor_fact,
    encode_java_script_array_species_state, encode_java_script_object_spread_state,
    encode_java_script_primitive_conversion_state, encode_java_script_semantic_value,
    is_wire_undefined, native, wire_number, wire_text, wire_text_units, wire_undefined,
};

pub(in super::super) fn decode_java_script_array_slice_state(
    data: &Data,
) -> native::JavaScriptArraySliceState {
    match data["$"].as_str().expect("model constructor") {
        "arraySliceState" => native::JavaScriptArraySliceState::ArraySliceState(
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            Box::new(decode_java_script_semantic_value(&data["start"])),
            Box::new(decode_java_script_semantic_value(&data["end"])),
            data["endPresent"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
            data["length"].as_f64().expect("model Number"),
            data["startIndex"].as_f64().expect("model Number"),
            data["endIndex"].as_f64().expect("model Number"),
            data["count"].as_f64().expect("model Number"),
            data["index"].as_f64().expect("model Number"),
            data["outputIndex"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["target"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            native::ml_string::Text::from_units(wire_text_units(&data["conversionKind"])),
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
                    Some(decode_java_script_array_slice_call(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_slice_state(
    value: &native::JavaScriptArraySliceState,
) -> Data {
    match value {
        native::JavaScriptArraySliceState::ArraySliceState(
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
        ) => {
            let mut result = json!({"$":"arraySliceState","receiver":encode_java_script_semantic_value(wire_field_0),"start":encode_java_script_semantic_value(wire_field_1),"end":encode_java_script_semantic_value(wire_field_2),"endPresent":Data::Bool(*wire_field_3),"exoticKey":encode_java_script_semantic_value(wire_field_4),"length":wire_number(*wire_field_5),"startIndex":wire_number(*wire_field_6),"endIndex":wire_number(*wire_field_7),"count":wire_number(*wire_field_8),"index":wire_number(*wire_field_9),"outputIndex":wire_number(*wire_field_10),"target":encode_java_script_semantic_value(wire_field_11),"value":encode_java_script_semantic_value(wire_field_12),"input":encode_java_script_semantic_value(wire_field_13),"phase":wire_text(&wire_field_14.0),"conversionKind":wire_text(&wire_field_15.0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_16 {
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
            if let Some((wire_present,)) = wire_field_17 {
                fields.insert(
                    "species".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_array_species_state(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_18 {
                fields.insert(
                    "allocationPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_19 {
                fields.insert(
                    "constructor".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_20 {
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
            if let Some((wire_present,)) = wire_field_21 {
                fields.insert(
                    "assignment".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_object_spread_state(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_22 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_array_slice_call(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_array_slice_call(
    data: &Data,
) -> native::JavaScriptArraySliceCall {
    match data["$"].as_str().expect("model constructor") {
        "arraySliceCall" => native::JavaScriptArraySliceCall::ArraySliceCall(
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

pub(in super::super) fn encode_java_script_array_slice_call(
    value: &native::JavaScriptArraySliceCall,
) -> Data {
    match value {
        native::JavaScriptArraySliceCall::ArraySliceCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arraySliceCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_array_transform_state(
    data: &Data,
) -> native::JavaScriptArrayTransformState {
    match data["$"].as_str().expect("model constructor") {
        "arrayTransformState" => native::JavaScriptArrayTransformState::ArrayTransformState(
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            Box::new(decode_java_script_semantic_value(&data["callback"])),
            Box::new(decode_java_script_semantic_value(&data["thisArgument"])),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
            data["length"].as_f64().expect("model Number"),
            data["index"].as_f64().expect("model Number"),
            data["outputIndex"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["target"])),
            Box::new(decode_java_script_semantic_value(&data["element"])),
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
            data.get("call").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_array_transform_call(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_transform_state(
    value: &native::JavaScriptArrayTransformState,
) -> Data {
    match value {
        native::JavaScriptArrayTransformState::ArrayTransformState(
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
        ) => {
            let mut result = json!({"$":"arrayTransformState","operation":wire_text(&wire_field_0.0),"receiver":encode_java_script_semantic_value(wire_field_1),"callback":encode_java_script_semantic_value(wire_field_2),"thisArgument":encode_java_script_semantic_value(wire_field_3),"exoticKey":encode_java_script_semantic_value(wire_field_4),"length":wire_number(*wire_field_5),"index":wire_number(*wire_field_6),"outputIndex":wire_number(*wire_field_7),"target":encode_java_script_semantic_value(wire_field_8),"element":encode_java_script_semantic_value(wire_field_9),"value":encode_java_script_semantic_value(wire_field_10),"input":encode_java_script_semantic_value(wire_field_11),"phase":wire_text(&wire_field_12.0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_13 {
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
            if let Some((wire_present,)) = wire_field_14 {
                fields.insert(
                    "species".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_array_species_state(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_15 {
                fields.insert(
                    "allocationPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_16 {
                fields.insert(
                    "constructor".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_17 {
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
            if let Some((wire_present,)) = wire_field_18 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_array_transform_call(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_array_transform_call(
    data: &Data,
) -> native::JavaScriptArrayTransformCall {
    match data["$"].as_str().expect("model constructor") {
        "arrayTransformSelectedCall" => {
            native::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(
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

pub(in super::super) fn encode_java_script_array_transform_call(
    value: &native::JavaScriptArrayTransformCall,
) -> Data {
    match value {
        native::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arrayTransformSelectedCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_array_search_state(
    data: &Data,
) -> native::JavaScriptArraySearchState {
    match data["$"].as_str().expect("model constructor") {
        "arraySearchState" => native::JavaScriptArraySearchState::ArraySearchState(
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            Box::new(decode_java_script_semantic_value(&data["searchElement"])),
            Box::new(decode_java_script_semantic_value(&data["fromIndex"])),
            data["fromPresent"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
            data["length"].as_f64().expect("model Number"),
            data["index"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            Box::new(decode_java_script_semantic_value(&data["input"])),
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
                    Some(decode_java_script_array_search_call(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_search_state(
    value: &native::JavaScriptArraySearchState,
) -> Data {
    match value {
        native::JavaScriptArraySearchState::ArraySearchState(
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
            let mut result = json!({"$":"arraySearchState","operation":wire_text(&wire_field_0.0),"receiver":encode_java_script_semantic_value(wire_field_1),"searchElement":encode_java_script_semantic_value(wire_field_2),"fromIndex":encode_java_script_semantic_value(wire_field_3),"fromPresent":Data::Bool(*wire_field_4),"exoticKey":encode_java_script_semantic_value(wire_field_5),"length":wire_number(*wire_field_6),"index":wire_number(*wire_field_7),"phase":wire_text(&wire_field_8.0),"input":encode_java_script_semantic_value(wire_field_9)});
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
                        Some(wire_present) => encode_java_script_array_search_call(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_array_search_call(
    data: &Data,
) -> native::JavaScriptArraySearchCall {
    match data["$"].as_str().expect("model constructor") {
        "arraySearchSelectedCall" => native::JavaScriptArraySearchCall::ArraySearchSelectedCall(
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

pub(in super::super) fn encode_java_script_array_search_call(
    value: &native::JavaScriptArraySearchCall,
) -> Data {
    match value {
        native::JavaScriptArraySearchCall::ArraySearchSelectedCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arraySearchSelectedCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_collection_for_each_state(
    data: &Data,
) -> native::JavaScriptCollectionForEachState {
    match data["$"].as_str().expect("model constructor") {
        "collectionForEachState" => {
            native::JavaScriptCollectionForEachState::CollectionForEachState(
                native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
                Box::new(decode_java_script_semantic_value(&data["receiver"])),
                Box::new(decode_java_script_semantic_value(&data["callback"])),
                Box::new(decode_java_script_semantic_value(&data["thisArgument"])),
                data["cursor"].as_f64().expect("model Number"),
                native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
                Box::new(decode_java_script_semantic_value(&data["key"])),
                Box::new(decode_java_script_semantic_value(&data["value"])),
                data.get("call").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_collection_for_each_call(wire_value))
                    },)
                }),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_collection_for_each_state(
    value: &native::JavaScriptCollectionForEachState,
) -> Data {
    match value {
        native::JavaScriptCollectionForEachState::CollectionForEachState(
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
            let mut result = json!({"$":"collectionForEachState","operation":wire_text(&wire_field_0.0),"receiver":encode_java_script_semantic_value(wire_field_1),"callback":encode_java_script_semantic_value(wire_field_2),"thisArgument":encode_java_script_semantic_value(wire_field_3),"cursor":wire_number(*wire_field_4),"phase":wire_text(&wire_field_5.0),"key":encode_java_script_semantic_value(wire_field_6),"value":encode_java_script_semantic_value(wire_field_7)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_collection_for_each_call(wire_present)
                        }
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_collection_for_each_call(
    data: &Data,
) -> native::JavaScriptCollectionForEachCall {
    match data["$"].as_str().expect("model constructor") {
        "collectionForEachSelectedCall" => {
            native::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(
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

pub(in super::super) fn encode_java_script_collection_for_each_call(
    value: &native::JavaScriptCollectionForEachCall,
) -> Data {
    match value {
        native::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"collectionForEachSelectedCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_array_callback_state(
    data: &Data,
) -> native::JavaScriptArrayCallbackState {
    match data["$"].as_str().expect("model constructor") {
        "arrayCallbackState" => native::JavaScriptArrayCallbackState::ArrayCallbackState(
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            Box::new(decode_java_script_semantic_value(&data["callback"])),
            Box::new(decode_java_script_semantic_value(&data["thisArgument"])),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
            data["length"].as_f64().expect("model Number"),
            data["index"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            data["initialPresent"].as_bool().expect("model Boolean"),
            data["hasAccumulator"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_semantic_value(&data["accumulator"])),
            Box::new(decode_java_script_semantic_value(&data["element"])),
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
                    Some(decode_java_script_array_callback_call(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_callback_state(
    value: &native::JavaScriptArrayCallbackState,
) -> Data {
    match value {
        native::JavaScriptArrayCallbackState::ArrayCallbackState(
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
        ) => {
            let mut result = json!({"$":"arrayCallbackState","operation":wire_text(&wire_field_0.0),"receiver":encode_java_script_semantic_value(wire_field_1),"callback":encode_java_script_semantic_value(wire_field_2),"thisArgument":encode_java_script_semantic_value(wire_field_3),"exoticKey":encode_java_script_semantic_value(wire_field_4),"length":wire_number(*wire_field_5),"index":wire_number(*wire_field_6),"phase":wire_text(&wire_field_7.0),"initialPresent":Data::Bool(*wire_field_8),"hasAccumulator":Data::Bool(*wire_field_9),"accumulator":encode_java_script_semantic_value(wire_field_10),"element":encode_java_script_semantic_value(wire_field_11)});
            let fields = result.as_object_mut().expect("model constructor object");
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
                        Some(wire_present) => encode_java_script_array_callback_call(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_array_callback_call(
    data: &Data,
) -> native::JavaScriptArrayCallbackCall {
    match data["$"].as_str().expect("model constructor") {
        "arrayCallbackSelectedCall" => {
            native::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(
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

pub(in super::super) fn encode_java_script_array_callback_call(
    value: &native::JavaScriptArrayCallbackCall,
) -> Data {
    match value {
        native::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arrayCallbackSelectedCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_object_enumeration_state(
    data: &Data,
) -> native::JavaScriptObjectEnumerationState {
    match data["$"].as_str().expect("model constructor") {
        "objectEnumeration" => native::JavaScriptObjectEnumerationState::ObjectEnumeration(
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["keys"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["values"]
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
            Box::new(decode_java_script_semantic_value(&data["pair"])),
            data["pairIndex"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["target"])),
            data["outputIndex"].as_f64().expect("model Number"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_object_enumeration_state(
    value: &native::JavaScriptObjectEnumerationState,
) -> Data {
    match value {
        native::JavaScriptObjectEnumerationState::ObjectEnumeration(
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
            json!({"$":"objectEnumeration","operation":wire_text(&wire_field_0.0),"input":encode_java_script_semantic_value(wire_field_1),"prototype":encode_java_script_semantic_value(wire_field_2),"keys":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"values":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"position":wire_number(*wire_field_5),"phase":wire_text(&wire_field_6.0),"value":encode_java_script_semantic_value(wire_field_7),"callee":encode_java_script_semantic_value(wire_field_8),"pair":encode_java_script_semantic_value(wire_field_9),"pairIndex":wire_number(*wire_field_10),"target":encode_java_script_semantic_value(wire_field_11),"outputIndex":wire_number(*wire_field_12)})
        }
    }
}

pub(in super::super) fn decode_java_script_property_definitions_state(
    data: &Data,
) -> native::JavaScriptPropertyDefinitionsState {
    match data["$"].as_str().expect("model constructor") {
        "propertyDefinitions" => native::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            Box::new(decode_java_script_semantic_value(&data["target"])),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
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
                data["descriptors"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            data["definitionIndex"].as_f64().expect("model Number"),
            data.get("conversion").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_descriptor_conversion_state(wire_value))
                },)
            }),
            data.get("definition").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_property_definition_state(wire_value))
                },)
            }),
            data.get("call").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_property_definitions_call(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
