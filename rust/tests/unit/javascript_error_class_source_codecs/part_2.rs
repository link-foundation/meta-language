use super::{
    Data, decode_java_script_error_expression, decode_java_script_error_parameter,
    encode_java_script_error_parameter, is_wire_undefined, native, wire_number, wire_text,
    wire_text_units, wire_undefined,
};

pub(in super::super) fn encode_java_script_error_expression(
    value: &native::JavaScriptErrorExpression,
) -> Data {
    match value {
        native::JavaScriptErrorExpression::ErrorConstant(wire_field_0) => {
            json!({"$":"errorConstant","value":encode_java_script_semantic_value(wire_field_0)})
        }
        native::JavaScriptErrorExpression::ErrorClosure(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"errorClosure","owner":wire_number(*wire_field_0),"ordinal":wire_number(*wire_field_1),"captures":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "receiverMode".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            result
        }
        native::JavaScriptErrorExpression::ErrorArgumentSpread(wire_field_0, wire_field_1) => {
            json!({"$":"errorArgumentSpread","value":encode_java_script_error_expression(wire_field_0),"iteratorKey":encode_java_script_error_expression(wire_field_1)})
        }
        native::JavaScriptErrorExpression::ErrorInstanceOf(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorInstanceOf","left":encode_java_script_error_expression(wire_field_0),"right":encode_java_script_error_expression(wire_field_1),"key":encode_java_script_error_expression(wire_field_2)})
        }
        native::JavaScriptErrorExpression::ErrorDelete(wire_field_0) => {
            json!({"$":"errorDelete","target":encode_java_script_error_expression(wire_field_0)})
        }
        native::JavaScriptErrorExpression::ErrorIn(wire_field_0, wire_field_1) => {
            json!({"$":"errorIn","key":encode_java_script_error_expression(wire_field_0),"object":encode_java_script_error_expression(wire_field_1)})
        }
        native::JavaScriptErrorExpression::ErrorBinding(wire_field_0, wire_field_1) => {
            let mut result = json!({"$":"errorBinding","identity":wire_number(*wire_field_0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_1 {
                fields.insert(
                    "immutable".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptErrorExpression::ErrorModuleBinding(wire_field_0) => {
            json!({"$":"errorModuleBinding","identity":wire_number(*wire_field_0)})
        }
        native::JavaScriptErrorExpression::ErrorThis => json!({"$":"errorThis"}),
        native::JavaScriptErrorExpression::ErrorConditional(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorConditional","condition":encode_java_script_error_expression(wire_field_0),"yes":encode_java_script_error_expression(wire_field_1),"no":encode_java_script_error_expression(wire_field_2)})
        }
        native::JavaScriptErrorExpression::ErrorGet(wire_field_0, wire_field_1) => {
            json!({"$":"errorGet","object":encode_java_script_error_expression(wire_field_0),"key":encode_java_script_error_expression(wire_field_1)})
        }
        native::JavaScriptErrorExpression::ErrorComputedGet(wire_field_0, wire_field_1) => {
            json!({"$":"errorComputedGet","object":encode_java_script_error_expression(wire_field_0),"key":encode_java_script_error_expression(wire_field_1)})
        }
        native::JavaScriptErrorExpression::ErrorComputedSet(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorComputedSet","object":encode_java_script_error_expression(wire_field_0),"key":encode_java_script_error_expression(wire_field_1),"value":encode_java_script_error_expression(wire_field_2)})
        }
        native::JavaScriptErrorExpression::ErrorComputedMethodCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorComputedMethodCall","object":encode_java_script_error_expression(wire_field_0),"key":encode_java_script_error_expression(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_expression).collect()) }})
        }
        native::JavaScriptErrorExpression::ErrorOptionalGet(wire_field_0, wire_field_1) => {
            json!({"$":"errorOptionalGet","object":encode_java_script_error_expression(wire_field_0),"key":encode_java_script_error_expression(wire_field_1)})
        }
        native::JavaScriptErrorExpression::ErrorSet(wire_field_0, wire_field_1, wire_field_2) => {
            json!({"$":"errorSet","object":encode_java_script_error_expression(wire_field_0),"key":encode_java_script_error_expression(wire_field_1),"value":encode_java_script_error_expression(wire_field_2)})
        }
        native::JavaScriptErrorExpression::ErrorModuleCell(wire_field_0) => {
            json!({"$":"errorModuleCell","identity":wire_number(*wire_field_0)})
        }
        native::JavaScriptErrorExpression::ErrorModuleCellSet(wire_field_0, wire_field_1) => {
            json!({"$":"errorModuleCellSet","identity":wire_number(*wire_field_0),"value":encode_java_script_error_expression(wire_field_1)})
        }
        native::JavaScriptErrorExpression::ErrorUnary(wire_field_0, wire_field_1) => {
            json!({"$":"errorUnary","operator":wire_text(&wire_field_0.0),"value":encode_java_script_error_expression(wire_field_1)})
        }
        native::JavaScriptErrorExpression::ErrorUpdate(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorUpdate","operator":wire_text(&wire_field_0.0),"target":encode_java_script_error_expression(wire_field_1),"prefix":Data::Bool(*wire_field_2)})
        }
        native::JavaScriptErrorExpression::ErrorCompoundAssignment(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorCompoundAssignment","operator":wire_text(&wire_field_0.0),"target":encode_java_script_error_expression(wire_field_1),"value":encode_java_script_error_expression(wire_field_2)})
        }
        native::JavaScriptErrorExpression::ErrorBinary(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"errorBinary","operator":wire_text(&wire_field_0.0),"left":encode_java_script_error_expression(wire_field_1),"right":encode_java_script_error_expression(wire_field_2),"port":wire_text(&wire_field_3.0)})
        }
        native::JavaScriptErrorExpression::ErrorArray(wire_field_0, wire_field_1) => {
            json!({"$":"errorArray","factory":wire_text(&wire_field_0.0),"items":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_expression).collect()) }})
        }
        native::JavaScriptErrorExpression::ErrorArrayAccumulation(wire_field_0, wire_field_1) => {
            json!({"$":"errorArrayAccumulation","parts":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_array_part).collect()) },"iteratorKey":encode_java_script_error_expression(wire_field_1)})
        }
        native::JavaScriptErrorExpression::ErrorArrayLiteral(wire_field_0, wire_field_1) => {
            json!({"$":"errorArrayLiteral","length":wire_number(*wire_field_0),"elements":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_array_element).collect()) }})
        }
        native::JavaScriptErrorExpression::ErrorObject(wire_field_0) => {
            json!({"$":"errorObject","properties":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_object_property).collect()) }})
        }
        native::JavaScriptErrorExpression::ErrorTemplate(wire_field_0) => {
            json!({"$":"errorTemplate","parts":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_template_part).collect()) }})
        }
        native::JavaScriptErrorExpression::ErrorCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"errorCall","callee":encode_java_script_error_expression(wire_field_0),"receiver":encode_java_script_error_expression(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_expression).collect()) },"construct":Data::Bool(*wire_field_3)})
        }
        native::JavaScriptErrorExpression::ErrorMethodCall(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorMethodCall","object":encode_java_script_error_expression(wire_field_0),"key":encode_java_script_error_expression(wire_field_1),"arguments":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_expression).collect()) }})
        }
        native::JavaScriptErrorExpression::ErrorFunctionReference(wire_field_0) => {
            json!({"$":"errorFunctionReference","identity":wire_number(*wire_field_0)})
        }
        native::JavaScriptErrorExpression::ErrorNativeReference(wire_field_0) => {
            json!({"$":"errorNativeReference","source":wire_text(&wire_field_0.0)})
        }
        native::JavaScriptErrorExpression::ErrorNewTarget => json!({"$":"errorNewTarget"}),
        native::JavaScriptErrorExpression::ErrorGenericConstruction(wire_field_0, wire_field_1) => {
            json!({"$":"errorGenericConstruction","callee":encode_java_script_error_expression(wire_field_0),"arguments":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_expression).collect()) }})
        }
        native::JavaScriptErrorExpression::ErrorReceiver => json!({"$":"errorReceiver"}),
        native::JavaScriptErrorExpression::ErrorOrdinaryConstruction(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"errorOrdinaryConstruction","identity":wire_number(*wire_field_0),"arguments":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_expression).collect()) }})
        }
        native::JavaScriptErrorExpression::ErrorCollectionConstruction(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"errorCollectionConstruction","source":wire_text(&wire_field_0.0),"arguments":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_expression).collect()) }})
        }
        native::JavaScriptErrorExpression::ErrorBindingSet(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"errorBindingSet","identity":wire_number(*wire_field_0),"value":encode_java_script_error_expression(wire_field_1)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "immutable".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptErrorExpression::ErrorSpecialNumber(wire_field_0) => {
            json!({"$":"errorSpecialNumber","form":wire_text(&wire_field_0.0)})
        }
        native::JavaScriptErrorExpression::ErrorStrictEqual(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorStrictEqual","left":encode_java_script_error_expression(wire_field_0),"right":encode_java_script_error_expression(wire_field_1),"negate":Data::Bool(*wire_field_2)})
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
            native::ml_string::Text::from_units(wire_text_units(&data["profile"])),
            native::ml_string::Text::from_units(wire_text_units(&data["text"])),
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
            json!({"$":"sourceText","profile":wire_text(&wire_field_0.0),"text":wire_text(&wire_field_1.0)})
        }
        native::JavaScriptSemanticValue::Symbol(wire_field_0) => {
            json!({"$":"symbol","identity":Data::String(wire_field_0.to_string())})
        }
        native::JavaScriptSemanticValue::Object(wire_field_0, wire_field_1) => {
            json!({"$":"object","identity":Data::String(wire_field_0.to_string()),"callable":Data::Bool(*wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_error_array_part(
    data: &Data,
) -> native::JavaScriptErrorArrayPart {
    match data["$"].as_str().expect("model constructor") {
        "errorArrayValue" => native::JavaScriptErrorArrayPart::ErrorArrayValue(Box::new(
            decode_java_script_error_expression(&data["value"]),
        )),
        "errorArraySpread" => native::JavaScriptErrorArrayPart::ErrorArraySpread(Box::new(
            decode_java_script_error_expression(&data["value"]),
        )),
        "errorArrayHole" => native::JavaScriptErrorArrayPart::ErrorArrayHole,
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_array_part(
    value: &native::JavaScriptErrorArrayPart,
) -> Data {
    match value {
        native::JavaScriptErrorArrayPart::ErrorArrayValue(wire_field_0) => {
            json!({"$":"errorArrayValue","value":encode_java_script_error_expression(wire_field_0)})
        }
        native::JavaScriptErrorArrayPart::ErrorArraySpread(wire_field_0) => {
            json!({"$":"errorArraySpread","value":encode_java_script_error_expression(wire_field_0)})
        }
        native::JavaScriptErrorArrayPart::ErrorArrayHole => json!({"$":"errorArrayHole"}),
    }
}

pub(in super::super) fn decode_java_script_error_array_element(
    data: &Data,
) -> native::JavaScriptErrorArrayElement {
    match data["$"].as_str().expect("model constructor") {
        "errorArrayElement" => native::JavaScriptErrorArrayElement::ErrorArrayElement(
            data["index"].as_f64().expect("model Number"),
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_array_element(
    value: &native::JavaScriptErrorArrayElement,
) -> Data {
    match value {
        native::JavaScriptErrorArrayElement::ErrorArrayElement(wire_field_0, wire_field_1) => {
            json!({"$":"errorArrayElement","index":wire_number(*wire_field_0),"value":encode_java_script_error_expression(wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_error_object_property(
    data: &Data,
) -> native::JavaScriptErrorObjectProperty {
    match data["$"].as_str().expect("model constructor") {
        "errorObjectProperty" => native::JavaScriptErrorObjectProperty::ErrorObjectProperty(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["key"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        "errorObjectAccessor" => native::JavaScriptErrorObjectProperty::ErrorObjectAccessor(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["key"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
            Box::new(decode_java_script_error_expression(&data["value"])),
            native::ml_string::Text::from_units(wire_text_units(&data["kind"])),
        ),
        "errorLiteralPrototype" => native::JavaScriptErrorObjectProperty::ErrorLiteralPrototype(
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        "errorComputedObjectProperty" => {
            native::JavaScriptErrorObjectProperty::ErrorComputedObjectProperty(
                Box::new(decode_java_script_error_expression(&data["key"])),
                Box::new(decode_java_script_error_expression(&data["value"])),
                data.get("inferName").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(wire_value.as_bool().expect("model Boolean"))
                    },)
                }),
                data.get("kind").map(|wire_value| {
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
        "errorObjectSpread" => native::JavaScriptErrorObjectProperty::ErrorObjectSpread(Box::new(
            decode_java_script_error_expression(&data["input"]),
        )),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_object_property(
    value: &native::JavaScriptErrorObjectProperty,
) -> Data {
    match value {
        native::JavaScriptErrorObjectProperty::ErrorObjectProperty(wire_field_0, wire_field_1) => {
            json!({"$":"errorObjectProperty","key":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) },"value":encode_java_script_error_expression(wire_field_1)})
        }
        native::JavaScriptErrorObjectProperty::ErrorObjectAccessor(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorObjectAccessor","key":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) },"value":encode_java_script_error_expression(wire_field_1),"kind":wire_text(&wire_field_2.0)})
        }
        native::JavaScriptErrorObjectProperty::ErrorLiteralPrototype(wire_field_0) => {
            json!({"$":"errorLiteralPrototype","value":encode_java_script_error_expression(wire_field_0)})
        }
        native::JavaScriptErrorObjectProperty::ErrorComputedObjectProperty(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"errorComputedObjectProperty","key":encode_java_script_error_expression(wire_field_0),"value":encode_java_script_error_expression(wire_field_1)});
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
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "kind".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_text(&wire_present.0),
                    },
                );
            }
            result
        }
        native::JavaScriptErrorObjectProperty::ErrorObjectSpread(wire_field_0) => {
            json!({"$":"errorObjectSpread","input":encode_java_script_error_expression(wire_field_0)})
        }
    }
}

pub(in super::super) fn decode_java_script_error_template_part(
    data: &Data,
) -> native::JavaScriptErrorTemplatePart {
    match data["$"].as_str().expect("model constructor") {
        "errorTemplateText" => native::JavaScriptErrorTemplatePart::ErrorTemplateText(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["units"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
        ),
        "errorTemplateValue" => native::JavaScriptErrorTemplatePart::ErrorTemplateValue(Box::new(
            decode_java_script_error_expression(&data["value"]),
        )),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_template_part(
    value: &native::JavaScriptErrorTemplatePart,
) -> Data {
    match value {
        native::JavaScriptErrorTemplatePart::ErrorTemplateText(wire_field_0) => {
            json!({"$":"errorTemplateText","units":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) }})
        }
        native::JavaScriptErrorTemplatePart::ErrorTemplateValue(wire_field_0) => {
            json!({"$":"errorTemplateValue","value":encode_java_script_error_expression(wire_field_0)})
        }
    }
}

pub(in super::super) fn decode_java_script_error_parameter_field(
    data: &Data,
) -> native::JavaScriptErrorParameterField {
    match data["$"].as_str().expect("model constructor") {
        "errorParameterField" => native::JavaScriptErrorParameterField::ErrorParameterField(
            data["identity"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["key"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
            data["hasDefault"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_error_expression(&data["value"])),
            data.get("keyExpression").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_error_expression(wire_value))
                },)
            }),
            data.get("pattern").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_error_parameter(wire_value))
                },)
            }),
            data.get("target").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_source_binding_target(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_parameter_field(
    value: &native::JavaScriptErrorParameterField,
) -> Data {
    match value {
        native::JavaScriptErrorParameterField::ErrorParameterField(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
        ) => {
            let mut result = json!({"$":"errorParameterField","identity":wire_number(*wire_field_0),"key":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) },"hasDefault":Data::Bool(*wire_field_2),"value":encode_java_script_error_expression(wire_field_3)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "keyExpression".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_error_expression(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "pattern".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_error_parameter(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_6 {
                fields.insert(
                    "target".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_source_binding_target(wire_present)
                        }
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_source_binding_target(
    data: &Data,
) -> native::JavaScriptSourceBindingTarget {
    match data["$"].as_str().expect("model constructor") {
        "sourceCatchBindingTarget" => {
            native::JavaScriptSourceBindingTarget::SourceCatchBindingTarget(
                data["identity"].as_f64().expect("model Number"),
                data["cell"].as_bool().expect("model Boolean"),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_source_binding_target(
    value: &native::JavaScriptSourceBindingTarget,
) -> Data {
    match value {
        native::JavaScriptSourceBindingTarget::SourceCatchBindingTarget(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"sourceCatchBindingTarget","identity":wire_number(*wire_field_0),"cell":Data::Bool(*wire_field_1)})
        }
    }
}

pub(in super::super) fn decode_java_script_error_array_binding_part(
    data: &Data,
) -> native::JavaScriptErrorArrayBindingPart {
    match data["$"].as_str().expect("model constructor") {
        "errorArrayBindingElement" => {
            native::JavaScriptErrorArrayBindingPart::ErrorArrayBindingElement(
                data["identity"].as_f64().expect("model Number"),
                data["hasDefault"].as_bool().expect("model Boolean"),
                Box::new(decode_java_script_error_expression(&data["value"])),
                data.get("pattern").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_error_parameter(wire_value))
                    },)
                }),
                data.get("target").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_source_binding_target(wire_value))
                    },)
                }),
            )
        }
        "errorArrayElision" => native::JavaScriptErrorArrayBindingPart::ErrorArrayElision,
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_array_binding_part(
    value: &native::JavaScriptErrorArrayBindingPart,
) -> Data {
    match value {
        native::JavaScriptErrorArrayBindingPart::ErrorArrayBindingElement(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            let mut result = json!({"$":"errorArrayBindingElement","identity":wire_number(*wire_field_0),"hasDefault":Data::Bool(*wire_field_1),"value":encode_java_script_error_expression(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "pattern".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_error_parameter(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "target".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_source_binding_target(wire_present)
                        }
                    },
                );
            }
            result
        }
        native::JavaScriptErrorArrayBindingPart::ErrorArrayElision => {
            json!({"$":"errorArrayElision"})
        }
    }
}

pub(in super::super) fn decode_java_script_error_array_binding_element(
    data: &Data,
) -> native::JavaScriptErrorArrayBindingElement {
    match data["$"].as_str().expect("model constructor") {
        "errorArrayBindingElement" => {
            native::JavaScriptErrorArrayBindingElement::ErrorArrayBindingElement(
                data["identity"].as_f64().expect("model Number"),
                data["hasDefault"].as_bool().expect("model Boolean"),
                Box::new(decode_java_script_error_expression(&data["value"])),
                data.get("pattern").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_error_parameter(wire_value))
                    },)
                }),
                data.get("target").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(decode_java_script_source_binding_target(wire_value))
                    },)
                }),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_array_binding_element(
    value: &native::JavaScriptErrorArrayBindingElement,
) -> Data {
    match value {
        native::JavaScriptErrorArrayBindingElement::ErrorArrayBindingElement(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            let mut result = json!({"$":"errorArrayBindingElement","identity":wire_number(*wire_field_0),"hasDefault":Data::Bool(*wire_field_1),"value":encode_java_script_error_expression(wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "pattern".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_error_parameter(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "target".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_source_binding_target(wire_present)
                        }
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_error_field_assignment(
    data: &Data,
) -> native::JavaScriptErrorFieldAssignment {
    match data["$"].as_str().expect("model constructor") {
        "errorFieldAssignment" => native::JavaScriptErrorFieldAssignment::ErrorFieldAssignment(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["name"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_field_assignment(
    value: &native::JavaScriptErrorFieldAssignment,
) -> Data {
    match value {
        native::JavaScriptErrorFieldAssignment::ErrorFieldAssignment(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"errorFieldAssignment","name":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) },"value":encode_java_script_error_expression(wire_field_1)})
        }
    }
}
