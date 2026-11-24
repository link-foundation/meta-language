pub fn execute_java_script_call_primitive(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    {
        let arity = self::read_java_script_call_primitive_arity_error(operation.clone(), { let ml_reference_array364 = values.clone(); let ml_reference_projection364 = ml_reference_array364.borrow().len() as f64; ml_reference_projection364 });
        if (arity != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(arity.clone())
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119, 32, 114, 101, 115, 116, 114, 105, 99, 116, 101, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 97, 99, 99, 101, 115, 115]))) {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![39, 99, 97, 108, 108, 101, 114, 39, 44, 32, 39, 99, 97, 108, 108, 101, 101, 39, 44, 32, 97, 110, 100, 32, 39, 97, 114, 103, 117, 109, 101, 110, 116, 115, 39, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 109, 97, 121, 32, 110, 111, 116, 32, 98, 101, 32, 97, 99, 99, 101, 115, 115, 101, 100, 32, 111, 110, 32, 115, 116, 114, 105, 99, 116, 32, 109, 111, 100, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 111, 114, 32, 116, 104, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 111, 98, 106, 101, 99, 116, 115, 32, 102, 111, 114, 32, 99, 97, 108, 108, 115, 32, 116, 111, 32, 116, 104, 101, 109]))
            } else {
                {
                    let source_ml_source1_temporary_index149 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s2157 = self::source_ml_source1_temporary_execute_java_script_call_primitive_loop150(values.clone(), source_ml_source1_temporary_index149.clone());
                        match source_ml_source2_s2157.clone() {
                            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Done => {
                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104]))) {
                                    {
                                        let value = ({
                                            let source_ml_source2_array_read_index2159 = 0f64;
                                            { let ml_reference_array365 = values.clone(); let ml_reference_position365 = source_ml_source2_array_read_index2159; let ml_reference_projection365 = self::ml_array::optional_at(&*ml_reference_array365.borrow(), self::ml_array::number_index(ml_reference_position365)); ml_reference_projection365 }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                        match value.clone() {
                                            self::JavaScriptSemanticValue::Number(value_value) => {
                                                if ((!(self::ml_math::is_safe_integer(value_value)) || (value_value < (0f64))) || {
                                                    let source_ml_source2_same_value_operand2169 = (-0f64);
                                                    (((value_value == source_ml_source2_same_value_operand2169) && ((value_value != (0f64)) || (((1f64 / value_value)) == ((1f64 / source_ml_source2_same_value_operand2169))))) || ((value_value != value_value) && (source_ml_source2_same_value_operand2169 != source_ml_source2_same_value_operand2169)))
                                                }) {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101, 32, 108, 101, 110, 103, 116, 104, 32, 102, 111, 108, 108, 111, 119, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104]))
                                                } else {
                                                    if (value_value > (4294967295f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]))
                                                    } else {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101, 32, 108, 101, 110, 103, 116, 104, 32, 102, 111, 108, 108, 111, 119, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104]))
                                            }
                                        }
                                    }
                                } else {
                                    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 99, 104, 101, 99, 107, 101, 100, 32, 97, 114, 114, 97, 121, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 105, 110, 100, 101, 120]))) {
                                        {
                                            let length = ({
                                                let source_ml_source2_array_read_index2171 = 0f64;
                                                { let ml_reference_array366 = values.clone(); let ml_reference_position366 = source_ml_source2_array_read_index2171; let ml_reference_projection366 = self::ml_array::optional_at(&*ml_reference_array366.borrow(), self::ml_array::number_index(ml_reference_position366)); ml_reference_projection366 }
                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                            {
                                                let value_2 = ({
                                                    let source_ml_source2_array_read_index2173 = 1f64;
                                                    { let ml_reference_array367 = values.clone(); let ml_reference_position367 = source_ml_source2_array_read_index2173; let ml_reference_projection367 = self::ml_array::optional_at(&*ml_reference_array367.borrow(), self::ml_array::number_index(ml_reference_position367)); ml_reference_projection367 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                match length.clone() {
                                                    self::JavaScriptSemanticValue::Number(length_value) => {
                                                        if ((!(self::ml_math::is_safe_integer(length_value)) || (length_value < (0f64))) || {
                                                            let source_ml_source2_same_value_operand2183 = (-0f64);
                                                            (((length_value == source_ml_source2_same_value_operand2183) && ((length_value != (0f64)) || (((1f64 / length_value)) == ((1f64 / source_ml_source2_same_value_operand2183))))) || ((length_value != length_value) && (source_ml_source2_same_value_operand2183 != source_ml_source2_same_value_operand2183)))
                                                        }) {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 105, 110, 100, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                        } else {
                                                            match value_2.clone() {
                                                                self::JavaScriptSemanticValue::Number(value_value_2) => {
                                                                    {
                                                                        let integer = if (value_value_2.is_nan() || (value_value_2 == (0f64))) {
                                                                            0f64
                                                                        } else {
                                                                            value_value_2.trunc()
                                                                        };
                                                                        {
                                                                            let index = if (integer < (0f64)) {
                                                                                (length_value + integer)
                                                                            } else {
                                                                                integer
                                                                            };
                                                                            if ((index < (0f64)) || (index >= length_value)) {
                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 105, 110, 100, 101, 120]))
                                                                            } else {
                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (index == (0f64)) {
                                                                                    0f64
                                                                                } else {
                                                                                    index
                                                                                })))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 105, 110, 100, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 105, 110, 100, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 99, 111, 117, 110, 116]))) {
                                            {
                                                let maximum = ({
                                                    let source_ml_source2_array_read_index2193 = 0f64;
                                                    { let ml_reference_array368 = values.clone(); let ml_reference_position368 = source_ml_source2_array_read_index2193; let ml_reference_projection368 = self::ml_array::optional_at(&*ml_reference_array368.borrow(), self::ml_array::number_index(ml_reference_position368)); ml_reference_projection368 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                {
                                                    let value_3 = ({
                                                        let source_ml_source2_array_read_index2195 = 1f64;
                                                        { let ml_reference_array369 = values.clone(); let ml_reference_position369 = source_ml_source2_array_read_index2195; let ml_reference_projection369 = self::ml_array::optional_at(&*ml_reference_array369.borrow(), self::ml_array::number_index(ml_reference_position369)); ml_reference_projection369 }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    match maximum.clone() {
                                                        self::JavaScriptSemanticValue::Number(maximum_value) => {
                                                            if ((!(self::ml_math::is_safe_integer(maximum_value)) || (maximum_value < (0f64))) || {
                                                                let source_ml_source2_same_value_operand2205 = (-0f64);
                                                                (((maximum_value == source_ml_source2_same_value_operand2205) && ((maximum_value != (0f64)) || (((1f64 / maximum_value)) == ((1f64 / source_ml_source2_same_value_operand2205))))) || ((maximum_value != maximum_value) && (source_ml_source2_same_value_operand2205 != source_ml_source2_same_value_operand2205)))
                                                            }) {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 99, 111, 117, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 114, 101, 109, 97, 105, 110, 105, 110, 103, 32, 108, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                            } else {
                                                                match value_3.clone() {
                                                                    self::JavaScriptSemanticValue::Number(value_value_3) => {
                                                                        {
                                                                            let integer_2 = if (value_value_3.is_nan() || (value_value_3 == (0f64))) {
                                                                                0f64
                                                                            } else {
                                                                                value_value_3.trunc()
                                                                            };
                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::min(self::ml_math::max(integer_2, 0f64), maximum_value))))
                                                                        }
                                                                    }
                                                                    _ => {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 99, 111, 117, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 114, 101, 109, 97, 105, 110, 105, 110, 103, 32, 108, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 99, 111, 117, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 114, 101, 109, 97, 105, 110, 105, 110, 103, 32, 108, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                                                self::construct_java_script_symbol_descriptive_text(({
                                                    let source_ml_source2_array_read_index2215 = 0f64;
                                                    { let ml_reference_array370 = values.clone(); let ml_reference_position370 = source_ml_source2_array_read_index2215; let ml_reference_projection370 = self::ml_array::optional_at(&*ml_reference_array370.borrow(), self::ml_array::number_index(ml_reference_position370)); ml_reference_projection370 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                            } else {
                                                if ((self::read_java_script_numeric_parsing_arity(operation.clone())) >= (0f64)) {
                                                    self::execute_java_script_numeric_parsing(operation.clone(), values.clone())
                                                } else {
                                                    if (operation == (self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 98, 105, 110, 97, 114, 121, 51, 50, 32, 110, 117, 109, 98, 101, 114]))) {
                                                        {
                                                            let value_4 = ({
                                                                let source_ml_source2_array_read_index2217 = 0f64;
                                                                { let ml_reference_array371 = values.clone(); let ml_reference_position371 = source_ml_source2_array_read_index2217; let ml_reference_projection371 = self::ml_array::optional_at(&*ml_reference_array371.borrow(), self::ml_array::number_index(ml_reference_position371)); ml_reference_projection371 }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                            match value_4.clone() {
                                                                self::JavaScriptSemanticValue::Number(value_value_4) => {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::round_java_script_binary32(value_value_4))))
                                                                }
                                                                _ => {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121, 51, 50, 32, 114, 111, 117, 110, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if (operation == (self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116, 32, 108, 101, 97, 100, 105, 110, 103, 32, 122, 101, 114, 111, 32, 98, 105, 116, 115]))) {
                                                            {
                                                                let value_5 = ({
                                                                    let source_ml_source2_array_read_index2227 = 0f64;
                                                                    { let ml_reference_array372 = values.clone(); let ml_reference_position372 = source_ml_source2_array_read_index2227; let ml_reference_projection372 = self::ml_array::optional_at(&*ml_reference_array372.borrow(), self::ml_array::number_index(ml_reference_position372)); ml_reference_projection372 }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                match value_5.clone() {
                                                                    self::JavaScriptSemanticValue::Number(value_value_5) => {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::count_java_script_leading_zero_bits(value_value_5))))
                                                                    }
                                                                    _ => {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![108, 101, 97, 100, 105, 110, 103, 32, 122, 101, 114, 111, 32, 99, 111, 117, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if (operation == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) {
                                                                {
                                                                    let left = ({
                                                                        let source_ml_source2_array_read_index2237 = 0f64;
                                                                        { let ml_reference_array373 = values.clone(); let ml_reference_position373 = source_ml_source2_array_read_index2237; let ml_reference_projection373 = self::ml_array::optional_at(&*ml_reference_array373.borrow(), self::ml_array::number_index(ml_reference_position373)); ml_reference_projection373 }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    {
                                                                        let right = ({
                                                                            let source_ml_source2_array_read_index2239 = 1f64;
                                                                            { let ml_reference_array374 = values.clone(); let ml_reference_position374 = source_ml_source2_array_read_index2239; let ml_reference_projection374 = self::ml_array::optional_at(&*ml_reference_array374.borrow(), self::ml_array::number_index(ml_reference_position374)); ml_reference_projection374 }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                        match left.clone() {
                                                                            self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                match right.clone() {
                                                                                    self::JavaScriptSemanticValue::Number(value_6) => {
                                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::multiply_java_script_signed_integer_bits(left_value, value_6))))
                                                                                    }
                                                                                    _ => {
                                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => {
                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if ((self::read_java_script_string_method_primitive_arity(operation.clone())) >= (0f64)) {
                                                                    self::execute_java_script_string_method_primitive(operation.clone(), values.clone())
                                                                } else {
                                                                    if (((operation == (self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]))) || (operation == (self::ml_string::Text::from_units(vec![100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101])))) || (operation == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101])))) {
                                                                        self::execute_java_script_call_numeric_update(operation.clone(), ({
                                                                            let source_ml_source2_array_read_index2257 = 0f64;
                                                                            { let ml_reference_array375 = values.clone(); let ml_reference_position375 = source_ml_source2_array_read_index2257; let ml_reference_projection375 = self::ml_array::optional_at(&*ml_reference_array375.borrow(), self::ml_array::number_index(ml_reference_position375)); ml_reference_projection375 }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                    } else {
                                                                        if self::ml_array::includes(&{ let ml_reference_value376 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 111, 119, 101, 114])]))).borrow().clone(); ml_reference_value376 }, &operation.clone(), 0f64, false, |a, b| a == b) {
                                                                            self::execute_java_script_call_numeric_binary(operation.clone(), ({
                                                                                let source_ml_source2_array_read_index2259 = 0f64;
                                                                                { let ml_reference_array377 = values.clone(); let ml_reference_position377 = source_ml_source2_array_read_index2259; let ml_reference_projection377 = self::ml_array::optional_at(&*ml_reference_array377.borrow(), self::ml_array::number_index(ml_reference_position377)); ml_reference_projection377 }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                                                let source_ml_source2_array_read_index2261 = 1f64;
                                                                                { let ml_reference_array378 = values.clone(); let ml_reference_position378 = source_ml_source2_array_read_index2261; let ml_reference_projection378 = self::ml_array::optional_at(&*ml_reference_array378.borrow(), self::ml_array::number_index(ml_reference_position378)); ml_reference_projection378 }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                        } else {
                                                                            if (operation == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 118, 97, 108, 117, 101, 115]))) {
                                                                                self::execute_java_script_call_relational_comparison(({
                                                                                    let source_ml_source2_array_read_index2263 = 0f64;
                                                                                    { let ml_reference_array379 = values.clone(); let ml_reference_position379 = source_ml_source2_array_read_index2263; let ml_reference_projection379 = self::ml_array::optional_at(&*ml_reference_array379.borrow(), self::ml_array::number_index(ml_reference_position379)); ml_reference_projection379 }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                                                    let source_ml_source2_array_read_index2265 = 1f64;
                                                                                    { let ml_reference_array380 = values.clone(); let ml_reference_position380 = source_ml_source2_array_read_index2265; let ml_reference_projection380 = self::ml_array::optional_at(&*ml_reference_array380.borrow(), self::ml_array::number_index(ml_reference_position380)); ml_reference_projection380 }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                            } else {
                                                                                if (operation == (self::ml_string::Text::from_units(vec![97, 100, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101, 115]))) {
                                                                                    self::execute_java_script_call_primitive_addition(({
                                                                                        let source_ml_source2_array_read_index2267 = 0f64;
                                                                                        { let ml_reference_array381 = values.clone(); let ml_reference_position381 = source_ml_source2_array_read_index2267; let ml_reference_projection381 = self::ml_array::optional_at(&*ml_reference_array381.borrow(), self::ml_array::number_index(ml_reference_position381)); ml_reference_projection381 }
                                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                                                        let source_ml_source2_array_read_index2269 = 1f64;
                                                                                        { let ml_reference_array382 = values.clone(); let ml_reference_position382 = source_ml_source2_array_read_index2269; let ml_reference_projection382 = self::ml_array::optional_at(&*ml_reference_array382.borrow(), self::ml_array::number_index(ml_reference_position382)); ml_reference_projection382 }
                                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                                } else {
                                                                                    self::execute_java_script_call_primitive_object_or_scalar(operation.clone(), values.clone())
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
                            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop150Return(source_ml_source1_temporary_result150) => {
                                let source_ml_source1_temporary_result150 = *source_ml_source1_temporary_result150;
                                source_ml_source1_temporary_result150.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_primitive_object_or_scalar(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    if ((operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32, 109, 101, 116, 104, 111, 100, 32, 111, 98, 106, 101, 99, 116]))) || (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 98, 106, 101, 99, 116])))) {
        {
            let value = ({
                let source_ml_source2_array_read_index2271 = 0f64;
                { let ml_reference_array383 = values.clone(); let ml_reference_position383 = source_ml_source2_array_read_index2271; let ml_reference_projection383 = self::ml_array::optional_at(&*ml_reference_array383.borrow(), self::ml_array::number_index(ml_reference_position383)); ml_reference_projection383 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            if match value.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => {
                    true
                }
                _ => {
                    false
                }
            } {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
            } else {
                if (match value.clone() {
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    _ => {
                        false
                    }
                } || match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    _ => {
                        false
                    }
                }) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 111, 114, 32, 110, 117, 108, 108, 32, 116, 111, 32, 111, 98, 106, 101, 99, 116]))
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32, 109, 101, 116, 104, 111, 100, 32, 111, 98, 106, 101, 99, 116]))) {
                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 109, 101, 116, 104, 111, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 98, 111, 120, 105, 110, 103, 32, 112, 114, 111, 116, 111, 99, 111, 108])
                    } else {
                        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 98, 111, 120, 105, 110, 103, 32, 112, 114, 111, 116, 111, 99, 111, 108])
                    })
                }
            }
        }
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 86, 56, 32, 97, 114, 114, 97, 121, 32, 106, 111, 105, 110, 32, 108, 101, 110, 103, 116, 104]))) {
            {
                let value_2 = ({
                    let source_ml_source2_array_read_index2300 = 0f64;
                    { let ml_reference_array384 = values.clone(); let ml_reference_position384 = source_ml_source2_array_read_index2300; let ml_reference_projection384 = self::ml_array::optional_at(&*ml_reference_array384.borrow(), self::ml_array::number_index(ml_reference_position384)); ml_reference_projection384 }
                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                match value_2.clone() {
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        if ((!(self::ml_math::is_safe_integer(value_value)) || (value_value < (0f64))) || {
                            let source_ml_source2_same_value_operand2310 = (-0f64);
                            (((value_value == source_ml_source2_same_value_operand2310) && ((value_value != (0f64)) || (((1f64 / value_value)) == ((1f64 / source_ml_source2_same_value_operand2310))))) || ((value_value != value_value) && (source_ml_source2_same_value_operand2310 != source_ml_source2_same_value_operand2310)))
                        }) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![86, 56, 32, 106, 111, 105, 110, 32, 108, 101, 110, 103, 116, 104, 32, 102, 111, 108, 108, 111, 119, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104]))
                        } else {
                            if (value_value > (4294967295f64)) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]))
                            } else {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value_2.clone()))
                            }
                        }
                    }
                    _ => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![86, 56, 32, 106, 111, 105, 110, 32, 108, 101, 110, 103, 116, 104, 32, 102, 111, 108, 108, 111, 119, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104]))
                    }
                }
            }
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116]))) {
                {
                    let value_3 = ({
                        let source_ml_source2_array_read_index2312 = 0f64;
                        { let ml_reference_array385 = values.clone(); let ml_reference_position385 = source_ml_source2_array_read_index2312; let ml_reference_projection385 = self::ml_array::optional_at(&*ml_reference_array385.borrow(), self::ml_array::number_index(ml_reference_position385)); ml_reference_projection385 }
                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    {
                        let invalid = self::construct_java_script_call_primitive_type_error(self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 32, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116]));
                        match value_3.clone() {
                            self::JavaScriptSemanticValue::Object(_, callable) => {
                                if callable {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value_3.clone()))
                                } else {
                                    invalid.clone()
                                }
                            }
                            _ => {
                                invalid.clone()
                            }
                        }
                    }
                }
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 79, 98, 106, 101, 99, 116]))) {
                    {
                        let value_4 = ({
                            let source_ml_source2_array_read_index2322 = 0f64;
                            { let ml_reference_array386 = values.clone(); let ml_reference_position386 = source_ml_source2_array_read_index2322; let ml_reference_projection386 = self::ml_array::optional_at(&*ml_reference_array386.borrow(), self::ml_array::number_index(ml_reference_position386)); ml_reference_projection386 }
                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        if match value_4.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                true
                            }
                            _ => {
                                false
                            }
                        } {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value_4.clone()))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 114, 101, 97, 116, 101, 76, 105, 115, 116, 70, 114, 111, 109, 65, 114, 114, 97, 121, 76, 105, 107, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 108, 105, 115, 116]))
                        }
                    }
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 79, 98, 106, 101, 99, 116, 32, 97, 115, 115, 105, 103, 110, 32, 116, 97, 114, 103, 101, 116]))) {
                        {
                            let value_5 = ({
                                let source_ml_source2_array_read_index2333 = 0f64;
                                { let ml_reference_array387 = values.clone(); let ml_reference_position387 = source_ml_source2_array_read_index2333; let ml_reference_projection387 = self::ml_array::optional_at(&*ml_reference_array387.borrow(), self::ml_array::number_index(ml_reference_position387)); ml_reference_projection387 }
                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                            if match value_5.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    true
                                }
                                _ => {
                                    false
                                }
                            } {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value_5.clone()))
                            } else {
                                if (match value_5.clone() {
                                    self::JavaScriptSemanticValue::Null => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                } || match value_5.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                }) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 111, 114, 32, 110, 117, 108, 108, 32, 116, 111, 32, 111, 98, 106, 101, 99, 116]))
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 97, 114, 103, 101, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 119, 114, 97, 112, 112, 101, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                                }
                            }
                        }
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116]))) {
                            {
                                let value_6 = ({
                                    let source_ml_source2_array_read_index2362 = 0f64;
                                    { let ml_reference_array388 = values.clone(); let ml_reference_position388 = source_ml_source2_array_read_index2362; let ml_reference_projection388 = self::ml_array::optional_at(&*ml_reference_array388.borrow(), self::ml_array::number_index(ml_reference_position388)); ml_reference_projection388 }
                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                if match value_6.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        true
                                    }
                                    _ => {
                                        false
                                    }
                                } {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value_6.clone()))
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                                }
                            }
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 102, 111, 114, 32, 105, 110]))) {
                                {
                                    let value_7 = ({
                                        let source_ml_source2_array_read_index2373 = 0f64;
                                        { let ml_reference_array389 = values.clone(); let ml_reference_position389 = source_ml_source2_array_read_index2373; let ml_reference_projection389 = self::ml_array::optional_at(&*ml_reference_array389.borrow(), self::ml_array::number_index(ml_reference_position389)); ml_reference_projection389 }
                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                    if match value_7.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                            true
                                        }
                                        _ => {
                                            false
                                        }
                                    } {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value_7.clone()))
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![84, 104, 101, 32, 114, 105, 103, 104, 116, 32, 111, 112, 101, 114, 97, 110, 100, 32, 111, 102, 32, 105, 110, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                                    }
                                }
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 110, 117, 108, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 110, 112, 117, 116]))) {
                                    {
                                        let value_8 = ({
                                            let source_ml_source2_array_read_index2384 = 0f64;
                                            { let ml_reference_array390 = values.clone(); let ml_reference_position390 = source_ml_source2_array_read_index2384; let ml_reference_projection390 = self::ml_array::optional_at(&*ml_reference_array390.borrow(), self::ml_array::number_index(ml_reference_position390)); ml_reference_projection390 }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                        if (match value_8.clone() {
                                            self::JavaScriptSemanticValue::Null => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        } || match value_8.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        }) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 111, 114, 32, 110, 117, 108, 108, 32, 116, 111, 32, 111, 98, 106, 101, 99, 116]))
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value_8.clone()))
                                        }
                                    }
                                } else {
                                    self::execute_java_script_call_primitive_scalar(operation.clone(), values.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
