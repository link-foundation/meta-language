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
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f58) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f59, source_ml_source2_f60) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f61) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f62) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f63) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f64) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f65, source_ml_source2_f66) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f67) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f68, source_ml_source2_f69) => {
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
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f70) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f71) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f72) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f73, source_ml_source2_f74) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f75) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f76, source_ml_source2_f77) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f78) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f79) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f80) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f81, source_ml_source2_f82) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f83) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f84, source_ml_source2_f85) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f86) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f87) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f88) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f89, source_ml_source2_f90) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f91) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f92, source_ml_source2_f93) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f48) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f49, source_ml_source2_f50) => {
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f94) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f95) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f96) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f97) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f98, source_ml_source2_f99) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f100, source_ml_source2_f101) => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, source_ml_source2_f51) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        false
                    }
                    self::JavaScriptSemanticValue::Null => {
                        false
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f102) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f103) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f104) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f105) => {
                        false
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f106, source_ml_source2_f107) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f108) => {
                        false
                    }
                    self::JavaScriptSemanticValue::Object(identity_2, source_ml_source2_f109) => {
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f110) => {
            {
                let source_ml_source2_f134 = source_ml_source2_f110;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f111) => {
            {
                let source_ml_source2_f143 = source_ml_source2_f111;
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f112) => {
            {
                let source_ml_source2_f152 = source_ml_source2_f112.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f113) => {
            {
                let units = source_ml_source2_f113;
                (units.len() as f64)
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f114, text) => {
            text.length()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f115) => {
            {
                let source_ml_source2_f171 = source_ml_source2_f115.clone();
                (-1f64)
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f116, source_ml_source2_f117) => {
            {
                let source_ml_source2_f180 = source_ml_source2_f116.clone();
                {
                    let source_ml_source2_f181 = source_ml_source2_f117;
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f182) => {
                {
                    let source_ml_source2_f206 = source_ml_source2_f182;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f183) => {
                {
                    let source_ml_source2_f215 = source_ml_source2_f183;
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f184) => {
                {
                    let source_ml_source2_f224 = source_ml_source2_f184.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f185) => {
                {
                    let units = source_ml_source2_f185;
                    (self::ml_array::optional_at(&units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64))
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f186, text) => {
                {
                    let source_ml_source2_string_unit_operand238 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand239 = index;
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
                                    let source_ml_source2_string_unit_operand245 = {
                                        let source_ml_source2_string_range_operand240 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand241 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand242 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position243 = source_ml_source2_string_range_operand241;
                                                    {
                                                        let source_ml_source2_string_range_position244 = source_ml_source2_string_range_operand242;
                                                        source_ml_source2_string_range_operand240.slice(source_ml_source2_string_range_position243, source_ml_source2_string_range_position244)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand246 = 0f64;
                                        source_ml_source2_string_unit_operand245.code_point_at(source_ml_source2_string_unit_operand246)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand238.clone(), source_ml_source2_string_unit_operand239)
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f187) => {
                {
                    let source_ml_source2_f252 = source_ml_source2_f187.clone();
                    (-1f64)
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f188, source_ml_source2_f189) => {
                {
                    let source_ml_source2_f261 = source_ml_source2_f188.clone();
                    {
                        let source_ml_source2_f262 = source_ml_source2_f189;
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
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f263) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f264) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f265) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f266) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f267, source_ml_source2_f268) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f269) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f270, source_ml_source2_f271) => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f272) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f273) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f274) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f275) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f276, source_ml_source2_f277) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f278) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f279, source_ml_source2_f280) => {
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
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value283) => {
                        Some((source_ml_source2_data_property_value283.clone(),))
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
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value284) => {
                        Some((source_ml_source2_data_property_value284.clone(),))
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
                        let source_ml_source2_s285 = self::source_ml_source1_temporary_test_java_script_semantic_text_equal_loop34(left.clone(), right.clone(), length, at);
                        match source_ml_source2_s285.clone() {
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Done => {
                                true
                            }
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Return(source_ml_source1_temporary_result34) => {
                                source_ml_source1_temporary_result34
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
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f286) => {
            {
                let source_ml_source2_f310 = source_ml_source2_f286;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f287) => {
            {
                let source_ml_source2_f319 = source_ml_source2_f287;
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f288) => {
            {
                let source_ml_source2_f328 = source_ml_source2_f288.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f289) => {
            {
                let units = source_ml_source2_f289;
                {
                    let text = self::ml_string::Text::from_units(vec![]);
                    {
                        let source_ml_source1_temporary_values35 = units;
                        {
                            let source_ml_source1_temporary_index35 = self::ml::Big::from_i128(0);
                            {
                                let text_2 = self::source_ml_source1_temporary_read_java_script_semantic_text_string_loop36(text.clone(), source_ml_source1_temporary_values35, source_ml_source1_temporary_index35.clone());
                                text_2.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f290, text_3) => {
            text_3.clone()
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f291) => {
            {
                let source_ml_source2_f347 = source_ml_source2_f291.clone();
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f292, source_ml_source2_f293) => {
            {
                let source_ml_source2_f356 = source_ml_source2_f292.clone();
                {
                    let source_ml_source2_f357 = source_ml_source2_f293;
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
                                let units_2 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units, at);
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
                                let units_4 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_3, at_2);
                                units_4
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f358) => {
                    {
                        let units_5 = Vec::<f64>::new();
                        {
                            let at_3 = 0f64;
                            {
                                let units_6 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_5, at_3);
                                units_6
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f359) => {
                    {
                        let units_7 = Vec::<f64>::new();
                        {
                            let at_4 = 0f64;
                            {
                                let units_8 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_7, at_4);
                                units_8
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f360) => {
                    {
                        let units_9 = Vec::<f64>::new();
                        {
                            let at_5 = 0f64;
                            {
                                let units_10 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_9, at_5);
                                units_10
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_11) => {
                    units_11
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f361, source_ml_source2_f362) => {
                    {
                        let units_12 = Vec::<f64>::new();
                        {
                            let at_6 = 0f64;
                            {
                                let units_13 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_12, at_6);
                                units_13
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f363) => {
                    {
                        let units_14 = Vec::<f64>::new();
                        {
                            let at_7 = 0f64;
                            {
                                let units_15 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_14, at_7);
                                units_15
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f364, source_ml_source2_f365) => {
                    {
                        let units_16 = Vec::<f64>::new();
                        {
                            let at_8 = 0f64;
                            {
                                let units_17 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(value.clone(), length, units_16, at_8);
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
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f366) => {
                {
                    let source_ml_source2_f383 = source_ml_source2_f366;
                    false
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f367) => {
                {
                    let source_ml_source2_f392 = source_ml_source2_f367;
                    false
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f368) => {
                {
                    let source_ml_source2_f401 = source_ml_source2_f368.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f369) => {
                {
                    let source_ml_source2_f410 = source_ml_source2_f369;
                    false
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f370, source_ml_source2_f371) => {
                {
                    let source_ml_source2_f419 = source_ml_source2_f370.clone();
                    {
                        let source_ml_source2_f420 = source_ml_source2_f371.clone();
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f372) => {
                {
                    let source_ml_source2_f429 = source_ml_source2_f372.clone();
                    false
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f373, source_ml_source2_f374) => {
                {
                    let source_ml_source2_f438 = source_ml_source2_f373.clone();
                    {
                        let callable = source_ml_source2_f374;
                        callable
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_number_conversion(operation: self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    if (((operation != (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) && (operation != (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) && (operation != (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99])))) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(f64::NAN)))
            }
            self::JavaScriptSemanticValue::Null => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(0f64)))
            }
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if value_value {
                    1f64
                } else {
                    0f64
                })))
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f439) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_2) => {
                if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114]))
                    } else {
                        if ((self::read_java_script_call_integer_width(value_value_2.clone())) < (0f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![101, 120, 112, 108, 105, 99, 105, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::convert_java_script_big_integer_number(value_value_2.clone()))))
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(units) => {
                if (((units.len() as f64)) > (4096f64)) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::convert_java_script_string_number_units(units))))
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f440, text) => {
                if ((text.length()) > (4096f64)) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                } else {
                    self::execute_java_script_call_number_conversion(operation.clone(), self::JavaScriptSemanticValue::Text(self::construct_java_script_semantic_text_units(value.clone(), 4096f64)))
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f441) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114]))
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f442, source_ml_source2_f443) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
            }
        }
    }
}

pub fn normalize_java_script_unsigned_integer(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-(((-value)).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

pub fn read_utf16_text_integer(value: f64) -> f64 {
    if value.is_nan() {
        0f64
    } else {
        value.trunc()
    }
}

pub fn read_utf16_text_slice_position(value: f64, length: f64) -> f64 {
    {
        let integer = self::read_utf16_text_integer(value);
        if (integer < (0f64)) {
            self::ml_math::max((length + integer), 0f64)
        } else {
            self::ml_math::min(integer, length)
        }
    }
}
