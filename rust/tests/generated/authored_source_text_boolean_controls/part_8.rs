pub fn read_java_script_semantic_text_unit(value: self::JavaScriptSemanticValue, index: f64) -> f64 {
    if ((!(self::ml_math::is_safe_integer(index)) || (index < (0f64))) || (index >= (self::read_java_script_semantic_text_length(value.clone())))) {
        (-1f64)
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Text(source_ml_source1_f143) => {
                ({ let ml_reference_array3 = source_ml_source1_f143.clone(); let ml_reference_position3 = index; let ml_reference_projection3 = self::ml_array::optional_at(&*ml_reference_array3.borrow(), self::ml_array::number_index(ml_reference_position3)); ml_reference_projection3 }).clone().unwrap_or_else(|| (-1f64))
            }
            self::JavaScriptSemanticValue::SourceText(_, text) => {
                {
                    let source_ml_source1_string_unit_operand198 = text.clone();
                    {
                        let source_ml_source1_string_unit_operand199 = index;
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
                                    let source_ml_source1_string_unit_operand205 = {
                                        let source_ml_source1_string_range_operand200 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand201 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand202 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position203 = source_ml_source1_string_range_operand201;
                                                    {
                                                        let source_ml_source1_string_range_position204 = source_ml_source1_string_range_operand202;
                                                        source_ml_source1_string_range_operand200.slice(source_ml_source1_string_range_position203, source_ml_source1_string_range_position204)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand206 = 0f64;
                                        source_ml_source1_string_unit_operand205.code_point_at(source_ml_source1_string_unit_operand206)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand198.clone(), source_ml_source1_string_unit_operand199)
                    }
                }
            }
            _ => {
                (-1f64)
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
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    true
                }
                _ => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    true
                }
                _ => {
                    false
                }
            }) {
                (((((match left.clone() {
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value243) => {
                        Some((source_ml_source1_data_property_value243.clone(),))
                    }
                    _ => {
                        None::<(self::ml_string::Text,)>
                    }
                }).clone().expect("a proven present value")).0).clone()) == ((((match right.clone() {
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value244) => {
                        Some((source_ml_source1_data_property_value244.clone(),))
                    }
                    _ => {
                        None::<(self::ml_string::Text,)>
                    }
                }).clone().expect("a proven present value")).0).clone()))
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s245 = self::source_ml_temporary_test_java_script_semantic_text_equal_loop2(left.clone(), right.clone(), length, at);
                        match source_ml_source1_s245.clone() {
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
        self::JavaScriptSemanticValue::Text(source_ml_source1_f249) => {
            {
                let text = self::ml_string::Text::from_units(vec![]);
                {
                    let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                    {
                        let text_2 = self::source_ml_temporary_read_java_script_semantic_text_string_loop4(text.clone(), source_ml_source1_f249.clone(), source_ml_temporary_index3.clone());
                        text_2.clone()
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(_, text_3) => {
            text_3.clone()
        }
        _ => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
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
                self::JavaScriptSemanticValue::Text(units) => {
                    units.clone()
                }
                _ => {
                    {
                        let units_2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                        {
                            let at = 0f64;
                            {
                                let units_3 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_2.clone(), at);
                                units_3.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn construct_java_script_source_text(text: self::ml_string::Text) -> self::JavaScriptSemanticValue {
    {
        let error = self::read_java_script_source_text_profile_error(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), text.length());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (error.clone()).0, message_present: true }).raise()
        } else {
            self::JavaScriptSemanticValue::SourceText(self::ml_string::Text::from_units(vec![105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49]), text.clone())
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
            self::JavaScriptSemanticValue::Object(_, source_ml_source1_f334) => {
                source_ml_source1_f334
            }
            _ => {
                false
            }
        }
    }
}

pub fn find_java_script_property_descriptor_entry(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s399 = self::source_ml_temporary_find_java_script_property_descriptor_entry_loop6(properties.clone(), key.clone(), at);
            match source_ml_source1_s399.clone() {
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done => {
                    (-1f64)
                }
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(source_ml_temporary_result6) => {
                    source_ml_temporary_result6
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

pub fn create_java_script_property_descriptor(kind: self::ml_string::Text, configurable: bool, enumerable: bool) -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field404 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
        {
            let source_ml_source1_field405 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source1_field406 = (kind != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                {
                    let source_ml_source1_field408 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                    {
                        let source_ml_source1_field409 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source1_field410 = (kind == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])));
                            {
                                let source_ml_source1_field411 = self::JavaScriptSemanticValue::Undefined;
                                (configurable, true, enumerable, true, source_ml_source1_field409.clone(), source_ml_source1_field408, source_ml_source1_field411.clone(), source_ml_source1_field410, source_ml_source1_field405.clone(), source_ml_source1_field404, false, source_ml_source1_field406)
                            }
                        }
                    }
                }
            }
        }
    }))
}

pub fn apply_java_script_property_descriptor_fields(base: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field413 = if ({ let ml_reference_value52 = proposed.borrow().clone(); ml_reference_value52 }).1 {
            ({ let ml_reference_value53 = proposed.borrow().clone(); ml_reference_value53 }).0
        } else {
            ({ let ml_reference_value54 = base.borrow().clone(); ml_reference_value54 }).0
        };
        {
            let source_ml_source1_field415 = if ({ let ml_reference_value55 = proposed.borrow().clone(); ml_reference_value55 }).3 {
                ({ let ml_reference_value56 = proposed.borrow().clone(); ml_reference_value56 }).2
            } else {
                ({ let ml_reference_value57 = base.borrow().clone(); ml_reference_value57 }).2
            };
            {
                let source_ml_source1_field416 = ({ let ml_reference_value58 = base.borrow().clone(); ml_reference_value58 }).9;
                {
                    let source_ml_source1_field417 = if (({ let ml_reference_value59 = base.borrow().clone(); ml_reference_value59 }).9 && ({ let ml_reference_value60 = proposed.borrow().clone(); ml_reference_value60 }).9) {
                        ({ let ml_reference_value61 = proposed.borrow().clone(); ml_reference_value61 }).8
                    } else {
                        ({ let ml_reference_value62 = base.borrow().clone(); ml_reference_value62 }).8
                    };
                    {
                        let source_ml_source1_field418 = ({ let ml_reference_value63 = base.borrow().clone(); ml_reference_value63 }).11;
                        {
                            let source_ml_source1_field419 = if (({ let ml_reference_value64 = base.borrow().clone(); ml_reference_value64 }).11 && ({ let ml_reference_value65 = proposed.borrow().clone(); ml_reference_value65 }).11) {
                                ({ let ml_reference_value66 = proposed.borrow().clone(); ml_reference_value66 }).10
                            } else {
                                ({ let ml_reference_value67 = base.borrow().clone(); ml_reference_value67 }).10
                            };
                            {
                                let source_ml_source1_field420 = ({ let ml_reference_value68 = base.borrow().clone(); ml_reference_value68 }).5;
                                {
                                    let source_ml_source1_field421 = if (({ let ml_reference_value69 = base.borrow().clone(); ml_reference_value69 }).5 && ({ let ml_reference_value70 = proposed.borrow().clone(); ml_reference_value70 }).5) {
                                        ({ let ml_reference_value71 = proposed.borrow().clone(); ml_reference_value71 }).4
                                    } else {
                                        ({ let ml_reference_value72 = base.borrow().clone(); ml_reference_value72 }).4
                                    };
                                    {
                                        let source_ml_source1_field422 = ({ let ml_reference_value73 = base.borrow().clone(); ml_reference_value73 }).7;
                                        {
                                            let source_ml_source1_field423 = if (({ let ml_reference_value74 = base.borrow().clone(); ml_reference_value74 }).7 && ({ let ml_reference_value75 = proposed.borrow().clone(); ml_reference_value75 }).7) {
                                                ({ let ml_reference_value76 = proposed.borrow().clone(); ml_reference_value76 }).6
                                            } else {
                                                ({ let ml_reference_value77 = base.borrow().clone(); ml_reference_value77 }).6
                                            };
                                            (source_ml_source1_field413, true, source_ml_source1_field415, true, source_ml_source1_field421.clone(), source_ml_source1_field420, source_ml_source1_field423.clone(), source_ml_source1_field422, source_ml_source1_field417.clone(), source_ml_source1_field416, source_ml_source1_field419, source_ml_source1_field418)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }))
}

pub fn read_java_script_property_descriptor_writes(descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>> {
    {
        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()));
        {
            let writes_3 = if ({ let ml_reference_value78 = descriptor.borrow().clone(); ml_reference_value78 }).1 {
                {
                    let writes_2 = std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source1_concat_value425 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101])]));
                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value79 = writes.borrow().clone(); ml_reference_value79 }), { let ml_reference_value80 = source_ml_source1_concat_value425.borrow().clone(); ml_reference_value80 })
                    }));
                    writes_2.clone()
                }
            } else {
                writes.clone()
            };
            {
                let writes_5 = if ({ let ml_reference_value81 = descriptor.borrow().clone(); ml_reference_value81 }).3 {
                    {
                        let writes_4 = std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source1_concat_value427 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101])]));
                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value82 = writes_3.borrow().clone(); ml_reference_value82 }), { let ml_reference_value83 = source_ml_source1_concat_value427.borrow().clone(); ml_reference_value83 })
                        }));
                        writes_4.clone()
                    }
                } else {
                    writes_3.clone()
                };
                {
                    let writes_7 = if ({ let ml_reference_value84 = descriptor.borrow().clone(); ml_reference_value84 }).9 {
                        {
                            let writes_6 = std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source1_concat_value429 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])]));
                                self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value85 = writes_5.borrow().clone(); ml_reference_value85 }), { let ml_reference_value86 = source_ml_source1_concat_value429.borrow().clone(); ml_reference_value86 })
                            }));
                            writes_6.clone()
                        }
                    } else {
                        writes_5.clone()
                    };
                    {
                        let writes_9 = if ({ let ml_reference_value87 = descriptor.borrow().clone(); ml_reference_value87 }).11 {
                            {
                                let writes_8 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source1_concat_value431 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101])]));
                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value88 = writes_7.borrow().clone(); ml_reference_value88 }), { let ml_reference_value89 = source_ml_source1_concat_value431.borrow().clone(); ml_reference_value89 })
                                }));
                                writes_8.clone()
                            }
                        } else {
                            writes_7.clone()
                        };
                        {
                            let writes_11 = if ({ let ml_reference_value90 = descriptor.borrow().clone(); ml_reference_value90 }).5 {
                                {
                                    let writes_10 = std::rc::Rc::new(std::cell::RefCell::new({
                                        let source_ml_source1_concat_value433 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![103, 101, 116])]));
                                        self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value91 = writes_9.borrow().clone(); ml_reference_value91 }), { let ml_reference_value92 = source_ml_source1_concat_value433.borrow().clone(); ml_reference_value92 })
                                    }));
                                    writes_10.clone()
                                }
                            } else {
                                writes_9.clone()
                            };
                            {
                                let writes_13 = if ({ let ml_reference_value93 = descriptor.borrow().clone(); ml_reference_value93 }).7 {
                                    {
                                        let writes_12 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source1_concat_value435 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 101, 116])]));
                                            self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), { let ml_reference_value94 = writes_11.borrow().clone(); ml_reference_value94 }), { let ml_reference_value95 = source_ml_source1_concat_value435.borrow().clone(); ml_reference_value95 })
                                        }));
                                        writes_12.clone()
                                    }
                                } else {
                                    writes_11.clone()
                                };
                                writes_13.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn create_java_script_property_transition(disposition: self::ml_string::Text, error: self::ml_string::Text, descriptor: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, writes: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>) -> std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>> {
    std::rc::Rc::new(std::cell::RefCell::new((descriptor.clone(), disposition.clone(), error.clone(), writes.clone())))
}

pub fn plan_java_script_property_definition(exists: bool, extensible: bool, current: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, proposed: std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>) -> std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>> {
    {
        let error = self::read_java_script_property_descriptor_error(proposed.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), error.clone(), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
        } else {
            if (exists && !(self::test_complete_java_script_property_descriptor(current.clone()))) {
                self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 67, 117, 114, 114, 101, 110, 116, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
            } else {
                {
                    let kind = self::read_java_script_property_descriptor_kind(proposed.clone());
                    if !exists {
                        if !extensible {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let base = self::create_java_script_property_descriptor(kind.clone(), false, false);
                                {
                                    let descriptor = self::apply_java_script_property_descriptor_fields(base.clone(), proposed.clone());
                                    self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![99, 114, 101, 97, 116, 101]), self::ml_string::Text::from_units(vec![]), descriptor.clone(), self::read_java_script_property_descriptor_writes(descriptor.clone()))
                                }
                            }
                        }
                    } else {
                        if (((((!(({ let ml_reference_value96 = proposed.borrow().clone(); ml_reference_value96 }).1) && !(({ let ml_reference_value97 = proposed.borrow().clone(); ml_reference_value97 }).3)) && !(({ let ml_reference_value98 = proposed.borrow().clone(); ml_reference_value98 }).9)) && !(({ let ml_reference_value99 = proposed.borrow().clone(); ml_reference_value99 }).11)) && !(({ let ml_reference_value100 = proposed.borrow().clone(); ml_reference_value100 }).5)) && !(({ let ml_reference_value101 = proposed.borrow().clone(); ml_reference_value101 }).7)) {
                            self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![107, 101, 101, 112]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new())))
                        } else {
                            {
                                let original = self::read_java_script_property_descriptor_kind(current.clone());
                                {
                                    let source_ml_source1_s440 = if !(({ let ml_reference_value102 = current.borrow().clone(); ml_reference_value102 }).0) {
                                        if (({ let ml_reference_value103 = proposed.borrow().clone(); ml_reference_value103 }).1 && ({ let ml_reference_value104 = proposed.borrow().clone(); ml_reference_value104 }).0) {
                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                        } else {
                                            if (({ let ml_reference_value105 = proposed.borrow().clone(); ml_reference_value105 }).3 && ((({ let ml_reference_value106 = proposed.borrow().clone(); ml_reference_value106 }).2) != (({ let ml_reference_value107 = current.borrow().clone(); ml_reference_value107 }).2))) {
                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                            } else {
                                                if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                } else {
                                                    if (original == (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]))) {
                                                        if (({ let ml_reference_value108 = proposed.borrow().clone(); ml_reference_value108 }).5 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value109 = proposed.borrow().clone(); ml_reference_value109 }).4, ({ let ml_reference_value110 = current.borrow().clone(); ml_reference_value110 }).4))) {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                        } else {
                                                            if (({ let ml_reference_value111 = proposed.borrow().clone(); ml_reference_value111 }).7 && !(self::test_java_script_semantic_same_value(({ let ml_reference_value112 = proposed.borrow().clone(); ml_reference_value112 }).6, ({ let ml_reference_value113 = current.borrow().clone(); ml_reference_value113 }).6))) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                            }
                                                        }
                                                    } else {
                                                        if !(({ let ml_reference_value114 = current.borrow().clone(); ml_reference_value114 }).10) {
                                                            if (({ let ml_reference_value115 = proposed.borrow().clone(); ml_reference_value115 }).11 && ({ let ml_reference_value116 = proposed.borrow().clone(); ml_reference_value116 }).10) {
                                                                self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                            } else {
                                                                if ({ let ml_reference_value117 = proposed.borrow().clone(); ml_reference_value117 }).9 {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(self::create_java_script_property_transition(if self::test_java_script_semantic_same_value(({ let ml_reference_value118 = proposed.borrow().clone(); ml_reference_value118 }).8, ({ let ml_reference_value119 = current.borrow().clone(); ml_reference_value119 }).8) {
                                                                        self::ml_string::Text::from_units(vec![107, 101, 101, 112])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![114, 101, 106, 101, 99, 116])
                                                                    }, self::ml_string::Text::from_units(vec![]), current.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml_string::Text>::new()))))
                                                                } else {
                                                                    self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next
                                    };
                                    match source_ml_source1_s440.clone() {
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next => {
                                            {
                                                let base_2 = if ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original)) {
                                                    self::create_java_script_property_descriptor(kind.clone(), ({ let ml_reference_value120 = current.borrow().clone(); ml_reference_value120 }).0, ({ let ml_reference_value121 = current.borrow().clone(); ml_reference_value121 }).2)
                                                } else {
                                                    current.clone()
                                                };
                                                {
                                                    let descriptor_2 = self::apply_java_script_property_descriptor_fields(base_2.clone(), proposed.clone());
                                                    {
                                                        let conversion = ((kind != (self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99]))) && (kind != original));
                                                        self::create_java_script_property_transition(if conversion {
                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![117, 112, 100, 97, 116, 101])
                                                        }, self::ml_string::Text::from_units(vec![]), descriptor_2.clone(), self::read_java_script_property_descriptor_writes(if conversion {
                                                            descriptor_2.clone()
                                                        } else {
                                                            proposed.clone()
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                        self::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7::MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(source_ml_temporary_result7) => {
                                            source_ml_temporary_result7.clone()
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

pub fn test_java_script_validated_heap_snapshot(_heap: self::JavaScriptOrdinaryHeap) -> bool {
    false
}

pub fn read_java_script_weak_collection_method_owner(operation: self::ml_string::Text) -> self::ml_string::Text {
    if self::ml_array::includes(&{ let ml_reference_value122 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 103, 101, 116]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 115, 101, 116]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 104, 97, 115]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 100, 101, 108, 101, 116, 101])]))).borrow().clone(); ml_reference_value122 }, &operation.clone(), 0f64, false, |a, b| a == b) {
        self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112])
    } else {
        if self::ml_array::includes(&{ let ml_reference_value123 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 46, 97, 100, 100]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 46, 104, 97, 115]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 46, 100, 101, 108, 101, 116, 101])]))).borrow().clone(); ml_reference_value123 }, &operation.clone(), 0f64, false, |a, b| a == b) {
            self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

pub fn read_java_script_weak_collection_operation_kind(operation: self::ml_string::Text) -> self::ml_string::Text {
    self::read_java_script_weak_collection_method_owner(operation.clone())
}

pub fn read_java_script_weak_key_eligibility(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f447) => {
            {
                let facts = if {
                    let source_ml_source1_comparison_operand515 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value514, _, _, _) => {
                            ((source_ml_source1_data_property_value514.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                        }
                    };
                    match (source_ml_source1_comparison_operand515.clone(), self::ml_string::Text::from_units(vec![97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                } {
                    (match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value517, _, _) => {
                            ((source_ml_source1_data_property_value517.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new())))
                } else {
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSymbolRegistrationFact>::new()))
                };
                {
                    let low = 0f64;
                    {
                        let high = { let ml_reference_array124 = facts.clone(); let ml_reference_projection124 = ml_reference_array124.borrow().len() as f64; ml_reference_projection124 };
                        {
                            let low_2 = self::source_ml_temporary_read_java_script_weak_key_eligibility_loop9(value.clone(), facts.clone(), low, high);
                            {
                                let candidate = { let ml_reference_array125 = facts.clone(); let ml_reference_position125 = low_2; let ml_reference_projection125 = self::ml_array::optional_at(&*ml_reference_array125.borrow(), self::ml_array::number_index(ml_reference_position125)); ml_reference_projection125 };
                                {
                                    let fact = if {
                                        let source_ml_source1_comparison_operand522 = {
                                            let source_ml_source1_optional_chain_receiver520 = (candidate.clone(),);
                                            if match (source_ml_source1_optional_chain_receiver520.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_temporary_optional_chain8 = ((source_ml_source1_optional_chain_receiver520.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_temporary_optional_chain8.clone() {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value521, _, _) => {
                                                            source_ml_source1_data_property_value521.clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::ml::Big>
                                            }
                                        };
                                        match (source_ml_source1_comparison_operand522.clone(), source_ml_source1_f447.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                    } {
                                        candidate.clone()
                                    } else {
                                        None::<self::JavaScriptSymbolRegistrationFact>
                                    };
                                    if match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121])
                                    } else {
                                        if match fact.clone().expect("a proven present value") {
                                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source1_data_property_value524, _) => {
                                                source_ml_source1_data_property_value524
                                            }
                                        } {
                                            self::ml_string::Text::from_units(vec![105, 110, 101, 108, 105, 103, 105, 98, 108, 101])
                                        } else {
                                            self::ml_string::Text::from_units(vec![101, 108, 105, 103, 105, 98, 108, 101])
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Object(_, _) => {
            self::ml_string::Text::from_units(vec![101, 108, 105, 103, 105, 98, 108, 101])
        }
        _ => {
            self::ml_string::Text::from_units(vec![105, 110, 101, 108, 105, 103, 105, 98, 108, 101])
        }
    }
}

pub fn read_java_script_literal_property_index(key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>) -> f64 {
    if ((({ let ml_reference_array126 = key.clone(); let ml_reference_projection126 = ml_reference_array126.borrow().len() as f64; ml_reference_projection126 }) == (0f64)) || (({ let ml_reference_array127 = key.clone(); let ml_reference_projection127 = ml_reference_array127.borrow().len() as f64; ml_reference_projection127 }) > (10f64))) {
        (-1f64)
    } else {
        if ((({ let ml_reference_array128 = key.clone(); let ml_reference_projection128 = ml_reference_array128.borrow().len() as f64; ml_reference_projection128 }) > (1f64)) && {
            let source_ml_source1_comparison_operand527 = {
                let source_ml_source1_array_read_index526 = 0f64;
                { let ml_reference_array129 = key.clone(); let ml_reference_position129 = source_ml_source1_array_read_index526; let ml_reference_projection129 = self::ml_array::optional_at(&*ml_reference_array129.borrow(), self::ml_array::number_index(ml_reference_position129)); ml_reference_projection129 }
            };
            {
                let source_ml_source1_comparison_operand528 = 48f64;
                match (source_ml_source1_comparison_operand527.clone(), source_ml_source1_comparison_operand528) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }
        }) {
            (-1f64)
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s529 = self::source_ml_temporary_read_java_script_literal_property_index_loop10(key.clone(), value, at);
                        match source_ml_source1_s529.clone() {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Done(value_2) => {
                                if (value_2 <= (4294967294f64)) {
                                    value_2
                                } else {
                                    (-1f64)
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Return(source_ml_temporary_result10) => {
                                source_ml_temporary_result10
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_literal_property_form(kind: self::ml_string::Text, index: f64) -> self::ml_string::Text {
    if ((((kind == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121]))) || (kind == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101])))) || (kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) && (index >= (0f64))) {
        self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
    } else {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])
    }
}

pub fn read_java_script_string_index_number_form(kind: self::ml_string::Text, element: self::ml_string::Text) -> self::ml_string::Text {
    if (kind == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else {
        if ((kind == (self::ml_string::Text::from_units(vec![105, 110, 116]))) || (kind == (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
            self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
        } else {
            if ((kind == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))) || ((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (element == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))) {
                self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
            } else {
                if ((kind == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]))) && (((element == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116]))) || (element == (self::ml_string::Text::from_units(vec![105, 110, 116])))) || (element == (self::ml_string::Text::from_units(vec![110, 97, 116]))))) {
                    self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
                } else {
                    self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
                }
            }
        }
    }
}

pub fn find_java_script_heap_directory_object(objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array130 = objects.clone(); let ml_reference_projection130 = ml_reference_array130.borrow().len() as f64; ml_reference_projection130 };
            {
                let source_ml_source1_s530 = self::source_ml_temporary_find_java_script_heap_directory_object_loop11(objects.clone(), identity.clone(), low, high);
                match source_ml_source1_s530.clone() {
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Return(source_ml_temporary_result11) => {
                        source_ml_temporary_result11
                    }
                }
            }
        }
    }
}

pub fn test_java_script_heap_directory_symbol(symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, identity: self::ml::Big) -> bool {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array131 = symbols.clone(); let ml_reference_projection131 = ml_reference_array131.borrow().len() as f64; ml_reference_projection131 };
            {
                let source_ml_source1_s531 = self::source_ml_temporary_test_java_script_heap_directory_symbol_loop12(symbols.clone(), identity.clone(), low, high);
                match source_ml_source1_s531.clone() {
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Return(source_ml_temporary_result12) => {
                        source_ml_temporary_result12
                    }
                }
            }
        }
    }
}

pub fn read_java_script_heap_directory_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, _, _, _, _) => {
            {
                let error = self::read_java_script_semantic_value_error(value.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    match value.clone() {
                        self::JavaScriptSemanticValue::Symbol(identity) => {
                            if self::test_java_script_heap_directory_symbol(shared_constructor_field2.clone(), identity.clone()) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f538, source_ml_source1_f539) => {
                            {
                                let at = self::find_java_script_heap_directory_object(shared_constructor_field1.clone(), source_ml_source1_f538.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = {
                                            let source_ml_source1_array_read_value599 = shared_constructor_field1.clone();
                                            { let ml_reference_array132 = source_ml_source1_array_read_value599.clone(); let ml_reference_position132 = at; let ml_reference_projection132 = self::ml_array::optional_at(&*ml_reference_array132.borrow(), self::ml_array::number_index(ml_reference_position132)); ml_reference_projection132 }
                                        };
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source1_data_property_value614, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value614
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source1_data_property_value615, _, _, _, _) => {
                                                source_ml_source1_data_property_value615
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source1_data_property_value616, _) => {
                                                source_ml_source1_data_property_value616
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source1_data_property_value617, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value617
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source1_data_property_value618, _, _) => {
                                                source_ml_source1_data_property_value618
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source1_data_property_value619, _, _) => {
                                                source_ml_source1_data_property_value619
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source1_data_property_value620, _, _) => {
                                                source_ml_source1_data_property_value620
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source1_data_property_value621, _, _) => {
                                                source_ml_source1_data_property_value621
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source1_data_property_value622, _) => {
                                                source_ml_source1_data_property_value622
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source1_data_property_value623, _, _, _) => {
                                                source_ml_source1_data_property_value623
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source1_data_property_value624, _, _, _) => {
                                                source_ml_source1_data_property_value624
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source1_data_property_value625, _, _, _) => {
                                                source_ml_source1_data_property_value625
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source1_data_property_value626, _, _, _) => {
                                                source_ml_source1_data_property_value626
                                            }
                                        }) == source_ml_source1_f539)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

pub fn read_java_script_heap_directory_key_error(heap: self::JavaScriptOrdinaryHeap, key: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    if ((match key.clone() {
        self::JavaScriptSemanticValue::Text(_) => {
            false
        }
        _ => {
            true
        }
    } && match key.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => {
            false
        }
        _ => {
            true
        }
    }) && match key.clone() {
        self::JavaScriptSemanticValue::Symbol(_) => {
            false
        }
        _ => {
            true
        }
    }) {
        self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 101, 120, 116, 32, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 119, 105, 116, 104, 111, 117, 116, 32, 105, 109, 112, 108, 105, 99, 105, 116, 32, 99, 111, 101, 114, 99, 105, 111, 110])
    } else {
        self::read_java_script_heap_directory_value_error(heap.clone(), key.clone())
    }
}

pub fn find_java_script_heap_directory_property(properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_property_descriptor_entry(properties.clone(), key.clone())
}

pub fn read_java_script_array_length_key() -> self::JavaScriptSemanticValue {
    self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64])))
}

pub fn test_java_script_array_length_key(key: self::JavaScriptSemanticValue) -> bool {
    self::test_java_script_semantic_same_value(key.clone(), self::read_java_script_array_length_key())
}
