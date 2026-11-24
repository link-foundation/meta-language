pub fn test_java_script_semantic_same_value(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101, 108])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f10) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f11) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f12) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f13) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f14, source_ml_source1_f15) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f16) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f17, source_ml_source1_f18) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f19) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f20) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f21) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f22) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f23, source_ml_source1_f24) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f25) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f26, source_ml_source1_f27) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Boolean(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(value) => {
                        (left_value == value)
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f28) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f29) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f30) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f31, source_ml_source1_f32) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f33) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f34, source_ml_source1_f35) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Number(left_value_2) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f36) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f37) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f38) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f39, source_ml_source1_f40) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f41) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f42, source_ml_source1_f43) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(left_value_3) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f44) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f45) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f46) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f47, source_ml_source1_f48) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f49) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f50, source_ml_source1_f51) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f6) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f7, source_ml_source1_f8) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f52) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f53) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f54) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f55) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f56, source_ml_source1_f57) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f58, source_ml_source1_f59) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source1_f9) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f60) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f61) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f62) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f63) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f64, source_ml_source1_f65) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f66) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(identity_2, source_ml_source1_f67) => {
                        (left_identity_2 == identity_2)
                    }
                }
            }
        }
    }
}

pub fn read_java_script_semantic_text_length(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Null => {
            (-1f64)
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f68) => {
            {
                let source_ml_source1_f92 = source_ml_source1_f68;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f69) => {
            {
                let source_ml_source1_f101 = source_ml_source1_f69;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f70) => {
            {
                let source_ml_source1_f110 = source_ml_source1_f70.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f71) => {
            {
                let units = source_ml_source1_f71;
                (units.len() as f64)
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f72, text) => {
            text.length()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f73) => {
            {
                let source_ml_source1_f129 = source_ml_source1_f73.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f74, source_ml_source1_f75) => {
            {
                let source_ml_source1_f138 = source_ml_source1_f74.clone();
                {
                    let source_ml_source1_f139 = source_ml_source1_f75;
                    (-1f64)
                }
            }
        }
    }
}

pub fn read_java_script_semantic_text_unit(value: self::JavaScriptSemanticValue, index: f64) -> f64 {
    if ((!(self::ml_math::is_safe_integer(index)) || (index < (0f64))) || (index >= (self::read_java_script_semantic_text_length(value.clone())))) {
        (-1f64)
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                (-1f64)
            }
            self::JavaScriptSemanticValue::Null => {
                (-1f64)
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f140) => {
                {
                    let source_ml_source1_f164 = source_ml_source1_f140;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f141) => {
                {
                    let source_ml_source1_f173 = source_ml_source1_f141;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f142) => {
                {
                    let source_ml_source1_f182 = source_ml_source1_f142.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f143) => {
                {
                    let units = source_ml_source1_f143;
                    (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64))
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f144, text) => {
                {
                    let source_ml_source1_string_unit_operand196 = text.clone();
                    {
                        let source_ml_source1_string_unit_operand197 = index;
                        (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text_2.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source1_string_unit_operand203 = {
                                        let source_ml_source1_string_range_operand198 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand199 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand200 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position201 = source_ml_source1_string_range_operand199;
                                                    {
                                                        let source_ml_source1_string_range_position202 = source_ml_source1_string_range_operand200;
                                                        source_ml_source1_string_range_operand198.slice(source_ml_source1_string_range_position201, source_ml_source1_string_range_position202)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand204 = 0f64;
                                        source_ml_source1_string_unit_operand203.code_point_at(source_ml_source1_string_unit_operand204)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand196.clone(), source_ml_source1_string_unit_operand197)
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f145) => {
                {
                    let source_ml_source1_f210 = source_ml_source1_f145.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f146, source_ml_source1_f147) => {
                {
                    let source_ml_source1_f219 = source_ml_source1_f146.clone();
                    {
                        let source_ml_source1_f220 = source_ml_source1_f147;
                        (-1f64)
                    }
                }
            }
        }
    }
}

pub fn test_java_script_semantic_text_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    {
        let length = self::read_java_script_semantic_text_length(left.clone());
        if ((length < (0f64)) || (length != (self::read_java_script_semantic_text_length(right.clone())))) {
            false
        } else {
            if (match left.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f221) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f222) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f223) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f224) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f225, source_ml_source1_f226) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f227) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f228, source_ml_source1_f229) => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f230) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f231) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f232) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f233) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f234, source_ml_source1_f235) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f236) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f237, source_ml_source1_f238) => {
                    false
                }
            }) {
                (((((match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Null => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value241) => {
                        Some((source_ml_source1_data_property_value241.clone(),))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        None::<(self::ml_string::Text,)>
                    }
                }).clone().expect("a proven present value")).0).clone()) == ((((match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Null => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value242) => {
                        Some((source_ml_source1_data_property_value242.clone(),))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        None::<(self::ml_string::Text,)>
                    }
                }).clone().expect("a proven present value")).0).clone()))
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s243 = self::source_ml_temporary_test_java_script_semantic_text_equal_loop2(left.clone(), right.clone(), length, at);
                        match source_ml_source1_s243.clone() {
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
}

pub fn read_java_script_semantic_text_string(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Null => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f244) => {
            {
                let source_ml_source1_f268 = source_ml_source1_f244;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source1_f245) => {
            {
                let source_ml_source1_f277 = source_ml_source1_f245;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f246) => {
            {
                let source_ml_source1_f286 = source_ml_source1_f246.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f247) => {
            {
                let units = source_ml_source1_f247;
                {
                    let text = self::ml_string::Text::from_units(vec![]);
                    {
                        let source_ml_temporary_values3 = units;
                        {
                            let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                            {
                                let text_2 = self::source_ml_temporary_read_java_script_semantic_text_string_loop4(text.clone(), source_ml_temporary_values3, source_ml_temporary_index3.clone());
                                text_2.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source1_f248, text_3) => {
            text_3.clone()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f249) => {
            {
                let source_ml_source1_f305 = source_ml_source1_f249.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source1_f250, source_ml_source1_f251) => {
            {
                let source_ml_source1_f314 = source_ml_source1_f250.clone();
                {
                    let source_ml_source1_f315 = source_ml_source1_f251;
                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
                }
            }
        }
    }
}

pub fn construct_java_script_semantic_text_units(value: self::JavaScriptSemanticValue, limit: f64) -> Vec<f64> {
    {
        let length = self::read_java_script_semantic_text_length(value.clone());
        if ((length < (0f64)) || (length > limit)) {
            (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])).0, message_present: true }).raise()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    {
                        let units = Vec::<f64>::new();
                        {
                            let at = 0f64;
                            {
                                let units_2 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units, at);
                                units_2
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    {
                        let units_3 = Vec::<f64>::new();
                        {
                            let at_2 = 0f64;
                            {
                                let units_4 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_3, at_2);
                                units_4
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f316) => {
                    {
                        let units_5 = Vec::<f64>::new();
                        {
                            let at_3 = 0f64;
                            {
                                let units_6 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_5, at_3);
                                units_6
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f317) => {
                    {
                        let units_7 = Vec::<f64>::new();
                        {
                            let at_4 = 0f64;
                            {
                                let units_8 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_7, at_4);
                                units_8
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f318) => {
                    {
                        let units_9 = Vec::<f64>::new();
                        {
                            let at_5 = 0f64;
                            {
                                let units_10 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_9, at_5);
                                units_10
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_11) => {
                    units_11
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source1_f319, source_ml_source1_f320) => {
                    {
                        let units_12 = Vec::<f64>::new();
                        {
                            let at_6 = 0f64;
                            {
                                let units_13 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_12, at_6);
                                units_13
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f321) => {
                    {
                        let units_14 = Vec::<f64>::new();
                        {
                            let at_7 = 0f64;
                            {
                                let units_15 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_14, at_7);
                                units_15
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f322, source_ml_source1_f323) => {
                    {
                        let units_16 = Vec::<f64>::new();
                        {
                            let at_8 = 0f64;
                            {
                                let units_17 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_16, at_8);
                                units_17
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn construct_java_script_source_text(text: self::ml_string::Text) -> self::JavaScriptSemanticValue {
    if ((self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 83, 116, 114, 105, 110, 103, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
    } else {
        {
            let profile = self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]);
            {
                let error = self::read_java_script_source_text_profile_error(profile.clone(), text.length());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (error.clone()).0, message_present: true }).raise()
                } else {
                    self::JavaScriptSemanticValue::SourceText(profile.clone(), text.clone())
                }
            }
        }
    }
}

pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        false
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                true
            }
            self::JavaScriptSemanticValue::Null => {
                false
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f324) => {
                {
                    let source_ml_source1_f341 = source_ml_source1_f324;
                    false
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source1_f325) => {
                {
                    let source_ml_source1_f350 = source_ml_source1_f325;
                    false
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f326) => {
                {
                    let source_ml_source1_f359 = source_ml_source1_f326.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f327) => {
                {
                    let source_ml_source1_f368 = source_ml_source1_f327;
                    false
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f328, source_ml_source1_f329) => {
                {
                    let source_ml_source1_f377 = source_ml_source1_f328.clone();
                    {
                        let source_ml_source1_f378 = source_ml_source1_f329.clone();
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f330) => {
                {
                    let source_ml_source1_f387 = source_ml_source1_f330.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f331, source_ml_source1_f332) => {
                {
                    let source_ml_source1_f396 = source_ml_source1_f331.clone();
                    {
                        let callable = source_ml_source1_f332;
                        callable
                    }
                }
            }
        }
    }
}

pub fn test_java_script_collection_key(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    {
        let source_ml_source1_s413 = match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f414) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
            }
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f422) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
                    }
                    self::JavaScriptSemanticValue::Number(value) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Return(((left_value == value) || (left_value.is_nan() && value.is_nan())))
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f423) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f424) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source1_f425, source_ml_source1_f426) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f427) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f428, source_ml_source1_f429) => {
                        self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f415) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f416) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f417, source_ml_source1_f418) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f419) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f420, source_ml_source1_f421) => {
                self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next
            }
        };
        match source_ml_source1_s413.clone() {
            self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Next => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::MlTemporaryTestJavaScriptCollectionKeyJoin6::MlTemporaryTestJavaScriptCollectionKeyJoin6Return(source_ml_temporary_result6) => {
                source_ml_temporary_result6
            }
        }
    }
}

pub fn normalize_java_script_collection_key(key: self::JavaScriptSemanticValue) -> self::JavaScriptSemanticValue {
    {
        let source_ml_source1_s438 = match key.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f439) => {
                self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next
            }
            self::JavaScriptSemanticValue::Number(value) => {
                if (value == (0f64)) {
                    self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Return(Box::new(self::JavaScriptSemanticValue::Number(0f64)))
                } else {
                    self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source1_f440) => {
                self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next
            }
            self::JavaScriptSemanticValue::Text(source_ml_source1_f441) => {
                self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source1_f442, source_ml_source1_f443) => {
                self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f444) => {
                self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f445, source_ml_source1_f446) => {
                self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next
            }
        };
        match source_ml_source1_s438.clone() {
            self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Next => {
                key.clone()
            }
            self::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7::MlTemporaryNormalizeJavaScriptCollectionKeyJoin7Return(source_ml_temporary_result7) => {
                let source_ml_temporary_result7 = *source_ml_temporary_result7;
                source_ml_temporary_result7.clone()
            }
        }
    }
}
