// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayIterationTransition;
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::read_java_script_string_iterator_entry_units;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_collection_iterator_result_loop1076(
    mut properties: f64,
    mut source_ml_source1_temporary_values1075: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index1075: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1075
            < ({
                let ml_reference_array2934 = source_ml_source1_temporary_values1075.clone();
                let ml_reference_projection2934 =
                    self::ml::Big::from_u128(ml_reference_array2934.borrow().len() as u128);
                ml_reference_projection2934
            })) {
            {
                let object = {
                    let ml_reference_array2935 = source_ml_source1_temporary_values1075.clone();
                    let ml_reference_position2935 = source_ml_source1_temporary_index1075.clone();
                    let ml_reference_projection2935 = self::ml_array::at(
                        &*ml_reference_array2935.borrow(),
                        ml_reference_position2935.to_index(),
                    );
                    ml_reference_projection2935
                };
                {
                    let properties_2 = (properties + {
                        let ml_reference_array2936 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41441,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41441.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41442,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41442.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41443,
                            ) => source_ml_source2_data_property_value41443.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41444,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41444.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41445,
                                _,
                            ) => source_ml_source2_data_property_value41445.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41446,
                                _,
                            ) => source_ml_source2_data_property_value41446.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41447,
                                _,
                            ) => source_ml_source2_data_property_value41447.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41448,
                                _,
                            ) => source_ml_source2_data_property_value41448.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41449,
                            ) => source_ml_source2_data_property_value41449.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41450,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41450.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41451,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41451.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41452,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41452.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value41453,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value41453.clone(),
                        };
                        let ml_reference_projection2936 =
                            ml_reference_array2936.borrow().len() as f64;
                        ml_reference_projection2936
                    });
                    {
                        let source_ml_source1_temporary_index1075_2 =
                            source_ml_source1_temporary_index1075.add(&self::ml::Big::from_i128(1));
                        {
                            (properties, source_ml_source1_temporary_index1075) = (
                                properties_2,
                                source_ml_source1_temporary_index1075_2.clone(),
                            );
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocate_java_script_string_iterator_loop1080(
    mut units: f64,
    mut source_ml_source1_temporary_values1079: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index1079: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1079
            < ({
                let ml_reference_array2937 = source_ml_source1_temporary_values1079.clone();
                let ml_reference_projection2937 =
                    self::ml::Big::from_u128(ml_reference_array2937.borrow().len() as u128);
                ml_reference_projection2937
            })) {
            {
                let row = {
                    let ml_reference_array2938 = source_ml_source1_temporary_values1079.clone();
                    let ml_reference_position2938 = source_ml_source1_temporary_index1079.clone();
                    let ml_reference_projection2938 = self::ml_array::at(
                        &*ml_reference_array2938.borrow(),
                        ml_reference_position2938.to_index(),
                    );
                    ml_reference_projection2938
                };
                {
                    let units_3 = if ((match row.clone() {
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => true,
                        _ => false,
                    } && (((((match row.clone() {
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value41468,
                            _,
                            _,
                        ) => Some((source_ml_source2_data_property_value41468.clone(),)),
                        _ => None::<(self::ml_string::Text,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0)
                        .clone())
                        == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))))
                        && !(match (
                            (((match row.clone() {
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value41469,
                                ) => Some((((source_ml_source2_data_property_value41469
                                    .clone()
                                    .unwrap_or_else(
                                        || (None::<self::JavaScriptSemanticValue>,),
                                    ))
                                .0)
                                    .clone(),)),
                                _ => None::<(Option<self::JavaScriptSemanticValue>,)>,
                            })
                            .clone()
                            .expect("a proven present value"))
                            .0)
                                .clone(),
                            None::<()>,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        })) {
                        {
                            let units_2 = (units + self::ml_math::max(0f64, self::read_java_script_semantic_text_length(((((match row.clone() {
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value41470) => {
                                    Some((((source_ml_source2_data_property_value41470.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone(),))
                                }
                                _ => {
                                    None::<(Option<self::JavaScriptSemanticValue>,)>
                                }
                            }).clone().expect("a proven present value")).0).clone()).clone().expect("a proven present value"))));
                            units_2
                        }
                    } else {
                        units
                    };
                    if (match row.clone() {
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => true,
                        _ => false,
                    } || match row.clone() {
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => true,
                        _ => false,
                    }) {
                        {
                            let units_4 = (units_3
                                + self::read_java_script_string_iterator_entry_units(
                                    (((match row.clone() {
                                        self::JavaScriptOrdinaryObject::MapObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value41483,
                                        ) => Some((
                                            source_ml_source2_data_property_value41483.clone(),
                                        )),
                                        self::JavaScriptOrdinaryObject::SetObject(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value41484,
                                        ) => Some((
                                            source_ml_source2_data_property_value41484.clone(),
                                        )),
                                        _ => {
                                            None::<(
                                                std::rc::Rc<
                                                    std::cell::RefCell<
                                                        Vec<self::JavaScriptCollectionEntry>,
                                                    >,
                                                >,
                                            )>
                                        }
                                    })
                                    .clone()
                                    .expect("a proven present value"))
                                    .0)
                                        .clone(),
                                ));
                            {
                                let source_ml_source1_temporary_index1079_2 =
                                    source_ml_source1_temporary_index1079
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (units, source_ml_source1_temporary_index1079) =
                                        (units_4, source_ml_source1_temporary_index1079_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source1_temporary_index1079_3 =
                                source_ml_source1_temporary_index1079
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (units, source_ml_source1_temporary_index1079) =
                                    (units_3, source_ml_source1_temporary_index1079_3.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            units
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_string_iterator_entry_units_loop1082(
    mut units: f64,
    mut source_ml_source1_temporary_values1081: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
    >,
    mut source_ml_source1_temporary_index1081: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1081
            < ({
                let ml_reference_array2939 = source_ml_source1_temporary_values1081.clone();
                let ml_reference_projection2939 =
                    self::ml::Big::from_u128(ml_reference_array2939.borrow().len() as u128);
                ml_reference_projection2939
            })) {
            {
                let entry = {
                    let ml_reference_array2940 = source_ml_source1_temporary_values1081.clone();
                    let ml_reference_position2940 = source_ml_source1_temporary_index1081.clone();
                    let ml_reference_projection2940 = self::ml_array::at(
                        &*ml_reference_array2940.borrow(),
                        ml_reference_position2940.to_index(),
                    );
                    ml_reference_projection2940
                };
                match entry.clone() {
                    self::JavaScriptCollectionEntry::CollectionEntry(
                        shared_constructor_field1,
                        shared_constructor_field2,
                        _,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        {
                            let units_2 = (units
                                + (self::ml_math::max(
                                    0f64,
                                    self::read_java_script_semantic_text_length(
                                        shared_constructor_field1.clone(),
                                    ),
                                ) + self::ml_math::max(
                                    0f64,
                                    self::read_java_script_semantic_text_length(
                                        shared_constructor_field2.clone(),
                                    ),
                                )));
                            {
                                let source_ml_source1_temporary_index1081_2 =
                                    source_ml_source1_temporary_index1081
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (units, source_ml_source1_temporary_index1081) =
                                        (units_2, source_ml_source1_temporary_index1081_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            units
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088 {
    MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088Next,
    MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Result {
    MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Done,
    MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_iterator_state_error_loop1089(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1085: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1085: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Result {
    loop {
        return if (source_ml_source1_temporary_index1085
            < ({
                let ml_reference_array2941 = source_ml_source1_temporary_values1085.clone();
                let ml_reference_projection2941 =
                    self::ml::Big::from_u128(ml_reference_array2941.borrow().len() as u128);
                ml_reference_projection2941
            })) {
            {
                let value = {
                    let ml_reference_array2942 = source_ml_source1_temporary_values1085.clone();
                    let ml_reference_position2942 = source_ml_source1_temporary_index1085.clone();
                    let ml_reference_projection2942 = self::ml_array::at(
                        &*ml_reference_array2942.borrow(),
                        ml_reference_position2942.to_index(),
                    );
                    ml_reference_projection2942
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Result::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1085_2 =
                                source_ml_source1_temporary_index1085
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1085 =
                                    source_ml_source1_temporary_index1085_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Result::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087 {
    MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087Next,
    MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Result {
    MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Done,
    MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_iterator_state_error_loop1086(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1084: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1084: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Result {
    loop {
        return if (source_ml_source1_temporary_index1084
            < ({
                let ml_reference_array2943 = source_ml_source1_temporary_values1084.clone();
                let ml_reference_projection2943 =
                    self::ml::Big::from_u128(ml_reference_array2943.borrow().len() as u128);
                ml_reference_projection2943
            })) {
            {
                let value = {
                    let ml_reference_array2944 = source_ml_source1_temporary_values1084.clone();
                    let ml_reference_position2944 = source_ml_source1_temporary_index1084.clone();
                    let ml_reference_projection2944 = self::ml_array::at(
                        &*ml_reference_array2944.borrow(),
                        ml_reference_position2944.to_index(),
                    );
                    ml_reference_projection2944
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Result::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1084_2 =
                                source_ml_source1_temporary_index1084
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1084 =
                                    source_ml_source1_temporary_index1084_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Result::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Result {
    MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Done,
    MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Return(
        Box<self::JavaScriptArrayIterationTransition>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_array_iterator_next_loop1100(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1099: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1099: self::ml::Big,
) -> self::MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Result {
    loop {
        return if (source_ml_source1_temporary_index1099
            < ({
                let ml_reference_array2945 = source_ml_source1_temporary_values1099.clone();
                let ml_reference_projection2945 =
                    self::ml::Big::from_u128(ml_reference_array2945.borrow().len() as u128);
                ml_reference_projection2945
            })) {
            {
                let value = {
                    let ml_reference_array2946 = source_ml_source1_temporary_values1099.clone();
                    let ml_reference_position2946 = source_ml_source1_temporary_index1099.clone();
                    let ml_reference_projection2946 = self::ml_array::at(
                        &*ml_reference_array2946.borrow(),
                        ml_reference_position2946.to_index(),
                    );
                    ml_reference_projection2946
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Result::MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Return(Box::new(self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), invalid.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index1099_2 =
                                source_ml_source1_temporary_index1099
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1099 =
                                    source_ml_source1_temporary_index1099_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Result::MlSource1TemporaryBeginJavaScriptArrayIteratorNextLoop1100Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108 {
    MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108Next,
    MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Result {
    MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Done,
    MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_species_construction_state_error_loop1109(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1105: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1105: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Result {
    loop {
        return if (source_ml_source1_temporary_index1105
            < ({
                let ml_reference_array2947 = source_ml_source1_temporary_values1105.clone();
                let ml_reference_projection2947 =
                    self::ml::Big::from_u128(ml_reference_array2947.borrow().len() as u128);
                ml_reference_projection2947
            })) {
            {
                let value = {
                    let ml_reference_array2948 = source_ml_source1_temporary_values1105.clone();
                    let ml_reference_position2948 = source_ml_source1_temporary_index1105.clone();
                    let ml_reference_projection2948 = self::ml_array::at(
                        &*ml_reference_array2948.borrow(),
                        ml_reference_position2948.to_index(),
                    );
                    ml_reference_projection2948
                };
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone()))
                    != (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Result::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Return(self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 97, 108, 108, 32, 118, 97, 108, 117, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index1105_2 =
                            source_ml_source1_temporary_index1105.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index1105 =
                                source_ml_source1_temporary_index1105_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Result::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107 {
    MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107Next,
    MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Result {
    MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Done,
    MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_species_construction_state_error_loop1106(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1103: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1103: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Result {
    loop {
        return if (source_ml_source1_temporary_index1103
            < ({
                let ml_reference_array2949 = source_ml_source1_temporary_values1103.clone();
                let ml_reference_projection2949 =
                    self::ml::Big::from_u128(ml_reference_array2949.borrow().len() as u128);
                ml_reference_projection2949
            })) {
            {
                let value = {
                    let ml_reference_array2950 = source_ml_source1_temporary_values1103.clone();
                    let ml_reference_position2950 = source_ml_source1_temporary_index1103.clone();
                    let ml_reference_projection2950 = self::ml_array::at(
                        &*ml_reference_array2950.borrow(),
                        ml_reference_position2950.to_index(),
                    );
                    ml_reference_projection2950
                };
                if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone()))
                    != (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Result::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Return(self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 118, 97, 108, 117, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 117, 114, 114, 101, 110, 116, 32, 115, 104, 97, 114, 101, 100, 32, 104, 101, 97, 112]))
                } else {
                    {
                        let source_ml_source1_temporary_index1103_2 =
                            source_ml_source1_temporary_index1103.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index1103 =
                                source_ml_source1_temporary_index1103_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Result::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Done
        };
    }
}
