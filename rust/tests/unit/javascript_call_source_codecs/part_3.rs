use super::{
    Data, decode_java_script_array_species_directory, decode_java_script_semantic_value,
    is_wire_undefined, native, wire_text_units,
};

pub(in super::super) fn decode_java_script_call_instruction(
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
            String::from_utf16(&wire_text_units(&data["mode"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["kind"]))
                .expect("model text requires Unicode scalar values in this native profile"),
        ),
        "collectionIteratorAllocate" => {
            native::JavaScriptCallInstruction::CollectionIteratorAllocate(
                data["destination"].as_f64().expect("model Number"),
                data["prototype"].as_f64().expect("model Number"),
                String::from_utf16(&wire_text_units(&data["kind"]))
                    .expect("model text requires Unicode scalar values in this native profile"),
                String::from_utf16(&wire_text_units(&data["iterationKind"]))
                    .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["kind"]))
                .expect("model text requires Unicode scalar values in this native profile"),
            data["arrayPrototype"].as_f64().expect("model Number"),
            data["objectPrototype"].as_f64().expect("model Number"),
        ),
        "collectionOperation" => native::JavaScriptCallInstruction::CollectionOperation(
            data["destination"].as_f64().expect("model Number"),
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
        ),
        "primitiveMethod" => native::JavaScriptCallInstruction::PrimitiveMethod(
            data["destination"].as_f64().expect("model Number"),
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
        ),
        "collectionForEach" => native::JavaScriptCallInstruction::CollectionForEach(
            data["destination"].as_f64().expect("model Number"),
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
            data["prototype"].as_f64().expect("model Number"),
            data["iteratorKey"].as_f64().expect("model Number"),
        ),
        "arrayCallback" => native::JavaScriptCallInstruction::ArrayCallback(
            data["destination"].as_f64().expect("model Number"),
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
            data["prototype"].as_f64().expect("model Number"),
        ),
        "propertyDefinitions" => native::JavaScriptCallInstruction::PropertyDefinitions(
            data["destination"].as_f64().expect("model Number"),
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
            data["target"].as_f64().expect("model Number"),
            data["input"].as_f64().expect("model Number"),
        ),
        "propertyReflection" => native::JavaScriptCallInstruction::PropertyReflection(
            data["destination"].as_f64().expect("model Number"),
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
        ),
        "objectObservation" => native::JavaScriptCallInstruction::ObjectObservation(
            data["destination"].as_f64().expect("model Number"),
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["errorClass"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["operator"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["kind"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
        ),
        "functionName" => native::JavaScriptCallInstruction::FunctionName(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            String::from_utf16(&wire_text_units(&data["prefix"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
        "objectSpread" => {
            native::JavaScriptCallInstruction::ObjectSpread(
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
                        Some(String::from_utf16(&wire_text_units(&wire_value)).expect(
                            "model text requires Unicode scalar values in this native profile",
                        ))
                    },)
                }),
            )
        }
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
        "propertyDefinition" => {
            native::JavaScriptCallInstruction::PropertyDefinition(
                data["destination"].as_f64().expect("model Number"),
                data["target"].as_f64().expect("model Number"),
                data["key"].as_f64().expect("model Number"),
                data["attributes"].as_f64().expect("model Number"),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
                data.get("operation").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(String::from_utf16(&wire_text_units(&wire_value)).expect(
                            "model text requires Unicode scalar values in this native profile",
                        ))
                    },)
                }),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}
