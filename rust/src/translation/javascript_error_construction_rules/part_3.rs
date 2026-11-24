// Generated from complete js/src/translation/javascript-error-construction-rules.js. Do not edit by hand.
use super::JavaScriptErrorAllocationDirectory;
use super::JavaScriptErrorAllocationIdentity;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::find_java_script_heap_directory_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_heap_directory_value_error;
use super::test_java_script_semantic_same_value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_semantic_text_equal_loop2(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if (at < length) {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Text(units) => {
                        (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_f812) => {
                        {
                            let text = source_ml_source2_f812.clone();
                            {
                                let source_ml_source2_string_unit_operand864 = text.clone();
                                {
                                    let source_ml_source2_string_unit_operand865 = at;
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
                                                let source_ml_source2_string_unit_operand871 = {
                                                    let source_ml_source2_string_range_operand866 = text_2.clone();
                                                    {
                                                        let source_ml_source2_string_range_operand867 = index;
                                                        {
                                                            let source_ml_source2_string_range_operand868 = (index + 1f64);
                                                            {
                                                                let source_ml_source2_string_range_position869 = source_ml_source2_string_range_operand867;
                                                                {
                                                                    let source_ml_source2_string_range_position870 = source_ml_source2_string_range_operand868;
                                                                    source_ml_source2_string_range_operand866.slice(source_ml_source2_string_range_position869, source_ml_source2_string_range_position870)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_string_unit_operand872 = 0f64;
                                                    source_ml_source2_string_unit_operand871.code_point_at(source_ml_source2_string_unit_operand872)
                                                }
                                            }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    } }))(source_ml_source2_string_unit_operand864.clone(), source_ml_source2_string_unit_operand865)
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
                            (self::ml_array::optional_at(&units_2, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_f893) => {
                            {
                                let text_2 = source_ml_source2_f893.clone();
                                {
                                    let source_ml_source2_string_unit_operand945 = text_2.clone();
                                    {
                                        let source_ml_source2_string_unit_operand946 = at;
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
                                                    let source_ml_source2_string_unit_operand952 = {
                                                        let source_ml_source2_string_range_operand947 = text_3.clone();
                                                        {
                                                            let source_ml_source2_string_range_operand948 = index;
                                                            {
                                                                let source_ml_source2_string_range_operand949 = (index + 1f64);
                                                                {
                                                                    let source_ml_source2_string_range_position950 = source_ml_source2_string_range_operand948;
                                                                    {
                                                                        let source_ml_source2_string_range_position951 = source_ml_source2_string_range_operand949;
                                                                        source_ml_source2_string_range_operand947.slice(source_ml_source2_string_range_position950, source_ml_source2_string_range_position951)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_string_unit_operand953 = 0f64;
                                                        source_ml_source2_string_unit_operand952.code_point_at(source_ml_source2_string_unit_operand953)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source2_string_unit_operand945.clone(), source_ml_source2_string_unit_operand946)
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done,
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop6(mut properties: Vec<self::JavaScriptOrdinaryProperty>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    loop {
        return if (at < ((properties.len() as f64))) {
            {
                let property = self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value(match property.clone().expect("a proven present value") {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value971, _) => {
                        let source_ml_source2_data_property_value971 = *source_ml_source2_data_property_value971;
                        source_ml_source2_data_property_value971.clone()
                    }
                }, key.clone())) {
                    self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(at)
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
            self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done,
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_heap_directory_object_loop8(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value985, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value985.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value986, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value986.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value987, _, _, _, _) => {
                                source_ml_source2_data_property_value987.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value988, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value988.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value989, _, _, _, _, _) => {
                                source_ml_source2_data_property_value989.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value990, _, _, _, _, _) => {
                                source_ml_source2_data_property_value990.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value991, _, _, _, _, _) => {
                                source_ml_source2_data_property_value991.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value992, _, _, _, _, _) => {
                                source_ml_source2_data_property_value992.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value993, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value993.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value994, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value994.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value995, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value995.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value996, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value996.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value997, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value997.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value1011, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1011.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value1012, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1012.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value1013, _, _, _, _) => {
                                    source_ml_source2_data_property_value1013.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value1014, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1014.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value1015, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1015.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value1016, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1016.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value1017, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1017.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value1018, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1018.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value1019, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1019.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value1020, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1020.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value1021, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1021.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value1022, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1022.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value1023, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value1023.clone()
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
            self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done,
    MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop9(mut symbols: Vec<self::ml::Big>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let symbol = self::ml_array::optional_at(&symbols, self::ml_array::number_index(middle));
                    if {
                        let source_ml_source2_comparison_operand1025 = None::<()>;
                        match (symbol.clone(), source_ml_source2_comparison_operand1025.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    } {
                        self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(false)
                    } else {
                        if ((symbol.clone().expect("a proven present value")) == identity) {
                            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(true)
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
            self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result {
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done,
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop13(mut heap: self::JavaScriptOrdinaryHeap, mut before: self::JavaScriptErrorAllocationDirectory, mut previous: self::ml::Big, mut source_ml_source1_temporary_values11: Vec<self::ml::Big>, mut source_ml_source1_temporary_index11: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result {
    loop {
        return if (source_ml_source1_temporary_index11 < (self::ml::Big::from_u128(source_ml_source1_temporary_values11.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_source1_temporary_values11, source_ml_source1_temporary_index11.to_index());
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || { let ml_values3 = match before.clone() {
                    self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(source_ml_source2_data_property_value1026, _) => {
                        source_ml_source2_data_property_value1026
                    }
                }; let ml_callback3 = { let symbol = symbol.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorAllocationIdentity| -> bool { ((match row.clone() {
                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value1028, _, _) => {
                        source_ml_source2_data_property_value1028.clone()
                    }
                }) == symbol) }) }; ml_values3.iter().cloned().enumerate().any(|(ml_index3, ml_value3)| ml_callback3(ml_value3)) }) {
                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text::from_units(vec![112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    if !(self::ml_array::includes((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source2_data_property_value1030, _, _, _, _) => source_ml_source2_data_property_value1030.as_slice() }), &symbol.clone(), 0f64, false, |a, b| a == b)) {
                        self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 83, 121, 109, 98, 111, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                    } else {
                        {
                            let source_ml_source1_temporary_index11_2 = source_ml_source1_temporary_index11.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_source1_temporary_index11) = (symbol.clone(), source_ml_source1_temporary_index11_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result {
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop12(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_source1_temporary_values10: Vec<self::JavaScriptErrorAllocationIdentity>, mut source_ml_source1_temporary_index10: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field4, _, _, _, _, _) => {
                if (source_ml_source1_temporary_index10 < (self::ml::Big::from_u128(source_ml_source1_temporary_values10.len() as u128))) {
                    {
                        let row = self::ml_array::at(&source_ml_source1_temporary_values10, source_ml_source1_temporary_index10.to_index());
                        match row.clone() {
                            self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                if ((shared_constructor_field1 <= previous) || (shared_constructor_field1 > (self::ml::Big::from_i128(18446744073709551615)))) {
                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text::from_units(vec![112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 109, 117, 115, 116, 32, 114, 101, 116, 97, 105, 110, 32, 115, 116, 114, 105, 99, 116, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 111, 114, 100, 101, 114]))
                                } else {
                                    {
                                        let previous_2 = shared_constructor_field1.clone();
                                        {
                                            let object = self::ml_array::optional_at(&shared_constructor_field4, self::ml_array::number_index(self::find_java_script_heap_directory_object(shared_constructor_field4, shared_constructor_field1.clone())));
                                            if ((match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                }
                                            }) != shared_constructor_field2)) || ((match object.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value1055, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1055
                                                }
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value1056, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1056
                                                }
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value1057, _) => {
                                                    source_ml_source2_data_property_value1057
                                                }
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value1058, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value1058
                                                }
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value1059, _, _) => {
                                                    source_ml_source2_data_property_value1059
                                                }
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value1060, _, _) => {
                                                    source_ml_source2_data_property_value1060
                                                }
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value1061, _, _) => {
                                                    source_ml_source2_data_property_value1061
                                                }
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value1062, _, _) => {
                                                    source_ml_source2_data_property_value1062
                                                }
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value1063, _) => {
                                                    source_ml_source2_data_property_value1063
                                                }
                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value1064, _, _, _) => {
                                                    source_ml_source2_data_property_value1064
                                                }
                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value1065, _, _, _) => {
                                                    source_ml_source2_data_property_value1065
                                                }
                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value1066, _, _, _) => {
                                                    source_ml_source2_data_property_value1066
                                                }
                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value1067, _, _, _) => {
                                                    source_ml_source2_data_property_value1067
                                                }
                                            }) != shared_constructor_field3)) {
                                                self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(self::ml_string::Text::from_units(vec![97, 110, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 109, 111, 118, 101, 32, 111, 114, 32, 114, 101, 116, 121, 112, 101, 32, 112, 114, 105, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index10_2 = source_ml_source1_temporary_index10.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (previous, source_ml_source1_temporary_index10) = (previous_2.clone(), source_ml_source1_temporary_index10_2.clone());
                                                        continue;
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
                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(previous.clone())
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result {
    MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done,
    MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop15(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values14: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index14: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result {
    loop {
        return if (source_ml_source1_temporary_index14 < (self::ml::Big::from_u128(source_ml_source1_temporary_values14.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values14, source_ml_source1_temporary_index14.to_index());
                {
                    let error = self::read_java_script_heap_directory_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index14_2 = source_ml_source1_temporary_index14.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index14 = source_ml_source1_temporary_index14_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done
        };
    }
}
