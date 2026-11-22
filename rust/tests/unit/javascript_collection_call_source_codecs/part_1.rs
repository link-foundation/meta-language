use super::{
    Data, encode_java_script_collection_iterator_method_owner, is_wire_undefined, native,
    wire_number, wire_text, wire_text_units, wire_undefined,
};

pub(in super::super) fn decode_java_script_collection_callable_directory(
    data: &Data,
) -> native::JavaScriptCollectionCallableDirectory {
    match data["$"].as_str().expect("model constructor") {
        "collectionCallableDirectory" => {
            native::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["realms"]
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
                Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["constructors"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_collection_constructor_owner)
                        .collect(),
                )),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["primitiveMethods"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_primitive_method_owner)
                        .collect(),
                )),
                data.get("iteratorMethods").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(std::rc::Rc::new(std::cell::RefCell::new(
                            wire_value
                                .as_array()
                                .expect("model array")
                                .iter()
                                .map(decode_java_script_collection_iterator_method_owner)
                                .collect(),
                        )))
                    },)
                }),
                data.get("ownerBudget").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(wire_value.as_f64().expect("model Number"))
                    },)
                }),
                data.get("ownerProfile").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(native::ml_string::Text::from_units(wire_text_units(
                            &wire_value,
                        )))
                    },)
                }),
                data.get("constructorDirectory").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_array_species_directory(wire_value))
                    },)
                }),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_collection_callable_directory(
    value: &native::JavaScriptCollectionCallableDirectory,
) -> Data {
    match value {
        native::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
        ) => {
            let mut result = json!({"$":"collectionCallableDirectory","realms":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(|wire_value|Data::String(wire_value.to_string())).collect()) },"exoticKey":encode_java_script_semantic_value(wire_field_1),"constructors":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_collection_constructor_owner).collect()) },"primitiveMethods":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_primitive_method_owner).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "iteratorMethods".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_collection_iterator_method_owner)
                                    .collect(),
                            )
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "ownerBudget".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_6 {
                fields.insert(
                    "ownerProfile".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_7 {
                fields.insert(
                    "constructorDirectory".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_array_species_directory(wire_present)
                        }
                    },
                );
            }
            result
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

pub(in super::super) fn decode_java_script_collection_constructor_owner(
    data: &Data,
) -> native::JavaScriptCollectionConstructorOwner {
    match data["$"].as_str().expect("model constructor") {
        "collectionConstructorOwner" => {
            native::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(
                native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
                native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
                Box::new(decode_java_script_semantic_value(&data["prototype"])),
                Box::new(decode_java_script_semantic_value(&data["adder"])),
                Box::new(decode_java_script_semantic_value(&data["iteratorKey"])),
                Box::new(decode_java_script_semantic_value(&data["iteratorMethod"])),
                Box::new(decode_java_script_semantic_value(
                    &data["iteratorPrototype"],
                )),
                Box::new(decode_java_script_semantic_value(&data["nextMethod"])),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["methods"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_collection_method_owner)
                        .collect(),
                )),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_collection_constructor_owner(
    value: &native::JavaScriptCollectionConstructorOwner,
) -> Data {
    match value {
        native::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(
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
            json!({"$":"collectionConstructorOwner","kind":wire_text(&wire_field_0.0),"identity":Data::String(wire_field_1.to_string()),"realm":Data::String(wire_field_2.to_string()),"prototype":encode_java_script_semantic_value(wire_field_3),"adder":encode_java_script_semantic_value(wire_field_4),"iteratorKey":encode_java_script_semantic_value(wire_field_5),"iteratorMethod":encode_java_script_semantic_value(wire_field_6),"iteratorPrototype":encode_java_script_semantic_value(wire_field_7),"nextMethod":encode_java_script_semantic_value(wire_field_8),"methods":{ let wire_reference = wire_field_9.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_collection_method_owner).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_collection_method_owner(
    data: &Data,
) -> native::JavaScriptCollectionMethodOwner {
    match data["$"].as_str().expect("model constructor") {
        "collectionMethodOwner" => native::JavaScriptCollectionMethodOwner::CollectionMethodOwner(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_collection_method_owner(
    value: &native::JavaScriptCollectionMethodOwner,
) -> Data {
    match value {
        native::JavaScriptCollectionMethodOwner::CollectionMethodOwner(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"collectionMethodOwner","identity":Data::String(wire_field_0.to_string()),"operation":wire_text(&wire_field_1.0)})
        }
    }
}

pub(in super::super) fn decode_java_script_primitive_method_owner(
    data: &Data,
) -> native::JavaScriptPrimitiveMethodOwner {
    match data["$"].as_str().expect("model constructor") {
        "primitiveMethodOwner" => native::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            data.get("species").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_array_species_directory(wire_value))
                },)
            }),
            data.get("arrayPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("objectPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("hasInstanceKey").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("matchKey").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("constructable").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
            data.get("unicodeProfile").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(native::ml_string::Text::from_units(wire_text_units(
                        &wire_value,
                    )))
                },)
            }),
            data.get("wrapperRoots").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_primitive_wrapper_roots(wire_value))
                },)
            }),
            data.get("iteratorKey").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("tagKey").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("spreadableKey").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("objectString").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_primitive_method_owner(
    value: &native::JavaScriptPrimitiveMethodOwner,
) -> Data {
    match value {
        native::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(
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
        ) => {
            let mut result = json!({"$":"primitiveMethodOwner","identity":Data::String(wire_field_0.to_string()),"realm":Data::String(wire_field_1.to_string()),"operation":wire_text(&wire_field_2.0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "species".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_array_species_directory(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "arrayPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "objectPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_6 {
                fields.insert(
                    "hasInstanceKey".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_7 {
                fields.insert(
                    "matchKey".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "constructable".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_9 {
                fields.insert(
                    "unicodeProfile".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_10 {
                fields.insert(
                    "wrapperRoots".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_primitive_wrapper_roots(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_11 {
                fields.insert(
                    "iteratorKey".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_12 {
                fields.insert(
                    "tagKey".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_13 {
                fields.insert(
                    "spreadableKey".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_14 {
                fields.insert(
                    "objectString".into(),
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

pub(in super::super) fn decode_java_script_array_species_directory(
    data: &Data,
) -> native::JavaScriptArraySpeciesDirectory {
    match data["$"].as_str().expect("model constructor") {
        "arraySpeciesDirectory" => native::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["arrayConstructor"])),
            Box::new(decode_java_script_semantic_value(&data["arrayPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["speciesKey"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["constructors"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_array_constructor_fact)
                    .collect(),
            )),
            data.get("constructorRealms").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_constructor_realm)
                            .collect(),
                    )))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_species_directory(
    value: &native::JavaScriptArraySpeciesDirectory,
) -> Data {
    match value {
        native::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"arraySpeciesDirectory","realm":Data::String(wire_field_0.to_string()),"arrayConstructor":encode_java_script_semantic_value(wire_field_1),"arrayPrototype":encode_java_script_semantic_value(wire_field_2),"speciesKey":encode_java_script_semantic_value(wire_field_3),"constructors":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_array_constructor_fact).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "constructorRealms".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_constructor_realm)
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

pub(in super::super) fn decode_java_script_array_constructor_fact(
    data: &Data,
) -> native::JavaScriptArrayConstructorFact {
    match data["$"].as_str().expect("model constructor") {
        "arrayConstructorFact" => native::JavaScriptArrayConstructorFact::ArrayConstructorFact(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            data["constructable"].as_bool().expect("model Boolean"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            data["intrinsicArray"].as_bool().expect("model Boolean"),
            data.get("intrinsicKind").map(|wire_value| {
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

pub(in super::super) fn encode_java_script_array_constructor_fact(
    value: &native::JavaScriptArrayConstructorFact,
) -> Data {
    match value {
        native::JavaScriptArrayConstructorFact::ArrayConstructorFact(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            let mut result = json!({"$":"arrayConstructorFact","identity":Data::String(wire_field_0.to_string()),"constructable":Data::Bool(*wire_field_1),"realm":Data::String(wire_field_2.to_string()),"intrinsicArray":Data::Bool(*wire_field_3)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "intrinsicKind".into(),
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

pub(in super::super) fn decode_java_script_constructor_realm(
    data: &Data,
) -> native::JavaScriptConstructorRealm {
    match data["$"].as_str().expect("model constructor") {
        "constructorRealm" => native::JavaScriptConstructorRealm::ConstructorRealm(
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["objectPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["arrayPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["arrayConstructor"])),
            data.get("numberPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("booleanPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("stringPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("integerPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("symbolPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("mapPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("setPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("weakMapPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("weakSetPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_constructor_realm(
    value: &native::JavaScriptConstructorRealm,
) -> Data {
    match value {
        native::JavaScriptConstructorRealm::ConstructorRealm(
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
            let mut result = json!({"$":"constructorRealm","realm":Data::String(wire_field_0.to_string()),"objectPrototype":encode_java_script_semantic_value(wire_field_1),"arrayPrototype":encode_java_script_semantic_value(wire_field_2),"arrayConstructor":encode_java_script_semantic_value(wire_field_3)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "numberPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "booleanPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_6 {
                fields.insert(
                    "stringPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_7 {
                fields.insert(
                    "integerPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "symbolPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_9 {
                fields.insert(
                    "mapPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_10 {
                fields.insert(
                    "setPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_11 {
                fields.insert(
                    "weakMapPrototype".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_12 {
                fields.insert(
                    "weakSetPrototype".into(),
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

pub(in super::super) fn decode_java_script_primitive_wrapper_roots(
    data: &Data,
) -> native::JavaScriptPrimitiveWrapperRoots {
    match data["$"].as_str().expect("model constructor") {
        "primitiveWrapperRoots" => native::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(
            Box::new(decode_java_script_semantic_value(&data["objectPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["stringPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["numberPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["booleanPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["integerPrototype"])),
            Box::new(decode_java_script_semantic_value(&data["symbolPrototype"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_primitive_wrapper_roots(
    value: &native::JavaScriptPrimitiveWrapperRoots,
) -> Data {
    match value {
        native::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"primitiveWrapperRoots","objectPrototype":encode_java_script_semantic_value(wire_field_0),"stringPrototype":encode_java_script_semantic_value(wire_field_1),"numberPrototype":encode_java_script_semantic_value(wire_field_2),"booleanPrototype":encode_java_script_semantic_value(wire_field_3),"integerPrototype":encode_java_script_semantic_value(wire_field_4),"symbolPrototype":encode_java_script_semantic_value(wire_field_5)})
        }
    }
}

pub(in super::super) fn decode_java_script_collection_iterator_method_owner(
    data: &Data,
) -> native::JavaScriptCollectionIteratorMethodOwner {
    match data["$"].as_str().expect("model constructor") {
        "collectionIteratorMethodOwner" => {
            native::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(
                native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
                native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
                Box::new(decode_java_script_semantic_value(
                    &data["iteratorPrototype"],
                )),
                Box::new(decode_java_script_semantic_value(&data["arrayPrototype"])),
                Box::new(decode_java_script_semantic_value(&data["objectPrototype"])),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}
