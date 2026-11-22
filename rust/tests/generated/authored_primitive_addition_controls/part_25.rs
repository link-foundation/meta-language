pub fn source_ml_source1_temporary_map_unicode_case_utf16_columns_loop56(mut cased: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut ignorable: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut points: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut following: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut next: bool, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<bool>>> {
    loop {
        return if (at >= (0f64)) {
            {
                let point = ({ let ml_reference_array477 = points.clone(); let ml_reference_position477 = at; let ml_reference_projection477 = self::ml_array::optional_at(&*ml_reference_array477.borrow(), self::ml_array::number_index(ml_reference_position477)); ml_reference_projection477 }).clone().unwrap_or_else(|| 0f64);
                {
                    let following_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value478 = following.borrow().clone(); ml_reference_value478 }, vec![next])));
                    if !(self::accept_unicode_case_property(point, ignorable.clone())) {
                        {
                            let next_2 = self::accept_unicode_case_property(point, cased.clone());
                            {
                                let at_2 = (at - 1f64);
                                {
                                    (following, next, at) = (following_2.clone(), next_2, at_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let at_3 = (at - 1f64);
                            {
                                (following, at) = (following_2.clone(), at_3);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            following.clone()
        };
    }
}

pub fn source_ml_source1_temporary_map_unicode_case_utf16_columns_loop58(mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut mapped: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (index < ({ let ml_reference_array479 = mapped.clone(); let ml_reference_projection479 = ml_reference_array479.borrow().len() as f64; ml_reference_projection479 })) {
            {
                let value = ({ let ml_reference_array480 = mapped.clone(); let ml_reference_position480 = index; let ml_reference_projection480 = self::ml_array::optional_at(&*ml_reference_array480.borrow(), self::ml_array::number_index(ml_reference_position480)); ml_reference_projection480 }).clone().unwrap_or_else(|| 0f64);
                if (value < (65536f64)) {
                    {
                        let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value481 = result.borrow().clone(); ml_reference_value481 }, vec![value])));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (result, index) = (result_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let result_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value482 = result.borrow().clone(); ml_reference_value482 }, vec![(55296f64 + (((value - 65536f64) / 1024f64)).floor())])));
                        {
                            let result_4 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value483 = result_3.borrow().clone(); ml_reference_value483 }, vec![(56320f64 + ((value - 65536f64) % 1024f64))])));
                            {
                                let index_3 = (index + 1f64);
                                {
                                    (result, index) = (result_4.clone(), index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_map_unicode_case_utf16_columns_loop57(mut upper: bool, mut ranges: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut indexes: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut cased: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut ignorable: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut points: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut following: std::rc::Rc<std::cell::RefCell<Vec<bool>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut previous: bool, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < ({ let ml_reference_array484 = points.clone(); let ml_reference_projection484 = ml_reference_array484.borrow().len() as f64; ml_reference_projection484 })) {
            {
                let point = ({ let ml_reference_array485 = points.clone(); let ml_reference_position485 = at; let ml_reference_projection485 = self::ml_array::optional_at(&*ml_reference_array485.borrow(), self::ml_array::number_index(ml_reference_position485)); ml_reference_projection485 }).clone().unwrap_or_else(|| 0f64);
                {
                    let final_sigma = (((!upper && (point == (931f64))) && previous) && !(({
                        let source_ml_source2_array_read_index3395 = (({ let ml_reference_array486 = points.clone(); let ml_reference_projection486 = ml_reference_array486.borrow().len() as f64; ml_reference_projection486 } - 1f64) - at);
                        { let ml_reference_array487 = following.clone(); let ml_reference_position487 = source_ml_source2_array_read_index3395; let ml_reference_projection487 = self::ml_array::optional_at(&*ml_reference_array487.borrow(), self::ml_array::number_index(ml_reference_position487)); ml_reference_projection487 }
                    }).clone().unwrap_or_else(|| false)));
                    {
                        let mapped = if final_sigma {
                            std::rc::Rc::new(std::cell::RefCell::new(vec![962f64]))
                        } else {
                            self::read_unicode_case_mapping(point, ranges.clone(), indexes.clone(), values.clone())
                        };
                        {
                            let index = 0f64;
                            {
                                let result_2 = self::source_ml_source1_temporary_map_unicode_case_utf16_columns_loop58(result.clone(), mapped.clone(), index);
                                if !(self::accept_unicode_case_property(point, ignorable.clone())) {
                                    {
                                        let previous_2 = self::accept_unicode_case_property(point, cased.clone());
                                        {
                                            let at_2 = (at + 1f64);
                                            {
                                                (result, previous, at) = (result_2.clone(), previous_2, at_2);
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at_3 = (at + 1f64);
                                        {
                                            (result, at) = (result_2.clone(), at_3);
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
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Result {
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Done,
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop59(mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Result {
    loop {
        return if (at < ({ let ml_reference_array488 = properties.clone(); let ml_reference_projection488 = ml_reference_array488.borrow().len() as f64; ml_reference_projection488 })) {
            {
                let property = { let ml_reference_array489 = properties.clone(); let ml_reference_position489 = at; let ml_reference_projection489 = self::ml_array::optional_at(&*ml_reference_array489.borrow(), self::ml_array::number_index(ml_reference_position489)); ml_reference_projection489 };
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value3398, _) => {
                        let source_ml_source2_data_property_value3398 = *source_ml_source2_data_property_value3398;
                        source_ml_source2_data_property_value3398.clone()
                    }
                }, key.clone())) {
                    self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Return(at)
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
            self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop59Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60 {
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Next,
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin60Return(std::rc::Rc<std::cell::RefCell<(std::rc::Rc<std::cell::RefCell<(bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)>>, self::ml_string::Text, self::ml_string::Text, std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>)>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Result {
    MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Done(f64),
    MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Return(f64),
}

pub fn source_ml_source1_temporary_read_java_script_literal_property_index_loop61(mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut value: f64, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Result {
    loop {
        return if (at < ({ let ml_reference_array490 = key.clone(); let ml_reference_projection490 = ml_reference_array490.borrow().len() as f64; ml_reference_projection490 })) {
            {
                let digit = (match { let ml_reference_array491 = key.clone(); let ml_reference_position491 = at; let ml_reference_projection491 = self::ml_array::optional_at(&*ml_reference_array491.borrow(), self::ml_array::number_index(ml_reference_position491)); ml_reference_projection491 } { Some(ml_present) => ml_present, None => f64::NAN } - 48f64);
                if ((!(self::ml_math::is_integer(digit)) || (digit < (0f64))) || (digit > (9f64))) {
                    self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Result::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Return((-1f64))
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
            self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Result::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop61Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Result {
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Done,
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Return(f64),
}

pub fn source_ml_source1_temporary_find_java_script_heap_directory_object_loop62(mut objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = { let ml_reference_array492 = objects.clone(); let ml_reference_position492 = middle; let ml_reference_projection492 = self::ml_array::optional_at(&*ml_reference_array492.borrow(), self::ml_array::number_index(ml_reference_position492)); ml_reference_projection492 };
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value3416, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3416.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value3417, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3417.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value3418, _, _, _, _) => {
                                source_ml_source2_data_property_value3418.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value3419, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3419.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value3420, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3420.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value3421, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3421.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value3422, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3422.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value3423, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3423.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value3424, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3424.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value3425, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3425.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value3426, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3426.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value3427, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3427.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value3428, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value3428.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value3442, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3442.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value3443, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3443.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value3444, _, _, _, _) => {
                                    source_ml_source2_data_property_value3444.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value3445, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3445.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value3446, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3446.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value3447, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3447.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value3448, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3448.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value3449, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3449.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value3450, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3450.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value3451, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3451.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value3452, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3452.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value3453, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3453.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value3454, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value3454.clone()
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
            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop62Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Result {
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Done,
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Return(bool),
}

pub fn source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop63(mut symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = { let ml_reference_array493 = symbols.clone(); let ml_reference_position493 = middle; let ml_reference_projection493 = self::ml_array::optional_at(&*ml_reference_array493.borrow(), self::ml_array::number_index(ml_reference_position493)); ml_reference_projection493 };
                    if {
                        let source_ml_source2_comparison_operand3458 = None::<()>;
                        match (symbol.clone(), source_ml_source2_comparison_operand3458.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    } {
                        self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Return(true)
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
            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop63Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Result {
    MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Done,
    MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Return(self::ml_string::Text),
}

pub fn source_ml_source1_temporary_read_java_script_array_object_error_loop65(mut length: f64, mut source_ml_source1_temporary_values64: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_source1_temporary_index64: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Result {
    loop {
        return if (source_ml_source1_temporary_index64 < ({ let ml_reference_array494 = source_ml_source1_temporary_values64.clone(); let ml_reference_projection494 = self::ml::Big::from_u128(ml_reference_array494.borrow().len() as u128); ml_reference_projection494 })) {
            {
                let entry = { let ml_reference_array495 = source_ml_source1_temporary_values64.clone(); let ml_reference_position495 = source_ml_source1_temporary_index64.clone(); let ml_reference_projection495 = self::ml_array::at(&*ml_reference_array495.borrow(), ml_reference_position495.to_index()); ml_reference_projection495 };
                {
                    let index = self::read_java_script_array_index(match entry.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value3461, _) => {
                            let source_ml_source2_data_property_value3461 = *source_ml_source2_data_property_value3461;
                            source_ml_source2_data_property_value3461.clone()
                        }
                    });
                    if (index >= length) {
                        self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 98, 101, 108, 111, 119, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104]))
                    } else {
                        {
                            let source_ml_source1_temporary_index64_2 = source_ml_source1_temporary_index64.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index64 = source_ml_source1_temporary_index64_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptArrayObjectErrorLoop65Done
        };
    }
}

pub fn source_ml_source1_temporary_plan_java_script_array_ordinary_definition_loop70(mut at: f64, mut replacement: self::JavaScriptOrdinaryProperty, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut index: f64, mut source_ml_source1_temporary_values69: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_source1_temporary_index69: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>> {
    loop {
        return if (source_ml_source1_temporary_index69 < ({ let ml_reference_array496 = source_ml_source1_temporary_values69.clone(); let ml_reference_projection496 = self::ml::Big::from_u128(ml_reference_array496.borrow().len() as u128); ml_reference_projection496 })) {
            {
                let entry = { let ml_reference_array497 = source_ml_source1_temporary_values69.clone(); let ml_reference_position497 = source_ml_source1_temporary_index69.clone(); let ml_reference_projection497 = self::ml_array::at(&*ml_reference_array497.borrow(), ml_reference_position497.to_index()); ml_reference_projection497 };
                {
                    let properties_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value498 = properties.borrow().clone(); ml_reference_value498 }, vec![if (index == at) {
                        replacement.clone()
                    } else {
                        entry.clone()
                    }])));
                    {
                        let index_2 = (index + 1f64);
                        {
                            let source_ml_source1_temporary_index69_2 = source_ml_source1_temporary_index69.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, index, source_ml_source1_temporary_index69) = (properties_2.clone(), index_2, source_ml_source1_temporary_index69_2.clone());
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

pub fn source_ml_source1_temporary_plan_java_script_array_length_definition_loop73(mut unsigned_length: f64, mut blocked: f64, mut source_ml_source1_temporary_values71: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_source1_temporary_index71: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index71 < ({ let ml_reference_array499 = source_ml_source1_temporary_values71.clone(); let ml_reference_projection499 = self::ml::Big::from_u128(ml_reference_array499.borrow().len() as u128); ml_reference_projection499 })) {
            {
                let property = { let ml_reference_array500 = source_ml_source1_temporary_values71.clone(); let ml_reference_position500 = source_ml_source1_temporary_index71.clone(); let ml_reference_projection500 = self::ml_array::at(&*ml_reference_array500.borrow(), ml_reference_position500.to_index()); ml_reference_projection500 };
                match property.clone() {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(shared_constructor_field1, shared_constructor_field2) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let index = self::read_java_script_array_index(shared_constructor_field1.clone());
                            if (((index >= unsigned_length) && !(({ let ml_reference_value501 = shared_constructor_field2.borrow().clone(); ml_reference_value501 }).0)) && (index > blocked)) {
                                {
                                    let source_ml_source1_temporary_index71_2 = source_ml_source1_temporary_index71.add(&self::ml::Big::from_i128(1));
                                    {
                                        (blocked, source_ml_source1_temporary_index71) = (index, source_ml_source1_temporary_index71_2.clone());
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source1_temporary_index71_3 = source_ml_source1_temporary_index71.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index71 = source_ml_source1_temporary_index71_3.clone();
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

pub fn source_ml_source1_temporary_plan_java_script_array_length_definition_loop74(mut new_writable: bool, mut final_length: f64, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_source1_temporary_values72: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_source1_temporary_index72: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>> {
    loop {
        return if (source_ml_source1_temporary_index72 < ({ let ml_reference_array502 = source_ml_source1_temporary_values72.clone(); let ml_reference_projection502 = self::ml::Big::from_u128(ml_reference_array502.borrow().len() as u128); ml_reference_projection502 })) {
            {
                let property = { let ml_reference_array503 = source_ml_source1_temporary_values72.clone(); let ml_reference_position503 = source_ml_source1_temporary_index72.clone(); let ml_reference_projection503 = self::ml_array::at(&*ml_reference_array503.borrow(), ml_reference_position503.to_index()); ml_reference_projection503 };
                match property.clone() {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(shared_constructor_field1, shared_constructor_field2) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let index = self::read_java_script_array_index(shared_constructor_field1.clone());
                            if ((index < (0f64)) || (index < final_length)) {
                                {
                                    let properties_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value504 = properties.borrow().clone(); ml_reference_value504 }, vec![if self::test_java_script_array_length_key(shared_constructor_field1.clone()) {
                                        {
                                            let source_ml_source2_nominal_field3489 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field3491 = shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_override3493 = std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source2_spread3473 = { let ml_reference_value505 = shared_constructor_field2.borrow().clone(); ml_reference_value505 };
                                                        {
                                                            let source_ml_source2_spread_field3474 = (source_ml_source2_spread3473.clone()).0;
                                                            {
                                                                let source_ml_source2_spread_field3475 = (source_ml_source2_spread3473.clone()).1;
                                                                {
                                                                    let source_ml_source2_spread_field3476 = (source_ml_source2_spread3473.clone()).2;
                                                                    {
                                                                        let source_ml_source2_spread_field3477 = (source_ml_source2_spread3473.clone()).3;
                                                                        {
                                                                            let source_ml_source2_spread_field3478 = (source_ml_source2_spread3473.clone()).4;
                                                                            {
                                                                                let source_ml_source2_spread_field3479 = (source_ml_source2_spread3473.clone()).5;
                                                                                {
                                                                                    let source_ml_source2_spread_field3480 = (source_ml_source2_spread3473.clone()).6;
                                                                                    {
                                                                                        let source_ml_source2_spread_field3481 = (source_ml_source2_spread3473.clone()).7;
                                                                                        {
                                                                                            let source_ml_source2_spread_field3482 = (source_ml_source2_spread3473.clone()).8;
                                                                                            {
                                                                                                let source_ml_source2_spread_field3483 = (source_ml_source2_spread3473.clone()).9;
                                                                                                {
                                                                                                    let source_ml_source2_spread_field3484 = (source_ml_source2_spread3473.clone()).10;
                                                                                                    {
                                                                                                        let source_ml_source2_spread_field3485 = (source_ml_source2_spread3473.clone()).11;
                                                                                                        {
                                                                                                            let source_ml_source2_field3486 = self::JavaScriptSemanticValue::Number(final_length);
                                                                                                            (source_ml_source2_spread_field3474, source_ml_source2_spread_field3475, source_ml_source2_spread_field3476, source_ml_source2_spread_field3477, source_ml_source2_spread_field3478.clone(), source_ml_source2_spread_field3479, source_ml_source2_spread_field3480.clone(), source_ml_source2_spread_field3481, source_ml_source2_field3486.clone(), source_ml_source2_spread_field3483, new_writable, source_ml_source2_spread_field3485)
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
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source2_nominal_field3489.clone()), source_ml_source2_nominal_override3493.clone())
                                                }
                                            }
                                        }
                                    } else {
                                        property.clone()
                                    }])));
                                    {
                                        let source_ml_source1_temporary_index72_2 = source_ml_source1_temporary_index72.add(&self::ml::Big::from_i128(1));
                                        {
                                            (properties, source_ml_source1_temporary_index72) = (properties_2.clone(), source_ml_source1_temporary_index72_2.clone());
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let source_ml_source1_temporary_index72_3 = source_ml_source1_temporary_index72.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index72 = source_ml_source1_temporary_index72_3.clone();
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

pub fn source_ml_source1_temporary_plan_java_script_array_property_definition_loop76(mut index: f64, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_source1_temporary_values75: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut source_ml_source1_temporary_index75: self::ml::Big) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>> {
    loop {
        return if (source_ml_source1_temporary_index75 < ({ let ml_reference_array506 = source_ml_source1_temporary_values75.clone(); let ml_reference_projection506 = self::ml::Big::from_u128(ml_reference_array506.borrow().len() as u128); ml_reference_projection506 })) {
            {
                let property = { let ml_reference_array507 = source_ml_source1_temporary_values75.clone(); let ml_reference_position507 = source_ml_source1_temporary_index75.clone(); let ml_reference_projection507 = self::ml_array::at(&*ml_reference_array507.borrow(), ml_reference_position507.to_index()); ml_reference_projection507 };
                match property.clone() {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(shared_constructor_field1, shared_constructor_field2) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let properties_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value508 = properties.borrow().clone(); ml_reference_value508 }, vec![if self::test_java_script_array_length_key(shared_constructor_field1.clone()) {
                                {
                                    let source_ml_source2_nominal_field3513 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field3515 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_override3517 = std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_spread3498 = { let ml_reference_value509 = shared_constructor_field2.borrow().clone(); ml_reference_value509 };
                                                {
                                                    let source_ml_source2_spread_field3499 = (source_ml_source2_spread3498.clone()).0;
                                                    {
                                                        let source_ml_source2_spread_field3500 = (source_ml_source2_spread3498.clone()).1;
                                                        {
                                                            let source_ml_source2_spread_field3501 = (source_ml_source2_spread3498.clone()).2;
                                                            {
                                                                let source_ml_source2_spread_field3502 = (source_ml_source2_spread3498.clone()).3;
                                                                {
                                                                    let source_ml_source2_spread_field3503 = (source_ml_source2_spread3498.clone()).4;
                                                                    {
                                                                        let source_ml_source2_spread_field3504 = (source_ml_source2_spread3498.clone()).5;
                                                                        {
                                                                            let source_ml_source2_spread_field3505 = (source_ml_source2_spread3498.clone()).6;
                                                                            {
                                                                                let source_ml_source2_spread_field3506 = (source_ml_source2_spread3498.clone()).7;
                                                                                {
                                                                                    let source_ml_source2_spread_field3507 = (source_ml_source2_spread3498.clone()).8;
                                                                                    {
                                                                                        let source_ml_source2_spread_field3508 = (source_ml_source2_spread3498.clone()).9;
                                                                                        {
                                                                                            let source_ml_source2_spread_field3509 = (source_ml_source2_spread3498.clone()).10;
                                                                                            {
                                                                                                let source_ml_source2_spread_field3510 = (source_ml_source2_spread3498.clone()).11;
                                                                                                {
                                                                                                    let source_ml_source2_field3511 = self::JavaScriptSemanticValue::Number((index + 1f64));
                                                                                                    (source_ml_source2_spread_field3499, source_ml_source2_spread_field3500, source_ml_source2_spread_field3501, source_ml_source2_spread_field3502, source_ml_source2_spread_field3503.clone(), source_ml_source2_spread_field3504, source_ml_source2_spread_field3505.clone(), source_ml_source2_spread_field3506, source_ml_source2_field3511.clone(), source_ml_source2_spread_field3508, source_ml_source2_spread_field3509, source_ml_source2_spread_field3510)
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
                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(Box::new(source_ml_source2_nominal_field3513.clone()), source_ml_source2_nominal_override3517.clone())
                                        }
                                    }
                                }
                            } else {
                                property.clone()
                            }])));
                            {
                                let source_ml_source1_temporary_index75_2 = source_ml_source1_temporary_index75.add(&self::ml::Big::from_i128(1));
                                {
                                    (properties, source_ml_source1_temporary_index75) = (properties_2.clone(), source_ml_source1_temporary_index75_2.clone());
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

pub fn source_ml_source1_temporary_slice_utf16_text_loop77(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut last: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    loop {
        return if (at < last) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value510 = result.borrow().clone(); ml_reference_value510 }, vec![({ let ml_reference_array511 = text.clone(); let ml_reference_position511 = at; let ml_reference_projection511 = self::ml_array::optional_at(&*ml_reference_array511.borrow(), self::ml_array::number_index(ml_reference_position511)); ml_reference_projection511 }).clone().unwrap_or_else(|| 0f64)])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_points_utf16_text_loop78(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut result: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>> {
    loop {
        return if (at < ({ let ml_reference_array512 = text.clone(); let ml_reference_projection512 = ml_reference_array512.borrow().len() as f64; ml_reference_projection512 })) {
            {
                let first = ({ let ml_reference_array513 = text.clone(); let ml_reference_position513 = at; let ml_reference_projection513 = self::ml_array::optional_at(&*ml_reference_array513.borrow(), self::ml_array::number_index(ml_reference_position513)); ml_reference_projection513 }).clone().unwrap_or_else(|| 0f64);
                {
                    let second = ({
                        let source_ml_source2_array_read_index3523 = (at + 1f64);
                        { let ml_reference_array514 = text.clone(); let ml_reference_position514 = source_ml_source2_array_read_index3523; let ml_reference_projection514 = self::ml_array::optional_at(&*ml_reference_array514.borrow(), self::ml_array::number_index(ml_reference_position514)); ml_reference_projection514 }
                    }).clone().unwrap_or_else(|| (-1f64));
                    if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        {
                            let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value515 = result.borrow().clone(); ml_reference_value515 }, vec![std::rc::Rc::new(std::cell::RefCell::new(vec![first, second]))])));
                            {
                                let at_2 = (at + 1f64);
                                {
                                    let at_3 = (at_2 + 1f64);
                                    {
                                        (result, at) = (result_2.clone(), at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value516 = result.borrow().clone(); ml_reference_value516 }, vec![std::rc::Rc::new(std::cell::RefCell::new(vec![first]))])));
                            {
                                let at_4 = (at + 1f64);
                                {
                                    (result, at) = (result_3.clone(), at_4);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

pub fn source_ml_source1_temporary_split_utf16_text_loop79(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>>, mut at: f64) -> std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>> {
    loop {
        return if ((at < ({ let ml_reference_array517 = text.clone(); let ml_reference_projection517 = ml_reference_array517.borrow().len() as f64; ml_reference_projection517 })) && (({ let ml_reference_array518 = result.clone(); let ml_reference_projection518 = ml_reference_array518.borrow().len() as f64; ml_reference_projection518 }) < count)) {
            {
                let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value519 = result.borrow().clone(); ml_reference_value519 }, vec![std::rc::Rc::new(std::cell::RefCell::new(vec![({ let ml_reference_array520 = text.clone(); let ml_reference_position520 = at; let ml_reference_projection520 = self::ml_array::optional_at(&*ml_reference_array520.borrow(), self::ml_array::number_index(ml_reference_position520)); ml_reference_projection520 }).clone().unwrap_or_else(|| 0f64)]))])));
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            result.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporarySplitUtf16TextLoop80Result {
    MlSource1TemporarySplitUtf16TextLoop80Done(std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>>, f64),
    MlSource1TemporarySplitUtf16TextLoop80Return(std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>>),
}

pub fn source_ml_source1_temporary_split_utf16_text_loop80(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut separator: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut result: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>>, mut start: f64) -> self::MlSource1TemporarySplitUtf16TextLoop80Result {
    loop {
        return {
            let found = self::index_utf16_text(text.clone(), separator.clone(), start);
            if (found < (0f64)) {
                self::MlSource1TemporarySplitUtf16TextLoop80Result::MlSource1TemporarySplitUtf16TextLoop80Done(result.clone(), start)
            } else {
                {
                    let result_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value521 = result.borrow().clone(); ml_reference_value521 }, vec![self::slice_utf16_text(text.clone(), start, found)])));
                    if (({ let ml_reference_array522 = result_2.clone(); let ml_reference_projection522 = ml_reference_array522.borrow().len() as f64; ml_reference_projection522 }) == count) {
                        self::MlSource1TemporarySplitUtf16TextLoop80Result::MlSource1TemporarySplitUtf16TextLoop80Return(result_2.clone())
                    } else {
                        {
                            let start_2 = (found + { let ml_reference_array523 = separator.clone(); let ml_reference_projection523 = ml_reference_array523.borrow().len() as f64; ml_reference_projection523 });
                            {
                                (result, start) = (result_2.clone(), start_2);
                                continue;
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMatchUtf16TextLoop81Result {
    MlSource1TemporaryMatchUtf16TextLoop81Done,
    MlSource1TemporaryMatchUtf16TextLoop81Return(bool),
}

pub fn source_ml_source1_temporary_match_utf16_text_loop81(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut start: f64, mut at: f64) -> self::MlSource1TemporaryMatchUtf16TextLoop81Result {
    loop {
        return if (at < ({ let ml_reference_array524 = search.clone(); let ml_reference_projection524 = ml_reference_array524.borrow().len() as f64; ml_reference_projection524 })) {
            if {
                let source_ml_source2_comparison_operand3530 = {
                    let source_ml_source2_array_read_index3527 = (start + at);
                    { let ml_reference_array525 = text.clone(); let ml_reference_position525 = source_ml_source2_array_read_index3527; let ml_reference_projection525 = self::ml_array::optional_at(&*ml_reference_array525.borrow(), self::ml_array::number_index(ml_reference_position525)); ml_reference_projection525 }
                };
                {
                    let source_ml_source2_comparison_operand3531 = { let ml_reference_array526 = search.clone(); let ml_reference_position526 = at; let ml_reference_projection526 = self::ml_array::optional_at(&*ml_reference_array526.borrow(), self::ml_array::number_index(ml_reference_position526)); ml_reference_projection526 };
                    !(match (source_ml_source2_comparison_operand3530.clone(), source_ml_source2_comparison_operand3531.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlSource1TemporaryMatchUtf16TextLoop81Result::MlSource1TemporaryMatchUtf16TextLoop81Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryMatchUtf16TextLoop81Result::MlSource1TemporaryMatchUtf16TextLoop81Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryIndexUtf16TextLoop82Result {
    MlSource1TemporaryIndexUtf16TextLoop82Done,
    MlSource1TemporaryIndexUtf16TextLoop82Return(f64),
}

pub fn source_ml_source1_temporary_index_utf16_text_loop82(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryIndexUtf16TextLoop82Result {
    loop {
        return if (at <= (({ let ml_reference_array527 = text.clone(); let ml_reference_projection527 = ml_reference_array527.borrow().len() as f64; ml_reference_projection527 } - { let ml_reference_array528 = search.clone(); let ml_reference_projection528 = ml_reference_array528.borrow().len() as f64; ml_reference_projection528 }))) {
            if self::match_utf16_text(text.clone(), search.clone(), at) {
                self::MlSource1TemporaryIndexUtf16TextLoop82Result::MlSource1TemporaryIndexUtf16TextLoop82Return(at)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryIndexUtf16TextLoop82Result::MlSource1TemporaryIndexUtf16TextLoop82Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLastIndexUtf16TextLoop83Result {
    MlSource1TemporaryLastIndexUtf16TextLoop83Done,
    MlSource1TemporaryLastIndexUtf16TextLoop83Return(f64),
}

pub fn source_ml_source1_temporary_last_index_utf16_text_loop83(mut text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryLastIndexUtf16TextLoop83Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text.clone(), search.clone(), at) {
                self::MlSource1TemporaryLastIndexUtf16TextLoop83Result::MlSource1TemporaryLastIndexUtf16TextLoop83Return(at)
            } else {
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLastIndexUtf16TextLoop83Result::MlSource1TemporaryLastIndexUtf16TextLoop83Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareUtf16TextLoop84Result {
    MlSource1TemporaryCompareUtf16TextLoop84Done,
    MlSource1TemporaryCompareUtf16TextLoop84Return(f64),
}

pub fn source_ml_source1_temporary_compare_utf16_text_loop84(mut left: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut right: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut count: f64, mut at: f64) -> self::MlSource1TemporaryCompareUtf16TextLoop84Result {
    loop {
        return if (at < count) {
            {
                let a = ({ let ml_reference_array529 = left.clone(); let ml_reference_position529 = at; let ml_reference_projection529 = self::ml_array::optional_at(&*ml_reference_array529.borrow(), self::ml_array::number_index(ml_reference_position529)); ml_reference_projection529 }).clone().unwrap_or_else(|| 0f64);
                {
                    let b = ({ let ml_reference_array530 = right.clone(); let ml_reference_position530 = at; let ml_reference_projection530 = self::ml_array::optional_at(&*ml_reference_array530.borrow(), self::ml_array::number_index(ml_reference_position530)); ml_reference_projection530 }).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlSource1TemporaryCompareUtf16TextLoop84Result::MlSource1TemporaryCompareUtf16TextLoop84Return(if (a < b) {
                            (-1f64)
                        } else {
                            1f64
                        })
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
            self::MlSource1TemporaryCompareUtf16TextLoop84Result::MlSource1TemporaryCompareUtf16TextLoop84Done
        };
    }
}
