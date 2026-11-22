use super::{
    Data, decode_java_script_call_array_part, decode_java_script_call_binding_declaration,
    decode_java_script_call_cell_initializer, decode_java_script_call_source_array_element,
    decode_java_script_call_source_property, decode_java_script_call_source_switch_clause,
    decode_java_script_semantic_value, is_wire_undefined, native, wire_text_units,
};

pub(in super::super) fn decode_java_script_call_source(
    data: &Data,
) -> native::JavaScriptCallSource {
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
            native::ml_string::Text::from_units(wire_text_units(&data["mode"])),
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
            native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
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
            native::ml_string::Text::from_units(wire_text_units(&data["operator"])),
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
            native::ml_string::Text::from_units(wire_text_units(&data["operation"])),
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
