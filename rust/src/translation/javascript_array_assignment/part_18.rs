// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193;
use super::MlSource1TemporaryExecuteJavaScriptCallTextConversionLoop194Result;
use super::construct_java_script_semantic_text_units;
use super::convert_java_script_big_integer_number;
use super::convert_java_script_boolean_big_integer;
use super::convert_java_script_number_big_integer;
use super::convert_java_script_string_big_integer_units;
use super::convert_java_script_string_number_units;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_integer_text;
use super::read_java_script_call_integer_width;
use super::read_java_script_integer_protocol_operation_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::refuse_java_script_call_integer_conversion;
use super::source_ml_source1_temporary_execute_java_script_call_text_conversion_loop194;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_integer_conversion(
    operation: &self::ml_string::Text,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let error = self::read_java_script_integer_protocol_operation_error(operation);
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())
            } else {
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::refuse_java_script_call_integer_conversion(
                            &(self::ml_string::Text::from_units(vec![
                                117, 110, 100, 101, 102, 105, 110, 101, 100,
                            ])),
                            &(self::ml_string::Text::from_units(vec![
                                117, 110, 100, 101, 102, 105, 110, 101, 100,
                            ])),
                        )
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::refuse_java_script_call_integer_conversion(
                            &(self::ml_string::Text::from_units(vec![110, 117, 108, 108])),
                            &(self::ml_string::Text::from_units(vec![110, 117, 108, 108])),
                        )
                    }
                    self::JavaScriptSemanticValue::Boolean(value_value) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                            self::JavaScriptSemanticValue::Integer(
                                self::convert_java_script_boolean_big_integer(value_value),
                            ),
                        ))
                    }
                    self::JavaScriptSemanticValue::Number(value_value_2) => {
                        if (*operation)
                            == (self::ml_string::Text::from_units(vec![
                                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105,
                                118, 101, 32, 105, 110, 116, 101, 103, 101, 114,
                            ]))
                        {
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114,
                                    116, 32, 97, 32, 78, 117, 109, 98, 101, 114, 32, 118, 97, 108,
                                    117, 101, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116,
                                ]),
                            )
                        } else {
                            {
                                let converted =
                                    self::convert_java_script_number_big_integer(value_value_2);
                                {
                                    let ml_product_condition_3 = {
                                        let ml_reference_array213 = converted.clone();
                                        ml_reference_array213.borrow().len() as f64
                                    };
                                    if ml_product_condition_3 {
                                        self::refuse_java_script_call_integer_conversion(
                                            &(self::ml_string::Text::from_units(vec![
                                                110, 117, 109, 98, 101, 114,
                                            ])),
                                            &(self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(value_value_2),
                                            )),
                                        )
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                            Box::new(self::JavaScriptSemanticValue::Integer(
                                                ({
                                                    let source_ml_source2_array_read_index18082 =
                                                        0f64;
                                                    {
                                                        let ml_reference_array214 =
                                                            converted.clone();
                                                        let ml_reference_position214 =
                                                            source_ml_source2_array_read_index18082;
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array214.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position214,
                                                            ),
                                                        )
                                                    }
                                                })
                                                .clone()
                                                .unwrap_or_else(|| self::ml::Big::from_i128(0)),
                                            )),
                                        )
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        let ml_product_condition_2 = {
                            let ml_reference_array215 = units.clone();
                            ml_reference_array215.borrow().len() as f64
                        };
                        if ml_product_condition_2 {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                                self::ml_string::Text::from_units(vec![
                                    66, 105, 103, 73, 110, 116, 32, 116, 101, 120, 116, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99,
                                    101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101,
                                    99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116,
                                    32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101,
                                ]),
                            )
                        } else {
                            {
                                let converted_2 =
                                    self::convert_java_script_string_big_integer_units(
                                        units.clone(),
                                    );
                                {
                                    let ml_product_condition_1 = {
                                        let ml_reference_array216 = converted_2.clone();
                                        ml_reference_array216.borrow().len() as f64
                                    };
                                    if ml_product_condition_1 {
                                        self::refuse_java_script_call_integer_conversion(
                                            &(self::ml_string::Text::from_units(vec![
                                                115, 116, 114, 105, 110, 103,
                                            ])),
                                            &(self::read_java_script_call_integer_text(
                                                units.clone(),
                                            )),
                                        )
                                    } else {
                                        {
                                            let integer = ({
                                                let source_ml_source2_array_read_index18084 = 0f64;
                                                {
                                                    let ml_reference_array217 = converted_2.clone();
                                                    let ml_reference_position217 =
                                                        source_ml_source2_array_read_index18084;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array217.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position217,
                                                        ),
                                                    )
                                                }
                                            })
                                            .clone()
                                            .unwrap_or_else(|| self::ml::Big::from_i128(0));
                                            if (self::read_java_script_call_integer_width(
                                                integer.clone(),
                                            )) < 0f64
                                            {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                            } else {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                                    Box::new(
                                                        self::JavaScriptSemanticValue::Integer(
                                                            integer.clone(),
                                                        ),
                                                    ),
                                                )
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::SourceText(_, text) => {
                        if (text.length()) > 4096f64 {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                                self::ml_string::Text::from_units(vec![
                                    66, 105, 103, 73, 110, 116, 32, 116, 101, 120, 116, 32, 99,
                                    111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99,
                                    101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101,
                                    99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116,
                                    32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101,
                                ]),
                            )
                        } else {
                            self::execute_java_script_call_integer_conversion(
                                operation,
                                self::JavaScriptSemanticValue::Text(
                                    self::construct_java_script_semantic_text_units(
                                        value.clone(),
                                        4096f64,
                                    ),
                                ),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveError(
                            self::ml_string::Text::from_units(vec![
                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116,
                                32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101,
                                32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116,
                            ]),
                        )
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                111, 98, 106, 101, 99, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99,
                                111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113,
                                117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84,
                                111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111,
                                116, 111, 99, 111, 108,
                            ]),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_number_conversion(
    operation: &self::ml_string::Text,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    if (((*operation)
        != (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110,
            117, 109, 98, 101, 114,
        ])))
        && ((*operation)
            != (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                110, 117, 109, 98, 101, 114,
            ]))))
        && ((*operation)
            != (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                110, 117, 109, 101, 114, 105, 99,
            ])))
    {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(
            vec![
                110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116,
                105, 111, 110,
            ],
        ))
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                    self::JavaScriptSemanticValue::Number(f64::NAN),
                ))
            }
            self::JavaScriptSemanticValue::Null => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                    self::JavaScriptSemanticValue::Number(0f64),
                ))
            }
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                    self::JavaScriptSemanticValue::Number(if value_value { 1f64 } else { 0f64 }),
                ))
            }
            self::JavaScriptSemanticValue::Number(_) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_2) => {
                if (*operation)
                    == (self::ml_string::Text::from_units(vec![
                        99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118,
                        101, 32, 110, 117, 109, 101, 114, 105, 99,
                    ]))
                {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
                } else if (*operation)
                    == (self::ml_string::Text::from_units(vec![
                        99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118,
                        101, 32, 110, 117, 109, 98, 101, 114,
                    ]))
                {
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(
                        self::ml_string::Text::from_units(vec![
                            84, 121, 112, 101, 69, 114, 114, 111, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32,
                            97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32,
                            116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114,
                        ]),
                    )
                } else if (self::read_java_script_call_integer_width(value_value_2.clone())) < 0f64
                {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                        self::ml_string::Text::from_units(vec![
                            101, 120, 112, 108, 105, 99, 105, 116, 32, 78, 117, 109, 98, 101, 114,
                            32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99,
                            101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116,
                            101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 119, 111, 114,
                            107, 32, 112, 114, 111, 102, 105, 108, 101,
                        ]),
                    )
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                        self::JavaScriptSemanticValue::Number(
                            self::convert_java_script_big_integer_number(value_value_2.clone()),
                        ),
                    ))
                }
            }
            self::JavaScriptSemanticValue::Text(units) => {
                let ml_product_condition_1 = {
                    let ml_reference_array218 = units.clone();
                    ml_reference_array218.borrow().len() as f64
                };
                if ml_product_condition_1 {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                        self::ml_string::Text::from_units(vec![
                            78, 117, 109, 98, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110,
                            118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100,
                            115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                            52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32,
                            112, 114, 111, 102, 105, 108, 101,
                        ]),
                    )
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                        self::JavaScriptSemanticValue::Number(
                            self::convert_java_script_string_number_units(units.clone()),
                        ),
                    ))
                }
            }
            self::JavaScriptSemanticValue::SourceText(_, text) => {
                if (text.length()) > 4096f64 {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                        self::ml_string::Text::from_units(vec![
                            78, 117, 109, 98, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110,
                            118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100,
                            115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                            52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32,
                            112, 114, 111, 102, 105, 108, 101,
                        ]),
                    )
                } else {
                    self::execute_java_script_call_number_conversion(
                        operation,
                        self::JavaScriptSemanticValue::Text(
                            self::construct_java_script_semantic_text_units(value.clone(), 4096f64),
                        ),
                    )
                }
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
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
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        111, 98, 106, 101, 99, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111,
                        110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114,
                        101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117,
                        115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105,
                        116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108,
                    ]),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_conversion(
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let text = self::ml_string::Text::from_units(vec![]);
        {
            let source_ml_source2_s18098 = match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Next(self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Next(self::ml_string::Text::from_units(vec![110, 117, 108, 108]))
                }
                self::JavaScriptSemanticValue::Boolean(value_value) => {
                    {
                        let text_2 = if value_value {
                            self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                        } else {
                            self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                        };
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Next(text_2)
                    }
                }
                self::JavaScriptSemanticValue::Number(value_value_2) => {
                    {
                        let text_3 = self::ml_string::Text::from_text(&self::ml_number::js_number(value_value_2));
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Next(text_3)
                    }
                }
                self::JavaScriptSemanticValue::Integer(value_value_3) => {
                    if (self::read_java_script_call_integer_width(value_value_3.clone())) < 0f64 {
                        self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))))
                    } else {
                        {
                            let text_4 = self::ml_string::Text::from_text(&value_value_3.to_string());
                            self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Next(text_4)
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(_) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))))
                }
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))))
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 115, 116, 114, 105, 110, 103]))))
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))))
                }
            };
            match source_ml_source2_s18098 {
                self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Next(text_5) => {
                    {
                        let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let index = 0f64;
                            {
                                let source_ml_source2_s18107 = self::source_ml_source1_temporary_execute_java_script_call_text_conversion_loop194(&(text_5), units.clone(), index);
                                match source_ml_source2_s18107 {
                                    self::MlSource1TemporaryExecuteJavaScriptCallTextConversionLoop194Result::MlSource1TemporaryExecuteJavaScriptCallTextConversionLoop194Done => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units.clone())))
                                    }
                                }
                            }
                        }
                    }
                }
                self::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193::MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin193Return(source_ml_source1_temporary_result195) => {
                    let source_ml_source1_temporary_result195 = *source_ml_source1_temporary_result195;
                    source_ml_source1_temporary_result195.clone()
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_scalar_conversion_error(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (((((((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110,
            117, 109, 98, 101, 114,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                110, 117, 109, 98, 101, 114,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                110, 117, 109, 101, 114, 105, 99,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101,
                32, 118, 97, 108, 117, 101,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 118, 101, 114, 116, 32, 110, 117, 109, 98, 101, 114, 32, 112, 114,
                105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101,
                32, 116, 101, 120, 116,
            ]))))
        || ((self::read_java_script_integer_protocol_operation_error(operation))
            == (self::ml_string::Text::from_units(vec![])))
    {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
            114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
            108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 78, 117, 109, 98, 101,
            114, 44, 32, 105, 110, 116, 101, 103, 101, 114, 32, 111, 114, 32, 116, 101, 120, 116,
            32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_scalar_conversion(
    operation: &self::ml_string::Text,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallPrimitiveResult {
    if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
            118, 97, 108, 117, 101,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 118, 101, 114, 116, 32, 110, 117, 109, 98, 101, 114, 32, 112, 114,
                105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101,
            ])))
    {
        if match value.clone() {
            self::JavaScriptSemanticValue::Object(_, _) => true,
            _ => false,
        } {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                self::ml_string::Text::from_units(vec![
                    111, 98, 106, 101, 99, 116, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118,
                    101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                    116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 45, 104, 105,
                    110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110,
                ]),
            )
        } else {
            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
        }
    } else if (((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110,
            117, 109, 98, 101, 114,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                110, 117, 109, 98, 101, 114,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                110, 117, 109, 101, 114, 105, 99,
            ])))
    {
        self::execute_java_script_call_number_conversion(operation, value.clone())
    } else if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105,
            110, 116, 101, 103, 101, 114,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
                105, 110, 116, 101, 103, 101, 114,
            ])))
    {
        self::execute_java_script_call_integer_conversion(operation, value.clone())
    } else if (*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
            116, 101, 120, 116,
        ]))
    {
        self::execute_java_script_call_text_conversion(value.clone())
    } else {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
            self::read_java_script_call_scalar_conversion_error(operation),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_conversion_spelling(
    name: &self::ml_string::Text,
) -> self::JavaScriptSemanticValue {
    if (*name) == (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])) {
        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![
            100f64, 101f64, 102f64, 97f64, 117f64, 108f64, 116f64,
        ])))
    } else if (*name) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])) {
        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![
            110f64, 117f64, 109f64, 98f64, 101f64, 114f64,
        ])))
    } else if (*name) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) {
        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![
            115f64, 116f64, 114f64, 105f64, 110f64, 103f64,
        ])))
    } else if (*name) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102]))
    {
        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![
            118f64, 97f64, 108f64, 117f64, 101f64, 79f64, 102f64,
        ])))
    } else if (*name)
        == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]))
    {
        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![
            116f64, 111f64, 83f64, 116f64, 114f64, 105f64, 110f64, 103f64,
        ])))
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101,
                114, 115, 105, 111, 110, 32, 115, 112, 101, 108, 108, 105, 110, 103, 32, 110, 101,
                101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103,
                105, 115, 116, 101, 114, 101, 100, 32, 110, 97, 109, 101,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_conversion_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
) -> self::ml_string::Text {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                error.clone()
            } else if {
                let source_ml_source2_s18118 = match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                        source_ml_source2_data_property_value18128,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value18128 =
                            *source_ml_source2_data_property_value18128;
                        source_ml_source2_data_property_value18128.clone()
                    }
                };
                match source_ml_source2_s18118 {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                }
            } {
                self::ml_string::Text::from_units(vec![
                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118,
                    101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116,
                    105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                    99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101,
                    105, 118, 101, 114,
                ])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                source_ml_source2_data_property_value18129,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value18129 =
                                    *source_ml_source2_data_property_value18129;
                                source_ml_source2_data_property_value18129.clone()
                            }
                        },
                    );
                    {
                        let ml_library_inequality_2 =
                            receiver_error != (self::ml_string::Text::from_units(vec![]));
                        if ml_library_inequality_2 {
                            receiver_error.clone()
                        } else if {
                            let source_ml_source2_s18131 = match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    source_ml_source2_data_property_value18141,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value18141 =
                                        *source_ml_source2_data_property_value18141;
                                    source_ml_source2_data_property_value18141.clone()
                                }
                            };
                            match source_ml_source2_s18131 {
                                self::JavaScriptSemanticValue::Symbol(_) => false,
                                _ => true,
                            }
                        } {
                            self::ml_string::Text::from_units(vec![
                                116, 104, 101, 32, 101, 120, 111, 116, 105, 99, 32, 109, 101, 116,
                                104, 111, 100, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98,
                                101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114,
                                101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 119, 101, 108,
                                108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108,
                            ])
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value18142, _, _) => {
                                        let source_ml_source2_data_property_value18142 = *source_ml_source2_data_property_value18142;
                                        source_ml_source2_data_property_value18142.clone()
                                    }
                                });
                                {
                                    let ml_library_inequality_1 =
                                        key_error != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_1 {
                                    key_error.clone()
                                } else if (((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value18144, _) => {
                                            source_ml_source2_data_property_value18144.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value18146, _) => {
                                            source_ml_source2_data_property_value18146.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value18148, _) => {
                                            source_ml_source2_data_property_value18148.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116])
                                    } else if (self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])]))).borrow().clone() }, &match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value18149) => {
                                                source_ml_source2_data_property_value18149.clone()
                                            }
                                        }, 0f64, false, |a, b| a == b)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
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
pub fn continue_java_script_primitive_conversion(
    state: self::JavaScriptPrimitiveConversionState,
    phase: &self::ml_string::Text,
) -> self::JavaScriptPrimitiveConversionTransition {
    self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(Box::new(
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            Box::new(match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    source_ml_source2_data_property_value18150,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value18150 =
                        *source_ml_source2_data_property_value18150;
                    source_ml_source2_data_property_value18150.clone()
                }
            }),
            Box::new(match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    source_ml_source2_data_property_value18151,
                    _,
                    _,
                ) => {
                    let source_ml_source2_data_property_value18151 =
                        *source_ml_source2_data_property_value18151;
                    source_ml_source2_data_property_value18151.clone()
                }
            }),
            match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    source_ml_source2_data_property_value18152,
                    _,
                ) => source_ml_source2_data_property_value18152.clone(),
            },
            phase.clone(),
        ),
    ))
}
