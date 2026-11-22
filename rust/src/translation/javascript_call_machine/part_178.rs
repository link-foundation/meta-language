// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin1379;
use super::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop1382Result;
use super::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Result;
use super::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1372Result;
use super::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Result;
use super::apply_java_script_number_bitwise_operation;
use super::construct_java_script_semantic_text_units;
use super::dependency_function1;
use super::dependency_function5;
use super::map_unicode_case_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_array_index;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_text_string;
use super::read_java_script_semantic_text_unit;
use super::read_utf16_text_integer;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop1380;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop1381;
use super::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop1382;
use super::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1366;
use super::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1372;
use super::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1374;
use super::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1375;
use super::test_java_script_array_length_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_string_method_primitive(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let receiver = ({
            let source_ml_source2_array_read_index32583 = 0f64;
            {
                let ml_reference_array2034 = values.clone();
                let ml_reference_position2034 = source_ml_source2_array_read_index32583;
                let ml_reference_projection2034 = self::ml_array::optional_at(
                    &*ml_reference_array2034.borrow(),
                    self::ml_array::number_index(ml_reference_position2034),
                );
                ml_reference_projection2034
            }
        })
        .clone()
        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let second = ({
                let source_ml_source2_array_read_index32585 = 1f64;
                {
                    let ml_reference_array2035 = values.clone();
                    let ml_reference_position2035 = source_ml_source2_array_read_index32585;
                    let ml_reference_projection2035 = self::ml_array::optional_at(
                        &*ml_reference_array2035.borrow(),
                        self::ml_array::number_index(ml_reference_position2035),
                    );
                    ml_reference_projection2035
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            {
                let third = ({
                    let source_ml_source2_array_read_index32587 = 2f64;
                    {
                        let ml_reference_array2036 = values.clone();
                        let ml_reference_position2036 = source_ml_source2_array_read_index32587;
                        let ml_reference_projection2036 = self::ml_array::optional_at(
                            &*ml_reference_array2036.borrow(),
                            self::ml_array::number_index(ml_reference_position2036),
                        );
                        ml_reference_projection2036
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                if (((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        99, 111, 110, 118, 101, 114, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101,
                        114, 32, 99, 111, 100, 101, 32, 116, 101, 120, 116,
                    ])))
                    || ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            99, 111, 110, 118, 101, 114, 116, 32, 99, 111, 100, 101, 32, 112, 111,
                            105, 110, 116, 32, 116, 101, 120, 116,
                        ]))))
                {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Number(value) => {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    99, 111, 110, 118, 101, 114, 116, 32, 99, 104, 97, 114, 97, 99,
                                    116, 101, 114, 32, 99, 111, 100, 101, 32, 116, 101, 120, 116,
                                ])))
                            {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                    self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                        std::cell::RefCell::new(vec![
                                            (self::dependency_function5(value) % 65536f64),
                                        ]),
                                    )),
                                ))
                            } else {
                                if ((!(self::ml_math::is_integer(value)) || (value < (0f64)))
                                    || (value > (1114111f64)))
                                {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                        self::ml_string::Text::from_units(vec![
                                            82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                                        ]),
                                        (self::ml_string::Text::from_units(vec![
                                            73, 110, 118, 97, 108, 105, 100, 32, 99, 111, 100, 101,
                                            32, 112, 111, 105, 110, 116, 32,
                                        ]))
                                        .concat(
                                            &self::ml_string::Text::from_text(
                                                &self::ml_number::js_number(value),
                                            ),
                                        ),
                                    )
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                        self::JavaScriptSemanticValue::Text(
                                            if (value <= (65535f64)) {
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    if (value == (0f64)) { 0f64 } else { value },
                                                ]))
                                            } else {
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                    (55296f64
                                                        + ((value - 65536f64) / 1024f64).floor()),
                                                    (56320f64 + ((value - 65536f64) % 1024f64)),
                                                ]))
                                            },
                                        ),
                                    ))
                                }
                            }
                        }
                        _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                83, 116, 114, 105, 110, 103, 32, 99, 111, 110, 115, 116, 114, 117,
                                99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 117, 109, 101, 115,
                                32, 101, 97, 99, 104, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111,
                                110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114,
                                32, 97, 114, 103, 117, 109, 101, 110, 116,
                            ]),
                        ),
                    }
                } else {
                    if ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            114, 101, 113, 117, 105, 114, 101, 32, 83, 116, 114, 105, 110, 103, 32,
                            114, 101, 99, 101, 105, 118, 101, 114,
                        ])))
                    {
                        if (match second.clone() {
                            self::JavaScriptSemanticValue::Text(_) => false,
                            _ => true,
                        } && match second.clone() {
                            self::JavaScriptSemanticValue::SourceText(_, _) => false,
                            _ => true,
                        }) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    83, 116, 114, 105, 110, 103, 32, 114, 101, 99, 101, 105, 118,
                                    101, 114, 32, 99, 104, 101, 99, 107, 115, 32, 114, 101, 116,
                                    97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116,
                                    117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100,
                                    32, 109, 101, 116, 104, 111, 100, 32, 110, 97, 109, 101,
                                ]),
                            )
                        } else {
                            if (match receiver.clone() {
                                self::JavaScriptSemanticValue::Null => true,
                                _ => false,
                            } || match receiver.clone() {
                                self::JavaScriptSemanticValue::Undefined => true,
                                _ => false,
                            }) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                    self::ml_string::Text::from_units(vec![
                                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                                    ]),
                                    ((self::ml_string::Text::from_units(vec![
                                        83, 116, 114, 105, 110, 103, 46, 112, 114, 111, 116, 111,
                                        116, 121, 112, 101, 46,
                                    ]))
                                    .concat(
                                        &self::read_java_script_semantic_text_string(
                                            second.clone(),
                                        ),
                                    ))
                                    .concat(
                                        &self::ml_string::Text::from_units(vec![
                                            32, 99, 97, 108, 108, 101, 100, 32, 111, 110, 32, 110,
                                            117, 108, 108, 32, 111, 114, 32, 117, 110, 100, 101,
                                            102, 105, 110, 101, 100,
                                        ]),
                                    ),
                                )
                            } else {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                    receiver.clone(),
                                ))
                            }
                        }
                    } else {
                        if (match receiver.clone() {
                            self::JavaScriptSemanticValue::Text(_) => false,
                            _ => true,
                        } && match receiver.clone() {
                            self::JavaScriptSemanticValue::SourceText(_, _) => false,
                            _ => true,
                        }) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    83, 116, 114, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100,
                                    115, 32, 99, 111, 110, 115, 117, 109, 101, 32, 116, 104, 101,
                                    105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118,
                                    101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 114, 101,
                                    99, 101, 105, 118, 101, 114,
                                ]),
                            )
                        } else {
                            {
                                let length =
                                    self::read_java_script_semantic_text_length(receiver.clone());
                                if (((*operation)
                                    == (self::ml_string::Text::from_units(vec![
                                        109, 97, 112, 32, 108, 111, 119, 101, 114, 32, 83, 116,
                                        114, 105, 110, 103, 32, 99, 97, 115, 101,
                                    ])))
                                    || ((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            109, 97, 112, 32, 117, 112, 112, 101, 114, 32, 83, 116,
                                            114, 105, 110, 103, 32, 99, 97, 115, 101,
                                        ]))))
                                {
                                    if (match second.clone() {
                                        self::JavaScriptSemanticValue::Text(_) => false,
                                        _ => true,
                                    } && match second.clone() {
                                        self::JavaScriptSemanticValue::SourceText(_, _) => false,
                                        _ => true,
                                    }) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                            self::ml_string::Text::from_units(vec![
                                                83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101,
                                                32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                                                110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                                                32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108,
                                                32, 97, 99, 116, 105, 118, 101, 32, 85, 110, 105,
                                                99, 111, 100, 101, 32, 112, 114, 111, 102, 105,
                                                108, 101,
                                            ]),
                                        )
                                    } else {
                                        {
                                            let profile_error =
                                                self::read_java_script_string_case_profile_error(
                                                    &(self::read_java_script_semantic_text_string(
                                                        second.clone(),
                                                    )),
                                                );
                                            if (profile_error
                                                != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(profile_error.clone())
                                            } else {
                                                if (length > (1048576f64)) {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 105, 110, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                } else {
                                                    {
                                                        let units = self::map_unicode_case_utf16_text(self::construct_java_script_semantic_text_units(receiver.clone(), 1048576f64), ((*operation) == (self::ml_string::Text::from_units(vec![109, 97, 112, 32, 117, 112, 112, 101, 114, 32, 83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101]))));
                                                        if (({
                                                            let ml_reference_array2037 =
                                                                units.clone();
                                                            let ml_reference_projection2037 =
                                                                ml_reference_array2037
                                                                    .borrow()
                                                                    .len()
                                                                    as f64;
                                                            ml_reference_projection2037
                                                        }) > (1048576f64))
                                                        {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                        } else {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units.clone())))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if (((*operation)
                                        == (self::ml_string::Text::from_units(vec![
                                            116, 101, 115, 116, 32, 119, 101, 108, 108, 32, 102,
                                            111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110,
                                            103,
                                        ])))
                                        || ((*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 111, 112, 121, 32, 119, 101, 108, 108, 32, 102,
                                                111, 114, 109, 101, 100, 32, 83, 116, 114, 105,
                                                110, 103,
                                            ]))))
                                    {
                                        if (length > (1048576f64)) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 112, 114, 111, 99, 101, 115, 115, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                        } else {
                                            {
                                                let units_2 = std::rc::Rc::new(
                                                    std::cell::RefCell::new(Vec::<f64>::new()),
                                                );
                                                {
                                                    let formed = true;
                                                    {
                                                        let at = 0f64;
                                                        {
                                                            let source_ml_source2_s32598 = self::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1366(operation, receiver.clone(), length, units_2.clone(), formed, at);
                                                            match source_ml_source2_s32598.clone() {
                                                                self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Done(formed_2) => {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103]))) {
                                                                        self::JavaScriptSemanticValue::Boolean(formed_2)
                                                                    } else {
                                                                        if formed_2 {
                                                                            receiver.clone()
                                                                        } else {
                                                                            self::JavaScriptSemanticValue::Text(units_2.clone())
                                                                        }
                                                                    }))
                                                                }
                                                                self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1366Return(source_ml_source1_temporary_result1366) => {
                                                                    let source_ml_source1_temporary_result1366 = *source_ml_source1_temporary_result1366;
                                                                    source_ml_source1_temporary_result1366.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (((*operation)
                                            == (self::ml_string::Text::from_units(vec![
                                                116, 101, 115, 116, 32, 83, 116, 114, 105, 110,
                                                103, 32, 111, 119, 110, 32, 107, 101, 121,
                                            ])))
                                            || ((*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    114, 101, 97, 100, 32, 83, 116, 114, 105, 110,
                                                    103, 32, 111, 119, 110, 32, 107, 101, 121,
                                                ]))))
                                        {
                                            if ((match second.clone() {
                                                self::JavaScriptSemanticValue::Text(_) => false,
                                                _ => true,
                                            } && match second.clone() {
                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                    false
                                                }
                                                _ => true,
                                            }) && match second.clone() {
                                                self::JavaScriptSemanticValue::Symbol(_) => false,
                                                _ => true,
                                            }) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 45, 112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 97, 100, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115]))
                                            } else {
                                                {
                                                    let index = self::read_java_script_array_index(
                                                        second.clone(),
                                                    );
                                                    {
                                                        let is_length =
                                                            self::test_java_script_array_length_key(
                                                                second.clone(),
                                                            );
                                                        {
                                                            let present = (is_length
                                                                || ((index >= (0f64))
                                                                    && (index < length)));
                                                            if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121]))) {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(present)))
                                                            } else {
                                                                if present {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(if is_length {
                                                                        self::JavaScriptSemanticValue::Number(length)
                                                                    } else {
                                                                        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_java_script_semantic_text_unit(receiver.clone(), index)])))
                                                                    }))
                                                                } else {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 45, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 101, 115, 101, 110, 116, 32, 111, 119, 110, 32, 107, 101, 121]))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if ((*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    114, 101, 97, 100, 32, 83, 116, 114, 105, 110,
                                                    103, 32, 99, 111, 100, 101, 32, 117, 110, 105,
                                                    116,
                                                ])))
                                            {
                                                match second.clone() {
                                                    self::JavaScriptSemanticValue::Number(value_2) => {
                                                        {
                                                            let position = self::read_utf16_text_integer(value_2);
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if ((position < (0f64)) || (position >= length)) {
                                                                f64::NAN
                                                            } else {
                                                                self::read_java_script_semantic_text_unit(receiver.clone(), position)
                                                            })))
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                    }
                                                }
                                            } else {
                                                if (((*operation)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        115, 117, 98, 115, 116, 114, 105, 110, 103,
                                                        32, 83, 116, 114, 105, 110, 103, 32, 117,
                                                        110, 105, 116, 115,
                                                    ])))
                                                    || ((*operation)
                                                        == (self::ml_string::Text::from_units(
                                                            vec![
                                                                115, 117, 98, 115, 116, 114, 32,
                                                                83, 116, 114, 105, 110, 103, 32,
                                                                117, 110, 105, 116, 115,
                                                            ],
                                                        ))))
                                                {
                                                    match second.clone() {
                                                        self::JavaScriptSemanticValue::Number(second_value) => {
                                                            match third.clone() {
                                                                self::JavaScriptSemanticValue::Number(value_3) => {
                                                                    {
                                                                        let first = self::read_utf16_text_integer(second_value);
                                                                        {
                                                                            let last = self::read_utf16_text_integer(value_3);
                                                                            {
                                                                                let start = if ((*operation) == (self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 32, 83, 116, 114, 105, 110, 103, 32, 117, 110, 105, 116, 115]))) {
                                                                                    if (first < (0f64)) {
                                                                                        self::ml_math::max((length + first), 0f64)
                                                                                    } else {
                                                                                        self::ml_math::min(first, length)
                                                                                    }
                                                                                } else {
                                                                                    self::ml_math::min(self::ml_math::max(first, 0f64), length)
                                                                                };
                                                                                {
                                                                                    let end = if ((*operation) == (self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 32, 83, 116, 114, 105, 110, 103, 32, 117, 110, 105, 116, 115]))) {
                                                                                        (start + self::ml_math::min(self::ml_math::max(last, 0f64), (length - start)))
                                                                                    } else {
                                                                                        self::ml_math::min(self::ml_math::max(last, 0f64), length)
                                                                                    };
                                                                                    {
                                                                                        let low = self::ml_math::min(start, end);
                                                                                        {
                                                                                            let high = self::ml_math::max(start, end);
                                                                                            if (((high - low)) > (1048576f64)) {
                                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 108, 105, 99, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 111, 117, 116, 112, 117, 116, 32, 98, 117, 100, 103, 101, 116]))
                                                                                            } else {
                                                                                                {
                                                                                                    let units_3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                                                    {
                                                                                                        let source_ml_source2_s32603 = self::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1372(receiver.clone(), high, units_3.clone(), low);
                                                                                                        match source_ml_source2_s32603.clone() {
                                                                                                            self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1372Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1372Done => {
                                                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units_3.clone())))
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
                                                                _ => {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 108, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 98, 111, 117, 110, 100, 115]))
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 108, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 98, 111, 117, 110, 100, 115]))
                                                        }
                                                    }
                                                } else {
                                                    if (match second.clone() {
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    } && match second.clone() {
                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 115, 101, 97, 114, 99, 104, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
                                                    } else {
                                                        match third.clone() {
                                                            self::JavaScriptSemanticValue::Number(value_4) => {
                                                                {
                                                                    let needle_length = self::read_java_script_semantic_text_length(second.clone());
                                                                    {
                                                                        let position_2 = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(value_4), 0f64), length);
                                                                        {
                                                                            let start_2 = if ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 115, 117, 102, 102, 105, 120, 32, 97, 116]))) {
                                                                                (position_2 - needle_length)
                                                                            } else {
                                                                                position_2
                                                                            };
                                                                            if (((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 97, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 115, 117, 102, 102, 105, 120, 32, 97, 116])))) {
                                                                                if (needle_length > (1048576f64)) {
                                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 98, 111, 117, 110, 100, 97, 114, 121, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 119, 111, 114, 107, 32, 98, 117, 100, 103, 101, 116]))
                                                                                } else {
                                                                                    {
                                                                                        let matches = ((start_2 >= (0f64)) && (((start_2 + needle_length)) <= length));
                                                                                        {
                                                                                            let at_2 = 0f64;
                                                                                            {
                                                                                                let matches_2 = self::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1374(receiver.clone(), second.clone(), needle_length, start_2, matches, at_2);
                                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_2)))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if ((*operation) != (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110, 32, 97, 116]))) {
                                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 104, 101, 108, 112, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                                                                } else {
                                                                                    if (((self::ml_math::max(0f64, (((length - position_2) - needle_length) + 1f64)) * needle_length)) > (67108864f64)) {
                                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 119, 111, 114, 107, 32, 98, 117, 100, 103, 101, 116]))
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_s32607 = self::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop1375(receiver.clone(), second.clone(), length, needle_length, position_2);
                                                                                            match source_ml_source2_s32607.clone() {
                                                                                                self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Done => {
                                                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(false)))
                                                                                                }
                                                                                                self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop1375Return(source_ml_source1_temporary_result1375) => {
                                                                                                    let source_ml_source1_temporary_result1375 = *source_ml_source1_temporary_result1375;
                                                                                                    source_ml_source1_temporary_result1375.clone()
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
                                                            _ => {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 115, 101, 97, 114, 99, 104, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 111, 115, 105, 116, 105, 111, 110]))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_case_profile_error(
    profile: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*profile) == (self::ml_string::Text::from_units(vec![49, 54, 46, 48]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        (self::ml_string::Text::from_units(vec![
            83, 116, 114, 105, 110, 103, 32, 100, 101, 102, 97, 117, 108, 116, 32, 99, 97, 115,
            105, 110, 103, 32, 104, 97, 115, 32, 110, 111, 32, 97, 117, 116, 104, 111, 114, 101,
            100, 32, 116, 97, 98, 108, 101, 32, 102, 111, 114, 32, 97, 99, 116, 105, 118, 101, 32,
            85, 110, 105, 99, 111, 100, 101, 32, 112, 114, 111, 102, 105, 108, 101, 32,
        ]))
        .concat(profile)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_big_integer_bitwise_operation(
    operation: f64,
    left: self::ml::Big,
    right: self::ml::Big,
) -> self::ml::Big {
    if (operation == (6f64)) {
        (self::ml::Big::from_i128(-1)).sub(&left)
    } else {
        if ((operation == (3f64)) || (operation == (4f64))) {
            if (left == (self::ml::Big::from_i128(0))) {
                self::ml::Big::from_i128(0)
            } else {
                {
                    let count = right.clone();
                    {
                        let forward = (operation == (3f64));
                        {
                            let source_ml_source2_s32609 = if (count
                                < (self::ml::Big::from_i128(0)))
                            {
                                {
                                    let count_2 = count.neg();
                                    {
                                        let forward_2 = !forward;
                                        self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin1379::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin1379Next(count_2.clone(), forward_2)
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin1379::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin1379Next(count.clone(), forward)
                            };
                            match source_ml_source2_s32609.clone() {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin1379::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin1379Next(count_3, forward_3) => {
                                    {
                                        let value = left.clone();
                                        {
                                            let value_4 = if forward_3 {
                                                {
                                                    let factor = self::ml::Big::from_i128(2);
                                                    {
                                                        let value_2 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop1380(count_3.clone(), value.clone(), factor.clone());
                                                        value_2.clone()
                                                    }
                                                }
                                            } else {
                                                {
                                                    let value_3 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop1381(count_3.clone(), value.clone());
                                                    value_3.clone()
                                                }
                                            };
                                            value_4.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            if (((operation != (0f64)) && (operation != (1f64))) && (operation != (2f64))) {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        97, 110, 32, 117, 110, 107, 110, 111, 119, 110, 32, 66, 105, 103, 73, 110,
                        116, 32, 98, 105, 116, 119, 105, 115, 101, 32, 111, 112, 101, 114, 97, 116,
                        105, 111, 110,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
            } else {
                {
                    let negative_left = (left < (self::ml::Big::from_i128(0)));
                    {
                        let negative_right = (right < (self::ml::Big::from_i128(0)));
                        {
                            let first = if negative_left {
                                (self::ml::Big::from_i128(-1)).sub(&left)
                            } else {
                                left.clone()
                            };
                            {
                                let second = if negative_right {
                                    (self::ml::Big::from_i128(-1)).sub(&right)
                                } else {
                                    right.clone()
                                };
                                {
                                    let weight = self::ml::Big::from_i128(1);
                                    {
                                        let result = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s32610 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop1382(operation, negative_left, negative_right, first.clone(), second.clone(), weight.clone(), result.clone());
                                            match source_ml_source2_s32610.clone() {
                                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop1382Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop1382Done(weight_2, result_2) => {
                                                    {
                                                        let negative = if (operation == (0f64)) {
                                                            (negative_left && negative_right)
                                                        } else {
                                                            if (operation == (1f64)) {
                                                                (negative_left || negative_right)
                                                            } else {
                                                                (negative_left != negative_right)
                                                            }
                                                        };
                                                        if negative {
                                                            result_2.sub(&weight_2)
                                                        } else {
                                                            result_2.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function7(value: self::ml::Big) -> f64 {
    self::dependency_function1(value.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_integer_bit_code(operation: &self::ml_string::Text) -> f64 {
    if (((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105,
            116, 115,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105,
                116, 115,
            ]))))
    {
        0f64
    } else {
        if (((*operation)
            == (self::ml_string::Text::from_units(vec![
                100, 105, 115, 106, 111, 105, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98,
                105, 116, 115,
            ])))
            || ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    100, 105, 115, 106, 111, 105, 110, 32, 110, 117, 109, 98, 101, 114, 32, 98,
                    105, 116, 115,
                ]))))
        {
            1f64
        } else {
            if (((*operation)
                == (self::ml_string::Text::from_units(vec![
                    101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 105, 110,
                    116, 101, 103, 101, 114, 32, 98, 105, 116, 115,
                ])))
                || ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        101, 120, 99, 108, 117, 100, 101, 32, 101, 113, 117, 97, 108, 32, 110, 117,
                        109, 98, 101, 114, 32, 98, 105, 116, 115,
                    ]))))
            {
                2f64
            } else {
                if (((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98,
                        105, 116, 115, 32, 108, 101, 102, 116,
                    ])))
                    || ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98, 105,
                            116, 115, 32, 108, 101, 102, 116,
                        ]))))
                {
                    3f64
                } else {
                    if (((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98,
                            105, 116, 115, 32, 114, 105, 103, 104, 116,
                        ])))
                        || ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98,
                                105, 116, 115, 32, 114, 105, 103, 104, 116,
                            ]))))
                    {
                        4f64
                    } else {
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                115, 104, 105, 102, 116, 32, 110, 117, 109, 98, 101, 114, 32, 98,
                                105, 116, 115, 32, 114, 105, 103, 104, 116, 32, 117, 110, 115, 105,
                                103, 110, 101, 100,
                            ])))
                        {
                            5f64
                        } else {
                            if (((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 105, 110,
                                    116, 101, 103, 101, 114, 32, 98, 105, 116, 115,
                                ])))
                                || ((*operation)
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 32, 110,
                                        117, 109, 98, 101, 114, 32, 98, 105, 116, 115,
                                    ]))))
                            {
                                6f64
                            } else {
                                (-1f64)
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
pub fn execute_java_script_call_number_bits(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let left = ({
            let source_ml_source2_array_read_index32616 = 0f64;
            {
                let ml_reference_array2038 = values.clone();
                let ml_reference_position2038 = source_ml_source2_array_read_index32616;
                let ml_reference_projection2038 = self::ml_array::optional_at(
                    &*ml_reference_array2038.borrow(),
                    self::ml_array::number_index(ml_reference_position2038),
                );
                ml_reference_projection2038
            }
        })
        .clone()
        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let right = ({
                let source_ml_source2_array_read_index32618 = 1f64;
                {
                    let ml_reference_array2039 = values.clone();
                    let ml_reference_position2039 = source_ml_source2_array_read_index32618;
                    let ml_reference_projection2039 = self::ml_array::optional_at(
                        &*ml_reference_array2039.borrow(),
                        self::ml_array::number_index(ml_reference_position2039),
                    );
                    ml_reference_projection2039
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Number(0f64));
            match left.clone() {
                self::JavaScriptSemanticValue::Number(left_value) => match right.clone() {
                    self::JavaScriptSemanticValue::Number(value) => {
                        let code = self::read_java_script_call_integer_bit_code(operation);
                        if (code < (0f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112,
                                    101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117,
                                    105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116,
                                    117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101,
                                    100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
                                ]),
                            )
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                self::JavaScriptSemanticValue::Number(
                                    self::apply_java_script_number_bitwise_operation(
                                        code, left_value, value,
                                    ),
                                ),
                            ))
                        }
                    }
                    _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                        self::ml_string::Text::from_units(vec![
                            78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114,
                            97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32,
                            97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101,
                            100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110,
                            100, 115,
                        ]),
                    ),
                },
                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        78, 117, 109, 98, 101, 114, 32, 98, 105, 116, 32, 111, 112, 101, 114, 97,
                        116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99,
                        116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78,
                        117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115,
                    ]),
                ),
            }
        }
    }
}
