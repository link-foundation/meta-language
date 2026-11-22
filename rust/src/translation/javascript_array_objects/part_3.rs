// Generated from complete js/src/translation/javascript-array-objects.js. Do not edit by hand.
use super::JavaScriptSemanticValue;
use super::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result;
use super::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_value_error;
use super::read_java_script_source_text_profile_error;
use super::source_ml_temporary_construct_java_script_semantic_text_units_loop5;
use super::source_ml_temporary_read_java_script_semantic_text_string_loop4;
use super::source_ml_temporary_test_java_script_semantic_text_equal_loop2;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_unit(
    value: self::JavaScriptSemanticValue,
    index: f64,
) -> f64 {
    if (!(self::ml_math::is_safe_integer(index)) || (index < 0f64))
        || (index >= (self::read_java_script_semantic_text_length(value.clone())))
    {
        -1f64
    } else if match value.clone() {
        self::JavaScriptSemanticValue::Undefined => false,
        self::JavaScriptSemanticValue::Null => false,
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f92) => false,
        self::JavaScriptSemanticValue::Number(source_ml_source1_f93) => false,
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f94) => false,
        self::JavaScriptSemanticValue::Text(source_ml_source1_f95) => false,
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f96, source_ml_source1_f97) => {
            true
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f98) => false,
        self::JavaScriptSemanticValue::Object(source_ml_source1_f99, source_ml_source1_f100) => {
            false
        }
    } {
        {
            let source_ml_source1_nullish_field_receiver102 = (match value.clone() {
                self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::SourceText(
                    _,
                    source_ml_source1_data_property_value101,
                ) => Some(source_ml_source1_data_property_value101.clone()),
                self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                self::JavaScriptSemanticValue::Object(_, _) => None::<self::ml_string::Text>,
            },);
            if (match (source_ml_source1_nullish_field_receiver102.clone()).0 {
                Some(ml_present) => {
                    let _ = ml_present;
                    self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                }
                None => self::ml_string::Text::from_units(vec![
                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                ]),
            }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
            {
                {
                    let source_ml_source1_nullish_field_present103 =
                        ((source_ml_source1_nullish_field_receiver102.clone()).0)
                            .clone()
                            .expect("a proven present value");
                    {
                        let source_ml_source1_string_unit_operand104 =
                            source_ml_source1_nullish_field_present103;
                        {
                            let source_ml_source1_string_unit_operand105 = index;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index_2 = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                            f64::NAN
                                        } else {
                                            ({
                                            let source_ml_source1_string_unit_operand111 = {
                                                let source_ml_source1_string_range_operand106 = text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand107 = index_2;
                                                    {
                                                        let source_ml_source1_string_range_operand108 = index_2 + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position109 = source_ml_source1_string_range_operand107;
                                                            {
                                                                let source_ml_source1_string_range_position110 = source_ml_source1_string_range_operand108;
                                                                source_ml_source1_string_range_operand106.slice(source_ml_source1_string_range_position109, source_ml_source1_string_range_position110)
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            {
                                                let source_ml_source1_string_unit_operand112 = 0f64;
                                                source_ml_source1_string_unit_operand111.code_point_at(source_ml_source1_string_unit_operand112)
                                            }
                                        }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source1_string_unit_operand104,
                                source_ml_source1_string_unit_operand105,
                            )
                        }
                    }
                }
            } else {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112,
                        101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102,
                        105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99,
                        104, 97, 114, 67, 111, 100, 101, 65, 116, 39, 41,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
            }
        }
    } else if match value.clone() {
        self::JavaScriptSemanticValue::Undefined => false,
        self::JavaScriptSemanticValue::Null => false,
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f113) => false,
        self::JavaScriptSemanticValue::Number(source_ml_source1_f114) => false,
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f115) => false,
        self::JavaScriptSemanticValue::Text(source_ml_source1_f116) => true,
        self::JavaScriptSemanticValue::SourceText(
            source_ml_source1_f117,
            source_ml_source1_f118,
        ) => false,
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f119) => false,
        self::JavaScriptSemanticValue::Object(source_ml_source1_f120, source_ml_source1_f121) => {
            false
        }
    } {
        ({
            let source_ml_source1_nullish_field_receiver123 = (
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                    }
                    self::JavaScriptSemanticValue::Null => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                    }
                    self::JavaScriptSemanticValue::Text(
                        source_ml_source1_data_property_value122,
                    ) => Some(source_ml_source1_data_property_value122.clone()),
                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                    }
                },
                index,
            );
            {
                let source_ml_source1_computed_property_key124 =
                    (source_ml_source1_nullish_field_receiver123.clone()).1;
                if match (source_ml_source1_nullish_field_receiver123.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    {
                        let source_ml_source1_nullish_field_present125 =
                            ((source_ml_source1_nullish_field_receiver123.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        {
                            let source_ml_source1_array_read_value126 =
                                source_ml_source1_nullish_field_present125;
                            {
                                let source_ml_source1_array_read_index127 =
                                    source_ml_source1_computed_property_key124;
                                {
                                    let ml_reference_array3 = source_ml_source1_array_read_value126;
                                    let ml_reference_position3 =
                                        source_ml_source1_array_read_index127;
                                    self::ml_array::optional_at(
                                        &*ml_reference_array3.borrow(),
                                        self::ml_array::number_index(ml_reference_position3),
                                    )
                                }
                            }
                        }
                    }
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (((self::ml_string::Text::from_units(vec![
                            67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111,
                            112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100,
                            101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103,
                            32, 39,
                        ]))
                        .concat(&self::ml_string::Text::from_text(
                            &self::ml_number::js_number(
                                (source_ml_source1_nullish_field_receiver123.clone()).1,
                            ),
                        )))
                        .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                        .0,
                        message_present: true,
                    })
                    .raise()
                }
            }
        })
        .clone()
        .unwrap_or_else(|| -1f64)
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_text_equal(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    {
        let length = self::read_java_script_semantic_text_length(left.clone());
        if (length < 0f64)
            || (length != (self::read_java_script_semantic_text_length(right.clone())))
        {
            false
        } else if match left.clone() {
            self::JavaScriptSemanticValue::Undefined => false,
            self::JavaScriptSemanticValue::Null => false,
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f128) => false,
            self::JavaScriptSemanticValue::Number(source_ml_source1_f129) => false,
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f130) => false,
            self::JavaScriptSemanticValue::Text(source_ml_source1_f131) => false,
            self::JavaScriptSemanticValue::SourceText(
                source_ml_source1_f132,
                source_ml_source1_f133,
            ) => true,
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f134) => false,
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f135,
                source_ml_source1_f136,
            ) => false,
        } && match right.clone() {
            self::JavaScriptSemanticValue::Undefined => false,
            self::JavaScriptSemanticValue::Null => false,
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f137) => false,
            self::JavaScriptSemanticValue::Number(source_ml_source1_f138) => false,
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f139) => false,
            self::JavaScriptSemanticValue::Text(source_ml_source1_f140) => false,
            self::JavaScriptSemanticValue::SourceText(
                source_ml_source1_f141,
                source_ml_source1_f142,
            ) => true,
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f143) => false,
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f144,
                source_ml_source1_f145,
            ) => false,
        } {
            {
                let source_ml_source1_comparison_operand148 = match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::SourceText(
                        _,
                        source_ml_source1_data_property_value146,
                    ) => Some(source_ml_source1_data_property_value146.clone()),
                    self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Object(_, _) => None::<self::ml_string::Text>,
                };
                {
                    let source_ml_source1_comparison_operand149 = match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::SourceText(
                            _,
                            source_ml_source1_data_property_value147,
                        ) => Some(source_ml_source1_data_property_value147.clone()),
                        self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    match (
                        source_ml_source1_comparison_operand148,
                        source_ml_source1_comparison_operand149,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            }
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source1_s150 =
                        self::source_ml_temporary_test_java_script_semantic_text_equal_loop2(
                            left.clone(),
                            right.clone(),
                            length,
                            at,
                        );
                    match source_ml_source1_s150 {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done => {
                                true
                            }
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(source_ml_temporary_result2) => {
                                source_ml_temporary_result2
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_string(
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97,
                110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101,
                32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101,
            ]))
            .0,
            message_present: true,
        })
        .raise(),
        self::JavaScriptSemanticValue::Null => (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97,
                110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101,
                32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101,
            ]))
            .0,
            message_present: true,
        })
        .raise(),
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f151) => {
            let source_ml_source1_f175 = source_ml_source1_f151;
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97,
                    110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101,
                    116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f152) => {
            let source_ml_source1_f184 = source_ml_source1_f152;
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97,
                    110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101,
                    116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f153) => {
            let source_ml_source1_f193 = source_ml_source1_f153.clone();
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97,
                    110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101,
                    116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f154) => {
            let units = source_ml_source1_f154.clone();
            {
                let text = self::ml_string::Text::from_units(vec![]);
                {
                    let source_ml_temporary_values3 = units;
                    {
                        let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                        {
                            self::source_ml_temporary_read_java_script_semantic_text_string_loop4(
                                text,
                                source_ml_temporary_values3,
                                source_ml_temporary_index3,
                            )
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f155, text_3) => text_3.clone(),
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f156) => {
            let source_ml_source1_f212 = source_ml_source1_f156.clone();
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97,
                    110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101,
                    116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f157, source_ml_source1_f158) => {
            let source_ml_source1_f221 = source_ml_source1_f157.clone();
            {
                let source_ml_source1_f222 = source_ml_source1_f158;
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114,
                        97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101,
                        115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109,
                        112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117,
                        101,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_semantic_text_units(
    value: self::JavaScriptSemanticValue,
    limit: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let length = self::read_java_script_semantic_text_length(value.clone());
        if (length < 0f64) || (length > limit) {
            (self::ml_source_exception::SourceException {
                class_name: String::from("RangeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 99, 111, 110,
                    118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97,
                    116, 105, 111, 110, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s231 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units.clone(), at);
                            match source_ml_source1_s231 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    let units_2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_2 = 0f64;
                        {
                            let source_ml_source1_s232 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_2.clone(), at_2);
                            match source_ml_source1_s232 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_2.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f223) => {
                    let units_3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_3 = 0f64;
                        {
                            let source_ml_source1_s233 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_3.clone(), at_3);
                            match source_ml_source1_s233 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_3.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f224) => {
                    let units_4 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_4 = 0f64;
                        {
                            let source_ml_source1_s234 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_4.clone(), at_4);
                            match source_ml_source1_s234 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_4.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f225) => {
                    let units_5 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_5 = 0f64;
                        {
                            let source_ml_source1_s235 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_5.clone(), at_5);
                            match source_ml_source1_s235 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_5.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_6) => units_6.clone(),
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f226,
                    source_ml_source1_f227,
                ) => {
                    let units_7 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_6 = 0f64;
                        {
                            let source_ml_source1_s236 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_7.clone(), at_6);
                            match source_ml_source1_s236 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_7.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f228) => {
                    let units_8 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_7 = 0f64;
                        {
                            let source_ml_source1_s237 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_8.clone(), at_7);
                            match source_ml_source1_s237 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_8.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f229,
                    source_ml_source1_f230,
                ) => {
                    let units_9 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_8 = 0f64;
                        {
                            let source_ml_source1_s238 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_9.clone(), at_8);
                            match source_ml_source1_s238 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_9.clone()
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
pub fn construct_java_script_source_text(
    text: &self::ml_string::Text,
) -> self::JavaScriptSemanticValue {
    if (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
        == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
    {
        {
            let profile = self::ml_string::Text::from_units(vec![
                105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32,
                115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49,
            ]);
            {
                let error =
                    self::read_java_script_source_text_profile_error(&(profile), text.length());
                if error == (self::ml_string::Text::from_units(vec![])) {
                    self::JavaScriptSemanticValue::SourceText(profile.clone(), text.clone())
                } else {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("RangeError"),
                        message_units: (error).0,
                        message_present: true,
                    })
                    .raise()
                }
            }
        }
    } else {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105,
                114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109,
                112, 108, 101, 116, 101, 32, 83, 116, 114, 105, 110, 103, 32, 118, 97, 108, 117,
                101,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    {
        let ml_library_inequality_1 = (self::read_java_script_semantic_value_error(value.clone()))
            != (self::ml_string::Text::from_units(vec![]));
        if ml_library_inequality_1 {
            false
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f239) => {
                    let source_ml_source1_f256 = source_ml_source1_f239;
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f240) => {
                    let source_ml_source1_f265 = source_ml_source1_f240;
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f241) => {
                    let source_ml_source1_f274 = source_ml_source1_f241.clone();
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f242) => {
                    let source_ml_source1_f283 = source_ml_source1_f242.clone();
                    false
                }
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f243,
                    source_ml_source1_f244,
                ) => {
                    let source_ml_source1_f292 = source_ml_source1_f243.clone();
                    {
                        let source_ml_source1_f293 = source_ml_source1_f244.clone();
                        false
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f245) => {
                    let source_ml_source1_f302 = source_ml_source1_f245.clone();
                    false
                }
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f246,
                    source_ml_source1_f247,
                ) => {
                    let source_ml_source1_f311 = source_ml_source1_f246.clone();
                    { source_ml_source1_f247 }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_property_descriptor_kind(
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        )>,
    >,
) -> self::ml_string::Text {
    {
        let data = ({ descriptor.borrow().clone() }).9 || ({ descriptor.borrow().clone() }).11;
        {
            let accessor =
                ({ descriptor.borrow().clone() }).5 || ({ descriptor.borrow().clone() }).7;
            if data && accessor {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else if data {
                self::ml_string::Text::from_units(vec![100, 97, 116, 97])
            } else if accessor {
                self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
            } else {
                self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn test_java_script_property_descriptors_same_value(
    left: std::rc::Rc<
        std::cell::RefCell<(
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        )>,
    >,
    right: std::rc::Rc<
        std::cell::RefCell<(
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        )>,
    >,
) -> bool {
    !(((((((((((((({ left.borrow().clone() }).1) != (({ right.borrow().clone() }).1))
        || (({ left.borrow().clone() }).1
            && ((({ left.borrow().clone() }).0) != (({ right.borrow().clone() }).0))))
        || ((({ left.borrow().clone() }).3) != (({ right.borrow().clone() }).3)))
        || (({ left.borrow().clone() }).3
            && ((({ left.borrow().clone() }).2) != (({ right.borrow().clone() }).2))))
        || ((({ left.borrow().clone() }).11) != (({ right.borrow().clone() }).11)))
        || (({ left.borrow().clone() }).11
            && ((({ left.borrow().clone() }).10) != (({ right.borrow().clone() }).10))))
        || ((({ left.borrow().clone() }).9) != (({ right.borrow().clone() }).9)))
        || (({ left.borrow().clone() }).9
            && !(self::test_java_script_semantic_same_value(
                ({ left.borrow().clone() }).8,
                ({ right.borrow().clone() }).8,
            ))))
        || ((({ left.borrow().clone() }).5) != (({ right.borrow().clone() }).5)))
        || (({ left.borrow().clone() }).5
            && !(self::test_java_script_semantic_same_value(
                ({ left.borrow().clone() }).4,
                ({ right.borrow().clone() }).4,
            ))))
        || ((({ left.borrow().clone() }).7) != (({ right.borrow().clone() }).7)))
        || (({ left.borrow().clone() }).7
            && !(self::test_java_script_semantic_same_value(
                ({ left.borrow().clone() }).6,
                ({ right.borrow().clone() }).6,
            ))))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_property_descriptor_error(
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        )>,
    >,
) -> self::ml_string::Text {
    if (self::read_java_script_property_descriptor_kind(descriptor.clone()))
        == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        let ml_product_condition_3 = { descriptor.borrow().clone() };
        if ml_product_condition_3 {
            self::ml_string::Text::from_units(vec![
                73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108,
                117, 101,
            ])
        } else {
            let ml_product_condition_2 = { descriptor.borrow().clone() };
            if ml_product_condition_2 {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                let ml_product_condition_1 = { descriptor.borrow().clone() };
                if ml_product_condition_1 {
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}
