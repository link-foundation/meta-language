pub fn execute_java_script_call_integer_conversion(operation: self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    {
        let error = self::read_java_script_integer_protocol_operation_error(operation.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::refuse_java_script_call_integer_conversion(self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]), self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))
                }
                self::JavaScriptSemanticValue::Null => {
                    self::refuse_java_script_call_integer_conversion(self::ml_string::Text::from_units(vec![110, 117, 108, 108]), self::ml_string::Text::from_units(vec![110, 117, 108, 108]))
                }
                self::JavaScriptSemanticValue::Boolean(value_value) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(self::convert_java_script_boolean_big_integer(value_value))))
                }
                self::JavaScriptSemanticValue::Number(value_value_2) => {
                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 78, 117, 109, 98, 101, 114, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116]))
                    } else {
                        {
                            let converted = self::convert_java_script_number_big_integer(value_value_2);
                            if (({ let ml_reference_array315 = converted.clone(); let ml_reference_projection315 = ml_reference_array315.borrow().len() as f64; ml_reference_projection315 }) != (1f64)) {
                                self::refuse_java_script_call_integer_conversion(self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_text(&self::ml_number::js_number(value_value_2)))
                            } else {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(({
                                    let source_ml_source2_array_read_index1980 = 0f64;
                                    { let ml_reference_array316 = converted.clone(); let ml_reference_position316 = source_ml_source2_array_read_index1980; let ml_reference_projection316 = self::ml_array::optional_at(&*ml_reference_array316.borrow(), self::ml_array::number_index(ml_reference_position316)); ml_reference_projection316 }
                                }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)))))
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
                }
                self::JavaScriptSemanticValue::Text(units) => {
                    if (({ let ml_reference_array317 = units.clone(); let ml_reference_projection317 = ml_reference_array317.borrow().len() as f64; ml_reference_projection317 }) > (4096f64)) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else {
                        {
                            let converted_2 = self::convert_java_script_string_big_integer_units(units.clone());
                            if (({ let ml_reference_array318 = converted_2.clone(); let ml_reference_projection318 = ml_reference_array318.borrow().len() as f64; ml_reference_projection318 }) != (1f64)) {
                                self::refuse_java_script_call_integer_conversion(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]), self::read_java_script_call_integer_text(units.clone()))
                            } else {
                                {
                                    let integer = ({
                                        let source_ml_source2_array_read_index1982 = 0f64;
                                        { let ml_reference_array319 = converted_2.clone(); let ml_reference_position319 = source_ml_source2_array_read_index1982; let ml_reference_projection319 = self::ml_array::optional_at(&*ml_reference_array319.borrow(), self::ml_array::number_index(ml_reference_position319)); ml_reference_projection319 }
                                    }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                    if ((self::dependency_function3(integer.clone())) < (0f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(integer.clone())))
                                    }
                                }
                            }
                        }
                    }
                }
                self::JavaScriptSemanticValue::SourceText(_, text) => {
                    if ((text.length()) > (4096f64)) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                    } else {
                        self::execute_java_script_call_integer_conversion(operation.clone(), self::JavaScriptSemanticValue::Text(self::construct_java_script_semantic_text_units(value.clone(), 4096f64)))
                    }
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116]))
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
                }
            }
        }
    }
}

pub fn read_java_script_call_scalar_conversion_error(operation: self::ml_string::Text) -> self::ml_string::Text {
    if (((((((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99])))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 110, 117, 109, 98, 101, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116])))) || ((self::read_java_script_integer_protocol_operation_error(operation.clone())) == (self::ml_string::Text::from_units(vec![])))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 78, 117, 109, 98, 101, 114, 44, 32, 105, 110, 116, 101, 103, 101, 114, 32, 111, 114, 32, 116, 101, 120, 116, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110])
    }
}

pub fn execute_java_script_call_scalar_conversion(operation: self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 110, 117, 109, 98, 101, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101])))) {
        if match value.clone() {
            self::JavaScriptSemanticValue::Object(_, _) => {
                true
            }
            _ => {
                false
            }
        } {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 45, 104, 105, 110, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
        } else {
            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
        }
    } else {
        if (((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99])))) {
            self::execute_java_script_call_number_conversion(operation.clone(), value.clone())
        } else {
            if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 116, 101, 103, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 101, 103, 101, 114])))) {
                self::execute_java_script_call_integer_conversion(operation.clone(), value.clone())
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                    self::dependency_function5(value.clone())
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::read_java_script_call_scalar_conversion_error(operation.clone()))
                }
            }
        }
    }
}

pub fn read_utf16_text_repetition_length(length: f64, count: f64) -> f64 {
    {
        let integer = if count.is_nan() {
            0f64
        } else {
            count.trunc()
        };
        if ((integer < (0f64)) || (integer == (f64::INFINITY))) {
            (-1f64)
        } else {
            if ((!(self::ml_math::is_safe_integer(length)) || (length < (0f64))) || (length > (1048576f64))) {
                (-2f64)
            } else {
                if ((length == (0f64)) || (integer == (0f64))) {
                    0f64
                } else {
                    if (integer > (((1048576f64 / length)).floor())) {
                        (-2f64)
                    } else {
                        (length * integer)
                    }
                }
            }
        }
    }
}

pub fn read_utf16_text_padding_length(length: f64, maximum: f64, filler_length: f64) -> f64 {
    if (((((!(self::ml_math::is_safe_integer(length)) || (length < (0f64))) || (length > (1048576f64))) || !(self::ml_math::is_safe_integer(filler_length))) || (filler_length < (0f64))) || (filler_length > (1048576f64))) {
        (-2f64)
    } else {
        {
            let integer = if maximum.is_nan() {
                0f64
            } else {
                maximum.trunc()
            };
            {
                let target = self::ml_math::min(9007199254740991f64, self::ml_math::max(0f64, integer));
                if ((target <= length) || (filler_length == (0f64))) {
                    length
                } else {
                    if (target > (1048576f64)) {
                        (-2f64)
                    } else {
                        target
                    }
                }
            }
        }
    }
}

pub fn construct_utf16_text_repetition(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, length: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((!(self::ml_math::is_safe_integer(length)) || (length < (0f64))) || (length > (1048576f64))) || ((length > (0f64)) && ((({ let ml_reference_array320 = text.clone(); let ml_reference_projection320 = ml_reference_array320.borrow().len() as f64; ml_reference_projection320 }) == (0f64)) || (((length % { let ml_reference_array321 = text.clone(); let ml_reference_projection321 = ml_reference_array321.borrow().len() as f64; ml_reference_projection321 })) != (0f64))))) {
        panic!("{}", "text repetition requires its complete admitted length")
    } else {
        {
            let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let index = 0f64;
                {
                    let result_2 = self::source_ml_source1_temporary_construct_utf16_text_repetition_loop144(text.clone(), length, result.clone(), index);
                    result_2.clone()
                }
            }
        }
    }
}

pub fn construct_utf16_text_padding(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, filler: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, length: f64, start: bool) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    if (((!(self::ml_math::is_safe_integer(length)) || (length < ({ let ml_reference_array322 = text.clone(); let ml_reference_projection322 = ml_reference_array322.borrow().len() as f64; ml_reference_projection322 }))) || (length > (1048576f64))) || ((length > ({ let ml_reference_array323 = text.clone(); let ml_reference_projection323 = ml_reference_array323.borrow().len() as f64; ml_reference_projection323 })) && (({ let ml_reference_array324 = filler.clone(); let ml_reference_projection324 = ml_reference_array324.borrow().len() as f64; ml_reference_projection324 }) == (0f64)))) {
        panic!("{}", "text padding requires its complete admitted length")
    } else {
        {
            let added = (length - { let ml_reference_array325 = text.clone(); let ml_reference_projection325 = ml_reference_array325.borrow().len() as f64; ml_reference_projection325 });
            {
                let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                {
                    let index = 0f64;
                    {
                        let result_2 = self::source_ml_source1_temporary_construct_utf16_text_padding_loop145(text.clone(), filler.clone(), length, start, added, result.clone(), index);
                        result_2.clone()
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_text_conversion(value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    self::dependency_function5(value.clone())
}

pub fn execute_java_script_call_numeric_primitive(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    if self::test_java_script_call_unary_number_operation(operation.clone()) {
        if (({ let ml_reference_array326 = values.clone(); let ml_reference_projection326 = ml_reference_array326.borrow().len() as f64; ml_reference_projection326 }) != (1f64)) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101]))
        } else {
            {
                let first = ({
                    let source_ml_source2_array_read_index1993 = 0f64;
                    { let ml_reference_array327 = values.clone(); let ml_reference_position327 = source_ml_source2_array_read_index1993; let ml_reference_projection327 = self::ml_array::optional_at(&*ml_reference_array327.borrow(), self::ml_array::number_index(ml_reference_position327)); ml_reference_projection327 }
                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                match first.clone() {
                    self::JavaScriptSemanticValue::Number(value) => {
                        self::execute_java_script_call_unary_number(operation.clone(), value)
                    }
                    _ => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                    }
                }
            }
        }
    } else {
        {
            let numbers = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let source_ml_source1_temporary_index146 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s2002 = self::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop147(numbers.clone(), values.clone(), source_ml_source1_temporary_index146.clone());
                    match source_ml_source2_s2002.clone() {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Done(numbers_2) => {
                            if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::min_of(&self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value328 = numbers_2.borrow().clone(); ml_reference_value328 })))))
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::max_of(&self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value329 = numbers_2.borrow().clone(); ml_reference_value329 })))))
                                } else {
                                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115]))) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_units_utf16_text(numbers_2.clone()))))
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop147Return(source_ml_source1_temporary_result147) => {
                            let source_ml_source1_temporary_result147 = *source_ml_source1_temporary_result147;
                            source_ml_source1_temporary_result147.clone()
                        }
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_unary_number(operation: self::ml_string::Text, number: f64) -> self::JavaScriptCallPrimitiveResult {
    if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 78, 97, 78, 32, 110, 117, 109, 98, 101, 114]))) {
        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(number.is_nan())))
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 102, 105, 110, 105, 116, 101, 32, 110, 117, 109, 98, 101, 114]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(number.is_finite())))
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]))) {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((-number))))
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![116, 114, 117, 110, 99, 97, 116, 101, 32, 110, 117, 109, 98, 101, 114]))) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(number.trunc())))
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]))) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ml_math::is_integer(number))))
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 110, 117, 109, 98, 101, 114]))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ml_math::is_safe_integer(number))))
                        } else {
                            if !(self::test_java_script_call_unary_number_operation(operation.clone())) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![117, 110, 97, 114, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                            } else {
                                {
                                    let result = if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 98, 115, 111, 108, 117, 116, 101, 32, 110, 117, 109, 98, 101, 114]))) {
                                        number.abs()
                                    } else {
                                        if (operation == (self::ml_string::Text::from_units(vec![102, 108, 111, 111, 114, 32, 110, 117, 109, 98, 101, 114]))) {
                                            number.floor()
                                        } else {
                                            if (operation == (self::ml_string::Text::from_units(vec![99, 101, 105, 108, 32, 110, 117, 109, 98, 101, 114]))) {
                                                number.ceil()
                                            } else {
                                                if (operation == (self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 110, 117, 109, 98, 101, 114]))) {
                                                    self::ml_math::round(number)
                                                } else {
                                                    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 98, 101, 114, 32, 115, 105, 103, 110]))) {
                                                        self::ml_math::sign(number)
                                                    } else {
                                                        number.sqrt()
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(result)))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_text_primitive(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    if { let ml_values330 = { let ml_reference_value331 = values.borrow().clone(); ml_reference_value331 }; let ml_callback330 = std::rc::Rc::new(move |value: self::JavaScriptSemanticValue| -> bool { match value.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => {
            true
        }
        _ => {
            false
        }
    } }); ml_values330.iter().cloned().enumerate().any(|(ml_index330, ml_value330)| ml_callback330(ml_value330)) } {
        self::execute_java_script_call_primitive(operation.clone(), values.clone())
    } else {
        if (((operation == (self::ml_string::Text::from_units(vec![114, 101, 112, 101, 97, 116, 32, 116, 101, 120, 116]))) || (operation == (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))) || (operation == (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 101, 110, 100])))) {
            self::execute_java_script_call_text_expansion(operation.clone(), values.clone())
        } else {
            if ((operation == (self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 116, 101, 120, 116]))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120, 116])))) {
                self::execute_java_script_call_text_replacement_operands(values.clone(), (operation == (self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120, 116]))))
            } else {
                {
                    let receiver = ({
                        let source_ml_source2_array_read_index2013 = 0f64;
                        { let ml_reference_array332 = values.clone(); let ml_reference_position332 = source_ml_source2_array_read_index2013; let ml_reference_projection332 = self::ml_array::optional_at(&*ml_reference_array332.borrow(), self::ml_array::number_index(ml_reference_position332)); ml_reference_projection332 }
                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Text(text) => {
                            if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number({ let ml_reference_array333 = text.clone(); let ml_reference_projection333 = ml_reference_array333.borrow().len() as f64; ml_reference_projection333 })))
                            } else {
                                if (((operation == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116]))) || (operation == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))) || (operation == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100])))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::trim_utf16_text(text.clone(), (operation != (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100]))), (operation != (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))))))
                                } else {
                                    if ((((operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116]))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])))) || (operation == (self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 116, 101, 120, 116])))) {
                                        self::execute_java_script_call_text_position(operation.clone(), text.clone(), values.clone())
                                    } else {
                                        self::execute_java_script_call_text_pair(operation.clone(), text.clone(), values.clone())
                                    }
                                }
                            }
                        }
                        _ => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                        }
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_text_expansion(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    if { let ml_values334 = { let ml_reference_value335 = values.borrow().clone(); ml_reference_value335 }; let ml_callback334 = std::rc::Rc::new(move |value: self::JavaScriptSemanticValue| -> bool { match value.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => {
            true
        }
        _ => {
            false
        }
    } }); ml_values334.iter().cloned().enumerate().any(|(ml_index334, ml_value334)| ml_callback334(ml_value334)) } {
        self::execute_java_script_call_primitive(operation.clone(), values.clone())
    } else {
        {
            let receiver = ({
                let source_ml_source2_array_read_index2032 = 0f64;
                { let ml_reference_array336 = values.clone(); let ml_reference_position336 = source_ml_source2_array_read_index2032; let ml_reference_projection336 = self::ml_array::optional_at(&*ml_reference_array336.borrow(), self::ml_array::number_index(ml_reference_position336)); ml_reference_projection336 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match receiver.clone() {
                self::JavaScriptSemanticValue::Text(receiver_units) => {
                    {
                        let count = ({
                            let source_ml_source2_array_read_index2042 = 1f64;
                            { let ml_reference_array337 = values.clone(); let ml_reference_position337 = source_ml_source2_array_read_index2042; let ml_reference_projection337 = self::ml_array::optional_at(&*ml_reference_array337.borrow(), self::ml_array::number_index(ml_reference_position337)); ml_reference_projection337 }
                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        match count.clone() {
                            self::JavaScriptSemanticValue::Number(value) => {
                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 112, 101, 97, 116, 32, 116, 101, 120, 116]))) {
                                    {
                                        let length = self::read_utf16_text_repetition_length({ let ml_reference_array338 = receiver_units.clone(); let ml_reference_projection338 = ml_reference_array338.borrow().len() as f64; ml_reference_projection338 }, value);
                                        if (length == ((-1f64))) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), (self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 99, 111, 117, 110, 116, 32, 118, 97, 108, 117, 101, 58, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(value))))
                                        } else {
                                            if (length < (0f64)) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 111, 117, 116, 112, 117, 116, 32, 98, 117, 100, 103, 101, 116]))
                                            } else {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::construct_utf16_text_repetition(receiver_units.clone(), length))))
                                            }
                                        }
                                    }
                                } else {
                                    if ((operation != (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116]))) && (operation != (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 101, 110, 100])))) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let filler = ({
                                                let source_ml_source2_array_read_index2053 = 2f64;
                                                { let ml_reference_array339 = values.clone(); let ml_reference_position339 = source_ml_source2_array_read_index2053; let ml_reference_projection339 = self::ml_array::optional_at(&*ml_reference_array339.borrow(), self::ml_array::number_index(ml_reference_position339)); ml_reference_projection339 }
                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                            match filler.clone() {
                                                self::JavaScriptSemanticValue::Text(units) => {
                                                    {
                                                        let length_2 = self::read_utf16_text_padding_length({ let ml_reference_array340 = receiver_units.clone(); let ml_reference_projection340 = ml_reference_array340.borrow().len() as f64; ml_reference_projection340 }, value, { let ml_reference_array341 = units.clone(); let ml_reference_projection341 = ml_reference_array341.borrow().len() as f64; ml_reference_projection341 });
                                                        if (length_2 < (0f64)) {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 111, 117, 116, 112, 117, 116, 32, 98, 117, 100, 103, 101, 116]))
                                                        } else {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::construct_utf16_text_padding(receiver_units.clone(), units.clone(), length_2, (operation == (self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])))))))
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 97, 100, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 108, 108, 101, 114]))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 98, 111, 117, 110, 100]))
                            }
                        }
                    }
                }
                _ => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                }
            }
        }
    }
}

pub fn execute_java_script_call_text_position(operation: self::ml_string::Text, text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    {
        let second = ({
            let source_ml_source2_array_read_index2063 = 1f64;
            { let ml_reference_array342 = values.clone(); let ml_reference_position342 = source_ml_source2_array_read_index2063; let ml_reference_projection342 = self::ml_array::optional_at(&*ml_reference_array342.borrow(), self::ml_array::number_index(ml_reference_position342)); ml_reference_projection342 }
        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let third = ({
                let source_ml_source2_array_read_index2065 = 2f64;
                { let ml_reference_array343 = values.clone(); let ml_reference_position343 = source_ml_source2_array_read_index2065; let ml_reference_projection343 = self::ml_array::optional_at(&*ml_reference_array343.borrow(), self::ml_array::number_index(ml_reference_position343)); ml_reference_projection343 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            if (((operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116]))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])))) {
                match second.clone() {
                    self::JavaScriptSemanticValue::Number(second_value) => {
                        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_utf16_text(text.clone(), second_value))))
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114]))) {
                                {
                                    let result = self::relative_character_utf16_text(text.clone(), second_value);
                                    {
                                        let value = {
                                            let source_ml_source2_array_read_index2075 = 0f64;
                                            { let ml_reference_array344 = result.clone(); let ml_reference_position344 = source_ml_source2_array_read_index2075; let ml_reference_projection344 = self::ml_array::optional_at(&*ml_reference_array344.borrow(), self::ml_array::number_index(ml_reference_position344)); ml_reference_projection344 }
                                        };
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::JavaScriptSemanticValue::Undefined
                                        } else {
                                            self::JavaScriptSemanticValue::Text(value.clone().expect("a proven present value"))
                                        }))
                                    }
                                }
                            } else {
                                {
                                    let result_2 = self::code_point_utf16_text(text.clone(), second_value);
                                    {
                                        let value_2 = {
                                            let source_ml_source2_array_read_index2077 = 0f64;
                                            { let ml_reference_array345 = result_2.clone(); let ml_reference_position345 = source_ml_source2_array_read_index2077; let ml_reference_projection345 = self::ml_array::optional_at(&*ml_reference_array345.borrow(), self::ml_array::number_index(ml_reference_position345)); ml_reference_projection345 }
                                        };
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(if {
                                            let source_ml_source2_comparison_operand2079 = None::<()>;
                                            match (value_2.clone(), source_ml_source2_comparison_operand2079.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                        } {
                                            self::JavaScriptSemanticValue::Undefined
                                        } else {
                                            self::JavaScriptSemanticValue::Number(value_2.clone().expect("a proven present value"))
                                        }))
                                    }
                                }
                            }
                        }
                    }
                    _ => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                    }
                }
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 116, 101, 120, 116]))) {
                    match second.clone() {
                        self::JavaScriptSemanticValue::Number(second_value_2) => {
                            match third.clone() {
                                self::JavaScriptSemanticValue::Number(value_3) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::slice_utf16_text(text.clone(), second_value_2, value_3))))
                                }
                                _ => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                            }
                        }
                        _ => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 108, 105, 99, 101, 32, 98, 111, 117, 110, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                    }
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 116, 101, 120, 116, 32, 112, 111, 115, 105, 116, 105, 111, 110, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                }
            }
        }
    }
}

pub fn execute_java_script_call_text_pair(operation: self::ml_string::Text, text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    {
        let second = ({
            let source_ml_source2_array_read_index2097 = 1f64;
            { let ml_reference_array346 = values.clone(); let ml_reference_position346 = source_ml_source2_array_read_index2097; let ml_reference_projection346 = self::ml_array::optional_at(&*ml_reference_array346.borrow(), self::ml_array::number_index(ml_reference_position346)); ml_reference_projection346 }
        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let third = ({
                let source_ml_source2_array_read_index2099 = 2f64;
                { let ml_reference_array347 = values.clone(); let ml_reference_position347 = source_ml_source2_array_read_index2099; let ml_reference_projection347 = self::ml_array::optional_at(&*ml_reference_array347.borrow(), self::ml_array::number_index(ml_reference_position347)); ml_reference_projection347 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match second.clone() {
                self::JavaScriptSemanticValue::Text(needle) => {
                    if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]))) {
                        if ((({ let ml_reference_array348 = text.clone(); let ml_reference_projection348 = ml_reference_array348.borrow().len() as f64; ml_reference_projection348 } + { let ml_reference_array349 = needle.clone(); let ml_reference_projection349 = ml_reference_array349.borrow().len() as f64; ml_reference_projection349 })) > (1048576f64)) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), { let ml_reference_value350 = text.borrow().clone(); ml_reference_value350 }), { let ml_reference_value351 = needle.borrow().clone(); ml_reference_value351 }))))))
                        }
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116]))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::compare_utf16_text(text.clone(), needle.clone()))))
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 112, 114, 101, 102, 105, 120]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::starts_utf16_text(text.clone(), needle.clone()))))
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 115, 117, 102, 102, 105, 120]))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::ends_utf16_text(text.clone(), needle.clone()))))
                                } else {
                                    if ((((self::ml_math::max(0f64, ({ let ml_reference_array352 = text.clone(); let ml_reference_projection352 = ml_reference_array352.borrow().len() as f64; ml_reference_projection352 } - { let ml_reference_array353 = needle.clone(); let ml_reference_projection353 = ml_reference_array353.borrow().len() as f64; ml_reference_projection353 })) + 1f64) * { let ml_reference_array354 = needle.clone(); let ml_reference_projection354 = ml_reference_array354.borrow().len() as f64; ml_reference_projection354 })) > (67108864f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 119, 111, 114, 107, 32, 113, 117, 111, 116, 97]))
                                    } else {
                                        if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 116, 101, 120, 116, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110]))) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean(self::includes_utf16_text(text.clone(), needle.clone()))))
                                        } else {
                                            if ((operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]))) || (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 108, 97, 115, 116, 32, 116, 101, 120, 116])))) {
                                                match third.clone() {
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (operation == (self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116]))) {
                                                            self::index_utf16_text(text.clone(), needle.clone(), value)
                                                        } else {
                                                            self::last_index_utf16_text(text.clone(), needle.clone(), value)
                                                        })))
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                }
                                            } else {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 116, 101, 120, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 115, 101, 97, 114, 99, 104, 32, 97, 110, 100, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100]))
                }
            }
        }
    }
}

pub fn execute_java_script_call_text_replacement_operands(values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, all: bool) -> self::JavaScriptCallPrimitiveResult {
    if { let ml_values355 = { let ml_reference_value356 = values.borrow().clone(); ml_reference_value356 }; let ml_callback355 = std::rc::Rc::new(move |value: self::JavaScriptSemanticValue| -> bool { match value.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => {
            true
        }
        _ => {
            false
        }
    } }); ml_values355.iter().cloned().enumerate().any(|(ml_index355, ml_value355)| ml_callback355(ml_value355)) } {
        self::execute_java_script_call_primitive(if all {
            self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 97, 108, 108, 32, 116, 101, 120, 116])
        } else {
            self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 32, 116, 101, 120, 116])
        }, values.clone())
    } else {
        {
            let text = ({
                let source_ml_source2_array_read_index2128 = 0f64;
                { let ml_reference_array357 = values.clone(); let ml_reference_position357 = source_ml_source2_array_read_index2128; let ml_reference_projection357 = self::ml_array::optional_at(&*ml_reference_array357.borrow(), self::ml_array::number_index(ml_reference_position357)); ml_reference_projection357 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            {
                let search = ({
                    let source_ml_source2_array_read_index2130 = 1f64;
                    { let ml_reference_array358 = values.clone(); let ml_reference_position358 = source_ml_source2_array_read_index2130; let ml_reference_projection358 = self::ml_array::optional_at(&*ml_reference_array358.borrow(), self::ml_array::number_index(ml_reference_position358)); ml_reference_projection358 }
                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let replacement = ({
                        let source_ml_source2_array_read_index2132 = 2f64;
                        { let ml_reference_array359 = values.clone(); let ml_reference_position359 = source_ml_source2_array_read_index2132; let ml_reference_projection359 = self::ml_array::optional_at(&*ml_reference_array359.borrow(), self::ml_array::number_index(ml_reference_position359)); ml_reference_projection359 }
                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    match text.clone() {
                        self::JavaScriptSemanticValue::Text(text_units) => {
                            match search.clone() {
                                self::JavaScriptSemanticValue::Text(search_units) => {
                                    match replacement.clone() {
                                        self::JavaScriptSemanticValue::Text(units) => {
                                            self::execute_java_script_call_text_replacement(text_units.clone(), search_units.clone(), units.clone(), all)
                                        }
                                        _ => {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                        }
                                    }
                                }
                                _ => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                            }
                        }
                        _ => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 116, 104, 114, 101, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_call_text_replacement_work_error(length: f64, search: f64, replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, all: bool) -> self::ml_string::Text {
    if (((length > (1048576f64)) || (search > (1048576f64))) || (({ let ml_reference_array360 = replacement.clone(); let ml_reference_projection360 = ml_reference_array360.borrow().len() as f64; ml_reference_projection360 }) > (4096f64))) {
        self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 112, 117, 116, 32, 111, 114, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 105, 111, 110, 45, 112, 108, 97, 110, 32, 98, 117, 100, 103, 101, 116])
    } else {
        {
            let matches = if (search == (0f64)) {
                (length + 1f64)
            } else {
                ((length / search)).floor()
            };
            {
                let matches_3 = if !all {
                    {
                        let matches_2 = self::ml_math::min(1f64, matches);
                        matches_2
                    }
                } else {
                    matches
                };
                if (matches_3 == (0f64)) {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    if ((((length + 1f64) * search)) > (67108864f64)) {
                        self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 101, 97, 114, 99, 104, 32, 119, 111, 114, 107, 32, 113, 117, 111, 116, 97])
                    } else {
                        {
                            let plan = self::plan_utf16_text_substitution(replacement.clone());
                            if (((matches_3 * ({ let ml_reference_array361 = plan.clone(); let ml_reference_projection361 = ml_reference_array361.borrow().len() as f64; ml_reference_projection361 } / 3f64))) > (67108864f64)) {
                                self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 105, 111, 110, 32, 119, 111, 114, 107, 32, 113, 117, 111, 116, 97])
                            } else {
                                {
                                    let width = 0f64;
                                    {
                                        let index = 0f64;
                                        {
                                            let width_2 = self::source_ml_source1_temporary_read_java_script_call_text_replacement_work_error_loop148(length, search, plan.clone(), width, index);
                                            if (((length + (matches_3 * width_2))) > (1048576f64)) {
                                                self::ml_string::Text::from_units(vec![116, 101, 120, 116, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 110, 115, 101, 114, 118, 97, 116, 105, 118, 101, 32, 85, 84, 70, 49, 54, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 98, 111, 117, 110, 100])
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
    }
}

pub fn execute_java_script_call_text_replacement(text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, replacement: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, all: bool) -> self::JavaScriptCallPrimitiveResult {
    {
        let error = self::read_java_script_call_text_replacement_work_error({ let ml_reference_array362 = text.clone(); let ml_reference_projection362 = ml_reference_array362.borrow().len() as f64; ml_reference_projection362 }, { let ml_reference_array363 = search.clone(); let ml_reference_projection363 = ml_reference_array363.borrow().len() as f64; ml_reference_projection363 }, replacement.clone(), all);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(error.clone())
        } else {
            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::replace_utf16_text(text.clone(), search.clone(), replacement.clone(), all))))
        }
    }
}

pub fn construct_java_script_call_primitive_type_error(reason: self::ml_string::Text) -> self::JavaScriptCallPrimitiveResult {
    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), reason.clone())
}
