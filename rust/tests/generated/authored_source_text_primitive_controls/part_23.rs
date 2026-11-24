pub fn execute_java_script_call_relational_comparison(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> self::JavaScriptCallPrimitiveResult {
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
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 98, 111, 116, 104, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 115]))
    } else {
        {
            let text_left = (match left.clone() {
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
            });
            {
                let text_right = (match right.clone() {
                    self::JavaScriptSemanticValue::Text(_) => {
                        true
                    }
                    _ => {
                        false
                    }
                } || match right.clone() {
                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                        true
                    }
                    _ => {
                        false
                    }
                });
                if (text_left && text_right) {
                    self::execute_java_script_call_primitive(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![left.clone(), right.clone()])))
                } else {
                    if (text_left && match right.clone() {
                        self::JavaScriptSemanticValue::Integer(_) => {
                            true
                        }
                        _ => {
                            false
                        }
                    }) {
                        self::execute_java_script_call_integer_primitive(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110, 116, 101, 103, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![left.clone(), right.clone()])))
                    } else {
                        if (match left.clone() {
                            self::JavaScriptSemanticValue::Integer(_) => {
                                true
                            }
                            _ => {
                                false
                            }
                        } && text_right) {
                            self::invert_java_script_call_relational_comparison(self::execute_java_script_call_integer_primitive(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110, 116, 101, 103, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![right.clone(), left.clone()]))))
                        } else {
                            {
                                let first = self::execute_java_script_call_primitive(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99]), std::rc::Rc::new(std::cell::RefCell::new(vec![left.clone()])));
                                match first.clone() {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(first_value) => {
                                        let first_value = *first_value;
                                        {
                                            let second = self::execute_java_script_call_primitive(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 101, 114, 105, 99]), std::rc::Rc::new(std::cell::RefCell::new(vec![right.clone()])));
                                            match second.clone() {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(second_value) => {
                                                    let second_value = *second_value;
                                                    if (match first_value.clone() {
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    } && match second_value.clone() {
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    }) {
                                                        self::execute_java_script_call_integer_primitive(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]), std::rc::Rc::new(std::cell::RefCell::new(vec![first_value.clone(), second_value.clone()])))
                                                    } else {
                                                        if (match first_value.clone() {
                                                            self::JavaScriptSemanticValue::Number(_) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        } && match second_value.clone() {
                                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }) {
                                                            self::execute_java_script_call_integer_primitive(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![first_value.clone(), second_value.clone()])))
                                                        } else {
                                                            if (match first_value.clone() {
                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            } && match second_value.clone() {
                                                                self::JavaScriptSemanticValue::Number(_) => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            }) {
                                                                self::invert_java_script_call_relational_comparison(self::execute_java_script_call_integer_primitive(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 116, 101, 103, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![second_value.clone(), first_value.clone()]))))
                                                            } else {
                                                                match first_value.clone() {
                                                                    self::JavaScriptSemanticValue::Number(a_value) => {
                                                                        match second_value.clone() {
                                                                            self::JavaScriptSemanticValue::Number(value) => {
                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (a_value.is_nan() || value.is_nan()) {
                                                                                    2f64
                                                                                } else {
                                                                                    if (a_value < value) {
                                                                                        (-1f64)
                                                                                    } else {
                                                                                        if (a_value > value) {
                                                                                            1f64
                                                                                        } else {
                                                                                            0f64
                                                                                        }
                                                                                    }
                                                                                })))
                                                                            }
                                                                            _ => {
                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                            }
                                                                        }
                                                                    }
                                                                    _ => {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
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
                        }
                    }
                }
            }
        }
    }
}

pub fn invert_java_script_call_relational_comparison(result: self::JavaScriptCallPrimitiveResult) -> self::JavaScriptCallPrimitiveResult {
    match result.clone() {
        self::JavaScriptCallPrimitiveResult::PrimitiveValue(order) => {
            let order = *order;
            match order.clone() {
                self::JavaScriptSemanticValue::Number(value) => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (value == (2f64)) {
                        2f64
                    } else {
                        if (value == (0f64)) {
                            0f64
                        } else {
                            (-value)
                        }
                    })))
                }
                _ => {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 105, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 101, 114, 105, 110, 103, 32, 99, 111, 100, 101]))
                }
            }
        }
        _ => {
            result.clone()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ({ let ml_reference_array414 = units.clone(); let ml_reference_projection414 = ml_reference_array414.borrow().len() as f64; ml_reference_projection414 })) {
            {
                let unit = ({ let ml_reference_array415 = units.clone(); let ml_reference_position415 = at; let ml_reference_projection415 = self::ml_array::optional_at(&*ml_reference_array415.borrow(), self::ml_array::number_index(ml_reference_position415)); ml_reference_projection415 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}

pub fn source_ml_source1_temporary_test_java_script_semantic_text_equal_loop2(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if (at < length) {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Text(units) => {
                        ({ let ml_reference_array416 = units.clone(); let ml_reference_position416 = at; let ml_reference_projection416 = self::ml_array::optional_at(&*ml_reference_array416.borrow(), self::ml_array::number_index(ml_reference_position416)); ml_reference_projection416 }).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_f3121) => {
                        {
                            let text = source_ml_source2_f3121.clone();
                            {
                                let source_ml_source2_string_unit_operand3175 = text.clone();
                                {
                                    let source_ml_source2_string_unit_operand3176 = at;
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
                                                let source_ml_source2_string_unit_operand3182 = {
                                                    let source_ml_source2_string_range_operand3177 = text_2.clone();
                                                    {
                                                        let source_ml_source2_string_range_operand3178 = index;
                                                        {
                                                            let source_ml_source2_string_range_operand3179 = (index + 1f64);
                                                            {
                                                                let source_ml_source2_string_range_position3180 = source_ml_source2_string_range_operand3178;
                                                                {
                                                                    let source_ml_source2_string_range_position3181 = source_ml_source2_string_range_operand3179;
                                                                    source_ml_source2_string_range_operand3177.slice(source_ml_source2_string_range_position3180, source_ml_source2_string_range_position3181)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_string_unit_operand3183 = 0f64;
                                                    source_ml_source2_string_unit_operand3182.code_point_at(source_ml_source2_string_unit_operand3183)
                                                }
                                            }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    } }))(source_ml_source2_string_unit_operand3175.clone(), source_ml_source2_string_unit_operand3176)
                                }
                            }
                        }
                    }
                    _ => {
                        (-1f64)
                    }
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            ({ let ml_reference_array417 = units_2.clone(); let ml_reference_position417 = at; let ml_reference_projection417 = self::ml_array::optional_at(&*ml_reference_array417.borrow(), self::ml_array::number_index(ml_reference_position417)); ml_reference_projection417 }).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_f3204) => {
                            {
                                let text_2 = source_ml_source2_f3204.clone();
                                {
                                    let source_ml_source2_string_unit_operand3258 = text_2.clone();
                                    {
                                        let source_ml_source2_string_unit_operand3259 = at;
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
                                                    let source_ml_source2_string_unit_operand3265 = {
                                                        let source_ml_source2_string_range_operand3260 = text_3.clone();
                                                        {
                                                            let source_ml_source2_string_range_operand3261 = index;
                                                            {
                                                                let source_ml_source2_string_range_operand3262 = (index + 1f64);
                                                                {
                                                                    let source_ml_source2_string_range_position3263 = source_ml_source2_string_range_operand3261;
                                                                    {
                                                                        let source_ml_source2_string_range_position3264 = source_ml_source2_string_range_operand3262;
                                                                        source_ml_source2_string_range_operand3260.slice(source_ml_source2_string_range_position3263, source_ml_source2_string_range_position3264)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_string_unit_operand3266 = 0f64;
                                                        source_ml_source2_string_unit_operand3265.code_point_at(source_ml_source2_string_unit_operand3266)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source2_string_unit_operand3258.clone(), source_ml_source2_string_unit_operand3259)
                                    }
                                }
                            }
                        }
                        _ => {
                            (-1f64)
                        }
                    };
                    if (a != b) {
                        self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
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
            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, mut source_ml_source1_temporary_values3: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_source1_temporary_index3 < ({ let ml_reference_array418 = source_ml_source1_temporary_values3.clone(); let ml_reference_projection418 = self::ml::Big::from_u128(ml_reference_array418.borrow().len() as u128); ml_reference_projection418 })) {
            {
                let unit = { let ml_reference_array419 = source_ml_source1_temporary_values3.clone(); let ml_reference_position419 = source_ml_source1_temporary_index3.clone(); let ml_reference_projection419 = self::ml_array::at(&*ml_reference_array419.borrow(), ml_reference_position419.to_index()); ml_reference_projection419 };
                {
                    let text_2 = text.concat(&self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![unit]).into_iter().map(|ml_unit| ml_unit as u16).collect()));
                    {
                        let source_ml_source1_temporary_index3_2 = source_ml_source1_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_source1_temporary_index3) = (text_2.clone(), source_ml_source1_temporary_index3_2.clone());
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

pub fn source_ml_source1_temporary_construct_java_script_semantic_text_units_loop5(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < length) {
            {
                let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value420 = units.borrow().clone(); ml_reference_value420 }, vec![self::read_java_script_semantic_text_unit(value.clone(), at)])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (units, at) = (units_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            units.clone()
        };
    }
}

pub fn source_ml_source1_temporary_construct_java_script_symbol_descriptive_text_loop7(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values6: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index6: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (source_ml_source1_temporary_index6 < ({ let ml_reference_array421 = source_ml_source1_temporary_values6.clone(); let ml_reference_projection421 = self::ml::Big::from_u128(ml_reference_array421.borrow().len() as u128); ml_reference_projection421 })) {
            {
                let unit = { let ml_reference_array422 = source_ml_source1_temporary_values6.clone(); let ml_reference_position422 = source_ml_source1_temporary_index6.clone(); let ml_reference_projection422 = self::ml_array::at(&*ml_reference_array422.borrow(), ml_reference_position422.to_index()); ml_reference_projection422 };
                {
                    let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value423 = units.borrow().clone(); ml_reference_value423 }, vec![unit])));
                    {
                        let source_ml_source1_temporary_index6_2 = source_ml_source1_temporary_index6.add(&self::ml::Big::from_i128(1));
                        {
                            (units, source_ml_source1_temporary_index6) = (units_2.clone(), source_ml_source1_temporary_index6_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            units.clone()
        };
    }
}

pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop8(mut count: f64, mut factor: f64, mut position: f64) -> f64 {
    loop {
        return if (position < count) {
            {
                let factor_2 = (factor * 2f64);
                {
                    let position_2 = (position + 1f64);
                    {
                        (factor, position) = (factor_2, position_2);
                        continue;
                    }
                }
            }
        } else {
            factor
        };
    }
}

pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop9(mut operation: f64, mut first: f64, mut second: f64, mut result: f64, mut weight: f64, mut position: f64) -> f64 {
    loop {
        return if (position < (32f64)) {
            {
                let first_bit = (first % 2f64);
                {
                    let second_bit = (second % 2f64);
                    {
                        let result_3 = if (((((operation == (0f64)) && (first_bit == (1f64))) && (second_bit == (1f64))) || ((operation == (1f64)) && ((first_bit == (1f64)) || (second_bit == (1f64))))) || ((operation == (2f64)) && (first_bit != second_bit))) {
                            {
                                let result_2 = (result + weight);
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let first_2 = ((first - first_bit) / 2f64);
                            {
                                let second_2 = ((second - second_bit) / 2f64);
                                {
                                    let weight_2 = (weight * 2f64);
                                    {
                                        let position_2 = (position + 1f64);
                                        {
                                            (first, second, result, weight, position) = (first_2, second_2, result_3, weight_2, position_2);
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
            result
        };
    }
}

pub fn source_ml_source1_temporary_count_java_script_leading_zero_bits_loop10(mut integer: f64, mut count: f64) -> f64 {
    loop {
        return if (integer > (0f64)) {
            {
                let integer_2 = ((integer / 2f64)).floor();
                {
                    let count_2 = (count - 1f64);
                    {
                        (integer, count) = (integer_2, count_2);
                        continue;
                    }
                }
            }
        } else {
            count
        };
    }
}

pub fn source_ml_source1_temporary_round_java_script_binary32_loop11(mut magnitude: f64, mut lower: f64, mut spacing: f64) -> f64 {
    loop {
        return if (magnitude >= ((lower * 2f64))) {
            {
                let lower_2 = (lower * 2f64);
                {
                    let spacing_2 = (spacing * 2f64);
                    {
                        (lower, spacing) = (lower_2, spacing_2);
                        continue;
                    }
                }
            }
        } else {
            spacing
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Done(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Done(f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop15(mut bytes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64, mut scalar: f64, mut remaining: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Result {
    loop {
        return if (remaining > (0f64)) {
            {
                let next = ({ let ml_reference_array424 = bytes.clone(); let ml_reference_position424 = index; let ml_reference_projection424 = self::ml_array::optional_at(&*ml_reference_array424.borrow(), self::ml_array::number_index(ml_reference_position424)); ml_reference_projection424 }).clone().unwrap_or_else(|| (-1f64));
                if ((next < (128f64)) || (next > (191f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Return(std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))
                } else {
                    {
                        let scalar_2 = (((scalar * 64f64) + next) - 128f64);
                        {
                            let index_2 = (index + 1f64);
                            {
                                let remaining_2 = (remaining - 1f64);
                                {
                                    (index, scalar, remaining) = (index_2, scalar_2, remaining_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Done(index, scalar)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14 {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14Next(f64, f64, f64),
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop13(mut bytes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result {
    loop {
        return if (index < ({ let ml_reference_array425 = bytes.clone(); let ml_reference_projection425 = ml_reference_array425.borrow().len() as f64; ml_reference_projection425 })) {
            {
                let scalar = ({ let ml_reference_array426 = bytes.clone(); let ml_reference_position426 = index; let ml_reference_projection426 = self::ml_array::optional_at(&*ml_reference_array426.borrow(), self::ml_array::number_index(ml_reference_position426)); ml_reference_projection426 }).clone().unwrap_or_else(|| (-1f64));
                {
                    let index_2 = (index + 1f64);
                    {
                        let remaining = 0f64;
                        {
                            let minimum = 0f64;
                            {
                                let source_ml_source2_s3292 = if ((scalar >= (194f64)) && (scalar <= (223f64))) {
                                    {
                                        let scalar_2 = (scalar - 192f64);
                                        {
                                            let remaining_2 = 1f64;
                                            {
                                                let minimum_2 = 128f64;
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14Next(scalar_2, remaining_2, minimum_2)
                                            }
                                        }
                                    }
                                } else {
                                    if ((scalar >= (224f64)) && (scalar <= (239f64))) {
                                        {
                                            let scalar_3 = (scalar - 224f64);
                                            {
                                                let remaining_3 = 2f64;
                                                {
                                                    let minimum_3 = 2048f64;
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14Next(scalar_3, remaining_3, minimum_3)
                                                }
                                            }
                                        }
                                    } else {
                                        if ((scalar >= (240f64)) && (scalar <= (244f64))) {
                                            {
                                                let scalar_4 = (scalar - 240f64);
                                                {
                                                    let remaining_4 = 3f64;
                                                    {
                                                        let minimum_4 = 65536f64;
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14Next(scalar_4, remaining_4, minimum_4)
                                                    }
                                                }
                                            }
                                        } else {
                                            if (scalar >= (128f64)) {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14Return(std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14Next(scalar, remaining, minimum)
                                            }
                                        }
                                    }
                                };
                                match source_ml_source2_s3292.clone() {
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14Next(scalar_5, remaining_5, minimum_5) => {
                                        {
                                            let source_ml_source2_s3293 = self::source_ml_source1_temporary_read_java_script_utf8_string_units_loop15(bytes.clone(), index_2, scalar_5, remaining_5);
                                            match source_ml_source2_s3293.clone() {
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Done(index_3, scalar_6) => {
                                                    if (((scalar_6 < minimum_5) || (scalar_6 > (1114111f64))) || ((scalar_6 >= (55296f64)) && (scalar_6 <= (57343f64)))) {
                                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Return(std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))
                                                    } else {
                                                        if (scalar_6 <= (65535f64)) {
                                                            {
                                                                let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value427 = units.borrow().clone(); ml_reference_value427 }, vec![scalar_6])));
                                                                {
                                                                    (units, index) = (units_2.clone(), index_3);
                                                                    continue;
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let units_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value428 = units.borrow().clone(); ml_reference_value428 }, vec![(55296f64 + (((scalar_6 - 65536f64) / 1024f64)).floor())])));
                                                                {
                                                                    let units_4 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value429 = units_3.borrow().clone(); ml_reference_value429 }, vec![(56320f64 + ((scalar_6 - 65536f64) % 1024f64))])));
                                                                    {
                                                                        (units, index) = (units_4.clone(), index_3);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop15Return(source_ml_source1_temporary_result15) => {
                                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Return(source_ml_source1_temporary_result15.clone())
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14::MlSource1TemporaryReadJavaScriptUtf8StringUnitsJoin14Return(source_ml_source1_temporary_result15_2) => {
                                        self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Return(source_ml_source1_temporary_result15_2.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop13Done(units.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Result {
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Done,
    MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

pub fn source_ml_source1_temporary_read_java_script_utf8_string_units_loop12(mut bytes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Result {
    loop {
        return if (index < ({ let ml_reference_array430 = bytes.clone(); let ml_reference_projection430 = ml_reference_array430.borrow().len() as f64; ml_reference_projection430 })) {
            {
                let unit = ({ let ml_reference_array431 = bytes.clone(); let ml_reference_position431 = index; let ml_reference_projection431 = self::ml_array::optional_at(&*ml_reference_array431.borrow(), self::ml_array::number_index(ml_reference_position431)); ml_reference_projection431 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_safe_integer(unit)) || (unit < (0f64))) || (unit > (255f64))) {
                    self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Return(std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))
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
        } else {
            self::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Result::MlSource1TemporaryReadJavaScriptUtf8StringUnitsLoop12Done
        };
    }
}

pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop16(mut exponent: f64, mut value: self::ml::Big, mut base: self::ml::Big) -> self::ml::Big {
    loop {
        return if (exponent > (0f64)) {
            {
                let value_3 = if (((exponent % 2f64)) == (1f64)) {
                    {
                        let value_2 = value.mul(&base);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let exponent_2 = ((exponent / 2f64)).floor();
                    if (exponent_2 > (0f64)) {
                        {
                            let base_2 = base.mul(&base);
                            {
                                (exponent, value, base) = (exponent_2, value_3.clone(), base_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (exponent, value) = (exponent_2, value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop17(mut value: self::ml::Big, mut result: f64, mut scale: f64) -> f64 {
    loop {
        return if (value > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = (result + scale);
                        result_2
                    }
                } else {
                    result
                };
                {
                    let value_2 = self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let scale_2 = (scale * 2f64);
                        {
                            (value, result, scale) = (value_2.clone(), result_3, scale_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_read_string_number_binary_power_loop19(mut exponent: f64, mut result: f64) -> f64 {
    loop {
        return if (exponent < (0f64)) {
            {
                let result_2 = (result / 2f64);
                {
                    let exponent_2 = (exponent + 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop18Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop18Done(f64, f64),
}

pub fn source_ml_source1_temporary_read_string_number_binary_power_loop18(mut exponent: f64, mut result: f64) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Result {
    loop {
        return if (exponent > (0f64)) {
            {
                let result_2 = (result * 2f64);
                {
                    let exponent_2 = (exponent - 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop23Result {
    MlSource1TemporaryConvertStringNumberRationalLoop23Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop23(mut quotient: self::ml::Big, mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop23Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let scale_2 = self::ml::divide(&scale, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remainder >= scale_2) {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (quotient_2.clone(), remainder_2.clone(), scale_2.clone(), weight_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (scale, weight) = (scale_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop23Result::MlSource1TemporaryConvertStringNumberRationalLoop23Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop22Result {
    MlSource1TemporaryConvertStringNumberRationalLoop22Done(self::ml::Big, self::ml::Big),
}

pub fn source_ml_source1_temporary_convert_string_number_rational_loop22(mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop22Result {
    loop {
        return if (scale <= remainder) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (scale, weight) = (scale_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop22Result::MlSource1TemporaryConvertStringNumberRationalLoop22Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin21 {
    MlSource1TemporaryConvertStringNumberRationalJoin21Next(self::ml::Big, self::ml::Big),
}
