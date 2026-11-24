// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIndexedPropertyCreation;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_transform_state_error_loop238(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values231: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index231: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Result {
    loop {
        return if (source_ml_source1_temporary_index231
            < ({
                let ml_reference_array2721 = source_ml_source1_temporary_values231.clone();
                let ml_reference_projection2721 =
                    self::ml::Big::from_u128(ml_reference_array2721.borrow().len() as u128);
                ml_reference_projection2721
            })) {
            {
                let value = {
                    let ml_reference_array2722 = source_ml_source1_temporary_values231.clone();
                    let ml_reference_position2722 = source_ml_source1_temporary_index231.clone();
                    let ml_reference_projection2722 = self::ml_array::at(
                        &*ml_reference_array2722.borrow(),
                        ml_reference_position2722.to_index(),
                    );
                    ml_reference_projection2722
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index231_2 =
                                source_ml_source1_temporary_index231
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index231 =
                                    source_ml_source1_temporary_index231_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop238Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236 {
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236Next,
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin236Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235 {
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235Next,
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin235Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234 {
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234Next,
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorJoin234Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Result {
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Done,
    MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_transform_state_error_loop233(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values229: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index229: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Result {
    loop {
        return if (source_ml_source1_temporary_index229
            < ({
                let ml_reference_array2723 = source_ml_source1_temporary_values229.clone();
                let ml_reference_projection2723 =
                    self::ml::Big::from_u128(ml_reference_array2723.borrow().len() as u128);
                ml_reference_projection2723
            })) {
            {
                let value = {
                    let ml_reference_array2724 = source_ml_source1_temporary_values229.clone();
                    let ml_reference_position2724 = source_ml_source1_temporary_index229.clone();
                    let ml_reference_projection2724 = self::ml_array::at(
                        &*ml_reference_array2724.borrow(),
                        ml_reference_position2724.to_index(),
                    );
                    ml_reference_projection2724
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index229_2 =
                                source_ml_source1_temporary_index229
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index229 =
                                    source_ml_source1_temporary_index229_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Result::MlSource1TemporaryReadJavaScriptArrayTransformStateErrorLoop233Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247 {
    MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247Next,
    MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin247Return(
        Box<self::JavaScriptIndexedPropertyCreation>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_indexed_data_property_loop248(
    mut total: f64,
    mut source_ml_source1_temporary_values244: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index244: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index244
            < ({
                let ml_reference_array2725 = source_ml_source1_temporary_values244.clone();
                let ml_reference_projection2725 =
                    self::ml::Big::from_u128(ml_reference_array2725.borrow().len() as u128);
                ml_reference_projection2725
            })) {
            {
                let row = {
                    let ml_reference_array2726 = source_ml_source1_temporary_values244.clone();
                    let ml_reference_position2726 = source_ml_source1_temporary_index244.clone();
                    let ml_reference_projection2726 = self::ml_array::at(
                        &*ml_reference_array2726.borrow(),
                        ml_reference_position2726.to_index(),
                    );
                    ml_reference_projection2726
                };
                {
                    let total_2 = (total + {
                        let ml_reference_array2727 = match row.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40288,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40288.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40289,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40289.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40290,
                            ) => source_ml_source2_data_property_value40290.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40291,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40291.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40292,
                                _,
                            ) => source_ml_source2_data_property_value40292.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40293,
                                _,
                            ) => source_ml_source2_data_property_value40293.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40294,
                                _,
                            ) => source_ml_source2_data_property_value40294.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40295,
                                _,
                            ) => source_ml_source2_data_property_value40295.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40296,
                            ) => source_ml_source2_data_property_value40296.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40297,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40297.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40298,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40298.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40299,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40299.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40300,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40300.clone(),
                        };
                        let ml_reference_projection2727 =
                            ml_reference_array2727.borrow().len() as f64;
                        ml_reference_projection2727
                    });
                    {
                        let source_ml_source1_temporary_index244_2 =
                            source_ml_source1_temporary_index244.add(&self::ml::Big::from_i128(1));
                        {
                            (total, source_ml_source1_temporary_index244) =
                                (total_2, source_ml_source1_temporary_index244_2.clone());
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246 {
    MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Next,
    MlSource1TemporaryCreateJavaScriptIndexedDataPropertyJoin246Return(
        Box<self::JavaScriptIndexedPropertyCreation>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCreateJavaScriptIndexedDataPropertyLoop245Result {
    MlSource1TemporaryCreateJavaScriptIndexedDataPropertyLoop245Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_indexed_data_property_loop245(
    name: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyLoop245Result {
    loop {
        return if (at < (name.length())) {
            {
                let source_ml_source1_temporary_create_java_script_indexed_data_property_ignored249 = {
                    let ml_method_owner2728 = units.clone();
                    let ml_method_argument2728_0 = {
                        let source_ml_source2_string_unit_operand40301 = name.clone();
                        {
                            let source_ml_source2_string_unit_operand40302 = at;
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
                                    let source_ml_source2_string_unit_operand40308 = {
                                        let source_ml_source2_string_range_operand40303 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand40304 = index;
                                            {
                                                let source_ml_source2_string_range_operand40305 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position40306 = source_ml_source2_string_range_operand40304;
                                                    {
                                                        let source_ml_source2_string_range_position40307 = source_ml_source2_string_range_operand40305;
                                                        source_ml_source2_string_range_operand40303.slice(source_ml_source2_string_range_position40306, source_ml_source2_string_range_position40307)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand40309 = 0f64;
                                        source_ml_source2_string_unit_operand40308.code_point_at(source_ml_source2_string_unit_operand40309)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand40301.clone(),
                                source_ml_source2_string_unit_operand40302,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument2728_0];
                    let mut ml_method_values2728 = ml_method_owner2728.borrow_mut();
                    let ml_length = ml_method_values2728
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2728.extend(ml_inserted);
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
            self::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyLoop245Result::MlSource1TemporaryCreateJavaScriptIndexedDataPropertyLoop245Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptArrayTransformLoop250Result {
    MlSource1TemporaryStepJavaScriptArrayTransformLoop250Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_array_transform_loop250(
    name: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryStepJavaScriptArrayTransformLoop250Result {
    loop {
        return if (at < (name.length())) {
            {
                let source_ml_source1_temporary_step_java_script_array_transform_ignored251 = {
                    let ml_method_owner2729 = units.clone();
                    let ml_method_argument2729_0 = {
                        let source_ml_source2_string_unit_operand40310 = name.clone();
                        {
                            let source_ml_source2_string_unit_operand40311 = at;
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
                                    let source_ml_source2_string_unit_operand40317 = {
                                        let source_ml_source2_string_range_operand40312 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand40313 = index;
                                            {
                                                let source_ml_source2_string_range_operand40314 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position40315 = source_ml_source2_string_range_operand40313;
                                                    {
                                                        let source_ml_source2_string_range_position40316 = source_ml_source2_string_range_operand40314;
                                                        source_ml_source2_string_range_operand40312.slice(source_ml_source2_string_range_position40315, source_ml_source2_string_range_position40316)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand40318 = 0f64;
                                        source_ml_source2_string_unit_operand40317.code_point_at(source_ml_source2_string_unit_operand40318)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand40310.clone(),
                                source_ml_source2_string_unit_operand40311,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument2729_0];
                    let mut ml_method_values2729 = ml_method_owner2729.borrow_mut();
                    let ml_length = ml_method_values2729
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2729.extend(ml_inserted);
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
            self::MlSource1TemporaryStepJavaScriptArrayTransformLoop250Result::MlSource1TemporaryStepJavaScriptArrayTransformLoop250Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_request_java_script_array_transform_allocation_loop254(
    mut properties: f64,
    mut source_ml_source1_temporary_values253: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index253: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index253
            < ({
                let ml_reference_array2730 = source_ml_source1_temporary_values253.clone();
                let ml_reference_projection2730 =
                    self::ml::Big::from_u128(ml_reference_array2730.borrow().len() as u128);
                ml_reference_projection2730
            })) {
            {
                let object = {
                    let ml_reference_array2731 = source_ml_source1_temporary_values253.clone();
                    let ml_reference_position2731 = source_ml_source1_temporary_index253.clone();
                    let ml_reference_projection2731 = self::ml_array::at(
                        &*ml_reference_array2731.borrow(),
                        ml_reference_position2731.to_index(),
                    );
                    ml_reference_projection2731
                };
                {
                    let properties_2 = (properties + {
                        let ml_reference_array2732 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40321,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40321.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40322,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40322.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40323,
                            ) => source_ml_source2_data_property_value40323.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40324,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40324.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40325,
                                _,
                            ) => source_ml_source2_data_property_value40325.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40326,
                                _,
                            ) => source_ml_source2_data_property_value40326.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40327,
                                _,
                            ) => source_ml_source2_data_property_value40327.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40328,
                                _,
                            ) => source_ml_source2_data_property_value40328.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40329,
                            ) => source_ml_source2_data_property_value40329.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40330,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40330.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40331,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40331.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40332,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40332.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value40333,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value40333.clone(),
                        };
                        let ml_reference_projection2732 =
                            ml_reference_array2732.borrow().len() as f64;
                        ml_reference_projection2732
                    });
                    {
                        let source_ml_source1_temporary_index253_2 =
                            source_ml_source1_temporary_index253.add(&self::ml::Big::from_i128(1));
                        {
                            (properties, source_ml_source1_temporary_index253) =
                                (properties_2, source_ml_source1_temporary_index253_2.clone());
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
pub enum MlSource1TemporaryConstructJavaScriptReflectionTextLoop255Result {
    MlSource1TemporaryConstructJavaScriptReflectionTextLoop255Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_reflection_text_loop255(
    value: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryConstructJavaScriptReflectionTextLoop255Result {
    loop {
        return if (at < (value.length())) {
            {
                let source_ml_source1_temporary_construct_java_script_reflection_text_ignored256 = {
                    let ml_method_owner2733 = units.clone();
                    let ml_method_argument2733_0 = {
                        let source_ml_source2_string_unit_operand40334 = value.clone();
                        {
                            let source_ml_source2_string_unit_operand40335 = at;
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
                                    let source_ml_source2_string_unit_operand40341 = {
                                        let source_ml_source2_string_range_operand40336 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand40337 = index;
                                            {
                                                let source_ml_source2_string_range_operand40338 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position40339 = source_ml_source2_string_range_operand40337;
                                                    {
                                                        let source_ml_source2_string_range_position40340 = source_ml_source2_string_range_operand40338;
                                                        source_ml_source2_string_range_operand40336.slice(source_ml_source2_string_range_position40339, source_ml_source2_string_range_position40340)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand40342 = 0f64;
                                        source_ml_source2_string_unit_operand40341.code_point_at(source_ml_source2_string_unit_operand40342)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand40334.clone(),
                                source_ml_source2_string_unit_operand40335,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument2733_0];
                    let mut ml_method_values2733 = ml_method_owner2733.borrow_mut();
                    let ml_length = ml_method_values2733
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2733.extend(ml_inserted);
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
            self::MlSource1TemporaryConstructJavaScriptReflectionTextLoop255Result::MlSource1TemporaryConstructJavaScriptReflectionTextLoop255Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptReflectedDescriptorFieldsJoin259 {
    MlSource1TemporaryConstructJavaScriptReflectedDescriptorFieldsJoin259Next,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264 {
    MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264Next(
        Box<self::JavaScriptSemanticValue>,
        bool,
    ),
    MlSource1TemporaryReadJavaScriptReflectedOwnPropertyJoin264Return(
        Option<self::JavaScriptOrdinaryProperty>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272 {
    MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272Next,
    MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Result {
    MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Done,
    MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_property_reflection_state_error_loop271(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut seen: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_source1_temporary_values268: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index268: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Result {
    loop {
        return if (source_ml_source1_temporary_index268
            < ({
                let ml_reference_array2734 = source_ml_source1_temporary_values268.clone();
                let ml_reference_projection2734 =
                    self::ml::Big::from_u128(ml_reference_array2734.borrow().len() as u128);
                ml_reference_projection2734
            })) {
            {
                let key = {
                    let ml_reference_array2735 = source_ml_source1_temporary_values268.clone();
                    let ml_reference_position2735 = source_ml_source1_temporary_index268.clone();
                    let ml_reference_projection2735 = self::ml_array::at(
                        &*ml_reference_array2735.borrow(),
                        ml_reference_position2735.to_index(),
                    );
                    ml_reference_projection2735
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Result::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Return(invalid.clone())
                    } else {
                        if {
                            let ml_values2736 = {
                                let ml_reference_value2737 = seen.borrow().clone();
                                ml_reference_value2737
                            };
                            let ml_callback2736 = {
                                let key = key.clone();
                                std::rc::Rc::new(
                                    move |prior: self::JavaScriptSemanticValue| -> bool {
                                        self::test_java_script_semantic_same_value(
                                            prior.clone(),
                                            key.clone(),
                                        )
                                    },
                                )
                            };
                            ml_values2736
                                .iter()
                                .cloned()
                                .enumerate()
                                .any(|(ml_index2736, ml_value2736)| ml_callback2736(ml_value2736))
                        } {
                            self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Result::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Return(self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 32, 107, 101, 121, 115]))
                        } else {
                            {
                                let source_ml_source1_temporary_read_java_script_property_reflection_state_error_ignored273 = {
                                    let ml_method_owner2738 = seen.clone();
                                    let ml_method_argument2738_0 = key.clone();
                                    let ml_inserted = vec![ml_method_argument2738_0];
                                    let mut ml_method_values2738 = ml_method_owner2738.borrow_mut();
                                    let ml_length = ml_method_values2738
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4294967295usize)
                                        .expect("Invalid array length");
                                    ml_method_values2738.extend(ml_inserted);
                                    ml_length as f64
                                };
                                {
                                    let source_ml_source1_temporary_index268_2 =
                                        source_ml_source1_temporary_index268
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index268 =
                                            source_ml_source1_temporary_index268_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Result::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270 {
    MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270Next,
    MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Result {
    MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Done,
    MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Return(
        self::ml_string::Text,
    ),
}
