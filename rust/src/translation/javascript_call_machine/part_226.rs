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
pub fn source_ml_source1_temporary_insert_java_script_ordinary_object_loop129(
    mut identity: self::ml::Big,
    mut at: f64,
    mut source_ml_source1_temporary_values127: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index127: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index127
            < ({
                let ml_reference_array2649 = source_ml_source1_temporary_values127.clone();
                let ml_reference_projection2649 =
                    self::ml::Big::from_u128(ml_reference_array2649.borrow().len() as u128);
                ml_reference_projection2649
            })) {
            {
                let object = {
                    let ml_reference_array2650 = source_ml_source1_temporary_values127.clone();
                    let ml_reference_position2650 = source_ml_source1_temporary_index127.clone();
                    let ml_reference_projection2650 = self::ml_array::at(
                        &*ml_reference_array2650.borrow(),
                        ml_reference_position2650.to_index(),
                    );
                    ml_reference_projection2650
                };
                if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::ClosureObject(
                        source_ml_source2_data_property_value39820,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39820.clone(),
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        source_ml_source2_data_property_value39821,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39821.clone(),
                    self::JavaScriptOrdinaryObject::ArrayObject(
                        source_ml_source2_data_property_value39822,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39822.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        source_ml_source2_data_property_value39823,
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
                    ) => source_ml_source2_data_property_value39823.clone(),
                    self::JavaScriptOrdinaryObject::MapObject(
                        source_ml_source2_data_property_value39824,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39824.clone(),
                    self::JavaScriptOrdinaryObject::SetObject(
                        source_ml_source2_data_property_value39825,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39825.clone(),
                    self::JavaScriptOrdinaryObject::WeakMapObject(
                        source_ml_source2_data_property_value39826,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39826.clone(),
                    self::JavaScriptOrdinaryObject::WeakSetObject(
                        source_ml_source2_data_property_value39827,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39827.clone(),
                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                        source_ml_source2_data_property_value39828,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39828.clone(),
                    self::JavaScriptOrdinaryObject::HostErrorObject(
                        source_ml_source2_data_property_value39829,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39829.clone(),
                    self::JavaScriptOrdinaryObject::HostURLObject(
                        source_ml_source2_data_property_value39830,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39830.clone(),
                    self::JavaScriptOrdinaryObject::HostArrayObject(
                        source_ml_source2_data_property_value39831,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39831.clone(),
                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                        source_ml_source2_data_property_value39832,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value39832.clone(),
                }) < identity)
                {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_source1_temporary_index127_2 =
                                source_ml_source1_temporary_index127
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_source1_temporary_index127) =
                                    (at_2, source_ml_source1_temporary_index127_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index127_3 =
                            source_ml_source1_temporary_index127.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index127 =
                                source_ml_source1_temporary_index127_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            at
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128 {
    MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128Next,
    MlSource1TemporaryInsertJavaScriptOrdinaryObjectJoin128Return(
        std::rc::Rc<
            std::cell::RefCell<(
                (self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool),
                Vec<f64>,
            )>,
        >,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_register_java_script_ordinary_symbol_loop131(
    mut identity: self::ml::Big,
    mut at: f64,
    mut source_ml_source1_temporary_values130: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut source_ml_source1_temporary_index130: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index130
            < ({
                let ml_reference_array2651 = source_ml_source1_temporary_values130.clone();
                let ml_reference_projection2651 =
                    self::ml::Big::from_u128(ml_reference_array2651.borrow().len() as u128);
                ml_reference_projection2651
            })) {
            {
                let symbol = {
                    let ml_reference_array2652 = source_ml_source1_temporary_values130.clone();
                    let ml_reference_position2652 = source_ml_source1_temporary_index130.clone();
                    let ml_reference_projection2652 = self::ml_array::at(
                        &*ml_reference_array2652.borrow(),
                        ml_reference_position2652.to_index(),
                    );
                    ml_reference_projection2652
                };
                if (symbol < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_source1_temporary_index130_2 =
                                source_ml_source1_temporary_index130
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_source1_temporary_index130) =
                                    (at_2, source_ml_source1_temporary_index130_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index130_3 =
                            source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index130 =
                                source_ml_source1_temporary_index130_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            at
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocate_java_script_array_loop134(
    mut count: f64,
    mut source_ml_source1_temporary_values132: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index132: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index132
            < ({
                let ml_reference_array2653 = source_ml_source1_temporary_values132.clone();
                let ml_reference_projection2653 =
                    self::ml::Big::from_u128(ml_reference_array2653.borrow().len() as u128);
                ml_reference_projection2653
            })) {
            {
                let object = {
                    let ml_reference_array2654 = source_ml_source1_temporary_values132.clone();
                    let ml_reference_position2654 = source_ml_source1_temporary_index132.clone();
                    let ml_reference_projection2654 = self::ml_array::at(
                        &*ml_reference_array2654.borrow(),
                        ml_reference_position2654.to_index(),
                    );
                    ml_reference_projection2654
                };
                {
                    let count_2 = (count + {
                        let ml_reference_array2655 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39837,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39837.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39838,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39838.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39839,
                            ) => source_ml_source2_data_property_value39839.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39840,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39840.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39841,
                                _,
                            ) => source_ml_source2_data_property_value39841.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39842,
                                _,
                            ) => source_ml_source2_data_property_value39842.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39843,
                                _,
                            ) => source_ml_source2_data_property_value39843.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39844,
                                _,
                            ) => source_ml_source2_data_property_value39844.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39845,
                            ) => source_ml_source2_data_property_value39845.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39846,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39846.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39847,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39847.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39848,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39848.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value39849,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value39849.clone(),
                        };
                        let ml_reference_projection2655 =
                            ml_reference_array2655.borrow().len() as f64;
                        ml_reference_projection2655
                    });
                    {
                        let source_ml_source1_temporary_index132_2 =
                            source_ml_source1_temporary_index132.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index132) =
                                (count_2, source_ml_source1_temporary_index132_2.clone());
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocate_java_script_array_loop135(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut at: f64,
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
                if ((at
                    < ({
                        let ml_reference_array2656 = shared_constructor_field1.clone();
                        let ml_reference_projection2656 =
                            ml_reference_array2656.borrow().len() as f64;
                        ml_reference_projection2656
                    }))
                    && ((({
                        let source_ml_source2_optional_chain_receiver39872 = ({
                            let source_ml_source2_array_read_value39870 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array2657 =
                                    source_ml_source2_array_read_value39870.clone();
                                let ml_reference_position2657 = at;
                                let ml_reference_projection2657 = self::ml_array::optional_at(
                                    &*ml_reference_array2657.borrow(),
                                    self::ml_array::number_index(ml_reference_position2657),
                                );
                                ml_reference_projection2657
                            }
                        },);
                        if match (source_ml_source2_optional_chain_receiver39872.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            Some({
                                let source_ml_source1_temporary_optional_chain133 =
                                    ((source_ml_source2_optional_chain_receiver39872.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_temporary_optional_chain133.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        source_ml_source2_data_property_value39873,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39873.clone(),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        source_ml_source2_data_property_value39874,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39874.clone(),
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        source_ml_source2_data_property_value39875,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39875.clone(),
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        source_ml_source2_data_property_value39876,
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
                                    ) => source_ml_source2_data_property_value39876.clone(),
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        source_ml_source2_data_property_value39877,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39877.clone(),
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        source_ml_source2_data_property_value39878,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39878.clone(),
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        source_ml_source2_data_property_value39879,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39879.clone(),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        source_ml_source2_data_property_value39880,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39880.clone(),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        source_ml_source2_data_property_value39881,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39881.clone(),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        source_ml_source2_data_property_value39882,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39882.clone(),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        source_ml_source2_data_property_value39883,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39883.clone(),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        source_ml_source2_data_property_value39884,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39884.clone(),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        source_ml_source2_data_property_value39885,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value39885.clone(),
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
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                } else {
                    at
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result {
    MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Done,
    MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_array_definition_state_error_loop137(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values136: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index136: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result {
    loop {
        return if (source_ml_source1_temporary_index136
            < ({
                let ml_reference_array2658 = source_ml_source1_temporary_values136.clone();
                let ml_reference_projection2658 =
                    self::ml::Big::from_u128(ml_reference_array2658.borrow().len() as u128);
                ml_reference_projection2658
            })) {
            {
                let value = {
                    let ml_reference_array2659 = source_ml_source1_temporary_values136.clone();
                    let ml_reference_position2659 = source_ml_source1_temporary_index136.clone();
                    let ml_reference_projection2659 = self::ml_array::at(
                        &*ml_reference_array2659.borrow(),
                        ml_reference_position2659.to_index(),
                    );
                    ml_reference_projection2659
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index136_2 =
                                source_ml_source1_temporary_index136
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index136 =
                                    source_ml_source1_temporary_index136_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Result {
    MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Done,
    MlSource1TemporaryReadJavaScriptIndexedPropertyPresenceLoop138Return(
        std::rc::Rc<std::cell::RefCell<((self::ml_string::Text, bool), Vec<f64>)>>,
    ),
}
