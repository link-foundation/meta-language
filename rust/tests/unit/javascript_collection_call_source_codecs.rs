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
pub(super) fn decode_java_script_collection_callable_directory(
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
pub(super) fn encode_java_script_collection_callable_directory(
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
pub(super) fn decode_java_script_collection_constructor_owner(
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
pub(super) fn encode_java_script_collection_constructor_owner(
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
pub(super) fn decode_java_script_collection_method_owner(
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
pub(super) fn encode_java_script_collection_method_owner(
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
pub(super) fn decode_java_script_primitive_method_owner(
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
pub(super) fn encode_java_script_primitive_method_owner(
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
pub(super) fn decode_java_script_array_species_directory(
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
pub(super) fn encode_java_script_array_species_directory(
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
pub(super) fn decode_java_script_array_constructor_fact(
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
pub(super) fn encode_java_script_array_constructor_fact(
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
pub(super) fn decode_java_script_constructor_realm(
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
pub(super) fn encode_java_script_constructor_realm(
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
pub(super) fn decode_java_script_primitive_wrapper_roots(
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
pub(super) fn encode_java_script_primitive_wrapper_roots(
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
pub(super) fn decode_java_script_collection_iterator_method_owner(
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
pub(super) fn encode_java_script_collection_iterator_method_owner(
    value: &native::JavaScriptCollectionIteratorMethodOwner,
) -> Data {
    match value {
        native::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"collectionIteratorMethodOwner","identity":Data::String(wire_field_0.to_string()),"realm":Data::String(wire_field_1.to_string()),"operation":wire_text(&wire_field_2.0),"iteratorPrototype":encode_java_script_semantic_value(wire_field_3),"arrayPrototype":encode_java_script_semantic_value(wire_field_4),"objectPrototype":encode_java_script_semantic_value(wire_field_5)})
        }
    }
}
pub(super) fn decode_java_script_collection_call_compilation(
    data: &Data,
) -> native::JavaScriptCollectionCallCompilation {
    match data["$"].as_str().expect("model constructor") {
        "collectionCallCompilation" => {
            native::JavaScriptCollectionCallCompilation::CollectionCallCompilation(
                Box::new(decode_java_script_call_program(&data["program"])),
                native::ml_string::Text::from_units(wire_text_units(&data["error"])),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_collection_call_compilation(
    value: &native::JavaScriptCollectionCallCompilation,
) -> Data {
    match value {
        native::JavaScriptCollectionCallCompilation::CollectionCallCompilation(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"collectionCallCompilation","program":encode_java_script_call_program(wire_field_0),"error":wire_text(&wire_field_1.0)})
        }
    }
}
pub(super) fn decode_java_script_call_program(data: &Data) -> native::JavaScriptCallProgram {
    match data["$"].as_str().expect("model constructor") {
        "callProgram" => native::JavaScriptCallProgram::CallProgram(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["functions"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_function)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["realms"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| {
                        native::ml::Big::parse(wire_value.as_str().expect("model exact integer"))
                    })
                    .collect(),
            )),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
            data.get("templates").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_call_template)
                            .collect(),
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
            data.get("sources").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_call_source_unit)
                            .collect(),
                    )))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_call_program(value: &native::JavaScriptCallProgram) -> Data {
    match value {
        native::JavaScriptCallProgram::CallProgram(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"callProgram","functions":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_function).collect()) },"realms":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(|wire_value|Data::String(wire_value.to_string())).collect()) },"exoticKey":encode_java_script_semantic_value(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "templates".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_call_template)
                                    .collect(),
                            )
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
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
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "sources".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_call_source_unit)
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
pub(super) fn decode_java_script_call_function(data: &Data) -> native::JavaScriptCallFunction {
    match data["$"].as_str().expect("model constructor") {
        "callFunction" => native::JavaScriptCallFunction::CallFunction(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["receiverMode"])),
            Box::new(decode_java_script_semantic_value(&data["lexicalReceiver"])),
            data["parameters"].as_f64().expect("model Number"),
            data["registers"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["captures"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["instructions"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_instruction)
                    .collect(),
            )),
            data.get("lexicalNewTarget").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
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
pub(super) fn encode_java_script_call_function(value: &native::JavaScriptCallFunction) -> Data {
    match value {
        native::JavaScriptCallFunction::CallFunction(
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
            let mut result = json!({"$":"callFunction","identity":Data::String(wire_field_0.to_string()),"realm":Data::String(wire_field_1.to_string()),"receiverMode":wire_text(&wire_field_2.0),"lexicalReceiver":encode_java_script_semantic_value(wire_field_3),"parameters":wire_number(*wire_field_4),"registers":wire_number(*wire_field_5),"captures":{ let wire_reference = wire_field_6.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"instructions":{ let wire_reference = wire_field_7.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_instruction).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "lexicalNewTarget".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_9 {
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
pub(super) fn decode_java_script_call_instruction(
    data: &Data,
) -> native::JavaScriptCallInstruction {
    match data["$"].as_str().expect("model constructor") {
        "constant" => native::JavaScriptCallInstruction::Constant(
            data["destination"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        "copy" => native::JavaScriptCallInstruction::Copy(
            data["destination"].as_f64().expect("model Number"),
            data["source"].as_f64().expect("model Number"),
        ),
        "argument" => native::JavaScriptCallInstruction::Argument(
            data["destination"].as_f64().expect("model Number"),
            data["index"].as_f64().expect("model Number"),
        ),
        "argumentCount" => native::JavaScriptCallInstruction::ArgumentCount(
            data["destination"].as_f64().expect("model Number"),
        ),
        "argumentAt" => native::JavaScriptCallInstruction::ArgumentAt(
            data["destination"].as_f64().expect("model Number"),
            data["index"].as_f64().expect("model Number"),
        ),
        "functionBind" => native::JavaScriptCallInstruction::FunctionBind(
            data["destination"].as_f64().expect("model Number"),
        ),
        "boundFunctionCall" => native::JavaScriptCallInstruction::BoundFunctionCall(
            data["destination"].as_f64().expect("model Number"),
        ),
        "functionToString" => native::JavaScriptCallInstruction::FunctionToString(
            data["destination"].as_f64().expect("model Number"),
        ),
        "pushFinally" => native::JavaScriptCallInstruction::PushFinally(
            data["target"].as_f64().expect("model Number"),
        ),
        "finishFinally" => native::JavaScriptCallInstruction::FinishFinally,
        "completeJump" => native::JavaScriptCallInstruction::CompleteJump(
            data["target"].as_f64().expect("model Number"),
            data["depth"].as_f64().expect("model Number"),
        ),
        "jump" => {
            native::JavaScriptCallInstruction::Jump(data["target"].as_f64().expect("model Number"))
        }
        "branch" => native::JavaScriptCallInstruction::Branch(
            data["condition"].as_f64().expect("model Number"),
            data["yes"].as_f64().expect("model Number"),
            data["no"].as_f64().expect("model Number"),
        ),
        "call" => native::JavaScriptCallInstruction::Call(
            data["destination"].as_f64().expect("model Number"),
            data["callee"].as_f64().expect("model Number"),
            data["receiver"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
        ),
        "argumentListOpen" => native::JavaScriptCallInstruction::ArgumentListOpen(
            data["slot"].as_f64().expect("model Number"),
        ),
        "argumentListAppend" => native::JavaScriptCallInstruction::ArgumentListAppend(
            data["slot"].as_f64().expect("model Number"),
            data["value"].as_f64().expect("model Number"),
        ),
        "argumentListCall" => native::JavaScriptCallInstruction::ArgumentListCall(
            data["destination"].as_f64().expect("model Number"),
            data["slot"].as_f64().expect("model Number"),
            data["callee"].as_f64().expect("model Number"),
            data["receiver"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["mode"])),
        ),
        "construct" => native::JavaScriptCallInstruction::Construct(
            data["destination"].as_f64().expect("model Number"),
            data["callee"].as_f64().expect("model Number"),
            data["newTarget"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
        ),
        "constructorCheck" => native::JavaScriptCallInstruction::ConstructorCheck(
            data["destination"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
            data["required"].as_bool().expect("model Boolean"),
        ),
        "readNewTarget" => native::JavaScriptCallInstruction::ReadNewTarget(
            data["destination"].as_f64().expect("model Number"),
        ),
        "arrayJoinEnter" => native::JavaScriptCallInstruction::ArrayJoinEnter(
            data["destination"].as_f64().expect("model Number"),
            data["receiver"].as_f64().expect("model Number"),
        ),
        "arrayJoinLeave" => native::JavaScriptCallInstruction::ArrayJoinLeave(
            data["receiver"].as_f64().expect("model Number"),
        ),
        "constructErrorClass" => native::JavaScriptCallInstruction::ConstructErrorClass(
            data["destination"].as_f64().expect("model Number"),
            data["callee"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
        ),
        "callClassConstructor" => native::JavaScriptCallInstruction::CallClassConstructor(
            data["destination"].as_f64().expect("model Number"),
            data["callee"].as_f64().expect("model Number"),
            data["receiver"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
        ),
        "constructCollection" => native::JavaScriptCallInstruction::ConstructCollection(
            data["destination"].as_f64().expect("model Number"),
            data["callee"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
        ),
        "collectionAllocate" => native::JavaScriptCallInstruction::CollectionAllocate(
            data["destination"].as_f64().expect("model Number"),
            data["prototype"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
        ),
        "collectionIteratorAllocate" => {
            native::JavaScriptCallInstruction::CollectionIteratorAllocate(
                data["destination"].as_f64().expect("model Number"),
                data["prototype"].as_f64().expect("model Number"),
                native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
                native::ml_string::Text::from_units(wire_text_units(&data["iterationKind"])),
                data.get("input").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(wire_value.as_f64().expect("model Number"))
                    },)
                }),
            )
        }
        "argumentRest" => native::JavaScriptCallInstruction::ArgumentRest(
            data["destination"].as_f64().expect("model Number"),
            data["prototype"].as_f64().expect("model Number"),
            data["from"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "iteratorProtocol" => native::JavaScriptCallInstruction::IteratorProtocol(
            data["destination"].as_f64().expect("model Number"),
            data["done"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
            data["iteratorKey"].as_f64().expect("model Number"),
            data["slot"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            data.get("readValue").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
        ),
        "collectionIteratorResult" => native::JavaScriptCallInstruction::CollectionIteratorResult(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
            data["arrayPrototype"].as_f64().expect("model Number"),
            data["objectPrototype"].as_f64().expect("model Number"),
        ),
        "collectionOperation" => native::JavaScriptCallInstruction::CollectionOperation(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
        ),
        "primitiveMethod" => native::JavaScriptCallInstruction::PrimitiveMethod(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
        ),
        "collectionForEach" => native::JavaScriptCallInstruction::CollectionForEach(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
        ),
        "arrayConcatenation" => native::JavaScriptCallInstruction::ArrayConcatenation(
            data["destination"].as_f64().expect("model Number"),
            data.get("input").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_f64().expect("model Number"))
                },)
            }),
            data["spreadableKey"].as_f64().expect("model Number"),
            Box::new(decode_java_script_array_species_directory(&data["species"])),
        ),
        "arraySlice" => native::JavaScriptCallInstruction::ArraySlice(
            data["destination"].as_f64().expect("model Number"),
            data.get("input").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_f64().expect("model Number"))
                },)
            }),
            Box::new(decode_java_script_array_species_directory(&data["species"])),
        ),
        "arrayCreation" => native::JavaScriptCallInstruction::ArrayCreation(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            data["prototype"].as_f64().expect("model Number"),
            data["iteratorKey"].as_f64().expect("model Number"),
        ),
        "arrayCallback" => native::JavaScriptCallInstruction::ArrayCallback(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            data.get("input").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_f64().expect("model Number"))
                },)
            }),
        ),
        "arraySearch" => native::JavaScriptCallInstruction::ArraySearch(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            data.get("input").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_f64().expect("model Number"))
                },)
            }),
        ),
        "arrayTransform" => native::JavaScriptCallInstruction::ArrayTransform(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            Box::new(decode_java_script_array_species_directory(&data["species"])),
            data.get("input").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_f64().expect("model Number"))
                },)
            }),
        ),
        "objectEnumeration" => native::JavaScriptCallInstruction::ObjectEnumeration(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            data["prototype"].as_f64().expect("model Number"),
        ),
        "propertyDefinitions" => native::JavaScriptCallInstruction::PropertyDefinitions(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            data["target"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
        ),
        "propertyReflection" => native::JavaScriptCallInstruction::PropertyReflection(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            data["input"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            data["objectPrototype"].as_f64().expect("model Number"),
            data["arrayPrototype"].as_f64().expect("model Number"),
        ),
        "instanceOf" => native::JavaScriptCallInstruction::InstanceOf(
            data["destination"].as_f64().expect("model Number"),
            data["left"].as_f64().expect("model Number"),
            data["right"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
        ),
        "objectObservation" => native::JavaScriptCallInstruction::ObjectObservation(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            data["input"].as_f64().expect("model Number"),
            data["other"].as_f64().expect("model Number"),
        ),
        "collectionCopy" => native::JavaScriptCallInstruction::CollectionCopy(
            data["destination"].as_f64().expect("model Number"),
            data["target"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
            data["adder"].as_f64().expect("model Number"),
            data["iteratorKey"].as_f64().expect("model Number"),
            data["iteratorMethod"].as_f64().expect("model Number"),
            data["iteratorPrototype"].as_f64().expect("model Number"),
            data["nextMethod"].as_f64().expect("model Number"),
        ),
        "constructErrorValue" => native::JavaScriptCallInstruction::ConstructErrorValue(
            data["destination"].as_f64().expect("model Number"),
            data["constructor"].as_f64().expect("model Number"),
            data["newTarget"].as_f64().expect("model Number"),
            data["prototype"].as_f64().expect("model Number"),
            data["message"].as_f64().expect("model Number"),
            data["present"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
            data["bindReceiver"].as_bool().expect("model Boolean"),
        ),
        "readDerivedReceiver" => native::JavaScriptCallInstruction::ReadDerivedReceiver(
            data["destination"].as_f64().expect("model Number"),
        ),
        "invokeErrorIntrinsic" => native::JavaScriptCallInstruction::InvokeErrorIntrinsic(
            data["destination"].as_f64().expect("model Number"),
        ),
        "invokeHostCapability" => native::JavaScriptCallInstruction::InvokeHostCapability(
            data["destination"].as_f64().expect("model Number"),
        ),
        "return" => native::JavaScriptCallInstruction::Return(
            data["source"].as_f64().expect("model Number"),
        ),
        "throw" => {
            native::JavaScriptCallInstruction::Throw(data["source"].as_f64().expect("model Number"))
        }
        "pushHandler" => native::JavaScriptCallInstruction::PushHandler(
            data["destination"].as_f64().expect("model Number"),
            data["target"].as_f64().expect("model Number"),
        ),
        "popHandler" => native::JavaScriptCallInstruction::PopHandler,
        "numberBinary" => native::JavaScriptCallInstruction::NumberBinary(
            data["destination"].as_f64().expect("model Number"),
            data["left"].as_f64().expect("model Number"),
            data["right"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operator"])),
        ),
        "sameValue" => native::JavaScriptCallInstruction::SameValue(
            data["destination"].as_f64().expect("model Number"),
            data["left"].as_f64().expect("model Number"),
            data["right"].as_f64().expect("model Number"),
        ),
        "strictEqual" => native::JavaScriptCallInstruction::StrictEqual(
            data["destination"].as_f64().expect("model Number"),
            data["left"].as_f64().expect("model Number"),
            data["right"].as_f64().expect("model Number"),
        ),
        "requireWrite" => native::JavaScriptCallInstruction::RequireWrite(
            data["success"].as_f64().expect("model Number"),
        ),
        "primitive" => native::JavaScriptCallInstruction::Primitive(
            data["destination"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
        ),
        "ordinaryAllocate" => native::JavaScriptCallInstruction::OrdinaryAllocate(
            data["destination"].as_f64().expect("model Number"),
            data["prototype"].as_f64().expect("model Number"),
        ),
        "literalPrototype" => native::JavaScriptCallInstruction::LiteralPrototype(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["value"].as_f64().expect("model Number"),
        ),
        "literalAccessor" => native::JavaScriptCallInstruction::LiteralAccessor(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            data["value"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "objectFromEntries" => native::JavaScriptCallInstruction::ObjectFromEntries(
            data["destination"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
            data["prototype"].as_f64().expect("model Number"),
            data["iteratorKey"].as_f64().expect("model Number"),
        ),
        "primitiveWrapper" => native::JavaScriptCallInstruction::PrimitiveWrapper(
            data["destination"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
            data["prototype"].as_f64().expect("model Number"),
        ),
        "symbolAllocate" => native::JavaScriptCallInstruction::SymbolAllocate(
            data["destination"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
        ),
        "symbolDescription" => native::JavaScriptCallInstruction::SymbolDescription(
            data["destination"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
        ),
        "symbolRegistry" => native::JavaScriptCallInstruction::SymbolRegistry(
            data["destination"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
        ),
        "functionName" => native::JavaScriptCallInstruction::FunctionName(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["prefix"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "arrayAllocate" => native::JavaScriptCallInstruction::ArrayAllocate(
            data["destination"].as_f64().expect("model Number"),
            data["prototype"].as_f64().expect("model Number"),
            data["length"].as_f64().expect("model Number"),
        ),
        "closureAllocate" => native::JavaScriptCallInstruction::ClosureAllocate(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["owner"].as_str().expect("model exact integer")),
            data["ordinal"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["captures"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
            data["receiver"].as_f64().expect("model Number"),
        ),
        "arraySpread" => native::JavaScriptCallInstruction::ArraySpread(
            data["destination"].as_f64().expect("model Number"),
            data["target"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
            data["index"].as_f64().expect("model Number"),
            data["iteratorKey"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "objectSpread" => native::JavaScriptCallInstruction::ObjectSpread(
            data["destination"].as_f64().expect("model Number"),
            data["target"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            data.get("excluded").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(|wire_value| wire_value.as_f64().expect("model Number"))
                            .collect(),
                    )))
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
        ),
        "ordinaryDataProperty" => native::JavaScriptCallInstruction::OrdinaryDataProperty(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            data["value"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "ordinaryGet" => native::JavaScriptCallInstruction::OrdinaryGet(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            data["receiver"].as_f64().expect("model Number"),
        ),
        "ordinaryDelete" => native::JavaScriptCallInstruction::OrdinaryDelete(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
        ),
        "ordinaryHas" => native::JavaScriptCallInstruction::OrdinaryHas(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
        ),
        "ordinarySet" => native::JavaScriptCallInstruction::OrdinarySet(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            data["value"].as_f64().expect("model Number"),
            data["receiver"].as_f64().expect("model Number"),
        ),
        "ordinaryDefine" => native::JavaScriptCallInstruction::OrdinaryDefine(
            data["destination"].as_f64().expect("model Number"),
            data["target"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["fields"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "propertyKey" => native::JavaScriptCallInstruction::PropertyKey(
            data["destination"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "propertyDefinition" => native::JavaScriptCallInstruction::PropertyDefinition(
            data["destination"].as_f64().expect("model Number"),
            data["target"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            data["attributes"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            data.get("operation").map(|wire_value| {
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
pub(super) fn encode_java_script_call_instruction(
    value: &native::JavaScriptCallInstruction,
) -> Data {
    match value {
        native::JavaScriptCallInstruction::Constant(wire_field_0, wire_field_1) => {
            json!({"$":"constant","destination":wire_number(*wire_field_0),"value":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptCallInstruction::Copy(wire_field_0, wire_field_1) => {
            json!({"$":"copy","destination":wire_number(*wire_field_0),"source":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallInstruction::Argument(wire_field_0, wire_field_1) => {
            json!({"$":"argument","destination":wire_number(*wire_field_0),"index":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallInstruction::ArgumentCount(wire_field_0) => {
            json!({"$":"argumentCount","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::ArgumentAt(wire_field_0, wire_field_1) => {
            json!({"$":"argumentAt","destination":wire_number(*wire_field_0),"index":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallInstruction::FunctionBind(wire_field_0) => {
            json!({"$":"functionBind","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::BoundFunctionCall(wire_field_0) => {
            json!({"$":"boundFunctionCall","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::FunctionToString(wire_field_0) => {
            json!({"$":"functionToString","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::PushFinally(wire_field_0) => {
            json!({"$":"pushFinally","target":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::FinishFinally => json!({"$":"finishFinally"}),
        native::JavaScriptCallInstruction::CompleteJump(wire_field_0, wire_field_1) => {
            json!({"$":"completeJump","target":wire_number(*wire_field_0),"depth":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallInstruction::Jump(wire_field_0) => {
            json!({"$":"jump","target":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::Branch(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"branch","condition":wire_number(*wire_field_0),"yes":wire_number(*wire_field_1),"no":wire_number(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::Call(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"call","destination":wire_number(*wire_field_0),"callee":wire_number(*wire_field_1),"receiver":wire_number(*wire_field_2),"arguments":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) }})
        }
        native::JavaScriptCallInstruction::ArgumentListOpen(wire_field_0) => {
            json!({"$":"argumentListOpen","slot":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::ArgumentListAppend(wire_field_0, wire_field_1) => {
            json!({"$":"argumentListAppend","slot":wire_number(*wire_field_0),"value":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallInstruction::ArgumentListCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"argumentListCall","destination":wire_number(*wire_field_0),"slot":wire_number(*wire_field_1),"callee":wire_number(*wire_field_2),"receiver":wire_number(*wire_field_3),"mode":wire_text(&wire_field_4.0)})
        }
        native::JavaScriptCallInstruction::Construct(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"construct","destination":wire_number(*wire_field_0),"callee":wire_number(*wire_field_1),"newTarget":wire_number(*wire_field_2),"arguments":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) }})
        }
        native::JavaScriptCallInstruction::ConstructorCheck(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"constructorCheck","destination":wire_number(*wire_field_0),"input":wire_number(*wire_field_1),"required":Data::Bool(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::ReadNewTarget(wire_field_0) => {
            json!({"$":"readNewTarget","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::ArrayJoinEnter(wire_field_0, wire_field_1) => {
            json!({"$":"arrayJoinEnter","destination":wire_number(*wire_field_0),"receiver":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallInstruction::ArrayJoinLeave(wire_field_0) => {
            json!({"$":"arrayJoinLeave","receiver":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::ConstructErrorClass(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"constructErrorClass","destination":wire_number(*wire_field_0),"callee":wire_number(*wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) }})
        }
        native::JavaScriptCallInstruction::CallClassConstructor(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"callClassConstructor","destination":wire_number(*wire_field_0),"callee":wire_number(*wire_field_1),"receiver":wire_number(*wire_field_2),"arguments":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) }})
        }
        native::JavaScriptCallInstruction::ConstructCollection(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"constructCollection","destination":wire_number(*wire_field_0),"callee":wire_number(*wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) }})
        }
        native::JavaScriptCallInstruction::CollectionAllocate(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"collectionAllocate","destination":wire_number(*wire_field_0),"prototype":wire_number(*wire_field_1),"kind":wire_text(&wire_field_2.0)})
        }
        native::JavaScriptCallInstruction::CollectionIteratorAllocate(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            let mut result = json!({"$":"collectionIteratorAllocate","destination":wire_number(*wire_field_0),"prototype":wire_number(*wire_field_1),"kind":wire_text(&wire_field_2.0),"iterationKind":wire_text(&wire_field_3.0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "input".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallInstruction::ArgumentRest(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"argumentRest","destination":wire_number(*wire_field_0),"prototype":wire_number(*wire_field_1),"from":wire_number(*wire_field_2),"realm":Data::String(wire_field_3.to_string())})
        }
        native::JavaScriptCallInstruction::IteratorProtocol(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
        ) => {
            let mut result = json!({"$":"iteratorProtocol","destination":wire_number(*wire_field_0),"done":wire_number(*wire_field_1),"input":wire_number(*wire_field_2),"iteratorKey":wire_number(*wire_field_3),"slot":wire_number(*wire_field_4),"operation":wire_text(&wire_field_5.0),"realm":Data::String(wire_field_6.to_string())});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_7 {
                fields.insert(
                    "readValue".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallInstruction::CollectionIteratorResult(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"collectionIteratorResult","destination":wire_number(*wire_field_0),"kind":wire_text(&wire_field_1.0),"arrayPrototype":wire_number(*wire_field_2),"objectPrototype":wire_number(*wire_field_3)})
        }
        native::JavaScriptCallInstruction::CollectionOperation(wire_field_0, wire_field_1) => {
            json!({"$":"collectionOperation","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0)})
        }
        native::JavaScriptCallInstruction::PrimitiveMethod(wire_field_0, wire_field_1) => {
            json!({"$":"primitiveMethod","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0)})
        }
        native::JavaScriptCallInstruction::CollectionForEach(wire_field_0, wire_field_1) => {
            json!({"$":"collectionForEach","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0)})
        }
        native::JavaScriptCallInstruction::ArrayConcatenation(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"arrayConcatenation","destination":wire_number(*wire_field_0),"spreadableKey":wire_number(*wire_field_2),"species":encode_java_script_array_species_directory(wire_field_3)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_1 {
                fields.insert(
                    "input".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallInstruction::ArraySlice(wire_field_0, wire_field_1, wire_field_2) => {
            let mut result = json!({"$":"arraySlice","destination":wire_number(*wire_field_0),"species":encode_java_script_array_species_directory(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_1 {
                fields.insert(
                    "input".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallInstruction::ArrayCreation(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"arrayCreation","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0),"prototype":wire_number(*wire_field_2),"iteratorKey":wire_number(*wire_field_3)})
        }
        native::JavaScriptCallInstruction::ArrayCallback(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"arrayCallback","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "input".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallInstruction::ArraySearch(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"arraySearch","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "input".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallInstruction::ArrayTransform(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"arrayTransform","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0),"species":encode_java_script_array_species_directory(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "input".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallInstruction::ObjectEnumeration(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"objectEnumeration","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0),"prototype":wire_number(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::PropertyDefinitions(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"propertyDefinitions","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0),"target":wire_number(*wire_field_2),"input":wire_number(*wire_field_3)})
        }
        native::JavaScriptCallInstruction::PropertyReflection(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"propertyReflection","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0),"input":wire_number(*wire_field_2),"key":wire_number(*wire_field_3),"objectPrototype":wire_number(*wire_field_4),"arrayPrototype":wire_number(*wire_field_5)})
        }
        native::JavaScriptCallInstruction::InstanceOf(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"instanceOf","destination":wire_number(*wire_field_0),"left":wire_number(*wire_field_1),"right":wire_number(*wire_field_2),"key":wire_number(*wire_field_3),"operation":wire_text(&wire_field_4.0)})
        }
        native::JavaScriptCallInstruction::ObjectObservation(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"objectObservation","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0),"input":wire_number(*wire_field_2),"other":wire_number(*wire_field_3)})
        }
        native::JavaScriptCallInstruction::CollectionCopy(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
        ) => {
            json!({"$":"collectionCopy","destination":wire_number(*wire_field_0),"target":wire_number(*wire_field_1),"input":wire_number(*wire_field_2),"adder":wire_number(*wire_field_3),"iteratorKey":wire_number(*wire_field_4),"iteratorMethod":wire_number(*wire_field_5),"iteratorPrototype":wire_number(*wire_field_6),"nextMethod":wire_number(*wire_field_7)})
        }
        native::JavaScriptCallInstruction::ConstructErrorValue(
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
            json!({"$":"constructErrorValue","destination":wire_number(*wire_field_0),"constructor":wire_number(*wire_field_1),"newTarget":wire_number(*wire_field_2),"prototype":wire_number(*wire_field_3),"message":wire_number(*wire_field_4),"present":wire_number(*wire_field_5),"realm":Data::String(wire_field_6.to_string()),"errorClass":wire_text(&wire_field_7.0),"bindReceiver":Data::Bool(*wire_field_8)})
        }
        native::JavaScriptCallInstruction::ReadDerivedReceiver(wire_field_0) => {
            json!({"$":"readDerivedReceiver","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::InvokeErrorIntrinsic(wire_field_0) => {
            json!({"$":"invokeErrorIntrinsic","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::InvokeHostCapability(wire_field_0) => {
            json!({"$":"invokeHostCapability","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::Return(wire_field_0) => {
            json!({"$":"return","source":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::Throw(wire_field_0) => {
            json!({"$":"throw","source":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::PushHandler(wire_field_0, wire_field_1) => {
            json!({"$":"pushHandler","destination":wire_number(*wire_field_0),"target":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallInstruction::PopHandler => json!({"$":"popHandler"}),
        native::JavaScriptCallInstruction::NumberBinary(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"numberBinary","destination":wire_number(*wire_field_0),"left":wire_number(*wire_field_1),"right":wire_number(*wire_field_2),"operator":wire_text(&wire_field_3.0)})
        }
        native::JavaScriptCallInstruction::SameValue(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sameValue","destination":wire_number(*wire_field_0),"left":wire_number(*wire_field_1),"right":wire_number(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::StrictEqual(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"strictEqual","destination":wire_number(*wire_field_0),"left":wire_number(*wire_field_1),"right":wire_number(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::RequireWrite(wire_field_0) => {
            json!({"$":"requireWrite","success":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallInstruction::Primitive(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"primitive","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) }})
        }
        native::JavaScriptCallInstruction::OrdinaryAllocate(wire_field_0, wire_field_1) => {
            json!({"$":"ordinaryAllocate","destination":wire_number(*wire_field_0),"prototype":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallInstruction::LiteralPrototype(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"literalPrototype","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"value":wire_number(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::LiteralAccessor(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"literalAccessor","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"value":wire_number(*wire_field_3),"kind":wire_text(&wire_field_4.0),"realm":Data::String(wire_field_5.to_string())})
        }
        native::JavaScriptCallInstruction::ObjectFromEntries(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"objectFromEntries","destination":wire_number(*wire_field_0),"input":wire_number(*wire_field_1),"prototype":wire_number(*wire_field_2),"iteratorKey":wire_number(*wire_field_3)})
        }
        native::JavaScriptCallInstruction::PrimitiveWrapper(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"primitiveWrapper","destination":wire_number(*wire_field_0),"input":wire_number(*wire_field_1),"prototype":wire_number(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::SymbolAllocate(wire_field_0, wire_field_1) => {
            json!({"$":"symbolAllocate","destination":wire_number(*wire_field_0),"input":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallInstruction::SymbolDescription(wire_field_0, wire_field_1) => {
            json!({"$":"symbolDescription","destination":wire_number(*wire_field_0),"input":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallInstruction::SymbolRegistry(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"symbolRegistry","destination":wire_number(*wire_field_0),"input":wire_number(*wire_field_1),"operation":wire_text(&wire_field_2.0)})
        }
        native::JavaScriptCallInstruction::FunctionName(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"functionName","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"prefix":wire_text(&wire_field_3.0),"realm":Data::String(wire_field_4.to_string())})
        }
        native::JavaScriptCallInstruction::ArrayAllocate(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arrayAllocate","destination":wire_number(*wire_field_0),"prototype":wire_number(*wire_field_1),"length":wire_number(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::ClosureAllocate(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"closureAllocate","destination":wire_number(*wire_field_0),"owner":Data::String(wire_field_1.to_string()),"ordinal":wire_number(*wire_field_2),"captures":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) },"receiver":wire_number(*wire_field_4)})
        }
        native::JavaScriptCallInstruction::ArraySpread(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"arraySpread","destination":wire_number(*wire_field_0),"target":wire_number(*wire_field_1),"input":wire_number(*wire_field_2),"index":wire_number(*wire_field_3),"iteratorKey":wire_number(*wire_field_4),"realm":Data::String(wire_field_5.to_string())})
        }
        native::JavaScriptCallInstruction::ObjectSpread(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"objectSpread","destination":wire_number(*wire_field_0),"target":wire_number(*wire_field_1),"input":wire_number(*wire_field_2),"realm":Data::String(wire_field_3.to_string())});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "excluded".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(|wire_value| wire_number(*wire_value))
                                    .collect(),
                            )
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "operation".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            result
        }
        native::JavaScriptCallInstruction::OrdinaryDataProperty(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"ordinaryDataProperty","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"value":wire_number(*wire_field_3),"realm":Data::String(wire_field_4.to_string())})
        }
        native::JavaScriptCallInstruction::OrdinaryGet(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"ordinaryGet","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"receiver":wire_number(*wire_field_3)})
        }
        native::JavaScriptCallInstruction::OrdinaryDelete(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"ordinaryDelete","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"key":wire_number(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::OrdinaryHas(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"ordinaryHas","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"key":wire_number(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::OrdinarySet(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"ordinarySet","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"value":wire_number(*wire_field_3),"receiver":wire_number(*wire_field_4)})
        }
        native::JavaScriptCallInstruction::OrdinaryDefine(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"ordinaryDefine","destination":wire_number(*wire_field_0),"target":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"fields":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) },"realm":Data::String(wire_field_4.to_string())})
        }
        native::JavaScriptCallInstruction::PropertyKey(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"propertyKey","destination":wire_number(*wire_field_0),"input":wire_number(*wire_field_1),"realm":Data::String(wire_field_2.to_string())})
        }
        native::JavaScriptCallInstruction::PropertyDefinition(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"propertyDefinition","destination":wire_number(*wire_field_0),"target":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"attributes":wire_number(*wire_field_3),"realm":Data::String(wire_field_4.to_string())});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "operation".into(),
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
pub(super) fn decode_java_script_call_source_span(data: &Data) -> native::JavaScriptCallSourceSpan {
    match data["$"].as_str().expect("model constructor") {
        "callSourceSpan" => native::JavaScriptCallSourceSpan::CallSourceSpan(
            native::ml_string::Text::from_units(wire_text_units(&data["unit"])),
            data["start"].as_f64().expect("model Number"),
            data["end"].as_f64().expect("model Number"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_call_source_span(
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
pub(super) fn decode_java_script_call_template(data: &Data) -> native::JavaScriptCallTemplate {
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
pub(super) fn encode_java_script_call_template(value: &native::JavaScriptCallTemplate) -> Data {
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
pub(super) fn decode_java_script_call_source_unit(data: &Data) -> native::JavaScriptCallSourceUnit {
    match data["$"].as_str().expect("model constructor") {
        "callSourceUnit" => native::JavaScriptCallSourceUnit::CallSourceUnit(
            native::ml_string::Text::from_units(wire_text_units(&data["identifier"])),
            Box::new(decode_java_script_semantic_value(&data["text"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_call_source_unit(
    value: &native::JavaScriptCallSourceUnit,
) -> Data {
    match value {
        native::JavaScriptCallSourceUnit::CallSourceUnit(wire_field_0, wire_field_1) => {
            json!({"$":"callSourceUnit","identifier":wire_text(&wire_field_0.0),"text":encode_java_script_semantic_value(wire_field_1)})
        }
    }
}
fn wire_undefined() -> Data {
    json!({"fieldPresence":"undefined"})
}
fn is_wire_undefined(value: &Data) -> bool {
    value.as_object().is_some_and(|fields| {
        fields.len() == 1 && fields.get("fieldPresence").and_then(Data::as_str) == Some("undefined")
    })
}
