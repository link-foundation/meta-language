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
pub(super) fn decode_java_script_call_source(data: &Data) -> native::JavaScriptCallSource {
    match data["$"].as_str().expect("model constructor") {
        "sourceConstant" => native::JavaScriptCallSource::SourceConstant(Box::new(
            decode_java_script_semantic_value(&data["value"]),
        )),
        "sourceNewTarget" => native::JavaScriptCallSource::SourceNewTarget,
        "sourceReceiver" => native::JavaScriptCallSource::SourceReceiver,
        "sourceArgument" => native::JavaScriptCallSource::SourceArgument(
            data["index"].as_f64().expect("model Number"),
        ),
        "sourceDerivedReceiver" => native::JavaScriptCallSource::SourceDerivedReceiver,
        "sourceErrorIntrinsic" => native::JavaScriptCallSource::SourceErrorIntrinsic,
        "sourceConstruct" => native::JavaScriptCallSource::SourceConstruct(
            Box::new(decode_java_script_call_source(&data["callee"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source)
                    .collect(),
            )),
        ),
        "sourceConstructErrorClass" => native::JavaScriptCallSource::SourceConstructErrorClass(
            Box::new(decode_java_script_call_source(&data["callee"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source)
                    .collect(),
            )),
        ),
        "sourceClassConstructorCall" => native::JavaScriptCallSource::SourceClassConstructorCall(
            Box::new(decode_java_script_call_source(&data["callee"])),
            Box::new(decode_java_script_call_source(&data["receiver"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source)
                    .collect(),
            )),
        ),
        "sourceConstructCollection" => native::JavaScriptCallSource::SourceConstructCollection(
            Box::new(decode_java_script_call_source(&data["callee"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source)
                    .collect(),
            )),
        ),
        "sourceVariadicCall" => native::JavaScriptCallSource::SourceVariadicCall(
            Box::new(decode_java_script_call_source(&data["callee"])),
            Box::new(decode_java_script_call_source(&data["receiver"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source)
                    .collect(),
            )),
            String::from_utf16(&wire_text_units(&data["mode"]))
                .expect("model text requires Unicode scalar values in this native profile"),
            data["receiverAfterArguments"]
                .as_bool()
                .expect("model Boolean"),
        ),
        "sourceArgumentSpread" => native::JavaScriptCallSource::SourceArgumentSpread(
            Box::new(decode_java_script_call_source(&data["value"])),
            Box::new(decode_java_script_call_source(&data["iteratorKey"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "sourceConstructErrorValue" => native::JavaScriptCallSource::SourceConstructErrorValue(
            Box::new(decode_java_script_call_source(&data["constructor"])),
            Box::new(decode_java_script_call_source(&data["newTarget"])),
            Box::new(decode_java_script_call_source(&data["prototype"])),
            Box::new(decode_java_script_call_source(&data["message"])),
            Box::new(decode_java_script_call_source(&data["present"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            String::from_utf16(&wire_text_units(&data["errorClass"]))
                .expect("model text requires Unicode scalar values in this native profile"),
            data["bindReceiver"].as_bool().expect("model Boolean"),
        ),
        "sourceThrow" => native::JavaScriptCallSource::SourceThrow(Box::new(
            decode_java_script_call_source(&data["value"]),
        )),
        "sourceReturn" => native::JavaScriptCallSource::SourceReturn(Box::new(
            decode_java_script_call_source(&data["value"]),
        )),
        "sourceAssignBinding" => native::JavaScriptCallSource::SourceAssignBinding(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["value"])),
        ),
        "sourceImmutableBindingWrite" => native::JavaScriptCallSource::SourceImmutableBindingWrite(
            Box::new(decode_java_script_call_source(&data["value"])),
        ),
        "sourceWhile" => native::JavaScriptCallSource::SourceWhile(
            Box::new(decode_java_script_call_source(&data["condition"])),
            Box::new(decode_java_script_call_source(&data["body"])),
            Box::new(decode_java_script_call_source(&data["after"])),
            data.get("target").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_f64().expect("model Number"))
                },)
            }),
            data.get("update").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_call_source(wire_value))
                },)
            }),
            data.get("postTest").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
            data.get("iteration").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_call_source(wire_value))
                },)
            }),
        ),
        "sourceCloneBindingCell" => native::JavaScriptCallSource::SourceCloneBindingCell(
            data["identity"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            data.get("lexical").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
        ),
        "sourceControlJump" => native::JavaScriptCallSource::SourceControlJump(
            data["target"].as_f64().expect("model Number"),
            data["continuing"].as_bool().expect("model Boolean"),
        ),
        "sourceControlBlock" => native::JavaScriptCallSource::SourceControlBlock(
            data["target"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["body"])),
            Box::new(decode_java_script_call_source(&data["after"])),
        ),
        "sourceSwitch" => native::JavaScriptCallSource::SourceSwitch(
            data["target"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["value"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["clauses"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source_switch_clause)
                    .collect(),
            )),
            Box::new(decode_java_script_call_source(&data["after"])),
        ),
        "sourceBindingScope" => native::JavaScriptCallSource::SourceBindingScope(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["declarations"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_binding_declaration)
                    .collect(),
            )),
            Box::new(decode_java_script_call_source(&data["initializer"])),
            Box::new(decode_java_script_call_source(&data["body"])),
        ),
        "sourceLexicalScope" => native::JavaScriptCallSource::SourceLexicalScope(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["identities"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
            data.get("existing").map(|wire_value| {
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
            data.get("initializers").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_call_cell_initializer)
                            .collect(),
                    )))
                },)
            }),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_call_source(&data["body"])),
        ),
        "sourceWriteExisting" => native::JavaScriptCallSource::SourceWriteExisting(
            data["identity"].as_f64().expect("model Number"),
            data["cell"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_call_source(&data["value"])),
            Box::new(decode_java_script_call_source(&data["body"])),
        ),
        "sourceInitializeCell" => native::JavaScriptCallSource::SourceInitializeCell(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["value"])),
            Box::new(decode_java_script_call_source(&data["body"])),
        ),
        "sourceInitializeBinding" => native::JavaScriptCallSource::SourceInitializeBinding(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["value"])),
            Box::new(decode_java_script_call_source(&data["body"])),
        ),
        "sourceIteratorBinding" => native::JavaScriptCallSource::SourceIteratorBinding(
            data["slot"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["input"])),
            Box::new(decode_java_script_call_source(&data["iteratorKey"])),
            Box::new(decode_java_script_call_source(&data["body"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "sourceIteratorStep" => native::JavaScriptCallSource::SourceIteratorStep(
            data["slot"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            data["readValue"].as_bool().expect("model Boolean"),
        ),
        "sourceIteratorRemainder" => native::JavaScriptCallSource::SourceIteratorRemainder(
            data["slot"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["prototype"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "sourceForOf" => native::JavaScriptCallSource::SourceForOf(
            data["slot"].as_f64().expect("model Number"),
            data["target"].as_f64().expect("model Number"),
            data["binding"].as_f64().expect("model Number"),
            data["mutable"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_call_source(&data["input"])),
            Box::new(decode_java_script_call_source(&data["iteratorKey"])),
            Box::new(decode_java_script_call_source(&data["body"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            data.get("scopeBindings").map(|wire_value| {
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
        ),
        "sourceIteratorLoop" => native::JavaScriptCallSource::SourceIteratorLoop(
            data["slot"].as_f64().expect("model Number"),
            data["target"].as_f64().expect("model Number"),
            data["binding"].as_f64().expect("model Number"),
            data["mutable"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_call_source(&data["body"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            data.get("scopeBindings").map(|wire_value| {
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
        ),
        "sourceIteratorClose" => native::JavaScriptCallSource::SourceIteratorClose(
            data["slot"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "sourceFinally" => native::JavaScriptCallSource::SourceFinally(
            Box::new(decode_java_script_call_source(&data["body"])),
            Box::new(decode_java_script_call_source(&data["finalizer"])),
        ),
        "sourceTry" => native::JavaScriptCallSource::SourceTry(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["body"])),
            Box::new(decode_java_script_call_source(&data["handler"])),
        ),
        "sourceBinding" => native::JavaScriptCallSource::SourceBinding(
            data["identity"].as_f64().expect("model Number"),
        ),
        "sourceLet" => native::JavaScriptCallSource::SourceLet(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["value"])),
            Box::new(decode_java_script_call_source(&data["body"])),
        ),
        "sourceLocalLet" => native::JavaScriptCallSource::SourceLocalLet(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["value"])),
            Box::new(decode_java_script_call_source(&data["body"])),
            data["mutable"].as_bool().expect("model Boolean"),
        ),
        "sourceArgumentRest" => native::JavaScriptCallSource::SourceArgumentRest(
            data["from"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["prototype"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "sourceParameterRest" => native::JavaScriptCallSource::SourceParameterRest(
            data["identity"].as_f64().expect("model Number"),
            data["from"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["prototype"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_call_source(&data["body"])),
        ),
        "sourceParameterDefault" => native::JavaScriptCallSource::SourceParameterDefault(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_call_source(&data["value"])),
            Box::new(decode_java_script_call_source(&data["body"])),
        ),
        "sourceIf" => native::JavaScriptCallSource::SourceIf(
            Box::new(decode_java_script_call_source(&data["condition"])),
            Box::new(decode_java_script_call_source(&data["yes"])),
            Box::new(decode_java_script_call_source(&data["no"])),
        ),
        "sourceNumberBinary" => native::JavaScriptCallSource::SourceNumberBinary(
            String::from_utf16(&wire_text_units(&data["operator"]))
                .expect("model text requires Unicode scalar values in this native profile"),
            Box::new(decode_java_script_call_source(&data["left"])),
            Box::new(decode_java_script_call_source(&data["right"])),
        ),
        "sourceSameValue" => native::JavaScriptCallSource::SourceSameValue(
            Box::new(decode_java_script_call_source(&data["left"])),
            Box::new(decode_java_script_call_source(&data["right"])),
        ),
        "sourceStrictEqual" => native::JavaScriptCallSource::SourceStrictEqual(
            Box::new(decode_java_script_call_source(&data["left"])),
            Box::new(decode_java_script_call_source(&data["right"])),
        ),
        "sourceSnapshotStrictEqual" => native::JavaScriptCallSource::SourceSnapshotStrictEqual(
            Box::new(decode_java_script_call_source(&data["left"])),
            Box::new(decode_java_script_call_source(&data["right"])),
        ),
        "sourcePrimitive" => native::JavaScriptCallSource::SourcePrimitive(
            String::from_utf16(&wire_text_units(&data["operation"]))
                .expect("model text requires Unicode scalar values in this native profile"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source)
                    .collect(),
            )),
        ),
        "sourceRecur" => {
            native::JavaScriptCallSource::SourceRecur(std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source)
                    .collect(),
            )))
        }
        "sourceAllocate" => native::JavaScriptCallSource::SourceAllocate(Box::new(
            decode_java_script_call_source(&data["prototype"]),
        )),
        "sourceClosure" => native::JavaScriptCallSource::SourceClosure(
            native::ml::Big::parse(data["owner"].as_str().expect("model exact integer")),
            data["ordinal"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["captures"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source)
                    .collect(),
            )),
            Box::new(decode_java_script_call_source(&data["receiver"])),
        ),
        "sourceObject" => native::JavaScriptCallSource::SourceObject(
            Box::new(decode_java_script_call_source(&data["prototype"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source_property)
                    .collect(),
            )),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "sourceArrayAccumulation" => native::JavaScriptCallSource::SourceArrayAccumulation(
            Box::new(decode_java_script_call_source(&data["prototype"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["parts"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_array_part)
                    .collect(),
            )),
            Box::new(decode_java_script_call_source(&data["iteratorKey"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "sourceArray" => native::JavaScriptCallSource::SourceArray(
            Box::new(decode_java_script_call_source(&data["prototype"])),
            data["length"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["elements"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source_array_element)
                    .collect(),
            )),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "sourcePropertyKey" => native::JavaScriptCallSource::SourcePropertyKey(
            Box::new(decode_java_script_call_source(&data["input"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "sourcePropertyDefinition" => native::JavaScriptCallSource::SourcePropertyDefinition(
            Box::new(decode_java_script_call_source(&data["target"])),
            Box::new(decode_java_script_call_source(&data["key"])),
            Box::new(decode_java_script_call_source(&data["attributes"])),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        "sourceSet" => native::JavaScriptCallSource::SourceSet(
            Box::new(decode_java_script_call_source(&data["object"])),
            Box::new(decode_java_script_call_source(&data["key"])),
            Box::new(decode_java_script_call_source(&data["value"])),
        ),
        "sourceSnapshotSet" => native::JavaScriptCallSource::SourceSnapshotSet(
            Box::new(decode_java_script_call_source(&data["object"])),
            Box::new(decode_java_script_call_source(&data["key"])),
            Box::new(decode_java_script_call_source(&data["value"])),
        ),
        "sourceStringGet" => native::JavaScriptCallSource::SourceStringGet(
            Box::new(decode_java_script_call_source(&data["input"])),
            Box::new(decode_java_script_call_source(&data["key"])),
            Box::new(decode_java_script_call_source(&data["prototype"])),
        ),
        "sourceGet" => native::JavaScriptCallSource::SourceGet(
            Box::new(decode_java_script_call_source(&data["object"])),
            Box::new(decode_java_script_call_source(&data["key"])),
            data.get("receiver").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_call_source(wire_value))
                },)
            }),
        ),
        "sourceInstanceOf" => native::JavaScriptCallSource::SourceInstanceOf(
            Box::new(decode_java_script_call_source(&data["left"])),
            Box::new(decode_java_script_call_source(&data["right"])),
            Box::new(decode_java_script_call_source(&data["key"])),
        ),
        "sourceDelete" => native::JavaScriptCallSource::SourceDelete(
            Box::new(decode_java_script_call_source(&data["object"])),
            Box::new(decode_java_script_call_source(&data["key"])),
        ),
        "sourceHas" => native::JavaScriptCallSource::SourceHas(
            Box::new(decode_java_script_call_source(&data["object"])),
            Box::new(decode_java_script_call_source(&data["key"])),
        ),
        "sourceCall" => native::JavaScriptCallSource::SourceCall(
            Box::new(decode_java_script_call_source(&data["callee"])),
            Box::new(decode_java_script_call_source(&data["receiver"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source)
                    .collect(),
            )),
        ),
        "sourceSnapshotCall" => native::JavaScriptCallSource::SourceSnapshotCall(
            Box::new(decode_java_script_call_source(&data["callee"])),
            Box::new(decode_java_script_call_source(&data["receiver"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_call_source)
                    .collect(),
            )),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_call_source(value: &native::JavaScriptCallSource) -> Data {
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
pub(super) fn decode_java_script_call_source_switch_clause(
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
pub(super) fn encode_java_script_call_source_switch_clause(
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
pub(super) fn decode_java_script_call_binding_declaration(
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
pub(super) fn encode_java_script_call_binding_declaration(
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
pub(super) fn decode_java_script_call_cell_initializer(
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
pub(super) fn encode_java_script_call_cell_initializer(
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
pub(super) fn decode_java_script_call_source_property(
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
pub(super) fn encode_java_script_call_source_property(
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
pub(super) fn decode_java_script_call_array_part(data: &Data) -> native::JavaScriptCallArrayPart {
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
pub(super) fn encode_java_script_call_array_part(value: &native::JavaScriptCallArrayPart) -> Data {
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
pub(super) fn decode_java_script_call_source_array_element(
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
pub(super) fn encode_java_script_call_source_array_element(
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
pub(super) fn decode_java_script_call_source_compilation(
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
pub(super) fn encode_java_script_call_source_compilation(
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
fn wire_undefined() -> Data {
    json!({"fieldPresence":"undefined"})
}
fn is_wire_undefined(value: &Data) -> bool {
    value.as_object().is_some_and(|fields| {
        fields.len() == 1 && fields.get("fieldPresence").and_then(Data::as_str) == Some("undefined")
    })
}
