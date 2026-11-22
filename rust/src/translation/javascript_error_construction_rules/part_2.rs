// Generated from complete js/src/translation/javascript-error-construction-rules.js. Do not edit by hand.
use super::JavaScriptErrorAllocationDirectory;
use super::JavaScriptErrorAllocationIdentity;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result;
use super::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result;
use super::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result;
use super::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result;
use super::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result;
use super::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result;
use super::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_value_error;
use super::source_ml_source1_temporary_find_java_script_heap_directory_object_loop8;
use super::source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop6;
use super::source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop15;
use super::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop12;
use super::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop13;
use super::source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop9;
use super::source_ml_source1_temporary_test_java_script_semantic_text_equal_loop2;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
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
                    self::JavaScriptSemanticValue::Undefined => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Null => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value241) => {
                        Some((source_ml_source2_data_property_value241.clone(),))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        None::<(self::ml_string::Text,)>
                    }
                }).clone().expect("a proven present value")).0).clone()) == ((((match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Null => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value242) => {
                        Some((source_ml_source2_data_property_value242.clone(),))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        None::<(self::ml_string::Text,)>
                    }
                }).clone().expect("a proven present value")).0).clone()))
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s243 = self::source_ml_source1_temporary_test_java_script_semantic_text_equal_loop2(left.clone(), right.clone(), length, at);
                        match source_ml_source2_s243.clone() {
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done => {
                                true
                            }
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(source_ml_source1_temporary_result2) => {
                                source_ml_source1_temporary_result2
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_property_descriptor_entry(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source2_s397 = self::source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop6(properties, key.clone(), at);
            match source_ml_source2_s397.clone() {
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(source_ml_source1_temporary_result6) => {
                    source_ml_source1_temporary_result6
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_heap_directory_object(objects: Vec<self::JavaScriptOrdinaryObject>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = (objects.len() as f64);
            {
                let source_ml_source2_s439 = self::source_ml_source1_temporary_find_java_script_heap_directory_object_loop8(objects, identity.clone(), low, high);
                match source_ml_source2_s439.clone() {
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(source_ml_source1_temporary_result8) => {
                        source_ml_source1_temporary_result8
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_heap_directory_symbol(symbols: Vec<self::ml::Big>, identity: self::ml::Big) -> bool {
    {
        let low = 0f64;
        {
            let high = (symbols.len() as f64);
            {
                let source_ml_source2_s440 = self::source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop9(symbols, identity.clone(), low, high);
                match source_ml_source2_s440.clone() {
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(source_ml_source1_temporary_result9) => {
                        source_ml_source1_temporary_result9
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
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
                            if self::test_java_script_heap_directory_symbol(shared_constructor_field2, identity.clone()) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f447, source_ml_source2_f448) => {
                            {
                                let at = self::find_java_script_heap_directory_object(shared_constructor_field1.clone(), source_ml_source2_f447.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value521, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value521
                                            }
                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value522, _, _, _, _) => {
                                                source_ml_source2_data_property_value522
                                            }
                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value523, _) => {
                                                source_ml_source2_data_property_value523
                                            }
                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value524, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value524
                                            }
                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value525, _, _) => {
                                                source_ml_source2_data_property_value525
                                            }
                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value526, _, _) => {
                                                source_ml_source2_data_property_value526
                                            }
                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value527, _, _) => {
                                                source_ml_source2_data_property_value527
                                            }
                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value528, _, _) => {
                                                source_ml_source2_data_property_value528
                                            }
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value529, _) => {
                                                source_ml_source2_data_property_value529
                                            }
                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value530, _, _, _) => {
                                                source_ml_source2_data_property_value530
                                            }
                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value531, _, _, _) => {
                                                source_ml_source2_data_property_value531
                                            }
                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value532, _, _, _) => {
                                                source_ml_source2_data_property_value532
                                            }
                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value533, _, _, _) => {
                                                source_ml_source2_data_property_value533
                                            }
                                        }) == source_ml_source2_f448)) {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_heap_directory_property(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_property_descriptor_entry(properties, key.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_operation(name: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*name) == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114])
    } else {
        if ((*name) == (self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if ((*name) == (self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if ((*name) == (self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if ((*name) == (self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if ((*name) == (self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if ((*name) == (self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114])
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_error_constructor_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode) == (self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_error_intrinsic_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode) == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_construction_kind_error(error_class: &self::ml_string::Text, bind_receiver: bool, mode: &self::ml_string::Text) -> self::ml_string::Text {
    if ((self::read_java_script_source_exception_operation(error_class)) == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 108, 97, 115, 115])
    } else {
        if (bind_receiver && !(self::test_java_script_error_constructor_mode(mode))) {
            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114, 105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 98, 105, 110, 100, 115, 32, 116, 104, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_allocation_directory(heap: self::JavaScriptOrdinaryHeap) -> self::JavaScriptErrorAllocationDirectory {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, _, _, _, _) => {
            self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory({ let ml_values1 = shared_constructor_field1; let ml_callback1 = std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptErrorAllocationIdentity { self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value578, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value578.clone()
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value579, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value579.clone()
                }
                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value580, _, _, _, _) => {
                    source_ml_source2_data_property_value580.clone()
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value581, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value581.clone()
                }
                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value582, _, _, _, _, _) => {
                    source_ml_source2_data_property_value582.clone()
                }
                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value583, _, _, _, _, _) => {
                    source_ml_source2_data_property_value583.clone()
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value584, _, _, _, _, _) => {
                    source_ml_source2_data_property_value584.clone()
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value585, _, _, _, _, _) => {
                    source_ml_source2_data_property_value585.clone()
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value586, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value586.clone()
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value587, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value587.clone()
                }
                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value588, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value588.clone()
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value589, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value589.clone()
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value590, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value590.clone()
                }
            }, match object.clone() {
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
            }, match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value591, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value591
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value592, _, _, _, _) => {
                    source_ml_source2_data_property_value592
                }
                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value593, _) => {
                    source_ml_source2_data_property_value593
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value594, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value594
                }
                self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value595, _, _) => {
                    source_ml_source2_data_property_value595
                }
                self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value596, _, _) => {
                    source_ml_source2_data_property_value596
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value597, _, _) => {
                    source_ml_source2_data_property_value597
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value598, _, _) => {
                    source_ml_source2_data_property_value598
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value599, _) => {
                    source_ml_source2_data_property_value599
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value600, _, _, _) => {
                    source_ml_source2_data_property_value600
                }
                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value601, _, _, _) => {
                    source_ml_source2_data_property_value601
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value602, _, _, _) => {
                    source_ml_source2_data_property_value602
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value603, _, _, _) => {
                    source_ml_source2_data_property_value603
                }
            }) }); ml_values1.iter().cloned().enumerate().map(|(ml_index1, ml_value1)| ml_callback1(ml_value1)).collect::<Vec<_>>() }, self::ml_array::append(Vec::<self::ml::Big>::new(), shared_constructor_field2))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_prior_directory_error(heap: self::JavaScriptOrdinaryHeap, before: self::JavaScriptErrorAllocationDirectory) -> self::ml_string::Text {
    match before.clone() {
        self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(shared_constructor_field1, shared_constructor_field2) => {
            if ((((shared_constructor_field1.len() as f64)) > (4096f64)) || (((shared_constructor_field2.len() as f64)) > (16384f64))) {
                self::ml_string::Text::from_units(vec![116, 104, 101, 32, 112, 114, 105, 111, 114, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let previous = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_temporary_values10 = shared_constructor_field1;
                        {
                            let source_ml_source1_temporary_index10 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s610 = self::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop12(heap.clone(), previous.clone(), source_ml_source1_temporary_values10, source_ml_source1_temporary_index10.clone());
                                match source_ml_source2_s610.clone() {
                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(previous_2) => {
                                        {
                                            let previous_3 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_temporary_values11 = shared_constructor_field2;
                                                {
                                                    let source_ml_source1_temporary_index11 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s612 = self::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop13(heap.clone(), before.clone(), previous_3.clone(), source_ml_source1_temporary_values11, source_ml_source1_temporary_index11.clone());
                                                        match source_ml_source2_s612.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done => {
                                                                self::ml_string::Text::from_units(vec![])
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(source_ml_source1_temporary_result13) => {
                                                                source_ml_source1_temporary_result13.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(source_ml_source1_temporary_result13_2) => {
                                        source_ml_source1_temporary_result13_2.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_construction_operands_error(heap: self::JavaScriptOrdinaryHeap, constructor: self::JavaScriptSemanticValue, new_target: self::JavaScriptSemanticValue, prototype: self::JavaScriptSemanticValue, message: self::JavaScriptSemanticValue, present: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let source_ml_source1_temporary_values14 = vec![constructor.clone(), new_target.clone(), prototype.clone(), message.clone(), present.clone()];
                {
                    let source_ml_source1_temporary_index14 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s613 = self::source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop15(heap.clone(), source_ml_source1_temporary_values14, source_ml_source1_temporary_index14.clone());
                        match source_ml_source2_s613.clone() {
                            self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done => {
                                if (match constructor.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                } || !(((match constructor.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        None::<(bool,)>
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        None::<(bool,)>
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        None::<(bool,)>
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        None::<(bool,)>
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        None::<(bool,)>
                                    }
                                    self::JavaScriptSemanticValue::Text(_) => {
                                        None::<(bool,)>
                                    }
                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                        None::<(bool,)>
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        None::<(bool,)>
                                    }
                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value623) => {
                                        Some((source_ml_source2_data_property_value623,))
                                    }
                                }).clone().expect("a proven present value")).0)) {
                                    self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])
                                } else {
                                    match new_target.clone() {
                                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                                            if !callable {
                                                self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])
                                            } else {
                                                if (match prototype.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                } || ((match prototype.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value640) => {
                                                        Some((source_ml_source2_data_property_value640,))
                                                    }
                                                }).clone().expect("a proven present value")).0) {
                                                    self::ml_string::Text::from_units(vec![116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 69, 114, 114, 111, 114, 32, 115, 117, 98, 99, 108, 97, 115, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 79, 98, 106, 101, 99, 116])
                                                } else {
                                                    if ((match message.clone() {
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    } && match message.clone() {
                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) || match present.clone() {
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 85, 84, 70, 49, 54, 32, 116, 101, 120, 116, 32, 97, 110, 100, 32, 105, 110, 100, 101, 112, 101, 110, 100, 101, 110, 116, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101])
                                                    } else {
                                                        {
                                                            let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_heap_directory_object(shared_constructor_field1, identity.clone())));
                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::ml_string::Text::from_units(vec![110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 104, 101, 97, 112])
                                                            } else {
                                                                {
                                                                    let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value670, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value670
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value671, _, _, _) => {
                                                                            source_ml_source2_data_property_value671
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value672) => {
                                                                            source_ml_source2_data_property_value672
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value673, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value673
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value674, _) => {
                                                                            source_ml_source2_data_property_value674
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value675, _) => {
                                                                            source_ml_source2_data_property_value675
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value676, _) => {
                                                                            source_ml_source2_data_property_value676
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value677, _) => {
                                                                            source_ml_source2_data_property_value677
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value678) => {
                                                                            source_ml_source2_data_property_value678
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value679, _, _) => {
                                                                            source_ml_source2_data_property_value679
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value680, _, _) => {
                                                                            source_ml_source2_data_property_value680
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value681, _, _) => {
                                                                            source_ml_source2_data_property_value681
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value682, _, _) => {
                                                                            source_ml_source2_data_property_value682
                                                                        }
                                                                    }, self::ml_array::number_index(self::find_java_script_heap_directory_property(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value683, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value683
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value684, _, _, _) => {
                                                                            source_ml_source2_data_property_value684
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value685) => {
                                                                            source_ml_source2_data_property_value685
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value686, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value686
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value687, _) => {
                                                                            source_ml_source2_data_property_value687
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value688, _) => {
                                                                            source_ml_source2_data_property_value688
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value689, _) => {
                                                                            source_ml_source2_data_property_value689
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value690, _) => {
                                                                            source_ml_source2_data_property_value690
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value691) => {
                                                                            source_ml_source2_data_property_value691
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value692, _, _) => {
                                                                            source_ml_source2_data_property_value692
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value693, _, _) => {
                                                                            source_ml_source2_data_property_value693
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value694, _, _) => {
                                                                            source_ml_source2_data_property_value694
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value695, _, _) => {
                                                                            source_ml_source2_data_property_value695
                                                                        }
                                                                    }, self::JavaScriptSemanticValue::Text(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))));
                                                                    if ((match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !((match property.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value696) => {
                                                                            source_ml_source2_data_property_value696.clone()
                                                                        }
                                                                    }).9)) || !(self::test_java_script_semantic_same_value((match property.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value697) => {
                                                                            source_ml_source2_data_property_value697.clone()
                                                                        }
                                                                    }).8, prototype.clone()))) {
                                                                        self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 112, 114, 111, 102, 105, 108, 101, 32, 110, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 97, 110, 32, 105, 110, 118, 101, 110, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 111, 114, 32, 103, 101, 116, 116, 101, 114])
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
                                        _ => {
                                            self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Return(source_ml_source1_temporary_result15) => {
                                source_ml_source1_temporary_result15.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_allocation_result_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue, prototype: self::JavaScriptSemanticValue, before: self::JavaScriptErrorAllocationDirectory) -> self::ml_string::Text {
    match before.clone() {
        self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(shared_constructor_field2, shared_constructor_field3) => {
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
                    {
                        let prior = self::read_java_script_error_prior_directory_error(heap.clone(), before.clone());
                        if (prior != (self::ml_string::Text::from_units(vec![]))) {
                            prior.clone()
                        } else {
                            {
                                let error = self::read_java_script_heap_directory_value_error(heap.clone(), value.clone());
                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                    error.clone()
                                } else {
                                    {
                                        let invalid_result = self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 103, 108, 111, 98, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]);
                                        match value.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                invalid_result.clone()
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                invalid_result.clone()
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                invalid_result.clone()
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                invalid_result.clone()
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                invalid_result.clone()
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                invalid_result.clone()
                                            }
                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                invalid_result.clone()
                                            }
                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                invalid_result.clone()
                                            }
                                            self::JavaScriptSemanticValue::Object(identity, callable) => {
                                                if ((callable || { let ml_values2 = shared_constructor_field2; let ml_callback2 = { let identity = identity.clone(); std::rc::Rc::new(move |row: self::JavaScriptErrorAllocationIdentity| -> bool { ((match row.clone() {
                                                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value707, _, _) => {
                                                        source_ml_source2_data_property_value707.clone()
                                                    }
                                                }) == identity) }) }; ml_values2.iter().cloned().enumerate().any(|(ml_index2, ml_value2)| ml_callback2(ml_value2)) }) || self::ml_array::includes(&shared_constructor_field3, &identity.clone(), 0f64, false, |a, b| a == b)) {
                                                    invalid_result.clone()
                                                } else {
                                                    {
                                                        let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_heap_directory_object(shared_constructor_field1, identity.clone())));
                                                        if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::ml_string::Text::from_units(vec![116, 104, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114, 105, 118, 101, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                        } else {
                                                            if (match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            } && self::test_java_script_semantic_same_value((((match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value807, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value807 = *source_ml_source2_data_property_value807;
                                                                    Some((source_ml_source2_data_property_value807.clone(),))
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0).clone(), prototype.clone())) {
                                                                self::ml_string::Text::from_units(vec![])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![116, 104, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114, 105, 118, 101, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
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
