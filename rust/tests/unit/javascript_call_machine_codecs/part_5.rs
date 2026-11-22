use super::{
    Data, decode_java_script_semantic_value, encode_java_script_array_assignment_state,
    encode_java_script_array_callback_state, encode_java_script_array_concatenation_state,
    encode_java_script_array_creation_fallback, encode_java_script_array_creation_state,
    encode_java_script_array_search_state, encode_java_script_array_slice_state,
    encode_java_script_array_transform_state, encode_java_script_collection_for_each_state,
    encode_java_script_construction_state, encode_java_script_error_allocation_directory,
    encode_java_script_function_bind_state, encode_java_script_instance_of_state,
    encode_java_script_number_call_state, encode_java_script_object_enumeration_state,
    encode_java_script_object_from_entries_error_request,
    encode_java_script_object_from_entries_state, encode_java_script_object_integrity_state,
    encode_java_script_object_spread_state, encode_java_script_primitive_wrapper_state,
    encode_java_script_property_definition_state, encode_java_script_property_definitions_state,
    encode_java_script_property_reflection_state, encode_java_script_semantic_value,
    encode_java_script_species_construction_state, encode_java_script_text_call_state,
    is_wire_undefined, native, wire_number, wire_text, wire_text_units, wire_undefined,
};

pub(in super::super) fn encode_java_script_call_pending(
    value: &native::JavaScriptCallPending,
) -> Data {
    match value {
        native::JavaScriptCallPending::CallIdle => json!({"$":"callIdle"}),
        native::JavaScriptCallPending::WaitCall(wire_field_0) => {
            json!({"$":"waitCall","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallPending::WaitGet(wire_field_0) => {
            json!({"$":"waitGet","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallPending::WaitSet(wire_field_0) => {
            json!({"$":"waitSet","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallPending::WaitHostSet(wire_field_0) => {
            json!({"$":"waitHostSet","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallPending::WaitArgumentRest(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitArgumentRest","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_argument_rest_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitIteratorProtocol(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"waitIteratorProtocol","destination":wire_number(*wire_field_0),"done":wire_number(*wire_field_1),"slot":wire_number(*wire_field_2),"operation":wire_text(&wire_field_3.0),"realm":Data::String(wire_field_4.to_string()),"state":encode_java_script_iterator_protocol_state(wire_field_5)})
        }
        native::JavaScriptCallPending::WaitArraySpread(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitArraySpread","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_array_spread_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitArrayIteratorNext(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitArrayIteratorNext","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_array_iterator_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitCollectionIteratorResult(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitCollectionIteratorResult","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_collection_iterator_result_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitArrayConcatenation(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"waitArrayConcatenation","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_array_concatenation_state(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "construction".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_species_construction_state(wire_present)
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptCallPending::WaitArraySlice(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"waitArraySlice","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_array_slice_state(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "construction".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_species_construction_state(wire_present)
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptCallPending::WaitArrayTransform(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"waitArrayTransform","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_array_transform_state(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "construction".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_species_construction_state(wire_present)
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptCallPending::WaitArraySearch(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitArraySearch","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_array_search_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitCollectionForEach(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitCollectionForEach","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_collection_for_each_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitArrayCallback(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitArrayCallback","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_array_callback_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitObjectSpread(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitObjectSpread","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_object_spread_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitObjectEnumeration(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitObjectEnumeration","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_object_enumeration_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitPropertyDefinitions(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitPropertyDefinitions","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_property_definitions_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitPropertyReflection(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitPropertyReflection","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_property_reflection_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitInstanceOf(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"waitInstanceOf","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_instance_of_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitObjectIntegrity(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitObjectIntegrity","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_object_integrity_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitObjectPrototype(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitObjectPrototype","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"input":encode_java_script_semantic_value(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitArrayAssignment(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitArrayAssignment","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_array_assignment_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitKey(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"waitKey","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_primitive_conversion_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitDefinition(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"waitDefinition","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_property_definition_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitNumber(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"waitNumber","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_number_call_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitText(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"waitText","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_text_call_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitArrayCreation(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"waitArrayCreation","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_array_creation_state(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "construction".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_construction_state(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "fallback".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_array_creation_fallback(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "error".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_object_from_entries_error_request(wire_present)
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptCallPending::WaitConstruction(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"waitConstruction","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_construction_state(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "heldArguments".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_semantic_value)
                                    .collect(),
                            )
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptCallPending::WaitFunctionBind(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitFunctionBind","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_function_bind_state(wire_field_2)})
        }
        native::JavaScriptCallPending::WaitFunctionText(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"waitFunctionText","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"input":encode_java_script_semantic_value(wire_field_2),"kind":wire_text(&wire_field_3.0)})
        }
        native::JavaScriptCallPending::WaitConstructorCheck(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"waitConstructorCheck","destination":wire_number(*wire_field_0),"input":encode_java_script_semantic_value(wire_field_1),"required":Data::Bool(*wire_field_2)})
        }
        native::JavaScriptCallPending::WaitObjectFromEntries(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"waitObjectFromEntries","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"state":encode_java_script_object_from_entries_state(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "error".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_object_from_entries_error_request(wire_present)
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptCallPending::WaitPrimitiveWrapper(wire_field_0, wire_field_1) => {
            json!({"$":"waitPrimitiveWrapper","destination":wire_number(*wire_field_0),"state":encode_java_script_primitive_wrapper_state(wire_field_1)})
        }
        native::JavaScriptCallPending::WaitSymbolAllocation(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"waitSymbolAllocation","destination":wire_number(*wire_field_0),"description":encode_java_script_semantic_value(wire_field_1)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "registryKey".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallPending::WaitSymbolDescription(wire_field_0, wire_field_1) => {
            json!({"$":"waitSymbolDescription","destination":wire_number(*wire_field_0),"symbol":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptCallPending::WaitFunctionNameSymbol(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"waitFunctionNameSymbol","destination":wire_number(*wire_field_0),"object":encode_java_script_semantic_value(wire_field_1),"key":encode_java_script_semantic_value(wire_field_2),"prefix":wire_text(&wire_field_3.0),"realm":Data::String(wire_field_4.to_string())})
        }
        native::JavaScriptCallPending::WaitSourceError(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"waitSourceError","realm":Data::String(wire_field_0.to_string()),"errorClass":wire_text(&wire_field_1.0),"message":encode_java_script_semantic_value(wire_field_2),"messagePresent":Data::Bool(*wire_field_3)})
        }
        native::JavaScriptCallPending::WaitErrorValue(
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
            json!({"$":"waitErrorValue","destination":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"errorClass":wire_text(&wire_field_2.0),"constructor":encode_java_script_semantic_value(wire_field_3),"newTarget":encode_java_script_semantic_value(wire_field_4),"prototype":encode_java_script_semantic_value(wire_field_5),"message":encode_java_script_semantic_value(wire_field_6),"messagePresent":Data::Bool(*wire_field_7),"bindReceiver":Data::Bool(*wire_field_8),"beforeDirectory":encode_java_script_error_allocation_directory(wire_field_9)})
        }
        native::JavaScriptCallPending::WaitErrorIntrinsic(wire_field_0) => {
            json!({"$":"waitErrorIntrinsic","destination":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallPending::WaitHostCapability(wire_field_0, wire_field_1) => {
            json!({"$":"waitHostCapability","destination":wire_number(*wire_field_0),"beforeDirectory":encode_java_script_error_allocation_directory(wire_field_1)})
        }
        native::JavaScriptCallPending::WaitError(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"waitError","realm":Data::String(wire_field_0.to_string()),"errorClass":wire_text(&wire_field_1.0),"reason":wire_text(&wire_field_2.0)})
        }
    }
}

pub(in super::super) fn decode_java_script_argument_rest_state(
    data: &Data,
) -> native::JavaScriptArgumentRestState {
    match data["$"].as_str().expect("model constructor") {
        "argumentRestState" => native::JavaScriptArgumentRestState::ArgumentRestState(
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["from"].as_f64().expect("model Number"),
            data["count"].as_f64().expect("model Number"),
            data["index"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["target"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_argument_rest_state(
    value: &native::JavaScriptArgumentRestState,
) -> Data {
    match value {
        native::JavaScriptArgumentRestState::ArgumentRestState(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"argumentRestState","prototype":encode_java_script_semantic_value(wire_field_0),"from":wire_number(*wire_field_1),"count":wire_number(*wire_field_2),"index":wire_number(*wire_field_3),"target":encode_java_script_semantic_value(wire_field_4)})
        }
    }
}

pub(in super::super) fn decode_java_script_iterator_protocol_state(
    data: &Data,
) -> native::JavaScriptIteratorProtocolState {
    match data["$"].as_str().expect("model constructor") {
        "iteratorProtocol" => native::JavaScriptIteratorProtocolState::IteratorProtocol(
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            Box::new(decode_java_script_semantic_value(&data["key"])),
            Box::new(decode_java_script_synchronous_iterator_record(
                &data["record"],
            )),
            Box::new(decode_java_script_semantic_value(&data["result"])),
            Box::new(decode_java_script_iterator_completion(&data["completion"])),
            data.get("readValue").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
            data.get("call").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_iterator_selected_call(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_iterator_protocol_state(
    value: &native::JavaScriptIteratorProtocolState,
) -> Data {
    match value {
        native::JavaScriptIteratorProtocolState::IteratorProtocol(
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
            let mut result = json!({"$":"iteratorProtocol","operation":wire_text(&wire_field_0.0),"phase":wire_text(&wire_field_1.0),"input":encode_java_script_semantic_value(wire_field_2),"key":encode_java_script_semantic_value(wire_field_3),"record":encode_java_script_synchronous_iterator_record(wire_field_4),"result":encode_java_script_semantic_value(wire_field_5),"completion":encode_java_script_iterator_completion(wire_field_6)});
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
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_iterator_selected_call(wire_present)
                        }
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_synchronous_iterator_record(
    data: &Data,
) -> native::JavaScriptSynchronousIteratorRecord {
    match data["$"].as_str().expect("model constructor") {
        "synchronousIteratorRecord" => {
            native::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
                Box::new(decode_java_script_semantic_value(&data["iterator"])),
                Box::new(decode_java_script_semantic_value(&data["nextMethod"])),
                data["done"].as_bool().expect("model Boolean"),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_synchronous_iterator_record(
    value: &native::JavaScriptSynchronousIteratorRecord,
) -> Data {
    match value {
        native::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"synchronousIteratorRecord","iterator":encode_java_script_semantic_value(wire_field_0),"nextMethod":encode_java_script_semantic_value(wire_field_1),"done":Data::Bool(*wire_field_2)})
        }
    }
}

pub(in super::super) fn decode_java_script_iterator_completion(
    data: &Data,
) -> native::JavaScriptIteratorCompletion {
    match data["$"].as_str().expect("model constructor") {
        "iteratorCompletion" => native::JavaScriptIteratorCompletion::IteratorCompletion(
            native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
            data["target"].as_f64().expect("model Number"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_iterator_completion(
    value: &native::JavaScriptIteratorCompletion,
) -> Data {
    match value {
        native::JavaScriptIteratorCompletion::IteratorCompletion(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"iteratorCompletion","kind":wire_text(&wire_field_0.0),"value":encode_java_script_semantic_value(wire_field_1),"target":wire_number(*wire_field_2)})
        }
    }
}

pub(in super::super) fn decode_java_script_iterator_selected_call(
    data: &Data,
) -> native::JavaScriptIteratorSelectedCall {
    match data["$"].as_str().expect("model constructor") {
        "iteratorSelectedCall" => native::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
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

pub(in super::super) fn encode_java_script_iterator_selected_call(
    value: &native::JavaScriptIteratorSelectedCall,
) -> Data {
    match value {
        native::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"iteratorSelectedCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_array_spread_state(
    data: &Data,
) -> native::JavaScriptArraySpreadState {
    match data["$"].as_str().expect("model constructor") {
        "arraySpreadState" => native::JavaScriptArraySpreadState::ArraySpreadState(
            Box::new(decode_java_script_semantic_value(&data["target"])),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            Box::new(decode_java_script_semantic_value(&data["iteratorKey"])),
            data["index"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
            data.get("protocol").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_iterator_protocol_state(wire_value))
                },)
            }),
            data.get("record").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_synchronous_iterator_record(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_spread_state(
    value: &native::JavaScriptArraySpreadState,
) -> Data {
    match value {
        native::JavaScriptArraySpreadState::ArraySpreadState(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
        ) => {
            let mut result = json!({"$":"arraySpreadState","target":encode_java_script_semantic_value(wire_field_0),"input":encode_java_script_semantic_value(wire_field_1),"iteratorKey":encode_java_script_semantic_value(wire_field_2),"index":wire_number(*wire_field_3),"phase":wire_text(&wire_field_4.0),"value":encode_java_script_semantic_value(wire_field_5)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_6 {
                fields.insert(
                    "protocol".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_iterator_protocol_state(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_7 {
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
            result
        }
    }
}

pub(in super::super) fn decode_java_script_array_iterator_state(
    data: &Data,
) -> native::JavaScriptArrayIteratorState {
    match data["$"].as_str().expect("model constructor") {
        "arrayIteratorState" => native::JavaScriptArrayIteratorState::ArrayIteratorState(
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            Box::new(decode_java_script_semantic_value(&data["source"])),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
            data["index"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
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
                    Some(decode_java_script_array_iterator_call(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_iterator_state(
    value: &native::JavaScriptArrayIteratorState,
) -> Data {
    match value {
        native::JavaScriptArrayIteratorState::ArrayIteratorState(
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
            let mut result = json!({"$":"arrayIteratorState","receiver":encode_java_script_semantic_value(wire_field_0),"source":encode_java_script_semantic_value(wire_field_1),"exoticKey":encode_java_script_semantic_value(wire_field_2),"index":wire_number(*wire_field_3),"kind":wire_text(&wire_field_4.0),"phase":wire_text(&wire_field_5.0),"input":encode_java_script_semantic_value(wire_field_6)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_7 {
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
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "call".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_array_iterator_call(wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_primitive_conversion_state(
    data: &Data,
) -> native::JavaScriptPrimitiveConversionState {
    match data["$"].as_str().expect("model constructor") {
        "primitiveConversion" => native::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            Box::new(decode_java_script_semantic_value(&data["receiver"])),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
            native::ml_string::Text::from_units(wire_text_units(&data["hint"])),
            native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_primitive_conversion_state(
    value: &native::JavaScriptPrimitiveConversionState,
) -> Data {
    match value {
        native::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"primitiveConversion","receiver":encode_java_script_semantic_value(wire_field_0),"exoticKey":encode_java_script_semantic_value(wire_field_1),"hint":wire_text(&wire_field_2.0),"phase":wire_text(&wire_field_3.0)})
        }
    }
}

pub(in super::super) fn decode_java_script_array_iterator_call(
    data: &Data,
) -> native::JavaScriptArrayIteratorCall {
    match data["$"].as_str().expect("model constructor") {
        "arrayIteratorCall" => native::JavaScriptArrayIteratorCall::ArrayIteratorCall(
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

pub(in super::super) fn encode_java_script_array_iterator_call(
    value: &native::JavaScriptArrayIteratorCall,
) -> Data {
    match value {
        native::JavaScriptArrayIteratorCall::ArrayIteratorCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arrayIteratorCall","callee":encode_java_script_semantic_value(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_collection_iterator_result_state(
    data: &Data,
) -> native::JavaScriptCollectionIteratorResultState {
    match data["$"].as_str().expect("model constructor") {
        "collectionIteratorResultState" => {
            native::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(
                native::ml_string::Text::from_units(wire_text_units(&data["phase"])),
                Box::new(decode_java_script_semantic_value(&data["receiver"])),
                native::ml_string::Text::from_units(wire_text_units(&data["collectionKind"])),
                Box::new(decode_java_script_semantic_value(&data["arrayPrototype"])),
                Box::new(decode_java_script_semantic_value(&data["objectPrototype"])),
                Box::new(decode_java_script_semantic_value(&data["key"])),
                Box::new(decode_java_script_semantic_value(&data["value"])),
                data["done"].as_bool().expect("model Boolean"),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_collection_iterator_result_state(
    value: &native::JavaScriptCollectionIteratorResultState,
) -> Data {
    match value {
        native::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
        ) => {
            json!({"$":"collectionIteratorResultState","phase":wire_text(&wire_field_0.0),"receiver":encode_java_script_semantic_value(wire_field_1),"collectionKind":wire_text(&wire_field_2.0),"arrayPrototype":encode_java_script_semantic_value(wire_field_3),"objectPrototype":encode_java_script_semantic_value(wire_field_4),"key":encode_java_script_semantic_value(wire_field_5),"value":encode_java_script_semantic_value(wire_field_6),"done":Data::Bool(*wire_field_7)})
        }
    }
}
