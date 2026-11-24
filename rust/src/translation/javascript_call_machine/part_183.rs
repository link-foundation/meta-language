// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallPrimitiveScalarLoop1404Result;
use super::construct_java_script_call_primitive_type_error;
use super::construct_java_script_source_text;
use super::execute_java_script_call_integer_primitive;
use super::execute_java_script_call_numeric_primitive;
use super::execute_java_script_call_primitive;
use super::execute_java_script_call_scalar_conversion;
use super::execute_java_script_call_text_primitive;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_like_length;
use super::read_java_script_call_integer_arity;
use super::read_java_script_call_scalar_conversion_error;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_text_string;
use super::read_java_script_source_exception_class;
use super::source_ml_source1_temporary_execute_java_script_call_primitive_scalar_loop1404;
use super::source_ml_source1_temporary_execute_java_script_call_primitive_scalar_loop1406;
use super::test_java_script_call_unary_number_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive_scalar(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 105, 110, 100, 101, 120,
        ])))
    {
        {
            let value = ({
                let source_ml_source2_array_read_index32777 = 0f64;
                {
                    let ml_reference_array2125 = values.clone();
                    let ml_reference_position2125 = source_ml_source2_array_read_index32777;
                    let ml_reference_projection2125 = self::ml_array::optional_at(
                        &*ml_reference_array2125.borrow(),
                        self::ml_array::number_index(ml_reference_position2125),
                    );
                    ml_reference_projection2125
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match value.clone() {
                self::JavaScriptSemanticValue::Number(value_value) => {
                    let index = if (value_value.is_nan() || (value_value == (0f64))) {
                        0f64
                    } else {
                        value_value.trunc()
                    };
                    if ((index < (0f64)) || (index > (9007199254740991f64))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveError(
                            self::ml_string::Text::from_units(vec![
                                82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                73, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 98, 101, 32,
                                98, 101, 116, 119, 101, 101, 110, 32, 48, 32, 97, 110, 100, 32, 57,
                                48, 48, 55, 49, 57, 57, 50, 53, 52, 55, 52, 48, 57, 57, 49,
                            ]),
                        )
                    } else {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                            self::JavaScriptSemanticValue::Number(if (index == (0f64)) {
                                0f64
                            } else {
                                index
                            }),
                        ))
                    }
                }
                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        84, 111, 73, 110, 100, 101, 120, 32, 99, 111, 110, 115, 117, 109, 101, 115,
                        32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84,
                        111, 78, 117, 109, 98, 101, 114, 32, 118, 97, 108, 117, 101,
                    ]),
                ),
            }
        }
    } else {
        if ((*operation)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 108, 101,
                110, 103, 116, 104,
            ])))
        {
            {
                let value_2 = ({
                    let source_ml_source2_array_read_index32779 = 0f64;
                    {
                        let ml_reference_array2126 = values.clone();
                        let ml_reference_position2126 = source_ml_source2_array_read_index32779;
                        let ml_reference_projection2126 = self::ml_array::optional_at(
                            &*ml_reference_array2126.borrow(),
                            self::ml_array::number_index(ml_reference_position2126),
                        );
                        ml_reference_projection2126
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                match value_2.clone() {
                    self::JavaScriptSemanticValue::Number(value_value_2) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                            self::JavaScriptSemanticValue::Number(
                                self::read_java_script_array_like_length(value_value_2),
                            ),
                        ))
                    }
                    _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                        self::ml_string::Text::from_units(vec![
                            84, 111, 76, 101, 110, 103, 116, 104, 32, 99, 111, 110, 115, 117, 109,
                            101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101,
                            100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 118, 97, 108, 117,
                            101,
                        ]),
                    ),
                }
            }
        } else {
            if ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 97, 114, 114,
                    97, 121, 32, 105, 110, 100, 101, 120,
                ])))
            {
                {
                    let length = ({
                        let source_ml_source2_array_read_index32781 = 0f64;
                        {
                            let ml_reference_array2127 = values.clone();
                            let ml_reference_position2127 = source_ml_source2_array_read_index32781;
                            let ml_reference_projection2127 = self::ml_array::optional_at(
                                &*ml_reference_array2127.borrow(),
                                self::ml_array::number_index(ml_reference_position2127),
                            );
                            ml_reference_projection2127
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    {
                        let number = ({
                            let source_ml_source2_array_read_index32783 = 1f64;
                            {
                                let ml_reference_array2128 = values.clone();
                                let ml_reference_position2128 =
                                    source_ml_source2_array_read_index32783;
                                let ml_reference_projection2128 = self::ml_array::optional_at(
                                    &*ml_reference_array2128.borrow(),
                                    self::ml_array::number_index(ml_reference_position2128),
                                );
                                ml_reference_projection2128
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        match length.clone() {
                            self::JavaScriptSemanticValue::Number(length_value) => {
                                if (!(self::ml_math::is_safe_integer(length_value))
                                    || (length_value < (0f64)))
                                {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                        self::ml_string::Text::from_units(vec![
                                            114, 101, 108, 97, 116, 105, 118, 101, 32, 65, 114,
                                            114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115, 32,
                                            114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97,
                                            108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97,
                                            110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101,
                                            100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 111,
                                            112, 101, 114, 97, 110, 100, 115,
                                        ]),
                                    )
                                } else {
                                    match number.clone() {
                                        self::JavaScriptSemanticValue::Number(value_3) => {
                                            let integer =
                                                if (value_3.is_nan() || (value_3 == (0f64))) {
                                                    0f64
                                                } else {
                                                    value_3.trunc()
                                                };
                                            {
                                                let index_2 = if (integer == (f64::NEG_INFINITY)) {
                                                    0f64
                                                } else {
                                                    if (integer < (0f64)) {
                                                        self::ml_math::max(
                                                            (length_value + integer),
                                                            0f64,
                                                        )
                                                    } else {
                                                        self::ml_math::min(integer, length_value)
                                                    }
                                                };
                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                                    Box::new(
                                                        self::JavaScriptSemanticValue::Number(
                                                            index_2,
                                                        ),
                                                    ),
                                                )
                                            }
                                        }
                                        _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                            self::ml_string::Text::from_units(vec![
                                                114, 101, 108, 97, 116, 105, 118, 101, 32, 65, 114,
                                                114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115,
                                                32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116,
                                                117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116,
                                                104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108,
                                                101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98,
                                                101, 114, 32, 111, 112, 101, 114, 97, 110, 100,
                                                115,
                                            ]),
                                        ),
                                    }
                                }
                            }
                            _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    114, 101, 108, 97, 116, 105, 118, 101, 32, 65, 114, 114, 97,
                                    121, 32, 105, 110, 100, 105, 99, 101, 115, 32, 114, 101, 116,
                                    97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76,
                                    101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109,
                                    112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98,
                                    101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115,
                                ]),
                            ),
                        }
                    }
                }
            } else {
                if ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        114, 101, 97, 100, 32, 99, 104, 101, 99, 107, 101, 100, 32, 97, 114, 114,
                        97, 121, 32, 97, 112, 112, 101, 110, 100, 32, 108, 101, 110, 103, 116, 104,
                    ])))
                {
                    {
                        let length_2 = ({
                            let source_ml_source2_array_read_index32785 = 0f64;
                            {
                                let ml_reference_array2129 = values.clone();
                                let ml_reference_position2129 =
                                    source_ml_source2_array_read_index32785;
                                let ml_reference_projection2129 = self::ml_array::optional_at(
                                    &*ml_reference_array2129.borrow(),
                                    self::ml_array::number_index(ml_reference_position2129),
                                );
                                ml_reference_projection2129
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let count = ({
                                let source_ml_source2_array_read_index32787 = 1f64;
                                {
                                    let ml_reference_array2130 = values.clone();
                                    let ml_reference_position2130 =
                                        source_ml_source2_array_read_index32787;
                                    let ml_reference_projection2130 = self::ml_array::optional_at(
                                        &*ml_reference_array2130.borrow(),
                                        self::ml_array::number_index(ml_reference_position2130),
                                    );
                                    ml_reference_projection2130
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                            match length_2.clone() {
                                self::JavaScriptSemanticValue::Number(length_value_2) => {
                                    match count.clone() {
                                        self::JavaScriptSemanticValue::Number(value_4) => {
                                            if (((!(self::ml_math::is_safe_integer(
                                                length_value_2,
                                            )) || (length_value_2 < (0f64)))
                                                || !(self::ml_math::is_safe_integer(value_4)))
                                                || (value_4 < (0f64)))
                                            {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 112, 112, 101, 110, 100, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 99, 111, 117, 110, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                            } else {
                                                if (length_value_2
                                                    > (9007199254740991f64 - value_4))
                                                {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 45, 108, 105, 107, 101, 32, 108, 101, 110, 103, 116, 104, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 109, 97, 120, 105, 109, 117, 109, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114]))
                                                } else {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((length_value_2 + value_4))))
                                                }
                                            }
                                        }
                                        _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                            self::ml_string::Text::from_units(vec![
                                                65, 114, 114, 97, 121, 32, 97, 112, 112, 101, 110,
                                                100, 32, 108, 101, 110, 103, 116, 104, 32, 114,
                                                101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117,
                                                97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104,
                                                32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101,
                                                110, 116, 32, 99, 111, 117, 110, 116, 32, 99, 111,
                                                111, 114, 100, 105, 110, 97, 116, 101, 115,
                                            ]),
                                        ),
                                    }
                                }
                                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 97, 112, 112, 101, 110, 100, 32,
                                        108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105,
                                        110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76,
                                        101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 97, 114,
                                        103, 117, 109, 101, 110, 116, 32, 99, 111, 117, 110, 116,
                                        32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115,
                                    ]),
                                ),
                            }
                        }
                    }
                } else {
                    if ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            114, 101, 113, 117, 105, 114, 101, 32, 100, 101, 108, 101, 116, 101,
                            32, 115, 117, 99, 99, 101, 115, 115,
                        ])))
                    {
                        {
                            let value_5 = ({
                                let source_ml_source2_array_read_index32789 = 0f64;
                                {
                                    let ml_reference_array2131 = values.clone();
                                    let ml_reference_position2131 =
                                        source_ml_source2_array_read_index32789;
                                    let ml_reference_projection2131 = self::ml_array::optional_at(
                                        &*ml_reference_array2131.borrow(),
                                        self::ml_array::number_index(ml_reference_position2131),
                                    );
                                    ml_reference_projection2131
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                            match value_5.clone() {
                                self::JavaScriptSemanticValue::Boolean(value_value_3) => {
                                    if value_value_3 {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                            Box::new(self::JavaScriptSemanticValue::Undefined),
                                        )
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                            self::ml_string::Text::from_units(vec![
                                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                67, 97, 110, 110, 111, 116, 32, 100, 101, 108, 101,
                                                116, 101, 32, 97, 32, 110, 111, 110, 45, 99, 111,
                                                110, 102, 105, 103, 117, 114, 97, 98, 108, 101, 32,
                                                112, 114, 111, 112, 101, 114, 116, 121,
                                            ]),
                                        )
                                    }
                                }
                                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        115, 116, 114, 105, 99, 116, 32, 100, 101, 108, 101, 116,
                                        105, 111, 110, 32, 99, 111, 110, 115, 117, 109, 101, 115,
                                        32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66,
                                        111, 111, 108, 101, 97, 110, 32, 114, 101, 115, 117, 108,
                                        116,
                                    ]),
                                ),
                            }
                        }
                    } else {
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                114, 101, 113, 117, 105, 114, 101, 32, 99, 111, 108, 108, 101, 99,
                                116, 105, 111, 110, 32, 97, 100, 100, 101, 114,
                            ])))
                        {
                            {
                                let value_6 = ({
                                    let source_ml_source2_array_read_index32791 = 0f64;
                                    {
                                        let ml_reference_array2132 = values.clone();
                                        let ml_reference_position2132 =
                                            source_ml_source2_array_read_index32791;
                                        let ml_reference_projection2132 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array2132.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position2132,
                                                ),
                                            );
                                        ml_reference_projection2132
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                {
                                    let invalid =
                                        self::construct_java_script_call_primitive_type_error(
                                            &(self::ml_string::Text::from_units(vec![
                                                67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32,
                                                97, 100, 100, 101, 114, 32, 105, 115, 32, 110, 111,
                                                116, 32, 99, 97, 108, 108, 97, 98, 108, 101,
                                            ])),
                                        );
                                    match value_6.clone() {
                                        self::JavaScriptSemanticValue::Object(_, callable) => {
                                            if callable {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                                    Box::new(value_6.clone()),
                                                )
                                            } else {
                                                invalid.clone()
                                            }
                                        }
                                        _ => invalid.clone(),
                                    }
                                }
                            }
                        } else {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    114, 101, 113, 117, 105, 114, 101, 32, 77, 97, 112, 32, 101,
                                    110, 116, 114, 121, 32, 111, 98, 106, 101, 99, 116,
                                ])))
                            {
                                {
                                    let value_7 = ({
                                        let source_ml_source2_array_read_index32794 = 0f64;
                                        {
                                            let ml_reference_array2133 = values.clone();
                                            let ml_reference_position2133 =
                                                source_ml_source2_array_read_index32794;
                                            let ml_reference_projection2133 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array2133.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position2133,
                                                    ),
                                                );
                                            ml_reference_projection2133
                                        }
                                    })
                                    .clone()
                                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                    if match value_7.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                            Box::new(value_7.clone()),
                                        )
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                            self::ml_string::Text::from_units(vec![
                                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                73, 116, 101, 114, 97, 116, 111, 114, 32, 118, 97,
                                                108, 117, 101, 32, 105, 115, 32, 110, 111, 116, 32,
                                                97, 110, 32, 101, 110, 116, 114, 121, 32, 111, 98,
                                                106, 101, 99, 116,
                                            ]),
                                        )
                                    }
                                }
                            } else {
                                if ((*operation)
                                    == (self::ml_string::Text::from_units(vec![
                                        114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101,
                                        99, 116, 32, 99, 111, 101, 114, 99, 105, 98, 108, 101,
                                    ])))
                                {
                                    {
                                        let value_8 = ({
                                            let source_ml_source2_array_read_index32798 = 0f64;
                                            {
                                                let ml_reference_array2134 = values.clone();
                                                let ml_reference_position2134 =
                                                    source_ml_source2_array_read_index32798;
                                                let ml_reference_projection2134 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array2134.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position2134,
                                                        ),
                                                    );
                                                ml_reference_projection2134
                                            }
                                        })
                                        .clone()
                                        .unwrap_or_else(|| {
                                            self::JavaScriptSemanticValue::Undefined
                                        });
                                        if (match value_8.clone() {
                                            self::JavaScriptSemanticValue::Null => true,
                                            _ => false,
                                        } || match value_8.clone() {
                                            self::JavaScriptSemanticValue::Undefined => true,
                                            _ => false,
                                        }) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                                self::ml_string::Text::from_units(vec![
                                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    67, 97, 110, 110, 111, 116, 32, 100, 101, 115,
                                                    116, 114, 117, 99, 116, 117, 114, 101, 32, 110,
                                                    117, 108, 108, 32, 111, 114, 32, 117, 110, 100,
                                                    101, 102, 105, 110, 101, 100,
                                                ]),
                                            )
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                                Box::new(self::JavaScriptSemanticValue::Undefined),
                                            )
                                        }
                                    }
                                } else {
                                    if ((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32,
                                            118, 97, 108, 117, 101,
                                        ])))
                                    {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                            Box::new(self::JavaScriptSemanticValue::Boolean({
                                                let source_ml_source2_s32801 = ({
                                                    let source_ml_source2_array_read_index32805 =
                                                        0f64;
                                                    {
                                                        let ml_reference_array2135 = values.clone();
                                                        let ml_reference_position2135 =
                                                            source_ml_source2_array_read_index32805;
                                                        let ml_reference_projection2135 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array2135.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position2135,
                                                                ),
                                                            );
                                                        ml_reference_projection2135
                                                    }
                                                })
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    self::JavaScriptSemanticValue::Undefined
                                                });
                                                match source_ml_source2_s32801.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        true
                                                    }
                                                    _ => false,
                                                }
                                            })),
                                        )
                                    } else {
                                        if ((*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116,
                                                105, 99, 32, 116, 121, 112, 101,
                                            ])))
                                        {
                                            {
                                                let value_9 = ({
                                                    let source_ml_source2_array_read_index32807 =
                                                        0f64;
                                                    {
                                                        let ml_reference_array2136 = values.clone();
                                                        let ml_reference_position2136 =
                                                            source_ml_source2_array_read_index32807;
                                                        let ml_reference_projection2136 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array2136.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position2136,
                                                                ),
                                                            );
                                                        ml_reference_projection2136
                                                    }
                                                })
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    self::JavaScriptSemanticValue::Undefined
                                                });
                                                {
                                                    let kind = if match value_9.clone() {
                                                        self::JavaScriptSemanticValue::Null => true,
                                                        _ => false,
                                                    } {
                                                        self::ml_string::Text::from_units(vec![
                                                            111, 98, 106, 101, 99, 116,
                                                        ])
                                                    } else {
                                                        match value_9.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, callable_2) => {
                                                                if callable_2 {
                                                                    self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                                                }
                                                            }
                                                            _ => {
                                                                match value_9.clone() {
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
                                                                        self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                                                                    }
                                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                                        self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                                                                    }
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                                                    }
                                                                    _ => {
                                                                        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let units = std::rc::Rc::new(
                                                            std::cell::RefCell::new(
                                                                Vec::<f64>::new(),
                                                            ),
                                                        );
                                                        {
                                                            let at = 0f64;
                                                            {
                                                                let source_ml_source2_s32842 = self::source_ml_source1_temporary_execute_java_script_call_primitive_scalar_loop1404(&(kind), units.clone(), at);
                                                                match source_ml_source2_s32842.clone() {
                                                                    self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveScalarLoop1404Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveScalarLoop1404Done => {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units.clone())))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if ((self::read_java_script_source_exception_class(
                                                operation,
                                            )) != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                if ((self::read_java_script_call_scalar_conversion_error(operation)) == (self::ml_string::Text::from_units(vec![]))) {
                                                    self::execute_java_script_call_scalar_conversion(operation, ({
                                                        let source_ml_source2_array_read_index32844 = 0f64;
                                                        { let ml_reference_array2137 = values.clone(); let ml_reference_position2137 = source_ml_source2_array_read_index32844; let ml_reference_projection2137 = self::ml_array::optional_at(&*ml_reference_array2137.borrow(), self::ml_array::number_index(ml_reference_position2137)); ml_reference_projection2137 }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                } else {
                                                    if { let ml_values2138 = { let ml_reference_value2139 = values.borrow().clone(); ml_reference_value2139 }; let ml_callback2138 = std::rc::Rc::new(move |value_10: self::JavaScriptSemanticValue| -> bool { match value_10.clone() {
                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    } }); ml_values2138.iter().cloned().enumerate().any(|(ml_index2138, ml_value2138)| ml_callback2138(ml_value2138)) } {
                                                        {
                                                            let first = ({
                                                                let source_ml_source2_array_read_index32848 = 0f64;
                                                                { let ml_reference_array2140 = values.clone(); let ml_reference_position2140 = source_ml_source2_array_read_index32848; let ml_reference_projection2140 = self::ml_array::optional_at(&*ml_reference_array2140.borrow(), self::ml_array::number_index(ml_reference_position2140)); ml_reference_projection2140 }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                            {
                                                                let second = ({
                                                                    let source_ml_source2_array_read_index32850 = 1f64;
                                                                    { let ml_reference_array2141 = values.clone(); let ml_reference_position2141 = source_ml_source2_array_read_index32850; let ml_reference_projection2141 = self::ml_array::optional_at(&*ml_reference_array2141.borrow(), self::ml_array::number_index(ml_reference_position2141)); ml_reference_projection2141 }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                if (((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]))) && ((self::read_java_script_semantic_text_length(first.clone())) >= (0f64))) {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::read_java_script_semantic_text_length(first.clone()))))
                                                                } else {
                                                                    if ((((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]))) && ((self::read_java_script_semantic_text_length(first.clone())) >= (0f64))) && ((self::read_java_script_semantic_text_length(second.clone())) >= (0f64))) {
                                                                        if (((self::read_java_script_semantic_text_length(first.clone()) + self::read_java_script_semantic_text_length(second.clone()))) > (16777216f64)) {
                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 54, 55, 55, 55, 50, 49, 54, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                        } else {
                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::construct_java_script_source_text(&((self::read_java_script_semantic_text_string(first.clone())).concat(&self::read_java_script_semantic_text_string(second.clone()))))))
                                                                        }
                                                                    } else {
                                                                        if { let ml_values2142 = { let ml_reference_value2143 = values.borrow().clone(); ml_reference_value2143 }; let ml_callback2142 = std::rc::Rc::new(move |value_10: self::JavaScriptSemanticValue| -> bool { (match value_10.clone() {
                                                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        } && ((((((match value_10.clone() {
                                                                            self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value32854) => {
                                                                                Some((source_ml_source2_data_property_value32854.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(self::ml_string::Text,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone()).length()) > (1048576f64))) }); ml_values2142.iter().cloned().enumerate().any(|(ml_index2142, ml_value2142)| ml_callback2142(ml_value2142)) } {
                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 116, 101, 120, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 115, 101, 112, 97, 114, 97, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 117, 110, 105, 116, 45, 97, 114, 114, 97, 121, 32, 97, 108, 103, 111, 114, 105, 116, 104, 109, 59, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 109, 97, 105, 110, 115, 32, 105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 97, 110, 100, 32, 114, 101, 115, 117, 109, 97, 98, 108, 101]))
                                                                        } else {
                                                                            {
                                                                                let materialized = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                {
                                                                                    let source_ml_source1_temporary_index1403 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let materialized_2 = self::source_ml_source1_temporary_execute_java_script_call_primitive_scalar_loop1406(materialized.clone(), values.clone(), source_ml_source1_temporary_index1403.clone());
                                                                                        self::execute_java_script_call_primitive(operation, materialized_2.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if ((self::read_java_script_call_integer_arity(operation)) != (0f64)) {
                                                            self::execute_java_script_call_integer_primitive(operation, values.clone())
                                                        } else {
                                                            if (self::test_java_script_call_unary_number_operation(operation) || self::ml_array::includes(&{ let ml_reference_value2144 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115])]))).borrow().clone(); ml_reference_value2144 }, operation, 0f64, false, |a, b| a == b)) {
                                                                self::execute_java_script_call_numeric_primitive(operation, values.clone())
                                                            } else {
                                                                self::execute_java_script_call_text_primitive(operation, values.clone())
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
            }
        }
    }
}
