use super::{
    Data, decode_java_script_array_constructor_fact, decode_java_script_call_machine,
    decode_java_script_call_program, decode_java_script_ordinary_heap,
    decode_java_script_semantic_value, encode_java_script_array_constructor_fact,
    encode_java_script_call_machine, encode_java_script_call_program,
    encode_java_script_ordinary_heap, encode_java_script_semantic_value, is_wire_undefined, native,
    wire_number, wire_text, wire_text_units, wire_undefined,
};

pub(in super::super) fn decode_java_script_call_machine_transition(
    data: &Data,
) -> native::JavaScriptCallMachineTransition {
    match data["$"].as_str().expect("model constructor") {
        "machineContinue" => native::JavaScriptCallMachineTransition::MachineContinue(Box::new(
            decode_java_script_call_machine(&data["machine"]),
        )),
        "machineNeedObjectPrototype" => {
            native::JavaScriptCallMachineTransition::MachineNeedObjectPrototype(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                Box::new(decode_java_script_semantic_value(&data["input"])),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            )
        }
        "machineAllocateSymbol" => native::JavaScriptCallMachineTransition::MachineAllocateSymbol(
            Box::new(decode_java_script_call_machine(&data["machine"])),
            Box::new(decode_java_script_semantic_value(&data["description"])),
            data.get("registryKey").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
        ),
        "machineNeedSymbolDescription" => {
            native::JavaScriptCallMachineTransition::MachineNeedSymbolDescription(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                Box::new(decode_java_script_semantic_value(&data["symbol"])),
            )
        }
        "machineNeedFunctionNameSymbolDescription" => {
            native::JavaScriptCallMachineTransition::MachineNeedFunctionNameSymbolDescription(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                Box::new(decode_java_script_semantic_value(&data["object"])),
                Box::new(decode_java_script_semantic_value(&data["key"])),
                native::ml_string::Text::from_units(wire_text_units(&data["prefix"])),
            )
        }
        "machineNeedFunctionText" => {
            native::JavaScriptCallMachineTransition::MachineNeedFunctionText(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                Box::new(decode_java_script_semantic_value(&data["input"])),
                native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            )
        }
        "machineCompleted" => native::JavaScriptCallMachineTransition::MachineCompleted(
            Box::new(decode_java_script_ordinary_heap(&data["heap"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        "machineThrow" => native::JavaScriptCallMachineTransition::MachineThrow(
            Box::new(decode_java_script_ordinary_heap(&data["heap"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        "machineAllocateObject" => native::JavaScriptCallMachineTransition::MachineAllocateObject(
            Box::new(decode_java_script_call_machine(&data["machine"])),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
        ),
        "machineAllocateError" => native::JavaScriptCallMachineTransition::MachineAllocateError(
            Box::new(decode_java_script_call_machine(&data["machine"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
            native::ml_string::Text::from_units(wire_text_units(&data["reason"])),
        ),
        "machineAllocateSourceError" => {
            native::JavaScriptCallMachineTransition::MachineAllocateSourceError(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
                native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
                Box::new(decode_java_script_semantic_value(&data["message"])),
                data["messagePresent"].as_bool().expect("model Boolean"),
            )
        }
        "machineAllocateErrorValue" => {
            native::JavaScriptCallMachineTransition::MachineAllocateErrorValue(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
                native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
                Box::new(decode_java_script_semantic_value(&data["constructor"])),
                Box::new(decode_java_script_semantic_value(&data["newTarget"])),
                Box::new(decode_java_script_semantic_value(&data["prototype"])),
                Box::new(decode_java_script_semantic_value(&data["message"])),
                data["messagePresent"].as_bool().expect("model Boolean"),
            )
        }
        "machineInvokeErrorIntrinsic" => {
            native::JavaScriptCallMachineTransition::MachineInvokeErrorIntrinsic(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
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
        "machineInvokeHostCapability" => {
            native::JavaScriptCallMachineTransition::MachineInvokeHostCapability(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
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
        "machineConstruct" => native::JavaScriptCallMachineTransition::MachineConstruct(
            Box::new(decode_java_script_call_machine(&data["machine"])),
            Box::new(decode_java_script_semantic_value(&data["constructor"])),
            Box::new(decode_java_script_semantic_value(&data["newTarget"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_semantic_value)
                    .collect(),
            )),
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
            native::ml_string::Text::from_units(wire_text_units(&data["reason"])),
        ),
        "machineConstructArraySpecies" => {
            native::JavaScriptCallMachineTransition::MachineConstructArraySpecies(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                Box::new(decode_java_script_semantic_value(&data["constructor"])),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["arguments"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_semantic_value)
                        .collect(),
                )),
                Box::new(decode_java_script_array_constructor_fact(&data["fact"])),
                data.get("newTarget").map(|wire_value| {
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
        "machineNeedPrimitiveIteratorGet" => {
            native::JavaScriptCallMachineTransition::MachineNeedPrimitiveIteratorGet(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                Box::new(decode_java_script_semantic_value(&data["receiver"])),
                Box::new(decode_java_script_semantic_value(&data["key"])),
            )
        }
        "machineNeedArrayIteratorBoxing" => {
            native::JavaScriptCallMachineTransition::MachineNeedArrayIteratorBoxing(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                Box::new(decode_java_script_semantic_value(&data["receiver"])),
            )
        }
        "machineNeedConstructorAuthority" => {
            native::JavaScriptCallMachineTransition::MachineNeedConstructorAuthority(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                Box::new(decode_java_script_semantic_value(&data["constructor"])),
            )
        }
        "machineNeedCollectionIterable" => {
            native::JavaScriptCallMachineTransition::MachineNeedCollectionIterable(
                Box::new(decode_java_script_call_machine(&data["machine"])),
                Box::new(decode_java_script_semantic_value(&data["target"])),
                Box::new(decode_java_script_semantic_value(&data["input"])),
            )
        }
        "machineInvalid" => native::JavaScriptCallMachineTransition::MachineInvalid(
            Box::new(decode_java_script_ordinary_heap(&data["heap"])),
            native::ml_string::Text::from_units(wire_text_units(&data["reason"])),
        ),
        "machineResourceLimit" => native::JavaScriptCallMachineTransition::MachineResourceLimit(
            Box::new(decode_java_script_call_machine(&data["machine"])),
            native::ml_string::Text::from_units(wire_text_units(&data["reason"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_machine_transition(
    value: &native::JavaScriptCallMachineTransition,
) -> Data {
    match value {
        native::JavaScriptCallMachineTransition::MachineContinue(wire_field_0) => {
            json!({"$":"machineContinue","machine":encode_java_script_call_machine(wire_field_0)})
        }
        native::JavaScriptCallMachineTransition::MachineNeedObjectPrototype(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"machineNeedObjectPrototype","machine":encode_java_script_call_machine(wire_field_0),"input":encode_java_script_semantic_value(wire_field_1),"realm":Data::String(wire_field_2.to_string())})
        }
        native::JavaScriptCallMachineTransition::MachineAllocateSymbol(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"machineAllocateSymbol","machine":encode_java_script_call_machine(wire_field_0),"description":encode_java_script_semantic_value(wire_field_1)});
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
        native::JavaScriptCallMachineTransition::MachineNeedSymbolDescription(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"machineNeedSymbolDescription","machine":encode_java_script_call_machine(wire_field_0),"symbol":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptCallMachineTransition::MachineNeedFunctionNameSymbolDescription(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"machineNeedFunctionNameSymbolDescription","machine":encode_java_script_call_machine(wire_field_0),"object":encode_java_script_semantic_value(wire_field_1),"key":encode_java_script_semantic_value(wire_field_2),"prefix":wire_text(&wire_field_3.0)})
        }
        native::JavaScriptCallMachineTransition::MachineNeedFunctionText(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"machineNeedFunctionText","machine":encode_java_script_call_machine(wire_field_0),"input":encode_java_script_semantic_value(wire_field_1),"kind":wire_text(&wire_field_2.0),"realm":Data::String(wire_field_3.to_string())})
        }
        native::JavaScriptCallMachineTransition::MachineCompleted(wire_field_0, wire_field_1) => {
            json!({"$":"machineCompleted","heap":encode_java_script_ordinary_heap(wire_field_0),"value":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptCallMachineTransition::MachineThrow(wire_field_0, wire_field_1) => {
            json!({"$":"machineThrow","heap":encode_java_script_ordinary_heap(wire_field_0),"value":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptCallMachineTransition::MachineAllocateObject(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"machineAllocateObject","machine":encode_java_script_call_machine(wire_field_0),"prototype":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptCallMachineTransition::MachineAllocateError(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"machineAllocateError","machine":encode_java_script_call_machine(wire_field_0),"realm":Data::String(wire_field_1.to_string()),"errorClass":wire_text(&wire_field_2.0),"reason":wire_text(&wire_field_3.0)})
        }
        native::JavaScriptCallMachineTransition::MachineAllocateSourceError(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"machineAllocateSourceError","machine":encode_java_script_call_machine(wire_field_0),"realm":Data::String(wire_field_1.to_string()),"errorClass":wire_text(&wire_field_2.0),"message":encode_java_script_semantic_value(wire_field_3),"messagePresent":Data::Bool(*wire_field_4)})
        }
        native::JavaScriptCallMachineTransition::MachineAllocateErrorValue(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
        ) => {
            json!({"$":"machineAllocateErrorValue","machine":encode_java_script_call_machine(wire_field_0),"realm":Data::String(wire_field_1.to_string()),"errorClass":wire_text(&wire_field_2.0),"constructor":encode_java_script_semantic_value(wire_field_3),"newTarget":encode_java_script_semantic_value(wire_field_4),"prototype":encode_java_script_semantic_value(wire_field_5),"message":encode_java_script_semantic_value(wire_field_6),"messagePresent":Data::Bool(*wire_field_7)})
        }
        native::JavaScriptCallMachineTransition::MachineInvokeErrorIntrinsic(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"machineInvokeErrorIntrinsic","machine":encode_java_script_call_machine(wire_field_0),"realm":Data::String(wire_field_1.to_string()),"callee":encode_java_script_semantic_value(wire_field_2),"receiver":encode_java_script_semantic_value(wire_field_3),"arguments":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
        native::JavaScriptCallMachineTransition::MachineInvokeHostCapability(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"machineInvokeHostCapability","machine":encode_java_script_call_machine(wire_field_0),"realm":Data::String(wire_field_1.to_string()),"callee":encode_java_script_semantic_value(wire_field_2),"receiver":encode_java_script_semantic_value(wire_field_3),"arguments":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) }})
        }
        native::JavaScriptCallMachineTransition::MachineConstruct(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"machineConstruct","machine":encode_java_script_call_machine(wire_field_0),"constructor":encode_java_script_semantic_value(wire_field_1),"newTarget":encode_java_script_semantic_value(wire_field_2),"arguments":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"operation":wire_text(&wire_field_4.0),"reason":wire_text(&wire_field_5.0)})
        }
        native::JavaScriptCallMachineTransition::MachineConstructArraySpecies(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
        ) => {
            let mut result = json!({"$":"machineConstructArraySpecies","machine":encode_java_script_call_machine(wire_field_0),"constructor":encode_java_script_semantic_value(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_semantic_value).collect()) },"fact":encode_java_script_array_constructor_fact(wire_field_3)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "newTarget".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_semantic_value(wire_present),
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
            if let Some((wire_present,)) = wire_field_6 {
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
        native::JavaScriptCallMachineTransition::MachineNeedPrimitiveIteratorGet(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"machineNeedPrimitiveIteratorGet","machine":encode_java_script_call_machine(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1),"key":encode_java_script_semantic_value(wire_field_2)})
        }
        native::JavaScriptCallMachineTransition::MachineNeedArrayIteratorBoxing(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"machineNeedArrayIteratorBoxing","machine":encode_java_script_call_machine(wire_field_0),"receiver":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptCallMachineTransition::MachineNeedConstructorAuthority(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"machineNeedConstructorAuthority","machine":encode_java_script_call_machine(wire_field_0),"constructor":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptCallMachineTransition::MachineNeedCollectionIterable(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"machineNeedCollectionIterable","machine":encode_java_script_call_machine(wire_field_0),"target":encode_java_script_semantic_value(wire_field_1),"input":encode_java_script_semantic_value(wire_field_2)})
        }
        native::JavaScriptCallMachineTransition::MachineInvalid(wire_field_0, wire_field_1) => {
            json!({"$":"machineInvalid","heap":encode_java_script_ordinary_heap(wire_field_0),"reason":wire_text(&wire_field_1.0)})
        }
        native::JavaScriptCallMachineTransition::MachineResourceLimit(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"machineResourceLimit","machine":encode_java_script_call_machine(wire_field_0),"reason":wire_text(&wire_field_1.0)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_execution(
    data: &Data,
) -> native::JavaScriptCallExecution {
    match data["$"].as_str().expect("model constructor") {
        "callExecution" => native::JavaScriptCallExecution::CallExecution(
            Box::new(decode_java_script_call_machine_transition(
                &data["transition"],
            )),
            data["steps"].as_f64().expect("model Number"),
            data["objectIdentitiesUsed"].as_f64().expect("model Number"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_execution(
    value: &native::JavaScriptCallExecution,
) -> Data {
    match value {
        native::JavaScriptCallExecution::CallExecution(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"callExecution","transition":encode_java_script_call_machine_transition(wire_field_0),"steps":wire_number(*wire_field_1),"objectIdentitiesUsed":wire_number(*wire_field_2)})
        }
    }
}

pub(in super::super) fn decode_java_script_array_authority_update(
    data: &Data,
) -> native::JavaScriptArrayAuthorityUpdate {
    match data["$"].as_str().expect("model constructor") {
        "arrayAuthorityUpdate" => native::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(
            Box::new(decode_java_script_call_program(&data["program"])),
            Box::new(decode_java_script_call_machine(&data["machine"])),
            native::ml_string::Text::from_units(wire_text_units(&data["error"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_array_authority_update(
    value: &native::JavaScriptArrayAuthorityUpdate,
) -> Data {
    match value {
        native::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"arrayAuthorityUpdate","program":encode_java_script_call_program(wire_field_0),"machine":encode_java_script_call_machine(wire_field_1),"error":wire_text(&wire_field_2.0)})
        }
    }
}
