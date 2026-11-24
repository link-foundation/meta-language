pub fn read_java_script_collection_operation_kind(operation: self::ml_string::Text) -> self::ml_string::Text {
    if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![77, 97, 112, 46, 103, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 115, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 104, 97, 115]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 100, 101, 108, 101, 116, 101]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 99, 108, 101, 97, 114]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 115, 105, 122, 101])], &operation.clone(), 0f64, false, |a, b| a == b) {
        self::ml_string::Text::from_units(vec![77, 97, 112])
    } else {
        if self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![83, 101, 116, 46, 97, 100, 100]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 104, 97, 115]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 100, 101, 108, 101, 116, 101]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 99, 108, 101, 97, 114]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 115, 105, 122, 101])], &operation.clone(), 0f64, false, |a, b| a == b) {
            self::ml_string::Text::from_units(vec![83, 101, 116])
        } else {
            self::read_java_script_weak_collection_method_owner(operation.clone())
        }
    }
}

pub fn read_java_script_collection_key_rank(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            0f64
        }
        self::JavaScriptSemanticValue::Null => {
            1f64
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f447) => {
            2f64
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f448) => {
            3f64
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f449) => {
            4f64
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f450) => {
            5f64
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f451, source_ml_source1_f452) => {
            5f64
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f453) => {
            6f64
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f454, source_ml_source1_f455) => {
            7f64
        }
    }
}

pub fn compare_java_script_collection_keys(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> f64 {
    {
        let a = self::read_java_script_collection_key_rank(left.clone());
        {
            let b = self::read_java_script_collection_key_rank(right.clone());
            if (a != b) {
                if (a < b) {
                    (-1f64)
                } else {
                    1f64
                }
            } else {
                {
                    let source_ml_source1_s507 = match left.clone() {
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
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if (left_value == value) {
                                        0f64
                                    } else {
                                        if left_value {
                                            1f64
                                        } else {
                                            (-1f64)
                                        }
                                    })
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f511) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f512) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f513) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f514, source_ml_source1_f515) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f516) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f517, source_ml_source1_f518) => {
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
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f519) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(value_2) => {
                                    if self::test_java_script_collection_key(left.clone(), right.clone()) {
                                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                    } else {
                                        if left_value_2.is_nan() {
                                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(1f64)
                                        } else {
                                            if value_2.is_nan() {
                                                self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return((-1f64))
                                            } else {
                                                self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if (left_value_2 < value_2) {
                                                    (-1f64)
                                                } else {
                                                    1f64
                                                })
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f520) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f521) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f522, source_ml_source1_f523) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f524) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f525, source_ml_source1_f526) => {
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
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f527) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f528) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(value_3) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if (left_value_3 == value_3) {
                                        0f64
                                    } else {
                                        if (left_value_3 < value_3) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f529) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f530, source_ml_source1_f531) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f532) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f533, source_ml_source1_f534) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f508) => {
                            self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f509, left_text) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f535) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f536) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f537) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f538) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f539, text) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if (left_text == text) {
                                        0f64
                                    } else {
                                        if <self::ml_string::Text as std::cmp::PartialOrd>::lt(&left_text, &text) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f540) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f541, source_ml_source1_f542) => {
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
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f543) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f544) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f545) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f546) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f547, source_ml_source1_f548) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(identity) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if (left_identity == identity) {
                                        0f64
                                    } else {
                                        if (left_identity < identity) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f549, source_ml_source1_f550) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source1_f510) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f551) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f552) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f553) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f554) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f555, source_ml_source1_f556) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f557) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(0f64)
                                }
                                self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f558) => {
                                    self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(if (left_identity_2 == identity_2) {
                                        0f64
                                    } else {
                                        if (left_identity_2 < identity_2) {
                                            (-1f64)
                                        } else {
                                            1f64
                                        }
                                    })
                                }
                            }
                        }
                    };
                    match source_ml_source1_s507.clone() {
                        self::MlTemporaryCompareJavaScriptCollectionKeysJoin8::MlTemporaryCompareJavaScriptCollectionKeysJoin8Next => {
                            if (a == (5f64)) {
                                {
                                    let first = self::read_java_script_semantic_text_length(left.clone());
                                    {
                                        let second = self::read_java_script_semantic_text_length(right.clone());
                                        {
                                            let length = self::ml_math::min(first, second);
                                            {
                                                let index = 0f64;
                                                {
                                                    let source_ml_source1_s559 = self::source_ml_temporary_compare_java_script_collection_keys_loop9(left.clone(), right.clone(), length, index);
                                                    match source_ml_source1_s559.clone() {
                                                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop9Result::MlTemporaryCompareJavaScriptCollectionKeysLoop9Done => {
                                                            if (first == second) {
                                                                0f64
                                                            } else {
                                                                if (first < second) {
                                                                    (-1f64)
                                                                } else {
                                                                    1f64
                                                                }
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

pub fn order_java_script_collection_keys(values: Vec<self::JavaScriptSemanticValue>) -> Vec<self::JavaScriptSemanticValue> {
    {
        let source = values;
        {
            let width = 1f64;
            {
                let source_2 = self::source_ml_temporary_order_java_script_collection_keys_loop10(source, width);
                source_2
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(mut units: Vec<f64>, mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
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

pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop2(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if (at < length) {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        (-1f64)
                    }
                    self::JavaScriptSemanticValue::Null => {
                        (-1f64)
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f560) => {
                        {
                            let source_ml_source1_f584 = source_ml_source1_f560;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f561) => {
                        {
                            let source_ml_source1_f593 = source_ml_source1_f561;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f562) => {
                        {
                            let source_ml_source1_f602 = source_ml_source1_f562.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f563, source_ml_source1_f564) => {
                        {
                            let source_ml_source1_f612 = source_ml_source1_f563.clone();
                            {
                                let text = source_ml_source1_f564.clone();
                                {
                                    let source_ml_source1_string_unit_operand616 = text.clone();
                                    {
                                        let source_ml_source1_string_unit_operand617 = at;
                                        (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if ((index < (0f64)) || (index >= (text_2.length()))) {
                                                f64::NAN
                                            } else {
                                                ({
                                                    let source_ml_source1_string_unit_operand623 = {
                                                        let source_ml_source1_string_range_operand618 = text_2.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand619 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand620 = (index + 1f64);
                                                                {
                                                                    let source_ml_source1_string_range_position621 = source_ml_source1_string_range_operand619;
                                                                    {
                                                                        let source_ml_source1_string_range_position622 = source_ml_source1_string_range_operand620;
                                                                        source_ml_source1_string_range_operand618.slice(source_ml_source1_string_range_position621, source_ml_source1_string_range_position622)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand624 = 0f64;
                                                        source_ml_source1_string_unit_operand623.code_point_at(source_ml_source1_string_unit_operand624)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source1_string_unit_operand616.clone(), source_ml_source1_string_unit_operand617)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f565) => {
                        {
                            let source_ml_source1_f630 = source_ml_source1_f565.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f566, source_ml_source1_f567) => {
                        {
                            let source_ml_source1_f639 = source_ml_source1_f566.clone();
                            {
                                let source_ml_source1_f640 = source_ml_source1_f567;
                                (-1f64)
                            }
                        }
                    }
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            (-1f64)
                        }
                        self::JavaScriptSemanticValue::Null => {
                            (-1f64)
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f641) => {
                            {
                                let source_ml_source1_f665 = source_ml_source1_f641;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f642) => {
                            {
                                let source_ml_source1_f674 = source_ml_source1_f642;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f643) => {
                            {
                                let source_ml_source1_f683 = source_ml_source1_f643.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            (self::ml_array::optional_at(&units_2, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f644, source_ml_source1_f645) => {
                            {
                                let source_ml_source1_f693 = source_ml_source1_f644.clone();
                                {
                                    let text_2 = source_ml_source1_f645.clone();
                                    {
                                        let source_ml_source1_string_unit_operand697 = text_2.clone();
                                        {
                                            let source_ml_source1_string_unit_operand698 = at;
                                            (std::rc::Rc::new(move |text_3: self::ml_string::Text, position: f64| -> f64 { {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if ((index < (0f64)) || (index >= (text_3.length()))) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                        let source_ml_source1_string_unit_operand704 = {
                                                            let source_ml_source1_string_range_operand699 = text_3.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand700 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand701 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source1_string_range_position702 = source_ml_source1_string_range_operand700;
                                                                        {
                                                                            let source_ml_source1_string_range_position703 = source_ml_source1_string_range_operand701;
                                                                            source_ml_source1_string_range_operand699.slice(source_ml_source1_string_range_position702, source_ml_source1_string_range_position703)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand705 = 0f64;
                                                            source_ml_source1_string_unit_operand704.code_point_at(source_ml_source1_string_unit_operand705)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand697.clone(), source_ml_source1_string_unit_operand698)
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f646) => {
                            {
                                let source_ml_source1_f711 = source_ml_source1_f646.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f647, source_ml_source1_f648) => {
                            {
                                let source_ml_source1_f720 = source_ml_source1_f647.clone();
                                {
                                    let source_ml_source1_f721 = source_ml_source1_f648;
                                    (-1f64)
                                }
                            }
                        }
                    };
                    if (a != b) {
                        self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
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

pub fn source_ml_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, mut source_ml_temporary_values3: Vec<f64>, mut source_ml_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_temporary_index3 < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))) {
            {
                let unit = self::ml_array::at(&source_ml_temporary_values3, source_ml_temporary_index3.to_index());
                {
                    let text_2 = text.concat(&{
                        let source_ml_source1_character_argument722 = unit;
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source1_character_argument722]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                    });
                    {
                        let source_ml_temporary_index3_2 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_temporary_index3) = (text_2.clone(), source_ml_temporary_index3_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            text.clone()
        };
    }
}

pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < length) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_java_script_semantic_text_unit(value.clone(), at)], units); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (units, at) = (units_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            units
        };
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysLoop9Result {
    MlTemporaryCompareJavaScriptCollectionKeysLoop9Done,
    MlTemporaryCompareJavaScriptCollectionKeysLoop9Return(f64),
}

pub fn source_ml_temporary_compare_java_script_collection_keys_loop9(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut index: f64) -> self::MlTemporaryCompareJavaScriptCollectionKeysLoop9Result {
    loop {
        return if (index < length) {
            {
                let x = self::read_java_script_semantic_text_unit(left.clone(), index);
                {
                    let y = self::read_java_script_semantic_text_unit(right.clone(), index);
                    if (x != y) {
                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop9Result::MlTemporaryCompareJavaScriptCollectionKeysLoop9Return(if (x < y) {
                            (-1f64)
                        } else {
                            1f64
                        })
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
            self::MlTemporaryCompareJavaScriptCollectionKeysLoop9Result::MlTemporaryCompareJavaScriptCollectionKeysLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysJoin8 {
    MlTemporaryCompareJavaScriptCollectionKeysJoin8Next,
    MlTemporaryCompareJavaScriptCollectionKeysJoin8Return(f64),
}

pub fn source_ml_temporary_order_java_script_collection_keys_loop12(mut source: Vec<self::JavaScriptSemanticValue>, mut target: Vec<self::JavaScriptSemanticValue>, mut middle: f64, mut end: f64, mut left: f64, mut right: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if ((left < middle) || (right < end)) {
            {
                let a = self::ml_array::optional_at(&source, self::ml_array::number_index(left));
                {
                    let b = self::ml_array::optional_at(&source, self::ml_array::number_index(right));
                    {
                        let take_left = (right >= end);
                        {
                            let take_left_3 = if (!take_left && (left < middle)) {
                                if !(match (a.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    if !(match (b.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let take_left_2 = ((self::compare_java_script_collection_keys(a.clone().expect("a proven present value"), b.clone().expect("a proven present value"))) <= (0f64));
                                            take_left_2
                                        }
                                    } else {
                                        take_left
                                    }
                                } else {
                                    take_left
                                }
                            } else {
                                take_left
                            };
                            if take_left_3 {
                                {
                                    let target_3 = if !(match (a.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let target_2 = { let (ml_append_right, ml_append_left) = (vec![a.clone().expect("a proven present value")], target); self::ml_array::append(ml_append_left, ml_append_right) };
                                            target_2
                                        }
                                    } else {
                                        target
                                    };
                                    {
                                        let left_2 = (left + 1f64);
                                        {
                                            (target, left) = (target_3, left_2);
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let target_5 = if !(match (b.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let target_4 = { let (ml_append_right, ml_append_left) = (vec![b.clone().expect("a proven present value")], target); self::ml_array::append(ml_append_left, ml_append_right) };
                                            target_4
                                        }
                                    } else {
                                        target
                                    };
                                    {
                                        let right_2 = (right + 1f64);
                                        {
                                            (target, right) = (target_5, right_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            target
        };
    }
}

pub fn source_ml_temporary_order_java_script_collection_keys_loop11(mut source: Vec<self::JavaScriptSemanticValue>, mut width: f64, mut target: Vec<self::JavaScriptSemanticValue>, mut start: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (start < ((source.len() as f64))) {
            {
                let middle = self::ml_math::min((start + width), (source.len() as f64));
                {
                    let end = self::ml_math::min((start + (width * 2f64)), (source.len() as f64));
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let target_2 = self::source_ml_temporary_order_java_script_collection_keys_loop12(source.clone(), target, middle, end, left, right);
                                {
                                    let start_2 = (start + (width * 2f64));
                                    {
                                        (target, start) = (target_2, start_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            target
        };
    }
}

pub fn source_ml_temporary_order_java_script_collection_keys_loop10(mut source: Vec<self::JavaScriptSemanticValue>, mut width: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (width < ((source.len() as f64))) {
            {
                let target = Vec::<self::JavaScriptSemanticValue>::new();
                {
                    let start = 0f64;
                    {
                        let target_2 = self::source_ml_temporary_order_java_script_collection_keys_loop11(source, width, target, start);
                        {
                            let source_2 = target_2;
                            {
                                let width_2 = (width * 2f64);
                                {
                                    (source, width) = (source_2, width_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            source
        };
    }
}

fn ml_main() {
    assert!(((self::test_java_script_collection_key(self::JavaScriptSemanticValue::Number((-0f64)), self::JavaScriptSemanticValue::Number(0f64))) == (true)), "assertion 1");
    assert!(((self::test_java_script_collection_key(self::JavaScriptSemanticValue::Number(f64::NAN), self::JavaScriptSemanticValue::Number(f64::NAN))) == (true)), "assertion 2");
    assert!(((self::test_java_script_collection_key(self::JavaScriptSemanticValue::Number(0f64), self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0)))) == (false)), "assertion 3");
    assert!(((self::test_java_script_collection_key(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units((vec![55296f64]).into_iter().map(|ml_unit| ml_unit as u16).collect())), self::JavaScriptSemanticValue::Text(vec![55296f64]))) == (true)), "assertion 4");
    assert!(((self::test_java_script_collection_key(self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(9), false), self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(10), false))) == (false)), "assertion 5");
    assert!(((self::test_java_script_collection_key(self::JavaScriptSemanticValue::Symbol(self::ml::Big::from_i128(9)), self::JavaScriptSemanticValue::Object(self::ml::Big::from_i128(9), false))) == (false)), "assertion 6");
    assert!(((self::normalize_java_script_collection_key(self::JavaScriptSemanticValue::Number((-0f64)))) == (self::JavaScriptSemanticValue::Number(0f64))), "assertion 7");
    assert!(((self::normalize_java_script_collection_key(self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0)))) == (self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0)))), "assertion 8");
    assert!(((self::read_java_script_collection_operation_kind(self::ml_string::Text::from_units(vec![77, 97, 112, 46, 103, 101, 116]))) == (self::ml_string::Text::from_units(vec![77, 97, 112]))), "assertion 9");
    assert!(((self::read_java_script_collection_operation_kind(self::ml_string::Text::from_units(vec![83, 101, 116, 46, 97, 100, 100]))) == (self::ml_string::Text::from_units(vec![83, 101, 116]))), "assertion 10");
    assert!(((self::read_java_script_collection_operation_kind(self::ml_string::Text::from_units(vec![83, 101, 116, 46, 115, 101, 116]))) == (self::ml_string::Text::from_units(vec![]))), "assertion 11");
    assert!(self::ml_number::same_value(self::compare_java_script_collection_keys(self::JavaScriptSemanticValue::Number(f64::NAN), self::JavaScriptSemanticValue::Number(f64::INFINITY)), 1f64), "assertion 12");
    assert!(self::ml_number::same_value(self::compare_java_script_collection_keys(self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), self::ml_string::Text::from_units(vec![97])), self::JavaScriptSemanticValue::Text(vec![97f64])), 0f64), "assertion 13");
    assert!(((self::order_java_script_collection_keys(vec![self::JavaScriptSemanticValue::Text(vec![98f64]), self::JavaScriptSemanticValue::Number(f64::NAN), self::JavaScriptSemanticValue::Number((-0f64)), self::JavaScriptSemanticValue::Text(vec![97f64])])) == (vec![self::JavaScriptSemanticValue::Number((-0f64)), self::JavaScriptSemanticValue::Number(f64::NAN), self::JavaScriptSemanticValue::Text(vec![97f64]), self::JavaScriptSemanticValue::Text(vec![98f64])])), "assertion 14");
}

// Deep recursion in the source is not bounded by a small native stack.
fn main() {
    let check = std::env::args().any(|argument| argument == "--ml-check-theorems");
    let worker = std::thread::Builder::new()
        .stack_size(1 << 28)
        .spawn(move || { ml_main() })
        .expect("spawn the program thread");
    if worker.join().is_err() {
        std::process::exit(101);
    }
}

#[test]
fn all_literal_inspectable_collection_controls() { main(); }
