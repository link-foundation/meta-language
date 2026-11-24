// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::construct_java_script_wrapper_index_key;
use super::construct_java_script_wrapper_string_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_unit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocation_request_loop362(
    mut properties: f64,
    mut source_ml_source1_temporary_values361: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index361: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index361
            < ({
                let ml_reference_array2789 = source_ml_source1_temporary_values361.clone();
                let ml_reference_projection2789 =
                    self::ml::Big::from_u128(ml_reference_array2789.borrow().len() as u128);
                ml_reference_projection2789
            })) {
            {
                let object = {
                    let ml_reference_array2790 = source_ml_source1_temporary_values361.clone();
                    let ml_reference_position2790 = source_ml_source1_temporary_index361.clone();
                    let ml_reference_projection2790 = self::ml_array::at(
                        &*ml_reference_array2790.borrow(),
                        ml_reference_position2790.to_index(),
                    );
                    ml_reference_projection2790
                };
                {
                    let properties_2 = (properties + {
                        let ml_reference_array2791 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40549,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40549.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40550,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40550.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40551,
                            ) => source_ml_source2_data_property_value40551.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40552,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40552.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40553,
                                _,
                            ) => source_ml_source2_data_property_value40553.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40554,
                                _,
                            ) => source_ml_source2_data_property_value40554.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40555,
                                _,
                            ) => source_ml_source2_data_property_value40555.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40556,
                                _,
                            ) => source_ml_source2_data_property_value40556.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40557,
                            ) => source_ml_source2_data_property_value40557.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40558,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40558.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40559,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40559.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40560,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40560.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40561,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40561.clone(),
                        };
                        let ml_reference_projection2791 =
                            ml_reference_array2791.borrow().len() as f64;
                        ml_reference_projection2791
                    });
                    {
                        let source_ml_source1_temporary_index361_2 =
                            source_ml_source1_temporary_index361.add(&self::ml::Big::from_i128(1));
                        {
                            (properties, source_ml_source1_temporary_index361) =
                                (properties_2, source_ml_source1_temporary_index361_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            properties
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptArraySlicePropertyAccessLoop366Result {
    MlSource1TemporaryStepJavaScriptArraySlicePropertyAccessLoop366Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_array_slice_property_access_loop366(
    name: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryStepJavaScriptArraySlicePropertyAccessLoop366Result {
    loop {
        return if (at < (name.length())) {
            {
                let source_ml_source1_temporary_step_java_script_array_slice_property_access_ignored367 = {
                    let ml_method_owner2792 = units.clone();
                    let ml_method_argument2792_0 = {
                        let source_ml_source2_string_unit_operand40562 = name.clone();
                        {
                            let source_ml_source2_string_unit_operand40563 = at;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if ((index < (0f64)) || (index >= (text.length()))) {
                                            f64::NAN
                                        } else {
                                            ({
                                    let source_ml_source2_string_unit_operand40569 = {
                                        let source_ml_source2_string_range_operand40564 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand40565 = index;
                                            {
                                                let source_ml_source2_string_range_operand40566 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position40567 = source_ml_source2_string_range_operand40565;
                                                    {
                                                        let source_ml_source2_string_range_position40568 = source_ml_source2_string_range_operand40566;
                                                        source_ml_source2_string_range_operand40564.slice(source_ml_source2_string_range_position40567, source_ml_source2_string_range_position40568)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand40570 = 0f64;
                                        source_ml_source2_string_unit_operand40569.code_point_at(source_ml_source2_string_unit_operand40570)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand40562.clone(),
                                source_ml_source2_string_unit_operand40563,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument2792_0];
                    let mut ml_method_values2792 = ml_method_owner2792.borrow_mut();
                    let ml_length = ml_method_values2792
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2792.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryStepJavaScriptArraySlicePropertyAccessLoop366Result::MlSource1TemporaryStepJavaScriptArraySlicePropertyAccessLoop366Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Result {
    MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Done,
    MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_synchronous_iterator_record_error_loop376(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values375: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index375: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Result {
    loop {
        return if (source_ml_source1_temporary_index375
            < ({
                let ml_reference_array2793 = source_ml_source1_temporary_values375.clone();
                let ml_reference_projection2793 =
                    self::ml::Big::from_u128(ml_reference_array2793.borrow().len() as u128);
                ml_reference_projection2793
            })) {
            {
                let value = {
                    let ml_reference_array2794 = source_ml_source1_temporary_values375.clone();
                    let ml_reference_position2794 = source_ml_source1_temporary_index375.clone();
                    let ml_reference_projection2794 = self::ml_array::at(
                        &*ml_reference_array2794.borrow(),
                        ml_reference_position2794.to_index(),
                    );
                    ml_reference_projection2794
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Result::MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index375_2 =
                                source_ml_source1_temporary_index375
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index375 =
                                    source_ml_source1_temporary_index375_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Result::MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381 {
    MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Next,
    MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Result {
    MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Done,
    MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_iterator_protocol_state_error_loop382(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values378: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index378: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Result {
    loop {
        return if (source_ml_source1_temporary_index378
            < ({
                let ml_reference_array2795 = source_ml_source1_temporary_values378.clone();
                let ml_reference_projection2795 =
                    self::ml::Big::from_u128(ml_reference_array2795.borrow().len() as u128);
                ml_reference_projection2795
            })) {
            {
                let value = {
                    let ml_reference_array2796 = source_ml_source1_temporary_values378.clone();
                    let ml_reference_position2796 = source_ml_source1_temporary_index378.clone();
                    let ml_reference_projection2796 = self::ml_array::at(
                        &*ml_reference_array2796.borrow(),
                        ml_reference_position2796.to_index(),
                    );
                    ml_reference_projection2796
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Result::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index378_2 =
                                source_ml_source1_temporary_index378
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index378 =
                                    source_ml_source1_temporary_index378_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Result::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380 {
    MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380Next,
    MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Result {
    MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Done,
    MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_iterator_protocol_state_error_loop379(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values377: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index377: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Result {
    loop {
        return if (source_ml_source1_temporary_index377
            < ({
                let ml_reference_array2797 = source_ml_source1_temporary_values377.clone();
                let ml_reference_projection2797 =
                    self::ml::Big::from_u128(ml_reference_array2797.borrow().len() as u128);
                ml_reference_projection2797
            })) {
            {
                let value = {
                    let ml_reference_array2798 = source_ml_source1_temporary_values377.clone();
                    let ml_reference_position2798 = source_ml_source1_temporary_index377.clone();
                    let ml_reference_projection2798 = self::ml_array::at(
                        &*ml_reference_array2798.borrow(),
                        ml_reference_position2798.to_index(),
                    );
                    ml_reference_projection2798
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Result::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index377_2 =
                                source_ml_source1_temporary_index377
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index377 =
                                    source_ml_source1_temporary_index377_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Result::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386 {
    MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386Next,
    MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Result {
    MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Done,
    MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_primitive_wrapper_state_error_loop385(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values384: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index384: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Result {
    loop {
        return if (source_ml_source1_temporary_index384
            < ({
                let ml_reference_array2799 = source_ml_source1_temporary_values384.clone();
                let ml_reference_projection2799 =
                    self::ml::Big::from_u128(ml_reference_array2799.borrow().len() as u128);
                ml_reference_projection2799
            })) {
            {
                let value = {
                    let ml_reference_array2800 = source_ml_source1_temporary_values384.clone();
                    let ml_reference_position2800 = source_ml_source1_temporary_index384.clone();
                    let ml_reference_projection2800 = self::ml_array::at(
                        &*ml_reference_array2800.borrow(),
                        ml_reference_position2800.to_index(),
                    );
                    ml_reference_projection2800
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Result::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index384_2 =
                                source_ml_source1_temporary_index384
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index384 =
                                    source_ml_source1_temporary_index384_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Result::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_primitive_wrapper_work_error_loop388(
    mut total: f64,
    mut source_ml_source1_temporary_values387: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index387: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index387
            < ({
                let ml_reference_array2801 = source_ml_source1_temporary_values387.clone();
                let ml_reference_projection2801 =
                    self::ml::Big::from_u128(ml_reference_array2801.borrow().len() as u128);
                ml_reference_projection2801
            })) {
            {
                let object = {
                    let ml_reference_array2802 = source_ml_source1_temporary_values387.clone();
                    let ml_reference_position2802 = source_ml_source1_temporary_index387.clone();
                    let ml_reference_projection2802 = self::ml_array::at(
                        &*ml_reference_array2802.borrow(),
                        ml_reference_position2802.to_index(),
                    );
                    ml_reference_projection2802
                };
                {
                    let total_2 = (total + {
                        let ml_reference_array2803 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40581,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40581.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40582,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40582.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40583,
                            ) => source_ml_source2_data_property_value40583.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40584,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40584.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40585,
                                _,
                            ) => source_ml_source2_data_property_value40585.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40586,
                                _,
                            ) => source_ml_source2_data_property_value40586.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40587,
                                _,
                            ) => source_ml_source2_data_property_value40587.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40588,
                                _,
                            ) => source_ml_source2_data_property_value40588.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40589,
                            ) => source_ml_source2_data_property_value40589.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40590,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40590.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40591,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40591.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40592,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40592.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40593,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40593.clone(),
                        };
                        let ml_reference_projection2803 =
                            ml_reference_array2803.borrow().len() as f64;
                        ml_reference_projection2803
                    });
                    {
                        let source_ml_source1_temporary_index387_2 =
                            source_ml_source1_temporary_index387.add(&self::ml::Big::from_i128(1));
                        {
                            (total, source_ml_source1_temporary_index387) =
                                (total_2, source_ml_source1_temporary_index387_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            total
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop390Result {
    MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop390Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_wrapper_index_key_loop390(
    mut reversed: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop390Result {
    loop {
        return if (at >= (0f64)) {
            {
                let source_ml_source1_temporary_construct_java_script_wrapper_index_key_ignored391 = {
                    let ml_method_owner2804 = units.clone();
                    let ml_method_argument2804_0 = ({
                        let ml_reference_array2805 = reversed.clone();
                        let ml_reference_position2805 = at;
                        let ml_reference_projection2805 = self::ml_array::optional_at(
                            &*ml_reference_array2805.borrow(),
                            self::ml_array::number_index(ml_reference_position2805),
                        );
                        ml_reference_projection2805
                    })
                    .clone()
                    .unwrap_or_else(|| 48f64);
                    let ml_inserted = vec![ml_method_argument2804_0];
                    let mut ml_method_values2804 = ml_method_owner2804.borrow_mut();
                    let ml_length = ml_method_values2804
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2804.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop390Result::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop390Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop389Result {
    MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop389Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_wrapper_index_key_loop389(
    mut reversed: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut remaining: f64,
) -> self::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop389Result {
    loop {
        return {
            let digit = (remaining % 10f64);
            {
                let source_ml_source1_temporary_construct_java_script_wrapper_index_key_ignored392 = {
                    let ml_method_owner2806 = reversed.clone();
                    let ml_method_argument2806_0 = (48f64 + digit);
                    let ml_inserted = vec![ml_method_argument2806_0];
                    let mut ml_method_values2806 = ml_method_owner2806.borrow_mut();
                    let ml_length = ml_method_values2806
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2806.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let remaining_2 = (remaining / 10f64).floor();
                    if (remaining_2 > (0f64)) {
                        {
                            remaining = remaining_2;
                            continue;
                        }
                    } else {
                        self::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop389Result::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop389Done
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesJoin395 {
    MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesJoin395Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesLoop396Result {
    MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesLoop396Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_wrapper_own_properties_loop396(
    mut input: self::JavaScriptSemanticValue,
    mut length: f64,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut index: f64,
) -> self::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesLoop396Result {
    loop {
        return if (index < length) {
            {
                let source_ml_source1_temporary_construct_java_script_wrapper_own_properties_ignored398 = {
                    let ml_method_owner2807 = properties.clone();
                    let ml_method_argument2807_0 =
                        self::construct_java_script_wrapper_string_property(
                            self::construct_java_script_wrapper_index_key(index),
                            self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                std::cell::RefCell::new(vec![
                                    self::read_java_script_semantic_text_unit(input.clone(), index),
                                ]),
                            )),
                            true,
                        );
                    let ml_inserted = vec![ml_method_argument2807_0];
                    let mut ml_method_values2807 = ml_method_owner2807.borrow_mut();
                    let ml_length = ml_method_values2807
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2807.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesLoop396Result::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesLoop396Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402 {
    MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402Next,
    MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401 {
    MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401Next,
    MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Result {
    MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Done,
    MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Return(self::ml_string::Text),
}
