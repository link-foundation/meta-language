// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptInstanceOfCall;
use super::JavaScriptInstanceOfState;
use super::JavaScriptInstanceOfTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::accept_java_script_instance_of_method;
use super::accept_java_script_instance_of_prototype;
use super::convert_java_script_semantic_boolean;
use super::finish_java_script_instance_of;
use super::ml_array;
use super::ml_string;
use super::read_java_script_instance_of_state_error;
use super::read_java_script_ordinary_value_error;
use super::test_java_script_standard_object_observation_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_instance_of(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptInstanceOfState,
    completion: self::JavaScriptSemanticCompletion,
) -> self::JavaScriptInstanceOfTransition {
    match state.clone() {
        self::JavaScriptInstanceOfState::InstanceOfState(
            _,
            _,
            _,
            _,
            shared_constructor_field1,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let invalid = {
                let source_ml_source2_logical26455 =
                    self::read_java_script_instance_of_state_error(heap.clone(), state.clone());
                if !source_ml_source2_logical26455.0.is_empty() {
                    source_ml_source2_logical26455.clone()
                } else {
                    self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match completion.clone() {
                            self::JavaScriptSemanticCompletion::NormalCompletion(
                                source_ml_source2_data_property_value26453,
                            ) => {
                                let source_ml_source2_data_property_value26453 =
                                    *source_ml_source2_data_property_value26453;
                                source_ml_source2_data_property_value26453.clone()
                            }
                            self::JavaScriptSemanticCompletion::ThrowCompletion(
                                source_ml_source2_data_property_value26454,
                            ) => {
                                let source_ml_source2_data_property_value26454 =
                                    *source_ml_source2_data_property_value26454;
                                source_ml_source2_data_property_value26454.clone()
                            }
                        },
                    )
                }
            };
            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptInstanceOfTransition::InstanceOfInvalid(
                    Box::new(heap.clone()),
                    invalid.clone(),
                )
            } else {
                if match (
                    match state.clone() {
                        self::JavaScriptInstanceOfState::InstanceOfState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value26456,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value26456
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptInstanceOfCall>,)))
                        .0)
                            .clone(),
                    },
                    None::<()>,
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptInstanceOfTransition::InstanceOfInvalid(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 115, 117,
                            109, 101, 115, 32, 111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116,
                            117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104,
                            105, 108, 100, 32, 99, 97, 108, 108,
                        ]),
                    )
                } else {
                    match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
                            let value = *value;
                            if (shared_constructor_field1
                                == (self::ml_string::Text::from_units(vec![
                                    109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101, 114,
                                ])))
                            {
                                self::accept_java_script_instance_of_method(
                                    heap.clone(),
                                    state.clone(),
                                    value.clone(),
                                )
                            } else {
                                if (shared_constructor_field1
                                    == (self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101,
                                        116, 116, 101, 114,
                                    ])))
                                {
                                    self::accept_java_script_instance_of_prototype(
                                        heap.clone(),
                                        state.clone(),
                                        value.clone(),
                                    )
                                } else {
                                    if (shared_constructor_field1
                                        == (self::ml_string::Text::from_units(vec![
                                            109, 101, 116, 104, 111, 100, 32, 118, 97, 108, 117,
                                            101,
                                        ])))
                                    {
                                        self::finish_java_script_instance_of(
                                            heap.clone(),
                                            state.clone(),
                                            self::convert_java_script_semantic_boolean(
                                                value.clone(),
                                            ),
                                        )
                                    } else {
                                        self::JavaScriptInstanceOfTransition::InstanceOfInvalid(
                                            Box::new(heap.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32,
                                                99, 111, 109, 112, 108, 101, 116, 105, 111, 110,
                                                32, 109, 117, 115, 116, 32, 109, 97, 116, 99, 104,
                                                32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108,
                                                32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99,
                                                97, 108, 108, 32, 112, 104, 97, 115, 101,
                                            ]),
                                        )
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
                            let value_2 = *value_2;
                            self::JavaScriptInstanceOfTransition::InstanceOfThrow(
                                Box::new(heap.clone()),
                                Box::new(value_2.clone()),
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_object_observation_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_object_observation_operation(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_java_script_object_observation_primitive(
    input: self::JavaScriptSemanticValue,
    primitive: Option<self::JavaScriptSemanticValue>,
    kind: &self::ml_string::Text,
) -> Option<self::JavaScriptSemanticValue> {
    if ((match input.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
        }
        self::JavaScriptSemanticValue::Null => {
            self::ml_string::Text::from_units(vec![110, 117, 108, 108])
        }
        self::JavaScriptSemanticValue::Boolean(_) => {
            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
        }
        self::JavaScriptSemanticValue::Number(_) => {
            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
        }
        self::JavaScriptSemanticValue::Integer(_) => {
            self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
        }
        self::JavaScriptSemanticValue::Text(_) => {
            self::ml_string::Text::from_units(vec![116, 101, 120, 116])
        }
        self::JavaScriptSemanticValue::SourceText(_, _) => {
            self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
        }
        self::JavaScriptSemanticValue::Symbol(_) => {
            self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
        }
        self::JavaScriptSemanticValue::Object(_, _) => {
            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
        }
    }) == (*kind))
    {
        Some(input.clone())
    } else {
        if (!(match (primitive.clone(), None::<()>) {
            (Some(_ml_left), Some(_ml_right)) => false,
            (None, None) => true,
            _ => false,
        }) && ((match primitive.clone().expect("a proven present value") {
            self::JavaScriptSemanticValue::Undefined => {
                self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
            }
            self::JavaScriptSemanticValue::Null => {
                self::ml_string::Text::from_units(vec![110, 117, 108, 108])
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
            }
            self::JavaScriptSemanticValue::Number(_) => {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::ml_string::Text::from_units(vec![116, 101, 120, 116])
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::ml_string::Text::from_units(vec![
                    115, 111, 117, 114, 99, 101, 84, 101, 120, 116,
                ])
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
            }
        }) == (*kind)))
        {
            Some(primitive.clone().expect("a proven present value"))
        } else {
            None::<self::JavaScriptSemanticValue>
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_integrity_kind(kind: &self::ml_string::Text) -> bool {
    self::ml_array::includes(
        &{
            let ml_reference_value1430 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101,
                    99, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116,
                ]),
                self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]),
                self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]),
                self::ml_string::Text::from_units(vec![
                    119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116,
                    111, 114, 79, 98, 106, 101, 99, 116,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value1430
        },
        kind,
        0f64,
        false,
        |a, b| a == b,
    )
}
