// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayIterationTransition;
use super::JavaScriptArrayIteratorCall;
use super::JavaScriptArrayIteratorState;
use super::JavaScriptCollectionIteratorResultCompletion;
use super::JavaScriptCollectionIteratorResultState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptStringIterationTransition;
use super::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087;
use super::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088;
use super::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Result;
use super::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Result;
use super::advance_java_script_string_iterator;
use super::find_java_script_ordinary_object;
use super::insert_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_collection_iterator_result_prototypes_error;
use super::read_java_script_collection_iterator_result_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_read_java_script_array_iterator_state_error_loop1086;
use super::source_ml_source1_temporary_read_java_script_array_iterator_state_error_loop1089;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_string_iterator_result(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    array_prototype: self::JavaScriptSemanticValue,
    object_prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptCollectionIteratorResultCompletion {
    {
        let error = self::read_java_script_collection_iterator_result_prototypes_error(
            heap.clone(),
            array_prototype.clone(),
            object_prototype.clone(),
        );
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            {
                let next =
                    self::advance_java_script_string_iterator(heap.clone(), receiver.clone());
                match next.clone() {
                    self::JavaScriptStringIterationTransition::StringIteratorComponents(
                        heap_2,
                        _,
                        done,
                        value,
                    ) => {
                        let heap_2 = *heap_2;
                        let value = *value;
                        {
                            let state = self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116]), Box::new(receiver.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]), Box::new(array_prototype.clone()), Box::new(object_prototype.clone()), Box::new(self::JavaScriptSemanticValue::Undefined), Box::new(value.clone()), done);
                            {
                                let invalid =
                                    self::read_java_script_collection_iterator_result_state_error(
                                        heap_2.clone(),
                                        state.clone(),
                                    );
                                if (invalid == (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultContinue(Box::new(heap_2.clone()), Box::new(state.clone()))
                                } else {
                                    self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap_2.clone()), invalid.clone())
                                }
                            }
                        }
                    }
                    self::JavaScriptStringIterationTransition::StringIteratorTypeError(
                        _,
                        reason,
                    ) => {
                        self::JavaScriptCollectionIteratorResultCompletion::IteratorResultTypeError(
                            Box::new(heap.clone()),
                            reason.clone(),
                        )
                    }
                    self::JavaScriptStringIterationTransition::StringIteratorInvalid(
                        _,
                        reason_2,
                    ) => self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(
                        Box::new(heap.clone()),
                        reason_2.clone(),
                    ),
                    _ => self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            83, 116, 114, 105, 110, 103, 32, 110, 101, 120, 116, 32, 114, 101, 116,
                            97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                            32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 99, 117, 114, 115,
                            111, 114, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110,
                        ]),
                    ),
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_array_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    kind: &self::ml_string::Text,
) -> self::JavaScriptArrayIterationTransition {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            {
                let invalid =
                    self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                        Box::new(heap.clone()),
                        invalid.clone(),
                    )
                } else {
                    if !(self::ml_array::includes(
                        &{
                            let ml_reference_value1242 =
                                (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                                    self::ml_string::Text::from_units(vec![
                                        118, 97, 108, 117, 101, 115,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        101, 110, 116, 114, 105, 101, 115,
                                    ]),
                                ])))
                                .borrow()
                                .clone();
                            ml_reference_value1242
                        },
                        kind,
                        0f64,
                        false,
                        |a, b| a == b,
                    )) {
                        self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 105, 111,
                                110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101,
                                114, 101, 100, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32,
                                107, 105, 110, 100,
                            ]),
                        )
                    } else {
                        if (match receiver.clone() {
                            self::JavaScriptSemanticValue::Null => true,
                            _ => false,
                        } || match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => true,
                            _ => false,
                        }) {
                            self::JavaScriptArrayIterationTransition::ArrayIteratorError(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114,
                                    116, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 111,
                                    114, 32, 110, 117, 108, 108, 32, 116, 111, 32, 111, 98, 106,
                                    101, 99, 116,
                                ]),
                            )
                        } else {
                            match receiver.clone() {
                                self::JavaScriptSemanticValue::Object(receiver_identity, _) => {
                                    if (({ let ml_reference_array1243 = match heap.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value23834, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value23834.clone()
                                        }
                                    }; let ml_reference_projection1243 = ml_reference_array1243.borrow().len() as f64; ml_reference_projection1243 }) >= (4096f64)) {
                                        self::JavaScriptArrayIterationTransition::ArrayIteratorAllocationResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116, 32, 98, 117, 100, 103, 101, 116]))
                                    } else {
                                        {
                                            let allocated = self::insert_java_script_ordinary_object(heap.clone(), identity.clone(), prototype.clone());
                                            if !((({ let ml_reference_value1244 = allocated.borrow().clone(); ml_reference_value1244 }).0).2) {
                                                self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), (({ let ml_reference_value1245 = allocated.borrow().clone(); ml_reference_value1245 }).0).0)
                                            } else {
                                                {
                                                    let index = self::find_java_script_ordinary_object(match (({ let ml_reference_value1246 = allocated.borrow().clone(); ml_reference_value1246 }).0).1 {
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value23835, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value23835.clone()
                                                        }
                                                    }, identity.clone());
                                                    {
                                                        let row = {
                                                            let source_ml_source2_array_read_value23837 = match (({ let ml_reference_value1247 = allocated.borrow().clone(); ml_reference_value1247 }).0).1 {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value23836, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value23836.clone()
                                                                }
                                                            };
                                                            { let ml_reference_array1248 = source_ml_source2_array_read_value23837.clone(); let ml_reference_position1248 = index; let ml_reference_projection1248 = self::ml_array::optional_at(&*ml_reference_array1248.borrow(), self::ml_array::number_index(ml_reference_position1248)); ml_reference_projection1248 }
                                                        };
                                                        if (match (row.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match row.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) {
                                                            self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 79, 98, 106, 101, 99, 116]))
                                                        } else {
                                                            {
                                                                let iterator = {
                                                                    let source_ml_source2_constructor_field23849 = (((match row.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value23848, _, _, _, _, _, _, _, _) => {
                                                                            Some((source_ml_source2_data_property_value23848.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::ml::Big,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                    {
                                                                        let source_ml_source2_constructor_field23851 = (((match row.clone().expect("a proven present value") {
                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value23850, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value23850 = *source_ml_source2_data_property_value23850;
                                                                                Some((source_ml_source2_data_property_value23850.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(self::JavaScriptSemanticValue,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone();
                                                                        {
                                                                            let source_ml_source2_constructor_field23853 = ((match row.clone().expect("a proven present value") {
                                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, source_ml_source2_data_property_value23852, _, _, _, _, _, _) => {
                                                                                    Some((source_ml_source2_data_property_value23852,))
                                                                                }
                                                                                _ => {
                                                                                    None::<(bool,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0;
                                                                            {
                                                                                let source_ml_source2_constructor_field23855 = ((match row.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value23854, _) => {
                                                                                        Some((source_ml_source2_data_property_value23854,))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(bool,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0;
                                                                                {
                                                                                    let source_ml_source2_constructor_field23857 = (((match row.clone().expect("a proven present value") {
                                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23856) => {
                                                                                            Some((source_ml_source2_data_property_value23856.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                                    {
                                                                                        let source_ml_source2_constructor_field23859 = 0f64;
                                                                                        {
                                                                                            let source_ml_source2_constructor_field23862 = Some(false);
                                                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_constructor_field23849.clone(), Box::new(source_ml_source2_constructor_field23851.clone()), source_ml_source2_constructor_field23853, source_ml_source2_constructor_field23855, source_ml_source2_constructor_field23857.clone(), receiver_identity.clone(), source_ml_source2_constructor_field23859, kind.clone(), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), Some((source_ml_source2_constructor_field23862.clone(),)), None::<(Option<self::JavaScriptSemanticValue>,)>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                };
                                                                {
                                                                    let updated = self::replace_java_script_ordinary_object((({ let ml_reference_value1249 = allocated.borrow().clone(); ml_reference_value1249 }).0).1, iterator.clone());
                                                                    {
                                                                        let validation = self::read_java_script_ordinary_heap_error(updated.clone());
                                                                        if (validation == (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::JavaScriptArrayIterationTransition::ArrayIteratorAllocated(Box::new(updated.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
                                                                        } else {
                                                                            self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), validation.clone())
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorNeedBoxing(Box::new(heap.clone()), Box::new(receiver.clone()))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_iterator_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayIteratorState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArrayIteratorState::ArrayIteratorState(
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field8 = *shared_constructor_field8;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_ordinary_heap_error(heap.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        error.clone()
                    } else {
                        {
                            let source_ml_source1_temporary_values1084 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field2.clone(),
                                    shared_constructor_field3.clone(),
                                    shared_constructor_field4.clone(),
                                    shared_constructor_field8.clone(),
                                ]));
                            {
                                let source_ml_source1_temporary_index1084 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s23867 = self::source_ml_source1_temporary_read_java_script_array_iterator_state_error_loop1086(heap.clone(), source_ml_source1_temporary_values1084.clone(), source_ml_source1_temporary_index1084.clone());
                                    match source_ml_source2_s23867.clone() {
                                            self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Result::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Done => {
                                                {
                                                    let iterator = if {
                                                        let source_ml_source2_s23869 = shared_constructor_field2.clone();
                                                        match source_ml_source2_s23869.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }
                                                    } {
                                                        {
                                                            let source_ml_source2_array_read_value23877 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_array_read_index23878 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field2.clone() {
                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value23876, _) => {
                                                                        Some((source_ml_source2_data_property_value23876.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::ml::Big,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone());
                                                                { let ml_reference_array1250 = source_ml_source2_array_read_value23877.clone(); let ml_reference_position1250 = source_ml_source2_array_read_index23878; let ml_reference_projection1250 = self::ml_array::optional_at(&*ml_reference_array1250.borrow(), self::ml_array::number_index(ml_reference_position1250)); ml_reference_projection1250 }
                                                            }
                                                        }
                                                    } else {
                                                        None::<self::JavaScriptOrdinaryObject>
                                                    };
                                                    if (match (iterator.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match iterator.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 101, 120, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 101, 99, 117, 116, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 44, 32, 115, 111, 117, 114, 99, 101, 44, 32, 105, 110, 100, 101, 120, 32, 97, 110, 100, 32, 112, 104, 97, 115, 101])
                                                    } else {
                                                        if ((((((((match state.clone() {
                                                            self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, _, _) => {
                                                                false
                                                            }
                                                        } || (((((match iterator.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23900, _, _) => {
                                                                Some((source_ml_source2_data_property_value23900.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml_string::Text,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()) != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121])))) || {
                                                            let source_ml_source2_comparison_operand23902 = (((match iterator.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value23901, _) => {
                                                                    Some((((source_ml_source2_data_property_value23901.clone().unwrap_or_else(|| (None::<bool>,))).0).clone(),))
                                                                }
                                                                _ => {
                                                                    None::<(Option<bool>,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0).clone();
                                                            !(match (source_ml_source2_comparison_operand23902.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                        }) || {
                                                            let source_ml_source2_s23905 = shared_constructor_field3.clone();
                                                            match source_ml_source2_s23905.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                        }) || (((((match iterator.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, source_ml_source2_data_property_value23912, _, _, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value23912.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml::Big,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()) != ((((match shared_constructor_field3.clone() {
                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value23914, _) => {
                                                                Some((source_ml_source2_data_property_value23914.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml::Big,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()))) || ((((match iterator.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, source_ml_source2_data_property_value23917, _, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value23917,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != shared_constructor_field5)) || (((((match iterator.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value23921, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value23921.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml_string::Text,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field6)) || {
                                                            let source_ml_source2_s23924 = shared_constructor_field4.clone();
                                                            match source_ml_source2_s23924.clone() {
                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                        }) || !(self::ml_array::includes(&{ let ml_reference_value1251 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value1251 }, &shared_constructor_field7.clone(), 0f64, false, |a, b| a == b))) {
                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 101, 120, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 101, 99, 117, 116, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 44, 32, 115, 111, 117, 114, 99, 101, 44, 32, 105, 110, 100, 101, 120, 32, 97, 110, 100, 32, 112, 104, 97, 115, 101])
                                                        } else {
                                                            {
                                                                let source_ml_source2_s23943 = if (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) {
                                                                    if (((match (match state.clone() {
                                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, source_ml_source2_data_property_value23946, _) => {
                                                                            ((source_ml_source2_data_property_value23946.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match (match state.clone() {
                                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, source_ml_source2_data_property_value23949, _) => {
                                                                            ((source_ml_source2_data_property_value23949.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value") {
                                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value23950, _) => {
                                                                            source_ml_source2_data_property_value23950.clone()
                                                                        }
                                                                    }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, source_ml_source2_data_property_value23951, _) => {
                                                                            ((source_ml_source2_data_property_value23951.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value") {
                                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value23952, _, _, _) => {
                                                                            let source_ml_source2_data_property_value23952 = *source_ml_source2_data_property_value23952;
                                                                            source_ml_source2_data_property_value23952.clone()
                                                                        }
                                                                    }, shared_constructor_field8.clone()))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, source_ml_source2_data_property_value23954, _) => {
                                                                            ((source_ml_source2_data_property_value23954.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value") {
                                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value23955, _, _) => {
                                                                            let source_ml_source2_data_property_value23955 = *source_ml_source2_data_property_value23955;
                                                                            source_ml_source2_data_property_value23955.clone()
                                                                        }
                                                                    }, shared_constructor_field4.clone()))) {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 112, 117, 116, 44, 32, 104, 105, 110, 116, 32, 97, 110, 100, 32, 83, 121, 109, 98, 111, 108]))
                                                                    } else {
                                                                        {
                                                                            let invalid = self::read_java_script_primitive_conversion_state_error(heap.clone(), (match state.clone() {
                                                                                self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, source_ml_source2_data_property_value23957, _) => {
                                                                                    ((source_ml_source2_data_property_value23957.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value"));
                                                                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087Return(invalid.clone())
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087Next
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    if !(match (match state.clone() {
                                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, source_ml_source2_data_property_value23958, _) => {
                                                                            ((source_ml_source2_data_property_value23958.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087Return(self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087Next
                                                                    }
                                                                };
                                                                match source_ml_source2_s23943.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087Next => {
                                                                        {
                                                                            let source_ml_source2_s23979 = if !(match (match state.clone() {
                                                                                self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23980) => {
                                                                                    ((source_ml_source2_data_property_value23980.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                if ((!(self::ml_array::includes(&{ let ml_reference_value1252 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value1252 }, &shared_constructor_field7.clone(), 0f64, false, |a, b| a == b)) || {
                                                                                    let source_ml_source2_s23984 = match (match state.clone() {
                                                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23987) => {
                                                                                            ((source_ml_source2_data_property_value23987.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptArrayIteratorCall::ArrayIteratorCall(source_ml_source2_data_property_value23988, _, _) => {
                                                                                            let source_ml_source2_data_property_value23988 = *source_ml_source2_data_property_value23988;
                                                                                            source_ml_source2_data_property_value23988.clone()
                                                                                        }
                                                                                    };
                                                                                    match source_ml_source2_s23984.clone() {
                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                            false
                                                                                        }
                                                                                        _ => {
                                                                                            true
                                                                                        }
                                                                                    }
                                                                                }) || !(((match match (match state.clone() {
                                                                                    self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23989) => {
                                                                                        ((source_ml_source2_data_property_value23989.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value") {
                                                                                    self::JavaScriptArrayIteratorCall::ArrayIteratorCall(source_ml_source2_data_property_value23990, _, _) => {
                                                                                        let source_ml_source2_data_property_value23990 = *source_ml_source2_data_property_value23990;
                                                                                        source_ml_source2_data_property_value23990.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value23991) => {
                                                                                        Some((source_ml_source2_data_property_value23991,))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(bool,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0)) {
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 47, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 108, 108]))
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source1_temporary_values1085 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![match (match state.clone() {
                                                                                            self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23992) => {
                                                                                                ((source_ml_source2_data_property_value23992.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptArrayIteratorCall::ArrayIteratorCall(source_ml_source2_data_property_value23993, _, _) => {
                                                                                                let source_ml_source2_data_property_value23993 = *source_ml_source2_data_property_value23993;
                                                                                                source_ml_source2_data_property_value23993.clone()
                                                                                            }
                                                                                        }, match (match state.clone() {
                                                                                            self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23994) => {
                                                                                                ((source_ml_source2_data_property_value23994.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptArrayIteratorCall::ArrayIteratorCall(_, source_ml_source2_data_property_value23995, _) => {
                                                                                                let source_ml_source2_data_property_value23995 = *source_ml_source2_data_property_value23995;
                                                                                                source_ml_source2_data_property_value23995.clone()
                                                                                            }
                                                                                        }], { let ml_reference_value1253 = (match (match state.clone() {
                                                                                            self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value23996) => {
                                                                                                ((source_ml_source2_data_property_value23996.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptArrayIteratorCall::ArrayIteratorCall(_, _, source_ml_source2_data_property_value23997) => {
                                                                                                source_ml_source2_data_property_value23997.clone()
                                                                                            }
                                                                                        }).borrow().clone(); ml_reference_value1253 })));
                                                                                        {
                                                                                            let source_ml_source1_temporary_index1085 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source2_s23998 = self::source_ml_source1_temporary_read_java_script_array_iterator_state_error_loop1089(heap.clone(), source_ml_source1_temporary_values1085.clone(), source_ml_source1_temporary_index1085.clone());
                                                                                                match source_ml_source2_s23998.clone() {
                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Result::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Done => {
                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088Next
                                                                                                    }
                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Result::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1089Return(source_ml_source1_temporary_result1089) => {
                                                                                                        self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088Return(source_ml_source1_temporary_result1089.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                if shared_constructor_field7.ends_with(&self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114])) {
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 112, 104, 97, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088Next
                                                                                }
                                                                            };
                                                                            match source_ml_source2_s23979.clone() {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088Next => {
                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                }
                                                                                self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1088Return(source_ml_source1_temporary_result1088) => {
                                                                                    source_ml_source1_temporary_result1088.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorJoin1087Return(source_ml_source1_temporary_result1089_2) => {
                                                                        source_ml_source1_temporary_result1089_2.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Result::MlSource1TemporaryReadJavaScriptArrayIteratorStateErrorLoop1086Return(source_ml_source1_temporary_result1089_3) => {
                                                source_ml_source1_temporary_result1089_3.clone()
                                            }
                                        }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
