pub fn execute_java_script_primitive_prefix_search(receiver_text: self::JavaScriptSemanticValue, args: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let search = (self::ml_array::optional_at(&args, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        match search.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                {
                    let search_error = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error.clone())
                    } else {
                        {
                            let converted = self::execute_java_script_call_text_conversion(search.clone());
                            match converted.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                    let value = *value;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f520) => {
                                    converted.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f521, source_ml_source2_f522) => {
                                    converted.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f523) => {
                                    converted.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                {
                    let search_error_2 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_2 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_2.clone())
                    } else {
                        {
                            let converted_2 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_2.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_2) => {
                                    let value_2 = *value_2;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_2.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f524) => {
                                    converted_2.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f525, source_ml_source2_f526) => {
                                    converted_2.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f527) => {
                                    converted_2.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f511) => {
                {
                    let search_error_3 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_3 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_3.clone())
                    } else {
                        {
                            let converted_3 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_3.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_3) => {
                                    let value_3 = *value_3;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_3.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f528) => {
                                    converted_3.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f529, source_ml_source2_f530) => {
                                    converted_3.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f531) => {
                                    converted_3.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f512) => {
                {
                    let search_error_4 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_4 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_4.clone())
                    } else {
                        {
                            let converted_4 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_4.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_4) => {
                                    let value_4 = *value_4;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_4.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f532) => {
                                    converted_4.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f533, source_ml_source2_f534) => {
                                    converted_4.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f535) => {
                                    converted_4.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f513) => {
                {
                    let search_error_5 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_5 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_5.clone())
                    } else {
                        {
                            let converted_5 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_5.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_5) => {
                                    let value_5 = *value_5;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_5.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f536) => {
                                    converted_5.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f537, source_ml_source2_f538) => {
                                    converted_5.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f539) => {
                                    converted_5.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f514) => {
                {
                    let search_error_6 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_6 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_6.clone())
                    } else {
                        {
                            let converted_6 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_6.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_6) => {
                                    let value_6 = *value_6;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_6.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f540) => {
                                    converted_6.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f541, source_ml_source2_f542) => {
                                    converted_6.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f543) => {
                                    converted_6.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::SourceText(source_ml_source2_f515, source_ml_source2_f516) => {
                {
                    let search_error_7 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_7 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_7.clone())
                    } else {
                        {
                            let converted_7 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_7.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_7) => {
                                    let value_7 = *value_7;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_7.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f544) => {
                                    converted_7.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f545, source_ml_source2_f546) => {
                                    converted_7.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f547) => {
                                    converted_7.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f517) => {
                {
                    let search_error_8 = self::read_java_script_semantic_value_error(search.clone());
                    if (search_error_8 != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(search_error_8.clone())
                    } else {
                        {
                            let converted_8 = self::execute_java_script_call_text_conversion(search.clone());
                            match converted_8.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_8) => {
                                    let value_8 = *value_8;
                                    self::execute_java_script_primitive_prefix_position(receiver_text.clone(), value_8.clone(), (self::ml_array::optional_at(&args, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f548) => {
                                    converted_8.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f549, source_ml_source2_f550) => {
                                    converted_8.clone()
                                }
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f551) => {
                                    converted_8.clone()
                                }
                            }
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f518, source_ml_source2_f519) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 73, 115, 82, 101, 103, 69, 120, 112, 32, 97, 110, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108, 115]))
            }
        }
    }
}

pub fn execute_java_script_primitive_prefix_position(receiver_text: self::JavaScriptSemanticValue, search_text: self::JavaScriptSemanticValue, position: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    match position.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            {
                let position_error = self::read_java_script_semantic_value_error(position.clone());
                if (position_error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error.clone())
                } else {
                    {
                        let number = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value) => {
                                let position_value = *position_value;
                                match position_value.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f565) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value) => {
                                        {
                                            let length = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start = self::ml_math::min(length, self::ml_math::max(0f64, self::read_utf16_text_integer(value)));
                                                    if (search_length > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches = (((start + search_length)) <= length);
                                                            {
                                                                let at = 0f64;
                                                                {
                                                                    let matches_2 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length, start, matches, at);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_2)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f566) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f567) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f568, source_ml_source2_f569) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f570) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f571, source_ml_source2_f572) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f561) => {
                                number.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f562, source_ml_source2_f563) => {
                                number.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f564) => {
                                number.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Null => {
            {
                let position_error_2 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_2 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_2.clone())
                } else {
                    {
                        let number_2 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_2.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_2) => {
                                let position_value_2 = *position_value_2;
                                match position_value_2.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f577) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_2) => {
                                        {
                                            let length_2 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_2 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_2 = self::ml_math::min(length_2, self::ml_math::max(0f64, self::read_utf16_text_integer(value_2)));
                                                    if (search_length_2 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_3 = (((start_2 + search_length_2)) <= length_2);
                                                            {
                                                                let at_2 = 0f64;
                                                                {
                                                                    let matches_4 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_2, start_2, matches_3, at_2);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_4)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f578) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f579) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f580, source_ml_source2_f581) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f582) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f583, source_ml_source2_f584) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f573) => {
                                number_2.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f574, source_ml_source2_f575) => {
                                number_2.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f576) => {
                                number_2.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f552) => {
            {
                let position_error_3 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_3 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_3.clone())
                } else {
                    {
                        let number_3 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_3.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_3) => {
                                let position_value_3 = *position_value_3;
                                match position_value_3.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f589) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_3) => {
                                        {
                                            let length_3 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_3 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_3 = self::ml_math::min(length_3, self::ml_math::max(0f64, self::read_utf16_text_integer(value_3)));
                                                    if (search_length_3 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_5 = (((start_3 + search_length_3)) <= length_3);
                                                            {
                                                                let at_3 = 0f64;
                                                                {
                                                                    let matches_6 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_3, start_3, matches_5, at_3);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_6)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f590) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f591) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f592, source_ml_source2_f593) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f594) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f595, source_ml_source2_f596) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f585) => {
                                number_3.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f586, source_ml_source2_f587) => {
                                number_3.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f588) => {
                                number_3.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Number(source_ml_source2_f553) => {
            {
                let position_error_4 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_4 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_4.clone())
                } else {
                    {
                        let number_4 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_4.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_4) => {
                                let position_value_4 = *position_value_4;
                                match position_value_4.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f601) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_4) => {
                                        {
                                            let length_4 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_4 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_4 = self::ml_math::min(length_4, self::ml_math::max(0f64, self::read_utf16_text_integer(value_4)));
                                                    if (search_length_4 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_7 = (((start_4 + search_length_4)) <= length_4);
                                                            {
                                                                let at_4 = 0f64;
                                                                {
                                                                    let matches_8 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_4, start_4, matches_7, at_4);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_8)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f602) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f603) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f604, source_ml_source2_f605) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f606) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f607, source_ml_source2_f608) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f597) => {
                                number_4.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f598, source_ml_source2_f599) => {
                                number_4.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f600) => {
                                number_4.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Integer(source_ml_source2_f554) => {
            {
                let position_error_5 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_5 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_5.clone())
                } else {
                    {
                        let number_5 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_5.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_5) => {
                                let position_value_5 = *position_value_5;
                                match position_value_5.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f613) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_5) => {
                                        {
                                            let length_5 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_5 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_5 = self::ml_math::min(length_5, self::ml_math::max(0f64, self::read_utf16_text_integer(value_5)));
                                                    if (search_length_5 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_9 = (((start_5 + search_length_5)) <= length_5);
                                                            {
                                                                let at_5 = 0f64;
                                                                {
                                                                    let matches_10 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_5, start_5, matches_9, at_5);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_10)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f614) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f615) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f616, source_ml_source2_f617) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f618) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f619, source_ml_source2_f620) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f609) => {
                                number_5.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f610, source_ml_source2_f611) => {
                                number_5.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f612) => {
                                number_5.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Text(source_ml_source2_f555) => {
            {
                let position_error_6 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_6 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_6.clone())
                } else {
                    {
                        let number_6 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_6.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_6) => {
                                let position_value_6 = *position_value_6;
                                match position_value_6.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f625) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_6) => {
                                        {
                                            let length_6 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_6 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_6 = self::ml_math::min(length_6, self::ml_math::max(0f64, self::read_utf16_text_integer(value_6)));
                                                    if (search_length_6 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_11 = (((start_6 + search_length_6)) <= length_6);
                                                            {
                                                                let at_6 = 0f64;
                                                                {
                                                                    let matches_12 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_6, start_6, matches_11, at_6);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_12)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f626) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f627) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f628, source_ml_source2_f629) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f630) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f631, source_ml_source2_f632) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f621) => {
                                number_6.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f622, source_ml_source2_f623) => {
                                number_6.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f624) => {
                                number_6.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f556, source_ml_source2_f557) => {
            {
                let position_error_7 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_7 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_7.clone())
                } else {
                    {
                        let number_7 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_7.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_7) => {
                                let position_value_7 = *position_value_7;
                                match position_value_7.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f637) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_7) => {
                                        {
                                            let length_7 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_7 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_7 = self::ml_math::min(length_7, self::ml_math::max(0f64, self::read_utf16_text_integer(value_7)));
                                                    if (search_length_7 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_13 = (((start_7 + search_length_7)) <= length_7);
                                                            {
                                                                let at_7 = 0f64;
                                                                {
                                                                    let matches_14 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_7, start_7, matches_13, at_7);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_14)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f638) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f639) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f640, source_ml_source2_f641) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f642) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f643, source_ml_source2_f644) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f633) => {
                                number_7.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f634, source_ml_source2_f635) => {
                                number_7.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f636) => {
                                number_7.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f558) => {
            {
                let position_error_8 = self::read_java_script_semantic_value_error(position.clone());
                if (position_error_8 != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(position_error_8.clone())
                } else {
                    {
                        let number_8 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), position.clone());
                        match number_8.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(position_value_8) => {
                                let position_value_8 = *position_value_8;
                                match position_value_8.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f649) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Number(value_8) => {
                                        {
                                            let length_8 = self::read_java_script_semantic_text_length(receiver_text.clone());
                                            {
                                                let search_length_8 = self::read_java_script_semantic_text_length(search_text.clone());
                                                {
                                                    let start_8 = self::ml_math::min(length_8, self::ml_math::max(0f64, self::read_utf16_text_integer(value_8)));
                                                    if (search_length_8 > (1048576f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                    } else {
                                                        {
                                                            let matches_15 = (((start_8 + search_length_8)) <= length_8);
                                                            {
                                                                let at_8 = 0f64;
                                                                {
                                                                    let matches_16 = self::source_ml_source1_temporary_execute_java_script_primitive_prefix_position_loop62(receiver_text.clone(), search_text.clone(), search_length_8, start_8, matches_15, at_8);
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_16)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f650) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Text(source_ml_source2_f651) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f652, source_ml_source2_f653) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f654) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                    self::JavaScriptSemanticValue::Object(source_ml_source2_f655, source_ml_source2_f656) => {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                                    }
                                }
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f645) => {
                                number_8.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f646, source_ml_source2_f647) => {
                                number_8.clone()
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f648) => {
                                number_8.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Object(source_ml_source2_f559, source_ml_source2_f560) => {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Result {
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Done(f64),
    MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_call_integer_width_loop1(mut rest: self::ml::Big, mut width: f64) -> self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Result {
    loop {
        return if (rest > (self::ml::Big::from_i128(0))) {
            if (width >= (4096f64)) {
                self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Return((-1f64))
            } else {
                {
                    let rest_2 = self::ml::divide(&rest, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let width_2 = (width + 1f64);
                        {
                            (rest, width) = (rest_2.clone(), width_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Result::MlSource1TemporaryReadJavaScriptCallIntegerWidthLoop1Done(width)
        };
    }
}
