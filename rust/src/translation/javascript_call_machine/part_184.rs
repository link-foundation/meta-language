// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::dependency_function8;
use super::execute_java_script_call_integer_primitive;
use super::execute_java_script_call_number_conversion;
use super::execute_java_script_call_primitive;
use super::invert_java_script_call_relational_comparison;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive_addition(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    if (match left.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => true,
        _ => false,
    } || match right.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => true,
        _ => false,
    }) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(
            vec![
                97, 100, 100, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 101, 114, 101,
                100, 32, 111, 98, 106, 101, 99, 116, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105,
                118, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115,
            ],
        ))
    } else {
        if (((match left.clone() {
            self::JavaScriptSemanticValue::Text(_) => true,
            _ => false,
        } || match left.clone() {
            self::JavaScriptSemanticValue::SourceText(_, _) => true,
            _ => false,
        }) || match right.clone() {
            self::JavaScriptSemanticValue::Text(_) => true,
            _ => false,
        }) || match right.clone() {
            self::JavaScriptSemanticValue::SourceText(_, _) => true,
            _ => false,
        }) {
            {
                let first = self::dependency_function8(left.clone());
                match first.clone() {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(first_value) => {
                        let first_value = *first_value;
                        {
                            let second = self::dependency_function8(right.clone());
                            match second.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                    let value = *value;
                                    self::execute_java_script_call_primitive(
                                        &(self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32,
                                            116, 101, 120, 116,
                                        ])),
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            first_value.clone(),
                                            value.clone(),
                                        ])),
                                    )
                                }
                                _ => second.clone(),
                            }
                        }
                    }
                    _ => first.clone(),
                }
            }
        } else {
            if (match left.clone() {
                self::JavaScriptSemanticValue::Symbol(_) => true,
                _ => false,
            } || match right.clone() {
                self::JavaScriptSemanticValue::Symbol(_) => true,
                _ => false,
            }) {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97,
                        32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111,
                        32, 97, 32, 110, 117, 109, 98, 101, 114,
                    ]),
                )
            } else {
                if (match left.clone() {
                    self::JavaScriptSemanticValue::Integer(_) => true,
                    _ => false,
                } && match right.clone() {
                    self::JavaScriptSemanticValue::Integer(_) => true,
                    _ => false,
                }) {
                    self::execute_java_script_call_integer_primitive(
                        &(self::ml_string::Text::from_units(vec![
                            97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115,
                        ])),
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            left.clone(),
                            right.clone(),
                        ])),
                    )
                } else {
                    if (match left.clone() {
                        self::JavaScriptSemanticValue::Integer(_) => true,
                        _ => false,
                    } || match right.clone() {
                        self::JavaScriptSemanticValue::Integer(_) => true,
                        _ => false,
                    }) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveError(
                            self::ml_string::Text::from_units(vec![
                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                67, 97, 110, 110, 111, 116, 32, 109, 105, 120, 32, 66, 105, 103,
                                73, 110, 116, 32, 97, 110, 100, 32, 111, 116, 104, 101, 114, 32,
                                116, 121, 112, 101, 115, 44, 32, 117, 115, 101, 32, 101, 120, 112,
                                108, 105, 99, 105, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105,
                                111, 110, 115,
                            ]),
                        )
                    } else {
                        {
                            let first_2 = self::execute_java_script_call_number_conversion(
                                &(self::ml_string::Text::from_units(vec![
                                    99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116,
                                    105, 118, 101, 32, 110, 117, 109, 98, 101, 114,
                                ])),
                                left.clone(),
                            );
                            match first_2.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                    first_value_2,
                                ) => {
                                    let first_value_2 = *first_value_2;
                                    {
                                        let second_2 =
                                            self::execute_java_script_call_number_conversion(
                                                &(self::ml_string::Text::from_units(vec![
                                                    99, 111, 101, 114, 99, 101, 32, 112, 114, 105,
                                                    109, 105, 116, 105, 118, 101, 32, 110, 117,
                                                    109, 98, 101, 114,
                                                ])),
                                                right.clone(),
                                            );
                                        match second_2.clone() {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                                second_value,
                                            ) => {
                                                let second_value = *second_value;
                                                match first_value_2.clone() {
                                                    self::JavaScriptSemanticValue::Number(first_number_value) => {
                                                        match second_value.clone() {
                                                            self::JavaScriptSemanticValue::Number(value_2) => {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((first_number_value + value_2))))
                                                            }
                                                            _ => {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 108, 116, 115]))
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 108, 116, 115]))
                                                    }
                                                }
                                            }
                                            _ => second_2.clone(),
                                        }
                                    }
                                }
                                _ => first_2.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_numeric_update(
    operation: &self::ml_string::Text,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    if ((((*operation)
        != (self::ml_string::Text::from_units(vec![
            105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32,
            118, 97, 108, 117, 101,
        ])))
        && ((*operation)
            != (self::ml_string::Text::from_units(vec![
                100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99,
                32, 118, 97, 108, 117, 101,
            ]))))
        && ((*operation)
            != (self::ml_string::Text::from_units(vec![
                110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97,
                108, 117, 101,
            ]))))
    {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(
            vec![
                110, 117, 109, 101, 114, 105, 99, 32, 117, 112, 100, 97, 116, 101, 115, 32, 114,
                101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117,
                97, 108, 32, 105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 111, 114, 32, 100,
                101, 99, 114, 101, 109, 101, 110, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111,
                110,
            ],
        ))
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Number(value_value) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                    self::JavaScriptSemanticValue::Number(
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101,
                                114, 105, 99, 32, 118, 97, 108, 117, 101,
                            ])))
                        {
                            (value_value + 1f64)
                        } else {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109,
                                    101, 114, 105, 99, 32, 118, 97, 108, 117, 101,
                                ])))
                            {
                                (value_value - 1f64)
                            } else {
                                (-value_value)
                            }
                        },
                    ),
                ))
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                if ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118,
                        97, 108, 117, 101,
                    ])))
                {
                    self::execute_java_script_call_integer_primitive(
                        &(self::ml_string::Text::from_units(vec![
                            110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114,
                        ])),
                        std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()])),
                    )
                } else {
                    self::execute_java_script_call_integer_primitive(
                        &(if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101,
                                114, 105, 99, 32, 118, 97, 108, 117, 101,
                            ])))
                        {
                            self::ml_string::Text::from_units(vec![
                                97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115,
                            ])
                        } else {
                            self::ml_string::Text::from_units(vec![
                                115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103,
                                101, 114, 115,
                            ])
                        }),
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            value.clone(),
                            self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(1)),
                        ])),
                    )
                }
            }
            _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                self::ml_string::Text::from_units(vec![
                    110, 117, 109, 101, 114, 105, 99, 32, 117, 112, 100, 97, 116, 101, 115, 32,
                    114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117,
                    97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 78, 117, 109, 98,
                    101, 114, 32, 111, 114, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118,
                    101, 114, 115, 105, 111, 110,
                ]),
            ),
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_numeric_binary(
    operation: &self::ml_string::Text,
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    if !(self::ml_array::includes(
        &{
            let ml_reference_value2145 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    115, 117, 98, 116, 114, 97, 99, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32,
                    118, 97, 108, 117, 101, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    109, 117, 108, 116, 105, 112, 108, 121, 32, 110, 117, 109, 101, 114, 105, 99,
                    32, 118, 97, 108, 117, 101, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    100, 105, 118, 105, 100, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118,
                    97, 108, 117, 101, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    114, 101, 97, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 101, 109, 97,
                    105, 110, 100, 101, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    114, 97, 105, 115, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 111,
                    119, 101, 114,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value2145
        },
        operation,
        0f64,
        false,
        |a, b| a == b,
    )) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(
            vec![
                110, 117, 109, 101, 114, 105, 99, 32, 98, 105, 110, 97, 114, 121, 32, 105, 110,
                115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114,
                101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 114, 105,
                116, 104, 109, 101, 116, 105, 99, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
            ],
        ))
    } else {
        if (!(self::ml_array::includes(
            &{
                let ml_reference_value2146 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]),
                    self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]),
                ])))
                .borrow()
                .clone();
                ml_reference_value2146
            },
            &match left.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![
                        117, 110, 100, 101, 102, 105, 110, 101, 100,
                    ])
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
            },
            0f64,
            false,
            |a, b| a == b,
        )) || !(self::ml_array::includes(
            &{
                let ml_reference_value2147 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]),
                    self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114]),
                ])))
                .borrow()
                .clone();
                ml_reference_value2147
            },
            &match right.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![
                        117, 110, 100, 101, 102, 105, 110, 101, 100,
                    ])
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
            },
            0f64,
            false,
            |a, b| a == b,
        ))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                self::ml_string::Text::from_units(vec![
                    110, 117, 109, 101, 114, 105, 99, 32, 97, 114, 105, 116, 104, 109, 101, 116,
                    105, 99, 32, 114, 101, 116, 97, 105, 110, 115, 32, 98, 111, 116, 104, 32, 97,
                    99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84,
                    111, 78, 117, 109, 101, 114, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100,
                    115,
                ]),
            )
        } else {
            if ((match left.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![
                        117, 110, 100, 101, 102, 105, 110, 101, 100,
                    ])
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
            }) != (match right.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![
                        117, 110, 100, 101, 102, 105, 110, 101, 100,
                    ])
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
            })) {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        67, 97, 110, 110, 111, 116, 32, 109, 105, 120, 32, 66, 105, 103, 73, 110,
                        116, 32, 97, 110, 100, 32, 111, 116, 104, 101, 114, 32, 116, 121, 112, 101,
                        115, 44, 32, 117, 115, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                        99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 115,
                    ]),
                )
            } else {
                if (match left.clone() {
                    self::JavaScriptSemanticValue::Integer(_) => true,
                    _ => false,
                } && match right.clone() {
                    self::JavaScriptSemanticValue::Integer(_) => true,
                    _ => false,
                }) {
                    self::execute_java_script_call_integer_primitive(
                        &(if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                115, 117, 98, 116, 114, 97, 99, 116, 32, 110, 117, 109, 101, 114,
                                105, 99, 32, 118, 97, 108, 117, 101, 115,
                            ])))
                        {
                            self::ml_string::Text::from_units(vec![
                                115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103,
                                101, 114, 115,
                            ])
                        } else {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    109, 117, 108, 116, 105, 112, 108, 121, 32, 110, 117, 109, 101,
                                    114, 105, 99, 32, 118, 97, 108, 117, 101, 115,
                                ])))
                            {
                                self::ml_string::Text::from_units(vec![
                                    109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101,
                                    103, 101, 114, 115,
                                ])
                            } else {
                                if ((*operation)
                                    == (self::ml_string::Text::from_units(vec![
                                        100, 105, 118, 105, 100, 101, 32, 110, 117, 109, 101, 114,
                                        105, 99, 32, 118, 97, 108, 117, 101, 115,
                                    ])))
                                {
                                    self::ml_string::Text::from_units(vec![
                                        100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103,
                                        101, 114, 115,
                                    ])
                                } else {
                                    if ((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            114, 101, 97, 100, 32, 110, 117, 109, 101, 114, 105,
                                            99, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114,
                                        ])))
                                    {
                                        self::ml_string::Text::from_units(vec![
                                            114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101,
                                            114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114,
                                        ])
                                    } else {
                                        self::ml_string::Text::from_units(vec![
                                            114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103,
                                            101, 114, 32, 112, 111, 119, 101, 114,
                                        ])
                                    }
                                }
                            }
                        }),
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            left.clone(),
                            right.clone(),
                        ])),
                    )
                } else {
                    match left.clone() {
                        self::JavaScriptSemanticValue::Number(left_value) => match right.clone() {
                            self::JavaScriptSemanticValue::Number(value) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                    self::JavaScriptSemanticValue::Number(
                                        if ((*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                115, 117, 98, 116, 114, 97, 99, 116, 32, 110, 117,
                                                109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101,
                                                115,
                                            ])))
                                        {
                                            (left_value - value)
                                        } else {
                                            if ((*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    109, 117, 108, 116, 105, 112, 108, 121, 32,
                                                    110, 117, 109, 101, 114, 105, 99, 32, 118, 97,
                                                    108, 117, 101, 115,
                                                ])))
                                            {
                                                (left_value * value)
                                            } else {
                                                if ((*operation)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        100, 105, 118, 105, 100, 101, 32, 110, 117,
                                                        109, 101, 114, 105, 99, 32, 118, 97, 108,
                                                        117, 101, 115,
                                                    ])))
                                                {
                                                    (left_value / value)
                                                } else {
                                                    if ((*operation)
                                                        == (self::ml_string::Text::from_units(
                                                            vec![
                                                                114, 101, 97, 100, 32, 110, 117,
                                                                109, 101, 114, 105, 99, 32, 114,
                                                                101, 109, 97, 105, 110, 100, 101,
                                                                114,
                                                            ],
                                                        )))
                                                    {
                                                        (left_value % value)
                                                    } else {
                                                        self::ml_big_integer_conversion::power_java_script_number(left_value,value)
                                                    }
                                                }
                                            }
                                        },
                                    ),
                                ))
                            }
                            _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    110, 117, 109, 101, 114, 105, 99, 32, 97, 114, 105, 116, 104,
                                    109, 101, 116, 105, 99, 32, 114, 101, 116, 97, 105, 110, 115,
                                    32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 78, 117, 109, 98,
                                    101, 114, 32, 111, 114, 32, 66, 105, 103, 73, 110, 116, 32,
                                    111, 112, 101, 114, 97, 110, 100, 115,
                                ]),
                            ),
                        },
                        _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                110, 117, 109, 101, 114, 105, 99, 32, 97, 114, 105, 116, 104, 109,
                                101, 116, 105, 99, 32, 114, 101, 116, 97, 105, 110, 115, 32, 109,
                                97, 116, 99, 104, 105, 110, 103, 32, 78, 117, 109, 98, 101, 114,
                                32, 111, 114, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101,
                                114, 97, 110, 100, 115,
                            ]),
                        ),
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_relational_comparison(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    if (match left.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => true,
        _ => false,
    } || match right.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => true,
        _ => false,
    }) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(
            vec![
                114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 99, 111, 109, 112, 97, 114,
                105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 98, 111, 116,
                104, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105,
                118, 101, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 115,
            ],
        ))
    } else {
        {
            let text_left = (match left.clone() {
                self::JavaScriptSemanticValue::Text(_) => true,
                _ => false,
            } || match left.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => true,
                _ => false,
            });
            {
                let text_right = (match right.clone() {
                    self::JavaScriptSemanticValue::Text(_) => true,
                    _ => false,
                } || match right.clone() {
                    self::JavaScriptSemanticValue::SourceText(_, _) => true,
                    _ => false,
                });
                if (text_left && text_right) {
                    self::execute_java_script_call_primitive(
                        &(self::ml_string::Text::from_units(vec![
                            99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116,
                        ])),
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            left.clone(),
                            right.clone(),
                        ])),
                    )
                } else {
                    if (text_left
                        && match right.clone() {
                            self::JavaScriptSemanticValue::Integer(_) => true,
                            _ => false,
                        })
                    {
                        self::execute_java_script_call_integer_primitive(
                            &(self::ml_string::Text::from_units(vec![
                                99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105,
                                110, 116, 101, 103, 101, 114,
                            ])),
                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                left.clone(),
                                right.clone(),
                            ])),
                        )
                    } else {
                        if (match left.clone() {
                            self::JavaScriptSemanticValue::Integer(_) => true,
                            _ => false,
                        } && text_right)
                        {
                            self::invert_java_script_call_relational_comparison(
                                self::execute_java_script_call_integer_primitive(
                                    &(self::ml_string::Text::from_units(vec![
                                        99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116,
                                        32, 105, 110, 116, 101, 103, 101, 114,
                                    ])),
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        right.clone(),
                                        left.clone(),
                                    ])),
                                ),
                            )
                        } else {
                            {
                                let first = self::execute_java_script_call_primitive(
                                    &(self::ml_string::Text::from_units(vec![
                                        99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105,
                                        116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99,
                                    ])),
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![left.clone()])),
                                );
                                match first.clone() {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                        first_value,
                                    ) => {
                                        let first_value = *first_value;
                                        {
                                            let second = self::execute_java_script_call_primitive(
                                                &(self::ml_string::Text::from_units(vec![
                                                    99, 111, 101, 114, 99, 101, 32, 112, 114, 105,
                                                    109, 105, 116, 105, 118, 101, 32, 110, 117,
                                                    109, 101, 114, 105, 99,
                                                ])),
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    right.clone(),
                                                ])),
                                            );
                                            match second.clone() {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(second_value) => {
                                                    let second_value = *second_value;
                                                    if (match first_value.clone() {
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    } && match second_value.clone() {
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    }) {
                                                        self::execute_java_script_call_integer_primitive(&(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115])), std::rc::Rc::new(std::cell::RefCell::new(vec![first_value.clone(), second_value.clone()])))
                                                    } else {
                                                        if (match first_value.clone() {
                                                            self::JavaScriptSemanticValue::Number(_) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        } && match second_value.clone() {
                                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }) {
                                                            self::execute_java_script_call_integer_primitive(&(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114])), std::rc::Rc::new(std::cell::RefCell::new(vec![first_value.clone(), second_value.clone()])))
                                                        } else {
                                                            if (match first_value.clone() {
                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            } && match second_value.clone() {
                                                                self::JavaScriptSemanticValue::Number(_) => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            }) {
                                                                self::invert_java_script_call_relational_comparison(self::execute_java_script_call_integer_primitive(&(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114])), std::rc::Rc::new(std::cell::RefCell::new(vec![second_value.clone(), first_value.clone()]))))
                                                            } else {
                                                                match first_value.clone() {
                                                                    self::JavaScriptSemanticValue::Number(a_value) => {
                                                                        match second_value.clone() {
                                                                            self::JavaScriptSemanticValue::Number(value) => {
                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (a_value.is_nan() || value.is_nan()) {
                                                                                    2f64
                                                                                } else {
                                                                                    if (a_value < value) {
                                                                                        (-1f64)
                                                                                    } else {
                                                                                        if (a_value > value) {
                                                                                            1f64
                                                                                        } else {
                                                                                            0f64
                                                                                        }
                                                                                    }
                                                                                })))
                                                                            }
                                                                            _ => {
                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                            }
                                                                        }
                                                                    }
                                                                    _ => {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    second.clone()
                                                }
                                            }
                                        }
                                    }
                                    _ => first.clone(),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
