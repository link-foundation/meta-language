use super::{
    Data, decode_java_script_symbol_registry_entry, decode_wire_record_order,
    encode_java_script_symbol_registry_entry, encode_wire_ordered_record, is_wire_undefined,
    native, wire_number, wire_text, wire_text_units, wire_undefined,
};

pub(in super::super) fn decode_java_script_ordinary_heap(
    data: &Data,
) -> native::JavaScriptOrdinaryHeap {
    match data["$"].as_str().expect("model constructor") {
        "ordinaryHeap" => native::JavaScriptOrdinaryHeap::OrdinaryHeap(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["objects"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_object)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["symbols"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| {
                        native::ml::Big::parse(wire_value.as_str().expect("model exact integer"))
                    })
                    .collect(),
            )),
            data.get("symbolRegistrationModel").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(native::ml_string::Text::from_units(wire_text_units(
                        &wire_value,
                    )))
                },)
            }),
            data.get("symbolRegistrations").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_symbol_registration_fact)
                            .collect(),
                    )))
                },)
            }),
            data.get("symbolRegistryModel").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(native::ml_string::Text::from_units(wire_text_units(
                        &wire_value,
                    )))
                },)
            }),
            data.get("symbolRegistry").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_symbol_registry_entry)
                            .collect(),
                    )))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_ordinary_heap(
    value: &native::JavaScriptOrdinaryHeap,
) -> Data {
    match value {
        native::JavaScriptOrdinaryHeap::OrdinaryHeap(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"ordinaryHeap","objects":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_object).collect()) },"symbols":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(|wire_value|Data::String(wire_value.to_string())).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "symbolRegistrationModel".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "symbolRegistrations".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_symbol_registration_fact)
                                    .collect(),
                            )
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "symbolRegistryModel".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "symbolRegistry".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_symbol_registry_entry)
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

pub(in super::super) fn decode_java_script_ordinary_object(
    data: &Data,
) -> native::JavaScriptOrdinaryObject {
    match data["$"].as_str().expect("model constructor") {
        "closureObject" => native::JavaScriptOrdinaryObject::ClosureObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            native::ml::Big::parse(data["owner"].as_str().expect("model exact integer")),
            data["ordinal"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["captures"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            Box::new(decode_java_script_semantic_value(&data["lexicalReceiver"])),
            data.get("lexicalNewTarget").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
        ),
        "boundFunctionObject" => native::JavaScriptOrdinaryObject::BoundFunctionObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            Box::new(decode_java_script_semantic_value(&data["boundTarget"])),
            Box::new(decode_java_script_semantic_value(&data["boundThis"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["boundArguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
        ),
        "arrayObject" => native::JavaScriptOrdinaryObject::ArrayObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
        ),
        "collectionIteratorObject" => native::JavaScriptOrdinaryObject::CollectionIteratorObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            native::ml::Big::parse(data["collection"].as_str().expect("model exact integer")),
            data["cursor"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
            native::ml_string::Text::from_units(wire_text_units(&data["collectionKind"])),
            data.get("executing").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
            data.get("iteratedText").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
        ),
        "mapObject" => native::JavaScriptOrdinaryObject::MapObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["entries"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_collection_entry)
                    .collect(),
            )),
        ),
        "setObject" => native::JavaScriptOrdinaryObject::SetObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["entries"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_collection_entry)
                    .collect(),
            )),
        ),
        "weakMapObject" => native::JavaScriptOrdinaryObject::WeakMapObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["weakEntries"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_weak_collection_entry)
                    .collect(),
            )),
        ),
        "weakSetObject" => native::JavaScriptOrdinaryObject::WeakSetObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["weakEntries"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_weak_collection_entry)
                    .collect(),
            )),
        ),
        "ordinaryObject" => native::JavaScriptOrdinaryObject::OrdinaryObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data.get("immutablePrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
            data.get("regularExpression").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
            data.get("stringData").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("primitiveData").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
        ),
        "hostErrorObject" => native::JavaScriptOrdinaryObject::HostErrorObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            Box::new(decode_java_script_semantic_value(&data["hostGet"])),
            Box::new(decode_java_script_semantic_value(&data["hostSet"])),
        ),
        "hostURLObject" => native::JavaScriptOrdinaryObject::HostURLObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            Box::new(decode_java_script_semantic_value(&data["hostGet"])),
            Box::new(decode_java_script_semantic_value(&data["hostSet"])),
        ),
        "hostArrayObject" => native::JavaScriptOrdinaryObject::HostArrayObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            Box::new(decode_java_script_semantic_value(&data["hostGet"])),
            Box::new(decode_java_script_semantic_value(&data["hostSet"])),
        ),
        "hostByteBufferObject" => native::JavaScriptOrdinaryObject::HostByteBufferObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_ordinary_property)
                    .collect(),
            )),
            Box::new(decode_java_script_semantic_value(&data["hostGet"])),
            Box::new(decode_java_script_semantic_value(&data["hostSet"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_ordinary_object(
    value: &native::JavaScriptOrdinaryObject,
) -> Data {
    match value {
        native::JavaScriptOrdinaryObject::ClosureObject(
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
            let mut result = json!({"$":"closureObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"owner":Data::String(wire_field_5.to_string()),"ordinal":wire_number(*wire_field_6),"captures":{ let wire_reference = wire_field_7.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"lexicalReceiver":encode_java_script_semantic_value(wire_field_8)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_9 {
                fields.insert(
                    "lexicalNewTarget".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptOrdinaryObject::BoundFunctionObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
        ) => {
            json!({"$":"boundFunctionObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"boundTarget":encode_java_script_semantic_value(wire_field_5),"boundThis":encode_java_script_semantic_value(wire_field_6),"boundArguments":{ let wire_reference = wire_field_7.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
        native::JavaScriptOrdinaryObject::ArrayObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"arrayObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) }})
        }
        native::JavaScriptOrdinaryObject::CollectionIteratorObject(
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
            let mut result = json!({"$":"collectionIteratorObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"collection":Data::String(wire_field_5.to_string()),"cursor":wire_number(*wire_field_6),"kind":wire_text(&wire_field_7.0),"collectionKind":wire_text(&wire_field_8.0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_9 {
                fields.insert(
                    "executing".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_10 {
                fields.insert(
                    "iteratedText".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptOrdinaryObject::MapObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"mapObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"entries":{ let wire_reference = wire_field_5.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_collection_entry).collect()) }})
        }
        native::JavaScriptOrdinaryObject::SetObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"setObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"entries":{ let wire_reference = wire_field_5.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_collection_entry).collect()) }})
        }
        native::JavaScriptOrdinaryObject::WeakMapObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"weakMapObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"weakEntries":{ let wire_reference = wire_field_5.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_weak_collection_entry).collect()) }})
        }
        native::JavaScriptOrdinaryObject::WeakSetObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"weakSetObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"weakEntries":{ let wire_reference = wire_field_5.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_weak_collection_entry).collect()) }})
        }
        native::JavaScriptOrdinaryObject::OrdinaryObject(
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
            let mut result = json!({"$":"ordinaryObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_7),"properties":{ let wire_reference = wire_field_8.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "immutablePrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "regularExpression".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "stringData".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_6 {
                fields.insert(
                    "primitiveData".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptOrdinaryObject::HostErrorObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
        ) => {
            json!({"$":"hostErrorObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"hostGet":encode_java_script_semantic_value(wire_field_5),"hostSet":encode_java_script_semantic_value(wire_field_6)})
        }
        native::JavaScriptOrdinaryObject::HostURLObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
        ) => {
            json!({"$":"hostURLObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"hostGet":encode_java_script_semantic_value(wire_field_5),"hostSet":encode_java_script_semantic_value(wire_field_6)})
        }
        native::JavaScriptOrdinaryObject::HostArrayObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
        ) => {
            json!({"$":"hostArrayObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"hostGet":encode_java_script_semantic_value(wire_field_5),"hostSet":encode_java_script_semantic_value(wire_field_6)})
        }
        native::JavaScriptOrdinaryObject::HostByteBufferObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
        ) => {
            json!({"$":"hostByteBufferObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_ordinary_property).collect()) },"hostGet":encode_java_script_semantic_value(wire_field_5),"hostSet":encode_java_script_semantic_value(wire_field_6)})
        }
    }
}

pub(in super::super) fn decode_java_script_semantic_value(
    data: &Data,
) -> native::JavaScriptSemanticValue {
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
        "text" => native::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(
            data["units"]
                .as_array()
                .expect("model array")
                .iter()
                .map(|wire_value| wire_value.as_f64().expect("model Number"))
                .collect(),
        ))),
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

pub(in super::super) fn encode_java_script_semantic_value(
    value: &native::JavaScriptSemanticValue,
) -> Data {
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
            json!({"$":"text","units":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) }})
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

pub(in super::super) fn decode_java_script_ordinary_property(
    data: &Data,
) -> native::JavaScriptOrdinaryProperty {
    match data["$"].as_str().expect("model constructor") {
        "ordinaryProperty" => native::JavaScriptOrdinaryProperty::OrdinaryProperty(
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
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_ordinary_property(
    value: &native::JavaScriptOrdinaryProperty,
) -> Data {
    match value {
        native::JavaScriptOrdinaryProperty::OrdinaryProperty(wire_field_0, wire_field_1) => {
            json!({"$":"ordinaryProperty","key":encode_java_script_semantic_value(wire_field_0),"descriptor":{ let wire_reference = wire_field_1.borrow(); encode_wire_ordered_record([("configurable", Some(Data::Bool(wire_reference.0.0))),("configurablePresent", Some(Data::Bool(wire_reference.0.1))),("enumerable", Some(Data::Bool(wire_reference.0.2))),("enumerablePresent", Some(Data::Bool(wire_reference.0.3))),("getter", Some(encode_java_script_semantic_value(&wire_reference.0.4))),("getterPresent", Some(Data::Bool(wire_reference.0.5))),("setter", Some(encode_java_script_semantic_value(&wire_reference.0.6))),("setterPresent", Some(Data::Bool(wire_reference.0.7))),("value", Some(encode_java_script_semantic_value(&wire_reference.0.8))),("valuePresent", Some(Data::Bool(wire_reference.0.9))),("writable", Some(Data::Bool(wire_reference.0.10))),("writablePresent", Some(Data::Bool(wire_reference.0.11)))], &wire_reference.1) }})
        }
    }
}

pub(in super::super) fn decode_java_script_collection_entry(
    data: &Data,
) -> native::JavaScriptCollectionEntry {
    match data["$"].as_str().expect("model constructor") {
        "collectionEntry" => native::JavaScriptCollectionEntry::CollectionEntry(
            Box::new(decode_java_script_semantic_value(&data["key"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
            data["present"].as_bool().expect("model Boolean"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_collection_entry(
    value: &native::JavaScriptCollectionEntry,
) -> Data {
    match value {
        native::JavaScriptCollectionEntry::CollectionEntry(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"collectionEntry","key":encode_java_script_semantic_value(wire_field_0),"value":encode_java_script_semantic_value(wire_field_1),"present":Data::Bool(*wire_field_2)})
        }
    }
}

pub(in super::super) fn decode_java_script_weak_collection_entry(
    data: &Data,
) -> native::JavaScriptWeakCollectionEntry {
    match data["$"].as_str().expect("model constructor") {
        "weakCollectionEntry" => native::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
            Box::new(decode_java_script_semantic_value(&data["key"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_weak_collection_entry(
    value: &native::JavaScriptWeakCollectionEntry,
) -> Data {
    match value {
        native::JavaScriptWeakCollectionEntry::WeakCollectionEntry(wire_field_0, wire_field_1) => {
            json!({"$":"weakCollectionEntry","key":encode_java_script_semantic_value(wire_field_0),"value":encode_java_script_semantic_value(wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_symbol_registration_fact(
    data: &Data,
) -> native::JavaScriptSymbolRegistrationFact {
    match data["$"].as_str().expect("model constructor") {
        "symbolRegistrationFact" => {
            native::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
                data["registered"].as_bool().expect("model Boolean"),
                data.get("key").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_semantic_value(wire_value))
                    },)
                }),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_symbol_registration_fact(
    value: &native::JavaScriptSymbolRegistrationFact,
) -> Data {
    match value {
        native::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"symbolRegistrationFact","identity":Data::String(wire_field_0.to_string()),"registered":Data::Bool(*wire_field_1)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "key".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            result
        }
    }
}
