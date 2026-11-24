// Generated from complete js/src/translation/javascript-collection-rules.js. Do not edit by hand.
use super::JavaScriptSemanticValue;
use super::MlTemporaryCompareJavaScriptCollectionKeysJoin8;
use super::MlTemporaryCompareJavaScriptCollectionKeysLoop9Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_collection_key_rank;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_text_unit;
use super::source_ml_temporary_compare_java_script_collection_keys_loop9;
use super::source_ml_temporary_order_java_script_collection_keys_loop10;
use super::test_java_script_collection_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_java_script_collection_keys(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> f64 {
    {
        let a = self::read_java_script_collection_key_rank(left.clone());
        {
            let b = self::read_java_script_collection_key_rank(right.clone());
            {
                let ml_library_inequality_1 = a != b;
                if ml_library_inequality_1 {
                    if a < b { -1f64 } else { 1f64 }
                } else {
                    {
                        let source_ml_source1_s412 = match left.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                        }
                        self::JavaScriptSemanticValue::Boolean(left_value) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(value) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if left_value == value {
                                        0f64
                                    } else if left_value {
                                            1f64
                                        } else {
                                            -1f64
                                        })
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f416) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f417) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f418) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f419, source_ml_source1_f420) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f421) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f422, source_ml_source1_f423) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Number(left_value_2) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f424) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(value_2) => {
                                    if self::test_java_script_collection_key(left.clone(), right.clone()) {
                                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                    } else if left_value_2.is_nan() {
                                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(1f64)
                                        } else if value_2.is_nan() {
                                                self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(-1f64)
                                            } else {
                                                self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if left_value_2 < value_2 {
                                                    -1f64
                                                } else {
                                                    1f64
                                                })
                                            }
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f425) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f426) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f427, source_ml_source1_f428) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f429) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f430, source_ml_source1_f431) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(left_value_3) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f432) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f433) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(value_3) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if left_value_3 == value_3 {
                                        0f64
                                    } else if left_value_3 < value_3 {
                                            -1f64
                                        } else {
                                            1f64
                                        })
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f434) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f435, source_ml_source1_f436) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f437) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f438, source_ml_source1_f439) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f413) => {
                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f414, left_text) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f440) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f441) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f442) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f443) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f444, text) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if left_text == text {
                                        0f64
                                    } else if <self::ml_string::Text as std::cmp::PartialOrd>::lt(&left_text, &text) {
                                            -1f64
                                        } else {
                                            1f64
                                        })
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f445) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f446, source_ml_source1_f447) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(left_identity) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f448) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f449) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f450) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f451) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f452, source_ml_source1_f453) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(identity) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if left_identity == identity {
                                        0f64
                                    } else if left_identity < identity {
                                            -1f64
                                        } else {
                                            1f64
                                        })
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f454, source_ml_source1_f455) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source1_f415) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f456) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f457) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f458) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f459) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f460, source_ml_source1_f461) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f462) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f463) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if left_identity_2 == identity_2 {
                                        0f64
                                    } else if left_identity_2 < identity_2 {
                                            -1f64
                                        } else {
                                            1f64
                                        })
                                }
                            }
                        }
                    };
                        match source_ml_source1_s412 {
                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next => {
                            if a == 5f64 {
                                {
                                    let first = self::read_java_script_semantic_text_length(left.clone());
                                    {
                                        let second = self::read_java_script_semantic_text_length(right.clone());
                                        {
                                            let length = self::ml_math::min(first, second);
                                            {
                                                let index = 0f64;
                                                {
                                                    let source_ml_source1_s464 = self::source_ml_temporary_compare_java_script_collection_keys_loop9(left.clone(), right.clone(), length, index);
                                                    match source_ml_source1_s464 {
                                                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop9Result::MlTemporaryCompareJavaScriptCollectionKeysLoop9Done => {
                                                            if first == second {
                                                                0f64
                                                            } else if first < second {
                                                                    -1f64
                                                                } else {
                                                                    1f64
                                                                }
                                                        }
                                                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop9Result::MlTemporaryCompareJavaScriptCollectionKeysLoop9Return(source_ml_temporary_result9) => {
                                                            source_ml_temporary_result9
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                0f64
                            }
                        }
                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(source_ml_temporary_result9_2) => {
                            source_ml_temporary_result9_2
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
pub fn order_java_script_collection_keys(
    values: Vec<self::JavaScriptSemanticValue>,
) -> Vec<self::JavaScriptSemanticValue> {
    {
        let source = values;
        {
            let width = 1f64;
            { self::source_ml_temporary_order_java_script_collection_keys_loop10(source, width) }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(
    units: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop2(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if at < length {
            {
                let a = if match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => false,
                    self::JavaScriptSemanticValue::Null => false,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f465) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f466) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f467) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f468) => true,
                    self::JavaScriptSemanticValue::SourceText(
                        source_ml_source1_f469,
                        source_ml_source1_f470,
                    ) => false,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f471) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f472,
                        source_ml_source1_f473,
                    ) => false,
                } {
                    ({
                        let source_ml_source1_nullish_field_receiver475 = (
                            match left.clone() {
                                self::JavaScriptSemanticValue::Undefined => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Null => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Boolean(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Number(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Integer(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Text(
                                    source_ml_source1_data_property_value474,
                                ) => Some(source_ml_source1_data_property_value474),
                                self::JavaScriptSemanticValue::SourceText(_, _) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Symbol(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Object(_, _) => None::<Vec<f64>>,
                            },
                            at,
                        );
                        {
                            let source_ml_source1_computed_property_key476 =
                                (source_ml_source1_nullish_field_receiver475.clone()).1;
                            if match (source_ml_source1_nullish_field_receiver475.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present477 =
                                        ((source_ml_source1_nullish_field_receiver475.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    self::ml_array::optional_at(
                                        &source_ml_source1_nullish_field_present477,
                                        self::ml_array::number_index(
                                            source_ml_source1_computed_property_key476,
                                        ),
                                    )
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (((self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39,
                                    ]))
                                    .concat(&self::ml_string::Text::from_text(
                                        &self::ml_number::js_number(
                                            (source_ml_source1_nullish_field_receiver475.clone()).1,
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
                } else if match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => false,
                    self::JavaScriptSemanticValue::Null => false,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f478) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f479) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f480) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f481) => false,
                    self::JavaScriptSemanticValue::SourceText(
                        source_ml_source1_f482,
                        source_ml_source1_f483,
                    ) => true,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f484) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f485,
                        source_ml_source1_f486,
                    ) => false,
                } {
                    {
                        let source_ml_source1_nullish_field_receiver488 = (match left.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                            self::JavaScriptSemanticValue::SourceText(
                                _,
                                source_ml_source1_data_property_value487,
                            ) => Some(source_ml_source1_data_property_value487.clone()),
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        },);
                        if (match (source_ml_source1_nullish_field_receiver488.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present;
                                self::ml_string::Text::from_units(vec![
                                    115, 116, 114, 105, 110, 103,
                                ])
                            }
                            None => self::ml_string::Text::from_units(vec![
                                117, 110, 100, 101, 102, 105, 110, 101, 100,
                            ]),
                        }) == (self::ml_string::Text::from_units(vec![
                            115, 116, 114, 105, 110, 103,
                        ])) {
                            {
                                let source_ml_source1_nullish_field_present489 =
                                    ((source_ml_source1_nullish_field_receiver488.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let source_ml_source1_string_unit_operand490 =
                                        source_ml_source1_nullish_field_present489;
                                    {
                                        let source_ml_source1_string_unit_operand491 = at;
                                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if (index < 0f64) || (index >= (text.length())) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                        let source_ml_source1_string_unit_operand497 = {
                                                            let source_ml_source1_string_range_operand492 = text.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand493 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand494 = index + 1f64;
                                                                    {
                                                                        let source_ml_source1_string_range_position495 = source_ml_source1_string_range_operand493;
                                                                        {
                                                                            let source_ml_source1_string_range_position496 = source_ml_source1_string_range_operand494;
                                                                            source_ml_source1_string_range_operand492.slice(source_ml_source1_string_range_position495, source_ml_source1_string_range_position496)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand498 = 0f64;
                                                            source_ml_source1_string_unit_operand497.code_point_at(source_ml_source1_string_unit_operand498)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand490, source_ml_source1_string_unit_operand491)
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 99, 104, 97, 114, 67, 111, 100,
                                    101, 65, 116, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                } else {
                    -1f64
                };
                {
                    let b = if match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => false,
                        self::JavaScriptSemanticValue::Null => false,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f499) => false,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f500) => false,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f501) => false,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f502) => true,
                        self::JavaScriptSemanticValue::SourceText(
                            source_ml_source1_f503,
                            source_ml_source1_f504,
                        ) => false,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f505) => false,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f506,
                            source_ml_source1_f507,
                        ) => false,
                    } {
                        ({
                            let source_ml_source1_nullish_field_receiver509 = (
                                match right.clone() {
                                    self::JavaScriptSemanticValue::Undefined => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Null => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Boolean(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Number(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Integer(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Text(
                                        source_ml_source1_data_property_value508,
                                    ) => Some(source_ml_source1_data_property_value508),
                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                        None::<Vec<f64>>
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Object(_, _) => None::<Vec<f64>>,
                                },
                                at,
                            );
                            {
                                let source_ml_source1_computed_property_key510 =
                                    (source_ml_source1_nullish_field_receiver509.clone()).1;
                                if match (source_ml_source1_nullish_field_receiver509.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source1_nullish_field_present511 =
                                            ((source_ml_source1_nullish_field_receiver509.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        self::ml_array::optional_at(
                                            &source_ml_source1_nullish_field_present511,
                                            self::ml_array::number_index(
                                                source_ml_source1_computed_property_key510,
                                            ),
                                        )
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (((self::ml_string::Text::from_units(
                                            vec![
                                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100,
                                                32, 112, 114, 111, 112, 101, 114, 116, 105, 101,
                                                115, 32, 111, 102, 32, 117, 110, 100, 101, 102,
                                                105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                                110, 103, 32, 39,
                                            ],
                                        ))
                                        .concat(&self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                (source_ml_source1_nullish_field_receiver509
                                                    .clone())
                                                .1,
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
                    } else if match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => false,
                        self::JavaScriptSemanticValue::Null => false,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f512) => false,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f513) => false,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f514) => false,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f515) => false,
                        self::JavaScriptSemanticValue::SourceText(
                            source_ml_source1_f516,
                            source_ml_source1_f517,
                        ) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f518) => false,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f519,
                            source_ml_source1_f520,
                        ) => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_receiver522 =
                                (match right.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Text(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::SourceText(
                                        _,
                                        source_ml_source1_data_property_value521,
                                    ) => Some(source_ml_source1_data_property_value521.clone()),
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        None::<self::ml_string::Text>
                                    }
                                },);
                            if (match (source_ml_source1_nullish_field_receiver522.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present;
                                    self::ml_string::Text::from_units(vec![
                                        115, 116, 114, 105, 110, 103,
                                    ])
                                }
                                None => self::ml_string::Text::from_units(vec![
                                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                                ]),
                            }) == (self::ml_string::Text::from_units(vec![
                                115, 116, 114, 105, 110, 103,
                            ])) {
                                {
                                    let source_ml_source1_nullish_field_present523 =
                                        ((source_ml_source1_nullish_field_receiver522.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    {
                                        let source_ml_source1_string_unit_operand524 =
                                            source_ml_source1_nullish_field_present523;
                                        {
                                            let source_ml_source1_string_unit_operand525 = at;
                                            (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                    let index = if position.is_nan() {
                                                        0f64
                                                    } else {
                                                        position.trunc()
                                                    };
                                                    if (index < 0f64) || (index >= (text.length())) {
                                                        f64::NAN
                                                    } else {
                                                        ({
                                                            let source_ml_source1_string_unit_operand531 = {
                                                                let source_ml_source1_string_range_operand526 = text.clone();
                                                                {
                                                                    let source_ml_source1_string_range_operand527 = index;
                                                                    {
                                                                        let source_ml_source1_string_range_operand528 = index + 1f64;
                                                                        {
                                                                            let source_ml_source1_string_range_position529 = source_ml_source1_string_range_operand527;
                                                                            {
                                                                                let source_ml_source1_string_range_position530 = source_ml_source1_string_range_operand528;
                                                                                source_ml_source1_string_range_operand526.slice(source_ml_source1_string_range_position529, source_ml_source1_string_range_position530)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_string_unit_operand532 = 0f64;
                                                                source_ml_source1_string_unit_operand531.code_point_at(source_ml_source1_string_unit_operand532)
                                                            }
                                                        }).clone().unwrap_or_else(|| f64::NAN)
                                                    }
                                                } }))(source_ml_source1_string_unit_operand524, source_ml_source1_string_unit_operand525)
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 104, 97, 114,
                                        67, 111, 100, 101, 65, 116, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        }
                    } else {
                        -1f64
                    };
                    {
                        let ml_library_inequality_1 = a != b;
                        if ml_library_inequality_1 {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
                        } else {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_text_string_loop4(
    mut text: self::ml_string::Text,
    source_ml_temporary_values3: &[f64],
    mut source_ml_temporary_index3: self::ml::Big,
) -> self::ml_string::Text {
    loop {
        if source_ml_temporary_index3
            < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))
        {
            {
                let unit = self::ml_array::at(
                    source_ml_temporary_values3,
                    source_ml_temporary_index3.to_index(),
                );
                {
                    let text_2 = text.concat(&{
                        let source_ml_source1_character_argument533 = unit;
                        self::ml_string::Text::from_units(
                            self::ml_utf16_text::character_units_utf16_text(&vec![
                                source_ml_source1_character_argument533,
                            ])
                            .into_iter()
                            .map(|ml_unit| ml_unit as u16)
                            .collect(),
                        )
                    });
                    {
                        let source_ml_temporary_index3_2 =
                            source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_temporary_index3) =
                                (text_2, source_ml_temporary_index3_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { text.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(
    mut value: self::JavaScriptSemanticValue,
    mut length: f64,
    mut units: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < length {
            {
                let units_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::read_java_script_semantic_text_unit(value.clone(), at)],
                        units,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (units, at) = (units_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { units };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptCollectionKeyJoin6 {
    MlTemporaryTestJavaScriptCollectionKeyJoin6Next,
    MlTemporaryTestJavaScriptCollectionKeyJoin6Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryNormalizeJavaScriptCollectionKeyJoin7 {
    MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next,
    MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Return(Box<self::JavaScriptSemanticValue>),
}
