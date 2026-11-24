// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_construction_allocation_loop988(
    mut properties: f64,
    mut source_ml_source1_temporary_values986: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index986: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index986
            < ({
                let ml_reference_array2875 = source_ml_source1_temporary_values986.clone();
                let ml_reference_projection2875 =
                    self::ml::Big::from_u128(ml_reference_array2875.borrow().len() as u128);
                ml_reference_projection2875
            })) {
            {
                let object = {
                    let ml_reference_array2876 = source_ml_source1_temporary_values986.clone();
                    let ml_reference_position2876 = source_ml_source1_temporary_index986.clone();
                    let ml_reference_projection2876 = self::ml_array::at(
                        &*ml_reference_array2876.borrow(),
                        ml_reference_position2876.to_index(),
                    );
                    ml_reference_projection2876
                };
                {
                    let properties_2 = (properties + {
                        let ml_reference_array2877 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41269,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41269.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41270,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41270.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41271,
                            ) => source_ml_source2_data_property_value41271.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41272,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41272.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41273,
                                _,
                            ) => source_ml_source2_data_property_value41273.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41274,
                                _,
                            ) => source_ml_source2_data_property_value41274.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41275,
                                _,
                            ) => source_ml_source2_data_property_value41275.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41276,
                                _,
                            ) => source_ml_source2_data_property_value41276.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41277,
                            ) => source_ml_source2_data_property_value41277.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41278,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41278.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41279,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41279.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41280,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41280.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41281,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41281.clone(),
                        };
                        let ml_reference_projection2877 =
                            ml_reference_array2877.borrow().len() as f64;
                        ml_reference_projection2877
                    });
                    {
                        let source_ml_source1_temporary_index986_2 =
                            source_ml_source1_temporary_index986.add(&self::ml::Big::from_i128(1));
                        {
                            (properties, source_ml_source1_temporary_index986) =
                                (properties_2, source_ml_source1_temporary_index986_2.clone());
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
pub enum MlSource1TemporaryStepJavaScriptConstructionAllocationLoop991Result {
    MlSource1TemporaryStepJavaScriptConstructionAllocationLoop991Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_construction_allocation_loop991(
    spelling: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryStepJavaScriptConstructionAllocationLoop991Result {
    loop {
        return if (index < (spelling.length())) {
            {
                let source_ml_source1_temporary_step_java_script_construction_allocation_ignored992 = {
                    let ml_method_owner2878 = units.clone();
                    let ml_method_argument2878_0 = {
                        let source_ml_source2_string_unit_operand41282 = spelling.clone();
                        {
                            let source_ml_source2_string_unit_operand41283 = index;
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
                                    let source_ml_source2_string_unit_operand41289 = {
                                        let source_ml_source2_string_range_operand41284 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand41285 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand41286 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position41287 = source_ml_source2_string_range_operand41285;
                                                    {
                                                        let source_ml_source2_string_range_position41288 = source_ml_source2_string_range_operand41286;
                                                        source_ml_source2_string_range_operand41284.slice(source_ml_source2_string_range_position41287, source_ml_source2_string_range_position41288)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand41290 = 0f64;
                                        source_ml_source2_string_unit_operand41289.code_point_at(source_ml_source2_string_unit_operand41290)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand41282.clone(),
                                source_ml_source2_string_unit_operand41283,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument2878_0];
                    let mut ml_method_values2878 = ml_method_owner2878.borrow_mut();
                    let ml_length = ml_method_values2878
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2878.extend(ml_inserted);
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
            self::MlSource1TemporaryStepJavaScriptConstructionAllocationLoop991Result::MlSource1TemporaryStepJavaScriptConstructionAllocationLoop991Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryFinishJavaScriptPropertyKeyConversionLoop994Result {
    MlSource1TemporaryFinishJavaScriptPropertyKeyConversionLoop994Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_finish_java_script_property_key_conversion_loop994(
    spelling: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryFinishJavaScriptPropertyKeyConversionLoop994Result {
    loop {
        return if (index < (spelling.length())) {
            {
                let source_ml_source1_temporary_finish_java_script_property_key_conversion_ignored995 = {
                    let ml_method_owner2879 = units.clone();
                    let ml_method_argument2879_0 = {
                        let source_ml_source2_string_unit_operand41291 = spelling.clone();
                        {
                            let source_ml_source2_string_unit_operand41292 = index;
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
                                    let source_ml_source2_string_unit_operand41298 = {
                                        let source_ml_source2_string_range_operand41293 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand41294 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand41295 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position41296 = source_ml_source2_string_range_operand41294;
                                                    {
                                                        let source_ml_source2_string_range_position41297 = source_ml_source2_string_range_operand41295;
                                                        source_ml_source2_string_range_operand41293.slice(source_ml_source2_string_range_position41296, source_ml_source2_string_range_position41297)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand41299 = 0f64;
                                        source_ml_source2_string_unit_operand41298.code_point_at(source_ml_source2_string_unit_operand41299)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand41291.clone(),
                                source_ml_source2_string_unit_operand41292,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument2879_0];
                    let mut ml_method_values2879 = ml_method_owner2879.borrow_mut();
                    let ml_length = ml_method_values2879
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2879.extend(ml_inserted);
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
            self::MlSource1TemporaryFinishJavaScriptPropertyKeyConversionLoop994Result::MlSource1TemporaryFinishJavaScriptPropertyKeyConversionLoop994Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000 {
    MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000Next,
    MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin1000Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999 {
    MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999Next,
    MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin999Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998 {
    MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998Next,
    MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorJoin998Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Result {
    MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Done,
    MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_from_entries_state_error_loop997(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values996: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index996: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Result {
    loop {
        return if (source_ml_source1_temporary_index996
            < ({
                let ml_reference_array2880 = source_ml_source1_temporary_values996.clone();
                let ml_reference_projection2880 =
                    self::ml::Big::from_u128(ml_reference_array2880.borrow().len() as u128);
                ml_reference_projection2880
            })) {
            {
                let value = {
                    let ml_reference_array2881 = source_ml_source1_temporary_values996.clone();
                    let ml_reference_position2881 = source_ml_source1_temporary_index996.clone();
                    let ml_reference_projection2881 = self::ml_array::at(
                        &*ml_reference_array2881.borrow(),
                        ml_reference_position2881.to_index(),
                    );
                    ml_reference_projection2881
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Result::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index996_2 =
                                source_ml_source1_temporary_index996
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index996 =
                                    source_ml_source1_temporary_index996_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Result::MlSource1TemporaryReadJavaScriptObjectFromEntriesStateErrorLoop997Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008 {
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Next,
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Result {
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Done,
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_from_entries_entry_state_error_loop1009(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1001: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1001: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Result {
    loop {
        return if (source_ml_source1_temporary_index1001
            < ({
                let ml_reference_array2882 = source_ml_source1_temporary_values1001.clone();
                let ml_reference_projection2882 =
                    self::ml::Big::from_u128(ml_reference_array2882.borrow().len() as u128);
                ml_reference_projection2882
            })) {
            {
                let value = {
                    let ml_reference_array2883 = source_ml_source1_temporary_values1001.clone();
                    let ml_reference_position2883 = source_ml_source1_temporary_index1001.clone();
                    let ml_reference_projection2883 = self::ml_array::at(
                        &*ml_reference_array2883.borrow(),
                        ml_reference_position2883.to_index(),
                    );
                    ml_reference_projection2883
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Result::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1001_2 =
                                source_ml_source1_temporary_index1001
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1001 =
                                    source_ml_source1_temporary_index1001_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Result::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007 {
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007Next,
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006 {
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006Next,
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005 {
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005Next,
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004 {
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004Next,
    MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012 {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Next,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorJoin1012Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Result {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Done,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_property_definition_state_error_loop1013(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1011: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1011: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Result {
    loop {
        return if (source_ml_source1_temporary_index1011
            < ({
                let ml_reference_array2884 = source_ml_source1_temporary_values1011.clone();
                let ml_reference_projection2884 =
                    self::ml::Big::from_u128(ml_reference_array2884.borrow().len() as u128);
                ml_reference_projection2884
            })) {
            {
                let value = {
                    let ml_reference_array2885 = source_ml_source1_temporary_values1011.clone();
                    let ml_reference_position2885 = source_ml_source1_temporary_index1011.clone();
                    let ml_reference_projection2885 = self::ml_array::at(
                        &*ml_reference_array2885.borrow(),
                        ml_reference_position2885.to_index(),
                    );
                    ml_reference_projection2885
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1011_2 =
                                source_ml_source1_temporary_index1011
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1011 =
                                    source_ml_source1_temporary_index1011_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionStateErrorLoop1013Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Result {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Done,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_property_definition_continuation_error_loop1015(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1014: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1014: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Result {
    loop {
        return if (source_ml_source1_temporary_index1014
            < ({
                let ml_reference_array2886 = source_ml_source1_temporary_values1014.clone();
                let ml_reference_projection2886 =
                    self::ml::Big::from_u128(ml_reference_array2886.borrow().len() as u128);
                ml_reference_projection2886
            })) {
            {
                let value = {
                    let ml_reference_array2887 = source_ml_source1_temporary_values1014.clone();
                    let ml_reference_position2887 = source_ml_source1_temporary_index1014.clone();
                    let ml_reference_projection2887 = self::ml_array::at(
                        &*ml_reference_array2887.borrow(),
                        ml_reference_position2887.to_index(),
                    );
                    ml_reference_projection2887
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1014_2 =
                                source_ml_source1_temporary_index1014
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1014 =
                                    source_ml_source1_temporary_index1014_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionContinuationErrorLoop1015Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036 {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036Next,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1036Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Result {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Done,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1037(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1030: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1030: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Result {
    loop {
        return if (source_ml_source1_temporary_index1030
            < ({
                let ml_reference_array2888 = source_ml_source1_temporary_values1030.clone();
                let ml_reference_projection2888 =
                    self::ml::Big::from_u128(ml_reference_array2888.borrow().len() as u128);
                ml_reference_projection2888
            })) {
            {
                let value = {
                    let ml_reference_array2889 = source_ml_source1_temporary_values1030.clone();
                    let ml_reference_position2889 = source_ml_source1_temporary_index1030.clone();
                    let ml_reference_projection2889 = self::ml_array::at(
                        &*ml_reference_array2889.borrow(),
                        ml_reference_position2889.to_index(),
                    );
                    ml_reference_projection2889
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1030_2 =
                                source_ml_source1_temporary_index1030
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1030 =
                                    source_ml_source1_temporary_index1030_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1037Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035 {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Next,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1035Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034 {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034Next,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorJoin1034Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Result {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Done,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Result {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Done,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1038(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1029: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1029: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Result {
    loop {
        return if (source_ml_source1_temporary_index1029
            < ({
                let ml_reference_array2890 = source_ml_source1_temporary_values1029.clone();
                let ml_reference_projection2890 =
                    self::ml::Big::from_u128(ml_reference_array2890.borrow().len() as u128);
                ml_reference_projection2890
            })) {
            {
                let value = {
                    let ml_reference_array2891 = source_ml_source1_temporary_values1029.clone();
                    let ml_reference_position2891 = source_ml_source1_temporary_index1029.clone();
                    let ml_reference_projection2891 = self::ml_array::at(
                        &*ml_reference_array2891.borrow(),
                        ml_reference_position2891.to_index(),
                    );
                    ml_reference_projection2891
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1029_2 =
                                source_ml_source1_temporary_index1029
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1029 =
                                    source_ml_source1_temporary_index1029_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Done
        };
    }
}
