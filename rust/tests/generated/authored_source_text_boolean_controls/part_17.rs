pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ({ let ml_reference_array308 = units.clone(); let ml_reference_projection308 = ml_reference_array308.borrow().len() as f64; ml_reference_projection308 })) {
            {
                let unit = ({ let ml_reference_array309 = units.clone(); let ml_reference_position309 = at; let ml_reference_projection309 = self::ml_array::optional_at(&*ml_reference_array309.borrow(), self::ml_array::number_index(ml_reference_position309)); ml_reference_projection309 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}

pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop2(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if (at < length) {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Text(units) => {
                        ({ let ml_reference_array310 = units.clone(); let ml_reference_position310 = at; let ml_reference_projection310 = self::ml_array::optional_at(&*ml_reference_array310.borrow(), self::ml_array::number_index(ml_reference_position310)); ml_reference_projection310 }).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f16004) => {
                        {
                            let text = source_ml_source1_f16004.clone();
                            {
                                let source_ml_source1_string_unit_operand16058 = text.clone();
                                {
                                    let source_ml_source1_string_unit_operand16059 = at;
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
                                                let source_ml_source1_string_unit_operand16065 = {
                                                    let source_ml_source1_string_range_operand16060 = text_2.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand16061 = index;
                                                        {
                                                            let source_ml_source1_string_range_operand16062 = (index + 1f64);
                                                            {
                                                                let source_ml_source1_string_range_position16063 = source_ml_source1_string_range_operand16061;
                                                                {
                                                                    let source_ml_source1_string_range_position16064 = source_ml_source1_string_range_operand16062;
                                                                    source_ml_source1_string_range_operand16060.slice(source_ml_source1_string_range_position16063, source_ml_source1_string_range_position16064)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_string_unit_operand16066 = 0f64;
                                                    source_ml_source1_string_unit_operand16065.code_point_at(source_ml_source1_string_unit_operand16066)
                                                }
                                            }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    } }))(source_ml_source1_string_unit_operand16058.clone(), source_ml_source1_string_unit_operand16059)
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
                            ({ let ml_reference_array311 = units_2.clone(); let ml_reference_position311 = at; let ml_reference_projection311 = self::ml_array::optional_at(&*ml_reference_array311.borrow(), self::ml_array::number_index(ml_reference_position311)); ml_reference_projection311 }).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f16087) => {
                            {
                                let text_2 = source_ml_source1_f16087.clone();
                                {
                                    let source_ml_source1_string_unit_operand16141 = text_2.clone();
                                    {
                                        let source_ml_source1_string_unit_operand16142 = at;
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
                                                    let source_ml_source1_string_unit_operand16148 = {
                                                        let source_ml_source1_string_range_operand16143 = text_3.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand16144 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand16145 = (index + 1f64);
                                                                {
                                                                    let source_ml_source1_string_range_position16146 = source_ml_source1_string_range_operand16144;
                                                                    {
                                                                        let source_ml_source1_string_range_position16147 = source_ml_source1_string_range_operand16145;
                                                                        source_ml_source1_string_range_operand16143.slice(source_ml_source1_string_range_position16146, source_ml_source1_string_range_position16147)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand16149 = 0f64;
                                                        source_ml_source1_string_unit_operand16148.code_point_at(source_ml_source1_string_unit_operand16149)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source1_string_unit_operand16141.clone(), source_ml_source1_string_unit_operand16142)
                                    }
                                }
                            }
                        }
                        _ => {
                            (-1f64)
                        }
                    };
                    if (a != b) {
                        self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
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
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

pub fn source_ml_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, mut source_ml_temporary_values3: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_temporary_index3 < ({ let ml_reference_array312 = source_ml_temporary_values3.clone(); let ml_reference_projection312 = self::ml::Big::from_u128(ml_reference_array312.borrow().len() as u128); ml_reference_projection312 })) {
            {
                let unit = { let ml_reference_array313 = source_ml_temporary_values3.clone(); let ml_reference_position313 = source_ml_temporary_index3.clone(); let ml_reference_projection313 = self::ml_array::at(&*ml_reference_array313.borrow(), ml_reference_position313.to_index()); ml_reference_projection313 };
                {
                    let text_2 = text.concat(&self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![unit]).into_iter().map(|ml_unit| ml_unit as u16).collect()));
                    {
                        let source_ml_temporary_index3_2 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_temporary_index3) = (text_2.clone(), source_ml_temporary_index3_2.clone());
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

pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < length) {
            {
                let units_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value314 = units.borrow().clone(); ml_reference_value314 }, vec![self::read_java_script_semantic_text_unit(value.clone(), at)])));
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done,
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(f64),
}

pub fn source_ml_temporary_find_java_script_property_descriptor_entry_loop6(mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    loop {
        return if (at < ({ let ml_reference_array315 = properties.clone(); let ml_reference_projection315 = ml_reference_array315.borrow().len() as f64; ml_reference_projection315 })) {
            {
                let property = { let ml_reference_array316 = properties.clone(); let ml_reference_position316 = at; let ml_reference_projection316 = self::ml_array::optional_at(&*ml_reference_array316.borrow(), self::ml_array::number_index(ml_reference_position316)); ml_reference_projection316 };
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value16171, _) => {
                        let source_ml_source1_data_property_value16171 = *source_ml_source1_data_property_value16171;
                        source_ml_source1_data_property_value16171.clone()
                    }
                }, key.clone())) {
                    self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(at)
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
            self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin7 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>>),
}

pub fn source_ml_temporary_read_java_script_weak_key_eligibility_loop9(mut value: self::JavaScriptSemanticValue, mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>, mut low: f64, mut high: f64) -> f64 {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let row = { let ml_reference_array317 = facts.clone(); let ml_reference_position317 = middle; let ml_reference_projection317 = self::ml_array::optional_at(&*ml_reference_array317.borrow(), self::ml_array::number_index(ml_reference_position317)); ml_reference_projection317 };
                    if (!(match (row.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                        let source_ml_source1_comparison_operand16177 = match row.clone().expect("a proven present value") {
                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value16174, _, _) => {
                                source_ml_source1_data_property_value16174.clone()
                            }
                        };
                        {
                            let source_ml_source1_comparison_operand16178 = match value.clone() {
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_data_property_value16175) => {
                                    Some(source_ml_source1_data_property_value16175.clone())
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_data_property_value16176, _) => {
                                    Some(source_ml_source1_data_property_value16176.clone())
                                }
                                _ => {
                                    None::<self::ml::Big>
                                }
                            };
                            if match (source_ml_source1_comparison_operand16178.clone(), None::<self::ml::Big>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                                {
                                    let source_ml_source1_comparison_order16179 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand16177.clone());
                                    (source_ml_source1_comparison_order16179 == (1f64))
                                }
                            } else {
                                (source_ml_source1_comparison_operand16177 < (source_ml_source1_comparison_operand16178.clone().expect("a proven present value")))
                            }
                        }
                    }) {
                        {
                            let low_2 = (middle + 1f64);
                            {
                                low = low_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            high = middle;
                            continue;
                        }
                    }
                }
            }
        } else {
            low
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Return(f64),
}

pub fn source_ml_temporary_read_java_script_literal_property_index_loop10(mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut value: f64, mut at: f64) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Result {
    loop {
        return if (at < ({ let ml_reference_array318 = key.clone(); let ml_reference_projection318 = ml_reference_array318.borrow().len() as f64; ml_reference_projection318 })) {
            {
                let digit = (match { let ml_reference_array319 = key.clone(); let ml_reference_position319 = at; let ml_reference_projection319 = self::ml_array::optional_at(&*ml_reference_array319.borrow(), self::ml_array::number_index(ml_reference_position319)); ml_reference_projection319 } { Some(ml_present) => ml_present, None => f64::NAN } - 48f64);
                if ((!(self::ml_math::is_integer(digit)) || (digit < (0f64))) || (digit > (9f64))) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Return((-1f64))
                } else {
                    {
                        let value_2 = ((value * 10f64) + digit);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop10Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Result {
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Done,
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Return(f64),
}

pub fn source_ml_temporary_find_java_script_heap_directory_object_loop11(mut objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = { let ml_reference_array320 = objects.clone(); let ml_reference_position320 = middle; let ml_reference_projection320 = self::ml_array::optional_at(&*ml_reference_array320.borrow(), self::ml_array::number_index(ml_reference_position320)); ml_reference_projection320 };
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value16197, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16197.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value16198, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16198.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value16199, _, _, _, _) => {
                                source_ml_source1_data_property_value16199.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value16200, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16200.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value16201, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16201.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value16202, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16202.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value16203, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16203.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value16204, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16204.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value16205, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16205.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value16206, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16206.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value16207, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16207.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value16208, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16208.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value16209, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value16209.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value16223, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16223.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value16224, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16224.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value16225, _, _, _, _) => {
                                    source_ml_source1_data_property_value16225.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value16226, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16226.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value16227, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16227.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value16228, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16228.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value16229, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16229.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value16230, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16230.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value16231, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16231.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value16232, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16232.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value16233, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16233.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value16234, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16234.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value16235, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16235.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Result {
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Done,
    MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Return(bool),
}

pub fn source_ml_temporary_test_java_script_heap_directory_symbol_loop12(mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = { let ml_reference_array321 = symbols.clone(); let ml_reference_position321 = middle; let ml_reference_projection321 = self::ml_array::optional_at(&*ml_reference_array321.borrow(), self::ml_array::number_index(ml_reference_position321)); ml_reference_projection321 };
                    if {
                        let source_ml_source1_comparison_operand16239 = None::<()>;
                        match (symbol.clone(), source_ml_source1_comparison_operand16239.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    } {
                        self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Return(true)
                        } else {
                            if ((symbol.clone().expect("a proven present value")) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptArrayObjectErrorLoop14Result {
    MlTemporaryReadJavaScriptArrayObjectErrorLoop14Done,
    MlTemporaryReadJavaScriptArrayObjectErrorLoop14Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_array_object_error_loop14(mut length: f64, mut source_ml_temporary_values13: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index13: self::ml::Big) -> self::MlTemporaryReadJavaScriptArrayObjectErrorLoop14Result {
    loop {
        return if (source_ml_temporary_index13 < ({ let ml_reference_array322 = source_ml_temporary_values13.clone(); let ml_reference_projection322 = self::ml::Big::from_u128(ml_reference_array322.borrow().len() as u128); ml_reference_projection322 })) {
            {
                let entry = { let ml_reference_array323 = source_ml_temporary_values13.clone(); let ml_reference_position323 = source_ml_temporary_index13.clone(); let ml_reference_projection323 = self::ml_array::at(&*ml_reference_array323.borrow(), ml_reference_position323.to_index()); ml_reference_projection323 };
                {
                    let index = self::read_java_script_array_index(match entry.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value16242, _) => {
                            let source_ml_source1_data_property_value16242 = *source_ml_source1_data_property_value16242;
                            source_ml_source1_data_property_value16242.clone()
                        }
                    });
                    if (index >= length) {
                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop14Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop14Return(self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 98, 101, 108, 111, 119, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104]))
                    } else {
                        {
                            let source_ml_temporary_index13_2 = source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index13 = source_ml_temporary_index13_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop14Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop14Done
        };
    }
}

pub fn source_ml_temporary_plan_java_script_array_ordinary_definition_loop19(mut at: f64, mut replacement: self::JavaScriptOrdinaryProperty, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut index: f64, mut source_ml_temporary_values18: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index18: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>> {
    loop {
        return if (source_ml_temporary_index18 < ({ let ml_reference_array324 = source_ml_temporary_values18.clone(); let ml_reference_projection324 = self::ml::Big::from_u128(ml_reference_array324.borrow().len() as u128); ml_reference_projection324 })) {
            {
                let entry = { let ml_reference_array325 = source_ml_temporary_values18.clone(); let ml_reference_position325 = source_ml_temporary_index18.clone(); let ml_reference_projection325 = self::ml_array::at(&*ml_reference_array325.borrow(), ml_reference_position325.to_index()); ml_reference_projection325 };
                {
                    let properties_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value326 = properties.borrow().clone(); ml_reference_value326 }, vec![if (index == at) {
                        replacement.clone()
                    } else {
                        entry.clone()
                    }])));
                    {
                        let index_2 = (index + 1f64);
                        {
                            let source_ml_temporary_index18_2 = source_ml_temporary_index18.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, index, source_ml_temporary_index18) = (properties_2.clone(), index_2, source_ml_temporary_index18_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            properties.clone()
        };
    }
}

pub fn source_ml_temporary_plan_java_script_array_length_definition_loop22(mut unsigned_length: f64, mut blocked: f64, mut source_ml_temporary_values20: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index20: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index20 < ({ let ml_reference_array327 = source_ml_temporary_values20.clone(); let ml_reference_projection327 = self::ml::Big::from_u128(ml_reference_array327.borrow().len() as u128); ml_reference_projection327 })) {
            {
                let property = { let ml_reference_array328 = source_ml_temporary_values20.clone(); let ml_reference_position328 = source_ml_temporary_index20.clone(); let ml_reference_projection328 = self::ml_array::at(&*ml_reference_array328.borrow(), ml_reference_position328.to_index()); ml_reference_projection328 };
                match property.clone() {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(shared_constructor_field1, shared_constructor_field2) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let index = self::read_java_script_array_index(shared_constructor_field1.clone());
                            if (((index >= unsigned_length) && !(({ let ml_reference_value329 = shared_constructor_field2.borrow().clone(); ml_reference_value329 }).0)) && (index > blocked)) {
                                {
                                    let source_ml_temporary_index20_2 = source_ml_temporary_index20.add(&self::ml::Big::from_i128(1));
                                    {
                                        (blocked, source_ml_temporary_index20) = (index, source_ml_temporary_index20_2.clone());
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let source_ml_temporary_index20_3 = source_ml_temporary_index20.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index20 = source_ml_temporary_index20_3.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            blocked
        };
    }
}

pub fn source_ml_temporary_plan_java_script_array_length_definition_loop23(mut new_writable: bool, mut final_length: f64, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_values21: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index21: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>> {
    loop {
        return if (source_ml_temporary_index21 < ({ let ml_reference_array330 = source_ml_temporary_values21.clone(); let ml_reference_projection330 = self::ml::Big::from_u128(ml_reference_array330.borrow().len() as u128); ml_reference_projection330 })) {
            {
                let property = { let ml_reference_array331 = source_ml_temporary_values21.clone(); let ml_reference_position331 = source_ml_temporary_index21.clone(); let ml_reference_projection331 = self::ml_array::at(&*ml_reference_array331.borrow(), ml_reference_position331.to_index()); ml_reference_projection331 };
                match property.clone() {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(shared_constructor_field1, shared_constructor_field2) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let index = self::read_java_script_array_index(shared_constructor_field1.clone());
                            if ((index < (0f64)) || (index < final_length)) {
                                {
                                    let properties_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value332 = properties.borrow().clone(); ml_reference_value332 }, vec![if self::test_java_script_array_length_key(shared_constructor_field1.clone()) {
                                        {
                                            let source_ml_source1_nominal_field16270 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source1_nominal_field16272 = shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source1_nominal_override16274 = std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source1_spread16254 = { let ml_reference_value333 = shared_constructor_field2.borrow().clone(); ml_reference_value333 };
                                                        {
                                                            let source_ml_source1_spread_field16255 = (source_ml_source1_spread16254.clone()).0;
                                                            {
                                                                let source_ml_source1_spread_field16256 = (source_ml_source1_spread16254.clone()).1;
                                                                {
                                                                    let source_ml_source1_spread_field16257 = (source_ml_source1_spread16254.clone()).2;
                                                                    {
                                                                        let source_ml_source1_spread_field16258 = (source_ml_source1_spread16254.clone()).3;
                                                                        {
                                                                            let source_ml_source1_spread_field16259 = (source_ml_source1_spread16254.clone()).4;
                                                                            {
                                                                                let source_ml_source1_spread_field16260 = (source_ml_source1_spread16254.clone()).5;
                                                                                {
                                                                                    let source_ml_source1_spread_field16261 = (source_ml_source1_spread16254.clone()).6;
                                                                                    {
                                                                                        let source_ml_source1_spread_field16262 = (source_ml_source1_spread16254.clone()).7;
                                                                                        {
                                                                                            let source_ml_source1_spread_field16263 = (source_ml_source1_spread16254.clone()).8;
                                                                                            {
                                                                                                let source_ml_source1_spread_field16264 = (source_ml_source1_spread16254.clone()).9;
                                                                                                {
                                                                                                    let source_ml_source1_spread_field16265 = (source_ml_source1_spread16254.clone()).10;
                                                                                                    {
                                                                                                        let source_ml_source1_spread_field16266 = (source_ml_source1_spread16254.clone()).11;
                                                                                                        {
                                                                                                            let source_ml_source1_field16267 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                            (source_ml_source1_spread_field16255, source_ml_source1_spread_field16256, source_ml_source1_spread_field16257, source_ml_source1_spread_field16258, source_ml_source1_spread_field16259.clone(), source_ml_source1_spread_field16260, source_ml_source1_spread_field16261.clone(), source_ml_source1_spread_field16262, source_ml_source1_field16267.clone(), source_ml_source1_spread_field16264, new_writable, source_ml_source1_spread_field16266)
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
                                                    }));
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source1_nominal_field16270.clone()), source_ml_source1_nominal_override16274.clone())
                                                }
                                            }
                                        }
                                    } else {
                                        property.clone()
                                    }])));
                                    {
                                        let source_ml_temporary_index21_2 = source_ml_temporary_index21.add(&self::ml::Big::from_i128(1));
                                        {
                                            (properties, source_ml_temporary_index21) = (properties_2.clone(), source_ml_temporary_index21_2.clone());
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let source_ml_temporary_index21_3 = source_ml_temporary_index21.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index21 = source_ml_temporary_index21_3.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            properties.clone()
        };
    }
}

pub fn source_ml_temporary_plan_java_script_array_property_definition_loop25(mut index: f64, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_values24: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_temporary_index24: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>> {
    loop {
        return if (source_ml_temporary_index24 < ({ let ml_reference_array334 = source_ml_temporary_values24.clone(); let ml_reference_projection334 = self::ml::Big::from_u128(ml_reference_array334.borrow().len() as u128); ml_reference_projection334 })) {
            {
                let property = { let ml_reference_array335 = source_ml_temporary_values24.clone(); let ml_reference_position335 = source_ml_temporary_index24.clone(); let ml_reference_projection335 = self::ml_array::at(&*ml_reference_array335.borrow(), ml_reference_position335.to_index()); ml_reference_projection335 };
                match property.clone() {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(shared_constructor_field1, shared_constructor_field2) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let properties_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value336 = properties.borrow().clone(); ml_reference_value336 }, vec![if self::test_java_script_array_length_key(shared_constructor_field1.clone()) {
                                {
                                    let source_ml_source1_nominal_field16294 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source1_nominal_field16296 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source1_nominal_override16298 = std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source1_spread16279 = { let ml_reference_value337 = shared_constructor_field2.borrow().clone(); ml_reference_value337 };
                                                {
                                                    let source_ml_source1_spread_field16280 = (source_ml_source1_spread16279.clone()).0;
                                                    {
                                                        let source_ml_source1_spread_field16281 = (source_ml_source1_spread16279.clone()).1;
                                                        {
                                                            let source_ml_source1_spread_field16282 = (source_ml_source1_spread16279.clone()).2;
                                                            {
                                                                let source_ml_source1_spread_field16283 = (source_ml_source1_spread16279.clone()).3;
                                                                {
                                                                    let source_ml_source1_spread_field16284 = (source_ml_source1_spread16279.clone()).4;
                                                                    {
                                                                        let source_ml_source1_spread_field16285 = (source_ml_source1_spread16279.clone()).5;
                                                                        {
                                                                            let source_ml_source1_spread_field16286 = (source_ml_source1_spread16279.clone()).6;
                                                                            {
                                                                                let source_ml_source1_spread_field16287 = (source_ml_source1_spread16279.clone()).7;
                                                                                {
                                                                                    let source_ml_source1_spread_field16288 = (source_ml_source1_spread16279.clone()).8;
                                                                                    {
                                                                                        let source_ml_source1_spread_field16289 = (source_ml_source1_spread16279.clone()).9;
                                                                                        {
                                                                                            let source_ml_source1_spread_field16290 = (source_ml_source1_spread16279.clone()).10;
                                                                                            {
                                                                                                let source_ml_source1_spread_field16291 = (source_ml_source1_spread16279.clone()).11;
                                                                                                {
                                                                                                    let source_ml_source1_field16292 = self::JavaScriptSemanticValue::Number((index + 1f64));
                                                                                                    (source_ml_source1_spread_field16280, source_ml_source1_spread_field16281, source_ml_source1_spread_field16282, source_ml_source1_spread_field16283, source_ml_source1_spread_field16284.clone(), source_ml_source1_spread_field16285, source_ml_source1_spread_field16286.clone(), source_ml_source1_spread_field16287, source_ml_source1_field16292.clone(), source_ml_source1_spread_field16289, source_ml_source1_spread_field16290, source_ml_source1_spread_field16291)
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
                                            }));
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source1_nominal_field16294.clone()), source_ml_source1_nominal_override16298.clone())
                                        }
                                    }
                                }
                            } else {
                                property.clone()
                            }])));
                            {
                                let source_ml_temporary_index24_2 = source_ml_temporary_index24.add(&self::ml::Big::from_i128(1));
                                {
                                    (properties, source_ml_temporary_index24) = (properties_2.clone(), source_ml_temporary_index24_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            properties.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptCollectionKeyJoin26 {
    MlTemporaryTestJavaScriptCollectionKeyJoin26Next,
    MlTemporaryTestJavaScriptCollectionKeyJoin26Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryNormalizeJavaScriptCollectionKeyJoin27 {
    MlTemporaryNormalizeJavaScriptCollectionKeyJoin27Next,
    MlTemporaryNormalizeJavaScriptCollectionKeyJoin27Return(Box<self::JavaScriptSemanticValue>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysLoop29Result {
    MlTemporaryCompareJavaScriptCollectionKeysLoop29Done,
    MlTemporaryCompareJavaScriptCollectionKeysLoop29Return(f64),
}

pub fn source_ml_temporary_compare_java_script_collection_keys_loop29(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut index: f64) -> self::MlTemporaryCompareJavaScriptCollectionKeysLoop29Result {
    loop {
        return if (index < length) {
            {
                let x = self::read_java_script_semantic_text_unit(left.clone(), index);
                {
                    let y = self::read_java_script_semantic_text_unit(right.clone(), index);
                    if (x != y) {
                        self::MlTemporaryCompareJavaScriptCollectionKeysLoop29Result::MlTemporaryCompareJavaScriptCollectionKeysLoop29Return(if (x < y) {
                            (-1f64)
                        } else {
                            1f64
                        })
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
            }
        } else {
            self::MlTemporaryCompareJavaScriptCollectionKeysLoop29Result::MlTemporaryCompareJavaScriptCollectionKeysLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareJavaScriptCollectionKeysJoin28 {
    MlTemporaryCompareJavaScriptCollectionKeysJoin28Next,
    MlTemporaryCompareJavaScriptCollectionKeysJoin28Return(f64),
}
