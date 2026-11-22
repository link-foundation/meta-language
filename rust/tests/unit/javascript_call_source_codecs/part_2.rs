use super::{
    Data, decode_java_script_call_instruction, decode_java_script_call_source,
    encode_java_script_call_instruction, is_wire_undefined, native, wire_number, wire_text,
    wire_text_units, wire_undefined,
};

pub(in super::super) fn encode_java_script_call_source(
    value: &native::JavaScriptCallSource,
) -> Data {
    match value {
        native::JavaScriptCallSource::SourceConstant(wire_field_0) => {
            json!({"$":"sourceConstant","value":encode_java_script_semantic_value(wire_field_0)})
        }
        native::JavaScriptCallSource::SourceNewTarget => json!({"$":"sourceNewTarget"}),
        native::JavaScriptCallSource::SourceReceiver => json!({"$":"sourceReceiver"}),
        native::JavaScriptCallSource::SourceArgument(wire_field_0) => {
            json!({"$":"sourceArgument","index":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallSource::SourceDerivedReceiver => json!({"$":"sourceDerivedReceiver"}),
        native::JavaScriptCallSource::SourceErrorIntrinsic => json!({"$":"sourceErrorIntrinsic"}),
        native::JavaScriptCallSource::SourceConstruct(wire_field_0, wire_field_1) => {
            json!({"$":"sourceConstruct","callee":encode_java_script_call_source(wire_field_0),"arguments":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source).collect()) }})
        }
        native::JavaScriptCallSource::SourceConstructErrorClass(wire_field_0, wire_field_1) => {
            json!({"$":"sourceConstructErrorClass","callee":encode_java_script_call_source(wire_field_0),"arguments":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source).collect()) }})
        }
        native::JavaScriptCallSource::SourceClassConstructorCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceClassConstructorCall","callee":encode_java_script_call_source(wire_field_0),"receiver":encode_java_script_call_source(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source).collect()) }})
        }
        native::JavaScriptCallSource::SourceConstructCollection(wire_field_0, wire_field_1) => {
            json!({"$":"sourceConstructCollection","callee":encode_java_script_call_source(wire_field_0),"arguments":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source).collect()) }})
        }
        native::JavaScriptCallSource::SourceVariadicCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"sourceVariadicCall","callee":encode_java_script_call_source(wire_field_0),"receiver":encode_java_script_call_source(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source).collect()) },"mode":wire_text(&wire_field_3.encode_utf16().collect::<Vec<u16>>()),"receiverAfterArguments":Data::Bool(*wire_field_4)})
        }
        native::JavaScriptCallSource::SourceArgumentSpread(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceArgumentSpread","value":encode_java_script_call_source(wire_field_0),"iteratorKey":encode_java_script_call_source(wire_field_1),"realm":Data::String(wire_field_2.to_string())})
        }
        native::JavaScriptCallSource::SourceConstructErrorValue(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
        ) => {
            json!({"$":"sourceConstructErrorValue","constructor":encode_java_script_call_source(wire_field_0),"newTarget":encode_java_script_call_source(wire_field_1),"prototype":encode_java_script_call_source(wire_field_2),"message":encode_java_script_call_source(wire_field_3),"present":encode_java_script_call_source(wire_field_4),"realm":Data::String(wire_field_5.to_string()),"errorClass":wire_text(&wire_field_6.encode_utf16().collect::<Vec<u16>>()),"bindReceiver":Data::Bool(*wire_field_7)})
        }
        native::JavaScriptCallSource::SourceThrow(wire_field_0) => {
            json!({"$":"sourceThrow","value":encode_java_script_call_source(wire_field_0)})
        }
        native::JavaScriptCallSource::SourceReturn(wire_field_0) => {
            json!({"$":"sourceReturn","value":encode_java_script_call_source(wire_field_0)})
        }
        native::JavaScriptCallSource::SourceAssignBinding(wire_field_0, wire_field_1) => {
            json!({"$":"sourceAssignBinding","identity":wire_number(*wire_field_0),"value":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSource::SourceImmutableBindingWrite(wire_field_0) => {
            json!({"$":"sourceImmutableBindingWrite","value":encode_java_script_call_source(wire_field_0)})
        }
        native::JavaScriptCallSource::SourceWhile(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
        ) => {
            let mut result = json!({"$":"sourceWhile","condition":encode_java_script_call_source(wire_field_0),"body":encode_java_script_call_source(wire_field_1),"after":encode_java_script_call_source(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "target".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "update".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_call_source(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "postTest".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_6 {
                fields.insert(
                    "iteration".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_call_source(wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallSource::SourceCloneBindingCell(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"sourceCloneBindingCell","identity":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string())});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "lexical".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallSource::SourceControlJump(wire_field_0, wire_field_1) => {
            json!({"$":"sourceControlJump","target":wire_number(*wire_field_0),"continuing":Data::Bool(*wire_field_1)})
        }
        native::JavaScriptCallSource::SourceControlBlock(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceControlBlock","target":wire_number(*wire_field_0),"body":encode_java_script_call_source(wire_field_1),"after":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceSwitch(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"sourceSwitch","target":wire_number(*wire_field_0),"value":encode_java_script_call_source(wire_field_1),"clauses":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source_switch_clause).collect()) },"after":encode_java_script_call_source(wire_field_3)})
        }
        native::JavaScriptCallSource::SourceBindingScope(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceBindingScope","declarations":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_binding_declaration).collect()) },"initializer":encode_java_script_call_source(wire_field_1),"body":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceLexicalScope(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            let mut result = json!({"$":"sourceLexicalScope","identities":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) },"realm":Data::String(wire_field_3.to_string()),"body":encode_java_script_call_source(wire_field_4)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_1 {
                fields.insert(
                    "existing".into(),
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
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "initializers".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_call_cell_initializer)
                                    .collect(),
                            )
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptCallSource::SourceWriteExisting(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"sourceWriteExisting","identity":wire_number(*wire_field_0),"cell":Data::Bool(*wire_field_1),"value":encode_java_script_call_source(wire_field_2),"body":encode_java_script_call_source(wire_field_3)})
        }
        native::JavaScriptCallSource::SourceInitializeCell(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceInitializeCell","identity":wire_number(*wire_field_0),"value":encode_java_script_call_source(wire_field_1),"body":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceInitializeBinding(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceInitializeBinding","identity":wire_number(*wire_field_0),"value":encode_java_script_call_source(wire_field_1),"body":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceIteratorBinding(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"sourceIteratorBinding","slot":wire_number(*wire_field_0),"input":encode_java_script_call_source(wire_field_1),"iteratorKey":encode_java_script_call_source(wire_field_2),"body":encode_java_script_call_source(wire_field_3),"realm":Data::String(wire_field_4.to_string())})
        }
        native::JavaScriptCallSource::SourceIteratorStep(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceIteratorStep","slot":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string()),"readValue":Data::Bool(*wire_field_2)})
        }
        native::JavaScriptCallSource::SourceIteratorRemainder(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceIteratorRemainder","slot":wire_number(*wire_field_0),"prototype":encode_java_script_call_source(wire_field_1),"realm":Data::String(wire_field_2.to_string())})
        }
        native::JavaScriptCallSource::SourceForOf(
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
            let mut result = json!({"$":"sourceForOf","slot":wire_number(*wire_field_0),"target":wire_number(*wire_field_1),"binding":wire_number(*wire_field_2),"mutable":Data::Bool(*wire_field_3),"input":encode_java_script_call_source(wire_field_4),"iteratorKey":encode_java_script_call_source(wire_field_5),"body":encode_java_script_call_source(wire_field_6),"realm":Data::String(wire_field_7.to_string())});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "scopeBindings".into(),
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
            result
        }
        native::JavaScriptCallSource::SourceIteratorLoop(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
        ) => {
            let mut result = json!({"$":"sourceIteratorLoop","slot":wire_number(*wire_field_0),"target":wire_number(*wire_field_1),"binding":wire_number(*wire_field_2),"mutable":Data::Bool(*wire_field_3),"body":encode_java_script_call_source(wire_field_4),"realm":Data::String(wire_field_5.to_string())});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_6 {
                fields.insert(
                    "scopeBindings".into(),
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
            result
        }
        native::JavaScriptCallSource::SourceIteratorClose(wire_field_0, wire_field_1) => {
            json!({"$":"sourceIteratorClose","slot":wire_number(*wire_field_0),"realm":Data::String(wire_field_1.to_string())})
        }
        native::JavaScriptCallSource::SourceFinally(wire_field_0, wire_field_1) => {
            json!({"$":"sourceFinally","body":encode_java_script_call_source(wire_field_0),"finalizer":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSource::SourceTry(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sourceTry","identity":wire_number(*wire_field_0),"body":encode_java_script_call_source(wire_field_1),"handler":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceBinding(wire_field_0) => {
            json!({"$":"sourceBinding","identity":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallSource::SourceLet(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sourceLet","identity":wire_number(*wire_field_0),"value":encode_java_script_call_source(wire_field_1),"body":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceLocalLet(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"sourceLocalLet","identity":wire_number(*wire_field_0),"value":encode_java_script_call_source(wire_field_1),"body":encode_java_script_call_source(wire_field_2),"mutable":Data::Bool(*wire_field_3)})
        }
        native::JavaScriptCallSource::SourceArgumentRest(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceArgumentRest","from":wire_number(*wire_field_0),"prototype":encode_java_script_call_source(wire_field_1),"realm":Data::String(wire_field_2.to_string())})
        }
        native::JavaScriptCallSource::SourceParameterRest(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"sourceParameterRest","identity":wire_number(*wire_field_0),"from":wire_number(*wire_field_1),"prototype":encode_java_script_call_source(wire_field_2),"realm":Data::String(wire_field_3.to_string()),"body":encode_java_script_call_source(wire_field_4)})
        }
        native::JavaScriptCallSource::SourceParameterDefault(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceParameterDefault","identity":wire_number(*wire_field_0),"value":encode_java_script_call_source(wire_field_1),"body":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceIf(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sourceIf","condition":encode_java_script_call_source(wire_field_0),"yes":encode_java_script_call_source(wire_field_1),"no":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceNumberBinary(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceNumberBinary","operator":wire_text(&wire_field_0.encode_utf16().collect::<Vec<u16>>()),"left":encode_java_script_call_source(wire_field_1),"right":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceSameValue(wire_field_0, wire_field_1) => {
            json!({"$":"sourceSameValue","left":encode_java_script_call_source(wire_field_0),"right":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSource::SourceStrictEqual(wire_field_0, wire_field_1) => {
            json!({"$":"sourceStrictEqual","left":encode_java_script_call_source(wire_field_0),"right":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSource::SourceSnapshotStrictEqual(wire_field_0, wire_field_1) => {
            json!({"$":"sourceSnapshotStrictEqual","left":encode_java_script_call_source(wire_field_0),"right":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSource::SourcePrimitive(wire_field_0, wire_field_1) => {
            json!({"$":"sourcePrimitive","operation":wire_text(&wire_field_0.encode_utf16().collect::<Vec<u16>>()),"arguments":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source).collect()) }})
        }
        native::JavaScriptCallSource::SourceRecur(wire_field_0) => {
            json!({"$":"sourceRecur","arguments":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source).collect()) }})
        }
        native::JavaScriptCallSource::SourceAllocate(wire_field_0) => {
            json!({"$":"sourceAllocate","prototype":encode_java_script_call_source(wire_field_0)})
        }
        native::JavaScriptCallSource::SourceClosure(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"sourceClosure","owner":Data::String(wire_field_0.to_string()),"ordinal":wire_number(*wire_field_1),"captures":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source).collect()) },"receiver":encode_java_script_call_source(wire_field_3)})
        }
        native::JavaScriptCallSource::SourceObject(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sourceObject","prototype":encode_java_script_call_source(wire_field_0),"properties":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source_property).collect()) },"realm":Data::String(wire_field_2.to_string())})
        }
        native::JavaScriptCallSource::SourceArrayAccumulation(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"sourceArrayAccumulation","prototype":encode_java_script_call_source(wire_field_0),"parts":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_array_part).collect()) },"iteratorKey":encode_java_script_call_source(wire_field_2),"realm":Data::String(wire_field_3.to_string())})
        }
        native::JavaScriptCallSource::SourceArray(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"sourceArray","prototype":encode_java_script_call_source(wire_field_0),"length":wire_number(*wire_field_1),"elements":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source_array_element).collect()) },"realm":Data::String(wire_field_3.to_string())})
        }
        native::JavaScriptCallSource::SourcePropertyKey(wire_field_0, wire_field_1) => {
            json!({"$":"sourcePropertyKey","input":encode_java_script_call_source(wire_field_0),"realm":Data::String(wire_field_1.to_string())})
        }
        native::JavaScriptCallSource::SourcePropertyDefinition(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"sourcePropertyDefinition","target":encode_java_script_call_source(wire_field_0),"key":encode_java_script_call_source(wire_field_1),"attributes":encode_java_script_call_source(wire_field_2),"realm":Data::String(wire_field_3.to_string())})
        }
        native::JavaScriptCallSource::SourceSet(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sourceSet","object":encode_java_script_call_source(wire_field_0),"key":encode_java_script_call_source(wire_field_1),"value":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceSnapshotSet(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceSnapshotSet","object":encode_java_script_call_source(wire_field_0),"key":encode_java_script_call_source(wire_field_1),"value":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceStringGet(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sourceStringGet","input":encode_java_script_call_source(wire_field_0),"key":encode_java_script_call_source(wire_field_1),"prototype":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceGet(wire_field_0, wire_field_1, wire_field_2) => {
            let mut result = json!({"$":"sourceGet","object":encode_java_script_call_source(wire_field_0),"key":encode_java_script_call_source(wire_field_1)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "receiver".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_call_source(wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallSource::SourceInstanceOf(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceInstanceOf","left":encode_java_script_call_source(wire_field_0),"right":encode_java_script_call_source(wire_field_1),"key":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceDelete(wire_field_0, wire_field_1) => {
            json!({"$":"sourceDelete","object":encode_java_script_call_source(wire_field_0),"key":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSource::SourceHas(wire_field_0, wire_field_1) => {
            json!({"$":"sourceHas","object":encode_java_script_call_source(wire_field_0),"key":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSource::SourceCall(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sourceCall","callee":encode_java_script_call_source(wire_field_0),"receiver":encode_java_script_call_source(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source).collect()) }})
        }
        native::JavaScriptCallSource::SourceSnapshotCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceSnapshotCall","callee":encode_java_script_call_source(wire_field_0),"receiver":encode_java_script_call_source(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_source).collect()) }})
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
            String::from_utf16(&wire_text_units(&data["profile"]))
                .expect("model text requires Unicode scalar values in this native profile"),
            String::from_utf16(&wire_text_units(&data["text"]))
                .expect("model text requires Unicode scalar values in this native profile"),
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
            json!({"$":"sourceText","profile":wire_text(&wire_field_0.encode_utf16().collect::<Vec<u16>>()),"text":wire_text(&wire_field_1.encode_utf16().collect::<Vec<u16>>())})
        }
        native::JavaScriptSemanticValue::Symbol(wire_field_0) => {
            json!({"$":"symbol","identity":Data::String(wire_field_0.to_string())})
        }
        native::JavaScriptSemanticValue::Object(wire_field_0, wire_field_1) => {
            json!({"$":"object","identity":Data::String(wire_field_0.to_string()),"callable":Data::Bool(*wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_source_switch_clause(
    data: &Data,
) -> native::JavaScriptCallSourceSwitchClause {
    match data["$"].as_str().expect("model constructor") {
        "sourceSwitchCase" => native::JavaScriptCallSourceSwitchClause::SourceSwitchCase(
            Box::new(decode_java_script_call_source(&data["value"])),
            Box::new(decode_java_script_call_source(&data["body"])),
        ),
        "sourceSwitchDefault" => native::JavaScriptCallSourceSwitchClause::SourceSwitchDefault(
            Box::new(decode_java_script_call_source(&data["body"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_source_switch_clause(
    value: &native::JavaScriptCallSourceSwitchClause,
) -> Data {
    match value {
        native::JavaScriptCallSourceSwitchClause::SourceSwitchCase(wire_field_0, wire_field_1) => {
            json!({"$":"sourceSwitchCase","value":encode_java_script_call_source(wire_field_0),"body":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSourceSwitchClause::SourceSwitchDefault(wire_field_0) => {
            json!({"$":"sourceSwitchDefault","body":encode_java_script_call_source(wire_field_0)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_binding_declaration(
    data: &Data,
) -> native::JavaScriptCallBindingDeclaration {
    match data["$"].as_str().expect("model constructor") {
        "sourceBindingDeclaration" => {
            native::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(
                data["identity"].as_f64().expect("model Number"),
                data["mutable"].as_bool().expect("model Boolean"),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_binding_declaration(
    value: &native::JavaScriptCallBindingDeclaration,
) -> Data {
    match value {
        native::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"sourceBindingDeclaration","identity":wire_number(*wire_field_0),"mutable":Data::Bool(*wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_cell_initializer(
    data: &Data,
) -> native::JavaScriptCallCellInitializer {
    match data["$"].as_str().expect("model constructor") {
        "sourceCellInitializer" => native::JavaScriptCallCellInitializer::SourceCellInitializer(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["value"])),
            data["initialized"].as_bool().expect("model Boolean"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_cell_initializer(
    value: &native::JavaScriptCallCellInitializer,
) -> Data {
    match value {
        native::JavaScriptCallCellInitializer::SourceCellInitializer(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceCellInitializer","identity":wire_number(*wire_field_0),"value":encode_java_script_call_source(wire_field_1),"initialized":Data::Bool(*wire_field_2)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_source_property(
    data: &Data,
) -> native::JavaScriptCallSourceProperty {
    match data["$"].as_str().expect("model constructor") {
        "sourceObjectProperty" => native::JavaScriptCallSourceProperty::SourceObjectProperty(
            Box::new(decode_java_script_call_source(&data["key"])),
            Box::new(decode_java_script_call_source(&data["value"])),
            data.get("inferName").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
        ),
        "sourceObjectAccessor" => native::JavaScriptCallSourceProperty::SourceObjectAccessor(
            Box::new(decode_java_script_call_source(&data["key"])),
            Box::new(decode_java_script_call_source(&data["value"])),
            String::from_utf16(&wire_text_units(&data["kind"]))
                .expect("model text requires Unicode scalar values in this native profile"),
            data.get("inferName").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
        ),
        "sourceLiteralPrototype" => native::JavaScriptCallSourceProperty::SourceLiteralPrototype(
            Box::new(decode_java_script_call_source(&data["value"])),
        ),
        "sourceObjectSpread" => native::JavaScriptCallSourceProperty::SourceObjectSpread(
            Box::new(decode_java_script_call_source(&data["input"])),
            data.get("excluded").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_call_source)
                            .collect(),
                    )))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_source_property(
    value: &native::JavaScriptCallSourceProperty,
) -> Data {
    match value {
        native::JavaScriptCallSourceProperty::SourceObjectProperty(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"sourceObjectProperty","key":encode_java_script_call_source(wire_field_0),"value":encode_java_script_call_source(wire_field_1)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "inferName".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallSourceProperty::SourceObjectAccessor(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"sourceObjectAccessor","key":encode_java_script_call_source(wire_field_0),"value":encode_java_script_call_source(wire_field_1),"kind":wire_text(&wire_field_2.encode_utf16().collect::<Vec<u16>>())});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "inferName".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptCallSourceProperty::SourceLiteralPrototype(wire_field_0) => {
            json!({"$":"sourceLiteralPrototype","value":encode_java_script_call_source(wire_field_0)})
        }
        native::JavaScriptCallSourceProperty::SourceObjectSpread(wire_field_0, wire_field_1) => {
            let mut result = json!({"$":"sourceObjectSpread","input":encode_java_script_call_source(wire_field_0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_1 {
                fields.insert(
                    "excluded".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_call_source)
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

pub(in super::super) fn decode_java_script_call_array_part(
    data: &Data,
) -> native::JavaScriptCallArrayPart {
    match data["$"].as_str().expect("model constructor") {
        "sourceArrayValue" => native::JavaScriptCallArrayPart::SourceArrayValue(Box::new(
            decode_java_script_call_source(&data["value"]),
        )),
        "sourceArraySpread" => native::JavaScriptCallArrayPart::SourceArraySpread(Box::new(
            decode_java_script_call_source(&data["value"]),
        )),
        "sourceArrayHole" => native::JavaScriptCallArrayPart::SourceArrayHole,
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_array_part(
    value: &native::JavaScriptCallArrayPart,
) -> Data {
    match value {
        native::JavaScriptCallArrayPart::SourceArrayValue(wire_field_0) => {
            json!({"$":"sourceArrayValue","value":encode_java_script_call_source(wire_field_0)})
        }
        native::JavaScriptCallArrayPart::SourceArraySpread(wire_field_0) => {
            json!({"$":"sourceArraySpread","value":encode_java_script_call_source(wire_field_0)})
        }
        native::JavaScriptCallArrayPart::SourceArrayHole => json!({"$":"sourceArrayHole"}),
    }
}

pub(in super::super) fn decode_java_script_call_source_array_element(
    data: &Data,
) -> native::JavaScriptCallSourceArrayElement {
    match data["$"].as_str().expect("model constructor") {
        "sourceArrayElement" => native::JavaScriptCallSourceArrayElement::SourceArrayElement(
            data["index"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["value"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_source_array_element(
    value: &native::JavaScriptCallSourceArrayElement,
) -> Data {
    match value {
        native::JavaScriptCallSourceArrayElement::SourceArrayElement(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"sourceArrayElement","index":wire_number(*wire_field_0),"value":encode_java_script_call_source(wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_call_source_compilation(
    data: &Data,
) -> native::JavaScriptCallSourceCompilation {
    match data["$"].as_str().expect("model constructor") {
        "sourceCompiled" => native::JavaScriptCallSourceCompilation::SourceCompiled(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["instructions"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_instruction)
                    .collect(),
            )),
            data["registers"].as_f64().expect("model Number"),
            String::from_utf16(&wire_text_units(&data["error"]))
                .expect("model text requires Unicode scalar values in this native profile"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_call_source_compilation(
    value: &native::JavaScriptCallSourceCompilation,
) -> Data {
    match value {
        native::JavaScriptCallSourceCompilation::SourceCompiled(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceCompiled","instructions":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_call_instruction).collect()) },"registers":wire_number(*wire_field_1),"error":wire_text(&wire_field_2.encode_utf16().collect::<Vec<u16>>())})
        }
    }
}
