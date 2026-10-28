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
pub(super) fn decode_java_script_call_source(data: &Data) -> native::JavaScriptCallSource {
    match data["$"].as_str().expect("model constructor") {
        "sourceConstant" => native::JavaScriptCallSource::SourceConstant(Box::new(
            decode_java_script_semantic_value(&data["value"]),
        )),
        "sourceReceiver" => native::JavaScriptCallSource::SourceReceiver,
        "sourceArgument" => native::JavaScriptCallSource::SourceArgument(
            data["index"].as_f64().expect("model Number"),
        ),
        "sourceThrow" => native::JavaScriptCallSource::SourceThrow(Box::new(
            decode_java_script_call_source(&data["value"]),
        )),
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
        "sourceIf" => native::JavaScriptCallSource::SourceIf(
            Box::new(decode_java_script_call_source(&data["condition"])),
            Box::new(decode_java_script_call_source(&data["yes"])),
            Box::new(decode_java_script_call_source(&data["no"])),
        ),
        "sourceNumberBinary" => native::JavaScriptCallSource::SourceNumberBinary(
            data["operator"].as_str().expect("model text").to_owned(),
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
        "sourcePrimitive" => native::JavaScriptCallSource::SourcePrimitive(
            data["operation"].as_str().expect("model text").to_owned(),
            data["arguments"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_call_source)
                .collect(),
        ),
        "sourceRecur" => native::JavaScriptCallSource::SourceRecur(
            data["arguments"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_call_source)
                .collect(),
        ),
        "sourceSet" => native::JavaScriptCallSource::SourceSet(
            Box::new(decode_java_script_call_source(&data["object"])),
            Box::new(decode_java_script_call_source(&data["key"])),
            Box::new(decode_java_script_call_source(&data["value"])),
        ),
        "sourceGet" => native::JavaScriptCallSource::SourceGet(
            Box::new(decode_java_script_call_source(&data["object"])),
            Box::new(decode_java_script_call_source(&data["key"])),
        ),
        "sourceCall" => native::JavaScriptCallSource::SourceCall(
            Box::new(decode_java_script_call_source(&data["callee"])),
            Box::new(decode_java_script_call_source(&data["receiver"])),
            data["arguments"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_call_source)
                .collect(),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
pub(super) fn encode_java_script_call_source(value: &native::JavaScriptCallSource) -> Data {
    match value {
        native::JavaScriptCallSource::SourceConstant(wire_field_0) => {
            json!({"$":"sourceConstant","value":encode_java_script_semantic_value(wire_field_0)})
        }
        native::JavaScriptCallSource::SourceReceiver => json!({"$":"sourceReceiver"}),
        native::JavaScriptCallSource::SourceArgument(wire_field_0) => {
            json!({"$":"sourceArgument","index":wire_number(*wire_field_0)})
        }
        native::JavaScriptCallSource::SourceThrow(wire_field_0) => {
            json!({"$":"sourceThrow","value":encode_java_script_call_source(wire_field_0)})
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
        native::JavaScriptCallSource::SourceIf(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sourceIf","condition":encode_java_script_call_source(wire_field_0),"yes":encode_java_script_call_source(wire_field_1),"no":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceNumberBinary(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceNumberBinary","operator":Data::String(wire_field_0.to_owned()),"left":encode_java_script_call_source(wire_field_1),"right":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceSameValue(wire_field_0, wire_field_1) => {
            json!({"$":"sourceSameValue","left":encode_java_script_call_source(wire_field_0),"right":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSource::SourceStrictEqual(wire_field_0, wire_field_1) => {
            json!({"$":"sourceStrictEqual","left":encode_java_script_call_source(wire_field_0),"right":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSource::SourcePrimitive(wire_field_0, wire_field_1) => {
            json!({"$":"sourcePrimitive","operation":Data::String(wire_field_0.to_owned()),"arguments":Data::Array(wire_field_1.iter().map(encode_java_script_call_source).collect())})
        }
        native::JavaScriptCallSource::SourceRecur(wire_field_0) => {
            json!({"$":"sourceRecur","arguments":Data::Array(wire_field_0.iter().map(encode_java_script_call_source).collect())})
        }
        native::JavaScriptCallSource::SourceSet(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sourceSet","object":encode_java_script_call_source(wire_field_0),"key":encode_java_script_call_source(wire_field_1),"value":encode_java_script_call_source(wire_field_2)})
        }
        native::JavaScriptCallSource::SourceGet(wire_field_0, wire_field_1) => {
            json!({"$":"sourceGet","object":encode_java_script_call_source(wire_field_0),"key":encode_java_script_call_source(wire_field_1)})
        }
        native::JavaScriptCallSource::SourceCall(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"sourceCall","callee":encode_java_script_call_source(wire_field_0),"receiver":encode_java_script_call_source(wire_field_1),"arguments":Data::Array(wire_field_2.iter().map(encode_java_script_call_source).collect())})
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
pub(super) fn decode_java_script_call_source_compilation(
    data: &Data,
) -> native::JavaScriptCallSourceCompilation {
    match data["$"].as_str().expect("model constructor") {
        "sourceCompiled" => native::JavaScriptCallSourceCompilation::SourceCompiled(
            data["instructions"]
                .as_array()
                .expect("model array")
                .iter()
                .map(decode_java_script_call_instruction)
                .collect(),
            data["registers"].as_f64().expect("model Number"),
            data["error"].as_str().expect("model text").to_owned(),
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
            json!({"$":"sourceCompiled","instructions":Data::Array(wire_field_0.iter().map(encode_java_script_call_instruction).collect()),"registers":wire_number(*wire_field_1),"error":Data::String(wire_field_2.to_owned())})
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
            data["operator"].as_str().expect("model text").to_owned(),
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
            data["operation"].as_str().expect("model text").to_owned(),
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
            json!({"$":"numberBinary","destination":wire_number(*wire_field_0),"left":wire_number(*wire_field_1),"right":wire_number(*wire_field_2),"operator":Data::String(wire_field_3.to_owned())})
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
            json!({"$":"primitive","destination":wire_number(*wire_field_0),"operation":Data::String(wire_field_1.to_owned()),"arguments":Data::Array(wire_field_2.iter().map(|wire_value|wire_number(*wire_value)).collect())})
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
