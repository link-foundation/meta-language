use super::{
    Data, decode_java_script_argument_rest_state, decode_java_script_array_assignment_state,
    decode_java_script_array_callback_state, decode_java_script_array_concatenation_state,
    decode_java_script_array_creation_fallback, decode_java_script_array_creation_state,
    decode_java_script_array_iterator_state, decode_java_script_array_search_state,
    decode_java_script_array_slice_state, decode_java_script_array_spread_state,
    decode_java_script_array_transform_state, decode_java_script_call_argument_list,
    decode_java_script_call_instruction, decode_java_script_call_iterator_region,
    decode_java_script_collection_for_each_state,
    decode_java_script_collection_iterator_result_state, decode_java_script_construction_state,
    decode_java_script_error_allocation_directory, decode_java_script_function_bind_state,
    decode_java_script_instance_of_state, decode_java_script_iterator_protocol_state,
    decode_java_script_number_call_state, decode_java_script_object_enumeration_state,
    decode_java_script_object_from_entries_error_request,
    decode_java_script_object_from_entries_state, decode_java_script_object_integrity_state,
    decode_java_script_object_spread_state, decode_java_script_ordinary_heap,
    decode_java_script_primitive_conversion_state, decode_java_script_primitive_wrapper_state,
    decode_java_script_property_definition_state, decode_java_script_property_definitions_state,
    decode_java_script_property_reflection_state, decode_java_script_semantic_value,
    decode_java_script_species_construction_state, decode_java_script_text_call_state,
    encode_java_script_call_argument_list, encode_java_script_call_instruction,
    encode_java_script_call_iterator_region, encode_java_script_call_pending,
    encode_java_script_ordinary_heap, encode_java_script_semantic_value, is_wire_undefined, native,
    wire_number, wire_text, wire_text_units, wire_undefined,
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

pub(in super::super) fn decode_java_script_call_machine(
    data: &Data,
) -> native::JavaScriptCallMachine {
    match data["$"].as_str().expect("model constructor") {
        "callMachine" => native::JavaScriptCallMachine::CallMachine(
            Box::new(decode_java_script_ordinary_heap(&data["heap"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["frames"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_frame)
                    .collect(),
            )),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_machine(
    value: &native::JavaScriptCallMachine,
) -> Data {
    match value {
        native::JavaScriptCallMachine::CallMachine(wire_field_0, wire_field_1) => {
            json!({"$":"callMachine","heap":encode_java_script_ordinary_heap(wire_field_0),"frames":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_frame).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_call_frame(data: &Data) -> native::JavaScriptCallFrame {
    match data["$"].as_str().expect("model constructor") {
        "callFrame" => native::JavaScriptCallFrame::CallFrame(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            data["position"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["registers"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["handlers"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_handler)
                    .collect(),
            )),
            Box::new(decode_java_script_call_pending(&data["pending"])),
            data.get("iterators").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_call_iterator_region)
                            .collect(),
                    )))
                },)
            }),
            data.get("argumentLists").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_call_argument_list)
                            .collect(),
                    )))
                },)
            }),
            data.get("activeJoin").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("newTarget").map(|wire_value| {
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

pub(in super::super) fn encode_java_script_call_frame(value: &native::JavaScriptCallFrame) -> Data {
    match value {
        native::JavaScriptCallFrame::CallFrame(
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
            let mut result = json!({"$":"callFrame","identity":Data::String(wire_field_0.to_string()),"position":wire_number(*wire_field_1),"registers":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"arguments":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"handlers":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_handler).collect()) },"pending":encode_java_script_call_pending(wire_field_5)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_6 {
                fields.insert(
                    "iterators".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_call_iterator_region)
                                    .collect(),
                            )
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_7 {
                fields.insert(
                    "argumentLists".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_call_argument_list)
                                    .collect(),
                            )
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "activeJoin".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_9 {
                fields.insert(
                    "newTarget".into(),
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

pub(in super::super) fn decode_java_script_call_handler(
    data: &Data,
) -> native::JavaScriptCallHandler {
    match data["$"].as_str().expect("model constructor") {
        "callHandler" => native::JavaScriptCallHandler::CallHandler(
            data["target"].as_f64().expect("model Number"),
            data["destination"].as_f64().expect("model Number"),
        ),
        "callFinallyHandler" => native::JavaScriptCallHandler::CallFinallyHandler(
            data["target"].as_f64().expect("model Number"),
        ),
        "callDeferredCompletion" => native::JavaScriptCallHandler::CallDeferredCompletion(
            Box::new(decode_java_script_call_completion(&data["completion"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_handler(
    value: &native::JavaScriptCallHandler,
) -> Data {
    match value {
        native::JavaScriptCallHandler::CallHandler(wire_field_0, wire_field_1) => {
            json!({"$":"callHandler","target":wire_number(*wire_field_0),"destination":wire_number(*wire_field_1)})
        }
        native::JavaScriptCallHandler::CallFinallyHandler(wire_field_0) => {
            json!({"$":"callFinallyHandler","target":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallHandler::CallDeferredCompletion(wire_field_0) => {
            json!({"$":"callDeferredCompletion","completion":encode_java_script_call_completion(wire_field_0)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_completion(
    data: &Data,
) -> native::JavaScriptCallCompletion {
    match data["$"].as_str().expect("model constructor") {
        "callCompletion" => native::JavaScriptCallCompletion::CallCompletion(
            native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
            data["target"].as_f64().expect("model Number"),
            data["depth"].as_f64().expect("model Number"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_completion(
    value: &native::JavaScriptCallCompletion,
) -> Data {
    match value {
        native::JavaScriptCallCompletion::CallCompletion(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"callCompletion","kind":wire_text(&wire_field_0.0),"value":encode_java_script_semantic_value(wire_field_1),"target":wire_number(*wire_field_2),"depth":wire_number(*wire_field_3)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_pending(
    data: &Data,
) -> native::JavaScriptCallPending {
    match data["$"].as_str().expect("model constructor") {
        "callIdle" => native::JavaScriptCallPending::CallIdle,
        "waitCall" => native::JavaScriptCallPending::WaitCall(
            data["destination"].as_f64().expect("model Number"),
        ),
        "waitGet" => native::JavaScriptCallPending::WaitGet(
            data["destination"].as_f64().expect("model Number"),
        ),
        "waitSet" => native::JavaScriptCallPending::WaitSet(
            data["destination"].as_f64().expect("model Number"),
        ),
        "waitHostSet" => native::JavaScriptCallPending::WaitHostSet(
            data["destination"].as_f64().expect("model Number"),
        ),
        "waitArgumentRest" => native::JavaScriptCallPending::WaitArgumentRest(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_argument_rest_state(&data["state"])),
        ),
        "waitIteratorProtocol" => native::JavaScriptCallPending::WaitIteratorProtocol(
            data["destination"].as_f64().expect("model Number"),
            data["done"].as_f64().expect("model Number"),
            data["slot"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_iterator_protocol_state(&data["state"])),
        ),
        "waitArraySpread" => native::JavaScriptCallPending::WaitArraySpread(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_array_spread_state(&data["state"])),
        ),
        "waitArrayIteratorNext" => native::JavaScriptCallPending::WaitArrayIteratorNext(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_array_iterator_state(&data["state"])),
        ),
        "waitCollectionIteratorResult" => {
            native::JavaScriptCallPending::WaitCollectionIteratorResult(
                data["destination"].as_f64().expect("model Number"),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
                Box::new(decode_java_script_collection_iterator_result_state(
                    &data["state"],
                )),
            )
        }
        "waitArrayConcatenation" => native::JavaScriptCallPending::WaitArrayConcatenation(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_array_concatenation_state(&data["state"])),
            data.get("construction").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_species_construction_state(wire_value))
                },)
            }),
        ),
        "waitArraySlice" => native::JavaScriptCallPending::WaitArraySlice(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_array_slice_state(&data["state"])),
            data.get("construction").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_species_construction_state(wire_value))
                },)
            }),
        ),
        "waitArrayTransform" => native::JavaScriptCallPending::WaitArrayTransform(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_array_transform_state(&data["state"])),
            data.get("construction").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_species_construction_state(wire_value))
                },)
            }),
        ),
        "waitArraySearch" => native::JavaScriptCallPending::WaitArraySearch(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_array_search_state(&data["state"])),
        ),
        "waitCollectionForEach" => native::JavaScriptCallPending::WaitCollectionForEach(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_collection_for_each_state(&data["state"])),
        ),
        "waitArrayCallback" => native::JavaScriptCallPending::WaitArrayCallback(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_array_callback_state(&data["state"])),
        ),
        "waitObjectSpread" => native::JavaScriptCallPending::WaitObjectSpread(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_object_spread_state(&data["state"])),
        ),
        "waitObjectEnumeration" => native::JavaScriptCallPending::WaitObjectEnumeration(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_object_enumeration_state(&data["state"])),
        ),
        "waitPropertyDefinitions" => native::JavaScriptCallPending::WaitPropertyDefinitions(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_property_definitions_state(
                &data["state"],
            )),
        ),
        "waitPropertyReflection" => native::JavaScriptCallPending::WaitPropertyReflection(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_property_reflection_state(&data["state"])),
        ),
        "waitInstanceOf" => native::JavaScriptCallPending::WaitInstanceOf(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_instance_of_state(&data["state"])),
        ),
        "waitObjectIntegrity" => native::JavaScriptCallPending::WaitObjectIntegrity(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_object_integrity_state(&data["state"])),
        ),
        "waitObjectPrototype" => native::JavaScriptCallPending::WaitObjectPrototype(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["input"])),
        ),
        "waitArrayAssignment" => native::JavaScriptCallPending::WaitArrayAssignment(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_array_assignment_state(&data["state"])),
        ),
        "waitKey" => native::JavaScriptCallPending::WaitKey(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_primitive_conversion_state(
                &data["state"],
            )),
        ),
        "waitDefinition" => native::JavaScriptCallPending::WaitDefinition(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_property_definition_state(&data["state"])),
        ),
        "waitNumber" => native::JavaScriptCallPending::WaitNumber(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_number_call_state(&data["state"])),
        ),
        "waitText" => native::JavaScriptCallPending::WaitText(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_text_call_state(&data["state"])),
        ),
        "waitArrayCreation" => native::JavaScriptCallPending::WaitArrayCreation(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_array_creation_state(&data["state"])),
            data.get("construction").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_construction_state(wire_value))
                },)
            }),
            data.get("fallback").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_array_creation_fallback(wire_value))
                },)
            }),
            data.get("error").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_object_from_entries_error_request(
                        wire_value,
                    ))
                },)
            }),
        ),
        "waitConstruction" => native::JavaScriptCallPending::WaitConstruction(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_construction_state(&data["state"])),
            data.get("heldArguments").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_semantic_value)
                            .collect(),
                    )))
                },)
            }),
        ),
        "waitFunctionBind" => native::JavaScriptCallPending::WaitFunctionBind(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_function_bind_state(&data["state"])),
        ),
        "waitFunctionText" => native::JavaScriptCallPending::WaitFunctionText(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
        ),
        "waitConstructorCheck" => native::JavaScriptCallPending::WaitConstructorCheck(
            data["destination"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["input"])),
            data["required"].as_bool().expect("model Boolean"),
        ),
        "waitObjectFromEntries" => native::JavaScriptCallPending::WaitObjectFromEntries(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_object_from_entries_state(&data["state"])),
            data.get("error").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_object_from_entries_error_request(
                        wire_value,
                    ))
                },)
            }),
        ),
        "waitPrimitiveWrapper" => native::JavaScriptCallPending::WaitPrimitiveWrapper(
            data["destination"].as_f64().expect("model Number"),
            Box::new(decode_java_script_primitive_wrapper_state(&data["state"])),
        ),
        "waitSymbolAllocation" => native::JavaScriptCallPending::WaitSymbolAllocation(
            data["destination"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["description"])),
            data.get("registryKey").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
        ),
        "waitSymbolDescription" => native::JavaScriptCallPending::WaitSymbolDescription(
            data["destination"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["symbol"])),
        ),
        "waitFunctionNameSymbol" => native::JavaScriptCallPending::WaitFunctionNameSymbol(
            data["destination"].as_f64().expect("model Number"),
            Box::new(decode_java_script_semantic_value(&data["object"])),
            Box::new(decode_java_script_semantic_value(&data["key"])),
            native::ml_string::Text::from_units(wire_text_units(&data["prefix"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "waitSourceError" => native::JavaScriptCallPending::WaitSourceError(
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
            Box::new(decode_java_script_semantic_value(&data["message"])),
            data["messagePresent"].as_bool().expect("model Boolean"),
        ),
        "waitErrorValue" => native::JavaScriptCallPending::WaitErrorValue(
            data["destination"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
            Box::new(decode_java_script_semantic_value(&data["constructor"])),
            Box::new(decode_java_script_semantic_value(&data["newTarget"])),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            Box::new(decode_java_script_semantic_value(&data["message"])),
            data["messagePresent"].as_bool().expect("model Boolean"),
            data["bindReceiver"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_error_allocation_directory(
                &data["beforeDirectory"],
            )),
        ),
        "waitErrorIntrinsic" => native::JavaScriptCallPending::WaitErrorIntrinsic(
            data["destination"].as_f64().expect("model Number"),
        ),
        "waitHostCapability" => native::JavaScriptCallPending::WaitHostCapability(
            data["destination"].as_f64().expect("model Number"),
            Box::new(decode_java_script_error_allocation_directory(
                &data["beforeDirectory"],
            )),
        ),
        "waitError" => native::JavaScriptCallPending::WaitError(
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
            native::ml_string::Text::from_units(wire_text_units(&data["reason"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
