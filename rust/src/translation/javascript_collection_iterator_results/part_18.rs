// Generated from complete js/src/translation/javascript-collection-iterator-results.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptCollectionIteration;
use super::JavaScriptCollectionIteratorResultCompletion;
use super::JavaScriptCollectionIteratorResultState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Result;
use super::MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Result;
use super::advance_java_script_branded_collection_iterator;
use super::construct_java_script_array_object;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_allocate_java_script_array_loop139;
use super::source_ml_temporary_allocate_java_script_array_loop140;
use super::source_ml_temporary_read_java_script_collection_iterator_result_prototypes_error_loop144;
use super::source_ml_temporary_read_java_script_collection_iterator_result_state_error_loop146;
use super::source_ml_temporary_step_java_script_collection_iterator_result_loop148;
use super::test_java_script_array_length;
use super::test_java_script_ordinary_symbol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_array(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
    length: f64,
) -> self::JavaScriptArrayCompletion {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_3 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_3 {
                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), error.clone())
            } else if (((identity <= (self::ml::Big::from_i128(0)))
                || (identity > (self::ml::Big::from_i128(18_446_744_073_709_551_615))))
                || ((self::find_java_script_ordinary_object(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            source_ml_source1_data_property_value17016,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17016.clone(),
                    },
                    identity.clone(),
                )) >= 0f64))
                || self::test_java_script_ordinary_symbol(
                    match heap.clone() {
                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                            _,
                            source_ml_source1_data_property_value17017,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value17017.clone(),
                    },
                    identity.clone(),
                )
            {
                self::JavaScriptArrayCompletion::ArrayInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
                        32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 102, 114, 101, 115,
                        104, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 117, 110, 115, 105,
                        103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100,
                        101, 110, 116, 105, 116, 121,
                    ]),
                )
            } else {
                {
                    let actual = if length == 0f64 { 0f64 } else { length };
                    if (self::test_java_script_array_length(actual)) {
                        if match prototype.clone() {
                            self::JavaScriptSemanticValue::Null => false,
                            _ => true,
                        } && match prototype.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        } {
                            self::JavaScriptArrayCompletion::ArrayInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                    111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                    115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117, 108, 108, 32,
                                    111, 114, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111,
                                    116, 111, 116, 121, 112, 101,
                                ]),
                            )
                        } else {
                            {
                                let invalid = self::read_java_script_ordinary_value_error(
                                    heap.clone(),
                                    prototype.clone(),
                                );
                                {
                                    let ml_library_inequality_2 =
                                        invalid != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_2 {
                                        self::JavaScriptArrayCompletion::ArrayInvalid(
                                            Box::new(heap.clone()),
                                            invalid.clone(),
                                        )
                                    } else {
                                        {
                                            let count = 0f64;
                                            {
                                                let source_ml_temporary_values137 = match heap
                                                    .clone()
                                                {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                        source_ml_source1_data_property_value17036,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source1_data_property_value17036
                                                        .clone(),
                                                };
                                                {
                                                    let source_ml_temporary_index137 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let count_2 = self::source_ml_temporary_allocate_java_script_array_loop139(count, source_ml_temporary_values137, source_ml_temporary_index137);
                                                        if (({
                                                            let ml_reference_array212 = match heap.clone() {
                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17038, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value17038.clone()
                                                        }
                                                    };
                                                            ml_reference_array212.borrow().len()
                                                                as f64
                                                        }) >= 4096f64)
                                                            || (count_2 >= 16_384f64)
                                                        {
                                                            self::JavaScriptArrayCompletion::ArrayResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 32, 111, 114, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116]), None::<(Option<self::JavaScriptArrayDefinitionState>,)>)
                                                        } else {
                                                            {
                                                                let object = self::construct_java_script_array_object(identity.clone(), prototype.clone(), actual);
                                                                {
                                                                    let at = 0f64;
                                                                    {
                                                                        let at_2 = self::source_ml_temporary_allocate_java_script_array_loop140(heap.clone(), identity.clone(), at);
                                                                        {
                                                                            let next = {
                                                                                let source_ml_source1_nominal_field17049 = match heap.clone() {
                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17050, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value17050.clone()
                                                                                }
                                                                            };
                                                                                {
                                                                                    let source_ml_source1_nominal_field17051 = match heap.clone() {
                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, source_ml_source1_data_property_value17052, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value17052.clone()
                                                                                    }
                                                                                };
                                                                                    {
                                                                                        let source_ml_source1_nominal_field17053 = match heap.clone() {
                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, source_ml_source1_data_property_value17054, _, _, _) => {
                                                                                            source_ml_source1_data_property_value17054.clone()
                                                                                        }
                                                                                    };
                                                                                        {
                                                                                            let source_ml_source1_nominal_field17055 = match heap.clone() {
                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, source_ml_source1_data_property_value17056, _, _) => {
                                                                                                source_ml_source1_data_property_value17056.clone()
                                                                                            }
                                                                                        };
                                                                                            {
                                                                                                let source_ml_source1_nominal_field17057 = match heap.clone() {
                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, source_ml_source1_data_property_value17058, _) => {
                                                                                                    source_ml_source1_data_property_value17058.clone()
                                                                                                }
                                                                                            };
                                                                                                {
                                                                                                    let source_ml_source1_nominal_field17059 = match heap.clone() {
                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(_, _, _, _, _, source_ml_source1_data_property_value17060) => {
                                                                                                        source_ml_source1_data_property_value17060.clone()
                                                                                                    }
                                                                                                };
                                                                                                    {
                                                                                                        let source_ml_source1_nominal_override17061 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                        let source_ml_source1_concat_value17045 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ (match heap.clone() {
                                                                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17042, _, _, _, _, _) => {
                                                                                                                source_ml_source1_data_property_value17042.clone()
                                                                                                            }
                                                                                                        }).borrow().clone() }, 0f64, at_2)));
                                                                                                        {
                                                                                                            let source_ml_source1_concat_value17046 = std::rc::Rc::new(std::cell::RefCell::new(vec![object]));
                                                                                                            {
                                                                                                                let source_ml_source1_concat_value17047 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ (match heap.clone() {
                                                                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17044, _, _, _, _, _) => {
                                                                                                                        source_ml_source1_data_property_value17044.clone()
                                                                                                                    }
                                                                                                                }).borrow().clone() }, at_2, f64::INFINITY)));
                                                                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { source_ml_source1_concat_value17045.borrow().clone() }), { source_ml_source1_concat_value17046.borrow().clone() }), { source_ml_source1_concat_value17047.borrow().clone() })
                                                                                                            }
                                                                                                        }
                                                                                                    }));
                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_nominal_override17061, source_ml_source1_nominal_field17051, source_ml_source1_nominal_field17053, source_ml_source1_nominal_field17055, source_ml_source1_nominal_field17057, source_ml_source1_nominal_field17059)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            };
                                                                            {
                                                                                let problem = self::read_java_script_ordinary_heap_error(next.clone());
                                                                                {
                                                                                    let ml_library_inequality_1 = problem != (self::ml_string::Text::from_units(vec![])) ;
                                                                                    if ml_library_inequality_1 {
                                                                                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), problem.clone())
                                                                            } else {
                                                                                self::JavaScriptArrayCompletion::ArrayNormal(Box::new(next.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
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
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        self::JavaScriptArrayCompletion::ArrayError(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                            ]),
                            self::ml_string::Text::from_units(vec![
                                73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32,
                                108, 101, 110, 103, 116, 104,
                            ]),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_result_prototypes_error(
    heap: self::JavaScriptOrdinaryHeap,
    array_prototype: self::JavaScriptSemanticValue,
    object_prototype: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                error.clone()
            } else {
                {
                    let source_ml_temporary_values143 =
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            array_prototype.clone(),
                            object_prototype.clone(),
                        ]));
                    {
                        let source_ml_temporary_index143 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s17603 = self::source_ml_temporary_read_java_script_collection_iterator_result_prototypes_error_loop144(heap.clone(), source_ml_temporary_values143, source_ml_temporary_index143);
                            match source_ml_source1_s17603 {
                            self::MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Result::MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Done => {
                                match array_prototype.clone() {
                                    self::JavaScriptSemanticValue::Object(array_prototype_identity, array_prototype_callable) => {
                                        match object_prototype.clone() {
                                            self::JavaScriptSemanticValue::Object(identity, callable) => {
                                                if array_prototype_callable || callable {
                                                    self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115])
                                                } else {
                                                    {
                                                        let array = {
                                                            let source_ml_source1_array_read_value17620 = match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17618, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value17618.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_array_read_index17621 = self::find_java_script_ordinary_object(match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17619, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value17619.clone()
                                                                    }
                                                                }, array_prototype_identity.clone());
                                                                { let ml_reference_array218 = source_ml_source1_array_read_value17620; let ml_reference_position218 = source_ml_source1_array_read_index17621; self::ml_array::optional_at(&*ml_reference_array218.borrow(), self::ml_array::number_index(ml_reference_position218)) }
                                                            }
                                                        };
                                                        {
                                                            let ordinary = {
                                                                let source_ml_source1_array_read_value17624 = match heap.clone() {
                                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17622, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value17622.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_array_read_index17625 = self::find_java_script_ordinary_object(match heap.clone() {
                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17623, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value17623.clone()
                                                                        }
                                                                    }, identity.clone());
                                                                    { let ml_reference_array219 = source_ml_source1_array_read_value17624; let ml_reference_position219 = source_ml_source1_array_read_index17625; self::ml_array::optional_at(&*ml_reference_array219.borrow(), self::ml_array::number_index(ml_reference_position219)) }
                                                                }
                                                            };
                                                            if ((match (array.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match array.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }) || match (ordinary.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match ordinary.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            } {
                                                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 32, 107, 105, 110, 100, 115])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![])
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115])
                                            }
                                        }
                                    }
                                    _ => {
                                        self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115])
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Result::MlTemporaryReadJavaScriptCollectionIteratorResultPrototypesErrorLoop144Return(source_ml_temporary_result144) => {
                                source_ml_temporary_result144.clone()
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
pub fn read_java_script_collection_iterator_result_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptCollectionIteratorResultState,
) -> self::ml_string::Text {
    {
        let error = self::read_java_script_collection_iterator_result_prototypes_error(
            heap.clone(),
            match state.clone() {
                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value17816,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source1_data_property_value17816 =
                        *source_ml_source1_data_property_value17816;
                    source_ml_source1_data_property_value17816.clone()
                }
            },
            match state.clone() {
                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value17817,
                    _,
                    _,
                    _,
                ) => {
                    let source_ml_source1_data_property_value17817 =
                        *source_ml_source1_data_property_value17817;
                    source_ml_source1_data_property_value17817.clone()
                }
            },
        );
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
            error.clone()
        } else if (((match state.clone() {
                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, _, _) => {
                    false
                }
            } || !(self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121]), self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116])]))).borrow().clone() }, &match state.clone() {
                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(source_ml_source1_data_property_value17826, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value17826.clone()
                }
            }, 0f64, false, |a, b| a == b))) || (({
                let source_ml_source1_type_query_operand17830 = match state.clone() {
                    self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, _, source_ml_source1_data_property_value17829) => {
                        source_ml_source1_data_property_value17829
                    }
                };
                self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
            }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) || (((match state.clone() {
                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(source_ml_source1_data_property_value17832, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value17832.clone()
                }
            }) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121]))) && match state.clone() {
                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, _, source_ml_source1_data_property_value17833) => {
                    source_ml_source1_data_property_value17833
                }
            })) || !(self::ml_array::includes(&{ (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone() }, &match state.clone() {
                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, source_ml_source1_data_property_value17834, _, _, _, _, _) => {
                    source_ml_source1_data_property_value17834.clone()
                }
            }, 0f64, false, |a, b| a == b)) {
                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 112, 104, 97, 115, 101, 44, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110])
            } else {
                {
                    let source_ml_temporary_values145 = std::rc::Rc::new(std::cell::RefCell::new(vec![match state.clone() {
                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, source_ml_source1_data_property_value17835, _, _, _, _, _, _) => {
                            let source_ml_source1_data_property_value17835 = *source_ml_source1_data_property_value17835;
                            source_ml_source1_data_property_value17835.clone()
                        }
                    }, match state.clone() {
                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, source_ml_source1_data_property_value17836, _, _) => {
                            let source_ml_source1_data_property_value17836 = *source_ml_source1_data_property_value17836;
                            source_ml_source1_data_property_value17836.clone()
                        }
                    }, match state.clone() {
                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, source_ml_source1_data_property_value17837, _) => {
                            let source_ml_source1_data_property_value17837 = *source_ml_source1_data_property_value17837;
                            source_ml_source1_data_property_value17837.clone()
                        }
                    }]));
                    {
                        let source_ml_temporary_index145 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s17838 = self::source_ml_temporary_read_java_script_collection_iterator_result_state_error_loop146(heap.clone(), source_ml_temporary_values145, source_ml_temporary_index145);
                            match source_ml_source1_s17838 {
                                self::MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Result::MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Done => {
                                    {
                                        let receiver = match state.clone() {
                                            self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, source_ml_source1_data_property_value17839, _, _, _, _, _, _) => {
                                                let source_ml_source1_data_property_value17839 = *source_ml_source1_data_property_value17839;
                                                source_ml_source1_data_property_value17839.clone()
                                            }
                                        };
                                        match receiver {
                                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                                {
                                                    let iterator = {
                                                        let source_ml_source1_array_read_value17850 = match heap.clone() {
                                                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17848, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value17848.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_array_read_index17851 = self::find_java_script_ordinary_object(match heap.clone() {
                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value17849, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value17849.clone()
                                                                }
                                                            }, identity.clone());
                                                            { let ml_reference_array222 = source_ml_source1_array_read_value17850; let ml_reference_position222 = source_ml_source1_array_read_index17851; self::ml_array::optional_at(&*ml_reference_array222.borrow(), self::ml_array::number_index(ml_reference_position222)) }
                                                        }
                                                    };
                                                    if match (iterator.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::ml_string::Text::from_units(vec![97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100])
                                                    } else {
                                                        match iterator.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, collection_kind, _, _) => {
                                                                if collection_kind != (match state.clone() {
                                                                    self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, source_ml_source1_data_property_value17947, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value17947.clone()
                                                                    }
                                                                }) {
                                                                    self::ml_string::Text::from_units(vec![97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100])
                                                                } else if ((match state.clone() {
                                                                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, source_ml_source1_data_property_value17949, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value17949.clone()
                                                                        }
                                                                    }) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) && ((match state.clone() {
                                                                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(source_ml_source1_data_property_value17951, _, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value17951.clone()
                                                                        }
                                                                    }) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121]))) {
                                                                        self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 115, 32, 121, 105, 101, 108, 100, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101, 115, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 101, 110, 116, 114, 121, 32, 112, 97, 105, 114, 115])
                                                                    } else if match state.clone() {
                                                                            self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, _, source_ml_source1_data_property_value17952) => {
                                                                                source_ml_source1_data_property_value17952
                                                                            }
                                                                        } && ({
                                                                            let source_ml_source1_s17954 = match state.clone() {
                                                                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, source_ml_source1_data_property_value17964, _, _) => {
                                                                                    let source_ml_source1_data_property_value17964 = *source_ml_source1_data_property_value17964;
                                                                                    source_ml_source1_data_property_value17964.clone()
                                                                                }
                                                                            };
                                                                            match source_ml_source1_s17954 {
                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        } || {
                                                                            let source_ml_source1_s17966 = match state.clone() {
                                                                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, _, _, source_ml_source1_data_property_value17976, _) => {
                                                                                    let source_ml_source1_data_property_value17976 = *source_ml_source1_data_property_value17976;
                                                                                    source_ml_source1_data_property_value17976.clone()
                                                                                }
                                                                            };
                                                                            match source_ml_source1_s17966 {
                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        }) {
                                                                            self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 121, 105, 101, 108, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 118, 97, 108, 117, 101])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![])
                                                                        }
                                                            }
                                                            _ => {
                                                                self::ml_string::Text::from_units(vec![97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100])
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::ml_string::Text::from_units(vec![97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100])
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Result::MlTemporaryReadJavaScriptCollectionIteratorResultStateErrorLoop146Return(source_ml_temporary_result146) => {
                                    source_ml_temporary_result146.clone()
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
pub fn begin_java_script_collection_iterator_result(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    collection_kind: &self::ml_string::Text,
    array_prototype: self::JavaScriptSemanticValue,
    object_prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptCollectionIteratorResultCompletion {
    {
        let error = self::read_java_script_collection_iterator_result_prototypes_error(
            heap.clone(),
            array_prototype.clone(),
            object_prototype.clone(),
        );
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let next = self::advance_java_script_branded_collection_iterator(
                        heap.clone(),
                        receiver.clone(),
                        collection_kind,
                    );
                    match next {
                    self::JavaScriptCollectionIteration::CollectionIteration(source_ml_source1_f17977, source_ml_source1_f17978, source_ml_source1_f17979, source_ml_source1_f17980, source_ml_source1_f17981) => {
                        let source_ml_source1_f17977 = *source_ml_source1_f17977;
                        let source_ml_source1_f17979 = *source_ml_source1_f17979;
                        let source_ml_source1_f17980 = *source_ml_source1_f17980;
                        {
                            let state = self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(if source_ml_source1_f17981 {
                                self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121])
                            } else {
                                self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116])
                            }, Box::new(receiver.clone()), collection_kind.clone(), Box::new(array_prototype.clone()), Box::new(object_prototype.clone()), Box::new(source_ml_source1_f17979.clone()), Box::new(source_ml_source1_f17980.clone()), source_ml_source1_f17978);
                            self::JavaScriptCollectionIteratorResultCompletion::IteratorResultContinue(Box::new(source_ml_source1_f17977.clone()), Box::new(state))
                        }
                    }
                    self::JavaScriptCollectionIteration::CollectionIterationInvalid(_, _) => {
                        self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 99, 117, 114, 115, 111, 114, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110]))
                    }
                    self::JavaScriptCollectionIteration::CollectionIterationTypeError(_, reason) => {
                        self::JavaScriptCollectionIteratorResultCompletion::IteratorResultTypeError(Box::new(heap.clone()), reason.clone())
                    }
                    self::JavaScriptCollectionIteration::CollectionIterationResourceLimit(_, _) => {
                        self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 99, 117, 114, 115, 111, 114, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110]))
                    }
                    self::JavaScriptCollectionIteration::CollectionIteratorAllocated(_, _) => {
                        self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 99, 117, 114, 115, 111, 114, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110]))
                    }
                }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn begin_java_script_array_iterator_result(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    done: bool,
    key: self::JavaScriptSemanticValue,
    value: self::JavaScriptSemanticValue,
    entry: bool,
    array_prototype: self::JavaScriptSemanticValue,
    object_prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptCollectionIteratorResultCompletion {
    {
        let state = self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(
            if entry {
                self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121])
            } else {
                self::ml_string::Text::from_units(vec![114, 101, 115, 117, 108, 116])
            },
            Box::new(receiver.clone()),
            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]),
            Box::new(array_prototype.clone()),
            Box::new(object_prototype.clone()),
            Box::new(key.clone()),
            Box::new(value.clone()),
            done,
        );
        {
            let error = self::read_java_script_collection_iterator_result_state_error(
                heap.clone(),
                state.clone(),
            );
            if error == (self::ml_string::Text::from_units(vec![])) {
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_collection_iterator_result(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptCollectionIteratorResultState,
) -> self::JavaScriptCollectionIteratorResultCompletion {
    {
        let error = self::read_java_script_collection_iterator_result_state_error(
            heap.clone(),
            state.clone(),
        );
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                {
                    let properties = 0f64;
                    {
                        let source_ml_temporary_values147 = match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                source_ml_source1_data_property_value18021,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value18021.clone(),
                        };
                        {
                            let source_ml_temporary_index147 = self::ml::Big::from_i128(0);
                            {
                                let properties_2 = self::source_ml_temporary_step_java_script_collection_iterator_result_loop148(properties, source_ml_temporary_values147, source_ml_temporary_index147);
                                if (({ let ml_reference_array223 = match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value18023, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value18023.clone()
                                }
                            }; ml_reference_array223.borrow().len() as f64 }) >= 4096f64) || ((properties_2 + if (match state.clone() {
                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(source_ml_source1_data_property_value18027, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value18027.clone()
                                }
                            }) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121])) {
                                3f64
                            } else {
                                2f64
                            }) > 16_384f64) {
                                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 47, 112, 114, 111, 112, 101, 114, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                self::JavaScriptCollectionIteratorResultCompletion::IteratorResultAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new(if (match state.clone() {
                                    self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(source_ml_source1_data_property_value18029, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18029.clone()
                                    }
                                }) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121])) {
                                    match state.clone() {
                                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, source_ml_source1_data_property_value18030, _, _, _, _) => {
                                            let source_ml_source1_data_property_value18030 = *source_ml_source1_data_property_value18030;
                                            source_ml_source1_data_property_value18030.clone()
                                        }
                                    }
                                } else {
                                    match state.clone() {
                                        self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, _, _, _, source_ml_source1_data_property_value18031, _, _, _) => {
                                            let source_ml_source1_data_property_value18031 = *source_ml_source1_data_property_value18031;
                                            source_ml_source1_data_property_value18031.clone()
                                        }
                                    }
                                }), if (match state.clone() {
                                    self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(source_ml_source1_data_property_value18033, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value18033.clone()
                                    }
                                }) == (self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121])) {
                                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])
                                } else {
                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121])
                                })
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
pub fn create_java_script_iterator_result_descriptor(
    value: self::JavaScriptSemanticValue,
) -> std::rc::Rc<
    std::cell::RefCell<(
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    )>,
> {
    {
        let descriptor = self::create_java_script_property_descriptor(
            &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
            true,
            true,
        );
        {
            let source_ml_temporary_create_java_script_iterator_result_descriptor_written149 = {
                let ml_field_owner224 = descriptor.clone();
                let ml_field_value224 = value.clone();
                ml_field_owner224.borrow_mut().8 = ml_field_value224;
                ()
            };
            {
                let source_ml_temporary_create_java_script_iterator_result_descriptor_written150 = {
                    let ml_field_owner225 = descriptor.clone();
                    let ml_field_value225 = true;
                    ml_field_owner225.borrow_mut().10 = ml_field_value225;
                    ()
                };
                descriptor.clone()
            }
        }
    }
}
