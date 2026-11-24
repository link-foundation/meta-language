// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConcatenationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_object_spread_loop319(
    mut count: f64,
    mut source_ml_source1_temporary_values312: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index312: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index312
            < ({
                let ml_reference_array2770 = source_ml_source1_temporary_values312.clone();
                let ml_reference_projection2770 =
                    self::ml::Big::from_u128(ml_reference_array2770.borrow().len() as u128);
                ml_reference_projection2770
            })) {
            {
                let object = {
                    let ml_reference_array2771 = source_ml_source1_temporary_values312.clone();
                    let ml_reference_position2771 = source_ml_source1_temporary_index312.clone();
                    let ml_reference_projection2771 = self::ml_array::at(
                        &*ml_reference_array2771.borrow(),
                        ml_reference_position2771.to_index(),
                    );
                    ml_reference_projection2771
                };
                {
                    let count_2 = (count + {
                        let ml_reference_array2772 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40494,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40494.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40495,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40495.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40496,
                            ) => source_ml_source2_data_property_value40496.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40497,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40497.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40498,
                                _,
                            ) => source_ml_source2_data_property_value40498.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40499,
                                _,
                            ) => source_ml_source2_data_property_value40499.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40500,
                                _,
                            ) => source_ml_source2_data_property_value40500.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40501,
                                _,
                            ) => source_ml_source2_data_property_value40501.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40502,
                            ) => source_ml_source2_data_property_value40502.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40503,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40503.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40504,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40504.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40505,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40505.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40506,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40506.clone(),
                        };
                        let ml_reference_projection2772 =
                            ml_reference_array2772.borrow().len() as f64;
                        ml_reference_projection2772
                    });
                    {
                        let source_ml_source1_temporary_index312_2 =
                            source_ml_source1_temporary_index312.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index312) =
                                (count_2, source_ml_source1_temporary_index312_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            count
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327 {
    MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327Next,
    MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin327Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326 {
    MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326Next,
    MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin326Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325 {
    MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325Next,
    MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorJoin325Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Result {
    MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Done,
    MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_concatenation_state_error_loop324(
    mut source_ml_source1_temporary_values321: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_source1_temporary_index321: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Result {
    loop {
        return if (source_ml_source1_temporary_index321
            < ({
                let ml_reference_array2773 = source_ml_source1_temporary_values321.clone();
                let ml_reference_projection2773 =
                    self::ml::Big::from_u128(ml_reference_array2773.borrow().len() as u128);
                ml_reference_projection2773
            })) {
            {
                let coordinate = {
                    let ml_reference_array2774 = source_ml_source1_temporary_values321.clone();
                    let ml_reference_position2774 = source_ml_source1_temporary_index321.clone();
                    let ml_reference_projection2774 = self::ml_array::at(
                        &*ml_reference_array2774.borrow(),
                        ml_reference_position2774.to_index(),
                    );
                    ml_reference_projection2774
                };
                if ((!(self::ml_math::is_safe_integer(coordinate)) || (coordinate < (0f64))) || {
                    let source_ml_source2_same_value_operand40510 = (-0f64);
                    (((coordinate == source_ml_source2_same_value_operand40510)
                        && ((coordinate != (0f64))
                            || ((1f64 / coordinate)
                                == (1f64 / source_ml_source2_same_value_operand40510))))
                        || ((coordinate != coordinate)
                            && (source_ml_source2_same_value_operand40510
                                != source_ml_source2_same_value_operand40510)))
                }) {
                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Result::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 32, 110, 111, 110, 110, 101, 103, 97, 116, 105, 118, 101, 32, 115, 97, 102, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index321_2 =
                            source_ml_source1_temporary_index321.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index321 =
                                source_ml_source1_temporary_index321_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Result::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop324Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Result {
    MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Done,
    MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_concatenation_state_error_loop323(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values320: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index320: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Result {
    loop {
        return if (source_ml_source1_temporary_index320
            < ({
                let ml_reference_array2775 = source_ml_source1_temporary_values320.clone();
                let ml_reference_projection2775 =
                    self::ml::Big::from_u128(ml_reference_array2775.borrow().len() as u128);
                ml_reference_projection2775
            })) {
            {
                let value = {
                    let ml_reference_array2776 = source_ml_source1_temporary_values320.clone();
                    let ml_reference_position2776 = source_ml_source1_temporary_index320.clone();
                    let ml_reference_projection2776 = self::ml_array::at(
                        &*ml_reference_array2776.borrow(),
                        ml_reference_position2776.to_index(),
                    );
                    ml_reference_projection2776
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Result::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index320_2 =
                                source_ml_source1_temporary_index320
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index320 =
                                    source_ml_source1_temporary_index320_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Result::MlSource1TemporaryReadJavaScriptArrayConcatenationStateErrorLoop323Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334 {
    MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Next,
    MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Result {
    MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Done,
    MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_concatenation_child_error_loop335(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values328: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index328: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Result {
    loop {
        return if (source_ml_source1_temporary_index328
            < ({
                let ml_reference_array2777 = source_ml_source1_temporary_values328.clone();
                let ml_reference_projection2777 =
                    self::ml::Big::from_u128(ml_reference_array2777.borrow().len() as u128);
                ml_reference_projection2777
            })) {
            {
                let value = {
                    let ml_reference_array2778 = source_ml_source1_temporary_values328.clone();
                    let ml_reference_position2778 = source_ml_source1_temporary_index328.clone();
                    let ml_reference_projection2778 = self::ml_array::at(
                        &*ml_reference_array2778.borrow(),
                        ml_reference_position2778.to_index(),
                    );
                    ml_reference_projection2778
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Result::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index328_2 =
                                source_ml_source1_temporary_index328
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index328 =
                                    source_ml_source1_temporary_index328_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Result::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333 {
    MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333Next,
    MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_request_java_script_array_concatenation_allocation_loop337(
    mut properties: f64,
    mut source_ml_source1_temporary_values336: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index336: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index336
            < ({
                let ml_reference_array2779 = source_ml_source1_temporary_values336.clone();
                let ml_reference_projection2779 =
                    self::ml::Big::from_u128(ml_reference_array2779.borrow().len() as u128);
                ml_reference_projection2779
            })) {
            {
                let object = {
                    let ml_reference_array2780 = source_ml_source1_temporary_values336.clone();
                    let ml_reference_position2780 = source_ml_source1_temporary_index336.clone();
                    let ml_reference_projection2780 = self::ml_array::at(
                        &*ml_reference_array2780.borrow(),
                        ml_reference_position2780.to_index(),
                    );
                    ml_reference_projection2780
                };
                {
                    let properties_2 = (properties + {
                        let ml_reference_array2781 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40517,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40517.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40518,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40518.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40519,
                            ) => source_ml_source2_data_property_value40519.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40520,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40520.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40521,
                                _,
                            ) => source_ml_source2_data_property_value40521.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40522,
                                _,
                            ) => source_ml_source2_data_property_value40522.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40523,
                                _,
                            ) => source_ml_source2_data_property_value40523.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40524,
                                _,
                            ) => source_ml_source2_data_property_value40524.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40525,
                            ) => source_ml_source2_data_property_value40525.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40526,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40526.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40527,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40527.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40528,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40528.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40529,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40529.clone(),
                        };
                        let ml_reference_projection2781 =
                            ml_reference_array2781.borrow().len() as f64;
                        ml_reference_projection2781
                    });
                    {
                        let source_ml_source1_temporary_index336_2 =
                            source_ml_source1_temporary_index336.add(&self::ml::Big::from_i128(1));
                        {
                            (properties, source_ml_source1_temporary_index336) =
                                (properties_2, source_ml_source1_temporary_index336_2.clone());
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341 {
    MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341Next(bool),
    MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341Return(
        Box<self::JavaScriptArrayConcatenationTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptArrayConcatenationElementLoop342Result {
    MlSource1TemporaryStepJavaScriptArrayConcatenationElementLoop342Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_array_concatenation_element_loop342(
    name: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryStepJavaScriptArrayConcatenationElementLoop342Result {
    loop {
        return if (index < (name.length())) {
            {
                let source_ml_source1_temporary_step_java_script_array_concatenation_element_ignored343 = {
                    let ml_method_owner2782 = units.clone();
                    let ml_method_argument2782_0 = {
                        let source_ml_source2_string_unit_operand40530 = name.clone();
                        {
                            let source_ml_source2_string_unit_operand40531 = index;
                            (std::rc::Rc::new(
                                move |text: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index_2 = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if ((index_2 < (0f64)) || (index_2 >= (text.length()))) {
                                            f64::NAN
                                        } else {
                                            ({
                                    let source_ml_source2_string_unit_operand40537 = {
                                        let source_ml_source2_string_range_operand40532 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand40533 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand40534 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position40535 = source_ml_source2_string_range_operand40533;
                                                    {
                                                        let source_ml_source2_string_range_position40536 = source_ml_source2_string_range_operand40534;
                                                        source_ml_source2_string_range_operand40532.slice(source_ml_source2_string_range_position40535, source_ml_source2_string_range_position40536)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand40538 = 0f64;
                                        source_ml_source2_string_unit_operand40537.code_point_at(source_ml_source2_string_unit_operand40538)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand40530.clone(),
                                source_ml_source2_string_unit_operand40531,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument2782_0];
                    let mut ml_method_values2782 = ml_method_owner2782.borrow_mut();
                    let ml_length = ml_method_values2782
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2782.extend(ml_inserted);
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
            self::MlSource1TemporaryStepJavaScriptArrayConcatenationElementLoop342Result::MlSource1TemporaryStepJavaScriptArrayConcatenationElementLoop342Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352 {
    MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352Next,
    MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin352Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351 {
    MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351Next,
    MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin351Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350 {
    MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350Next,
    MlSource1TemporaryReadJavaScriptArraySliceStateErrorJoin350Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Result {
    MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Done,
    MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_slice_state_error_loop349(
    mut source_ml_source1_temporary_values346: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_source1_temporary_index346: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Result {
    loop {
        return if (source_ml_source1_temporary_index346
            < ({
                let ml_reference_array2783 = source_ml_source1_temporary_values346.clone();
                let ml_reference_projection2783 =
                    self::ml::Big::from_u128(ml_reference_array2783.borrow().len() as u128);
                ml_reference_projection2783
            })) {
            {
                let coordinate = {
                    let ml_reference_array2784 = source_ml_source1_temporary_values346.clone();
                    let ml_reference_position2784 = source_ml_source1_temporary_index346.clone();
                    let ml_reference_projection2784 = self::ml_array::at(
                        &*ml_reference_array2784.borrow(),
                        ml_reference_position2784.to_index(),
                    );
                    ml_reference_projection2784
                };
                if ((!(self::ml_math::is_safe_integer(coordinate)) || (coordinate < (0f64))) || {
                    let source_ml_source2_same_value_operand40542 = (-0f64);
                    (((coordinate == source_ml_source2_same_value_operand40542)
                        && ((coordinate != (0f64))
                            || ((1f64 / coordinate)
                                == (1f64 / source_ml_source2_same_value_operand40542))))
                        || ((coordinate != coordinate)
                            && (source_ml_source2_same_value_operand40542
                                != source_ml_source2_same_value_operand40542)))
                }) {
                    self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Result::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 32, 110, 111, 110, 110, 101, 103, 97, 116, 105, 118, 101, 32, 115, 97, 102, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index346_2 =
                            source_ml_source1_temporary_index346.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index346 =
                                source_ml_source1_temporary_index346_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Result::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop349Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Result {
    MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Done,
    MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_slice_state_error_loop348(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values345: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index345: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Result {
    loop {
        return if (source_ml_source1_temporary_index345
            < ({
                let ml_reference_array2785 = source_ml_source1_temporary_values345.clone();
                let ml_reference_projection2785 =
                    self::ml::Big::from_u128(ml_reference_array2785.borrow().len() as u128);
                ml_reference_projection2785
            })) {
            {
                let value = {
                    let ml_reference_array2786 = source_ml_source1_temporary_values345.clone();
                    let ml_reference_position2786 = source_ml_source1_temporary_index345.clone();
                    let ml_reference_projection2786 = self::ml_array::at(
                        &*ml_reference_array2786.borrow(),
                        ml_reference_position2786.to_index(),
                    );
                    ml_reference_projection2786
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Result::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index345_2 =
                                source_ml_source1_temporary_index345
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index345 =
                                    source_ml_source1_temporary_index345_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Result::MlSource1TemporaryReadJavaScriptArraySliceStateErrorLoop348Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359 {
    MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Next,
    MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Result {
    MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Done,
    MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_slice_child_state_error_loop360(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values353: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index353: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Result {
    loop {
        return if (source_ml_source1_temporary_index353
            < ({
                let ml_reference_array2787 = source_ml_source1_temporary_values353.clone();
                let ml_reference_projection2787 =
                    self::ml::Big::from_u128(ml_reference_array2787.borrow().len() as u128);
                ml_reference_projection2787
            })) {
            {
                let value = {
                    let ml_reference_array2788 = source_ml_source1_temporary_values353.clone();
                    let ml_reference_position2788 = source_ml_source1_temporary_index353.clone();
                    let ml_reference_projection2788 = self::ml_array::at(
                        &*ml_reference_array2788.borrow(),
                        ml_reference_position2788.to_index(),
                    );
                    ml_reference_projection2788
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Result::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index353_2 =
                                source_ml_source1_temporary_index353
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index353 =
                                    source_ml_source1_temporary_index353_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Result::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358 {
    MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358Next,
    MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358Return(self::ml_string::Text),
}
