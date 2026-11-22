// Generated from complete js/src/translation/javascript-error-construction-rules.js. Do not edit by hand.
use super::JavaScriptErrorAllocationDirectory;
use super::JavaScriptErrorAllocationIdentity;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result;
use super::find_java_script_heap_directory_object;
use super::find_java_script_heap_directory_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_error_prior_directory_error;
use super::read_java_script_heap_directory_value_error;
use super::source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop15;
use super::test_java_script_semantic_same_value;

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
                        let source_ml_source2_s522 = self::source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop15(heap.clone(), source_ml_source1_temporary_values14, source_ml_source1_temporary_index14.clone());
                        match source_ml_source2_s522.clone() {
                            self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop15Done => {
                                if (match constructor.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                } || !(match match constructor.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        None::<bool>
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        None::<bool>
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        None::<bool>
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        None::<bool>
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        None::<bool>
                                    }
                                    self::JavaScriptSemanticValue::Text(_) => {
                                        None::<bool>
                                    }
                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                        None::<bool>
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        None::<bool>
                                    }
                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value532) => {
                                        Some(source_ml_source2_data_property_value532)
                                    }
                                } { Some(ml_present) => ml_present, None => false })) {
                                    self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])
                                } else {
                                    match new_target.clone() {
                                        self::JavaScriptSemanticValue::Object(identity, callable) => {
                                            if !callable {
                                                self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])
                                            } else {
                                                if match {
                                                    let source_ml_source2_logical550 = match prototype.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    };
                                                    if source_ml_source2_logical550 {
                                                        Some(source_ml_source2_logical550)
                                                    } else {
                                                        match prototype.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Null => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Number(_) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Text(_) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                None::<bool>
                                                            }
                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value549) => {
                                                                Some(source_ml_source2_data_property_value549)
                                                            }
                                                        }
                                                    }
                                                } { Some(ml_present) => ml_present, None => false } {
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
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value580, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value580
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value581, _, _, _) => {
                                                                            source_ml_source2_data_property_value581
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value582) => {
                                                                            source_ml_source2_data_property_value582
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value583, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value583
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value584, _) => {
                                                                            source_ml_source2_data_property_value584
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value585, _) => {
                                                                            source_ml_source2_data_property_value585
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value586, _) => {
                                                                            source_ml_source2_data_property_value586
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value587, _) => {
                                                                            source_ml_source2_data_property_value587
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value588) => {
                                                                            source_ml_source2_data_property_value588
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value589, _, _) => {
                                                                            source_ml_source2_data_property_value589
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value590, _, _) => {
                                                                            source_ml_source2_data_property_value590
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value591, _, _) => {
                                                                            source_ml_source2_data_property_value591
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value592, _, _) => {
                                                                            source_ml_source2_data_property_value592
                                                                        }
                                                                    }, self::ml_array::number_index(self::find_java_script_heap_directory_property(match object.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value593, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value593
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value594, _, _, _) => {
                                                                            source_ml_source2_data_property_value594
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value595) => {
                                                                            source_ml_source2_data_property_value595
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value596, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value596
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value597, _) => {
                                                                            source_ml_source2_data_property_value597
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value598, _) => {
                                                                            source_ml_source2_data_property_value598
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value599, _) => {
                                                                            source_ml_source2_data_property_value599
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value600, _) => {
                                                                            source_ml_source2_data_property_value600
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value601) => {
                                                                            source_ml_source2_data_property_value601
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value602, _, _) => {
                                                                            source_ml_source2_data_property_value602
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value603, _, _) => {
                                                                            source_ml_source2_data_property_value603
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value604, _, _) => {
                                                                            source_ml_source2_data_property_value604
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value605, _, _) => {
                                                                            source_ml_source2_data_property_value605
                                                                        }
                                                                    }, self::JavaScriptSemanticValue::Text(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))));
                                                                    if ((match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({
                                                                        let source_ml_source2_nullish_field_receiver606 = (property.clone(),);
                                                                        if match (source_ml_source2_nullish_field_receiver606.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source2_nullish_field_present607 = ((source_ml_source2_nullish_field_receiver606.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source2_nullish_field_present607.clone() {
                                                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value608) => {
                                                                                        source_ml_source2_data_property_value608.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    }).9)) || !(self::test_java_script_semantic_same_value(({
                                                                        let source_ml_source2_nullish_field_receiver609 = (property.clone(),);
                                                                        if match (source_ml_source2_nullish_field_receiver609.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source2_nullish_field_present610 = ((source_ml_source2_nullish_field_receiver609.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source2_nullish_field_present610.clone() {
                                                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value611) => {
                                                                                        source_ml_source2_data_property_value611.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
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
                                                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value621, _, _) => {
                                                        source_ml_source2_data_property_value621.clone()
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
                                                            } && self::test_java_script_semantic_same_value(match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value721, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value721 = *source_ml_source2_data_property_value721;
                                                                    source_ml_source2_data_property_value721.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value722, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value722 = *source_ml_source2_data_property_value722;
                                                                    source_ml_source2_data_property_value722.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value723, _, _, _) => {
                                                                    let source_ml_source2_data_property_value723 = *source_ml_source2_data_property_value723;
                                                                    source_ml_source2_data_property_value723.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value724, _, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value724 = *source_ml_source2_data_property_value724;
                                                                    source_ml_source2_data_property_value724.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value725, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value725 = *source_ml_source2_data_property_value725;
                                                                    source_ml_source2_data_property_value725.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value726, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value726 = *source_ml_source2_data_property_value726;
                                                                    source_ml_source2_data_property_value726.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value727, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value727 = *source_ml_source2_data_property_value727;
                                                                    source_ml_source2_data_property_value727.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value728, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value728 = *source_ml_source2_data_property_value728;
                                                                    source_ml_source2_data_property_value728.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value729, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value729 = *source_ml_source2_data_property_value729;
                                                                    source_ml_source2_data_property_value729.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value730, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value730 = *source_ml_source2_data_property_value730;
                                                                    source_ml_source2_data_property_value730.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, source_ml_source2_data_property_value731, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value731 = *source_ml_source2_data_property_value731;
                                                                    source_ml_source2_data_property_value731.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, source_ml_source2_data_property_value732, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value732 = *source_ml_source2_data_property_value732;
                                                                    source_ml_source2_data_property_value732.clone()
                                                                }
                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, source_ml_source2_data_property_value733, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value733 = *source_ml_source2_data_property_value733;
                                                                    source_ml_source2_data_property_value733.clone()
                                                                }
                                                            }, prototype.clone())) {
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
                let a = if match left.clone() {
                    self::JavaScriptSemanticValue::Text(_) => {
                        true
                    }
                    _ => {
                        false
                    }
                } {
                    ({
                        let source_ml_source2_nullish_field_receiver744 = (match left.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                None::<Vec<f64>>
                            }
                            self::JavaScriptSemanticValue::Null => {
                                None::<Vec<f64>>
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                None::<Vec<f64>>
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                None::<Vec<f64>>
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                None::<Vec<f64>>
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source2_data_property_value743) => {
                                Some(source_ml_source2_data_property_value743)
                            }
                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                None::<Vec<f64>>
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<Vec<f64>>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<Vec<f64>>
                            }
                        }, at);
                        {
                            let source_ml_source2_computed_property_key745 = (source_ml_source2_nullish_field_receiver744.clone()).1;
                            if match (source_ml_source2_nullish_field_receiver744.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source2_nullish_field_present746 = ((source_ml_source2_nullish_field_receiver744.clone()).0).clone().expect("a proven present value");
                                    self::ml_array::optional_at(&source_ml_source2_nullish_field_present746, self::ml_array::number_index(source_ml_source2_computed_property_key745))
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source2_nullish_field_receiver744.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
                            }
                        }
                    }).clone().unwrap_or_else(|| (-1f64))
                } else {
                    if match left.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                            true
                        }
                        _ => {
                            false
                        }
                    } {
                        {
                            let source_ml_source2_nullish_field_receiver757 = (match left.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    None::<self::ml_string::Text>
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    None::<self::ml_string::Text>
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    None::<self::ml_string::Text>
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    None::<self::ml_string::Text>
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    None::<self::ml_string::Text>
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    None::<self::ml_string::Text>
                                }
                                self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value756) => {
                                    Some(source_ml_source2_data_property_value756.clone())
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    None::<self::ml_string::Text>
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    None::<self::ml_string::Text>
                                }
                            },);
                            if ((match (source_ml_source2_nullish_field_receiver757.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                                {
                                    let source_ml_source2_nullish_field_present758 = ((source_ml_source2_nullish_field_receiver757.clone()).0).clone().expect("a proven present value");
                                    {
                                        let source_ml_source2_string_unit_operand759 = source_ml_source2_nullish_field_present758.clone();
                                        {
                                            let source_ml_source2_string_unit_operand760 = at;
                                            (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if ((index < (0f64)) || (index >= (text.length()))) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                        let source_ml_source2_string_unit_operand766 = {
                                                            let source_ml_source2_string_range_operand761 = text.clone();
                                                            {
                                                                let source_ml_source2_string_range_operand762 = index;
                                                                {
                                                                    let source_ml_source2_string_range_operand763 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source2_string_range_position764 = source_ml_source2_string_range_operand762;
                                                                        {
                                                                            let source_ml_source2_string_range_position765 = source_ml_source2_string_range_operand763;
                                                                            source_ml_source2_string_range_operand761.slice(source_ml_source2_string_range_position764, source_ml_source2_string_range_position765)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_string_unit_operand767 = 0f64;
                                                            source_ml_source2_string_unit_operand766.code_point_at(source_ml_source2_string_unit_operand767)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source2_string_unit_operand759.clone(), source_ml_source2_string_unit_operand760)
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 39, 41])).0, message_present: true }).raise()
                            }
                        }
                    } else {
                        (-1f64)
                    }
                };
                {
                    let b = if match right.clone() {
                        self::JavaScriptSemanticValue::Text(_) => {
                            true
                        }
                        _ => {
                            false
                        }
                    } {
                        ({
                            let source_ml_source2_nullish_field_receiver778 = (match right.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    None::<Vec<f64>>
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    None::<Vec<f64>>
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    None::<Vec<f64>>
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    None::<Vec<f64>>
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    None::<Vec<f64>>
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source2_data_property_value777) => {
                                    Some(source_ml_source2_data_property_value777)
                                }
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    None::<Vec<f64>>
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    None::<Vec<f64>>
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    None::<Vec<f64>>
                                }
                            }, at);
                            {
                                let source_ml_source2_computed_property_key779 = (source_ml_source2_nullish_field_receiver778.clone()).1;
                                if match (source_ml_source2_nullish_field_receiver778.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source2_nullish_field_present780 = ((source_ml_source2_nullish_field_receiver778.clone()).0).clone().expect("a proven present value");
                                        self::ml_array::optional_at(&source_ml_source2_nullish_field_present780, self::ml_array::number_index(source_ml_source2_computed_property_key779))
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source2_nullish_field_receiver778.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
                                }
                            }
                        }).clone().unwrap_or_else(|| (-1f64))
                    } else {
                        if match right.clone() {
                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                true
                            }
                            _ => {
                                false
                            }
                        } {
                            {
                                let source_ml_source2_nullish_field_receiver791 = (match right.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Text(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value790) => {
                                        Some(source_ml_source2_data_property_value790.clone())
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        None::<self::ml_string::Text>
                                    }
                                },);
                                if ((match (source_ml_source2_nullish_field_receiver791.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                                    {
                                        let source_ml_source2_nullish_field_present792 = ((source_ml_source2_nullish_field_receiver791.clone()).0).clone().expect("a proven present value");
                                        {
                                            let source_ml_source2_string_unit_operand793 = source_ml_source2_nullish_field_present792.clone();
                                            {
                                                let source_ml_source2_string_unit_operand794 = at;
                                                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                    let index = if position.is_nan() {
                                                        0f64
                                                    } else {
                                                        position.trunc()
                                                    };
                                                    if ((index < (0f64)) || (index >= (text.length()))) {
                                                        f64::NAN
                                                    } else {
                                                        ({
                                                            let source_ml_source2_string_unit_operand800 = {
                                                                let source_ml_source2_string_range_operand795 = text.clone();
                                                                {
                                                                    let source_ml_source2_string_range_operand796 = index;
                                                                    {
                                                                        let source_ml_source2_string_range_operand797 = (index + 1f64);
                                                                        {
                                                                            let source_ml_source2_string_range_position798 = source_ml_source2_string_range_operand796;
                                                                            {
                                                                                let source_ml_source2_string_range_position799 = source_ml_source2_string_range_operand797;
                                                                                source_ml_source2_string_range_operand795.slice(source_ml_source2_string_range_position798, source_ml_source2_string_range_position799)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_string_unit_operand801 = 0f64;
                                                                source_ml_source2_string_unit_operand800.code_point_at(source_ml_source2_string_unit_operand801)
                                                            }
                                                        }).clone().unwrap_or_else(|| f64::NAN)
                                                    }
                                                } }))(source_ml_source2_string_unit_operand793.clone(), source_ml_source2_string_unit_operand794)
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 39, 41])).0, message_present: true }).raise()
                                }
                            }
                        } else {
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
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value({
                    let source_ml_source2_nullish_field_receiver803 = (property.clone(),);
                    if match (source_ml_source2_nullish_field_receiver803.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present804 = ((source_ml_source2_nullish_field_receiver803.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present804.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value805, _) => {
                                    let source_ml_source2_data_property_value805 = *source_ml_source2_data_property_value805;
                                    source_ml_source2_data_property_value805.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
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
