// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::character_utf16_text;
use super::code_point_utf16_text;
use super::compare_utf16_text;
use super::ends_utf16_text;
use super::execute_java_script_call_primitive;
use super::includes_utf16_text;
use super::index_utf16_text;
use super::last_index_utf16_text;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::plan_utf16_text_substitution;
use super::relative_character_utf16_text;
use super::replace_utf16_text;
use super::slice_utf16_text;
use super::source_ml_source1_temporary_read_java_script_call_text_replacement_work_error_loop1400;
use super::starts_utf16_text;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_position(
    operation: &self::ml_string::Text,
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let second = ({
            let source_ml_source2_array_read_index32683 = 1f64;
            {
                let ml_reference_array2076 = values.clone();
                let ml_reference_position2076 = source_ml_source2_array_read_index32683;
                let ml_reference_projection2076 = self::ml_array::optional_at(
                    &*ml_reference_array2076.borrow(),
                    self::ml_array::number_index(ml_reference_position2076),
                );
                ml_reference_projection2076
            }
        })
        .clone()
        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let third = ({
                let source_ml_source2_array_read_index32685 = 2f64;
                {
                    let ml_reference_array2077 = values.clone();
                    let ml_reference_position2077 = source_ml_source2_array_read_index32685;
                    let ml_reference_projection2077 = self::ml_array::optional_at(
                        &*ml_reference_array2077.borrow(),
                        self::ml_array::number_index(ml_reference_position2077),
                    );
                    ml_reference_projection2077
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            if ((((*operation)
                == (self::ml_string::Text::from_units(vec![
                    114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111, 100, 101, 32, 112, 111,
                    105, 110, 116,
                ])))
                || ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99,
                        116, 101, 114,
                    ]))))
                || ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101,
                        120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114,
                    ]))))
            {
                match second.clone() {
                    self::JavaScriptSemanticValue::Number(second_value) => {
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114,
                                97, 99, 116, 101, 114,
                            ])))
                        {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                self::JavaScriptSemanticValue::Text(self::character_utf16_text(
                                    text.clone(),
                                    second_value,
                                )),
                            ))
                        } else {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101,
                                    32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101,
                                    114,
                                ])))
                            {
                                {
                                    let result = self::relative_character_utf16_text(
                                        text.clone(),
                                        second_value,
                                    );
                                    {
                                        let value = {
                                            let source_ml_source2_array_read_index32687 = 0f64;
                                            {
                                                let ml_reference_array2078 = result.clone();
                                                let ml_reference_position2078 =
                                                    source_ml_source2_array_read_index32687;
                                                let ml_reference_projection2078 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array2078.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position2078,
                                                        ),
                                                    );
                                                ml_reference_projection2078
                                            }
                                        };
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                            Box::new(
                                                if match (value.clone(), None::<()>) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                } {
                                                    self::JavaScriptSemanticValue::Undefined
                                                } else {
                                                    self::JavaScriptSemanticValue::Text(
                                                        value
                                                            .clone()
                                                            .expect("a proven present value"),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                }
                            } else {
                                {
                                    let result_2 =
                                        self::code_point_utf16_text(text.clone(), second_value);
                                    {
                                        let value_2 = {
                                            let source_ml_source2_array_read_index32689 = 0f64;
                                            {
                                                let ml_reference_array2079 = result_2.clone();
                                                let ml_reference_position2079 =
                                                    source_ml_source2_array_read_index32689;
                                                let ml_reference_projection2079 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array2079.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position2079,
                                                        ),
                                                    );
                                                ml_reference_projection2079
                                            }
                                        };
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                            Box::new(
                                                if {
                                                    let source_ml_source2_comparison_operand32691 =
                                                        None::<()>;
                                                    match (
                                                        value_2.clone(),
                                                        source_ml_source2_comparison_operand32691
                                                            .clone(),
                                                    ) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    }
                                                } {
                                                    self::JavaScriptSemanticValue::Undefined
                                                } else {
                                                    self::JavaScriptSemanticValue::Number(
                                                        value_2
                                                            .clone()
                                                            .expect("a proven present value"),
                                                    )
                                                },
                                            ),
                                        )
                                    }
                                }
                            }
                        }
                    }
                    _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                        self::ml_string::Text::from_units(vec![
                            116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115,
                            32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32,
                            97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101,
                            100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110,
                            100,
                        ]),
                    ),
                }
            } else {
                if ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        115, 108, 105, 99, 101, 32, 116, 101, 120, 116,
                    ])))
                {
                    match second.clone() {
                        self::JavaScriptSemanticValue::Number(second_value_2) => {
                            match third.clone() {
                                self::JavaScriptSemanticValue::Number(value_3) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                        self::JavaScriptSemanticValue::Text(
                                            self::slice_utf16_text(
                                                text.clone(),
                                                second_value_2,
                                                value_3,
                                            ),
                                        ),
                                    ))
                                }
                                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98,
                                        111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114,
                                        101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97,
                                        108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32,
                                        78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97,
                                        110, 100, 115,
                                    ]),
                                ),
                            }
                        }
                        _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117,
                                110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104,
                                101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110,
                                118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32,
                                111, 112, 101, 114, 97, 110, 100, 115,
                            ]),
                        ),
                    }
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                        self::ml_string::Text::from_units(vec![
                            97, 32, 116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110,
                            32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115,
                            32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103,
                            105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116,
                            105, 111, 110,
                        ]),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_pair(
    operation: &self::ml_string::Text,
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let second = ({
            let source_ml_source2_array_read_index32693 = 1f64;
            {
                let ml_reference_array2080 = values.clone();
                let ml_reference_position2080 = source_ml_source2_array_read_index32693;
                let ml_reference_projection2080 = self::ml_array::optional_at(
                    &*ml_reference_array2080.borrow(),
                    self::ml_array::number_index(ml_reference_position2080),
                );
                ml_reference_projection2080
            }
        })
        .clone()
        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let third = ({
                let source_ml_source2_array_read_index32695 = 2f64;
                {
                    let ml_reference_array2081 = values.clone();
                    let ml_reference_position2081 = source_ml_source2_array_read_index32695;
                    let ml_reference_projection2081 = self::ml_array::optional_at(
                        &*ml_reference_array2081.borrow(),
                        self::ml_array::number_index(ml_reference_position2081),
                    );
                    ml_reference_projection2081
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match second.clone() {
                self::JavaScriptSemanticValue::Text(needle) => {
                    if ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120,
                            116,
                        ])))
                    {
                        if (({
                            let ml_reference_array2082 = text.clone();
                            let ml_reference_projection2082 =
                                ml_reference_array2082.borrow().len() as f64;
                            ml_reference_projection2082
                        } + {
                            let ml_reference_array2083 = needle.clone();
                            let ml_reference_projection2083 =
                                ml_reference_array2083.borrow().len() as f64;
                            ml_reference_projection2083
                        }) > (1048576f64))
                        {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120,
                                    116, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101,
                                    101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105,
                                    99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116,
                                    32, 98, 117, 100, 103, 101, 116,
                                ]),
                            )
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                    std::cell::RefCell::new(self::ml_array::append(
                                        self::ml_array::append(Vec::<f64>::new(), {
                                            let ml_reference_value2084 = text.borrow().clone();
                                            ml_reference_value2084
                                        }),
                                        {
                                            let ml_reference_value2085 = needle.borrow().clone();
                                            ml_reference_value2085
                                        },
                                    )),
                                )),
                            ))
                        }
                    } else {
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116,
                            ])))
                        {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                self::JavaScriptSemanticValue::Number(self::compare_utf16_text(
                                    text.clone(),
                                    needle.clone(),
                                )),
                            ))
                        } else {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 112, 114, 101,
                                    102, 105, 120,
                                ])))
                            {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                    self::JavaScriptSemanticValue::Boolean(
                                        self::starts_utf16_text(text.clone(), needle.clone()),
                                    ),
                                ))
                            } else {
                                if ((*operation)
                                    == (self::ml_string::Text::from_units(vec![
                                        116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 115, 117,
                                        102, 102, 105, 120,
                                    ])))
                                {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                        self::JavaScriptSemanticValue::Boolean(
                                            self::ends_utf16_text(text.clone(), needle.clone()),
                                        ),
                                    ))
                                } else {
                                    if (((self::ml_math::max(
                                        0f64,
                                        ({
                                            let ml_reference_array2086 = text.clone();
                                            let ml_reference_projection2086 =
                                                ml_reference_array2086.borrow().len() as f64;
                                            ml_reference_projection2086
                                        } - {
                                            let ml_reference_array2087 = needle.clone();
                                            let ml_reference_projection2087 =
                                                ml_reference_array2087.borrow().len() as f64;
                                            ml_reference_projection2087
                                        }),
                                    ) + 1f64)
                                        * {
                                            let ml_reference_array2088 = needle.clone();
                                            let ml_reference_projection2088 =
                                                ml_reference_array2088.borrow().len() as f64;
                                            ml_reference_projection2088
                                        })
                                        > (67108864f64))
                                    {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                                            self::ml_string::Text::from_units(vec![
                                                112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                                                116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104,
                                                32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116,
                                                115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                                99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32,
                                                119, 111, 114, 107, 32, 113, 117, 111, 116, 97,
                                            ]),
                                        )
                                    } else {
                                        if ((*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                116, 101, 115, 116, 32, 116, 101, 120, 116, 32,
                                                105, 110, 99, 108, 117, 115, 105, 111, 110,
                                            ])))
                                        {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                                Box::new(self::JavaScriptSemanticValue::Boolean(
                                                    self::includes_utf16_text(
                                                        text.clone(),
                                                        needle.clone(),
                                                    ),
                                                )),
                                            )
                                        } else {
                                            if (((*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    102, 105, 110, 100, 32, 116, 101, 120, 116,
                                                ])))
                                                || ((*operation)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        102, 105, 110, 100, 32, 108, 97, 115, 116,
                                                        32, 116, 101, 120, 116,
                                                    ]))))
                                            {
                                                match third.clone() {
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if ((*operation) == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]))) {
                                                            self::index_utf16_text(text.clone(), needle.clone(), value)
                                                        } else {
                                                            self::last_index_utf16_text(text.clone(), needle.clone(), value)
                                                        })))
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                }
                                            } else {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32,
                        99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101,
                        113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108,
                        32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32,
                        111, 112, 101, 114, 97, 110, 100,
                    ]),
                ),
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_replacement_operands(
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    all: bool,
) -> self::JavaScriptCallPrimitiveResult {
    if {
        let ml_values2089 = {
            let ml_reference_value2090 = values.borrow().clone();
            ml_reference_value2090
        };
        let ml_callback2089 =
            std::rc::Rc::new(move |value: self::JavaScriptSemanticValue| -> bool {
                match value.clone() {
                    self::JavaScriptSemanticValue::SourceText(_, _) => true,
                    _ => false,
                }
            });
        ml_values2089
            .iter()
            .cloned()
            .enumerate()
            .any(|(ml_index2089, ml_value2089)| ml_callback2089(ml_value2089))
    } {
        self::execute_java_script_call_primitive(
            &(if all {
                self::ml_string::Text::from_units(vec![
                    114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120, 116,
                ])
            } else {
                self::ml_string::Text::from_units(vec![
                    114, 101, 112, 108, 97, 99, 101, 32, 116, 101, 120, 116,
                ])
            }),
            values.clone(),
        )
    } else {
        {
            let text = ({
                let source_ml_source2_array_read_index32701 = 0f64;
                {
                    let ml_reference_array2091 = values.clone();
                    let ml_reference_position2091 = source_ml_source2_array_read_index32701;
                    let ml_reference_projection2091 = self::ml_array::optional_at(
                        &*ml_reference_array2091.borrow(),
                        self::ml_array::number_index(ml_reference_position2091),
                    );
                    ml_reference_projection2091
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            {
                let search = ({
                    let source_ml_source2_array_read_index32703 = 1f64;
                    {
                        let ml_reference_array2092 = values.clone();
                        let ml_reference_position2092 = source_ml_source2_array_read_index32703;
                        let ml_reference_projection2092 = self::ml_array::optional_at(
                            &*ml_reference_array2092.borrow(),
                            self::ml_array::number_index(ml_reference_position2092),
                        );
                        ml_reference_projection2092
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let replacement = ({
                        let source_ml_source2_array_read_index32705 = 2f64;
                        {
                            let ml_reference_array2093 = values.clone();
                            let ml_reference_position2093 = source_ml_source2_array_read_index32705;
                            let ml_reference_projection2093 = self::ml_array::optional_at(
                                &*ml_reference_array2093.borrow(),
                                self::ml_array::number_index(ml_reference_position2093),
                            );
                            ml_reference_projection2093
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    match text.clone() {
                        self::JavaScriptSemanticValue::Text(text_units) => match search.clone() {
                            self::JavaScriptSemanticValue::Text(search_units) => {
                                match replacement.clone() {
                                    self::JavaScriptSemanticValue::Text(units) => {
                                        self::execute_java_script_call_text_replacement(
                                            text_units.clone(),
                                            search_units.clone(),
                                            units.clone(),
                                            all,
                                        )
                                    }
                                    _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                        self::ml_string::Text::from_units(vec![
                                            116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99,
                                            101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105,
                                            114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114,
                                            101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111,
                                            110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49,
                                            54, 32, 111, 112, 101, 114, 97, 110, 100, 115,
                                        ]),
                                    ),
                                }
                            }
                            _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109,
                                    101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                                    105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116,
                                    117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100,
                                    32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100,
                                    115,
                                ]),
                            ),
                        },
                        _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101,
                                110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116,
                                115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108,
                                32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49,
                                54, 32, 111, 112, 101, 114, 97, 110, 100, 115,
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
pub fn read_java_script_call_text_replacement_work_error(
    length: f64,
    search: f64,
    replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    all: bool,
) -> self::ml_string::Text {
    if (((length > (1048576f64)) || (search > (1048576f64)))
        || (({
            let ml_reference_array2094 = replacement.clone();
            let ml_reference_projection2094 = ml_reference_array2094.borrow().len() as f64;
            ml_reference_projection2094
        }) > (4096f64)))
    {
        self::ml_string::Text::from_units(vec![
            116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 101,
            120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105,
            116, 32, 105, 110, 112, 117, 116, 32, 111, 114, 32, 115, 117, 98, 115, 116, 105, 116,
            117, 116, 105, 111, 110, 45, 112, 108, 97, 110, 32, 98, 117, 100, 103, 101, 116,
        ])
    } else {
        {
            let matches = if (search == (0f64)) {
                (length + 1f64)
            } else {
                (length / search).floor()
            };
            {
                let matches_3 = if !all {
                    {
                        let matches_2 = self::ml_math::min(1f64, matches);
                        matches_2
                    }
                } else {
                    matches
                };
                if (matches_3 == (0f64)) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if (((length + 1f64) * search) > (67108864f64)) {
                        self::ml_string::Text::from_units(vec![
                            116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110,
                            116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101,
                            120, 112, 108, 105, 99, 105, 116, 32, 115, 101, 97, 114, 99, 104, 32,
                            119, 111, 114, 107, 32, 113, 117, 111, 116, 97,
                        ])
                    } else {
                        {
                            let plan = self::plan_utf16_text_substitution(replacement.clone());
                            if ((matches_3
                                * ({
                                    let ml_reference_array2095 = plan.clone();
                                    let ml_reference_projection2095 =
                                        ml_reference_array2095.borrow().len() as f64;
                                    ml_reference_projection2095
                                } / 3f64))
                                > (67108864f64))
                            {
                                self::ml_string::Text::from_units(vec![
                                    116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109,
                                    101, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105,
                                    116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115,
                                    117, 98, 115, 116, 105, 116, 117, 116, 105, 111, 110, 32, 119,
                                    111, 114, 107, 32, 113, 117, 111, 116, 97,
                                ])
                            } else {
                                {
                                    let width = 0f64;
                                    {
                                        let index = 0f64;
                                        {
                                            let width_2 = self::source_ml_source1_temporary_read_java_script_call_text_replacement_work_error_loop1400(length, search, plan.clone(), width, index);
                                            if ((length + (matches_3 * width_2)) > (1048576f64)) {
                                                self::ml_string::Text::from_units(vec![
                                                    116, 101, 120, 116, 32, 114, 101, 112, 108, 97,
                                                    99, 101, 109, 101, 110, 116, 32, 101, 120, 99,
                                                    101, 101, 100, 115, 32, 105, 116, 115, 32, 99,
                                                    111, 110, 115, 101, 114, 118, 97, 116, 105,
                                                    118, 101, 32, 85, 84, 70, 49, 54, 32, 111, 117,
                                                    116, 112, 117, 116, 32, 119, 111, 114, 107, 32,
                                                    98, 111, 117, 110, 100,
                                                ])
                                            } else {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                        }
                                    }
                                }
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
pub fn execute_java_script_call_text_replacement(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    all: bool,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let error = self::read_java_script_call_text_replacement_work_error(
            {
                let ml_reference_array2096 = text.clone();
                let ml_reference_projection2096 = ml_reference_array2096.borrow().len() as f64;
                ml_reference_projection2096
            },
            {
                let ml_reference_array2097 = search.clone();
                let ml_reference_projection2097 = ml_reference_array2097.borrow().len() as f64;
                ml_reference_projection2097
            },
            replacement.clone(),
            all,
        );
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(error.clone())
        } else {
            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                self::JavaScriptSemanticValue::Text(self::replace_utf16_text(
                    text.clone(),
                    search.clone(),
                    replacement.clone(),
                    all,
                )),
            ))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_call_primitive_type_error(
    reason: &self::ml_string::Text,
) -> self::JavaScriptCallPrimitiveResult {
    self::JavaScriptCallPrimitiveResult::PrimitiveError(
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]),
        reason.clone(),
    )
}
