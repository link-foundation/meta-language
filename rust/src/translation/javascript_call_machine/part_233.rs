// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionCompletion;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptObjectSpreadTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_allocate_java_script_weak_collection_loop294(
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
                        let ml_reference_array2751 = shared_constructor_field1.clone();
                        let ml_reference_projection2751 =
                            ml_reference_array2751.borrow().len() as f64;
                        ml_reference_projection2751
                    }))
                    && ((({
                        let source_ml_source2_optional_chain_receiver40443 = ({
                            let source_ml_source2_array_read_value40441 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array2752 =
                                    source_ml_source2_array_read_value40441.clone();
                                let ml_reference_position2752 = index;
                                let ml_reference_projection2752 = self::ml_array::optional_at(
                                    &*ml_reference_array2752.borrow(),
                                    self::ml_array::number_index(ml_reference_position2752),
                                );
                                ml_reference_projection2752
                            }
                        },);
                        if match (source_ml_source2_optional_chain_receiver40443.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            Some({
                                let source_ml_source1_temporary_optional_chain293 =
                                    ((source_ml_source2_optional_chain_receiver40443.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_temporary_optional_chain293.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        source_ml_source2_data_property_value40444,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40444.clone(),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        source_ml_source2_data_property_value40445,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40445.clone(),
                                    self::JavaScriptOrdinaryObject::ArrayObject(
                                        source_ml_source2_data_property_value40446,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40446.clone(),
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        source_ml_source2_data_property_value40447,
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
                                    ) => source_ml_source2_data_property_value40447.clone(),
                                    self::JavaScriptOrdinaryObject::MapObject(
                                        source_ml_source2_data_property_value40448,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40448.clone(),
                                    self::JavaScriptOrdinaryObject::SetObject(
                                        source_ml_source2_data_property_value40449,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40449.clone(),
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        source_ml_source2_data_property_value40450,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40450.clone(),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        source_ml_source2_data_property_value40451,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40451.clone(),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        source_ml_source2_data_property_value40452,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40452.clone(),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        source_ml_source2_data_property_value40453,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40453.clone(),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        source_ml_source2_data_property_value40454,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40454.clone(),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        source_ml_source2_data_property_value40455,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40455.clone(),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        source_ml_source2_data_property_value40456,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value40456.clone(),
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
pub enum MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Result {
    MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Done,
    MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Return(
        Box<self::JavaScriptCollectionCompletion>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_weak_collection_operation_loop297(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values295: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index295: self::ml::Big,
) -> self::MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Result {
    loop {
        return if (source_ml_source1_temporary_index295
            < ({
                let ml_reference_array2753 = source_ml_source1_temporary_values295.clone();
                let ml_reference_projection2753 =
                    self::ml::Big::from_u128(ml_reference_array2753.borrow().len() as u128);
                ml_reference_projection2753
            })) {
            {
                let value = {
                    let ml_reference_array2754 = source_ml_source1_temporary_values295.clone();
                    let ml_reference_position2754 = source_ml_source1_temporary_index295.clone();
                    let ml_reference_projection2754 = self::ml_array::at(
                        &*ml_reference_array2754.borrow(),
                        ml_reference_position2754.to_index(),
                    );
                    ml_reference_projection2754
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Result::MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Return(Box::new(self::JavaScriptCollectionCompletion::CollectionInvalid(Box::new(heap.clone()), invalid.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index295_2 =
                                source_ml_source1_temporary_index295
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index295 =
                                    source_ml_source1_temporary_index295_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Result::MlSource1TemporaryExecuteJavaScriptWeakCollectionOperationLoop297Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Result {
    MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Done,
    MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_duplicate_java_script_object_spread_key_name_loop298(
    mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
    mut at: f64,
) -> self::MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Result {
    loop {
        return if (at
            < ({
                let ml_reference_array2755 = ordered.clone();
                let ml_reference_projection2755 = ml_reference_array2755.borrow().len() as f64;
                ml_reference_projection2755
            })) {
            if {
                let source_ml_source2_comparison_operand40463 = {
                    let ml_reference_array2756 = ordered.clone();
                    let ml_reference_position2756 = at;
                    let ml_reference_projection2756 = self::ml_array::optional_at(
                        &*ml_reference_array2756.borrow(),
                        self::ml_array::number_index(ml_reference_position2756),
                    );
                    ml_reference_projection2756
                };
                {
                    let source_ml_source2_comparison_operand40464 = {
                        let source_ml_source2_array_read_index40462 = (at - 1f64);
                        {
                            let ml_reference_array2757 = ordered.clone();
                            let ml_reference_position2757 = source_ml_source2_array_read_index40462;
                            let ml_reference_projection2757 = self::ml_array::optional_at(
                                &*ml_reference_array2757.borrow(),
                                self::ml_array::number_index(ml_reference_position2757),
                            );
                            ml_reference_projection2757
                        }
                    };
                    match (
                        source_ml_source2_comparison_operand40463.clone(),
                        source_ml_source2_comparison_operand40464.clone(),
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } {
                self::MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Result::MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Return(true)
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
            self::MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Result::MlSource1TemporaryTestDuplicateJavaScriptObjectSpreadKeyNameLoop298Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Result {
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Done,
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_spread_state_error_loop309(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values302: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index302: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Result {
    loop {
        return if (source_ml_source1_temporary_index302
            < ({
                let ml_reference_array2758 = source_ml_source1_temporary_values302.clone();
                let ml_reference_projection2758 =
                    self::ml::Big::from_u128(ml_reference_array2758.borrow().len() as u128);
                ml_reference_projection2758
            })) {
            {
                let key = {
                    let ml_reference_array2759 = source_ml_source1_temporary_values302.clone();
                    let ml_reference_position2759 = source_ml_source1_temporary_index302.clone();
                    let ml_reference_projection2759 = self::ml_array::at(
                        &*ml_reference_array2759.borrow(),
                        ml_reference_position2759.to_index(),
                    );
                    ml_reference_projection2759
                };
                {
                    let error =
                        self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index302_2 =
                                source_ml_source1_temporary_index302
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index302 =
                                    source_ml_source1_temporary_index302_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop309Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Result {
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Done,
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_spread_state_error_loop308(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values301: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index301: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Result {
    loop {
        return if (source_ml_source1_temporary_index301
            < ({
                let ml_reference_array2760 = source_ml_source1_temporary_values301.clone();
                let ml_reference_projection2760 =
                    self::ml::Big::from_u128(ml_reference_array2760.borrow().len() as u128);
                ml_reference_projection2760
            })) {
            {
                let key = {
                    let ml_reference_array2761 = source_ml_source1_temporary_values301.clone();
                    let ml_reference_position2761 = source_ml_source1_temporary_index301.clone();
                    let ml_reference_projection2761 = self::ml_array::at(
                        &*ml_reference_array2761.borrow(),
                        ml_reference_position2761.to_index(),
                    );
                    ml_reference_projection2761
                };
                {
                    let error =
                        self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index301_2 =
                                source_ml_source1_temporary_index301
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index301 =
                                    source_ml_source1_temporary_index301_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop308Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307 {
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307Next,
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin307Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Result {
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Done,
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_spread_state_error_loop310(
    mut state: self::JavaScriptObjectSpreadState,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Result {
    loop {
        return match state.clone() {
            self::JavaScriptObjectSpreadState::ObjectSpreadState(
                _,
                _,
                _,
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (at
                    < ({
                        let ml_reference_array2762 = shared_constructor_field1.clone();
                        let ml_reference_projection2762 =
                            ml_reference_array2762.borrow().len() as f64;
                        ml_reference_projection2762
                    }))
                {
                    {
                        let key = {
                            let source_ml_source2_array_read_value40472 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array2763 =
                                    source_ml_source2_array_read_value40472.clone();
                                let ml_reference_position2763 = at;
                                let ml_reference_projection2763 = self::ml_array::optional_at(
                                    &*ml_reference_array2763.borrow(),
                                    self::ml_array::number_index(ml_reference_position2763),
                                );
                                ml_reference_projection2763
                            }
                        };
                        if ((match (key.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || match key.clone().expect("a proven present value") {
                            self::JavaScriptSemanticValue::Text(_) => false,
                            _ => true,
                        }) || ((self::read_java_script_semantic_text_string(
                            key.clone().expect("a proven present value"),
                        )) != (self::ml_string::Text::from_text(
                            &self::ml_number::js_number(at),
                        )))) {
                            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 115, 112, 114, 101, 97, 100, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 105, 110, 100, 105, 99, 101, 115]))
                        } else {
                            {
                                let at_2 = (at + 1f64);
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop310Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306 {
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306Next,
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin306Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305 {
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305Next,
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin305Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Result {
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Done,
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_spread_state_error_loop311(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values300: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index300: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Result {
    loop {
        return if (source_ml_source1_temporary_index300
            < ({
                let ml_reference_array2764 = source_ml_source1_temporary_values300.clone();
                let ml_reference_projection2764 =
                    self::ml::Big::from_u128(ml_reference_array2764.borrow().len() as u128);
                ml_reference_projection2764
            })) {
            {
                let value = {
                    let ml_reference_array2765 = source_ml_source1_temporary_values300.clone();
                    let ml_reference_position2765 = source_ml_source1_temporary_index300.clone();
                    let ml_reference_projection2765 = self::ml_array::at(
                        &*ml_reference_array2765.borrow(),
                        ml_reference_position2765.to_index(),
                    );
                    ml_reference_projection2765
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index300_2 =
                                source_ml_source1_temporary_index300
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index300 =
                                    source_ml_source1_temporary_index300_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop311Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304 {
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304Next,
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorJoin304Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Result {
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Done,
    MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_spread_state_error_loop303(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values299: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index299: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Result {
    loop {
        return if (source_ml_source1_temporary_index299
            < ({
                let ml_reference_array2766 = source_ml_source1_temporary_values299.clone();
                let ml_reference_projection2766 =
                    self::ml::Big::from_u128(ml_reference_array2766.borrow().len() as u128);
                ml_reference_projection2766
            })) {
            {
                let value = {
                    let ml_reference_array2767 = source_ml_source1_temporary_values299.clone();
                    let ml_reference_position2767 = source_ml_source1_temporary_index299.clone();
                    let ml_reference_projection2767 = self::ml_array::at(
                        &*ml_reference_array2767.borrow(),
                        ml_reference_position2767.to_index(),
                    );
                    ml_reference_projection2767
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index299_2 =
                                source_ml_source1_temporary_index299
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index299 =
                                    source_ml_source1_temporary_index299_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Result::MlSource1TemporaryReadJavaScriptObjectSpreadStateErrorLoop303Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptObjectSpreadJoin313 {
    MlSource1TemporaryStepJavaScriptObjectSpreadJoin313Next(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    MlSource1TemporaryStepJavaScriptObjectSpreadJoin313Return(
        Box<self::JavaScriptObjectSpreadTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptObjectSpreadLoop314Result {
    MlSource1TemporaryStepJavaScriptObjectSpreadLoop314Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptObjectSpreadLoop315Result {
    MlSource1TemporaryStepJavaScriptObjectSpreadLoop315Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_object_spread_loop315(
    key: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut unit: f64,
) -> self::MlSource1TemporaryStepJavaScriptObjectSpreadLoop315Result {
    loop {
        return if (unit < (key.length())) {
            {
                let source_ml_source1_temporary_step_java_script_object_spread_ignored317 = {
                    let ml_method_owner2768 = units.clone();
                    let ml_method_argument2768_0 = {
                        let source_ml_source2_string_unit_operand40481 = key.clone();
                        {
                            let source_ml_source2_string_unit_operand40482 = unit;
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
                                    let source_ml_source2_string_unit_operand40488 = {
                                        let source_ml_source2_string_range_operand40483 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand40484 = index;
                                            {
                                                let source_ml_source2_string_range_operand40485 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position40486 = source_ml_source2_string_range_operand40484;
                                                    {
                                                        let source_ml_source2_string_range_position40487 = source_ml_source2_string_range_operand40485;
                                                        source_ml_source2_string_range_operand40483.slice(source_ml_source2_string_range_position40486, source_ml_source2_string_range_position40487)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand40489 = 0f64;
                                        source_ml_source2_string_unit_operand40488.code_point_at(source_ml_source2_string_unit_operand40489)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand40481.clone(),
                                source_ml_source2_string_unit_operand40482,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument2768_0];
                    let mut ml_method_values2768 = ml_method_owner2768.borrow_mut();
                    let ml_length = ml_method_values2768
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2768.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let unit_2 = (unit + 1f64);
                    {
                        unit = unit_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryStepJavaScriptObjectSpreadLoop315Result::MlSource1TemporaryStepJavaScriptObjectSpreadLoop315Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_object_spread_loop314(
    mut keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut length: f64,
    mut at: f64,
) -> self::MlSource1TemporaryStepJavaScriptObjectSpreadLoop314Result {
    loop {
        return if (at < length) {
            {
                let key = self::ml_string::Text::from_text(&self::ml_number::js_number(at));
                {
                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let unit = 0f64;
                        {
                            let source_ml_source2_s40491 = self::source_ml_source1_temporary_step_java_script_object_spread_loop315(&(key), units.clone(), unit);
                            match source_ml_source2_s40491.clone() {
                                self::MlSource1TemporaryStepJavaScriptObjectSpreadLoop315Result::MlSource1TemporaryStepJavaScriptObjectSpreadLoop315Done => {
                                    {
                                        let source_ml_source1_temporary_step_java_script_object_spread_ignored316 = { let ml_method_owner2769 = keys.clone(); let ml_method_argument2769_0 = self::JavaScriptSemanticValue::Text(units.clone()); let ml_inserted = vec![ml_method_argument2769_0]; let mut ml_method_values2769 = ml_method_owner2769.borrow_mut(); let ml_length = ml_method_values2769.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values2769.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let at_2 = (at + 1f64);
                                            {
                                                at = at_2;
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
            self::MlSource1TemporaryStepJavaScriptObjectSpreadLoop314Result::MlSource1TemporaryStepJavaScriptObjectSpreadLoop314Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptObjectSpreadJoin318 {
    MlSource1TemporaryStepJavaScriptObjectSpreadJoin318Next,
    MlSource1TemporaryStepJavaScriptObjectSpreadJoin318Return(
        Box<self::JavaScriptObjectSpreadTransition>,
    ),
}
