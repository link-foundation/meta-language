// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorProtocolTransition;
use super::JavaScriptIteratorSelectedCall;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptPrimitiveWrapperTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop389Result;
use super::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop390Result;
use super::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesJoin395;
use super::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesLoop396Result;
use super::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386;
use super::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Result;
use super::continue_java_script_iterator_protocol_value;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_iterator_protocol_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::request_java_script_iterator_protocol_call;
use super::source_ml_source1_temporary_construct_java_script_wrapper_index_key_loop389;
use super::source_ml_source1_temporary_construct_java_script_wrapper_index_key_loop390;
use super::source_ml_source1_temporary_construct_java_script_wrapper_own_properties_loop396;
use super::source_ml_source1_temporary_read_java_script_primitive_wrapper_state_error_loop385;
use super::source_ml_source1_temporary_read_java_script_primitive_wrapper_work_error_loop388;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn provide_java_script_iterator_primitive_prototype(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptIteratorProtocolState,
    prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptIteratorProtocolTransition {
    match state.clone() {
        self::JavaScriptIteratorProtocolState::IteratorProtocol(
            _,
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            {
                let error = self::read_java_script_iterator_protocol_state_error(
                    heap.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if (((((shared_constructor_field1
                        != (self::ml_string::Text::from_units(vec![
                            109, 101, 116, 104, 111, 100,
                        ])))
                        || {
                            let source_ml_source2_s13718 = shared_constructor_field2.clone();
                            match source_ml_source2_s13718.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => true,
                                _ => false,
                            }
                        })
                        || {
                            let source_ml_source2_s13723 = shared_constructor_field2.clone();
                            match source_ml_source2_s13723.clone() {
                                self::JavaScriptSemanticValue::Null => true,
                                _ => false,
                            }
                        })
                        || {
                            let source_ml_source2_s13726 = shared_constructor_field2.clone();
                            match source_ml_source2_s13726.clone() {
                                self::JavaScriptSemanticValue::Undefined => true,
                                _ => false,
                            }
                        })
                        || !(match (
                            match state.clone() {
                                self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value13728,
                                ) => ((source_ml_source2_data_property_value13728
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptIteratorSelectedCall>,)
                                    }))
                                .0)
                                    .clone(),
                            },
                            None::<()>,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }))
                    {
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 116, 101,
                                114, 97, 116, 111, 114, 32, 112, 114, 111, 116, 111, 116, 121, 112,
                                101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103,
                                32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116,
                                104, 111, 100, 32, 71, 101, 116,
                            ]),
                        )
                    } else {
                        {
                            let invalid = self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                prototype.clone(),
                            );
                            if ((invalid != (self::ml_string::Text::from_units(vec![])))
                                || match prototype.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => false,
                                    _ => true,
                                })
                            {
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                                    Box::new(heap.clone()),
                                    if !invalid.0.is_empty() {
                                        invalid.clone()
                                    } else {
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105,
                                            116, 101, 114, 97, 116, 111, 114, 32, 71, 101, 116, 32,
                                            114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                                            32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108,
                                            109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101,
                                        ])
                                    },
                                )
                            } else {
                                {
                                    let get = self::plan_java_script_ordinary_get(
                                        heap.clone(),
                                        (((match prototype.clone() {
                                            self::JavaScriptSemanticValue::Object(
                                                source_ml_source2_data_property_value13732,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value13732
                                                    .clone(),))
                                            }
                                            _ => None::<(self::ml::Big,)>,
                                        })
                                        .clone()
                                        .expect("a proven present value"))
                                        .0)
                                            .clone(),
                                        shared_constructor_field3.clone(),
                                        shared_constructor_field2.clone(),
                                    );
                                    if (((({
                                        let ml_reference_value796 = get.borrow().clone();
                                        ml_reference_value796
                                    })
                                    .0)
                                        .3)
                                        != (self::ml_string::Text::from_units(vec![])))
                                    {
                                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(Box::new(heap.clone()), (({ let ml_reference_value797 = get.borrow().clone(); ml_reference_value797 }).0).3)
                                    } else {
                                        if (((({
                                            let ml_reference_value798 = get.borrow().clone();
                                            ml_reference_value798
                                        })
                                        .0)
                                            .0)
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 97, 108, 108,
                                            ])))
                                        {
                                            self::request_java_script_iterator_protocol_call(
                                                heap.clone(),
                                                state.clone(),
                                                (({
                                                    let ml_reference_value799 =
                                                        get.borrow().clone();
                                                    ml_reference_value799
                                                })
                                                .0)
                                                    .2,
                                                (({
                                                    let ml_reference_value800 =
                                                        get.borrow().clone();
                                                    ml_reference_value800
                                                })
                                                .0)
                                                    .4,
                                                &(self::ml_string::Text::from_units(vec![
                                                    109, 101, 116, 104, 111, 100, 32, 103, 101,
                                                    116, 116, 101, 114,
                                                ])),
                                            )
                                        } else {
                                            if (((({
                                                let ml_reference_value801 = get.borrow().clone();
                                                ml_reference_value801
                                            })
                                            .0)
                                                .0)
                                                == (self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101,
                                                ])))
                                            {
                                                self::continue_java_script_iterator_protocol_value(
                                                    heap.clone(),
                                                    state.clone(),
                                                    (({
                                                        let ml_reference_value802 =
                                                            get.borrow().clone();
                                                        ml_reference_value802
                                                    })
                                                    .0)
                                                        .5,
                                                )
                                            } else {
                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_wrapper_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveWrapperState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(
            shared_constructor_field2,
            shared_constructor_field3,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let invalid = self::read_java_script_ordinary_heap_error(heap.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        invalid.clone()
                    } else {
                        if (match state.clone() {
                            self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(_, _) => {
                                false
                            }
                        } || {
                            let source_ml_source2_s13738 = shared_constructor_field2.clone();
                            match source_ml_source2_s13738.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => true,
                                _ => false,
                            }
                        }) {
                            self::ml_string::Text::from_units(vec![
                                119, 114, 97, 112, 112, 101, 114, 32, 97, 108, 108, 111, 99, 97,
                                116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97,
                                110, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105,
                                116, 105, 118, 101, 32, 111, 114, 32, 110, 117, 108, 108, 105, 115,
                                104, 32, 79, 98, 106, 101, 99, 116, 32, 67, 97, 108, 108, 32, 105,
                                110, 112, 117, 116,
                            ])
                        } else {
                            {
                                let source_ml_source1_temporary_values384 =
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        shared_constructor_field2.clone(),
                                        shared_constructor_field3.clone(),
                                    ]));
                                {
                                    let source_ml_source1_temporary_index384 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s13744 = self::source_ml_source1_temporary_read_java_script_primitive_wrapper_state_error_loop385(heap.clone(), source_ml_source1_temporary_values384.clone(), source_ml_source1_temporary_index384.clone());
                                        match source_ml_source2_s13744.clone() {
                                                self::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Result::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Done => {
                                                    if ({
                                                        let source_ml_source2_s13746 = shared_constructor_field3.clone();
                                                        match source_ml_source2_s13746.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    } && {
                                                        let source_ml_source2_s13751 = shared_constructor_field3.clone();
                                                        match source_ml_source2_s13751.clone() {
                                                            self::JavaScriptSemanticValue::Null => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 114, 101, 97, 108, 109, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                    } else {
                                                        {
                                                            let source_ml_source2_s13764 = if {
                                                                let source_ml_source2_s13766 = shared_constructor_field3.clone();
                                                                match source_ml_source2_s13766.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                }
                                                            } {
                                                                {
                                                                    let parent = {
                                                                        let source_ml_source2_array_read_value13774 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index13775 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field3.clone() {
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value13773, _) => {
                                                                                    Some((source_ml_source2_data_property_value13773.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(self::ml::Big,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone());
                                                                            { let ml_reference_array803 = source_ml_source2_array_read_value13774.clone(); let ml_reference_position803 = source_ml_source2_array_read_index13775; let ml_reference_projection803 = self::ml_array::optional_at(&*ml_reference_array803.borrow(), self::ml_array::number_index(ml_reference_position803)); ml_reference_projection803 }
                                                                        }
                                                                    };
                                                                    if (!(match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(self::ml_array::includes(&{ let ml_reference_value804 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])]))).borrow().clone(); ml_reference_value804 }, &match parent.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                            self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                                        }
                                                                    }, 0f64, false, |a, b| a == b))) {
                                                                        self::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386Return(self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 119, 105, 116, 104, 32, 97, 110, 32, 111, 112, 97, 113, 117, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386Next
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386Next
                                                            };
                                                            match source_ml_source2_s13764.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386Next => {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorJoin386Return(source_ml_source1_temporary_result386) => {
                                                                    source_ml_source1_temporary_result386.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Result::MlSource1TemporaryReadJavaScriptPrimitiveWrapperStateErrorLoop385Return(source_ml_source1_temporary_result386_2) => {
                                                    source_ml_source1_temporary_result386_2.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_wrapper_work_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveWrapperState,
) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            if (({
                let ml_reference_array805 = shared_constructor_field1.clone();
                let ml_reference_projection805 = ml_reference_array805.borrow().len() as f64;
                ml_reference_projection805
            }) >= (4096f64))
            {
                self::ml_string::Text::from_units(vec![
                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101,
                    114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101,
                    101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100,
                    32, 52, 48, 57, 54, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116,
                    105, 116, 121, 32, 112, 114, 111, 102, 105, 108, 101,
                ])
            } else {
                {
                    let length = self::read_java_script_semantic_text_length(match state.clone() {
                        self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(
                            source_ml_source2_data_property_value13778,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value13778 =
                                *source_ml_source2_data_property_value13778;
                            source_ml_source2_data_property_value13778.clone()
                        }
                    });
                    {
                        let properties = if (length >= (0f64)) {
                            (length + 1f64)
                        } else {
                            0f64
                        };
                        if (properties > (4096f64)) {
                            self::ml_string::Text::from_units(vec![
                                83, 116, 114, 105, 110, 103, 32, 119, 114, 97, 112, 112, 101, 114,
                                32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120,
                                99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101,
                                99, 116, 101, 100, 32, 52, 48, 57, 54, 32, 111, 119, 110, 45, 112,
                                114, 111, 112, 101, 114, 116, 121, 32, 112, 114, 111, 102, 105,
                                108, 101,
                            ])
                        } else {
                            {
                                let total = properties;
                                {
                                    let source_ml_source1_temporary_values387 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source1_temporary_index387 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let total_2 = self::source_ml_source1_temporary_read_java_script_primitive_wrapper_work_error_loop388(total, source_ml_source1_temporary_values387.clone(), source_ml_source1_temporary_index387.clone());
                                            if (total_2 > (16384f64)) {
                                                self::ml_string::Text::from_units(vec![
                                                    112, 114, 105, 109, 105, 116, 105, 118, 101,
                                                    32, 119, 114, 97, 112, 112, 101, 114, 32, 97,
                                                    108, 108, 111, 99, 97, 116, 105, 111, 110, 32,
                                                    101, 120, 99, 101, 101, 100, 115, 32, 116, 104,
                                                    101, 32, 115, 101, 108, 101, 99, 116, 101, 100,
                                                    32, 49, 54, 51, 56, 52, 32, 115, 104, 97, 114,
                                                    101, 100, 32, 111, 119, 110, 45, 112, 114, 111,
                                                    112, 101, 114, 116, 121, 32, 112, 114, 111,
                                                    102, 105, 108, 101,
                                                ])
                                            } else {
                                                self::ml_string::Text::from_units(vec![])
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_primitive_wrapper(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptPrimitiveWrapperTransition {
    {
        let invalid = {
            let source_ml_source2_logical13780 =
                self::read_java_script_ordinary_heap_error(heap.clone());
            if !source_ml_source2_logical13780.0.is_empty() {
                source_ml_source2_logical13780.clone()
            } else {
                self::read_java_script_ordinary_value_error(heap.clone(), input.clone())
            }
        };
        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(
                Box::new(heap.clone()),
                invalid.clone(),
            )
        } else {
            match input.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => {
                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperCompleted(
                        Box::new(heap.clone()),
                        Box::new(input.clone()),
                    )
                }
                _ => {
                    let state = self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(
                        Box::new(input.clone()),
                        Box::new(prototype.clone()),
                    );
                    {
                        let error = self::read_java_script_primitive_wrapper_state_error(
                            heap.clone(),
                            state.clone(),
                        );
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(
                                Box::new(heap.clone()),
                                error.clone(),
                            )
                        } else {
                            {
                                let work = self::read_java_script_primitive_wrapper_work_error(
                                    heap.clone(),
                                    state.clone(),
                                );
                                if (work != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), work.clone())
                                } else {
                                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new(prototype.clone()))
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
pub fn construct_java_script_wrapper_index_key(index: f64) -> self::JavaScriptSemanticValue {
    {
        let reversed = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let source_ml_source2_s13783 = self::source_ml_source1_temporary_construct_java_script_wrapper_index_key_loop389(reversed.clone(), index);
                match source_ml_source2_s13783.clone() {
                    self::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop389Result::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop389Done => {
                        {
                            let at = ({ let ml_reference_array806 = reversed.clone(); let ml_reference_projection806 = ml_reference_array806.borrow().len() as f64; ml_reference_projection806 } - 1f64);
                            {
                                let source_ml_source2_s13784 = self::source_ml_source1_temporary_construct_java_script_wrapper_index_key_loop390(reversed.clone(), units.clone(), at);
                                match source_ml_source2_s13784.clone() {
                                    self::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop390Result::MlSource1TemporaryConstructJavaScriptWrapperIndexKeyLoop390Done => {
                                        self::JavaScriptSemanticValue::Text(units.clone())
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_wrapper_string_property(
    key: self::JavaScriptSemanticValue,
    value: self::JavaScriptSemanticValue,
    enumerable: bool,
) -> self::JavaScriptOrdinaryProperty {
    {
        let descriptor = self::create_java_script_property_descriptor(
            &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
            false,
            enumerable,
        );
        {
            let source_ml_source1_temporary_construct_java_script_wrapper_string_property_written393 = {
                let ml_field_owner807 = descriptor.clone();
                let ml_field_value807 = value.clone();
                ml_field_owner807.borrow_mut().0.8 = ml_field_value807;
                ()
            };
            {
                let source_ml_source1_temporary_construct_java_script_wrapper_string_property_written394 = {
                    let ml_field_owner808 = descriptor.clone();
                    let ml_field_value808 = false;
                    ml_field_owner808.borrow_mut().0.10 = ml_field_value808;
                    ()
                };
                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                    Box::new(key.clone()),
                    descriptor.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_wrapper_own_properties(
    input: self::JavaScriptSemanticValue,
    length: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>> {
    {
        let properties = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
            self::JavaScriptOrdinaryProperty,
        >::new()));
        {
            let source_ml_source2_s13786 = if (length >= (0f64)) {
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s13787 = self::source_ml_source1_temporary_construct_java_script_wrapper_own_properties_loop396(input.clone(), length, properties.clone(), index);
                        match source_ml_source2_s13787.clone() {
                            self::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesLoop396Result::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesLoop396Done => {
                                {
                                    let source_ml_source1_temporary_construct_java_script_wrapper_own_properties_ignored397 = { let ml_method_owner809 = properties.clone(); let ml_method_argument809_0 = self::construct_java_script_wrapper_string_property(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64]))), self::JavaScriptSemanticValue::Number(length), false); let ml_inserted = vec![ml_method_argument809_0]; let mut ml_method_values809 = ml_method_owner809.borrow_mut(); let ml_length = ml_method_values809.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values809.extend(ml_inserted); ml_length as f64 };
                                    self::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesJoin395::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesJoin395Next
                                }
                            }
                        }
                    }
                }
            } else {
                self::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesJoin395::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesJoin395Next
            };
            match source_ml_source2_s13786.clone() {
                self::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesJoin395::MlSource1TemporaryConstructJavaScriptWrapperOwnPropertiesJoin395Next => {
                    properties.clone()
                }
            }
        }
    }
}
