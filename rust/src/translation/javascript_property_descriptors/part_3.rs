// Generated from complete js/src/translation/javascript-property-descriptors.js. Do not edit by hand.
use super::JavaScriptSemanticValue;
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
                    self::JavaScriptSemanticValue::Undefined => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Null => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Boolean(_) => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Number(_) => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Integer(_) => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Text(
                        source_ml_source1_data_property_value122,
                    ) => Some(source_ml_source1_data_property_value122),
                    self::JavaScriptSemanticValue::SourceText(_, _) => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Symbol(_) => None::<Vec<f64>>,
                    self::JavaScriptSemanticValue::Object(_, _) => None::<Vec<f64>>,
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
                        self::ml_array::optional_at(
                            &source_ml_source1_nullish_field_present125,
                            self::ml_array::number_index(
                                source_ml_source1_computed_property_key124,
                            ),
                        )
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f126) => false,
            self::JavaScriptSemanticValue::Number(source_ml_source1_f127) => false,
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f128) => false,
            self::JavaScriptSemanticValue::Text(source_ml_source1_f129) => false,
            self::JavaScriptSemanticValue::SourceText(
                source_ml_source1_f130,
                source_ml_source1_f131,
            ) => true,
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f132) => false,
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f133,
                source_ml_source1_f134,
            ) => false,
        } && match right.clone() {
            self::JavaScriptSemanticValue::Undefined => false,
            self::JavaScriptSemanticValue::Null => false,
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f135) => false,
            self::JavaScriptSemanticValue::Number(source_ml_source1_f136) => false,
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f137) => false,
            self::JavaScriptSemanticValue::Text(source_ml_source1_f138) => false,
            self::JavaScriptSemanticValue::SourceText(
                source_ml_source1_f139,
                source_ml_source1_f140,
            ) => true,
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f141) => false,
            self::JavaScriptSemanticValue::Object(
                source_ml_source1_f142,
                source_ml_source1_f143,
            ) => false,
        } {
            {
                let source_ml_source1_comparison_operand146 = match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::SourceText(
                        _,
                        source_ml_source1_data_property_value144,
                    ) => Some(source_ml_source1_data_property_value144.clone()),
                    self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Object(_, _) => None::<self::ml_string::Text>,
                };
                {
                    let source_ml_source1_comparison_operand147 = match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::SourceText(
                            _,
                            source_ml_source1_data_property_value145,
                        ) => Some(source_ml_source1_data_property_value145.clone()),
                        self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    match (
                        source_ml_source1_comparison_operand146,
                        source_ml_source1_comparison_operand147,
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
                    let source_ml_source1_s148 =
                        self::source_ml_temporary_test_java_script_semantic_text_equal_loop2(
                            left.clone(),
                            right.clone(),
                            length,
                            at,
                        );
                    match source_ml_source1_s148 {
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f149) => {
            let source_ml_source1_f173 = source_ml_source1_f149;
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
        self::JavaScriptSemanticValue::Number(source_ml_source1_f150) => {
            let source_ml_source1_f182 = source_ml_source1_f150;
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
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f151) => {
            let source_ml_source1_f191 = source_ml_source1_f151.clone();
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
        self::JavaScriptSemanticValue::Text(source_ml_source1_f152) => {
            let units = source_ml_source1_f152;
            {
                let text = self::ml_string::Text::from_units(vec![]);
                {
                    let source_ml_temporary_values3 = units;
                    {
                        let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                        {
                            self::source_ml_temporary_read_java_script_semantic_text_string_loop4(
                                text,
                                &(source_ml_temporary_values3),
                                source_ml_temporary_index3,
                            )
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f153, text_3) => text_3.clone(),
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f154) => {
            let source_ml_source1_f210 = source_ml_source1_f154.clone();
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
        self::JavaScriptSemanticValue::Object(source_ml_source1_f155, source_ml_source1_f156) => {
            let source_ml_source1_f219 = source_ml_source1_f155.clone();
            {
                let source_ml_source1_f220 = source_ml_source1_f156;
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
) -> Vec<f64> {
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
                    let units = Vec::<f64>::new();
                    {
                        let at = 0f64;
                        {
                            self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units, at)
                        }
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    let units_3 = Vec::<f64>::new();
                    {
                        let at_2 = 0f64;
                        {
                            self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_3, at_2)
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f221) => {
                    let units_5 = Vec::<f64>::new();
                    {
                        let at_3 = 0f64;
                        {
                            self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_5, at_3)
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f222) => {
                    let units_7 = Vec::<f64>::new();
                    {
                        let at_4 = 0f64;
                        {
                            self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_7, at_4)
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f223) => {
                    let units_9 = Vec::<f64>::new();
                    {
                        let at_5 = 0f64;
                        {
                            self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_9, at_5)
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_11) => units_11,
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f224,
                    source_ml_source1_f225,
                ) => {
                    let units_12 = Vec::<f64>::new();
                    {
                        let at_6 = 0f64;
                        {
                            self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_12, at_6)
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f226) => {
                    let units_14 = Vec::<f64>::new();
                    {
                        let at_7 = 0f64;
                        {
                            self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_14, at_7)
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f227,
                    source_ml_source1_f228,
                ) => {
                    let units_16 = Vec::<f64>::new();
                    {
                        let at_8 = 0f64;
                        {
                            self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_16, at_8)
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f229) => {
                    let source_ml_source1_f246 = source_ml_source1_f229;
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f230) => {
                    let source_ml_source1_f255 = source_ml_source1_f230;
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f231) => {
                    let source_ml_source1_f264 = source_ml_source1_f231.clone();
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f232) => {
                    let source_ml_source1_f273 = source_ml_source1_f232;
                    false
                }
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f233,
                    source_ml_source1_f234,
                ) => {
                    let source_ml_source1_f282 = source_ml_source1_f233.clone();
                    {
                        let source_ml_source1_f283 = source_ml_source1_f234.clone();
                        false
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f235) => {
                    let source_ml_source1_f292 = source_ml_source1_f235.clone();
                    false
                }
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f236,
                    source_ml_source1_f237,
                ) => {
                    let source_ml_source1_f301 = source_ml_source1_f236.clone();
                    { source_ml_source1_f237 }
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
    descriptor: (
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
    ),
) -> self::ml_string::Text {
    {
        let data = (descriptor.clone()).9 || (descriptor.clone()).11;
        {
            let accessor = (descriptor.clone()).5 || (descriptor.clone()).7;
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
    left: (
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
    ),
    right: (
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
    ),
) -> bool {
    !((((((((((((((left.clone()).1) != ((right.clone()).1))
        || ((left.clone()).1 && (((left.clone()).0) != ((right.clone()).0))))
        || (((left.clone()).3) != ((right.clone()).3)))
        || ((left.clone()).3 && (((left.clone()).2) != ((right.clone()).2))))
        || (((left.clone()).11) != ((right.clone()).11)))
        || ((left.clone()).11 && (((left.clone()).10) != ((right.clone()).10))))
        || (((left.clone()).9) != ((right.clone()).9)))
        || ((left.clone()).9
            && !(self::test_java_script_semantic_same_value(
                (left.clone()).8,
                (right.clone()).8,
            ))))
        || (((left.clone()).5) != ((right.clone()).5)))
        || ((left.clone()).5
            && !(self::test_java_script_semantic_same_value(
                (left.clone()).4,
                (right.clone()).4,
            ))))
        || (((left.clone()).7) != ((right.clone()).7)))
        || ((left.clone()).7
            && !(self::test_java_script_semantic_same_value((left.clone()).6, (right.clone()).6))))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_property_descriptor_error(
    descriptor: (
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
    ),
) -> self::ml_string::Text {
    if (self::read_java_script_property_descriptor_kind(descriptor.clone()))
        == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else if (descriptor.clone()).9
        && ((self::read_java_script_semantic_value_error((descriptor.clone()).8))
            != (self::ml_string::Text::from_units(vec![])))
    {
        self::ml_string::Text::from_units(vec![
            73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108, 117,
            101,
        ])
    } else if (descriptor.clone()).5
        && (((self::read_java_script_semantic_value_error((descriptor.clone()).4))
            != (self::ml_string::Text::from_units(vec![])))
            || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else if (descriptor.clone()).7
        && (((self::read_java_script_semantic_value_error((descriptor.clone()).6))
            != (self::ml_string::Text::from_units(vec![])))
            || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn test_complete_java_script_property_descriptor(
    descriptor: (
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
    ),
) -> bool {
    if (!((descriptor.clone()).1) || !((descriptor.clone()).3))
        || ((self::read_java_script_property_descriptor_error(descriptor.clone()))
            != (self::ml_string::Text::from_units(vec![])))
    {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])) {
                (descriptor.clone()).9 && (descriptor.clone()).11
            } else {
                ((kind
                    == (self::ml_string::Text::from_units(vec![
                        97, 99, 99, 101, 115, 115, 111, 114,
                    ])))
                    && (descriptor.clone()).5)
                    && (descriptor.clone()).7
            }
        }
    }
}
