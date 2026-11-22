// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptObjectIntegrityTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Result;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_heap_directory_value_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_synchronous_iterator_record_error;
use super::test_java_script_iterator_region_slot;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_instance_of_state_error_loop1190(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1186: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1186: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Result {
    loop {
        return if (source_ml_source1_temporary_index1186
            < ({
                let ml_reference_array3010 = source_ml_source1_temporary_values1186.clone();
                let ml_reference_projection3010 =
                    self::ml::Big::from_u128(ml_reference_array3010.borrow().len() as u128);
                ml_reference_projection3010
            })) {
            {
                let value = {
                    let ml_reference_array3011 = source_ml_source1_temporary_values1186.clone();
                    let ml_reference_position3011 = source_ml_source1_temporary_index1186.clone();
                    let ml_reference_projection3011 = self::ml_array::at(
                        &*ml_reference_array3011.borrow(),
                        ml_reference_position3011.to_index(),
                    );
                    ml_reference_projection3011
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1186_2 =
                                source_ml_source1_temporary_index1186
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1186 =
                                    source_ml_source1_temporary_index1186_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1190Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188 {
    MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188Next,
    MlSource1TemporaryReadJavaScriptInstanceOfStateErrorJoin1188Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Result {
    MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Done(
        Box<self::JavaScriptSemanticValue>,
    ),
    MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_instance_of_state_error_loop1191(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut target: self::JavaScriptSemanticValue,
    mut source_ml_source1_temporary_values1185: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut source_ml_source1_temporary_index1185: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (source_ml_source1_temporary_index1185
                    < ({
                        let ml_reference_array3012 = source_ml_source1_temporary_values1185.clone();
                        let ml_reference_projection3012 =
                            self::ml::Big::from_u128(ml_reference_array3012.borrow().len() as u128);
                        ml_reference_projection3012
                    }))
                {
                    {
                        let identity = {
                            let ml_reference_array3013 =
                                source_ml_source1_temporary_values1185.clone();
                            let ml_reference_position3013 =
                                source_ml_source1_temporary_index1185.clone();
                            let ml_reference_projection3013 = self::ml_array::at(
                                &*ml_reference_array3013.borrow(),
                                ml_reference_position3013.to_index(),
                            );
                            ml_reference_projection3013
                        };
                        match target.clone() {
                            self::JavaScriptSemanticValue::Object(target_identity, _) => {
                                {
                                    let bound = {
                                        let source_ml_source2_array_read_value41758 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_array_read_index41759 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), target_identity.clone());
                                            { let ml_reference_array3014 = source_ml_source2_array_read_value41758.clone(); let ml_reference_position3014 = source_ml_source2_array_read_index41759; let ml_reference_projection3014 = self::ml_array::optional_at(&*ml_reference_array3014.borrow(), self::ml_array::number_index(ml_reference_position3014)); ml_reference_projection3014 }
                                        }
                                    };
                                    if ((match (bound.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match bound.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }) || (((((match bound.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value41769, _, _, _, _, _, _, _) => {
                                            Some((source_ml_source2_data_property_value41769.clone(),))
                                        }
                                        _ => {
                                            None::<(self::ml::Big,)>
                                        }
                                    }).clone().expect("a proven present value")).0).clone()) != identity)) {
                                        self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 100, 101, 108, 101, 103, 97, 116, 105, 111, 110, 32, 102, 111, 108, 108, 111, 119, 115, 32, 111, 110, 108, 121, 32, 97, 99, 116, 117, 97, 108, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116, 32, 108, 105, 110, 107, 115]))
                                    } else {
                                        {
                                            let target_2 = (((match bound.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, source_ml_source2_data_property_value41770, _, _) => {
                                                    let source_ml_source2_data_property_value41770 = *source_ml_source2_data_property_value41770;
                                                    Some((source_ml_source2_data_property_value41770.clone(),))
                                                }
                                                _ => {
                                                    None::<(self::JavaScriptSemanticValue,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone();
                                            {
                                                let source_ml_source1_temporary_index1185_2 = source_ml_source1_temporary_index1185.add(&self::ml::Big::from_i128(1));
                                                {
                                                    (target, source_ml_source1_temporary_index1185) = (target_2.clone(), source_ml_source1_temporary_index1185_2.clone());
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Return(self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 100, 101, 108, 101, 103, 97, 116, 105, 111, 110, 32, 102, 111, 108, 108, 111, 119, 115, 32, 111, 110, 108, 121, 32, 97, 99, 116, 117, 97, 108, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116, 32, 108, 105, 110, 107, 115]))
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1191Done(Box::new(target.clone()))
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Result {
    MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Done,
    MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_instance_of_state_error_loop1187(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1184: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1184: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Result {
    loop {
        return if (source_ml_source1_temporary_index1184
            < ({
                let ml_reference_array3015 = source_ml_source1_temporary_values1184.clone();
                let ml_reference_projection3015 =
                    self::ml::Big::from_u128(ml_reference_array3015.borrow().len() as u128);
                ml_reference_projection3015
            })) {
            {
                let value = {
                    let ml_reference_array3016 = source_ml_source1_temporary_values1184.clone();
                    let ml_reference_position3016 = source_ml_source1_temporary_index1184.clone();
                    let ml_reference_projection3016 = self::ml_array::at(
                        &*ml_reference_array3016.borrow(),
                        ml_reference_position3016.to_index(),
                    );
                    ml_reference_projection3016
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1184_2 =
                                source_ml_source1_temporary_index1184
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1184 =
                                    source_ml_source1_temporary_index1184_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Result::MlSource1TemporaryReadJavaScriptInstanceOfStateErrorLoop1187Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryBeginJavaScriptObjectObservationLoop1196Result {
    MlSource1TemporaryBeginJavaScriptObjectObservationLoop1196Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_object_observation_loop1196(
    tag: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut index: f64,
) -> self::MlSource1TemporaryBeginJavaScriptObjectObservationLoop1196Result {
    loop {
        return if (index < (tag.length())) {
            {
                let source_ml_source1_temporary_begin_java_script_object_observation_ignored1197 = {
                    let ml_method_owner3017 = units.clone();
                    let ml_method_argument3017_0 = {
                        let source_ml_source2_string_unit_operand41773 = tag.clone();
                        {
                            let source_ml_source2_string_unit_operand41774 = index;
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
                                    let source_ml_source2_string_unit_operand41780 = {
                                        let source_ml_source2_string_range_operand41775 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand41776 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand41777 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position41778 = source_ml_source2_string_range_operand41776;
                                                    {
                                                        let source_ml_source2_string_range_position41779 = source_ml_source2_string_range_operand41777;
                                                        source_ml_source2_string_range_operand41775.slice(source_ml_source2_string_range_position41778, source_ml_source2_string_range_position41779)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand41781 = 0f64;
                                        source_ml_source2_string_unit_operand41780.code_point_at(source_ml_source2_string_unit_operand41781)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand41773.clone(),
                                source_ml_source2_string_unit_operand41774,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument3017_0];
                    let mut ml_method_values3017 = ml_method_owner3017.borrow_mut();
                    let ml_length = ml_method_values3017
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3017.extend(ml_inserted);
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
            self::MlSource1TemporaryBeginJavaScriptObjectObservationLoop1196Result::MlSource1TemporaryBeginJavaScriptObjectObservationLoop1196Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195 {
    MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Next(self::ml_string::Text),
    MlSource1TemporaryBeginJavaScriptObjectObservationJoin1195Return(
        Box<self::JavaScriptObjectIntegrityTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199 {
    MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199Next(bool),
    MlSource1TemporaryObserveJavaScriptObjectPropertiesJoin1199Return(
        Box<self::JavaScriptObjectIntegrityTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Result {
    MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Done,
    MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_integrity_state_error_loop1201(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1200: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1200: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Result {
    loop {
        return if (source_ml_source1_temporary_index1200
            < ({
                let ml_reference_array3018 = source_ml_source1_temporary_values1200.clone();
                let ml_reference_projection3018 =
                    self::ml::Big::from_u128(ml_reference_array3018.borrow().len() as u128);
                ml_reference_projection3018
            })) {
            {
                let key = {
                    let ml_reference_array3019 = source_ml_source1_temporary_values1200.clone();
                    let ml_reference_position3019 = source_ml_source1_temporary_index1200.clone();
                    let ml_reference_projection3019 = self::ml_array::at(
                        &*ml_reference_array3019.borrow(),
                        ml_reference_position3019.to_index(),
                    );
                    ml_reference_projection3019
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Result::MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1200_2 =
                                source_ml_source1_temporary_index1200
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1200 =
                                    source_ml_source1_temporary_index1200_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Result::MlSource1TemporaryReadJavaScriptObjectIntegrityStateErrorLoop1201Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Result {
    MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Done,
    MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_iterator_region_loop1206(
    mut regions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>,
    mut slot: f64,
    mut low: f64,
    mut high: f64,
) -> self::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Result {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let row = {
                        let ml_reference_array3020 = regions.clone();
                        let ml_reference_position3020 = middle;
                        let ml_reference_projection3020 = self::ml_array::optional_at(
                            &*ml_reference_array3020.borrow(),
                            self::ml_array::number_index(ml_reference_position3020),
                        );
                        ml_reference_projection3020
                    };
                    if match (row.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Result::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Return((-1f64))
                    } else {
                        if ((match row.clone().expect("a proven present value") {
                            self::JavaScriptCallIteratorRegion::CallIteratorRegion(
                                source_ml_source2_data_property_value41787,
                                _,
                            ) => source_ml_source2_data_property_value41787,
                        }) == slot)
                        {
                            self::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Result::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Return(middle)
                        } else {
                            if ((match row.clone().expect("a proven present value") {
                                self::JavaScriptCallIteratorRegion::CallIteratorRegion(
                                    source_ml_source2_data_property_value41789,
                                    _,
                                ) => source_ml_source2_data_property_value41789,
                            }) < slot)
                            {
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
            self::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Result::MlSource1TemporaryFindJavaScriptIteratorRegionLoop1206Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Result {
    MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Done,
    MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_iterator_regions_error_loop1208(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: f64,
    mut source_ml_source1_temporary_values1207: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>,
    >,
    mut source_ml_source1_temporary_index1207: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Result {
    loop {
        return if (source_ml_source1_temporary_index1207
            < ({
                let ml_reference_array3021 = source_ml_source1_temporary_values1207.clone();
                let ml_reference_projection3021 =
                    self::ml::Big::from_u128(ml_reference_array3021.borrow().len() as u128);
                ml_reference_projection3021
            })) {
            {
                let region = {
                    let ml_reference_array3022 = source_ml_source1_temporary_values1207.clone();
                    let ml_reference_position3022 = source_ml_source1_temporary_index1207.clone();
                    let ml_reference_projection3022 = self::ml_array::at(
                        &*ml_reference_array3022.borrow(),
                        ml_reference_position3022.to_index(),
                    );
                    ml_reference_projection3022
                };
                match region.clone() {
                    self::JavaScriptCallIteratorRegion::CallIteratorRegion(
                        shared_constructor_field1,
                        shared_constructor_field2,
                    ) => {
                        let shared_constructor_field2 = *shared_constructor_field2;
                        if ((match region.clone() {
                            self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, _) => false,
                        } || !(self::test_java_script_iterator_region_slot(
                            shared_constructor_field1,
                        ))) || (shared_constructor_field1 <= previous))
                        {
                            self::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Result::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 103, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 108, 111, 116, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                        } else {
                            {
                                let previous_2 = shared_constructor_field1;
                                {
                                    let invalid =
                                        self::read_java_script_synchronous_iterator_record_error(
                                            heap.clone(),
                                            shared_constructor_field2.clone(),
                                        );
                                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                        self::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Result::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Return(invalid.clone())
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index1207_2 =
                                                source_ml_source1_temporary_index1207
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (previous, source_ml_source1_temporary_index1207) = (
                                                    previous_2,
                                                    source_ml_source1_temporary_index1207_2.clone(),
                                                );
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
            self::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Result::MlSource1TemporaryReadJavaScriptIteratorRegionsErrorLoop1208Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_insert_java_script_iterator_region_loop1210(
    mut regions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>,
    mut slot: f64,
    mut at: f64,
) -> f64 {
    loop {
        return if ((at
            < ({
                let ml_reference_array3023 = regions.clone();
                let ml_reference_projection3023 = ml_reference_array3023.borrow().len() as f64;
                ml_reference_projection3023
            }))
            && ((({
                let source_ml_source2_optional_chain_receiver41805 = ({
                    let ml_reference_array3024 = regions.clone();
                    let ml_reference_position3024 = at;
                    let ml_reference_projection3024 = self::ml_array::optional_at(
                        &*ml_reference_array3024.borrow(),
                        self::ml_array::number_index(ml_reference_position3024),
                    );
                    ml_reference_projection3024
                },);
                if match (source_ml_source2_optional_chain_receiver41805.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    Some({
                        let source_ml_source1_temporary_optional_chain1209 =
                            ((source_ml_source2_optional_chain_receiver41805.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        match source_ml_source1_temporary_optional_chain1209.clone() {
                            self::JavaScriptCallIteratorRegion::CallIteratorRegion(
                                source_ml_source2_data_property_value41806,
                                _,
                            ) => source_ml_source2_data_property_value41806,
                        }
                    })
                } else {
                    None::<f64>
                }
            })
            .clone()
            .unwrap_or_else(|| slot))
                < slot))
        {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Result {
    MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Done,
    MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_argument_list_loop1212(
    mut lists: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>,
    mut slot: f64,
    mut at: f64,
) -> self::MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Result {
    loop {
        return if (at
            < ({
                let ml_reference_array3025 = lists.clone();
                let ml_reference_projection3025 = ml_reference_array3025.borrow().len() as f64;
                ml_reference_projection3025
            })) {
            if {
                let source_ml_source2_comparison_operand41811 = {
                    let source_ml_source2_optional_chain_receiver41809 = ({
                        let ml_reference_array3026 = lists.clone();
                        let ml_reference_position3026 = at;
                        let ml_reference_projection3026 = self::ml_array::optional_at(
                            &*ml_reference_array3026.borrow(),
                            self::ml_array::number_index(ml_reference_position3026),
                        );
                        ml_reference_projection3026
                    },);
                    if match (source_ml_source2_optional_chain_receiver41809.clone()).0 {
                        Some(ml_present) => {
                            let _ = ml_present.clone();
                            true
                        }
                        None => false,
                    } {
                        Some({
                            let source_ml_source1_temporary_optional_chain1211 =
                                ((source_ml_source2_optional_chain_receiver41809.clone()).0)
                                    .clone()
                                    .expect("a proven present value");
                            match source_ml_source1_temporary_optional_chain1211.clone() {
                                self::JavaScriptCallArgumentList::CallArgumentList(
                                    source_ml_source2_data_property_value41810,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value41810,
                            }
                        })
                    } else {
                        None::<f64>
                    }
                };
                match (source_ml_source2_comparison_operand41811.clone(), slot) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            } {
                self::MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Result::MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Return(at)
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
            self::MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Result::MlSource1TemporaryFindJavaScriptCallArgumentListLoop1212Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_count_java_script_call_argument_list_values_loop1214(
    mut count: f64,
    mut source_ml_source1_temporary_values1213: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>,
    >,
    mut source_ml_source1_temporary_index1213: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1213
            < ({
                let ml_reference_array3027 = source_ml_source1_temporary_values1213.clone();
                let ml_reference_projection3027 =
                    self::ml::Big::from_u128(ml_reference_array3027.borrow().len() as u128);
                ml_reference_projection3027
            })) {
            {
                let list = {
                    let ml_reference_array3028 = source_ml_source1_temporary_values1213.clone();
                    let ml_reference_position3028 = source_ml_source1_temporary_index1213.clone();
                    let ml_reference_projection3028 = self::ml_array::at(
                        &*ml_reference_array3028.borrow(),
                        ml_reference_position3028.to_index(),
                    );
                    ml_reference_projection3028
                };
                {
                    let count_2 = (count + {
                        let ml_reference_array3029 = match list.clone() {
                            self::JavaScriptCallArgumentList::CallArgumentList(
                                _,
                                _,
                                source_ml_source2_data_property_value41815,
                            ) => source_ml_source2_data_property_value41815.clone(),
                        };
                        let ml_reference_projection3029 =
                            ml_reference_array3029.borrow().len() as f64;
                        ml_reference_projection3029
                    });
                    {
                        let source_ml_source1_temporary_index1213_2 =
                            source_ml_source1_temporary_index1213.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index1213) =
                                (count_2, source_ml_source1_temporary_index1213_2.clone());
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
pub enum MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Result {
    MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Done,
    MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1217Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Result {
    MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Done,
    MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_argument_lists_error_loop1218(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1216: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1216: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Result {
    loop {
        return if (source_ml_source1_temporary_index1216
            < ({
                let ml_reference_array3030 = source_ml_source1_temporary_values1216.clone();
                let ml_reference_projection3030 =
                    self::ml::Big::from_u128(ml_reference_array3030.borrow().len() as u128);
                ml_reference_projection3030
            })) {
            {
                let value = {
                    let ml_reference_array3031 = source_ml_source1_temporary_values1216.clone();
                    let ml_reference_position3031 = source_ml_source1_temporary_index1216.clone();
                    let ml_reference_projection3031 = self::ml_array::at(
                        &*ml_reference_array3031.borrow(),
                        ml_reference_position3031.to_index(),
                    );
                    ml_reference_projection3031
                };
                {
                    let error = self::read_java_script_heap_directory_value_error(
                        heap.clone(),
                        value.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Result::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1216_2 =
                                source_ml_source1_temporary_index1216
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1216 =
                                    source_ml_source1_temporary_index1216_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Result::MlSource1TemporaryReadJavaScriptCallArgumentListsErrorLoop1218Done
        };
    }
}
