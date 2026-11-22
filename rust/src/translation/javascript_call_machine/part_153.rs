// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptErrorAllocationDirectory;
use super::JavaScriptErrorAllocationIdentity;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryCharacterUnitsUtf16TextLoop1282Result;
use super::MlSource1TemporaryCompareUtf16TextLoop1273Result;
use super::MlSource1TemporaryIndexUtf16TextLoop1271Result;
use super::MlSource1TemporaryLastIndexUtf16TextLoop1272Result;
use super::MlSource1TemporaryMatchUtf16TextLoop1270Result;
use super::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Result;
use super::MlSource1TemporarySliceUtf16TextLoop1260Result;
use super::find_java_script_heap_directory_object;
use super::find_java_script_heap_directory_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_error_prior_directory_error;
use super::read_java_script_heap_directory_value_error;
use super::source_ml_source1_temporary_character_units_utf16_text_loop1282;
use super::source_ml_source1_temporary_compare_utf16_text_loop1273;
use super::source_ml_source1_temporary_index_utf16_text_loop1271;
use super::source_ml_source1_temporary_last_index_utf16_text_loop1272;
use super::source_ml_source1_temporary_match_utf16_text_loop1270;
use super::source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop1259;
use super::source_ml_source1_temporary_slice_utf16_text_loop1260;
use super::source_ml_source1_temporary_trim_utf16_text_loop1280;
use super::source_ml_source1_temporary_trim_utf16_text_loop1281;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_construction_operands_error(
    heap: self::JavaScriptOrdinaryHeap,
    constructor: self::JavaScriptSemanticValue,
    new_target: self::JavaScriptSemanticValue,
    prototype: self::JavaScriptSemanticValue,
    message: self::JavaScriptSemanticValue,
    present: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let source_ml_source1_temporary_values1258 =
                std::rc::Rc::new(std::cell::RefCell::new(vec![
                    constructor.clone(),
                    new_target.clone(),
                    prototype.clone(),
                    message.clone(),
                    present.clone(),
                ]));
            {
                let source_ml_source1_temporary_index1258 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s29044 = self::source_ml_source1_temporary_read_java_script_error_construction_operands_error_loop1259(heap.clone(), source_ml_source1_temporary_values1258.clone(), source_ml_source1_temporary_index1258.clone());
                    match source_ml_source2_s29044.clone() {
                            self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Done => {
                                if (match constructor.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                } || !(((match constructor.clone() {
                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value29047) => {
                                        Some((source_ml_source2_data_property_value29047,))
                                    }
                                    _ => {
                                        None::<(bool,)>
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
                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value29050) => {
                                                        Some((source_ml_source2_data_property_value29050,))
                                                    }
                                                    _ => {
                                                        None::<(bool,)>
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
                                                            let object = {
                                                                let source_ml_source2_array_read_value29057 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index29058 = self::find_java_script_heap_directory_object(shared_constructor_field1.clone(), identity.clone());
                                                                    { let ml_reference_array1592 = source_ml_source2_array_read_value29057.clone(); let ml_reference_position1592 = source_ml_source2_array_read_index29058; let ml_reference_projection1592 = self::ml_array::optional_at(&*ml_reference_array1592.borrow(), self::ml_array::number_index(ml_reference_position1592)); ml_reference_projection1592 }
                                                                }
                                                            };
                                                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::ml_string::Text::from_units(vec![110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 104, 101, 97, 112])
                                                            } else {
                                                                {
                                                                    let property = {
                                                                        let source_ml_source2_array_read_value29085 = match object.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value29059, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value29059.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value29060, _, _, _) => {
                                                                                source_ml_source2_data_property_value29060.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value29061) => {
                                                                                source_ml_source2_data_property_value29061.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value29062, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value29062.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value29063, _) => {
                                                                                source_ml_source2_data_property_value29063.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value29064, _) => {
                                                                                source_ml_source2_data_property_value29064.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value29065, _) => {
                                                                                source_ml_source2_data_property_value29065.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value29066, _) => {
                                                                                source_ml_source2_data_property_value29066.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value29067) => {
                                                                                source_ml_source2_data_property_value29067.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value29068, _, _) => {
                                                                                source_ml_source2_data_property_value29068.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value29069, _, _) => {
                                                                                source_ml_source2_data_property_value29069.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value29070, _, _) => {
                                                                                source_ml_source2_data_property_value29070.clone()
                                                                            }
                                                                            self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value29071, _, _) => {
                                                                                source_ml_source2_data_property_value29071.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_array_read_index29086 = self::find_java_script_heap_directory_property(match object.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value29072, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value29072.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value29073, _, _, _) => {
                                                                                    source_ml_source2_data_property_value29073.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value29074) => {
                                                                                    source_ml_source2_data_property_value29074.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value29075, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value29075.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value29076, _) => {
                                                                                    source_ml_source2_data_property_value29076.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value29077, _) => {
                                                                                    source_ml_source2_data_property_value29077.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value29078, _) => {
                                                                                    source_ml_source2_data_property_value29078.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value29079, _) => {
                                                                                    source_ml_source2_data_property_value29079.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value29080) => {
                                                                                    source_ml_source2_data_property_value29080.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value29081, _, _) => {
                                                                                    source_ml_source2_data_property_value29081.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value29082, _, _) => {
                                                                                    source_ml_source2_data_property_value29082.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value29083, _, _) => {
                                                                                    source_ml_source2_data_property_value29083.clone()
                                                                                }
                                                                                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value29084, _, _) => {
                                                                                    source_ml_source2_data_property_value29084.clone()
                                                                                }
                                                                            }, self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))));
                                                                            { let ml_reference_array1593 = source_ml_source2_array_read_value29085.clone(); let ml_reference_position1593 = source_ml_source2_array_read_index29086; let ml_reference_projection1593 = self::ml_array::optional_at(&*ml_reference_array1593.borrow(), self::ml_array::number_index(ml_reference_position1593)); ml_reference_projection1593 }
                                                                        }
                                                                    };
                                                                    if ((match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !((({ let ml_reference_value1594 = (match property.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value29087) => {
                                                                            source_ml_source2_data_property_value29087.clone()
                                                                        }
                                                                    }).borrow().clone(); ml_reference_value1594 }).0).9)) || !(self::test_java_script_semantic_same_value((({ let ml_reference_value1595 = (match property.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value29088) => {
                                                                            source_ml_source2_data_property_value29088.clone()
                                                                        }
                                                                    }).borrow().clone(); ml_reference_value1595 }).0).8, prototype.clone()))) {
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
                            self::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Result::MlSource1TemporaryReadJavaScriptErrorConstructionOperandsErrorLoop1259Return(source_ml_source1_temporary_result1259) => {
                                source_ml_source1_temporary_result1259.clone()
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_allocation_result_error(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
    prototype: self::JavaScriptSemanticValue,
    before: self::JavaScriptErrorAllocationDirectory,
) -> self::ml_string::Text {
    match before.clone() {
        self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(
            shared_constructor_field2,
            shared_constructor_field3,
        ) => match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let prior = self::read_java_script_error_prior_directory_error(
                    heap.clone(),
                    before.clone(),
                );
                if (prior != (self::ml_string::Text::from_units(vec![]))) {
                    prior.clone()
                } else {
                    {
                        let error = self::read_java_script_heap_directory_value_error(
                            heap.clone(),
                            value.clone(),
                        );
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            error.clone()
                        } else {
                            {
                                let invalid_result = self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32,
                                    69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32,
                                    114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32,
                                    97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32,
                                    110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 103,
                                    108, 111, 98, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105,
                                    100, 101, 110, 116, 105, 116, 121,
                                ]);
                                match value.clone() {
                                    self::JavaScriptSemanticValue::Object(identity, callable) => {
                                        if ((callable
                                            || {
                                                let ml_values1596 = {
                                                    let ml_reference_value1597 =
                                                        shared_constructor_field2.borrow().clone();
                                                    ml_reference_value1597
                                                };
                                                let ml_callback1596 = {
                                                    let identity = identity.clone();
                                                    std::rc::Rc::new(move |row: self::JavaScriptErrorAllocationIdentity| -> bool { ((match row.clone() {
                                                    self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(source_ml_source2_data_property_value29091, _, _) => {
                                                        source_ml_source2_data_property_value29091.clone()
                                                    }
                                                }) == identity) })
                                                };
                                                ml_values1596.iter().cloned().enumerate().any(
                                                    |(ml_index1596, ml_value1596)| {
                                                        ml_callback1596(ml_value1596)
                                                    },
                                                )
                                            })
                                            || self::ml_array::includes(
                                                &{
                                                    let ml_reference_value1598 =
                                                        shared_constructor_field3.borrow().clone();
                                                    ml_reference_value1598
                                                },
                                                &identity.clone(),
                                                0f64,
                                                false,
                                                |a, b| a == b,
                                            ))
                                        {
                                            invalid_result.clone()
                                        } else {
                                            {
                                                let object = {
                                                    let source_ml_source2_array_read_value29096 =
                                                        shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_array_read_index29097 = self::find_java_script_heap_directory_object(shared_constructor_field1.clone(), identity.clone());
                                                        {
                                                            let ml_reference_array1599 = source_ml_source2_array_read_value29096.clone();
                                                            let ml_reference_position1599 = source_ml_source2_array_read_index29097;
                                                            let ml_reference_projection1599 =
                                                                self::ml_array::optional_at(
                                                                    &*ml_reference_array1599
                                                                        .borrow(),
                                                                    self::ml_array::number_index(
                                                                        ml_reference_position1599,
                                                                    ),
                                                                );
                                                            ml_reference_projection1599
                                                        }
                                                    }
                                                };
                                                if match (object.clone(), None::<()>) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                } {
                                                    self::ml_string::Text::from_units(vec![
                                                        116, 104, 101, 32, 105, 110, 116, 114, 105,
                                                        110, 115, 105, 99, 32, 69, 114, 114, 111,
                                                        114, 32, 114, 101, 115, 117, 108, 116, 32,
                                                        114, 101, 116, 97, 105, 110, 115, 32, 105,
                                                        116, 115, 32, 111, 112, 97, 113, 117, 101,
                                                        32, 104, 111, 115, 116, 32, 112, 114, 111,
                                                        116, 111, 99, 111, 108, 32, 97, 110, 100,
                                                        32, 115, 101, 108, 101, 99, 116, 101, 100,
                                                        32, 97, 99, 116, 117, 97, 108, 32, 100,
                                                        101, 114, 105, 118, 101, 100, 32, 112, 114,
                                                        111, 116, 111, 116, 121, 112, 101,
                                                    ])
                                                } else {
                                                    if (match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            } && self::test_java_script_semantic_same_value((((match object.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::HostErrorObject(_, source_ml_source2_data_property_value29105, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value29105 = *source_ml_source2_data_property_value29105;
                                                                    Some((source_ml_source2_data_property_value29105.clone(),))
                                                                }
                                                                _ => {
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
                                    _ => invalid_result.clone(),
                                }
                            }
                        }
                    }
                }
            }
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_integer(value: f64) -> f64 {
    if value.is_nan() { 0f64 } else { value.trunc() }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_slice_position(value: f64, length: f64) -> f64 {
    {
        let integer = self::read_utf16_text_integer(value);
        if (integer < (0f64)) {
            self::ml_math::max((length + integer), 0f64)
        } else {
            self::ml_math::min(integer, length)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn slice_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    start: f64,
    end: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let first = self::read_utf16_text_slice_position(start, {
            let ml_reference_array1600 = text.clone();
            let ml_reference_projection1600 = ml_reference_array1600.borrow().len() as f64;
            ml_reference_projection1600
        });
        {
            let last = self::read_utf16_text_slice_position(end, {
                let ml_reference_array1601 = text.clone();
                let ml_reference_projection1601 = ml_reference_array1601.borrow().len() as f64;
                ml_reference_projection1601
            });
            {
                let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                {
                    let source_ml_source2_s29106 =
                        self::source_ml_source1_temporary_slice_utf16_text_loop1260(
                            text.clone(),
                            last,
                            result.clone(),
                            first,
                        );
                    match source_ml_source2_s29106.clone() {
                        self::MlSource1TemporarySliceUtf16TextLoop1260Result::MlSource1TemporarySliceUtf16TextLoop1260Done => {
                            result.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    position: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64))
            || (at
                >= ({
                    let ml_reference_array1602 = text.clone();
                    let ml_reference_projection1602 = ml_reference_array1602.borrow().len() as f64;
                    ml_reference_projection1602
                })))
        {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
        } else {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                ({
                    let ml_reference_array1603 = text.clone();
                    let ml_reference_position1603 = at;
                    let ml_reference_projection1603 = self::ml_array::optional_at(
                        &*ml_reference_array1603.borrow(),
                        self::ml_array::number_index(ml_reference_position1603),
                    );
                    ml_reference_projection1603
                })
                .clone()
                .unwrap_or_else(|| 0f64),
            ]))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn relative_character_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    position: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>> {
    {
        let integer = self::read_utf16_text_integer(position);
        {
            let at = if (integer < (0f64)) {
                ({
                    let ml_reference_array1604 = text.clone();
                    let ml_reference_projection1604 = ml_reference_array1604.borrow().len() as f64;
                    ml_reference_projection1604
                } + integer)
            } else {
                integer
            };
            if ((at < (0f64))
                || (at
                    >= ({
                        let ml_reference_array1605 = text.clone();
                        let ml_reference_projection1605 =
                            ml_reference_array1605.borrow().len() as f64;
                        ml_reference_projection1605
                    })))
            {
                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
                >::new()))
            } else {
                std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(
                    std::cell::RefCell::new(vec![
                        ({
                            let ml_reference_array1606 = text.clone();
                            let ml_reference_position1606 = at;
                            let ml_reference_projection1606 = self::ml_array::optional_at(
                                &*ml_reference_array1606.borrow(),
                                self::ml_array::number_index(ml_reference_position1606),
                            );
                            ml_reference_projection1606
                        })
                        .clone()
                        .unwrap_or_else(|| 0f64),
                    ]),
                )]))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn code_point_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    position: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64))
            || (at
                >= ({
                    let ml_reference_array1607 = text.clone();
                    let ml_reference_projection1607 = ml_reference_array1607.borrow().len() as f64;
                    ml_reference_projection1607
                })))
        {
            std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))
        } else {
            {
                let first = ({
                    let ml_reference_array1608 = text.clone();
                    let ml_reference_position1608 = at;
                    let ml_reference_projection1608 = self::ml_array::optional_at(
                        &*ml_reference_array1608.borrow(),
                        self::ml_array::number_index(ml_reference_position1608),
                    );
                    ml_reference_projection1608
                })
                .clone()
                .unwrap_or_else(|| 0f64);
                {
                    let second = ({
                        let source_ml_source2_array_read_index29114 = (at + 1f64);
                        {
                            let ml_reference_array1609 = text.clone();
                            let ml_reference_position1609 = source_ml_source2_array_read_index29114;
                            let ml_reference_projection1609 = self::ml_array::optional_at(
                                &*ml_reference_array1609.borrow(),
                                self::ml_array::number_index(ml_reference_position1609),
                            );
                            ml_reference_projection1609
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64));
                    std::rc::Rc::new(std::cell::RefCell::new(vec![if ((((first >= (55296f64))
                        && (first <= (56319f64)))
                        && (second >= (56320f64)))
                        && (second <= (57343f64)))
                    {
                        (((65536f64 + ((first - 55296f64) * 1024f64)) + second) - 56320f64)
                    } else {
                        first
                    }]))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn match_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    start: f64,
) -> bool {
    if ((start < (0f64))
        || ((start + {
            let ml_reference_array1610 = search.clone();
            let ml_reference_projection1610 = ml_reference_array1610.borrow().len() as f64;
            ml_reference_projection1610
        }) > ({
            let ml_reference_array1611 = text.clone();
            let ml_reference_projection1611 = ml_reference_array1611.borrow().len() as f64;
            ml_reference_projection1611
        })))
    {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s29118 =
                    self::source_ml_source1_temporary_match_utf16_text_loop1270(
                        text.clone(),
                        search.clone(),
                        start,
                        at,
                    );
                match source_ml_source2_s29118.clone() {
                    self::MlSource1TemporaryMatchUtf16TextLoop1270Result::MlSource1TemporaryMatchUtf16TextLoop1270Done => {
                        true
                    }
                    self::MlSource1TemporaryMatchUtf16TextLoop1270Result::MlSource1TemporaryMatchUtf16TextLoop1270Return(source_ml_source1_temporary_result1270) => {
                        source_ml_source1_temporary_result1270
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn index_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    position: f64,
) -> f64 {
    {
        let start = self::ml_math::min(
            self::ml_math::max(self::read_utf16_text_integer(position), 0f64),
            {
                let ml_reference_array1612 = text.clone();
                let ml_reference_projection1612 = ml_reference_array1612.borrow().len() as f64;
                ml_reference_projection1612
            },
        );
        {
            let source_ml_source2_s29119 =
                self::source_ml_source1_temporary_index_utf16_text_loop1271(
                    text.clone(),
                    search.clone(),
                    start,
                );
            match source_ml_source2_s29119.clone() {
                self::MlSource1TemporaryIndexUtf16TextLoop1271Result::MlSource1TemporaryIndexUtf16TextLoop1271Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryIndexUtf16TextLoop1271Result::MlSource1TemporaryIndexUtf16TextLoop1271Return(source_ml_source1_temporary_result1271) => {
                    source_ml_source1_temporary_result1271
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn last_index_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    position: f64,
) -> f64 {
    {
        let integer = if position.is_nan() {
            f64::INFINITY
        } else {
            self::read_utf16_text_integer(position)
        };
        {
            let start = self::ml_math::min(
                self::ml_math::max(integer, 0f64),
                ({
                    let ml_reference_array1613 = text.clone();
                    let ml_reference_projection1613 = ml_reference_array1613.borrow().len() as f64;
                    ml_reference_projection1613
                } - {
                    let ml_reference_array1614 = search.clone();
                    let ml_reference_projection1614 = ml_reference_array1614.borrow().len() as f64;
                    ml_reference_projection1614
                }),
            );
            {
                let source_ml_source2_s29120 =
                    self::source_ml_source1_temporary_last_index_utf16_text_loop1272(
                        text.clone(),
                        search.clone(),
                        start,
                    );
                match source_ml_source2_s29120.clone() {
                    self::MlSource1TemporaryLastIndexUtf16TextLoop1272Result::MlSource1TemporaryLastIndexUtf16TextLoop1272Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryLastIndexUtf16TextLoop1272Result::MlSource1TemporaryLastIndexUtf16TextLoop1272Return(source_ml_source1_temporary_result1272) => {
                        source_ml_source1_temporary_result1272
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_utf16_text(
    left: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    right: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> f64 {
    {
        let count = self::ml_math::min(
            {
                let ml_reference_array1615 = left.clone();
                let ml_reference_projection1615 = ml_reference_array1615.borrow().len() as f64;
                ml_reference_projection1615
            },
            {
                let ml_reference_array1616 = right.clone();
                let ml_reference_projection1616 = ml_reference_array1616.borrow().len() as f64;
                ml_reference_projection1616
            },
        );
        {
            let at = 0f64;
            {
                let source_ml_source2_s29121 =
                    self::source_ml_source1_temporary_compare_utf16_text_loop1273(
                        left.clone(),
                        right.clone(),
                        count,
                        at,
                    );
                match source_ml_source2_s29121.clone() {
                    self::MlSource1TemporaryCompareUtf16TextLoop1273Result::MlSource1TemporaryCompareUtf16TextLoop1273Done => {
                        if (({ let ml_reference_array1617 = left.clone(); let ml_reference_projection1617 = ml_reference_array1617.borrow().len() as f64; ml_reference_projection1617 }) == ({ let ml_reference_array1618 = right.clone(); let ml_reference_projection1618 = ml_reference_array1618.borrow().len() as f64; ml_reference_projection1618 })) {
                            0f64
                        } else {
                            if (({ let ml_reference_array1619 = left.clone(); let ml_reference_projection1619 = ml_reference_array1619.borrow().len() as f64; ml_reference_projection1619 }) < ({ let ml_reference_array1620 = right.clone(); let ml_reference_projection1620 = ml_reference_array1620.borrow().len() as f64; ml_reference_projection1620 })) {
                                (-1f64)
                            } else {
                                1f64
                            }
                        }
                    }
                    self::MlSource1TemporaryCompareUtf16TextLoop1273Result::MlSource1TemporaryCompareUtf16TextLoop1273Return(source_ml_source1_temporary_result1273) => {
                        source_ml_source1_temporary_result1273
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn starts_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> bool {
    self::match_utf16_text(text.clone(), search.clone(), 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ends_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> bool {
    self::match_utf16_text(
        text.clone(),
        search.clone(),
        ({
            let ml_reference_array1621 = text.clone();
            let ml_reference_projection1621 = ml_reference_array1621.borrow().len() as f64;
            ml_reference_projection1621
        } - {
            let ml_reference_array1622 = search.clone();
            let ml_reference_projection1622 = ml_reference_array1622.borrow().len() as f64;
            ml_reference_projection1622
        }),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn includes_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    search: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> bool {
    ((self::index_utf16_text(text.clone(), search.clone(), 0f64)) >= (0f64))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn trim_utf16_text(
    text: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    start: bool,
    end: bool,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let first = 0f64;
        {
            let last = {
                let ml_reference_array1623 = text.clone();
                let ml_reference_projection1623 = ml_reference_array1623.borrow().len() as f64;
                ml_reference_projection1623
            };
            {
                let first_3 = if start {
                    {
                        let first_2 = self::source_ml_source1_temporary_trim_utf16_text_loop1280(
                            text.clone(),
                            first,
                            last,
                        );
                        first_2
                    }
                } else {
                    first
                };
                {
                    let last_3 = if end {
                        {
                            let last_2 = self::source_ml_source1_temporary_trim_utf16_text_loop1281(
                                text.clone(),
                                first_3,
                                last,
                            );
                            last_2
                        }
                    } else {
                        last
                    };
                    self::slice_utf16_text(text.clone(), first_3, last_3)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_units_utf16_text(
    values: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let result = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let at = 0f64;
            {
                let source_ml_source2_s29123 =
                    self::source_ml_source1_temporary_character_units_utf16_text_loop1282(
                        values.clone(),
                        result.clone(),
                        at,
                    );
                match source_ml_source2_s29123.clone() {
                    self::MlSource1TemporaryCharacterUnitsUtf16TextLoop1282Result::MlSource1TemporaryCharacterUnitsUtf16TextLoop1282Done => {
                        result.clone()
                    }
                }
            }
        }
    }
}
