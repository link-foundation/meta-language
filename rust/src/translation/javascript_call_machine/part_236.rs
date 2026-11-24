// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptCallInstruction;
use super::JavaScriptCollectionCompletion;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_creation_state_error_loop400(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values399: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index399: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Result {
    loop {
        return if (source_ml_source1_temporary_index399
            < ({
                let ml_reference_array2808 = source_ml_source1_temporary_values399.clone();
                let ml_reference_projection2808 =
                    self::ml::Big::from_u128(ml_reference_array2808.borrow().len() as u128);
                ml_reference_projection2808
            })) {
            {
                let value = {
                    let ml_reference_array2809 = source_ml_source1_temporary_values399.clone();
                    let ml_reference_position2809 = source_ml_source1_temporary_index399.clone();
                    let ml_reference_projection2809 = self::ml_array::at(
                        &*ml_reference_array2809.borrow(),
                        ml_reference_position2809.to_index(),
                    );
                    ml_reference_projection2809
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Result::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index399_2 =
                                source_ml_source1_temporary_index399
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index399 =
                                    source_ml_source1_temporary_index399_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Result::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408 {
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408Next,
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407 {
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407Next,
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406 {
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406Next,
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405 {
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405Next,
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404 {
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404Next,
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403 {
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403Next,
    MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413 {
    MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Next,
    MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorJoin413Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Result {
    MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Done,
    MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_creation_callback_error_loop414(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values409: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index409: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Result {
    loop {
        return if (source_ml_source1_temporary_index409
            < ({
                let ml_reference_array2810 = source_ml_source1_temporary_values409.clone();
                let ml_reference_projection2810 =
                    self::ml::Big::from_u128(ml_reference_array2810.borrow().len() as u128);
                ml_reference_projection2810
            })) {
            {
                let value = {
                    let ml_reference_array2811 = source_ml_source1_temporary_values409.clone();
                    let ml_reference_position2811 = source_ml_source1_temporary_index409.clone();
                    let ml_reference_projection2811 = self::ml_array::at(
                        &*ml_reference_array2811.borrow(),
                        ml_reference_position2811.to_index(),
                    );
                    ml_reference_projection2811
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Result::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index409_2 =
                                source_ml_source1_temporary_index409
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index409 =
                                    source_ml_source1_temporary_index409_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Result::MlSource1TemporaryReadJavaScriptArrayCreationCallbackErrorLoop414Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Result {
    MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Done,
    MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Return(
        Box<self::JavaScriptArrayCreationTransition>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_begin_java_script_array_creation_loop416(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values415: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index415: self::ml::Big,
) -> self::MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Result {
    loop {
        return if (source_ml_source1_temporary_index415
            < ({
                let ml_reference_array2812 = source_ml_source1_temporary_values415.clone();
                let ml_reference_projection2812 =
                    self::ml::Big::from_u128(ml_reference_array2812.borrow().len() as u128);
                ml_reference_projection2812
            })) {
            {
                let value = {
                    let ml_reference_array2813 = source_ml_source1_temporary_values415.clone();
                    let ml_reference_position2813 = source_ml_source1_temporary_index415.clone();
                    let ml_reference_projection2813 = self::ml_array::at(
                        &*ml_reference_array2813.borrow(),
                        ml_reference_position2813.to_index(),
                    );
                    ml_reference_projection2813
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Result::MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Return(Box::new(self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap.clone()), error.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index415_2 =
                                source_ml_source1_temporary_index415
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index415 =
                                    source_ml_source1_temporary_index415_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Result::MlSource1TemporaryBeginJavaScriptArrayCreationLoop416Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop419Result {
    MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop419Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_array_creation_index_loop419(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut reversed: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop419Result {
    loop {
        return if (at >= (0f64)) {
            {
                let source_ml_source1_temporary_construct_java_script_array_creation_index_ignored420 = {
                    let ml_method_owner2814 = units.clone();
                    let ml_method_argument2814_0 = ({
                        let ml_reference_array2815 = reversed.clone();
                        let ml_reference_position2815 = at;
                        let ml_reference_projection2815 = self::ml_array::optional_at(
                            &*ml_reference_array2815.borrow(),
                            self::ml_array::number_index(ml_reference_position2815),
                        );
                        ml_reference_projection2815
                    })
                    .clone()
                    .unwrap_or_else(|| 48f64);
                    let ml_inserted = vec![ml_method_argument2814_0];
                    let mut ml_method_values2814 = ml_method_owner2814.borrow_mut();
                    let ml_length = ml_method_values2814
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2814.extend(ml_inserted);
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
            self::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop419Result::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop419Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop418Result {
    MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop418Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_array_creation_index_loop418(
    mut reversed: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut remaining: f64,
) -> self::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop418Result {
    loop {
        return {
            let digit = (remaining % 10f64);
            {
                let source_ml_source1_temporary_construct_java_script_array_creation_index_ignored421 = {
                    let ml_method_owner2816 = reversed.clone();
                    let ml_method_argument2816_0 = (48f64 + digit);
                    let ml_inserted = vec![ml_method_argument2816_0];
                    let mut ml_method_values2816 = ml_method_owner2816.borrow_mut();
                    let ml_length = ml_method_values2816
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2816.extend(ml_inserted);
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
                        self::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop418Result::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop418Done
                    }
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocate_java_script_collection_loop423(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut index: f64,
) -> f64 {
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
                if ((index
                    < ({
                        let ml_reference_array2817 = shared_constructor_field1.clone();
                        let ml_reference_projection2817 =
                            ml_reference_array2817.borrow().len() as f64;
                        ml_reference_projection2817
                    }))
                    && ((({
                        let source_ml_source2_optional_chain_receiver40626 = ({
                            let source_ml_source2_array_read_value40624 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array2818 =
                                    source_ml_source2_array_read_value40624.clone();
                                let ml_reference_position2818 = index;
                                let ml_reference_projection2818 = self::ml_array::optional_at(
                                    &*ml_reference_array2818.borrow(),
                                    self::ml_array::number_index(ml_reference_position2818),
                                );
                                ml_reference_projection2818
                            }
                        },);
                        if match (source_ml_source2_optional_chain_receiver40626.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            Some({
                                let source_ml_source1_temporary_optional_chain422 =
                                    ((source_ml_source2_optional_chain_receiver40626.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_temporary_optional_chain422.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        source_ml_source2_data_property_value40627,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40627.clone(),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        source_ml_source2_data_property_value40628,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40628.clone(),
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        source_ml_source2_data_property_value40629,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40629.clone(),
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        source_ml_source2_data_property_value40630,
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
                                    ) => source_ml_source2_data_property_value40630.clone(),
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        source_ml_source2_data_property_value40631,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40631.clone(),
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        source_ml_source2_data_property_value40632,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40632.clone(),
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        source_ml_source2_data_property_value40633,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40633.clone(),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        source_ml_source2_data_property_value40634,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40634.clone(),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        source_ml_source2_data_property_value40635,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40635.clone(),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        source_ml_source2_data_property_value40636,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40636.clone(),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        source_ml_source2_data_property_value40637,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40637.clone(),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        source_ml_source2_data_property_value40638,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40638.clone(),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        source_ml_source2_data_property_value40639,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40639.clone(),
                                }
                            })
                        } else {
                            None::<self::ml::Big>
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| identity.clone()))
                        < identity))
                {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                } else {
                    index
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Result {
    MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Done,
    MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Return(
        Box<self::JavaScriptCollectionCompletion>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_collection_operation_loop425(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values424: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index424: self::ml::Big,
) -> self::MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Result {
    loop {
        return if (source_ml_source1_temporary_index424
            < ({
                let ml_reference_array2819 = source_ml_source1_temporary_values424.clone();
                let ml_reference_projection2819 =
                    self::ml::Big::from_u128(ml_reference_array2819.borrow().len() as u128);
                ml_reference_projection2819
            })) {
            {
                let value = {
                    let ml_reference_array2820 = source_ml_source1_temporary_values424.clone();
                    let ml_reference_position2820 = source_ml_source1_temporary_index424.clone();
                    let ml_reference_projection2820 = self::ml_array::at(
                        &*ml_reference_array2820.borrow(),
                        ml_reference_position2820.to_index(),
                    );
                    ml_reference_projection2820
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Result::MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Return(Box::new(self::JavaScriptCollectionCompletion::CollectionInvalid(Box::new(heap.clone()), invalid.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index424_2 =
                                source_ml_source1_temporary_index424
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index424 =
                                    source_ml_source1_temporary_index424_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Result::MlSource1TemporaryExecuteJavaScriptCollectionOperationLoop425Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCopyJavaScriptCollectionLoop427Result {
    MlSource1TemporaryCopyJavaScriptCollectionLoop427Done,
    MlSource1TemporaryCopyJavaScriptCollectionLoop427Return(
        Box<self::JavaScriptCollectionCompletion>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_copy_java_script_collection_loop427(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values426: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index426: self::ml::Big,
) -> self::MlSource1TemporaryCopyJavaScriptCollectionLoop427Result {
    loop {
        return if (source_ml_source1_temporary_index426
            < ({
                let ml_reference_array2821 = source_ml_source1_temporary_values426.clone();
                let ml_reference_projection2821 =
                    self::ml::Big::from_u128(ml_reference_array2821.borrow().len() as u128);
                ml_reference_projection2821
            })) {
            {
                let value = {
                    let ml_reference_array2822 = source_ml_source1_temporary_values426.clone();
                    let ml_reference_position2822 = source_ml_source1_temporary_index426.clone();
                    let ml_reference_projection2822 = self::ml_array::at(
                        &*ml_reference_array2822.borrow(),
                        ml_reference_position2822.to_index(),
                    );
                    ml_reference_projection2822
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryCopyJavaScriptCollectionLoop427Result::MlSource1TemporaryCopyJavaScriptCollectionLoop427Return(Box::new(self::JavaScriptCollectionCompletion::CollectionInvalid(Box::new(heap.clone()), invalid.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index426_2 =
                                source_ml_source1_temporary_index426
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index426 =
                                    source_ml_source1_temporary_index426_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCopyJavaScriptCollectionLoop427Result::MlSource1TemporaryCopyJavaScriptCollectionLoop427Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDependencyFunction3Join716 {
    MlSource1TemporaryDependencyFunction3Join716Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryDependencyFunction3Loop718Result {
    MlSource1TemporaryDependencyFunction3Loop718Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_dependency_function3_loop718(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_source1_temporary_values715: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_source1_temporary_index715: self::ml::Big,
) -> self::MlSource1TemporaryDependencyFunction3Loop718Result {
    loop {
        return if (source_ml_source1_temporary_index715
            < ({
                let ml_reference_array2823 = source_ml_source1_temporary_values715.clone();
                let ml_reference_projection2823 =
                    self::ml::Big::from_u128(ml_reference_array2823.borrow().len() as u128);
                ml_reference_projection2823
            })) {
            {
                let unit = {
                    let ml_reference_array2824 = source_ml_source1_temporary_values715.clone();
                    let ml_reference_position2824 = source_ml_source1_temporary_index715.clone();
                    let ml_reference_projection2824 = self::ml_array::at(
                        &*ml_reference_array2824.borrow(),
                        ml_reference_position2824.to_index(),
                    );
                    ml_reference_projection2824
                };
                {
                    let source_ml_source1_temporary_dependency_function3_ignored719 = {
                        let ml_method_owner2825 = units.clone();
                        let ml_method_argument2825_0 = unit;
                        let ml_inserted = vec![ml_method_argument2825_0];
                        let mut ml_method_values2825 = ml_method_owner2825.borrow_mut();
                        let ml_length = ml_method_values2825
                            .len()
                            .checked_add(ml_inserted.len())
                            .filter(|length| *length <= 4294967295usize)
                            .expect("Invalid array length");
                        ml_method_values2825.extend(ml_inserted);
                        ml_length as f64
                    };
                    {
                        let source_ml_source1_temporary_index715_2 =
                            source_ml_source1_temporary_index715.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index715 =
                                source_ml_source1_temporary_index715_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryDependencyFunction3Loop718Result::MlSource1TemporaryDependencyFunction3Loop718Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Result {
    MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Done(self::ml_string::Text),
    MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_collection_constructor_kind_loop845(
    mut kind: self::ml_string::Text,
    mut source_ml_source1_temporary_values844: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>,
    >,
    mut source_ml_source1_temporary_index844: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Result {
    loop {
        return if (source_ml_source1_temporary_index844
            < ({
                let ml_reference_array2826 = source_ml_source1_temporary_values844.clone();
                let ml_reference_projection2826 =
                    self::ml::Big::from_u128(ml_reference_array2826.borrow().len() as u128);
                ml_reference_projection2826
            })) {
            {
                let instruction = {
                    let ml_reference_array2827 = source_ml_source1_temporary_values844.clone();
                    let ml_reference_position2827 = source_ml_source1_temporary_index844.clone();
                    let ml_reference_projection2827 = self::ml_array::at(
                        &*ml_reference_array2827.borrow(),
                        ml_reference_position2827.to_index(),
                    );
                    ml_reference_projection2827
                };
                match instruction.clone() {
                    self::JavaScriptCallInstruction::CollectionAllocate(_, _, instruction_kind) => {
                        if ((kind != (self::ml_string::Text::from_units(vec![])))
                            || !(self::ml_array::includes(
                                &{
                                    let ml_reference_value2828 =
                                        (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            self::ml_string::Text::from_units(vec![77, 97, 112]),
                                            self::ml_string::Text::from_units(vec![83, 101, 116]),
                                            self::ml_string::Text::from_units(vec![
                                                87, 101, 97, 107, 77, 97, 112,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                87, 101, 97, 107, 83, 101, 116,
                                            ]),
                                        ])))
                                        .borrow()
                                        .clone();
                                    ml_reference_value2828
                                },
                                &instruction_kind.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            )))
                        {
                            self::MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Result::MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Return(self::ml_string::Text::from_units(vec![]))
                        } else {
                            {
                                let source_ml_source1_temporary_index844_2 =
                                    source_ml_source1_temporary_index844
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (kind, source_ml_source1_temporary_index844) = (
                                        instruction_kind.clone(),
                                        source_ml_source1_temporary_index844_2.clone(),
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                    _ => {
                        let source_ml_source1_temporary_index844_3 =
                            source_ml_source1_temporary_index844.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index844 =
                                source_ml_source1_temporary_index844_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Result::MlSource1TemporaryReadJavaScriptCollectionConstructorKindLoop845Done(kind.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallTemplateLoop923Result {
    MlSource1TemporaryFindJavaScriptCallTemplateLoop923Done,
    MlSource1TemporaryFindJavaScriptCallTemplateLoop923Return(f64),
}
