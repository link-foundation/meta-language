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
pub(super) fn decode_java_script_ordinary_heap(data: &Data) -> native::JavaScriptOrdinaryHeap {
    match data["$"].as_str().expect("model constructor") {
        "ordinaryHeap" => native::JavaScriptOrdinaryHeap::OrdinaryHeap(
            data["objects"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_ordinary_object)
                .collect(),
            data["symbols"]
                .as_array()
                .expect("model array")
                .iter()
                .map(|wire_value| {
                    native::ml::Big::parse(wire_value.as_str().expect("model exact integer"))
                })
                .collect(),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_ordinary_heap(value: &native::JavaScriptOrdinaryHeap) -> Data {
    match value {
        native::JavaScriptOrdinaryHeap::OrdinaryHeap(wire_field_0, wire_field_1) => {
            json!({"$":"ordinaryHeap","objects":Data::Array(wire_field_0.iter().map(encode_java_script_ordinary_object).collect()),"symbols":Data::Array(wire_field_1.iter().map(|wire_value|Data::String(wire_value.to_string())).collect())})
        }
    }
}
pub(super) fn decode_java_script_ordinary_object(data: &Data) -> native::JavaScriptOrdinaryObject {
    match data["$"].as_str().expect("model constructor") {
        "ordinaryObject" => native::JavaScriptOrdinaryObject::OrdinaryObject(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            data["extensible"].as_bool().expect("model Boolean"),
            data["callable"].as_bool().expect("model Boolean"),
            data["properties"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_ordinary_property)
                .collect(),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_ordinary_object(value: &native::JavaScriptOrdinaryObject) -> Data {
    match value {
        native::JavaScriptOrdinaryObject::OrdinaryObject(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"ordinaryObject","identity":Data::String(wire_field_0.to_string()),"prototype":encode_java_script_semantic_value(wire_field_1),"extensible":Data::Bool(*wire_field_2),"callable":Data::Bool(*wire_field_3),"properties":Data::Array(wire_field_4.iter().map(encode_java_script_ordinary_property).collect())})
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
        "text" => native::JavaScriptSemanticValue::Text(
            data["units"]
                .as_array()
                .expect("model array")
                .iter()
                .map(|wire_value| wire_value.as_f64().expect("model Number"))
                .collect(),
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
            json!({"$":"text","units":Data::Array(wire_field_0.iter().map(|wire_value|wire_number(*wire_value)).collect())})
        }
        native::JavaScriptSemanticValue::Symbol(wire_field_0) => {
            json!({"$":"symbol","identity":Data::String(wire_field_0.to_string())})
        }
        native::JavaScriptSemanticValue::Object(wire_field_0, wire_field_1) => {
            json!({"$":"object","identity":Data::String(wire_field_0.to_string()),"callable":Data::Bool(*wire_field_1)})
        }
    }
}
pub(super) fn decode_java_script_ordinary_property(
    data: &Data,
) -> native::JavaScriptOrdinaryProperty {
    match data["$"].as_str().expect("model constructor") {
        "ordinaryProperty" => native::JavaScriptOrdinaryProperty::OrdinaryProperty(
            Box::new(decode_java_script_semantic_value(&data["key"])),
            (
                data["descriptor"]["configurable"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["configurablePresent"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["enumerable"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["enumerablePresent"]
                    .as_bool()
                    .expect("model Boolean"),
                decode_java_script_semantic_value(&data["descriptor"]["getter"]),
                data["descriptor"]["getterPresent"]
                    .as_bool()
                    .expect("model Boolean"),
                decode_java_script_semantic_value(&data["descriptor"]["setter"]),
                data["descriptor"]["setterPresent"]
                    .as_bool()
                    .expect("model Boolean"),
                decode_java_script_semantic_value(&data["descriptor"]["value"]),
                data["descriptor"]["valuePresent"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["writable"]
                    .as_bool()
                    .expect("model Boolean"),
                data["descriptor"]["writablePresent"]
                    .as_bool()
                    .expect("model Boolean"),
            ),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_ordinary_property(
    value: &native::JavaScriptOrdinaryProperty,
) -> Data {
    match value {
        native::JavaScriptOrdinaryProperty::OrdinaryProperty(wire_field_0, wire_field_1) => {
            json!({"$":"ordinaryProperty","key":encode_java_script_semantic_value(wire_field_0),"descriptor":json!({"configurable":Data::Bool(wire_field_1.0),"configurablePresent":Data::Bool(wire_field_1.1),"enumerable":Data::Bool(wire_field_1.2),"enumerablePresent":Data::Bool(wire_field_1.3),"getter":encode_java_script_semantic_value(&wire_field_1.4),"getterPresent":Data::Bool(wire_field_1.5),"setter":encode_java_script_semantic_value(&wire_field_1.6),"setterPresent":Data::Bool(wire_field_1.7),"value":encode_java_script_semantic_value(&wire_field_1.8),"valuePresent":Data::Bool(wire_field_1.9),"writable":Data::Bool(wire_field_1.10),"writablePresent":Data::Bool(wire_field_1.11)})})
        }
    }
}
pub(super) fn decode_java_script_call_program(data: &Data) -> native::JavaScriptCallProgram {
    match data["$"].as_str().expect("model constructor") {
        "callProgram" => native::JavaScriptCallProgram::CallProgram(
            data["functions"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_call_function)
                .collect(),
            data["realms"]
                .as_array()
                .expect("model array")
                .iter()
                .map(|wire_value| {
                    native::ml::Big::parse(wire_value.as_str().expect("model exact integer"))
                })
                .collect(),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_call_program(value: &native::JavaScriptCallProgram) -> Data {
    match value {
        native::JavaScriptCallProgram::CallProgram(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"callProgram","functions":Data::Array(wire_field_0.iter().map(encode_java_script_call_function).collect()),"realms":Data::Array(wire_field_1.iter().map(|wire_value|Data::String(wire_value.to_string())).collect()),"exoticKey":encode_java_script_semantic_value(wire_field_2)})
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
            data["captures"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_semantic_value)
                .collect(),
            data["instructions"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_call_instruction)
                .collect(),
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
        ) => {
            json!({"$":"callFunction","identity":Data::String(wire_field_0.to_string()),"realm":Data::String(wire_field_1.to_string()),"receiverMode":wire_text(&wire_field_2.0),"lexicalReceiver":encode_java_script_semantic_value(wire_field_3),"parameters":wire_number(*wire_field_4),"registers":wire_number(*wire_field_5),"captures":Data::Array(wire_field_6.iter().map(encode_java_script_semantic_value).collect()),"instructions":Data::Array(wire_field_7.iter().map(encode_java_script_call_instruction).collect())})
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
            data["arguments"]
                .as_array()
                .expect("model array")
                .iter()
                .map(|wire_value| wire_value.as_f64().expect("model Number"))
                .collect(),
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
            data["arguments"]
                .as_array()
                .expect("model array")
                .iter()
                .map(|wire_value| wire_value.as_f64().expect("model Number"))
                .collect(),
        ),
        "ordinaryGet" => native::JavaScriptCallInstruction::OrdinaryGet(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            data["receiver"].as_f64().expect("model Number"),
        ),
        "ordinarySet" => native::JavaScriptCallInstruction::OrdinarySet(
            data["destination"].as_f64().expect("model Number"),
            data["object"].as_f64().expect("model Number"),
            data["key"].as_f64().expect("model Number"),
            data["value"].as_f64().expect("model Number"),
            data["receiver"].as_f64().expect("model Number"),
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
            json!({"$":"call","destination":wire_number(*wire_field_0),"callee":wire_number(*wire_field_1),"receiver":wire_number(*wire_field_2),"arguments":Data::Array(wire_field_3.iter().map(|wire_value|wire_number(*wire_value)).collect())})
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
            json!({"$":"primitive","destination":wire_number(*wire_field_0),"operation":wire_text(&wire_field_1.0),"arguments":Data::Array(wire_field_2.iter().map(|wire_value|wire_number(*wire_value)).collect())})
        }
        native::JavaScriptCallInstruction::OrdinaryGet(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"ordinaryGet","destination":wire_number(*wire_field_0),"object":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"receiver":wire_number(*wire_field_3)})
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
        ) => {
            json!({"$":"propertyDefinition","destination":wire_number(*wire_field_0),"target":wire_number(*wire_field_1),"key":wire_number(*wire_field_2),"attributes":wire_number(*wire_field_3),"realm":Data::String(wire_field_4.to_string())})
        }
    }
}
pub(super) fn decode_java_script_call_machine(data: &Data) -> native::JavaScriptCallMachine {
    match data["$"].as_str().expect("model constructor") {
        "callMachine" => native::JavaScriptCallMachine::CallMachine(
            Box::new(decode_java_script_ordinary_heap(&data["heap"])),
            data["frames"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_call_frame)
                .collect(),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_call_machine(value: &native::JavaScriptCallMachine) -> Data {
    match value {
        native::JavaScriptCallMachine::CallMachine(wire_field_0, wire_field_1) => {
            json!({"$":"callMachine","heap":encode_java_script_ordinary_heap(wire_field_0),"frames":Data::Array(wire_field_1.iter().map(encode_java_script_call_frame).collect())})
        }
    }
}
pub(super) fn decode_java_script_call_frame(data: &Data) -> native::JavaScriptCallFrame {
    match data["$"].as_str().expect("model constructor") {
        "callFrame" => native::JavaScriptCallFrame::CallFrame(
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            data["position"].as_f64().expect("model Number"),
            data["registers"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_semantic_value)
                .collect(),
            data["arguments"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_semantic_value)
                .collect(),
            data["handlers"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_call_handler)
                .collect(),
            Box::new(decode_java_script_call_pending(&data["pending"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_call_frame(value: &native::JavaScriptCallFrame) -> Data {
    match value {
        native::JavaScriptCallFrame::CallFrame(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"callFrame","identity":Data::String(wire_field_0.to_string()),"position":wire_number(*wire_field_1),"registers":Data::Array(wire_field_2.iter().map(encode_java_script_semantic_value).collect()),"arguments":Data::Array(wire_field_3.iter().map(encode_java_script_semantic_value).collect()),"handlers":Data::Array(wire_field_4.iter().map(encode_java_script_call_handler).collect()),"pending":encode_java_script_call_pending(wire_field_5)})
        }
    }
}
pub(super) fn decode_java_script_call_handler(data: &Data) -> native::JavaScriptCallHandler {
    match data["$"].as_str().expect("model constructor") {
        "callHandler" => native::JavaScriptCallHandler::CallHandler(
            data["target"].as_f64().expect("model Number"),
            data["destination"].as_f64().expect("model Number"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_call_handler(value: &native::JavaScriptCallHandler) -> Data {
    match value {
        native::JavaScriptCallHandler::CallHandler(wire_field_0, wire_field_1) => {
            json!({"$":"callHandler","target":wire_number(*wire_field_0),"destination":wire_number(*wire_field_1)})
        }
    }
}
pub(super) fn decode_java_script_call_pending(data: &Data) -> native::JavaScriptCallPending {
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
        "waitSourceError" => native::JavaScriptCallPending::WaitSourceError(
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
            Box::new(decode_java_script_semantic_value(&data["message"])),
            data["messagePresent"].as_bool().expect("model Boolean"),
        ),
        "waitError" => native::JavaScriptCallPending::WaitError(
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            native::ml_string::Text::from_units(wire_text_units(&data["errorClass"])),
            native::ml_string::Text::from_units(wire_text_units(&data["reason"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_call_pending(value: &native::JavaScriptCallPending) -> Data {
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
        native::JavaScriptCallPending::WaitSourceError(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"waitSourceError","realm":Data::String(wire_field_0.to_string()),"errorClass":wire_text(&wire_field_1.0),"message":encode_java_script_semantic_value(wire_field_2),"messagePresent":Data::Bool(*wire_field_3)})
        }
        native::JavaScriptCallPending::WaitError(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"waitError","realm":Data::String(wire_field_0.to_string()),"errorClass":wire_text(&wire_field_1.0),"reason":wire_text(&wire_field_2.0)})
        }
    }
}
pub(super) fn decode_java_script_primitive_conversion_state(
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
pub(super) fn encode_java_script_primitive_conversion_state(
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
pub(super) fn decode_java_script_property_definition_state(
    data: &Data,
) -> native::JavaScriptPropertyDefinitionState {
    match data["$"].as_str().expect("model constructor") {
        "propertyDefinitionKey" => {
            native::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                Box::new(decode_java_script_semantic_value(&data["target"])),
                Box::new(decode_java_script_semantic_value(&data["attributes"])),
                Box::new(decode_java_script_primitive_conversion_state(
                    &data["conversion"],
                )),
            )
        }
        "propertyDefinitionDescriptor" => {
            native::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                Box::new(decode_java_script_semantic_value(&data["target"])),
                Box::new(decode_java_script_semantic_value(&data["key"])),
                Box::new(decode_java_script_descriptor_conversion_state(
                    &data["conversion"],
                )),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_property_definition_state(
    value: &native::JavaScriptPropertyDefinitionState,
) -> Data {
    match value {
        native::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"propertyDefinitionKey","target":encode_java_script_semantic_value(wire_field_0),"attributes":encode_java_script_semantic_value(wire_field_1),"conversion":encode_java_script_primitive_conversion_state(wire_field_2)})
        }
        native::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"propertyDefinitionDescriptor","target":encode_java_script_semantic_value(wire_field_0),"key":encode_java_script_semantic_value(wire_field_1),"conversion":encode_java_script_descriptor_conversion_state(wire_field_2)})
        }
    }
}
pub(super) fn decode_java_script_descriptor_conversion_state(
    data: &Data,
) -> native::JavaScriptDescriptorConversionState {
    match data["$"].as_str().expect("model constructor") {
        "descriptorConversion" => {
            native::JavaScriptDescriptorConversionState::DescriptorConversion(
                Box::new(decode_java_script_semantic_value(&data["receiver"])),
                data["next"].as_f64().expect("model Number"),
                (
                    data["descriptor"]["configurable"]
                        .as_bool()
                        .expect("model Boolean"),
                    data["descriptor"]["configurablePresent"]
                        .as_bool()
                        .expect("model Boolean"),
                    data["descriptor"]["enumerable"]
                        .as_bool()
                        .expect("model Boolean"),
                    data["descriptor"]["enumerablePresent"]
                        .as_bool()
                        .expect("model Boolean"),
                    decode_java_script_semantic_value(&data["descriptor"]["getter"]),
                    data["descriptor"]["getterPresent"]
                        .as_bool()
                        .expect("model Boolean"),
                    decode_java_script_semantic_value(&data["descriptor"]["setter"]),
                    data["descriptor"]["setterPresent"]
                        .as_bool()
                        .expect("model Boolean"),
                    decode_java_script_semantic_value(&data["descriptor"]["value"]),
                    data["descriptor"]["valuePresent"]
                        .as_bool()
                        .expect("model Boolean"),
                    data["descriptor"]["writable"]
                        .as_bool()
                        .expect("model Boolean"),
                    data["descriptor"]["writablePresent"]
                        .as_bool()
                        .expect("model Boolean"),
                ),
                data["pending"].as_bool().expect("model Boolean"),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_descriptor_conversion_state(
    value: &native::JavaScriptDescriptorConversionState,
) -> Data {
    match value {
        native::JavaScriptDescriptorConversionState::DescriptorConversion(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"descriptorConversion","receiver":encode_java_script_semantic_value(wire_field_0),"next":wire_number(*wire_field_1),"descriptor":json!({"configurable":Data::Bool(wire_field_2.0),"configurablePresent":Data::Bool(wire_field_2.1),"enumerable":Data::Bool(wire_field_2.2),"enumerablePresent":Data::Bool(wire_field_2.3),"getter":encode_java_script_semantic_value(&wire_field_2.4),"getterPresent":Data::Bool(wire_field_2.5),"setter":encode_java_script_semantic_value(&wire_field_2.6),"setterPresent":Data::Bool(wire_field_2.7),"value":encode_java_script_semantic_value(&wire_field_2.8),"valuePresent":Data::Bool(wire_field_2.9),"writable":Data::Bool(wire_field_2.10),"writablePresent":Data::Bool(wire_field_2.11)}),"pending":Data::Bool(*wire_field_3)})
        }
    }
}
pub(super) fn decode_java_script_number_call_state(
    data: &Data,
) -> native::JavaScriptNumberCallState {
    match data["$"].as_str().expect("model constructor") {
        "numberProtocolState" => native::JavaScriptNumberCallState::NumberProtocolState(Box::new(
            decode_java_script_primitive_conversion_state(&data["state"]),
        )),
        "numberPrimitiveValue" => native::JavaScriptNumberCallState::NumberPrimitiveValue(
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_number_call_state(
    value: &native::JavaScriptNumberCallState,
) -> Data {
    match value {
        native::JavaScriptNumberCallState::NumberProtocolState(wire_field_0) => {
            json!({"$":"numberProtocolState","state":encode_java_script_primitive_conversion_state(wire_field_0)})
        }
        native::JavaScriptNumberCallState::NumberPrimitiveValue(wire_field_0) => {
            json!({"$":"numberPrimitiveValue","value":encode_java_script_semantic_value(wire_field_0)})
        }
    }
}
pub(super) fn decode_java_script_text_call_state(data: &Data) -> native::JavaScriptTextCallState {
    match data["$"].as_str().expect("model constructor") {
        "textProtocolState" => native::JavaScriptTextCallState::TextProtocolState(Box::new(
            decode_java_script_primitive_conversion_state(&data["state"]),
        )),
        "textPrimitiveValue" => native::JavaScriptTextCallState::TextPrimitiveValue(Box::new(
            decode_java_script_semantic_value(&data["value"]),
        )),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_text_call_state(value: &native::JavaScriptTextCallState) -> Data {
    match value {
        native::JavaScriptTextCallState::TextProtocolState(wire_field_0) => {
            json!({"$":"textProtocolState","state":encode_java_script_primitive_conversion_state(wire_field_0)})
        }
        native::JavaScriptTextCallState::TextPrimitiveValue(wire_field_0) => {
            json!({"$":"textPrimitiveValue","value":encode_java_script_semantic_value(wire_field_0)})
        }
    }
}
pub(super) fn decode_java_script_call_machine_transition(
    data: &Data,
) -> native::JavaScriptCallMachineTransition {
    match data["$"].as_str().expect("model constructor") {
        "machineContinue" => native::JavaScriptCallMachineTransition::MachineContinue(Box::new(
            decode_java_script_call_machine(&data["machine"]),
        )),
        "machineCompleted" => native::JavaScriptCallMachineTransition::MachineCompleted(
            Box::new(decode_java_script_ordinary_heap(&data["heap"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
        ),
        "machineThrow" => native::JavaScriptCallMachineTransition::MachineThrow(
            Box::new(decode_java_script_ordinary_heap(&data["heap"])),
            Box::new(decode_java_script_semantic_value(&data["value"])),
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
pub(super) fn encode_java_script_call_machine_transition(
    value: &native::JavaScriptCallMachineTransition,
) -> Data {
    match value {
        native::JavaScriptCallMachineTransition::MachineContinue(wire_field_0) => {
            json!({"$":"machineContinue","machine":encode_java_script_call_machine(wire_field_0)})
        }
        native::JavaScriptCallMachineTransition::MachineCompleted(wire_field_0, wire_field_1) => {
            json!({"$":"machineCompleted","heap":encode_java_script_ordinary_heap(wire_field_0),"value":encode_java_script_semantic_value(wire_field_1)})
        }
        native::JavaScriptCallMachineTransition::MachineThrow(wire_field_0, wire_field_1) => {
            json!({"$":"machineThrow","heap":encode_java_script_ordinary_heap(wire_field_0),"value":encode_java_script_semantic_value(wire_field_1)})
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
