pub fn execute_java_script_string_method_primitive(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    {
        let receiver = ({
            let source_ml_source2_array_read_index1475 = 0f64;
            { let ml_reference_array243 = values.clone(); let ml_reference_position243 = source_ml_source2_array_read_index1475; let ml_reference_projection243 = self::ml_array::optional_at(&*ml_reference_array243.borrow(), self::ml_array::number_index(ml_reference_position243)); ml_reference_projection243 }
        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let second = ({
                let source_ml_source2_array_read_index1477 = 1f64;
                { let ml_reference_array244 = values.clone(); let ml_reference_position244 = source_ml_source2_array_read_index1477; let ml_reference_projection244 = self::ml_array::optional_at(&*ml_reference_array244.borrow(), self::ml_array::number_index(ml_reference_position244)); ml_reference_projection244 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            {
                let third = ({
                    let source_ml_source2_array_read_index1479 = 2f64;
                    { let ml_reference_array245 = values.clone(); let ml_reference_position245 = source_ml_source2_array_read_index1479; let ml_reference_projection245 = self::ml_array::optional_at(&*ml_reference_array245.borrow(), self::ml_array::number_index(ml_reference_position245)); ml_reference_projection245 }
                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114, 32, 99, 111, 100, 101, 32, 116, 101, 120, 116]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116, 32, 116, 101, 120, 116])))) {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Number(value) => {
                            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114, 32, 99, 111, 100, 101, 32, 116, 101, 120, 116]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![(self::dependency_function1(value) % 65536f64)])))))
                            } else {
                                if ((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value > (1114111f64))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), (self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(value))))
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(if (value <= (65535f64)) {
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![if (value == (0f64)) {
                                            0f64
                                        } else {
                                            value
                                        }]))
                                    } else {
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![(55296f64 + (((value - 65536f64) / 1024f64)).floor()), (56320f64 + ((value - 65536f64) % 1024f64))]))
                                    })))
                                }
                            }
                        }
                        _ => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 101, 97, 99, 104, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 97, 114, 103, 117, 109, 101, 110, 116]))
                        }
                    }
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 83, 116, 114, 105, 110, 103, 32, 114, 101, 99, 101, 105, 118, 101, 114]))) {
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
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 99, 104, 101, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 109, 101, 116, 104, 111, 100, 32, 110, 97, 109, 101]))
                        } else {
                            if (match receiver.clone() {
                                self::JavaScriptSemanticValue::Null => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            } || match receiver.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            }) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), ((self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46])).concat(&self::read_java_script_semantic_text_string(second.clone()))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 108, 108, 101, 100, 32, 111, 110, 32, 110, 117, 108, 108, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100])))
                            } else {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(receiver.clone()))
                            }
                        }
                    } else {
                        if (match receiver.clone() {
                            self::JavaScriptSemanticValue::Text(_) => {
                                false
                            }
                            _ => {
                                true
                            }
                        } && match receiver.clone() {
                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                false
                            }
                            _ => {
                                true
                            }
                        }) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115, 32, 99, 111, 110, 115, 117, 109, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        } else {
                            {
                                let length = self::read_java_script_semantic_text_length(receiver.clone());
                                if ((operation == (self::ml_string::Text::from_units(vec![109, 97, 112, 32, 108, 111, 119, 101, 114, 32, 83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101]))) || (operation == (self::ml_string::Text::from_units(vec![109, 97, 112, 32, 117, 112, 112, 101, 114, 32, 83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101])))) {
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
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 85, 110, 105, 99, 111, 100, 101, 32, 112, 114, 111, 102, 105, 108, 101]))
                                    } else {
                                        {
                                            let profile_error = self::read_java_script_string_case_profile_error(self::read_java_script_semantic_text_string(second.clone()));
                                            if (profile_error != (self::ml_string::Text::from_units(vec![]))) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(profile_error.clone())
                                            } else {
                                                if (length > (1048576f64)) {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 105, 110, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                } else {
                                                    {
                                                        let units = self::map_unicode_case_utf16_text(self::construct_java_script_semantic_text_units(receiver.clone(), 1048576f64), (operation == (self::ml_string::Text::from_units(vec![109, 97, 112, 32, 117, 112, 112, 101, 114, 32, 83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101]))));
                                                        if (({ let ml_reference_array246 = units.clone(); let ml_reference_projection246 = ml_reference_array246.borrow().len() as f64; ml_reference_projection246 }) > (1048576f64)) {
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
                                    if ((operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 112, 121, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103])))) {
                                        if (length > (1048576f64)) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 112, 114, 111, 99, 101, 115, 115, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 48, 52, 56, 53, 55, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                        } else {
                                            {
                                                let units_2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                {
                                                    let formed = true;
                                                    {
                                                        let at = 0f64;
                                                        {
                                                            let source_ml_source2_s1561 = self::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop101(operation.clone(), receiver.clone(), length, units_2.clone(), formed, at);
                                                            match source_ml_source2_s1561.clone() {
                                                                self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Done(units_3, formed_2) => {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103]))) {
                                                                        self::JavaScriptSemanticValue::Boolean(formed_2)
                                                                    } else {
                                                                        if formed_2 {
                                                                            receiver.clone()
                                                                        } else {
                                                                            self::JavaScriptSemanticValue::Text(units_3.clone())
                                                                        }
                                                                    }))
                                                                }
                                                                self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop101Return(source_ml_source1_temporary_result101) => {
                                                                    let source_ml_source1_temporary_result101 = *source_ml_source1_temporary_result101;
                                                                    source_ml_source1_temporary_result101.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if ((operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121]))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121])))) {
                                            if ((match second.clone() {
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
                                            }) && match second.clone() {
                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 45, 112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 97, 100, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115]))
                                            } else {
                                                {
                                                    let index = self::read_java_script_array_index(second.clone());
                                                    {
                                                        let is_length = self::test_java_script_array_length_key(second.clone());
                                                        {
                                                            let present = (is_length || ((index >= (0f64)) && (index < length)));
                                                            if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 107, 101, 121]))) {
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
                                            if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116]))) {
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
                                                if ((operation == (self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 105, 110, 103, 32, 83, 116, 114, 105, 110, 103, 32, 117, 110, 105, 116, 115]))) || (operation == (self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 32, 83, 116, 114, 105, 110, 103, 32, 117, 110, 105, 116, 115])))) {
                                                    match second.clone() {
                                                        self::JavaScriptSemanticValue::Number(second_value) => {
                                                            match third.clone() {
                                                                self::JavaScriptSemanticValue::Number(value_3) => {
                                                                    {
                                                                        let first = self::read_utf16_text_integer(second_value);
                                                                        {
                                                                            let last = self::read_utf16_text_integer(value_3);
                                                                            {
                                                                                let start = if (operation == (self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 32, 83, 116, 114, 105, 110, 103, 32, 117, 110, 105, 116, 115]))) {
                                                                                    if (first < (0f64)) {
                                                                                        self::ml_math::max((length + first), 0f64)
                                                                                    } else {
                                                                                        self::ml_math::min(first, length)
                                                                                    }
                                                                                } else {
                                                                                    self::ml_math::min(self::ml_math::max(first, 0f64), length)
                                                                                };
                                                                                {
                                                                                    let end = if (operation == (self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 32, 83, 116, 114, 105, 110, 103, 32, 117, 110, 105, 116, 115]))) {
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
                                                                                                    let units_4 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                                                    {
                                                                                                        let units_5 = self::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop102(receiver.clone(), high, units_4.clone(), low);
                                                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units_5.clone())))
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
                                                                            let start_2 = if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 115, 117, 102, 102, 105, 120, 32, 97, 116]))) {
                                                                                (position_2 - needle_length)
                                                                            } else {
                                                                                position_2
                                                                            };
                                                                            if ((operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 97, 116]))) || (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 115, 117, 102, 102, 105, 120, 32, 97, 116])))) {
                                                                                if (needle_length > (1048576f64)) {
                                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 98, 111, 117, 110, 100, 97, 114, 121, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 119, 111, 114, 107, 32, 98, 117, 100, 103, 101, 116]))
                                                                                } else {
                                                                                    {
                                                                                        let matches = ((start_2 >= (0f64)) && (((start_2 + needle_length)) <= length));
                                                                                        {
                                                                                            let at_2 = 0f64;
                                                                                            {
                                                                                                let matches_2 = self::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop103(receiver.clone(), second.clone(), needle_length, start_2, matches, at_2);
                                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(matches_2)))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if (operation != (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110, 32, 97, 116]))) {
                                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 104, 101, 108, 112, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                                                                } else {
                                                                                    if (((self::ml_math::max(0f64, (((length - position_2) - needle_length) + 1f64)) * needle_length)) > (67108864f64)) {
                                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 119, 111, 114, 107, 32, 98, 117, 100, 103, 101, 116]))
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_s1639 = self::source_ml_source1_temporary_execute_java_script_string_method_primitive_loop104(receiver.clone(), second.clone(), length, needle_length, position_2);
                                                                                            match source_ml_source2_s1639.clone() {
                                                                                                self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Done => {
                                                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(false)))
                                                                                                }
                                                                                                self::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Result::MlSource1TemporaryExecuteJavaScriptStringMethodPrimitiveLoop104Return(source_ml_source1_temporary_result104) => {
                                                                                                    let source_ml_source1_temporary_result104 = *source_ml_source1_temporary_result104;
                                                                                                    source_ml_source1_temporary_result104.clone()
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

pub fn read_java_script_string_case_profile_error(profile: self::ml_string::Text) -> self::ml_string::Text {
    if (profile == (self::ml_string::Text::from_units(vec![49, 54, 46, 48]))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 100, 101, 102, 97, 117, 108, 116, 32, 99, 97, 115, 105, 110, 103, 32, 104, 97, 115, 32, 110, 111, 32, 97, 117, 116, 104, 111, 114, 101, 100, 32, 116, 97, 98, 108, 101, 32, 102, 111, 114, 32, 97, 99, 116, 105, 118, 101, 32, 85, 110, 105, 99, 111, 100, 101, 32, 112, 114, 111, 102, 105, 108, 101, 32])).concat(&profile)
    }
}

pub fn read_java_script_array_like_length(value: f64) -> f64 {
    if (value.is_nan() || (value <= (0f64))) {
        0f64
    } else {
        self::ml_math::min(value.floor(), 9007199254740991f64)
    }
}

pub fn test_java_script_array_search_operation(operation: self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value247 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 105, 110, 99, 108, 117, 100, 101, 115]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 105, 110, 100, 101, 120, 79, 102]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 97, 116])]))).borrow().clone(); ml_reference_value247 }, &operation.clone(), 0f64, false, |a, b| a == b)
}

pub fn read_java_script_array_search_index(operation: self::ml_string::Text, length: f64, present: bool, value: f64) -> f64 {
    if ((operation == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102]))) && !present) {
        (length - 1f64)
    } else {
        {
            let integer = if (value.is_nan() || (value == (0f64))) {
                0f64
            } else {
                value.trunc()
            };
            if (operation == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 97, 116]))) {
                {
                    let index = if (integer < (0f64)) {
                        (length + integer)
                    } else {
                        integer
                    };
                    if ((index < (0f64)) || (index >= length)) {
                        (-1f64)
                    } else {
                        index
                    }
                }
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102]))) {
                    if (integer == (f64::NEG_INFINITY)) {
                        (-1f64)
                    } else {
                        if (integer >= (0f64)) {
                            self::ml_math::min(integer, (length - 1f64))
                        } else {
                            self::ml_math::max((length + integer), (-1f64))
                        }
                    }
                } else {
                    if (integer == (f64::INFINITY)) {
                        length
                    } else {
                        if (integer >= (0f64)) {
                            self::ml_math::min(integer, length)
                        } else {
                            self::ml_math::max((length + integer), 0f64)
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_indexed_property_presence(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, key: self::JavaScriptSemanticValue) -> std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, bool)>> {
    {
        let depth = 0f64;
        {
            let source_ml_source2_s1640 = self::source_ml_source1_temporary_read_java_script_indexed_property_presence_loop106(heap.clone(), key.clone(), identity.clone(), depth);
            match source_ml_source2_s1640.clone() {
                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Done => {
                    std::rc::Rc::new(std::cell::RefCell::new((self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 108, 111, 111, 107, 117, 112, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]), false)))
                }
                self::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Result::MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop106Return(source_ml_source1_temporary_result106) => {
                    source_ml_source1_temporary_result106.clone()
                }
            }
        }
    }
}

pub fn read_big_integer_bitwise_operation_code(operation: self::ml_string::Text) -> f64 {
    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115]))) {
        0f64
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115]))) {
            1f64
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 69, 113, 117, 97, 108, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115]))) {
                2f64
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115, 76, 101, 102, 116]))) {
                    3f64
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116]))) {
                        4f64
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115]))) {
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

pub fn read_big_integer_bitwise_operation(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
        self::ml_string::Text::from_units(vec![99, 111, 110, 106, 111, 105, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115])
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
            self::ml_string::Text::from_units(vec![100, 105, 115, 106, 111, 105, 110, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115])
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 69, 113, 117, 97, 108, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
                self::ml_string::Text::from_units(vec![101, 120, 99, 108, 117, 100, 101, 69, 113, 117, 97, 108, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115])
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 76, 101, 102, 116]))) {
                    self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115, 76, 101, 102, 116])
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116]))) {
                        self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115, 82, 105, 103, 104, 116])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 78, 117, 109, 98, 101, 114, 66, 105, 116, 115]))) {
                            self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 109, 101, 110, 116, 66, 105, 103, 73, 110, 116, 101, 103, 101, 114, 66, 105, 116, 115])
                        } else {
                            if ((self::read_big_integer_bitwise_operation_code(operation.clone())) < (0f64)) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                operation.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_big_integer_bitwise_arity(operation: self::ml_string::Text) -> f64 {
    {
        let code = self::read_big_integer_bitwise_operation_code(operation.clone());
        if (code == (6f64)) {
            1f64
        } else {
            if (code < (0f64)) {
                0f64
            } else {
                2f64
            }
        }
    }
}

pub fn accept_linked_big_integer_bitwise_operation(expression: self::ml_string::Text, operation: self::ml_string::Text) -> bool {
    ((expression == (self::ml_string::Text::from_units(vec![109, 97, 116, 104]))) && ((self::read_big_integer_bitwise_arity(operation.clone())) != (0f64)))
}

pub fn read_primitive_bitwise_domain(kinds: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> self::ml_string::Text {
    if (({ let ml_reference_array248 = kinds.clone(); let ml_reference_projection248 = ml_reference_array248.borrow().len() as f64; ml_reference_projection248 }) == (0f64)) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let number = true;
            {
                let integer = true;
                {
                    let source_ml_source1_temporary_index107 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s1643 = self::source_ml_source1_temporary_read_primitive_bitwise_domain_loop108(number, integer, kinds.clone(), source_ml_source1_temporary_index107.clone());
                        match source_ml_source2_s1643.clone() {
                            self::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop108Result::MlSource1TemporaryReadPrimitiveBitwiseDomainLoop108Done(number_2, integer_2) => {
                                if number_2 {
                                    self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114])
                                } else {
                                    if integer_2 {
                                        self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116])
                                    } else {
                                        self::ml_string::Text::from_units(vec![])
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

pub fn apply_big_integer_bitwise_operation(operation: f64, left: self::ml::Big, right: self::ml::Big) -> self::ml::Big {
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
                            let source_ml_source2_s1644 = if (count < (self::ml::Big::from_i128(0))) {
                                {
                                    let count_2 = count.neg();
                                    {
                                        let forward_2 = !forward;
                                        self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin109::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin109Next(count_2.clone(), forward_2)
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin109::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin109Next(count.clone(), forward)
                            };
                            match source_ml_source2_s1644.clone() {
                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin109::MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin109Next(count_3, forward_3) => {
                                    {
                                        let value = left.clone();
                                        {
                                            let value_4 = if forward_3 {
                                                {
                                                    let factor = self::ml::Big::from_i128(2);
                                                    {
                                                        let value_2 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop110(count_3.clone(), value.clone(), factor.clone());
                                                        value_2.clone()
                                                    }
                                                }
                                            } else {
                                                {
                                                    let value_3 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop111(count_3.clone(), value.clone());
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
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 117, 110, 107, 110, 111, 119, 110, 32, 66, 105, 103, 73, 110, 116, 32, 98, 105, 116, 119, 105, 115, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])).0, message_present: true }).raise()
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
                                            let source_ml_source2_s1645 = self::source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop112(operation, negative_left, negative_right, first.clone(), second.clone(), weight.clone(), result.clone());
                                            match source_ml_source2_s1645.clone() {
                                                self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop112Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop112Done(weight_2, result_2) => {
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

pub fn render_big_integer_bitwise_expression(target: self::ml_string::Text, operation: self::ml_string::Text, arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> self::ml_string::Text {
    {
        let code = self::read_big_integer_bitwise_operation_code(operation.clone());
        if ((code < (0f64)) || (({ let ml_reference_array249 = arguments_.clone(); let ml_reference_projection249 = ml_reference_array249.borrow().len() as f64; ml_reference_projection249 }) != (self::read_big_integer_bitwise_arity(operation.clone())))) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let left = ({
                    let source_ml_source2_array_read_index1647 = 0f64;
                    { let ml_reference_array250 = arguments_.clone(); let ml_reference_position250 = source_ml_source2_array_read_index1647; let ml_reference_projection250 = self::ml_array::optional_at(&*ml_reference_array250.borrow(), self::ml_array::number_index(ml_reference_position250)); ml_reference_projection250 }
                }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                {
                    let right = ({
                        let source_ml_source2_array_read_index1649 = 1f64;
                        { let ml_reference_array251 = arguments_.clone(); let ml_reference_position251 = source_ml_source2_array_read_index1649; let ml_reference_projection251 = self::ml_array::optional_at(&*ml_reference_array251.borrow(), self::ml_array::number_index(ml_reference_position251)); ml_reference_projection251 }
                    }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    if (target == (self::ml_string::Text::from_units(vec![82, 111, 99, 113]))) {
                        {
                            let operation_name = if (code == (0f64)) {
                                self::ml_string::Text::from_units(vec![90, 46, 108, 97, 110, 100])
                            } else {
                                if (code == (1f64)) {
                                    self::ml_string::Text::from_units(vec![90, 46, 108, 111, 114])
                                } else {
                                    if (code == (2f64)) {
                                        self::ml_string::Text::from_units(vec![90, 46, 108, 120, 111, 114])
                                    } else {
                                        if (code == (3f64)) {
                                            self::ml_string::Text::from_units(vec![90, 46, 115, 104, 105, 102, 116, 108])
                                        } else {
                                            if (code == (4f64)) {
                                                self::ml_string::Text::from_units(vec![90, 46, 115, 104, 105, 102, 116, 114])
                                            } else {
                                                self::ml_string::Text::from_units(vec![90, 46, 108, 110, 111, 116])
                                            }
                                        }
                                    }
                                }
                            };
                            (((((self::ml_string::Text::from_units(vec![40])).concat(&operation_name)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&left)).concat(&if (code == (6f64)) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                (self::ml_string::Text::from_units(vec![32])).concat(&right)
                            })).concat(&self::ml_string::Text::from_units(vec![41]))
                        }
                    } else {
                        if (target == (self::ml_string::Text::from_units(vec![76, 101, 97, 110]))) {
                            if (code == (6f64)) {
                                ((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                            } else {
                                {
                                    let text = (left.concat(&self::ml_string::Text::from_units(vec![32]))).concat(&right);
                                    {
                                        let suffix = 0f64;
                                        {
                                            let suffix_2 = self::source_ml_source1_temporary_render_big_integer_bitwise_expression_loop113(text.clone(), suffix);
                                            {
                                                let a = (self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 108, 101, 102, 116])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix_2)));
                                                {
                                                    let b = (self::ml_string::Text::from_units(vec![109, 108, 95, 105, 110, 116, 101, 103, 101, 114, 95, 114, 105, 103, 104, 116])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(suffix_2)));
                                                    {
                                                        let am = ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32, 40, 45, 49, 32, 45, 32])).concat(&a)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                        {
                                                            let bm = ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32, 40, 45, 49, 32, 45, 32])).concat(&b)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                            {
                                                                let ap = ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32])).concat(&a)).concat(&self::ml_string::Text::from_units(vec![41]));
                                                                {
                                                                    let bp = ((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32])).concat(&b)).concat(&self::ml_string::Text::from_units(vec![41]));
                                                                    {
                                                                        let prefix = ((((((((self::ml_string::Text::from_units(vec![40, 108, 101, 116, 32])).concat(&a)).concat(&self::ml_string::Text::from_units(vec![32, 58, 32, 73, 110, 116, 32, 58, 61, 32]))).concat(&left)).concat(&self::ml_string::Text::from_units(vec![59, 32, 108, 101, 116, 32]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 58, 32, 73, 110, 116, 32, 58, 61, 32]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![59, 32]));
                                                                        if ((code == (3f64)) || (code == (4f64))) {
                                                                            {
                                                                                let count = ((((((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 116, 111, 78, 97, 116, 32, 40, 105, 102, 32])).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32, 45]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                                                {
                                                                                    let shift_left = ((((self::ml_string::Text::from_units(vec![40])).concat(&a)).concat(&self::ml_string::Text::from_units(vec![32, 42, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 115, 104, 105, 102, 116, 76, 101, 102, 116, 32, 49, 32]))).concat(&count)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                                                    {
                                                                                        let shift_right = ((((((((((self::ml_string::Text::from_units(vec![40, 105, 102, 32])).concat(&a)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 115, 104, 105, 102, 116, 82, 105, 103, 104, 116, 32]))).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&count)).concat(&self::ml_string::Text::from_units(vec![41, 32, 101, 108, 115, 101, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 115, 104, 105, 102, 116, 82, 105, 103, 104, 116, 32]))).concat(&ap)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&count)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                                                        ((((((((prefix.concat(&self::ml_string::Text::from_units(vec![105, 102, 32]))).concat(&a)).concat(&self::ml_string::Text::from_units(vec![32, 61, 32, 48, 32, 116, 104, 101, 110, 32, 48, 32, 101, 108, 115, 101, 32, 105, 102, 32]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32]))).concat(&if (code == (3f64)) {
                                                                                            shift_right.clone()
                                                                                        } else {
                                                                                            shift_left.clone()
                                                                                        })).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32]))).concat(&if (code == (3f64)) {
                                                                                            shift_left.clone()
                                                                                        } else {
                                                                                            shift_right.clone()
                                                                                        })).concat(&self::ml_string::Text::from_units(vec![41]))
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            {
                                                                                let both = if (code == (0f64)) {
                                                                                    ((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 108, 111, 114, 32])).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                } else {
                                                                                    if (code == (1f64)) {
                                                                                        ((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 108, 97, 110, 100, 32])).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                    } else {
                                                                                        ((((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 120, 111, 114, 32])).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let left_negative = if (code == (0f64)) {
                                                                                        (((((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&self::ml_string::Text::from_units(vec![40, 78, 97, 116, 46, 98, 105, 116, 119, 105, 115, 101, 32, 40, 102, 117, 110, 32, 109, 108, 95, 98, 105, 116, 95, 108, 101, 102, 116, 32, 109, 108, 95, 98, 105, 116, 95, 114, 105, 103, 104, 116, 32, 61, 62, 32, 109, 108, 95, 98, 105, 116, 95, 108, 101, 102, 116, 32, 38, 38, 32, 33, 109, 108, 95, 98, 105, 116, 95, 114, 105, 103, 104, 116, 41, 32]))).concat(&bp)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&am)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                    } else {
                                                                                        if (code == (1f64)) {
                                                                                            (((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&self::ml_string::Text::from_units(vec![40, 78, 97, 116, 46, 98, 105, 116, 119, 105, 115, 101, 32, 40, 102, 117, 110, 32, 109, 108, 95, 98, 105, 116, 95, 108, 101, 102, 116, 32, 109, 108, 95, 98, 105, 116, 95, 114, 105, 103, 104, 116, 32, 61, 62, 32, 109, 108, 95, 98, 105, 116, 95, 108, 101, 102, 116, 32, 38, 38, 32, 33, 109, 108, 95, 98, 105, 116, 95, 114, 105, 103, 104, 116, 41, 32]))).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bp)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                        } else {
                                                                                            ((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 120, 111, 114, 32])).concat(&am)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bp)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let right_negative = if (code == (0f64)) {
                                                                                            (((((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&self::ml_string::Text::from_units(vec![40, 78, 97, 116, 46, 98, 105, 116, 119, 105, 115, 101, 32, 40, 102, 117, 110, 32, 109, 108, 95, 98, 105, 116, 95, 108, 101, 102, 116, 32, 109, 108, 95, 98, 105, 116, 95, 114, 105, 103, 104, 116, 32, 61, 62, 32, 109, 108, 95, 98, 105, 116, 95, 108, 101, 102, 116, 32, 38, 38, 32, 33, 109, 108, 95, 98, 105, 116, 95, 114, 105, 103, 104, 116, 41, 32]))).concat(&ap)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                        } else {
                                                                                            if (code == (1f64)) {
                                                                                                (((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32])).concat(&self::ml_string::Text::from_units(vec![40, 78, 97, 116, 46, 98, 105, 116, 119, 105, 115, 101, 32, 40, 102, 117, 110, 32, 109, 108, 95, 98, 105, 116, 95, 108, 101, 102, 116, 32, 109, 108, 95, 98, 105, 116, 95, 114, 105, 103, 104, 116, 32, 61, 62, 32, 109, 108, 95, 98, 105, 116, 95, 108, 101, 102, 116, 32, 38, 38, 32, 33, 109, 108, 95, 98, 105, 116, 95, 114, 105, 103, 104, 116, 41, 32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&ap)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                            } else {
                                                                                                ((((self::ml_string::Text::from_units(vec![40, 45, 49, 32, 45, 32, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40, 78, 97, 116, 46, 120, 111, 114, 32])).concat(&ap)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bm)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let positive = ((((((self::ml_string::Text::from_units(vec![40, 73, 110, 116, 46, 111, 102, 78, 97, 116, 32, 40])).concat(&if (code == (0f64)) {
                                                                                                self::ml_string::Text::from_units(vec![78, 97, 116, 46, 108, 97, 110, 100])
                                                                                            } else {
                                                                                                if (code == (1f64)) {
                                                                                                    self::ml_string::Text::from_units(vec![78, 97, 116, 46, 108, 111, 114])
                                                                                                } else {
                                                                                                    self::ml_string::Text::from_units(vec![78, 97, 116, 46, 120, 111, 114])
                                                                                                }
                                                                                            })).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&ap)).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&bp)).concat(&self::ml_string::Text::from_units(vec![41, 41]));
                                                                                            ((((((((((((((prefix.concat(&self::ml_string::Text::from_units(vec![105, 102, 32]))).concat(&a)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32, 40, 105, 102, 32]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32]))).concat(&both)).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32]))).concat(&left_negative)).concat(&self::ml_string::Text::from_units(vec![41, 32, 101, 108, 115, 101, 32, 40, 105, 102, 32]))).concat(&b)).concat(&self::ml_string::Text::from_units(vec![32, 60, 32, 48, 32, 116, 104, 101, 110, 32]))).concat(&right_negative)).concat(&self::ml_string::Text::from_units(vec![32, 101, 108, 115, 101, 32]))).concat(&positive)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
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
                        } else {
                            if (target == (self::ml_string::Text::from_units(vec![82, 117, 115, 116]))) {
                                ((((((self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101, 58, 58, 109, 108, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 98, 105, 116, 119, 105, 115, 101, 58, 58, 97, 112, 112, 108, 121, 95, 98, 105, 103, 95, 105, 110, 116, 101, 103, 101, 114, 95, 98, 105, 116, 119, 105, 115, 101, 95, 111, 112, 101, 114, 97, 116, 105, 111, 110, 40])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(code)))).concat(&self::ml_string::Text::from_units(vec![102, 54, 52, 44, 32]))).concat(&left)).concat(&self::ml_string::Text::from_units(vec![44, 32]))).concat(&if (code == (6f64)) {
                                    self::ml_string::Text::from_units(vec![109, 108, 58, 58, 66, 105, 103, 58, 58, 122, 101, 114, 111, 40, 41])
                                } else {
                                    right.clone()
                                })).concat(&self::ml_string::Text::from_units(vec![41]))
                            } else {
                                if (target != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116]))) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    if (code == (6f64)) {
                                        ((self::ml_string::Text::from_units(vec![40, 126, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
                                    } else {
                                        {
                                            let symbol = if (code == (0f64)) {
                                                self::ml_string::Text::from_units(vec![38])
                                            } else {
                                                if (code == (1f64)) {
                                                    self::ml_string::Text::from_units(vec![124])
                                                } else {
                                                    if (code == (2f64)) {
                                                        self::ml_string::Text::from_units(vec![94])
                                                    } else {
                                                        if (code == (3f64)) {
                                                            self::ml_string::Text::from_units(vec![60, 60])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![62, 62])
                                                        }
                                                    }
                                                }
                                            };
                                            ((((((self::ml_string::Text::from_units(vec![40, 40])).concat(&left)).concat(&self::ml_string::Text::from_units(vec![41, 32]))).concat(&symbol)).concat(&self::ml_string::Text::from_units(vec![32, 40]))).concat(&right)).concat(&self::ml_string::Text::from_units(vec![41, 41]))
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

pub fn read_big_integer_bitwise_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 32, 98, 105, 116, 119, 105, 115, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 117, 115, 101, 32, 101, 120, 97, 99, 116, 32, 117, 110, 98, 111, 117, 110, 100, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115, 32, 119, 105, 116, 104, 32, 105, 110, 102, 105, 110, 105, 116, 101, 32, 115, 105, 103, 110, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 44, 32, 110, 101, 103, 97, 116, 105, 118, 101, 32, 115, 104, 105, 102, 116, 32, 114, 101, 118, 101, 114, 115, 97, 108, 32, 97, 110, 100, 32, 97, 114, 105, 116, 104, 109, 101, 116, 105, 99, 32, 114, 105, 103, 104, 116, 32, 115, 104, 105, 102, 116, 115, 59, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 114, 101, 32, 101, 118, 97, 108, 117, 97, 116, 101, 100, 32, 111, 110, 99, 101, 32, 105, 110, 32, 115, 111, 117, 114, 99, 101, 32, 111, 114, 100, 101, 114, 32, 119, 105, 116, 104, 111, 117, 116, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 101, 114, 99, 105, 111, 110, 32, 111, 114, 32, 109, 111, 100, 117, 108, 111, 45, 51, 50, 32, 115, 104, 105, 102, 116, 32, 99, 111, 117, 110, 116, 115, 46, 32, 78, 97, 116, 105, 118, 101, 32, 115, 111, 117, 114, 99, 101, 32, 97, 110, 100, 32, 116, 104, 101, 32, 101, 109, 105, 116, 116, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 117, 110, 116, 105, 109, 101, 32, 97, 114, 101, 32, 103, 101, 110, 101, 114, 97, 116, 101, 100, 32, 102, 114, 111, 109, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 99, 104, 101, 99, 107, 101, 100, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 46])
}
