use super::{
    Data, decode_java_script_error_array_binding_element,
    decode_java_script_error_array_binding_part, decode_java_script_error_array_element,
    decode_java_script_error_array_part, decode_java_script_error_arrow_template_plan,
    decode_java_script_error_field_assignment, decode_java_script_error_object_property,
    decode_java_script_error_parameter_field, decode_java_script_error_template_part,
    decode_java_script_semantic_value, decode_java_script_source_binding_target,
    decode_java_script_source_lexical_binding, decode_java_script_source_statement,
    encode_java_script_error_array_binding_element, encode_java_script_error_array_binding_part,
    encode_java_script_error_arrow_template_plan, encode_java_script_error_expression,
    encode_java_script_error_field_assignment, encode_java_script_error_parameter_field,
    encode_java_script_source_binding_target, encode_java_script_source_lexical_binding,
    encode_java_script_source_statement, is_wire_undefined, native, wire_number, wire_text,
    wire_text_units, wire_undefined,
};

pub(in super::super) fn decode_java_script_error_module_plan(
    data: &Data,
) -> native::JavaScriptErrorModulePlan {
    match data["$"].as_str().expect("model constructor") {
        "errorModulePlan" => native::JavaScriptErrorModulePlan::ErrorModulePlan(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["functions"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_function_plan)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["exports"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
            data["nextBinding"].as_f64().expect("model Number"),
            data.get("templates").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_error_arrow_template_plan)
                            .collect(),
                    )))
                },)
            }),
            data.get("sharedBindings").map(|wire_value| {
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
            data.get("lexicalBindings").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(std::rc::Rc::new(std::cell::RefCell::new(
                        wire_value
                            .as_array()
                            .expect("model array")
                            .iter()
                            .map(decode_java_script_source_lexical_binding)
                            .collect(),
                    )))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_module_plan(
    value: &native::JavaScriptErrorModulePlan,
) -> Data {
    match value {
        native::JavaScriptErrorModulePlan::ErrorModulePlan(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"errorModulePlan","functions":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_function_plan).collect()) },"exports":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) },"nextBinding":wire_number(*wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "templates".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_error_arrow_template_plan)
                                    .collect(),
                            )
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "sharedBindings".into(),
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
                    "lexicalBindings".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            let wire_reference = wire_present.borrow();
                            Data::Array(
                                wire_reference
                                    .iter()
                                    .map(encode_java_script_source_lexical_binding)
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

pub(in super::super) fn decode_java_script_error_function_plan(
    data: &Data,
) -> native::JavaScriptErrorFunctionPlan {
    match data["$"].as_str().expect("model constructor") {
        "errorConstructorPlan" => native::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(
            data["identity"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["name"])),
            native::ml_string::Text::from_units(wire_text_units(&data["baseClass"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["parameters"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_parameter)
                    .collect(),
            )),
            Box::new(decode_java_script_error_expression(&data["message"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["fields"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_field_assignment)
                    .collect(),
            )),
        ),
        "errorFunctionPlan" => native::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(
            data["identity"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["name"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["parameters"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_parameter)
                    .collect(),
            )),
            Box::new(decode_java_script_error_expression(&data["body"])),
            data["abrupt"].as_bool().expect("model Boolean"),
        ),
        "errorOrdinaryConstructorPlan" => {
            native::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(
                data["identity"].as_f64().expect("model Number"),
                native::ml_string::Text::from_units(wire_text_units(&data["name"])),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["parameters"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_error_parameter)
                        .collect(),
                )),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["statements"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_source_statement)
                        .collect(),
                )),
            )
        }
        "errorMethodFunctionPlan" => native::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(
            data["identity"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["name"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["parameters"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_parameter)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["statements"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
        ),
        "errorMutableStatementFunctionPlan" => {
            native::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(
                data["identity"].as_f64().expect("model Number"),
                native::ml_string::Text::from_units(wire_text_units(&data["name"])),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["parameters"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_error_parameter)
                        .collect(),
                )),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["statements"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_source_statement)
                        .collect(),
                )),
            )
        }
        "errorStatementFunctionPlan" => {
            native::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(
                data["identity"].as_f64().expect("model Number"),
                native::ml_string::Text::from_units(wire_text_units(&data["name"])),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["parameters"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_error_parameter)
                        .collect(),
                )),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["statements"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_source_statement)
                        .collect(),
                )),
            )
        }
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_function_plan(
    value: &native::JavaScriptErrorFunctionPlan,
) -> Data {
    match value {
        native::JavaScriptErrorFunctionPlan::ErrorConstructorPlan(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"errorConstructorPlan","identity":wire_number(*wire_field_0),"name":wire_text(&wire_field_1.0),"baseClass":wire_text(&wire_field_2.0),"parameters":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_parameter).collect()) },"message":encode_java_script_error_expression(wire_field_4),"fields":{ let wire_reference = wire_field_5.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_field_assignment).collect()) }})
        }
        native::JavaScriptErrorFunctionPlan::ErrorFunctionPlan(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"errorFunctionPlan","identity":wire_number(*wire_field_0),"name":wire_text(&wire_field_1.0),"parameters":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_parameter).collect()) },"body":encode_java_script_error_expression(wire_field_3),"abrupt":Data::Bool(*wire_field_4)})
        }
        native::JavaScriptErrorFunctionPlan::ErrorOrdinaryConstructorPlan(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"errorOrdinaryConstructorPlan","identity":wire_number(*wire_field_0),"name":wire_text(&wire_field_1.0),"parameters":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_parameter).collect()) },"statements":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
        native::JavaScriptErrorFunctionPlan::ErrorMethodFunctionPlan(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"errorMethodFunctionPlan","identity":wire_number(*wire_field_0),"name":wire_text(&wire_field_1.0),"parameters":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_parameter).collect()) },"statements":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
        native::JavaScriptErrorFunctionPlan::ErrorMutableStatementFunctionPlan(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"errorMutableStatementFunctionPlan","identity":wire_number(*wire_field_0),"name":wire_text(&wire_field_1.0),"parameters":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_parameter).collect()) },"statements":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
        native::JavaScriptErrorFunctionPlan::ErrorStatementFunctionPlan(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"errorStatementFunctionPlan","identity":wire_number(*wire_field_0),"name":wire_text(&wire_field_1.0),"parameters":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_parameter).collect()) },"statements":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_error_parameter(
    data: &Data,
) -> native::JavaScriptErrorParameter {
    match data["$"].as_str().expect("model constructor") {
        "errorRestParameter" => native::JavaScriptErrorParameter::ErrorRestParameter(
            data["identity"].as_f64().expect("model Number"),
            data.get("pattern").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_error_parameter(wire_value))
                },)
            }),
            data.get("shared").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
        ),
        "errorRequiredParameter" => native::JavaScriptErrorParameter::ErrorRequiredParameter(
            data["identity"].as_f64().expect("model Number"),
            data.get("shared").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
        ),
        "errorDefaultParameter" => native::JavaScriptErrorParameter::ErrorDefaultParameter(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_error_expression(&data["value"])),
            data.get("shared").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
        ),
        "errorObjectParameter" => native::JavaScriptErrorParameter::ErrorObjectParameter(
            data["identity"].as_f64().expect("model Number"),
            data["hasDefault"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_error_expression(&data["value"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["fields"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_parameter_field)
                    .collect(),
            )),
            data.get("rest").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_f64().expect("model Number"))
                },)
            }),
            data.get("restTarget").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_source_binding_target(wire_value))
                },)
            }),
        ),
        "errorArrayParameter" => native::JavaScriptErrorParameter::ErrorArrayParameter(
            data["identity"].as_f64().expect("model Number"),
            data["hasDefault"].as_bool().expect("model Boolean"),
            Box::new(decode_java_script_error_expression(&data["value"])),
            Box::new(decode_java_script_error_expression(&data["iteratorKey"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["elements"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_array_binding_part)
                    .collect(),
            )),
            data.get("rest").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_error_array_binding_element(wire_value))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_parameter(
    value: &native::JavaScriptErrorParameter,
) -> Data {
    match value {
        native::JavaScriptErrorParameter::ErrorRestParameter(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result =
                json!({"$":"errorRestParameter","identity":wire_number(*wire_field_0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_1 {
                fields.insert(
                    "pattern".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_error_parameter(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "shared".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptErrorParameter::ErrorRequiredParameter(wire_field_0, wire_field_1) => {
            let mut result =
                json!({"$":"errorRequiredParameter","identity":wire_number(*wire_field_0)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_1 {
                fields.insert(
                    "shared".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptErrorParameter::ErrorDefaultParameter(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"errorDefaultParameter","identity":wire_number(*wire_field_0),"value":encode_java_script_error_expression(wire_field_1)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "shared".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptErrorParameter::ErrorObjectParameter(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"errorObjectParameter","identity":wire_number(*wire_field_0),"hasDefault":Data::Bool(*wire_field_1),"value":encode_java_script_error_expression(wire_field_2),"fields":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_parameter_field).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "rest".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "restTarget".into(),
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
        native::JavaScriptErrorParameter::ErrorArrayParameter(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"errorArrayParameter","identity":wire_number(*wire_field_0),"hasDefault":Data::Bool(*wire_field_1),"value":encode_java_script_error_expression(wire_field_2),"iteratorKey":encode_java_script_error_expression(wire_field_3),"elements":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_array_binding_part).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "rest".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_error_array_binding_element(wire_present)
                        }
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_error_expression(
    data: &Data,
) -> native::JavaScriptErrorExpression {
    match data["$"].as_str().expect("model constructor") {
        "errorConstant" => native::JavaScriptErrorExpression::ErrorConstant(Box::new(
            decode_java_script_semantic_value(&data["value"]),
        )),
        "errorClosure" => native::JavaScriptErrorExpression::ErrorClosure(
            data["owner"].as_f64().expect("model Number"),
            data["ordinal"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["captures"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| wire_value.as_f64().expect("model Number"))
                    .collect(),
            )),
            data.get("receiverMode").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(native::ml_string::Text::from_units(wire_text_units(
                        &wire_value,
                    )))
                },)
            }),
        ),
        "errorArgumentSpread" => native::JavaScriptErrorExpression::ErrorArgumentSpread(
            Box::new(decode_java_script_error_expression(&data["value"])),
            Box::new(decode_java_script_error_expression(&data["iteratorKey"])),
        ),
        "errorInstanceOf" => native::JavaScriptErrorExpression::ErrorInstanceOf(
            Box::new(decode_java_script_error_expression(&data["left"])),
            Box::new(decode_java_script_error_expression(&data["right"])),
            Box::new(decode_java_script_error_expression(&data["key"])),
        ),
        "errorDelete" => native::JavaScriptErrorExpression::ErrorDelete(Box::new(
            decode_java_script_error_expression(&data["target"]),
        )),
        "errorIn" => native::JavaScriptErrorExpression::ErrorIn(
            Box::new(decode_java_script_error_expression(&data["key"])),
            Box::new(decode_java_script_error_expression(&data["object"])),
        ),
        "errorBinding" => native::JavaScriptErrorExpression::ErrorBinding(
            data["identity"].as_f64().expect("model Number"),
            data.get("immutable").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
        ),
        "errorModuleBinding" => native::JavaScriptErrorExpression::ErrorModuleBinding(
            data["identity"].as_f64().expect("model Number"),
        ),
        "errorThis" => native::JavaScriptErrorExpression::ErrorThis,
        "errorConditional" => native::JavaScriptErrorExpression::ErrorConditional(
            Box::new(decode_java_script_error_expression(&data["condition"])),
            Box::new(decode_java_script_error_expression(&data["yes"])),
            Box::new(decode_java_script_error_expression(&data["no"])),
        ),
        "errorGet" => native::JavaScriptErrorExpression::ErrorGet(
            Box::new(decode_java_script_error_expression(&data["object"])),
            Box::new(decode_java_script_error_expression(&data["key"])),
        ),
        "errorComputedGet" => native::JavaScriptErrorExpression::ErrorComputedGet(
            Box::new(decode_java_script_error_expression(&data["object"])),
            Box::new(decode_java_script_error_expression(&data["key"])),
        ),
        "errorComputedSet" => native::JavaScriptErrorExpression::ErrorComputedSet(
            Box::new(decode_java_script_error_expression(&data["object"])),
            Box::new(decode_java_script_error_expression(&data["key"])),
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        "errorComputedMethodCall" => native::JavaScriptErrorExpression::ErrorComputedMethodCall(
            Box::new(decode_java_script_error_expression(&data["object"])),
            Box::new(decode_java_script_error_expression(&data["key"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_expression)
                    .collect(),
            )),
        ),
        "errorOptionalGet" => native::JavaScriptErrorExpression::ErrorOptionalGet(
            Box::new(decode_java_script_error_expression(&data["object"])),
            Box::new(decode_java_script_error_expression(&data["key"])),
        ),
        "errorSet" => native::JavaScriptErrorExpression::ErrorSet(
            Box::new(decode_java_script_error_expression(&data["object"])),
            Box::new(decode_java_script_error_expression(&data["key"])),
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        "errorModuleCell" => native::JavaScriptErrorExpression::ErrorModuleCell(
            data["identity"].as_f64().expect("model Number"),
        ),
        "errorModuleCellSet" => native::JavaScriptErrorExpression::ErrorModuleCellSet(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        "errorUnary" => native::JavaScriptErrorExpression::ErrorUnary(
            native::ml_string::Text::from_units(wire_text_units(&data["operator"])),
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        "errorUpdate" => native::JavaScriptErrorExpression::ErrorUpdate(
            native::ml_string::Text::from_units(wire_text_units(&data["operator"])),
            Box::new(decode_java_script_error_expression(&data["target"])),
            data["prefix"].as_bool().expect("model Boolean"),
        ),
        "errorCompoundAssignment" => native::JavaScriptErrorExpression::ErrorCompoundAssignment(
            native::ml_string::Text::from_units(wire_text_units(&data["operator"])),
            Box::new(decode_java_script_error_expression(&data["target"])),
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        "errorBinary" => native::JavaScriptErrorExpression::ErrorBinary(
            native::ml_string::Text::from_units(wire_text_units(&data["operator"])),
            Box::new(decode_java_script_error_expression(&data["left"])),
            Box::new(decode_java_script_error_expression(&data["right"])),
            native::ml_string::Text::from_units(wire_text_units(&data["port"])),
        ),
        "errorArray" => native::JavaScriptErrorExpression::ErrorArray(
            native::ml_string::Text::from_units(wire_text_units(&data["factory"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["items"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_expression)
                    .collect(),
            )),
        ),
        "errorArrayAccumulation" => native::JavaScriptErrorExpression::ErrorArrayAccumulation(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["parts"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_array_part)
                    .collect(),
            )),
            Box::new(decode_java_script_error_expression(&data["iteratorKey"])),
        ),
        "errorArrayLiteral" => native::JavaScriptErrorExpression::ErrorArrayLiteral(
            data["length"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["elements"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_array_element)
                    .collect(),
            )),
        ),
        "errorObject" => native::JavaScriptErrorExpression::ErrorObject(std::rc::Rc::new(
            std::cell::RefCell::new(
                data["properties"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_object_property)
                    .collect(),
            ),
        )),
        "errorTemplate" => native::JavaScriptErrorExpression::ErrorTemplate(std::rc::Rc::new(
            std::cell::RefCell::new(
                data["parts"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_template_part)
                    .collect(),
            ),
        )),
        "errorCall" => native::JavaScriptErrorExpression::ErrorCall(
            Box::new(decode_java_script_error_expression(&data["callee"])),
            Box::new(decode_java_script_error_expression(&data["receiver"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_expression)
                    .collect(),
            )),
            data["construct"].as_bool().expect("model Boolean"),
        ),
        "errorMethodCall" => native::JavaScriptErrorExpression::ErrorMethodCall(
            Box::new(decode_java_script_error_expression(&data["object"])),
            Box::new(decode_java_script_error_expression(&data["key"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_expression)
                    .collect(),
            )),
        ),
        "errorFunctionReference" => native::JavaScriptErrorExpression::ErrorFunctionReference(
            data["identity"].as_f64().expect("model Number"),
        ),
        "errorNativeReference" => native::JavaScriptErrorExpression::ErrorNativeReference(
            native::ml_string::Text::from_units(wire_text_units(&data["source"])),
        ),
        "errorNewTarget" => native::JavaScriptErrorExpression::ErrorNewTarget,
        "errorGenericConstruction" => native::JavaScriptErrorExpression::ErrorGenericConstruction(
            Box::new(decode_java_script_error_expression(&data["callee"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["arguments"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_expression)
                    .collect(),
            )),
        ),
        "errorReceiver" => native::JavaScriptErrorExpression::ErrorReceiver,
        "errorOrdinaryConstruction" => {
            native::JavaScriptErrorExpression::ErrorOrdinaryConstruction(
                data["identity"].as_f64().expect("model Number"),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["arguments"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_error_expression)
                        .collect(),
                )),
            )
        }
        "errorCollectionConstruction" => {
            native::JavaScriptErrorExpression::ErrorCollectionConstruction(
                native::ml_string::Text::from_units(wire_text_units(&data["source"])),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["arguments"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_error_expression)
                        .collect(),
                )),
            )
        }
        "errorBindingSet" => native::JavaScriptErrorExpression::ErrorBindingSet(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_error_expression(&data["value"])),
            data.get("immutable").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
        ),
        "errorSpecialNumber" => native::JavaScriptErrorExpression::ErrorSpecialNumber(
            native::ml_string::Text::from_units(wire_text_units(&data["form"])),
        ),
        "errorStrictEqual" => native::JavaScriptErrorExpression::ErrorStrictEqual(
            Box::new(decode_java_script_error_expression(&data["left"])),
            Box::new(decode_java_script_error_expression(&data["right"])),
            data["negate"].as_bool().expect("model Boolean"),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
