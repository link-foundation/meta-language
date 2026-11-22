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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f10) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f11) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f12) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f13) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f14, source_ml_source2_f15) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f16) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f17, source_ml_source2_f18) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f19) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f20) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f21) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f22) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f23, source_ml_source2_f24) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f25) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f26, source_ml_source2_f27) => {
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
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f28) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f29) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f30) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f31, source_ml_source2_f32) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f33) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f34, source_ml_source2_f35) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f36) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f37) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f38) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f39, source_ml_source2_f40) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f41) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f42, source_ml_source2_f43) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f44) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f45) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f46) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f47, source_ml_source2_f48) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f49) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f50, source_ml_source2_f51) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f6) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f7, source_ml_source2_f8) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f52) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f53) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f54) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f55) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f56, source_ml_source2_f57) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f58, source_ml_source2_f59) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source2_f9) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f60) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f61) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f62) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f63) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f64, source_ml_source2_f65) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f66) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f67) => {
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f68) => {
            {
                let source_ml_source2_f92 = source_ml_source2_f68;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f69) => {
            {
                let source_ml_source2_f101 = source_ml_source2_f69;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f70) => {
            {
                let source_ml_source2_f110 = source_ml_source2_f70.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f71) => {
            {
                let units = source_ml_source2_f71.clone();
                { let ml_reference_array2 = units.clone(); let ml_reference_projection2 = ml_reference_array2.borrow().len() as f64; ml_reference_projection2 }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f72, text) => {
            text.length()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f73) => {
            {
                let source_ml_source2_f129 = source_ml_source2_f73.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f74, source_ml_source2_f75) => {
            {
                let source_ml_source2_f138 = source_ml_source2_f74.clone();
                {
                    let source_ml_source2_f139 = source_ml_source2_f75;
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f140) => {
                {
                    let source_ml_source2_f164 = source_ml_source2_f140;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f141) => {
                {
                    let source_ml_source2_f173 = source_ml_source2_f141;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f142) => {
                {
                    let source_ml_source2_f182 = source_ml_source2_f142.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f143) => {
                {
                    let units = source_ml_source2_f143.clone();
                    ({
                        let source_ml_source2_array_read_value196 = units.clone();
                        {
                            let source_ml_source2_array_read_index197 = index;
                            { let ml_reference_array3 = source_ml_source2_array_read_value196.clone(); let ml_reference_position3 = source_ml_source2_array_read_index197; let ml_reference_projection3 = self::ml_array::optional_at(&*ml_reference_array3.borrow(), self::ml_array::number_index(ml_reference_position3)); ml_reference_projection3 }
                        }
                    }).clone().unwrap_or_else(|| (-1f64))
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f144, text) => {
                {
                    let source_ml_source2_string_unit_operand198 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand199 = index;
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
                                    let source_ml_source2_string_unit_operand205 = {
                                        let source_ml_source2_string_range_operand200 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand201 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand202 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position203 = source_ml_source2_string_range_operand201;
                                                    {
                                                        let source_ml_source2_string_range_position204 = source_ml_source2_string_range_operand202;
                                                        source_ml_source2_string_range_operand200.slice(source_ml_source2_string_range_position203, source_ml_source2_string_range_position204)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand206 = 0f64;
                                        source_ml_source2_string_unit_operand205.code_point_at(source_ml_source2_string_unit_operand206)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand198.clone(), source_ml_source2_string_unit_operand199)
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f145) => {
                {
                    let source_ml_source2_f212 = source_ml_source2_f145.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f146, source_ml_source2_f147) => {
                {
                    let source_ml_source2_f221 = source_ml_source2_f146.clone();
                    {
                        let source_ml_source2_f222 = source_ml_source2_f147;
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f223) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f224) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f225) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f226) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f227, source_ml_source2_f228) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f229) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f230, source_ml_source2_f231) => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f232) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f233) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f234) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f235) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f236, source_ml_source2_f237) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f238) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f239, source_ml_source2_f240) => {
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
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value243) => {
                        Some((source_ml_source2_data_property_value243.clone(),))
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
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value244) => {
                        Some((source_ml_source2_data_property_value244.clone(),))
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
                        let source_ml_source2_s245 = self::source_ml_source1_temporary_test_java_script_semantic_text_equal_loop2(left.clone(), right.clone(), length, at);
                        match source_ml_source2_s245.clone() {
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done => {
                                true
                            }
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(source_ml_source1_temporary_result2) => {
                                source_ml_source1_temporary_result2
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f246) => {
            {
                let source_ml_source2_f270 = source_ml_source2_f246;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f247) => {
            {
                let source_ml_source2_f279 = source_ml_source2_f247;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f248) => {
            {
                let source_ml_source2_f288 = source_ml_source2_f248.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f249) => {
            {
                let units = source_ml_source2_f249.clone();
                {
                    let text = self::ml_string::Text::from_units(vec![]);
                    {
                        let source_ml_source1_temporary_values3 = units.clone();
                        {
                            let source_ml_source1_temporary_index3 = self::ml::Big::from_i128(0);
                            {
                                let text_2 = self::source_ml_source1_temporary_read_java_script_semantic_text_string_loop4(text.clone(), source_ml_source1_temporary_values3.clone(), source_ml_source1_temporary_index3.clone());
                                text_2.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f250, text_3) => {
            text_3.clone()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f251) => {
            {
                let source_ml_source2_f307 = source_ml_source2_f251.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f252, source_ml_source2_f253) => {
            {
                let source_ml_source2_f316 = source_ml_source2_f252.clone();
                {
                    let source_ml_source2_f317 = source_ml_source2_f253;
                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
                }
            }
        }
    }
}

pub fn construct_java_script_semantic_text_units(value: self::JavaScriptSemanticValue, limit: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let length = self::read_java_script_semantic_text_length(value.clone());
        if ((length < (0f64)) || (length > limit)) {
            (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116])).0, message_present: true }).raise()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    {
                        let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at = 0f64;
                            {
                                let units_2 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units.clone(), at);
                                units_2.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    {
                        let units_3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_2 = 0f64;
                            {
                                let units_4 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_3.clone(), at_2);
                                units_4.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f318) => {
                    {
                        let units_5 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_3 = 0f64;
                            {
                                let units_6 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_5.clone(), at_3);
                                units_6.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f319) => {
                    {
                        let units_7 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_4 = 0f64;
                            {
                                let units_8 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_7.clone(), at_4);
                                units_8.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f320) => {
                    {
                        let units_9 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_5 = 0f64;
                            {
                                let units_10 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_9.clone(), at_5);
                                units_10.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_11) => {
                    units_11.clone()
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f321, source_ml_source2_f322) => {
                    {
                        let units_12 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_6 = 0f64;
                            {
                                let units_13 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_12.clone(), at_6);
                                units_13.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f323) => {
                    {
                        let units_14 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_7 = 0f64;
                            {
                                let units_15 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_14.clone(), at_7);
                                units_15.clone()
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f324, source_ml_source2_f325) => {
                    {
                        let units_16 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at_8 = 0f64;
                            {
                                let units_17 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_16.clone(), at_8);
                                units_17.clone()
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f326) => {
                {
                    let source_ml_source2_f343 = source_ml_source2_f326;
                    false
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f327) => {
                {
                    let source_ml_source2_f352 = source_ml_source2_f327;
                    false
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f328) => {
                {
                    let source_ml_source2_f361 = source_ml_source2_f328.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f329) => {
                {
                    let source_ml_source2_f370 = source_ml_source2_f329.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f330, source_ml_source2_f331) => {
                {
                    let source_ml_source2_f379 = source_ml_source2_f330.clone();
                    {
                        let source_ml_source2_f380 = source_ml_source2_f331.clone();
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f332) => {
                {
                    let source_ml_source2_f389 = source_ml_source2_f332.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f333, source_ml_source2_f334) => {
                {
                    let source_ml_source2_f398 = source_ml_source2_f333.clone();
                    {
                        let callable = source_ml_source2_f334;
                        callable
                    }
                }
            }
        }
    }
}

pub fn find_java_script_property_descriptor_entry(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source2_s399 = self::source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop6(properties.clone(), key.clone(), at);
            match source_ml_source2_s399.clone() {
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(source_ml_source1_temporary_result6) => {
                    source_ml_source1_temporary_result6
                }
            }
        }
    }
}

pub fn read_java_script_property_descriptor_kind(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::ml_string::Text {
    {
        let data = (({ let ml_reference_value4 = descriptor.borrow().clone(); ml_reference_value4 }).9 || ({ let ml_reference_value5 = descriptor.borrow().clone(); ml_reference_value5 }).11);
        {
            let accessor = (({ let ml_reference_value6 = descriptor.borrow().clone(); ml_reference_value6 }).5 || ({ let ml_reference_value7 = descriptor.borrow().clone(); ml_reference_value7 }).7);
            if (data && accessor) {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else {
                if data {
                    self::ml_string::Text::from_units(vec![100, 97, 116, 97])
                } else {
                    if accessor {
                        self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
                    }
                }
            }
        }
    }
}

pub fn test_java_script_property_descriptors_same_value(left: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, right: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> bool {
    !((((((((((((((({ let ml_reference_value8 = left.borrow().clone(); ml_reference_value8 }).1) != (({ let ml_reference_value9 = right.borrow().clone(); ml_reference_value9 }).1)) || (({ let ml_reference_value10 = left.borrow().clone(); ml_reference_value10 }).1 && ((({ let ml_reference_value11 = left.borrow().clone(); ml_reference_value11 }).0) != (({ let ml_reference_value12 = right.borrow().clone(); ml_reference_value12 }).0)))) || ((({ let ml_reference_value13 = left.borrow().clone(); ml_reference_value13 }).3) != (({ let ml_reference_value14 = right.borrow().clone(); ml_reference_value14 }).3))) || (({ let ml_reference_value15 = left.borrow().clone(); ml_reference_value15 }).3 && ((({ let ml_reference_value16 = left.borrow().clone(); ml_reference_value16 }).2) != (({ let ml_reference_value17 = right.borrow().clone(); ml_reference_value17 }).2)))) || ((({ let ml_reference_value18 = left.borrow().clone(); ml_reference_value18 }).11) != (({ let ml_reference_value19 = right.borrow().clone(); ml_reference_value19 }).11))) || (({ let ml_reference_value20 = left.borrow().clone(); ml_reference_value20 }).11 && ((({ let ml_reference_value21 = left.borrow().clone(); ml_reference_value21 }).10) != (({ let ml_reference_value22 = right.borrow().clone(); ml_reference_value22 }).10)))) || ((({ let ml_reference_value23 = left.borrow().clone(); ml_reference_value23 }).9) != (({ let ml_reference_value24 = right.borrow().clone(); ml_reference_value24 }).9))) || (({ let ml_reference_value25 = left.borrow().clone(); ml_reference_value25 }).9 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value26 = left.borrow().clone(); ml_reference_value26 }).8, ({ let ml_reference_value27 = right.borrow().clone(); ml_reference_value27 }).8)))) || ((({ let ml_reference_value28 = left.borrow().clone(); ml_reference_value28 }).5) != (({ let ml_reference_value29 = right.borrow().clone(); ml_reference_value29 }).5))) || (({ let ml_reference_value30 = left.borrow().clone(); ml_reference_value30 }).5 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value31 = left.borrow().clone(); ml_reference_value31 }).4, ({ let ml_reference_value32 = right.borrow().clone(); ml_reference_value32 }).4)))) || ((({ let ml_reference_value33 = left.borrow().clone(); ml_reference_value33 }).7) != (({ let ml_reference_value34 = right.borrow().clone(); ml_reference_value34 }).7))) || (({ let ml_reference_value35 = left.borrow().clone(); ml_reference_value35 }).7 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value36 = left.borrow().clone(); ml_reference_value36 }).6, ({ let ml_reference_value37 = right.borrow().clone(); ml_reference_value37 }).6)))))
}

pub fn read_java_script_property_descriptor_error(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> self::ml_string::Text {
    if ((self::read_java_script_property_descriptor_kind(descriptor.clone())) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))) {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        if (({ let ml_reference_value38 = descriptor.borrow().clone(); ml_reference_value38 }).9 && ((self::read_java_script_semantic_value_error(({ let ml_reference_value39 = descriptor.borrow().clone(); ml_reference_value39 }).8)) != (self::ml_string::Text::from_units(vec![])))) {
            self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108, 117, 101])
        } else {
            if (({ let ml_reference_value40 = descriptor.borrow().clone(); ml_reference_value40 }).5 && (((self::read_java_script_semantic_value_error(({ let ml_reference_value41 = descriptor.borrow().clone(); ml_reference_value41 }).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor(({ let ml_reference_value42 = descriptor.borrow().clone(); ml_reference_value42 }).4)))) {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                if (({ let ml_reference_value43 = descriptor.borrow().clone(); ml_reference_value43 }).7 && (((self::read_java_script_semantic_value_error(({ let ml_reference_value44 = descriptor.borrow().clone(); ml_reference_value44 }).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor(({ let ml_reference_value45 = descriptor.borrow().clone(); ml_reference_value45 }).6)))) {
                    self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

pub fn test_complete_java_script_property_descriptor(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> bool {
    if ((!(({ let ml_reference_value46 = descriptor.borrow().clone(); ml_reference_value46 }).1) || !(({ let ml_reference_value47 = descriptor.borrow().clone(); ml_reference_value47 }).3)) || ((self::read_java_script_property_descriptor_error(descriptor.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if (kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97]))) {
                (({ let ml_reference_value48 = descriptor.borrow().clone(); ml_reference_value48 }).9 && ({ let ml_reference_value49 = descriptor.borrow().clone(); ml_reference_value49 }).11)
            } else {
                (((kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) && ({ let ml_reference_value50 = descriptor.borrow().clone(); ml_reference_value50 }).5) && ({ let ml_reference_value51 = descriptor.borrow().clone(); ml_reference_value51 }).7)
            }
        }
    }
}
