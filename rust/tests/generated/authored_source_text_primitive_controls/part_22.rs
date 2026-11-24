pub fn execute_java_script_call_primitive_scalar(operation: self::ml_string::Text, values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>) -> self::JavaScriptCallPrimitiveResult {
    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 105, 110, 100, 101, 120]))) {
        {
            let value = ({
                let source_ml_source2_array_read_index2404 = 0f64;
                { let ml_reference_array391 = values.clone(); let ml_reference_position391 = source_ml_source2_array_read_index2404; let ml_reference_projection391 = self::ml_array::optional_at(&*ml_reference_array391.borrow(), self::ml_array::number_index(ml_reference_position391)); ml_reference_projection391 }
            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match value.clone() {
                self::JavaScriptSemanticValue::Number(value_value) => {
                    {
                        let index = if (value_value.is_nan() || (value_value == (0f64))) {
                            0f64
                        } else {
                            value_value.trunc()
                        };
                        if ((index < (0f64)) || (index > (9007199254740991f64))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![73, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 98, 101, 32, 98, 101, 116, 119, 101, 101, 110, 32, 48, 32, 97, 110, 100, 32, 57, 48, 48, 55, 49, 57, 57, 50, 53, 52, 55, 52, 48, 57, 57, 49]))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (index == (0f64)) {
                                0f64
                            } else {
                                index
                            })))
                        }
                    }
                }
                _ => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![84, 111, 73, 110, 100, 101, 120, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 118, 97, 108, 117, 101]))
                }
            }
        }
    } else {
        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 108, 101, 110, 103, 116, 104]))) {
            {
                let value_2 = ({
                    let source_ml_source2_array_read_index2414 = 0f64;
                    { let ml_reference_array392 = values.clone(); let ml_reference_position392 = source_ml_source2_array_read_index2414; let ml_reference_projection392 = self::ml_array::optional_at(&*ml_reference_array392.borrow(), self::ml_array::number_index(ml_reference_position392)); ml_reference_projection392 }
                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                match value_2.clone() {
                    self::JavaScriptSemanticValue::Number(value_value_2) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::read_java_script_array_like_length(value_value_2))))
                    }
                    _ => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![84, 111, 76, 101, 110, 103, 116, 104, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 118, 97, 108, 117, 101]))
                    }
                }
            }
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 97, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120]))) {
                {
                    let length = ({
                        let source_ml_source2_array_read_index2424 = 0f64;
                        { let ml_reference_array393 = values.clone(); let ml_reference_position393 = source_ml_source2_array_read_index2424; let ml_reference_projection393 = self::ml_array::optional_at(&*ml_reference_array393.borrow(), self::ml_array::number_index(ml_reference_position393)); ml_reference_projection393 }
                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    {
                        let number = ({
                            let source_ml_source2_array_read_index2426 = 1f64;
                            { let ml_reference_array394 = values.clone(); let ml_reference_position394 = source_ml_source2_array_read_index2426; let ml_reference_projection394 = self::ml_array::optional_at(&*ml_reference_array394.borrow(), self::ml_array::number_index(ml_reference_position394)); ml_reference_projection394 }
                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        match length.clone() {
                            self::JavaScriptSemanticValue::Number(length_value) => {
                                if (!(self::ml_math::is_safe_integer(length_value)) || (length_value < (0f64))) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 108, 97, 116, 105, 118, 101, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                } else {
                                    match number.clone() {
                                        self::JavaScriptSemanticValue::Number(value_3) => {
                                            {
                                                let integer = if (value_3.is_nan() || (value_3 == (0f64))) {
                                                    0f64
                                                } else {
                                                    value_3.trunc()
                                                };
                                                {
                                                    let index_2 = if (integer == (f64::NEG_INFINITY)) {
                                                        0f64
                                                    } else {
                                                        if (integer < (0f64)) {
                                                            self::ml_math::max((length_value + integer), 0f64)
                                                        } else {
                                                            self::ml_math::min(integer, length_value)
                                                        }
                                                    };
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(index_2)))
                                                }
                                            }
                                        }
                                        _ => {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 108, 97, 116, 105, 118, 101, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                        }
                                    }
                                }
                            }
                            _ => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 108, 97, 116, 105, 118, 101, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                            }
                        }
                    }
                }
            } else {
                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 99, 104, 101, 99, 107, 101, 100, 32, 97, 114, 114, 97, 121, 32, 97, 112, 112, 101, 110, 100, 32, 108, 101, 110, 103, 116, 104]))) {
                    {
                        let length_2 = ({
                            let source_ml_source2_array_read_index2444 = 0f64;
                            { let ml_reference_array395 = values.clone(); let ml_reference_position395 = source_ml_source2_array_read_index2444; let ml_reference_projection395 = self::ml_array::optional_at(&*ml_reference_array395.borrow(), self::ml_array::number_index(ml_reference_position395)); ml_reference_projection395 }
                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let count = ({
                                let source_ml_source2_array_read_index2446 = 1f64;
                                { let ml_reference_array396 = values.clone(); let ml_reference_position396 = source_ml_source2_array_read_index2446; let ml_reference_projection396 = self::ml_array::optional_at(&*ml_reference_array396.borrow(), self::ml_array::number_index(ml_reference_position396)); ml_reference_projection396 }
                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                            match length_2.clone() {
                                self::JavaScriptSemanticValue::Number(length_value_2) => {
                                    match count.clone() {
                                        self::JavaScriptSemanticValue::Number(value_4) => {
                                            if (((!(self::ml_math::is_safe_integer(length_value_2)) || (length_value_2 < (0f64))) || !(self::ml_math::is_safe_integer(value_4))) || (value_4 < (0f64))) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 112, 112, 101, 110, 100, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 99, 111, 117, 110, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                            } else {
                                                if (length_value_2 > ((9007199254740991f64 - value_4))) {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 45, 108, 105, 107, 101, 32, 108, 101, 110, 103, 116, 104, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 109, 97, 120, 105, 109, 117, 109, 32, 115, 97, 102, 101, 32, 105, 110, 116, 101, 103, 101, 114]))
                                                } else {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((length_value_2 + value_4))))
                                                }
                                            }
                                        }
                                        _ => {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 112, 112, 101, 110, 100, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 99, 111, 117, 110, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                        }
                                    }
                                }
                                _ => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 112, 112, 101, 110, 100, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 99, 111, 117, 110, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                }
                            }
                        }
                    }
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 100, 101, 108, 101, 116, 101, 32, 115, 117, 99, 99, 101, 115, 115]))) {
                        {
                            let value_5 = ({
                                let source_ml_source2_array_read_index2464 = 0f64;
                                { let ml_reference_array397 = values.clone(); let ml_reference_position397 = source_ml_source2_array_read_index2464; let ml_reference_projection397 = self::ml_array::optional_at(&*ml_reference_array397.borrow(), self::ml_array::number_index(ml_reference_position397)); ml_reference_projection397 }
                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                            match value_5.clone() {
                                self::JavaScriptSemanticValue::Boolean(value_value_3) => {
                                    if value_value_3 {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Undefined))
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 100, 101, 108, 101, 116, 101, 32, 97, 32, 110, 111, 110, 45, 99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121]))
                                    }
                                }
                                _ => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110, 32, 114, 101, 115, 117, 108, 116]))
                                }
                            }
                        }
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 100, 100, 101, 114]))) {
                            {
                                let value_6 = ({
                                    let source_ml_source2_array_read_index2474 = 0f64;
                                    { let ml_reference_array398 = values.clone(); let ml_reference_position398 = source_ml_source2_array_read_index2474; let ml_reference_projection398 = self::ml_array::optional_at(&*ml_reference_array398.borrow(), self::ml_array::number_index(ml_reference_position398)); ml_reference_projection398 }
                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                {
                                    let invalid = self::construct_java_script_call_primitive_type_error(self::ml_string::Text::from_units(vec![67, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 100, 100, 101, 114, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101]));
                                    match value_6.clone() {
                                        self::JavaScriptSemanticValue::Object(_, callable) => {
                                            if callable {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value_6.clone()))
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
                            if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 77, 97, 112, 32, 101, 110, 116, 114, 121, 32, 111, 98, 106, 101, 99, 116]))) {
                                {
                                    let value_7 = ({
                                        let source_ml_source2_array_read_index2484 = 0f64;
                                        { let ml_reference_array399 = values.clone(); let ml_reference_position399 = source_ml_source2_array_read_index2484; let ml_reference_projection399 = self::ml_array::optional_at(&*ml_reference_array399.borrow(), self::ml_array::number_index(ml_reference_position399)); ml_reference_projection399 }
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
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![73, 116, 101, 114, 97, 116, 111, 114, 32, 118, 97, 108, 117, 101, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 101, 110, 116, 114, 121, 32, 111, 98, 106, 101, 99, 116]))
                                    }
                                }
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 99, 111, 101, 114, 99, 105, 98, 108, 101]))) {
                                    {
                                        let value_8 = ({
                                            let source_ml_source2_array_read_index2495 = 0f64;
                                            { let ml_reference_array400 = values.clone(); let ml_reference_position400 = source_ml_source2_array_read_index2495; let ml_reference_projection400 = self::ml_array::optional_at(&*ml_reference_array400.borrow(), self::ml_array::number_index(ml_reference_position400)); ml_reference_projection400 }
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
                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 100, 101, 115, 116, 114, 117, 99, 116, 117, 114, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Undefined))
                                        }
                                    }
                                } else {
                                    if (operation == (self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]))) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Boolean({
                                            let source_ml_source2_s2516 = ({
                                                let source_ml_source2_array_read_index2527 = 0f64;
                                                { let ml_reference_array401 = values.clone(); let ml_reference_position401 = source_ml_source2_array_read_index2527; let ml_reference_projection401 = self::ml_array::optional_at(&*ml_reference_array401.borrow(), self::ml_array::number_index(ml_reference_position401)); ml_reference_projection401 }
                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                            match source_ml_source2_s2516.clone() {
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            }
                                        })))
                                    } else {
                                        if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]))) {
                                            {
                                                let value_9 = ({
                                                    let source_ml_source2_array_read_index2529 = 0f64;
                                                    { let ml_reference_array402 = values.clone(); let ml_reference_position402 = source_ml_source2_array_read_index2529; let ml_reference_projection402 = self::ml_array::optional_at(&*ml_reference_array402.borrow(), self::ml_array::number_index(ml_reference_position402)); ml_reference_projection402 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                {
                                                    let kind = if match value_9.clone() {
                                                        self::JavaScriptSemanticValue::Null => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    } {
                                                        self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                                    } else {
                                                        match value_9.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                                                            }
                                                            self::JavaScriptSemanticValue::Null => {
                                                                self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                                                            }
                                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                                            }
                                                            self::JavaScriptSemanticValue::Number(_) => {
                                                                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                                            }
                                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                                self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                                                            }
                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                                                            }
                                                            self::JavaScriptSemanticValue::Object(_, callable_2) => {
                                                                if callable_2 {
                                                                    self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                                                }
                                                            }
                                                            _ => {
                                                                self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                        {
                                                            let at = 0f64;
                                                            {
                                                                let units_2 = self::source_ml_source1_temporary_execute_java_script_call_primitive_scalar_loop152(kind.clone(), units.clone(), at);
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units_2.clone())))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if ((self::read_java_script_source_exception_class(operation.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                if ((self::read_java_script_call_scalar_conversion_error(operation.clone())) == (self::ml_string::Text::from_units(vec![]))) {
                                                    self::execute_java_script_call_scalar_conversion(operation.clone(), ({
                                                        let source_ml_source2_array_read_index2746 = 0f64;
                                                        { let ml_reference_array403 = values.clone(); let ml_reference_position403 = source_ml_source2_array_read_index2746; let ml_reference_projection403 = self::ml_array::optional_at(&*ml_reference_array403.borrow(), self::ml_array::number_index(ml_reference_position403)); ml_reference_projection403 }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                } else {
                                                    if { let ml_values404 = { let ml_reference_value405 = values.borrow().clone(); ml_reference_value405 }; let ml_callback404 = std::rc::Rc::new(move |value_10: self::JavaScriptSemanticValue| -> bool { match value_10.clone() {
                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    } }); ml_values404.iter().cloned().enumerate().any(|(ml_index404, ml_value404)| ml_callback404(ml_value404)) } {
                                                        {
                                                            let first = ({
                                                                let source_ml_source2_array_read_index2757 = 0f64;
                                                                { let ml_reference_array406 = values.clone(); let ml_reference_position406 = source_ml_source2_array_read_index2757; let ml_reference_projection406 = self::ml_array::optional_at(&*ml_reference_array406.borrow(), self::ml_array::number_index(ml_reference_position406)); ml_reference_projection406 }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                            {
                                                                let second = ({
                                                                    let source_ml_source2_array_read_index2759 = 1f64;
                                                                    { let ml_reference_array407 = values.clone(); let ml_reference_position407 = source_ml_source2_array_read_index2759; let ml_reference_projection407 = self::ml_array::optional_at(&*ml_reference_array407.borrow(), self::ml_array::number_index(ml_reference_position407)); ml_reference_projection407 }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                if ((operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]))) && ((self::read_java_script_semantic_text_length(first.clone())) >= (0f64))) {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::read_java_script_semantic_text_length(first.clone()))))
                                                                } else {
                                                                    if (((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]))) && ((self::read_java_script_semantic_text_length(first.clone())) >= (0f64))) && ((self::read_java_script_semantic_text_length(second.clone())) >= (0f64))) {
                                                                        if (((self::read_java_script_semantic_text_length(first.clone()) + self::read_java_script_semantic_text_length(second.clone()))) > (16777216f64)) {
                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 54, 55, 55, 55, 50, 49, 54, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                        } else {
                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::construct_java_script_source_text((self::read_java_script_semantic_text_string(first.clone())).concat(&self::read_java_script_semantic_text_string(second.clone())))))
                                                                        }
                                                                    } else {
                                                                        if { let ml_values408 = { let ml_reference_value409 = values.borrow().clone(); ml_reference_value409 }; let ml_callback408 = std::rc::Rc::new(move |value_10: self::JavaScriptSemanticValue| -> bool { (match value_10.clone() {
                                                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        } && ((((((match value_10.clone() {
                                                                            self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value2770) => {
                                                                                Some((source_ml_source2_data_property_value2770.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(self::ml_string::Text,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone()).length()) > (1048576f64))) }); ml_values408.iter().cloned().enumerate().any(|(ml_index408, ml_value408)| ml_callback408(ml_value408)) } {
                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 116, 101, 120, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 115, 101, 112, 97, 114, 97, 116, 101, 32, 98, 111, 117, 110, 100, 101, 100, 32, 117, 110, 105, 116, 45, 97, 114, 114, 97, 121, 32, 97, 108, 103, 111, 114, 105, 116, 104, 109, 59, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 109, 97, 105, 110, 115, 32, 105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 97, 110, 100, 32, 114, 101, 115, 117, 109, 97, 98, 108, 101]))
                                                                        } else {
                                                                            {
                                                                                let materialized = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                {
                                                                                    let source_ml_source1_temporary_index151 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let materialized_2 = self::source_ml_source1_temporary_execute_java_script_call_primitive_scalar_loop153(materialized.clone(), values.clone(), source_ml_source1_temporary_index151.clone());
                                                                                        self::execute_java_script_call_primitive(operation.clone(), materialized_2.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if ((self::read_java_script_call_integer_arity(operation.clone())) != (0f64)) {
                                                            self::execute_java_script_call_integer_primitive(operation.clone(), values.clone())
                                                        } else {
                                                            if (self::test_java_script_call_unary_number_operation(operation.clone()) || self::ml_array::includes(&{ let ml_reference_value410 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 105, 110, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 109, 97, 120, 105, 109, 117, 109, 32, 110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 116, 101, 120, 116, 32, 117, 110, 105, 116, 115])]))).borrow().clone(); ml_reference_value410 }, &operation.clone(), 0f64, false, |a, b| a == b)) {
                                                                self::execute_java_script_call_numeric_primitive(operation.clone(), values.clone())
                                                            } else {
                                                                self::execute_java_script_call_text_primitive(operation.clone(), values.clone())
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

pub fn execute_java_script_call_primitive_addition(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    if (match left.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => {
            true
        }
        _ => {
            false
        }
    } || match right.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => {
            true
        }
        _ => {
            false
        }
    }) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 100, 100, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 101, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 115]))
    } else {
        if (((match left.clone() {
            self::JavaScriptSemanticValue::Text(_) => {
                true
            }
            _ => {
                false
            }
        } || match left.clone() {
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                true
            }
            _ => {
                false
            }
        }) || match right.clone() {
            self::JavaScriptSemanticValue::Text(_) => {
                true
            }
            _ => {
                false
            }
        }) || match right.clone() {
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                true
            }
            _ => {
                false
            }
        }) {
            {
                let first = self::execute_java_script_call_text_conversion(left.clone());
                match first.clone() {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(first_value) => {
                        let first_value = *first_value;
                        {
                            let second = self::execute_java_script_call_text_conversion(right.clone());
                            match second.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                    let value = *value;
                                    self::execute_java_script_call_primitive(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![first_value.clone(), value.clone()])))
                                }
                                _ => {
                                    second.clone()
                                }
                            }
                        }
                    }
                    _ => {
                        first.clone()
                    }
                }
            }
        } else {
            if (match left.clone() {
                self::JavaScriptSemanticValue::Symbol(_) => {
                    true
                }
                _ => {
                    false
                }
            } || match right.clone() {
                self::JavaScriptSemanticValue::Symbol(_) => {
                    true
                }
                _ => {
                    false
                }
            }) {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32, 97, 32, 83, 121, 109, 98, 111, 108, 32, 118, 97, 108, 117, 101, 32, 116, 111, 32, 97, 32, 110, 117, 109, 98, 101, 114]))
            } else {
                if (match left.clone() {
                    self::JavaScriptSemanticValue::Integer(_) => {
                        true
                    }
                    _ => {
                        false
                    }
                } && match right.clone() {
                    self::JavaScriptSemanticValue::Integer(_) => {
                        true
                    }
                    _ => {
                        false
                    }
                }) {
                    self::execute_java_script_call_integer_primitive(self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115]), std::rc::Rc::new(std::cell::RefCell::new(vec![left.clone(), right.clone()])))
                } else {
                    if (match left.clone() {
                        self::JavaScriptSemanticValue::Integer(_) => {
                            true
                        }
                        _ => {
                            false
                        }
                    } || match right.clone() {
                        self::JavaScriptSemanticValue::Integer(_) => {
                            true
                        }
                        _ => {
                            false
                        }
                    }) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 109, 105, 120, 32, 66, 105, 103, 73, 110, 116, 32, 97, 110, 100, 32, 111, 116, 104, 101, 114, 32, 116, 121, 112, 101, 115, 44, 32, 117, 115, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 115]))
                    } else {
                        {
                            let first_2 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), left.clone());
                            match first_2.clone() {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(first_value_2) => {
                                    let first_value_2 = *first_value_2;
                                    {
                                        let second_2 = self::execute_java_script_call_number_conversion(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), right.clone());
                                        match second_2.clone() {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(second_value) => {
                                                let second_value = *second_value;
                                                match first_value_2.clone() {
                                                    self::JavaScriptSemanticValue::Number(first_number_value) => {
                                                        match second_value.clone() {
                                                            self::JavaScriptSemanticValue::Number(value_2) => {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((first_number_value + value_2))))
                                                            }
                                                            _ => {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 108, 116, 115]))
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 97, 100, 100, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 115, 117, 108, 116, 115]))
                                                    }
                                                }
                                            }
                                            _ => {
                                                second_2.clone()
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    first_2.clone()
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn execute_java_script_call_numeric_update(operation: self::ml_string::Text, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    if (((operation != (self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]))) && (operation != (self::ml_string::Text::from_units(vec![100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101])))) && (operation != (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101])))) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 117, 112, 100, 97, 116, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 111, 114, 32, 100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Number(value_value) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (operation == (self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]))) {
                    (value_value + 1f64)
                } else {
                    if (operation == (self::ml_string::Text::from_units(vec![100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]))) {
                        (value_value - 1f64)
                    } else {
                        (-value_value)
                    }
                })))
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                if (operation == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]))) {
                    self::execute_java_script_call_integer_primitive(self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone()])))
                } else {
                    self::execute_java_script_call_integer_primitive(if (operation == (self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]))) {
                        self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115])
                    } else {
                        self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115])
                    }, std::rc::Rc::new(std::cell::RefCell::new(vec![value.clone(), self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(1))])))
                }
            }
            _ => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 117, 112, 100, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 114, 32, 66, 105, 103, 73, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
            }
        }
    }
}

pub fn execute_java_script_call_numeric_binary(operation: self::ml_string::Text, left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
    if !(self::ml_array::includes(&{ let ml_reference_value411 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 111, 119, 101, 114])]))).borrow().clone(); ml_reference_value411 }, &operation.clone(), 0f64, false, |a, b| a == b)) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 98, 105, 110, 97, 114, 121, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 114, 105, 116, 104, 109, 101, 116, 105, 99, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
    } else {
        if (!(self::ml_array::includes(&{ let ml_reference_value412 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])]))).borrow().clone(); ml_reference_value412 }, &match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
            }
            self::JavaScriptSemanticValue::Null => {
                self::ml_string::Text::from_units(vec![110, 117, 108, 108])
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
            }
            self::JavaScriptSemanticValue::Number(_) => {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::ml_string::Text::from_units(vec![116, 101, 120, 116])
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
            }
        }, 0f64, false, |a, b| a == b)) || !(self::ml_array::includes(&{ let ml_reference_value413 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])]))).borrow().clone(); ml_reference_value413 }, &match right.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
            }
            self::JavaScriptSemanticValue::Null => {
                self::ml_string::Text::from_units(vec![110, 117, 108, 108])
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
            }
            self::JavaScriptSemanticValue::Number(_) => {
                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::ml_string::Text::from_units(vec![116, 101, 120, 116])
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
            }
        }, 0f64, false, |a, b| a == b))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 97, 114, 105, 116, 104, 109, 101, 116, 105, 99, 32, 114, 101, 116, 97, 105, 110, 115, 32, 98, 111, 116, 104, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 101, 114, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
        } else {
            if ((match left.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                }
                self::JavaScriptSemanticValue::Null => {
                    self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                }
                self::JavaScriptSemanticValue::Boolean(_) => {
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                }
                self::JavaScriptSemanticValue::Number(_) => {
                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
                }
                self::JavaScriptSemanticValue::Text(_) => {
                    self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                }
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                }
            }) != (match right.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                }
                self::JavaScriptSemanticValue::Null => {
                    self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                }
                self::JavaScriptSemanticValue::Boolean(_) => {
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                }
                self::JavaScriptSemanticValue::Number(_) => {
                    self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
                }
                self::JavaScriptSemanticValue::Text(_) => {
                    self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                }
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                }
            })) {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 109, 105, 120, 32, 66, 105, 103, 73, 110, 116, 32, 97, 110, 100, 32, 111, 116, 104, 101, 114, 32, 116, 121, 112, 101, 115, 44, 32, 117, 115, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 115]))
            } else {
                if (match left.clone() {
                    self::JavaScriptSemanticValue::Integer(_) => {
                        true
                    }
                    _ => {
                        false
                    }
                } && match right.clone() {
                    self::JavaScriptSemanticValue::Integer(_) => {
                        true
                    }
                    _ => {
                        false
                    }
                }) {
                    self::execute_java_script_call_integer_primitive(if (operation == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]))) {
                        self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115])
                    } else {
                        if (operation == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]))) {
                            self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115])
                        } else {
                            if (operation == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]))) {
                                self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115])
                            } else {
                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114]))) {
                                    self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])
                                } else {
                                    self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114])
                                }
                            }
                        }
                    }, std::rc::Rc::new(std::cell::RefCell::new(vec![left.clone(), right.clone()])))
                } else {
                    match left.clone() {
                        self::JavaScriptSemanticValue::Number(left_value) => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::Number(value) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (operation == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]))) {
                                        (left_value - value)
                                    } else {
                                        if (operation == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]))) {
                                            (left_value * value)
                                        } else {
                                            if (operation == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]))) {
                                                (left_value / value)
                                            } else {
                                                if (operation == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114]))) {
                                                    (left_value % value)
                                                } else {
                                                    self::ml_big_integer_conversion::power_java_script_number(left_value,value)
                                                }
                                            }
                                        }
                                    })))
                                }
                                _ => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 97, 114, 105, 116, 104, 109, 101, 116, 105, 99, 32, 114, 101, 116, 97, 105, 110, 115, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 78, 117, 109, 98, 101, 114, 32, 111, 114, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                }
                            }
                        }
                        _ => {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 97, 114, 105, 116, 104, 109, 101, 116, 105, 99, 32, 114, 101, 116, 97, 105, 110, 115, 32, 109, 97, 116, 99, 104, 105, 110, 103, 32, 78, 117, 109, 98, 101, 114, 32, 111, 114, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                        }
                    }
                }
            }
        }
    }
}
