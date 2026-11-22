use super::{
    Data, decode_java_script_semantic_value, encode_java_script_constructor_realm,
    encode_java_script_semantic_value, is_wire_undefined, native, wire_number, wire_text,
    wire_text_units, wire_undefined,
};

pub(in super::super) fn encode_java_script_call_instruction(
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
            json!({"$":"argumentListCall","destination":wire_number(*wire_field_0),"slot":wire_number(*wire_field_1),"callee":wire_number(*wire_field_2),"receiver":wire_number(*wire_field_3),"mode":wire_text(&wire_field_4.encode_utf16().collect::<Vec<u16>>())})
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
            json!({"$":"collectionAllocate","destination":wire_number(*wire_field_0),"prototype":wire_number(*wire_field_1),"kind":wire_text(&wire_field_2.encode_utf16().collect::<Vec<u16>>())})
        }
        native::JavaScriptCallInstruction::CollectionIteratorAllocate(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            let mut result = json!({"$":"collectionIteratorAllocate","destination":wire_number(*wire_field_0),"prototype":wire_number(*wire_field_1),"kind":wire_text(&wire_field_2.encode_utf16().collect::<Vec<u16>>()),"iterationKind":wire_text(&wire_field_3.encode_utf16().collect::<Vec<u16>>())});
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
            let mut result = json!({"$":"iteratorProtocol","destination":wire_number(*wire_field_0),"done":wire_number(*wire_field_1),"input":wire_number(*wire_field_2),"iteratorKey":wire_number(*wire_field_3),"slot":wire_number(*wire_field_4),"operation":wire_text(&wire_field_5.encode_utf16().collect::<Vec<u16>>()),"realm":Data::String(wire_field_6.to_string())});
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
            json!({"$":"collectionIteratorResult","destination":wire_number(*wire_field_0),"kind":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>()),"arrayPrototype":wire_number(*wire_field_2),"objectPrototype":wire_number(*wire_field_3)})
        }
        native::JavaScriptCallInstruction::CollectionOperation(wire_field_0, wire_field_1) => {
            json!({"$":"collectionOperation","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>())})
        }
        native::JavaScriptCallInstruction::PrimitiveMethod(wire_field_0, wire_field_1) => {
            json!({"$":"primitiveMethod","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>())})
        }
        native::JavaScriptCallInstruction::CollectionForEach(wire_field_0, wire_field_1) => {
            json!({"$":"collectionForEach","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>())})
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
            json!({"$":"arrayCreation","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>()),"prototype":wire_number(*wire_field_2),"iteratorKey":wire_number(*wire_field_3)})
        }
        native::JavaScriptCallInstruction::ArrayCallback(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"arrayCallback","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>())});
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
            let mut result = json!({"$":"arraySearch","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>())});
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
            let mut result = json!({"$":"arrayTransform","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>()),"species":encode_java_script_array_species_directory(wire_field_2)});
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
            json!({"$":"objectEnumeration","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>()),"prototype":wire_number(*wire_field_2)})
        }
        native::JavaScriptCallInstruction::PropertyDefinitions(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"propertyDefinitions","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>()),"target":wire_number(*wire_field_2),"input":wire_number(*wire_field_3)})
        }
        native::JavaScriptCallInstruction::PropertyReflection(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"propertyReflection","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>()),"input":wire_number(*wire_field_2),"key":wire_number(*wire_field_3),"objectPrototype":wire_number(*wire_field_4),"arrayPrototype":wire_number(*wire_field_5)})
        }
        native::JavaScriptCallInstruction::InstanceOf(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"instanceOf","destination":wire_number(*wire_field_0),"left":wire_number(*wire_field_1),"right":wire_number(*wire_field_2),"key":wire_number(*wire_field_3),"operation":wire_text(&wire_field_4.encode_utf16().collect::<Vec<u16>>())})
        }
        native::JavaScriptCallInstruction::ObjectObservation(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"objectObservation","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>()),"input":wire_number(*wire_field_2),"other":wire_number(*wire_field_3)})
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
            json!({"$":"constructErrorValue","destination":wire_number(*wire_field_0),"constructor":wire_number(*wire_field_1),"newTarget":wire_number(*wire_field_2),"prototype":wire_number(*wire_field_3),"message":wire_number(*wire_field_4),"present":wire_number(*wire_field_5),"realm":Data::String(wire_field_6.to_string()),"errorClass":wire_text(&wire_field_7.encode_utf16().collect::<Vec<u16>>()),"bindReceiver":Data::Bool(*wire_field_8)})
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
            json!({"$":"numberBinary","destination":wire_number(*wire_field_0),"left":wire_number(*wire_field_1),"right":wire_number(*wire_field_2),"operator":wire_text(&wire_field_3.encode_utf16().collect::<Vec<u16>>())})
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
            json!({"$":"primitive","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>()),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) }})
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
            json!({"$":"literalAccessor","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"value":wire_number(*wire_field_3),"kind":wire_text(&wire_field_4.encode_utf16().collect::<Vec<u16>>()),"realm":Data::String(wire_field_5.to_string())})
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
            json!({"$":"symbolRegistry","destination":wire_number(*wire_field_0),"input":wire_number(*wire_field_1),"operation":wire_text(&wire_field_2.encode_utf16().collect::<Vec<u16>>())})
        }
        native::JavaScriptCallInstruction::FunctionName(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"functionName","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"prefix":wire_text(&wire_field_3.encode_utf16().collect::<Vec<u16>>()),"realm":Data::String(wire_field_4.to_string())})
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
                        Some(wire_present) => {
                            wire_text(&wire_present.encode_utf16().collect::<Vec<u16>>())
                        }
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
                        Some(wire_present) => {
                            wire_text(&wire_present.encode_utf16().collect::<Vec<u16>>())
                        }
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
        "arrayConstructorFact" => {
            native::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
                data["constructable"].as_bool().expect("model Boolean"),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
                data["intrinsicArray"].as_bool().expect("model Boolean"),
                data.get("intrinsicKind").map(|wire_value| {
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
                        Some(wire_present) => {
                            wire_text(&wire_present.encode_utf16().collect::<Vec<u16>>())
                        }
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
