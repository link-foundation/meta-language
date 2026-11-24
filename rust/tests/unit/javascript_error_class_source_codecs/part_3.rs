use super::{
    Data, decode_java_script_error_expression, decode_java_script_error_intrinsic_owner,
    decode_java_script_error_parameter, decode_java_script_semantic_value,
    decode_java_script_source_binding_target, encode_java_script_error_expression,
    encode_java_script_error_intrinsic_owner, encode_java_script_error_parameter,
    encode_java_script_error_realm_operations, encode_java_script_semantic_value,
    encode_java_script_source_binding_target, is_wire_undefined, native, wire_number, wire_text,
    wire_text_units, wire_undefined,
};

pub(in super::super) fn decode_java_script_source_statement(
    data: &Data,
) -> native::JavaScriptSourceStatement {
    match data["$"].as_str().expect("model constructor") {
        "sourceForOfStatement" => native::JavaScriptSourceStatement::SourceForOfStatement(
            data["target"].as_f64().expect("model Number"),
            data["binding"].as_f64().expect("model Number"),
            data["mutable"].as_bool().expect("model Boolean"),
            data.get("variable").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
            data.get("bindingTarget").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_source_binding_target(wire_value))
                },)
            }),
            Box::new(decode_java_script_error_expression(&data["input"])),
            Box::new(decode_java_script_error_expression(&data["iteratorKey"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["body"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
            data.get("pattern").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_error_parameter(wire_value))
                },)
            }),
        ),
        "sourceFinallyStatement" => native::JavaScriptSourceStatement::SourceFinallyStatement(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["body"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["finalizer"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
        ),
        "sourceTryStatement" => native::JavaScriptSourceStatement::SourceTryStatement(
            data["identity"].as_f64().expect("model Number"),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["body"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["handler"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
            data.get("pattern").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_error_parameter(wire_value))
                },)
            }),
        ),
        "sourceArrayBindingStatement" => {
            native::JavaScriptSourceStatement::SourceArrayBindingStatement(
                Box::new(decode_java_script_error_parameter(&data["pattern"])),
                Box::new(decode_java_script_error_expression(&data["value"])),
                data["mutable"].as_bool().expect("model Boolean"),
                data.get("variable").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(wire_value.as_bool().expect("model Boolean"))
                    },)
                }),
            )
        }
        "sourceObjectBindingStatement" => {
            native::JavaScriptSourceStatement::SourceObjectBindingStatement(
                Box::new(decode_java_script_error_parameter(&data["pattern"])),
                Box::new(decode_java_script_error_expression(&data["value"])),
                data["mutable"].as_bool().expect("model Boolean"),
                data.get("variable").map(|wire_value| {
                    (if is_wire_undefined(&wire_value) {
                        None
                    } else {
                        Some(wire_value.as_bool().expect("model Boolean"))
                    },)
                }),
            )
        }
        "sourceVariableStatement" => native::JavaScriptSourceStatement::SourceVariableStatement(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_error_expression(&data["value"])),
            data["hasInitializer"].as_bool().expect("model Boolean"),
            data.get("target").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_source_binding_target(wire_value))
                },)
            }),
        ),
        "sourceConstStatement" => native::JavaScriptSourceStatement::SourceConstStatement(
            data["identity"].as_f64().expect("model Number"),
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        "sourceMutableStatement" => native::JavaScriptSourceStatement::SourceMutableStatement(
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
        "sourceExpressionStatement" => {
            native::JavaScriptSourceStatement::SourceExpressionStatement(Box::new(
                decode_java_script_error_expression(&data["value"]),
            ))
        }
        "sourceReturnStatement" => native::JavaScriptSourceStatement::SourceReturnStatement(
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        "sourceThrowStatement" => native::JavaScriptSourceStatement::SourceThrowStatement(
            Box::new(decode_java_script_error_expression(&data["value"])),
        ),
        "sourceIfStatement" => native::JavaScriptSourceStatement::SourceIfStatement(
            Box::new(decode_java_script_error_expression(&data["condition"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["yes"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["no"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
        ),
        "sourceBlockStatement" => native::JavaScriptSourceStatement::SourceBlockStatement(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["statements"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
        ),
        "sourceDeclarationGroupStatement" => {
            native::JavaScriptSourceStatement::SourceDeclarationGroupStatement(std::rc::Rc::new(
                std::cell::RefCell::new(
                    data["statements"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_source_statement)
                        .collect(),
                ),
            ))
        }
        "sourceWhileStatement" => native::JavaScriptSourceStatement::SourceWhileStatement(
            Box::new(decode_java_script_error_expression(&data["condition"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["body"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
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
                    Some(decode_java_script_error_expression(wire_value))
                },)
            }),
            data.get("postTest").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
            data.get("iterationBindings").map(|wire_value| {
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
        "sourceControlBlockStatement" => {
            native::JavaScriptSourceStatement::SourceControlBlockStatement(
                data["target"].as_f64().expect("model Number"),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["body"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_source_statement)
                        .collect(),
                )),
            )
        }
        "sourceControlJumpStatement" => {
            native::JavaScriptSourceStatement::SourceControlJumpStatement(
                data["target"].as_f64().expect("model Number"),
                data["continuing"].as_bool().expect("model Boolean"),
            )
        }
        "sourceSwitchStatement" => native::JavaScriptSourceStatement::SourceSwitchStatement(
            data["target"].as_f64().expect("model Number"),
            Box::new(decode_java_script_error_expression(&data["value"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["clauses"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_switch_clause)
                    .collect(),
            )),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_source_statement(
    value: &native::JavaScriptSourceStatement,
) -> Data {
    match value {
        native::JavaScriptSourceStatement::SourceForOfStatement(
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
            let mut result = json!({"$":"sourceForOfStatement","target":wire_number(*wire_field_0),"binding":wire_number(*wire_field_1),"mutable":Data::Bool(*wire_field_2),"input":encode_java_script_error_expression(wire_field_5),"iteratorKey":encode_java_script_error_expression(wire_field_6),"body":{ let wire_reference = wire_field_7.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "variable".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "bindingTarget".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => {
                            encode_java_script_source_binding_target(wire_present)
                        }
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_8 {
                fields.insert(
                    "pattern".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_error_parameter(wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptSourceStatement::SourceFinallyStatement(wire_field_0, wire_field_1) => {
            json!({"$":"sourceFinallyStatement","body":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) },"finalizer":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
        native::JavaScriptSourceStatement::SourceTryStatement(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"sourceTryStatement","identity":wire_number(*wire_field_0),"body":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) },"handler":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }});
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
            result
        }
        native::JavaScriptSourceStatement::SourceArrayBindingStatement(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"sourceArrayBindingStatement","pattern":encode_java_script_error_parameter(wire_field_0),"value":encode_java_script_error_expression(wire_field_1),"mutable":Data::Bool(*wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "variable".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptSourceStatement::SourceObjectBindingStatement(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"sourceObjectBindingStatement","pattern":encode_java_script_error_parameter(wire_field_0),"value":encode_java_script_error_expression(wire_field_1),"mutable":Data::Bool(*wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "variable".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            result
        }
        native::JavaScriptSourceStatement::SourceVariableStatement(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            let mut result = json!({"$":"sourceVariableStatement","identity":wire_number(*wire_field_0),"value":encode_java_script_error_expression(wire_field_1),"hasInitializer":Data::Bool(*wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
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
        native::JavaScriptSourceStatement::SourceConstStatement(wire_field_0, wire_field_1) => {
            json!({"$":"sourceConstStatement","identity":wire_number(*wire_field_0),"value":encode_java_script_error_expression(wire_field_1)})
        }
        native::JavaScriptSourceStatement::SourceMutableStatement(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            let mut result = json!({"$":"sourceMutableStatement","identity":wire_number(*wire_field_0),"value":encode_java_script_error_expression(wire_field_1)});
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
        native::JavaScriptSourceStatement::SourceExpressionStatement(wire_field_0) => {
            json!({"$":"sourceExpressionStatement","value":encode_java_script_error_expression(wire_field_0)})
        }
        native::JavaScriptSourceStatement::SourceReturnStatement(wire_field_0) => {
            json!({"$":"sourceReturnStatement","value":encode_java_script_error_expression(wire_field_0)})
        }
        native::JavaScriptSourceStatement::SourceThrowStatement(wire_field_0) => {
            json!({"$":"sourceThrowStatement","value":encode_java_script_error_expression(wire_field_0)})
        }
        native::JavaScriptSourceStatement::SourceIfStatement(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceIfStatement","condition":encode_java_script_error_expression(wire_field_0),"yes":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) },"no":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
        native::JavaScriptSourceStatement::SourceBlockStatement(wire_field_0) => {
            json!({"$":"sourceBlockStatement","statements":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
        native::JavaScriptSourceStatement::SourceDeclarationGroupStatement(wire_field_0) => {
            json!({"$":"sourceDeclarationGroupStatement","statements":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
        native::JavaScriptSourceStatement::SourceWhileStatement(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            let mut result = json!({"$":"sourceWhileStatement","condition":encode_java_script_error_expression(wire_field_0),"body":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_2 {
                fields.insert(
                    "target".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "update".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => encode_java_script_error_expression(wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "postTest".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_5 {
                fields.insert(
                    "iterationBindings".into(),
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
        native::JavaScriptSourceStatement::SourceControlBlockStatement(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"sourceControlBlockStatement","target":wire_number(*wire_field_0),"body":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
        native::JavaScriptSourceStatement::SourceControlJumpStatement(
            wire_field_0,
            wire_field_1,
        ) => {
            json!({"$":"sourceControlJumpStatement","target":wire_number(*wire_field_0),"continuing":Data::Bool(*wire_field_1)})
        }
        native::JavaScriptSourceStatement::SourceSwitchStatement(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"sourceSwitchStatement","target":wire_number(*wire_field_0),"value":encode_java_script_error_expression(wire_field_1),"clauses":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_switch_clause).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_source_switch_clause(
    data: &Data,
) -> native::JavaScriptSourceSwitchClause {
    match data["$"].as_str().expect("model constructor") {
        "sourceCaseStatement" => native::JavaScriptSourceSwitchClause::SourceCaseStatement(
            Box::new(decode_java_script_error_expression(&data["value"])),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["body"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
        ),
        "sourceDefaultStatement" => native::JavaScriptSourceSwitchClause::SourceDefaultStatement(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["body"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_source_statement)
                    .collect(),
            )),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_source_switch_clause(
    value: &native::JavaScriptSourceSwitchClause,
) -> Data {
    match value {
        native::JavaScriptSourceSwitchClause::SourceCaseStatement(wire_field_0, wire_field_1) => {
            json!({"$":"sourceCaseStatement","value":encode_java_script_error_expression(wire_field_0),"body":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
        native::JavaScriptSourceSwitchClause::SourceDefaultStatement(wire_field_0) => {
            json!({"$":"sourceDefaultStatement","body":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }})
        }
    }
}

pub(in super::super) fn decode_java_script_error_arrow_template_plan(
    data: &Data,
) -> native::JavaScriptErrorArrowTemplatePlan {
    match data["$"].as_str().expect("model constructor") {
        "errorArrowTemplatePlan" => {
            native::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(
                data["owner"].as_f64().expect("model Number"),
                data["ordinal"].as_f64().expect("model Number"),
                native::ml_string::Text::from_units(wire_text_units(&data["name"])),
                data["length"].as_f64().expect("model Number"),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["parameters"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(decode_java_script_error_parameter)
                        .collect(),
                )),
                std::rc::Rc::new(std::cell::RefCell::new(
                    data["captures"]
                        .as_array()
                        .expect("model array")
                        .iter()
                        .map(|wire_value| wire_value.as_f64().expect("model Number"))
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
                data.get("receiverMode").map(|wire_value| {
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
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_arrow_template_plan(
    value: &native::JavaScriptErrorArrowTemplatePlan,
) -> Data {
    match value {
        native::JavaScriptErrorArrowTemplatePlan::ErrorArrowTemplatePlan(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
            wire_field_6,
            wire_field_7,
        ) => {
            let mut result = json!({"$":"errorArrowTemplatePlan","owner":wire_number(*wire_field_0),"ordinal":wire_number(*wire_field_1),"name":wire_text(&wire_field_2.0),"length":wire_number(*wire_field_3),"parameters":{ let wire_reference = wire_field_4.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_parameter).collect()) },"captures":{ let wire_reference = wire_field_5.borrow(); Data::Array(wire_reference.iter().map(|wire_value|wire_number(*wire_value)).collect()) },"statements":{ let wire_reference = wire_field_6.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_source_statement).collect()) }});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_7 {
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
    }
}

pub(in super::super) fn decode_java_script_source_lexical_binding(
    data: &Data,
) -> native::JavaScriptSourceLexicalBinding {
    match data["$"].as_str().expect("model constructor") {
        "sourceLexicalBinding" => native::JavaScriptSourceLexicalBinding::SourceLexicalBinding(
            data["identity"].as_f64().expect("model Number"),
            native::ml_string::Text::from_units(wire_text_units(&data["name"])),
            data["immutable"].as_bool().expect("model Boolean"),
            data.get("hoisted").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_bool().expect("model Boolean"))
                },)
            }),
            data.get("parameter").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(wire_value.as_f64().expect("model Number"))
                },)
            }),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_source_lexical_binding(
    value: &native::JavaScriptSourceLexicalBinding,
) -> Data {
    match value {
        native::JavaScriptSourceLexicalBinding::SourceLexicalBinding(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            let mut result = json!({"$":"sourceLexicalBinding","identity":wire_number(*wire_field_0),"name":wire_text(&wire_field_1.0),"immutable":Data::Bool(*wire_field_2)});
            let fields = result.as_object_mut().expect("model constructor object");
            if let Some((wire_present,)) = wire_field_3 {
                fields.insert(
                    "hoisted".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => Data::Bool(*wire_present),
                    },
                );
            }
            if let Some((wire_present,)) = wire_field_4 {
                fields.insert(
                    "parameter".into(),
                    match wire_present {
                        None => wire_undefined(),
                        Some(wire_present) => wire_number(*wire_present),
                    },
                );
            }
            result
        }
    }
}

pub(in super::super) fn decode_java_script_error_module_registry(
    data: &Data,
) -> native::JavaScriptErrorModuleRegistry {
    match data["$"].as_str().expect("model constructor") {
        "errorModuleRegistry" => native::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(
            std::rc::Rc::new(std::cell::RefCell::new(
                data["owners"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_function_owner)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["operations"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_realm_operations)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["intrinsics"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(decode_java_script_error_intrinsic_owner)
                    .collect(),
            )),
            std::rc::Rc::new(std::cell::RefCell::new(
                data["realms"]
                    .as_array()
                    .expect("model array")
                    .iter()
                    .map(|wire_value| {
                        native::ml::Big::parse(wire_value.as_str().expect("model exact integer"))
                    })
                    .collect(),
            )),
            Box::new(decode_java_script_semantic_value(&data["exoticKey"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_module_registry(
    value: &native::JavaScriptErrorModuleRegistry,
) -> Data {
    match value {
        native::JavaScriptErrorModuleRegistry::ErrorModuleRegistry(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
        ) => {
            json!({"$":"errorModuleRegistry","owners":{ let wire_reference = wire_field_0.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_function_owner).collect()) },"operations":{ let wire_reference = wire_field_1.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_realm_operations).collect()) },"intrinsics":{ let wire_reference = wire_field_2.borrow(); Data::Array(wire_reference.iter().map(encode_java_script_error_intrinsic_owner).collect()) },"realms":{ let wire_reference = wire_field_3.borrow(); Data::Array(wire_reference.iter().map(|wire_value|Data::String(wire_value.to_string())).collect()) },"exoticKey":encode_java_script_semantic_value(wire_field_4)})
        }
    }
}

pub(in super::super) fn decode_java_script_error_function_owner(
    data: &Data,
) -> native::JavaScriptErrorFunctionOwner {
    match data["$"].as_str().expect("model constructor") {
        "errorConstructorOwner" => native::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(
            data["source"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["prototype"])),
            Box::new(decode_java_script_semantic_value(&data["baseConstructor"])),
            native::ml::Big::parse(data["baseRealm"].as_str().expect("model exact integer")),
        ),
        "ordinaryConstructorOwner" => {
            native::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(
                data["source"].as_f64().expect("model Number"),
                native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
                native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
                Box::new(decode_java_script_semantic_value(&data["prototype"])),
            )
        }
        "errorFunctionOwner" => native::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(
            data["source"].as_f64().expect("model Number"),
            native::ml::Big::parse(data["identity"].as_str().expect("model exact integer")),
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}

pub(in super::super) fn encode_java_script_error_function_owner(
    value: &native::JavaScriptErrorFunctionOwner,
) -> Data {
    match value {
        native::JavaScriptErrorFunctionOwner::ErrorConstructorOwner(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
            wire_field_4,
            wire_field_5,
        ) => {
            json!({"$":"errorConstructorOwner","source":wire_number(*wire_field_0),"identity":Data::String(wire_field_1.to_string()),"realm":Data::String(wire_field_2.to_string()),"prototype":encode_java_script_semantic_value(wire_field_3),"baseConstructor":encode_java_script_semantic_value(wire_field_4),"baseRealm":Data::String(wire_field_5.to_string())})
        }
        native::JavaScriptErrorFunctionOwner::OrdinaryConstructorOwner(
            wire_field_0,
            wire_field_1,
            wire_field_2,
            wire_field_3,
        ) => {
            json!({"$":"ordinaryConstructorOwner","source":wire_number(*wire_field_0),"identity":Data::String(wire_field_1.to_string()),"realm":Data::String(wire_field_2.to_string()),"prototype":encode_java_script_semantic_value(wire_field_3)})
        }
        native::JavaScriptErrorFunctionOwner::ErrorFunctionOwner(
            wire_field_0,
            wire_field_1,
            wire_field_2,
        ) => {
            json!({"$":"errorFunctionOwner","source":wire_number(*wire_field_0),"identity":Data::String(wire_field_1.to_string()),"realm":Data::String(wire_field_2.to_string())})
        }
    }
}

pub(in super::super) fn decode_java_script_error_realm_operations(
    data: &Data,
) -> native::JavaScriptErrorRealmOperations {
    match data["$"].as_str().expect("model constructor") {
        "errorRealmOperations" => native::JavaScriptErrorRealmOperations::ErrorRealmOperations(
            native::ml::Big::parse(data["realm"].as_str().expect("model exact integer")),
            Box::new(decode_java_script_semantic_value(&data["objectPrototype"])),
            data.get("arrayPrototype").map(|wire_value| {
                (if is_wire_undefined(&wire_value) {
                    None
                } else {
                    Some(decode_java_script_semantic_value(wire_value))
                },)
            }),
            data.get("functionPrototype").map(|wire_value| {
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
            Box::new(decode_java_script_semantic_value(&data["get"])),
            Box::new(decode_java_script_semantic_value(&data["assign"])),
        ),
        _ => panic!("unregistered actual model constructor"),
    }
}
