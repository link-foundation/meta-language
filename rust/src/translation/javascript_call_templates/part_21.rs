// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptClosureAllocation;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_value_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_allocate_java_script_call_closure_loop175(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut identity: self::ml::Big,
    mut at: f64,
) -> f64 {
    loop {
        if (at
            < ({
                let ml_reference_array203 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        source_ml_source1_data_property_value21445,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value21445.clone(),
                };
                ml_reference_array203.borrow().len() as f64
            }))
            && ((({
                let source_ml_source1_optional_chain_receiver21466 = ({
                    let source_ml_source1_array_read_value21464 = match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value21463,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value21463.clone(),
                    };
                    {
                        let ml_reference_array204 = source_ml_source1_array_read_value21464;
                        let ml_reference_position204 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array204.borrow(),
                            self::ml_array::number_index(ml_reference_position204),
                        )
                    }
                },);
                if match (source_ml_source1_optional_chain_receiver21466.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    Some({
                        let source_ml_temporary_optional_chain172 =
                            ((source_ml_source1_optional_chain_receiver21466.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        match source_ml_temporary_optional_chain172 {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                source_ml_source1_data_property_value21467,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21467.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                source_ml_source1_data_property_value21468,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21468.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                source_ml_source1_data_property_value21469,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21469.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                source_ml_source1_data_property_value21470,
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
                            ) => source_ml_source1_data_property_value21470.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                source_ml_source1_data_property_value21471,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21471.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                source_ml_source1_data_property_value21472,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21472.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                source_ml_source1_data_property_value21473,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21473.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                source_ml_source1_data_property_value21474,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21474.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                source_ml_source1_data_property_value21475,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21475.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                source_ml_source1_data_property_value21476,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21476.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                source_ml_source1_data_property_value21477,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21477.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                source_ml_source1_data_property_value21478,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21478.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                source_ml_source1_data_property_value21479,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21479.clone(),
                        }
                    })
                } else {
                    None::<self::ml::Big>
                }
            })
            .clone()
            .unwrap_or_else(|| identity.clone()))
                < identity)
        {
            {
                let at_2 = at + 1f64;
                {
                    at = at_2;
                    continue;
                }
            }
        }
        return { at };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAllocateJavaScriptCallClosureLoop174Result {
    MlTemporaryAllocateJavaScriptCallClosureLoop174Done,
    MlTemporaryAllocateJavaScriptCallClosureLoop174Return(Box<self::JavaScriptClosureAllocation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_allocate_java_script_call_closure_loop174(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values171: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index171: self::ml::Big,
) -> self::MlTemporaryAllocateJavaScriptCallClosureLoop174Result {
    loop {
        return if source_ml_temporary_index171
            < ({
                let ml_reference_array205 = source_ml_temporary_values171.clone();
                self::ml::Big::from_u128(ml_reference_array205.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array206 = source_ml_temporary_values171.clone();
                    let ml_reference_position206 = source_ml_temporary_index171.clone();
                    self::ml_array::at(
                        &*ml_reference_array206.borrow(),
                        ml_reference_position206.to_index(),
                    )
                };
                {
                    let invalid = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if invalid == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index171_2 =
                                source_ml_temporary_index171.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index171 = source_ml_temporary_index171_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryAllocateJavaScriptCallClosureLoop174Result::MlTemporaryAllocateJavaScriptCallClosureLoop174Return(Box::new(self::JavaScriptClosureAllocation::ClosureAllocationInvalid(Box::new(heap.clone()), invalid.clone())))
                    }
                }
            }
        } else {
            self::MlTemporaryAllocateJavaScriptCallClosureLoop174Result::MlTemporaryAllocateJavaScriptCallClosureLoop174Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryAllocateJavaScriptCallClosureLoop173Result {
    MlTemporaryAllocateJavaScriptCallClosureLoop173Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_allocate_java_script_call_closure_loop173(
    mut capture_count: f64,
    mut properties: f64,
    mut source_ml_temporary_values170: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_temporary_index170: self::ml::Big,
) -> self::MlTemporaryAllocateJavaScriptCallClosureLoop173Result {
    loop {
        return if source_ml_temporary_index170
            < ({
                let ml_reference_array207 = source_ml_temporary_values170.clone();
                self::ml::Big::from_u128(ml_reference_array207.borrow().len() as u128)
            }) {
            {
                let object = {
                    let ml_reference_array208 = source_ml_temporary_values170.clone();
                    let ml_reference_position208 = source_ml_temporary_index170.clone();
                    self::ml_array::at(
                        &*ml_reference_array208.borrow(),
                        ml_reference_position208.to_index(),
                    )
                };
                {
                    let properties_2 = properties + {
                        let ml_reference_array209 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21484,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21484.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21485,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21485.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21486,
                            ) => source_ml_source1_data_property_value21486.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21487,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21487.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21488,
                                _,
                            ) => source_ml_source1_data_property_value21488.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21489,
                                _,
                            ) => source_ml_source1_data_property_value21489.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21490,
                                _,
                            ) => source_ml_source1_data_property_value21490.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21491,
                                _,
                            ) => source_ml_source1_data_property_value21491.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21492,
                            ) => source_ml_source1_data_property_value21492.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21493,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21493.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21494,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21494.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21495,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21495.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value21496,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value21496.clone(),
                        };
                        ml_reference_array209.borrow().len() as f64
                    };
                    match object.clone() {
                        self::JavaScriptOrdinaryObject::ClosureObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            captures,
                            _,
                            _,
                        ) => {
                            let capture_count_2 = capture_count + {
                                let ml_reference_array210 = captures.clone();
                                ml_reference_array210.borrow().len() as f64
                            };
                            {
                                let source_ml_temporary_index170_2 =
                                    source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                                {
                                    (capture_count, properties, source_ml_temporary_index170) = (
                                        capture_count_2,
                                        properties_2,
                                        source_ml_temporary_index170_2,
                                    );
                                    continue;
                                }
                            }
                        }
                        self::JavaScriptOrdinaryObject::BoundFunctionObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_temporary_index170_3 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_3);
                                continue;
                            }
                        }
                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                            let source_ml_temporary_index170_4 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_4);
                                continue;
                            }
                        }
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
                        ) => {
                            let source_ml_temporary_index170_5 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_5);
                                continue;
                            }
                        }
                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                            let source_ml_temporary_index170_6 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_6);
                                continue;
                            }
                        }
                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                            let source_ml_temporary_index170_7 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_7);
                                continue;
                            }
                        }
                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                            let source_ml_temporary_index170_8 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_8);
                                continue;
                            }
                        }
                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                            let source_ml_temporary_index170_9 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_9);
                                continue;
                            }
                        }
                        self::JavaScriptOrdinaryObject::OrdinaryObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_temporary_index170_10 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_10);
                                continue;
                            }
                        }
                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                            let source_ml_temporary_index170_11 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_11);
                                continue;
                            }
                        }
                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                            let source_ml_temporary_index170_12 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_12);
                                continue;
                            }
                        }
                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                            let source_ml_temporary_index170_13 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_13);
                                continue;
                            }
                        }
                        self::JavaScriptOrdinaryObject::HostByteBufferObject(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_temporary_index170_14 =
                                source_ml_temporary_index170.add(&self::ml::Big::from_i128(1));
                            {
                                (properties, source_ml_temporary_index170) =
                                    (properties_2, source_ml_temporary_index170_14);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAllocateJavaScriptCallClosureLoop173Result::MlTemporaryAllocateJavaScriptCallClosureLoop173Done(capture_count, properties)
        };
    }
}
