// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryConstructUtf16TextPaddingLoop1395Result;
use super::MlSource1TemporaryConstructUtf16TextRepetitionLoop1393Result;
use super::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Result;
use super::character_units_utf16_text;
use super::dependency_function2;
use super::execute_java_script_call_primitive;
use super::execute_java_script_call_text_pair;
use super::execute_java_script_call_text_position;
use super::execute_java_script_call_text_replacement_operands;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_string;
use super::read_utf16_text_padding_length;
use super::read_utf16_text_repetition_length;
use super::source_ml_source1_temporary_construct_utf16_text_padding_loop1395;
use super::source_ml_source1_temporary_construct_utf16_text_repetition_loop1393;
use super::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop1398;
use super::test_java_script_call_unary_number_operation;
use super::trim_utf16_text;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_utf16_text_repetition(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    length: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((!(self::ml_math::is_safe_integer(length)) || (length < (0f64)))
        || (length > (1048576f64)))
        || ((length > (0f64))
            && ((({
                let ml_reference_array2054 = text.clone();
                let ml_reference_projection2054 = ml_reference_array2054.borrow().len() as f64;
                ml_reference_projection2054
            }) == (0f64))
                || ((length % {
                    let ml_reference_array2055 = text.clone();
                    let ml_reference_projection2055 = ml_reference_array2055.borrow().len() as f64;
                    ml_reference_projection2055
                }) != (0f64)))))
    {
        panic!(
            "{}",
            "text repetition requires its complete admitted length"
        )
    } else {
        {
            let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let index = 0f64;
                {
                    let source_ml_source2_s32664 =
                        self::source_ml_source1_temporary_construct_utf16_text_repetition_loop1393(
                            text.clone(),
                            length,
                            result.clone(),
                            index,
                        );
                    match source_ml_source2_s32664.clone() {
                        self::MlSource1TemporaryConstructUtf16TextRepetitionLoop1393Result::MlSource1TemporaryConstructUtf16TextRepetitionLoop1393Done => {
                            result.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_utf16_text_padding(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    filler: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    length: f64,
    start: bool,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((!(self::ml_math::is_safe_integer(length))
        || (length
            < ({
                let ml_reference_array2056 = text.clone();
                let ml_reference_projection2056 = ml_reference_array2056.borrow().len() as f64;
                ml_reference_projection2056
            })))
        || (length > (1048576f64)))
        || ((length
            > ({
                let ml_reference_array2057 = text.clone();
                let ml_reference_projection2057 = ml_reference_array2057.borrow().len() as f64;
                ml_reference_projection2057
            }))
            && (({
                let ml_reference_array2058 = filler.clone();
                let ml_reference_projection2058 = ml_reference_array2058.borrow().len() as f64;
                ml_reference_projection2058
            }) == (0f64))))
    {
        panic!("{}", "text padding requires its complete admitted length")
    } else {
        {
            let added = (length - {
                let ml_reference_array2059 = text.clone();
                let ml_reference_projection2059 = ml_reference_array2059.borrow().len() as f64;
                ml_reference_projection2059
            });
            {
                let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s32665 =
                            self::source_ml_source1_temporary_construct_utf16_text_padding_loop1395(
                                text.clone(),
                                filler.clone(),
                                length,
                                start,
                                added,
                                result.clone(),
                                index,
                            );
                        match source_ml_source2_s32665.clone() {
                            self::MlSource1TemporaryConstructUtf16TextPaddingLoop1395Result::MlSource1TemporaryConstructUtf16TextPaddingLoop1395Done => {
                                result.clone()
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
pub fn dependency_function8(
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    self::dependency_function2(value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_numeric_primitive(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    if self::test_java_script_call_unary_number_operation(operation) {
        if (({
            let ml_reference_array2060 = values.clone();
            let ml_reference_projection2060 = ml_reference_array2060.borrow().len() as f64;
            ml_reference_projection2060
        }) != (1f64))
        {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                self::ml_string::Text::from_units(vec![
                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117,
                    99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101,
                    118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97,
                    110, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101,
                ]),
            )
        } else {
            {
                let first = ({
                    let source_ml_source2_array_read_index32667 = 0f64;
                    {
                        let ml_reference_array2061 = values.clone();
                        let ml_reference_position2061 = source_ml_source2_array_read_index32667;
                        let ml_reference_projection2061 = self::ml_array::optional_at(
                            &*ml_reference_array2061.borrow(),
                            self::ml_array::number_index(ml_reference_position2061),
                        );
                        ml_reference_projection2061
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                match first.clone() {
                    self::JavaScriptSemanticValue::Number(value) => {
                        self::execute_java_script_call_unary_number(operation, value)
                    }
                    _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                        self::ml_string::Text::from_units(vec![
                            110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116,
                            105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99,
                            116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101,
                            114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117,
                            114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                        ]),
                    ),
                }
            }
        }
    } else {
        {
            let numbers = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let source_ml_source1_temporary_index1397 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s32668 = self::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop1398(numbers.clone(), values.clone(), source_ml_source1_temporary_index1397.clone());
                    match source_ml_source2_s32668.clone() {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Done => {
                            if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::min_of(&self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value2062 = numbers.borrow().clone(); ml_reference_value2062 })))))
                            } else {
                                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::max_of(&self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value2063 = numbers.borrow().clone(); ml_reference_value2063 })))))
                                } else {
                                    if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115]))) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_units_utf16_text(numbers.clone()))))
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Return(source_ml_source1_temporary_result1398) => {
                            let source_ml_source1_temporary_result1398 = *source_ml_source1_temporary_result1398;
                            source_ml_source1_temporary_result1398.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_unary_number(
    operation: &self::ml_string::Text,
    number: f64,
) -> self::JavaScriptCallPrimitiveResult {
    if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            116, 101, 115, 116, 32, 78, 97, 78, 32, 110, 117, 109, 98, 101, 114,
        ])))
    {
        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
            self::JavaScriptSemanticValue::Boolean(number.is_nan()),
        ))
    } else {
        if ((*operation)
            == (self::ml_string::Text::from_units(vec![
                116, 101, 115, 116, 32, 102, 105, 110, 105, 116, 101, 32, 110, 117, 109, 98, 101,
                114,
            ])))
        {
            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                self::JavaScriptSemanticValue::Boolean(number.is_finite()),
            ))
        } else {
            if ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114,
                ])))
            {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                    self::JavaScriptSemanticValue::Number((-number)),
                ))
            } else {
                if ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114,
                    ])))
                {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                        self::JavaScriptSemanticValue::Number(number.trunc()),
                    ))
                } else {
                    if ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110,
                            117, 109, 98, 101, 114,
                        ])))
                    {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                            self::JavaScriptSemanticValue::Boolean(self::ml_math::is_integer(
                                number,
                            )),
                        ))
                    } else {
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101,
                                103, 101, 114, 32, 110, 117, 109, 98, 101, 114,
                            ])))
                        {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                self::JavaScriptSemanticValue::Boolean(
                                    self::ml_math::is_safe_integer(number),
                                ),
                            ))
                        } else {
                            if !(self::test_java_script_call_unary_number_operation(operation)) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        117, 110, 97, 114, 121, 32, 110, 117, 109, 101, 114, 105,
                                        99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115,
                                        32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101,
                                        105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103,
                                        105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114,
                                        97, 116, 105, 111, 110,
                                    ]),
                                )
                            } else {
                                {
                                    let result = if ((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116,
                                            101, 32, 110, 117, 109, 98, 101, 114,
                                        ]))) {
                                        number.abs()
                                    } else {
                                        if ((*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                102, 108, 111, 111, 114, 32, 110, 117, 109, 98,
                                                101, 114,
                                            ])))
                                        {
                                            number.floor()
                                        } else {
                                            if ((*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    99, 101, 105, 108, 32, 110, 117, 109, 98, 101,
                                                    114,
                                                ])))
                                            {
                                                number.ceil()
                                            } else {
                                                if ((*operation)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        114, 111, 117, 110, 100, 32, 110, 117, 109,
                                                        98, 101, 114,
                                                    ])))
                                                {
                                                    self::ml_math::round(number)
                                                } else {
                                                    if ((*operation)
                                                        == (self::ml_string::Text::from_units(
                                                            vec![
                                                                114, 101, 97, 100, 32, 110, 117,
                                                                109, 98, 101, 114, 32, 115, 105,
                                                                103, 110,
                                                            ],
                                                        )))
                                                    {
                                                        self::ml_math::sign(number)
                                                    } else {
                                                        number.sqrt()
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                        self::JavaScriptSemanticValue::Number(result),
                                    ))
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
pub fn execute_java_script_call_text_primitive(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    if {
        let ml_values2064 = {
            let ml_reference_value2065 = values.borrow().clone();
            ml_reference_value2065
        };
        let ml_callback2064 =
            std::rc::Rc::new(move |value: self::JavaScriptSemanticValue| -> bool {
                match value.clone() {
                    self::JavaScriptSemanticValue::SourceText(_, _) => true,
                    _ => false,
                }
            });
        ml_values2064
            .iter()
            .cloned()
            .enumerate()
            .any(|(ml_index2064, ml_value2064)| ml_callback2064(ml_value2064))
    } {
        self::execute_java_script_call_primitive(operation, values.clone())
    } else {
        if ((((*operation)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 112, 101, 97, 116, 32, 116, 101, 120, 116,
            ])))
            || ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116,
                ]))))
            || ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    112, 97, 100, 32, 116, 101, 120, 116, 32, 101, 110, 100,
                ]))))
        {
            self::execute_java_script_call_text_expansion(operation, values.clone())
        } else {
            if (((*operation)
                == (self::ml_string::Text::from_units(vec![
                    114, 101, 112, 108, 97, 99, 101, 32, 116, 101, 120, 116,
                ])))
                || ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120, 116,
                    ]))))
            {
                self::execute_java_script_call_text_replacement_operands(
                    values.clone(),
                    ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120,
                            116,
                        ]))),
                )
            } else {
                {
                    let receiver = ({
                        let source_ml_source2_array_read_index32672 = 0f64;
                        {
                            let ml_reference_array2066 = values.clone();
                            let ml_reference_position2066 = source_ml_source2_array_read_index32672;
                            let ml_reference_projection2066 = self::ml_array::optional_at(
                                &*ml_reference_array2066.borrow(),
                                self::ml_array::number_index(ml_reference_position2066),
                            );
                            ml_reference_projection2066
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Text(text) => {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110,
                                    103, 116, 104,
                                ])))
                            {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                    self::JavaScriptSemanticValue::Number({
                                        let ml_reference_array2067 = text.clone();
                                        let ml_reference_projection2067 =
                                            ml_reference_array2067.borrow().len() as f64;
                                        ml_reference_projection2067
                                    }),
                                ))
                            } else {
                                if ((((*operation)
                                    == (self::ml_string::Text::from_units(vec![
                                        116, 114, 105, 109, 32, 116, 101, 120, 116,
                                    ])))
                                    || ((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115,
                                            116, 97, 114, 116,
                                        ]))))
                                    || ((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101,
                                            110, 100,
                                        ]))))
                                {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                        self::JavaScriptSemanticValue::Text(self::trim_utf16_text(
                                            text.clone(),
                                            ((*operation)
                                                != (self::ml_string::Text::from_units(vec![
                                                    116, 114, 105, 109, 32, 116, 101, 120, 116, 32,
                                                    101, 110, 100,
                                                ]))),
                                            ((*operation)
                                                != (self::ml_string::Text::from_units(vec![
                                                    116, 114, 105, 109, 32, 116, 101, 120, 116, 32,
                                                    115, 116, 97, 114, 116,
                                                ]))),
                                        )),
                                    ))
                                } else {
                                    if (((((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111,
                                            100, 101, 32, 112, 111, 105, 110, 116,
                                        ])))
                                        || ((*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99,
                                                104, 97, 114, 97, 99, 116, 101, 114,
                                            ]))))
                                        || ((*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105,
                                                118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97,
                                                114, 97, 99, 116, 101, 114,
                                            ]))))
                                        || ((*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                115, 108, 105, 99, 101, 32, 116, 101, 120, 116,
                                            ]))))
                                    {
                                        self::execute_java_script_call_text_position(
                                            operation,
                                            text.clone(),
                                            values.clone(),
                                        )
                                    } else {
                                        self::execute_java_script_call_text_pair(
                                            operation,
                                            text.clone(),
                                            values.clone(),
                                        )
                                    }
                                }
                            }
                        }
                        _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118,
                                101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101,
                                105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112,
                                108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101,
                                105, 118, 101, 114,
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
pub fn execute_java_script_call_text_expansion(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    if {
        let ml_values2068 = {
            let ml_reference_value2069 = values.borrow().clone();
            ml_reference_value2069
        };
        let ml_callback2068 =
            std::rc::Rc::new(move |value: self::JavaScriptSemanticValue| -> bool {
                match value.clone() {
                    self::JavaScriptSemanticValue::SourceText(_, _) => true,
                    _ => false,
                }
            });
        ml_values2068
            .iter()
            .cloned()
            .enumerate()
            .any(|(ml_index2068, ml_value2068)| ml_callback2068(ml_value2068))
    } {
        self::execute_java_script_call_primitive(operation, values.clone())
    } else {
        {
            let receiver = ({
                let source_ml_source2_array_read_index32676 = 0f64;
                {
                    let ml_reference_array2070 = values.clone();
                    let ml_reference_position2070 = source_ml_source2_array_read_index32676;
                    let ml_reference_projection2070 = self::ml_array::optional_at(
                        &*ml_reference_array2070.borrow(),
                        self::ml_array::number_index(ml_reference_position2070),
                    );
                    ml_reference_projection2070
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match receiver.clone() {
                self::JavaScriptSemanticValue::Text(receiver_units) => {
                    let count = ({
                        let source_ml_source2_array_read_index32678 = 1f64;
                        {
                            let ml_reference_array2071 = values.clone();
                            let ml_reference_position2071 = source_ml_source2_array_read_index32678;
                            let ml_reference_projection2071 = self::ml_array::optional_at(
                                &*ml_reference_array2071.borrow(),
                                self::ml_array::number_index(ml_reference_position2071),
                            );
                            ml_reference_projection2071
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    match count.clone() {
                        self::JavaScriptSemanticValue::Number(value) => {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    114, 101, 112, 101, 97, 116, 32, 116, 101, 120, 116,
                                ])))
                            {
                                {
                                    let length = self::read_utf16_text_repetition_length(
                                        {
                                            let ml_reference_array2072 = receiver_units.clone();
                                            let ml_reference_projection2072 =
                                                ml_reference_array2072.borrow().len() as f64;
                                            ml_reference_projection2072
                                        },
                                        value,
                                    );
                                    if (length == (-1f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                            self::ml_string::Text::from_units(vec![
                                                82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                                            ]),
                                            (self::ml_string::Text::from_units(vec![
                                                73, 110, 118, 97, 108, 105, 100, 32, 99, 111, 117,
                                                110, 116, 32, 118, 97, 108, 117, 101, 58, 32,
                                            ]))
                                            .concat(
                                                &self::ml_string::Text::from_text(
                                                    &self::ml_number::js_number(value),
                                                ),
                                            ),
                                        )
                                    } else {
                                        if (length < (0f64)) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 111, 117, 116, 112, 117, 116, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                                Box::new(self::JavaScriptSemanticValue::Text(
                                                    self::construct_utf16_text_repetition(
                                                        receiver_units.clone(),
                                                        length,
                                                    ),
                                                )),
                                            )
                                        }
                                    }
                                }
                            } else {
                                if (((*operation)
                                    != (self::ml_string::Text::from_units(vec![
                                        112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97,
                                        114, 116,
                                    ])))
                                    && ((*operation)
                                        != (self::ml_string::Text::from_units(vec![
                                            112, 97, 100, 32, 116, 101, 120, 116, 32, 101, 110, 100,
                                        ]))))
                                {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                        self::ml_string::Text::from_units(vec![
                                            116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115,
                                            105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101,
                                            115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                            32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100,
                                            32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
                                        ]),
                                    )
                                } else {
                                    {
                                        let filler = ({
                                            let source_ml_source2_array_read_index32681 = 2f64;
                                            {
                                                let ml_reference_array2073 = values.clone();
                                                let ml_reference_position2073 =
                                                    source_ml_source2_array_read_index32681;
                                                let ml_reference_projection2073 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array2073.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position2073,
                                                        ),
                                                    );
                                                ml_reference_projection2073
                                            }
                                        })
                                        .clone()
                                        .unwrap_or_else(|| {
                                            self::JavaScriptSemanticValue::Undefined
                                        });
                                        match filler.clone() {
                                                self::JavaScriptSemanticValue::Text(units) => {
                                                    {
                                                        let length_2 = self::read_utf16_text_padding_length({ let ml_reference_array2074 = receiver_units.clone(); let ml_reference_projection2074 = ml_reference_array2074.borrow().len() as f64; ml_reference_projection2074 }, value, { let ml_reference_array2075 = units.clone(); let ml_reference_projection2075 = ml_reference_array2075.borrow().len() as f64; ml_reference_projection2075 });
                                                        if (length_2 < (0f64)) {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 111, 117, 116, 112, 117, 116, 32, 98, 117, 100, 103, 101, 116]))
                                                        } else {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::construct_utf16_text_padding(receiver_units.clone(), units.clone(), length_2, ((*operation) == (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))))))
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 97, 100, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 108, 108, 101, 114]))
                                                }
                                            }
                                    }
                                }
                            }
                        }
                        _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110,
                                32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32,
                                97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116,
                                101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 98, 111, 117, 110,
                                100,
                            ]),
                        ),
                    }
                }
                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32,
                        114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                        117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70,
                        49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114,
                    ]),
                ),
            }
        }
    }
}
