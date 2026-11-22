// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAuthorityUpdate;
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructorRealm;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1387Result;
use super::construct_java_script_semantic_text_units;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_source_exception;
use super::ml_string;
use super::ml_string_number;
use super::read_java_script_constructor_realm_fallback;
use super::read_java_script_semantic_value_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_integer_radix_text_loop1387(
    mut remaining: self::ml::Big,
    mut divisor: self::ml::Big,
    mut reversed: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> self::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1387Result {
    loop {
        return {
            let digit = self::ml_string_number::convert_java_script_big_integer_number({
                let source_ml_source2_integer_division42099 =
                    self::ml_big_integer_conversion::divide_java_script_big_integer(
                        remaining.clone(),
                        divisor.clone(),
                    );
                if ((source_ml_source2_integer_division42099.len() as f64) == (2f64)) {
                    (self::ml_array::optional_at(
                        &source_ml_source2_integer_division42099,
                        self::ml_array::number_index(1f64),
                    ))
                    .clone()
                    .unwrap_or_else(|| self::ml::Big::from_i128(0))
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("RangeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114,
                            111,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                }
            });
            {
                let source_ml_source1_temporary_execute_java_script_call_integer_radix_text_ignored1392 = {
                    let ml_method_owner3160 = reversed.clone();
                    let ml_method_argument3160_0 = if (digit < (10f64)) {
                        (48f64 + digit)
                    } else {
                        (87f64 + digit)
                    };
                    let ml_inserted = vec![ml_method_argument3160_0];
                    let mut ml_method_values3160 = ml_method_owner3160.borrow_mut();
                    let ml_length = ml_method_values3160
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3160.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let remaining_2 = {
                        let source_ml_source2_integer_division42100 =
                            self::ml_big_integer_conversion::divide_java_script_big_integer(
                                remaining.clone(),
                                divisor.clone(),
                            );
                        if ((source_ml_source2_integer_division42100.len() as f64) == (2f64)) {
                            (self::ml_array::optional_at(
                                &source_ml_source2_integer_division42100,
                                self::ml_array::number_index(0f64),
                            ))
                            .clone()
                            .unwrap_or_else(|| self::ml::Big::from_i128(0))
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("RangeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122,
                                    101, 114, 111,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    };
                    if (remaining_2 > (self::ml::Big::from_i128(0))) {
                        {
                            remaining = remaining_2.clone();
                            continue;
                        }
                    } else {
                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1387Result::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1387Done
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructUtf16TextRepetitionLoop1393Result {
    MlSource1TemporaryConstructUtf16TextRepetitionLoop1393Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_utf16_text_repetition_loop1393(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut length: f64,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryConstructUtf16TextRepetitionLoop1393Result {
    loop {
        return if (index < length) {
            {
                let source_ml_source1_temporary_construct_utf16_text_repetition_ignored1394 = {
                    let ml_method_owner3161 = result.clone();
                    let ml_method_argument3161_0 = ({
                        let source_ml_source2_array_read_index42102 = (index % {
                            let ml_reference_array3162 = text.clone();
                            let ml_reference_projection3162 =
                                ml_reference_array3162.borrow().len() as f64;
                            ml_reference_projection3162
                        });
                        {
                            let ml_reference_array3163 = text.clone();
                            let ml_reference_position3163 = source_ml_source2_array_read_index42102;
                            let ml_reference_projection3163 = self::ml_array::optional_at(
                                &*ml_reference_array3163.borrow(),
                                self::ml_array::number_index(ml_reference_position3163),
                            );
                            ml_reference_projection3163
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| 0f64);
                    let ml_inserted = vec![ml_method_argument3161_0];
                    let mut ml_method_values3161 = ml_method_owner3161.borrow_mut();
                    let ml_length = ml_method_values3161
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3161.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConstructUtf16TextRepetitionLoop1393Result::MlSource1TemporaryConstructUtf16TextRepetitionLoop1393Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructUtf16TextPaddingLoop1395Result {
    MlSource1TemporaryConstructUtf16TextPaddingLoop1395Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_utf16_text_padding_loop1395(
    mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut filler: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut length: f64,
    mut start: bool,
    mut added: f64,
    mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryConstructUtf16TextPaddingLoop1395Result {
    loop {
        return if (index < length) {
            {
                let original = if start {
                    (index >= added)
                } else {
                    (index
                        < ({
                            let ml_reference_array3164 = text.clone();
                            let ml_reference_projection3164 =
                                ml_reference_array3164.borrow().len() as f64;
                            ml_reference_projection3164
                        }))
                };
                {
                    let position = if original {
                        if start { (index - added) } else { index }
                    } else {
                        if start {
                            index
                        } else {
                            (index - {
                                let ml_reference_array3165 = text.clone();
                                let ml_reference_projection3165 =
                                    ml_reference_array3165.borrow().len() as f64;
                                ml_reference_projection3165
                            })
                        }
                    };
                    {
                        let source_ml_source1_temporary_construct_utf16_text_padding_ignored1396 = {
                            let ml_method_owner3166 = result.clone();
                            let ml_method_argument3166_0 = (if original {
                                {
                                    let ml_reference_array3167 = text.clone();
                                    let ml_reference_position3167 = position;
                                    let ml_reference_projection3167 = self::ml_array::optional_at(
                                        &*ml_reference_array3167.borrow(),
                                        self::ml_array::number_index(ml_reference_position3167),
                                    );
                                    ml_reference_projection3167
                                }
                            } else {
                                {
                                    let source_ml_source2_array_read_index42106 = (position % {
                                        let ml_reference_array3168 = filler.clone();
                                        let ml_reference_projection3168 =
                                            ml_reference_array3168.borrow().len() as f64;
                                        ml_reference_projection3168
                                    });
                                    {
                                        let ml_reference_array3169 = filler.clone();
                                        let ml_reference_position3169 =
                                            source_ml_source2_array_read_index42106;
                                        let ml_reference_projection3169 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array3169.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position3169,
                                                ),
                                            );
                                        ml_reference_projection3169
                                    }
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| 0f64);
                            let ml_inserted = vec![ml_method_argument3166_0];
                            let mut ml_method_values3166 = ml_method_owner3166.borrow_mut();
                            let ml_length = ml_method_values3166
                                .len()
                                .checked_add(ml_inserted.len())
                                .filter(|length| *length <= 4294967295usize)
                                .expect("Invalid array length");
                            ml_method_values3166.extend(ml_inserted);
                            ml_length as f64
                        };
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConstructUtf16TextPaddingLoop1395Result::MlSource1TemporaryConstructUtf16TextPaddingLoop1395Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Result {
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Done,
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Return(
        Box<self::JavaScriptCallPrimitiveResult>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop1398(
    mut numbers: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_source1_temporary_values1397: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1397: self::ml::Big,
) -> self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Result {
    loop {
        return if (source_ml_source1_temporary_index1397
            < ({
                let ml_reference_array3170 = source_ml_source1_temporary_values1397.clone();
                let ml_reference_projection3170 =
                    self::ml::Big::from_u128(ml_reference_array3170.borrow().len() as u128);
                ml_reference_projection3170
            })) {
            {
                let value = {
                    let ml_reference_array3171 = source_ml_source1_temporary_values1397.clone();
                    let ml_reference_position3171 = source_ml_source1_temporary_index1397.clone();
                    let ml_reference_projection3171 = self::ml_array::at(
                        &*ml_reference_array3171.borrow(),
                        ml_reference_position3171.to_index(),
                    );
                    ml_reference_projection3171
                };
                match value.clone() {
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        {
                            let source_ml_source1_temporary_execute_java_script_call_numeric_primitive_ignored1399 = { let ml_method_owner3172 = numbers.clone(); let ml_method_argument3172_0 = value_value; let ml_inserted = vec![ml_method_argument3172_0]; let mut ml_method_values3172 = ml_method_owner3172.borrow_mut(); let ml_length = ml_method_values3172.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values3172.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_temporary_index1397_2 = source_ml_source1_temporary_index1397.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index1397 = source_ml_source1_temporary_index1397_2.clone();
                                    continue;
                                }
                            }
                        }
                    }
                    _ => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop1398Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_text_replacement_work_error_loop1400(
    mut length: f64,
    mut search: f64,
    mut plan: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut width: f64,
    mut index: f64,
) -> f64 {
    loop {
        return if (index
            < ({
                let ml_reference_array3173 = plan.clone();
                let ml_reference_projection3173 = ml_reference_array3173.borrow().len() as f64;
                ml_reference_projection3173
            })) {
            {
                let kind = ({
                    let ml_reference_array3174 = plan.clone();
                    let ml_reference_position3174 = index;
                    let ml_reference_projection3174 = self::ml_array::optional_at(
                        &*ml_reference_array3174.borrow(),
                        self::ml_array::number_index(ml_reference_position3174),
                    );
                    ml_reference_projection3174
                })
                .clone()
                .unwrap_or_else(|| 0f64);
                {
                    let width_2 = (width
                        + if (kind == (0f64)) {
                            (({
                                let source_ml_source2_array_read_index42112 = (index + 2f64);
                                {
                                    let ml_reference_array3175 = plan.clone();
                                    let ml_reference_position3175 =
                                        source_ml_source2_array_read_index42112;
                                    let ml_reference_projection3175 = self::ml_array::optional_at(
                                        &*ml_reference_array3175.borrow(),
                                        self::ml_array::number_index(ml_reference_position3175),
                                    );
                                    ml_reference_projection3175
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| 0f64)
                                - ({
                                    let source_ml_source2_array_read_index42114 = (index + 1f64);
                                    {
                                        let ml_reference_array3176 = plan.clone();
                                        let ml_reference_position3176 =
                                            source_ml_source2_array_read_index42114;
                                        let ml_reference_projection3176 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array3176.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position3176,
                                                ),
                                            );
                                        ml_reference_projection3176
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| 0f64))
                        } else {
                            if (kind == (1f64)) { search } else { length }
                        });
                    {
                        let index_2 = (index + 3f64);
                        {
                            (width, index) = (width_2, index_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            width
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Result {
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Done,
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Return(
        Box<self::JavaScriptCallPrimitiveResult>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_primitive_loop1402(
    mut source_ml_source1_temporary_values1401: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1401: self::ml::Big,
) -> self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Result {
    loop {
        return if (source_ml_source1_temporary_index1401
            < ({
                let ml_reference_array3177 = source_ml_source1_temporary_values1401.clone();
                let ml_reference_projection3177 =
                    self::ml::Big::from_u128(ml_reference_array3177.borrow().len() as u128);
                ml_reference_projection3177
            })) {
            {
                let value = {
                    let ml_reference_array3178 = source_ml_source1_temporary_values1401.clone();
                    let ml_reference_position3178 = source_ml_source1_temporary_index1401.clone();
                    let ml_reference_projection3178 = self::ml_array::at(
                        &*ml_reference_array3178.borrow(),
                        ml_reference_position3178.to_index(),
                    );
                    ml_reference_projection3178
                };
                {
                    let error = self::read_java_script_semantic_value_error(value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index1401_2 =
                                source_ml_source1_temporary_index1401
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1401 =
                                    source_ml_source1_temporary_index1401_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallPrimitiveScalarLoop1404Result {
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveScalarLoop1404Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_primitive_scalar_loop1404(
    kind: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveScalarLoop1404Result {
    loop {
        return if (at < (kind.length())) {
            {
                let source_ml_source1_temporary_execute_java_script_call_primitive_scalar_ignored1405 = {
                    let ml_method_owner3179 = units.clone();
                    let ml_method_argument3179_0 = {
                        let source_ml_source2_string_unit_operand42117 = kind.clone();
                        {
                            let source_ml_source2_string_unit_operand42118 = at;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if ((index < (0f64)) || (index >= (text.length()))) {
                                            f64::NAN
                                        } else {
                                            ({
                                    let source_ml_source2_string_unit_operand42124 = {
                                        let source_ml_source2_string_range_operand42119 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand42120 = index;
                                            {
                                                let source_ml_source2_string_range_operand42121 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position42122 = source_ml_source2_string_range_operand42120;
                                                    {
                                                        let source_ml_source2_string_range_position42123 = source_ml_source2_string_range_operand42121;
                                                        source_ml_source2_string_range_operand42119.slice(source_ml_source2_string_range_position42122, source_ml_source2_string_range_position42123)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand42125 = 0f64;
                                        source_ml_source2_string_unit_operand42124.code_point_at(source_ml_source2_string_unit_operand42125)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand42117.clone(),
                                source_ml_source2_string_unit_operand42118,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument3179_0];
                    let mut ml_method_values3179 = ml_method_owner3179.borrow_mut();
                    let ml_length = ml_method_values3179
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3179.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveScalarLoop1404Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveScalarLoop1404Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_primitive_scalar_loop1406(
    mut materialized: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_source1_temporary_values1403: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1403: self::ml::Big,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if (source_ml_source1_temporary_index1403
            < ({
                let ml_reference_array3180 = source_ml_source1_temporary_values1403.clone();
                let ml_reference_projection3180 =
                    self::ml::Big::from_u128(ml_reference_array3180.borrow().len() as u128);
                ml_reference_projection3180
            })) {
            {
                let value = {
                    let ml_reference_array3181 = source_ml_source1_temporary_values1403.clone();
                    let ml_reference_position3181 = source_ml_source1_temporary_index1403.clone();
                    let ml_reference_projection3181 = self::ml_array::at(
                        &*ml_reference_array3181.borrow(),
                        ml_reference_position3181.to_index(),
                    );
                    ml_reference_projection3181
                };
                {
                    let operand = if match value.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => true,
                        _ => false,
                    } {
                        self::JavaScriptSemanticValue::Text(
                            self::construct_java_script_semantic_text_units(
                                value.clone(),
                                1048576f64,
                            ),
                        )
                    } else {
                        value.clone()
                    };
                    {
                        let materialized_2 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_concat_value42131 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![operand.clone()]));
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<self::JavaScriptSemanticValue>::new(),
                                    {
                                        let ml_reference_value3182 = materialized.borrow().clone();
                                        ml_reference_value3182
                                    },
                                ),
                                {
                                    let ml_reference_value3183 =
                                        source_ml_source2_concat_value42131.borrow().clone();
                                    ml_reference_value3183
                                },
                            )
                        }));
                        {
                            let source_ml_source1_temporary_index1403_2 =
                                source_ml_source1_temporary_index1403
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (materialized, source_ml_source1_temporary_index1403) = (
                                    materialized_2.clone(),
                                    source_ml_source1_temporary_index1403_2.clone(),
                                );
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            materialized.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Result {
    MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Done,
    MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_selected_call_frame_error_loop1407(
    mut frame: self::JavaScriptCallFrame,
    mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3184 = arguments_.clone();
                let ml_reference_projection3184 = ml_reference_array3184.borrow().len() as f64;
                ml_reference_projection3184
            })) {
            if !(self::test_java_script_semantic_same_value(
                ({
                    let ml_reference_array3185 = arguments_.clone();
                    let ml_reference_position3185 = at;
                    let ml_reference_projection3185 = self::ml_array::optional_at(
                        &*ml_reference_array3185.borrow(),
                        self::ml_array::number_index(ml_reference_position3185),
                    );
                    ml_reference_projection3185
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                ({
                    let source_ml_source2_array_read_value42135 = match frame.clone() {
                        self::JavaScriptCallFrame::CallFrame(
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value42134,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value42134.clone(),
                    };
                    {
                        let ml_reference_array3186 =
                            source_ml_source2_array_read_value42135.clone();
                        let ml_reference_position3186 = at;
                        let ml_reference_projection3186 = self::ml_array::optional_at(
                            &*ml_reference_array3186.borrow(),
                            self::ml_array::number_index(ml_reference_position3186),
                        );
                        ml_reference_projection3186
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
            )) {
                self::MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Result::MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Return(self::ml_string::Text::from_units(vec![97, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 101, 114, 101, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116]))
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Result::MlSource1TemporaryReadJavaScriptSelectedCallFrameErrorLoop1407Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Result {
    MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Done,
    MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Return(
        Box<self::JavaScriptArrayAuthorityUpdate>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Result {
    MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Done,
    MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1423Return(
        Box<self::JavaScriptArrayAuthorityUpdate>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Result {
    MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Done,
    MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Return(
        Box<self::JavaScriptArrayAuthorityUpdate>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_extend_java_script_call_array_authority_loop1424(
    mut program: self::JavaScriptCallProgram,
    mut machine: self::JavaScriptCallMachine,
    mut root: self::JavaScriptConstructorRealm,
    mut next: self::JavaScriptConstructorRealm,
    mut index: f64,
) -> self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Result {
    loop {
        return if (index < (9f64)) {
            {
                let value = self::read_java_script_constructor_realm_fallback(root.clone(), index);
                {
                    let updated =
                        self::read_java_script_constructor_realm_fallback(next.clone(), index);
                    if (!(match (value.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && (match (updated.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || !(self::test_java_script_semantic_same_value(
                        updated.clone().expect("a proven present value"),
                        value.clone().expect("a proven present value"),
                    )))) {
                        self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Return(Box::new(self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(Box::new(program.clone()), Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 112, 114, 101, 115, 101, 114, 118, 101, 115, 32, 101, 118, 101, 114, 121, 32, 101, 115, 116, 97, 98, 108, 105, 115, 104, 101, 100, 32, 114, 101, 97, 108, 109, 32, 114, 111, 111, 116]))))
                    } else {
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1424Done
        };
    }
}
