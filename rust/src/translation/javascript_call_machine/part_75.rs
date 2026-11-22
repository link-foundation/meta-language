// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayCreationCall;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorSelectedCall;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403;
use super::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404;
use super::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405;
use super::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406;
use super::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407;
use super::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_assignment_state_error;
use super::read_java_script_array_creation_callback_error;
use super::read_java_script_iterator_protocol_state_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::read_java_script_synchronous_iterator_record_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_creation_child_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArrayCreationState::ArrayCreationState(
            _,
            _,
            _,
            _,
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            _,
            _,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            _,
            shared_constructor_field9,
            _,
            shared_constructor_field10,
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
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            {
                let source_ml_source2_s14115 = if !(match (
                    match state.clone() {
                        self::JavaScriptArrayCreationState::ArrayCreationState(
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
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value14116,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value14116
                            .clone()
                            .unwrap_or_else(|| {
                                (None::<self::JavaScriptSynchronousIteratorRecord>,)
                            }))
                        .0)
                            .clone(),
                    },
                    None::<()>,
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) {
                    {
                        let invalid = self::read_java_script_synchronous_iterator_record_error(
                            heap.clone(),
                            (match state.clone() {
                                self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value14117,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value14117
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptSynchronousIteratorRecord>,)
                                    }))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value"),
                        );
                        if ((invalid != (self::ml_string::Text::from_units(vec![])))
                            || !shared_constructor_field4)
                        {
                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403Return(if !invalid.0.is_empty() {
                                invalid.clone()
                            } else {
                                self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 111, 119, 110, 115, 32, 97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 99, 111, 114, 100])
                            })
                        } else {
                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403Next
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403Next
                };
                match source_ml_source2_s14115.clone() {
                    self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403Next => {
                        {
                            let source_ml_source2_s14179 = if !(match (match state.clone() {
                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14180, _, _, _, _, _, _, _, _) => {
                                    ((source_ml_source2_data_property_value14180.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                }
                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                {
                                    let invalid_2 = self::read_java_script_iterator_protocol_state_error(heap.clone(), (match state.clone() {
                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14181, _, _, _, _, _, _, _, _) => {
                                            ((source_ml_source2_data_property_value14181.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value"));
                                    if (((invalid_2 != (self::ml_string::Text::from_units(vec![]))) || !(self::ml_array::includes(&{ let ml_reference_value831 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 101]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])]))).borrow().clone(); ml_reference_value831 }, &shared_constructor_field5.clone(), 0f64, false, |a, b| a == b))) || ((match (match state.clone() {
                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14189, _, _, _, _, _, _, _, _) => {
                                            ((source_ml_source2_data_property_value14189.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value") {
                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_data_property_value14190, _, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value14190.clone()
                                        }
                                    }) != (if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110]))) {
                                        self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101])
                                    } else {
                                        if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 101]))) {
                                            self::ml_string::Text::from_units(vec![115, 116, 101, 112])
                                        } else {
                                            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])
                                        }
                                    }))) {
                                        self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404Return(if !invalid_2.0.is_empty() {
                                            invalid_2.clone()
                                        } else {
                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108])
                                        })
                                    } else {
                                        if ((((shared_constructor_field5 == (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110]))) && ((match (match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14200, _, _, _, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value14200.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value") {
                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, source_ml_source2_data_property_value14201, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value14201.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108])))) && !(match (match (match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14202, _, _, _, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value14202.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value") {
                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14203) => {
                                                ((source_ml_source2_data_property_value14203.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) && !(self::test_java_script_semantic_same_value(match (match (match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14204, _, _, _, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value14204.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value") {
                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value14205) => {
                                                ((source_ml_source2_data_property_value14205.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value") {
                                            self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(source_ml_source2_data_property_value14206, _, _) => {
                                                let source_ml_source2_data_property_value14206 = *source_ml_source2_data_property_value14206;
                                                source_ml_source2_data_property_value14206.clone()
                                            }
                                        }, shared_constructor_field8.clone()))) {
                                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 99, 97, 108, 108, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 101, 118, 105, 111, 117, 115, 108, 121, 32, 99, 97, 99, 104, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 109, 101, 116, 104, 111, 100]))
                                        } else {
                                            if ((shared_constructor_field5 == (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110]))) && (!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14210, _, _, _, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value14210.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value") {
                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, source_ml_source2_data_property_value14211, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value14211 = *source_ml_source2_data_property_value14211;
                                                    source_ml_source2_data_property_value14211.clone()
                                                }
                                            }, shared_constructor_field3.clone())) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14213, _, _, _, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value14213.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value") {
                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, source_ml_source2_data_property_value14214, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value14214 = *source_ml_source2_data_property_value14214;
                                                    source_ml_source2_data_property_value14214.clone()
                                                }
                                            }, shared_constructor_field1.clone())))) {
                                                self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 99, 97, 99, 104, 101, 100, 32, 109, 101, 116, 104, 111, 100]))
                                            } else {
                                                if ((shared_constructor_field5 != (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110]))) && (((match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14218, _, _, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14218.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                    }
                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(match match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14219, _, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14219.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value14220, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value14220 = *source_ml_source2_data_property_value14220;
                                                        source_ml_source2_data_property_value14220.clone()
                                                    }
                                                } {
                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value14221, _, _) => {
                                                        let source_ml_source2_data_property_value14221 = *source_ml_source2_data_property_value14221;
                                                        source_ml_source2_data_property_value14221.clone()
                                                    }
                                                }, match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14222, _, _, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14222.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value14223, _, _) => {
                                                        let source_ml_source2_data_property_value14223 = *source_ml_source2_data_property_value14223;
                                                        source_ml_source2_data_property_value14223.clone()
                                                    }
                                                }))) || !(self::test_java_script_semantic_same_value(match match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14224, _, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14224.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value14225, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value14225 = *source_ml_source2_data_property_value14225;
                                                        source_ml_source2_data_property_value14225.clone()
                                                    }
                                                } {
                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value14226, _) => {
                                                        let source_ml_source2_data_property_value14226 = *source_ml_source2_data_property_value14226;
                                                        source_ml_source2_data_property_value14226.clone()
                                                    }
                                                }, match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14227, _, _, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14227.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value14228, _) => {
                                                        let source_ml_source2_data_property_value14228 = *source_ml_source2_data_property_value14228;
                                                        source_ml_source2_data_property_value14228.clone()
                                                    }
                                                }))) || ((match match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14234, _, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14234.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value14235, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value14235 = *source_ml_source2_data_property_value14235;
                                                        source_ml_source2_data_property_value14235.clone()
                                                    }
                                                } {
                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value14236) => {
                                                        source_ml_source2_data_property_value14236
                                                    }
                                                }) != (match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14237, _, _, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14237.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value14238) => {
                                                        source_ml_source2_data_property_value14238
                                                    }
                                                })))) {
                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 99, 104, 101, 100, 32, 110, 101, 120, 116, 32, 109, 101, 116, 104, 111, 100, 32, 97, 110, 100, 32, 100, 111, 110, 101, 32, 102, 108, 97, 103]))
                                                } else {
                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404Next
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404Next
                            };
                            match source_ml_source2_s14179.clone() {
                                self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404Next => {
                                    {
                                        let source_ml_source2_s14254 = if !(match (match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14255, _, _, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value14255.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                            {
                                                let invalid_3 = self::read_java_script_primitive_conversion_state_error(heap.clone(), (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14256, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14256.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value"));
                                                if (((((invalid_3 != (self::ml_string::Text::from_units(vec![]))) || (shared_constructor_field5 != (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])))) || ((match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14261, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14261.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value14262, _) => {
                                                        source_ml_source2_data_property_value14262.clone()
                                                    }
                                                }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14263, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14263.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value14264, _, _, _) => {
                                                        let source_ml_source2_data_property_value14264 = *source_ml_source2_data_property_value14264;
                                                        source_ml_source2_data_property_value14264.clone()
                                                    }
                                                }, shared_constructor_field10.clone()))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14266, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14266.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value14267, _, _) => {
                                                        let source_ml_source2_data_property_value14267 = *source_ml_source2_data_property_value14267;
                                                        source_ml_source2_data_property_value14267.clone()
                                                    }
                                                }, shared_constructor_field2.clone()))) {
                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405Return(if !invalid_3.0.is_empty() {
                                                        invalid_3.clone()
                                                    } else {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 110, 117, 109, 98, 101, 114, 32, 104, 105, 110, 116, 32, 97, 110, 100, 32, 83, 121, 109, 98, 111, 108])
                                                    })
                                                } else {
                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405Next
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405Next
                                        };
                                        match source_ml_source2_s14254.clone() {
                                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405Next => {
                                                if (!(match (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14270, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14270.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (((shared_constructor_field5 != (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) || {
                                                    let source_ml_source2_s14274 = (match state.clone() {
                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14277, _, _, _, _, _, _) => {
                                                            ((source_ml_source2_data_property_value14277.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    }).clone().expect("a proven present value");
                                                    match source_ml_source2_s14274.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    }
                                                }) || ((self::read_java_script_ordinary_value_error(heap.clone(), (match state.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14279, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value14279.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![]))))) {
                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 115, 117, 115, 112, 101, 110, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 108, 101, 110, 103, 116, 104])
                                                } else {
                                                    if (((shared_constructor_field5 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && match (match state.clone() {
                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14282, _, _, _, _, _, _, _) => {
                                                            ((source_ml_source2_data_property_value14282.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                        }
                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && match (match state.clone() {
                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14283, _, _, _, _, _, _) => {
                                                            ((source_ml_source2_data_property_value14283.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 108, 111, 115, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                                    } else {
                                                        {
                                                            let source_ml_source2_s14323 = if !(match (match state.clone() {
                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14324, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value14324.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                {
                                                                    let invalid_4 = self::read_java_script_array_assignment_state_error(heap.clone(), (match state.clone() {
                                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14325, _, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value14325.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value"));
                                                                    if ((((((invalid_4 != (self::ml_string::Text::from_units(vec![]))) || (shared_constructor_field5 != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])))) || {
                                                                        let source_ml_source2_s14329 = shared_constructor_field9.clone();
                                                                        match source_ml_source2_s14329.clone() {
                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                false
                                                                            }
                                                                            _ => {
                                                                                true
                                                                            }
                                                                        }
                                                                    }) || ((match match (match state.clone() {
                                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14340, _, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value14340.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value") {
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(source_ml_source2_data_property_value14341, _) => {
                                                                            let source_ml_source2_data_property_value14341 = *source_ml_source2_data_property_value14341;
                                                                            source_ml_source2_data_property_value14341.clone()
                                                                        }
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(source_ml_source2_data_property_value14342, _, _) => {
                                                                            let source_ml_source2_data_property_value14342 = *source_ml_source2_data_property_value14342;
                                                                            source_ml_source2_data_property_value14342.clone()
                                                                        }
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(source_ml_source2_data_property_value14343, _, _) => {
                                                                            let source_ml_source2_data_property_value14343 = *source_ml_source2_data_property_value14343;
                                                                            source_ml_source2_data_property_value14343.clone()
                                                                        }
                                                                    } {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(source_ml_source2_data_property_value14344, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value14344.clone()
                                                                        }
                                                                    }) != ((((match shared_constructor_field9.clone() {
                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value14346, _) => {
                                                                            Some((source_ml_source2_data_property_value14346.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::ml::Big,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone()))) || !(self::test_java_script_semantic_same_value(match match (match state.clone() {
                                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14347, _, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value14347.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value") {
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(source_ml_source2_data_property_value14348, _) => {
                                                                            let source_ml_source2_data_property_value14348 = *source_ml_source2_data_property_value14348;
                                                                            source_ml_source2_data_property_value14348.clone()
                                                                        }
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(source_ml_source2_data_property_value14349, _, _) => {
                                                                            let source_ml_source2_data_property_value14349 = *source_ml_source2_data_property_value14349;
                                                                            source_ml_source2_data_property_value14349.clone()
                                                                        }
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(source_ml_source2_data_property_value14350, _, _) => {
                                                                            let source_ml_source2_data_property_value14350 = *source_ml_source2_data_property_value14350;
                                                                            source_ml_source2_data_property_value14350.clone()
                                                                        }
                                                                    } {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, source_ml_source2_data_property_value14351, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value14351 = *source_ml_source2_data_property_value14351;
                                                                            source_ml_source2_data_property_value14351.clone()
                                                                        }
                                                                    }, self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64])))))) || !(self::test_java_script_semantic_same_value((({ let ml_reference_value832 = (match match (match state.clone() {
                                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14352, _, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value14352.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value") {
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(source_ml_source2_data_property_value14353, _) => {
                                                                            let source_ml_source2_data_property_value14353 = *source_ml_source2_data_property_value14353;
                                                                            source_ml_source2_data_property_value14353.clone()
                                                                        }
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(source_ml_source2_data_property_value14354, _, _) => {
                                                                            let source_ml_source2_data_property_value14354 = *source_ml_source2_data_property_value14354;
                                                                            source_ml_source2_data_property_value14354.clone()
                                                                        }
                                                                        self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(source_ml_source2_data_property_value14355, _, _) => {
                                                                            let source_ml_source2_data_property_value14355 = *source_ml_source2_data_property_value14355;
                                                                            source_ml_source2_data_property_value14355.clone()
                                                                        }
                                                                    } {
                                                                        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value14356, _, _, _) => {
                                                                            source_ml_source2_data_property_value14356.clone()
                                                                        }
                                                                    }).borrow().clone(); ml_reference_value832 }).0).8, self::JavaScriptSemanticValue::Number(if shared_constructor_field4 {
                                                                        shared_constructor_field6
                                                                    } else {
                                                                        shared_constructor_field7
                                                                    })))) {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406Return(if !invalid_4.0.is_empty() {
                                                                            invalid_4.clone()
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 102, 105, 110, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 83, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])
                                                                        })
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406Next
                                                                    }
                                                                }
                                                            } else {
                                                                if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]))) {
                                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 83, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 115, 116, 101, 100, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406Next
                                                                }
                                                            };
                                                            match source_ml_source2_s14323.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406Next => {
                                                                    {
                                                                        let source_ml_source2_s14385 = if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                                                                            if ((((((match (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14388, _, _) => {
                                                                                    ((source_ml_source2_data_property_value14388.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14389, _, _, _, _, _, _, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value14389.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14390, _, _, _, _, _, _, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value14390.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value") {
                                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value14391) => {
                                                                                    source_ml_source2_data_property_value14391
                                                                                }
                                                                            }) || match (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14392, _, _, _, _, _, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value14392.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || ((match (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14395, _, _, _, _, _, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value14395.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value") {
                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_data_property_value14396, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value14396.clone()
                                                                                }
                                                                            }) != (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])))) || ((match match (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14400, _, _, _, _, _, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value14400.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value") {
                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, source_ml_source2_data_property_value14401, _, _) => {
                                                                                    let source_ml_source2_data_property_value14401 = *source_ml_source2_data_property_value14401;
                                                                                    source_ml_source2_data_property_value14401.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptIteratorCompletion::IteratorCompletion(source_ml_source2_data_property_value14402, _, _) => {
                                                                                    source_ml_source2_data_property_value14402.clone()
                                                                                }
                                                                            }) != (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119])))) || !(self::test_java_script_semantic_same_value(match match (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14403, _, _, _, _, _, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value14403.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value") {
                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, source_ml_source2_data_property_value14404, _, _) => {
                                                                                    let source_ml_source2_data_property_value14404 = *source_ml_source2_data_property_value14404;
                                                                                    source_ml_source2_data_property_value14404.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptIteratorCompletion::IteratorCompletion(_, source_ml_source2_data_property_value14405, _) => {
                                                                                    let source_ml_source2_data_property_value14405 = *source_ml_source2_data_property_value14405;
                                                                                    source_ml_source2_data_property_value14405.clone()
                                                                                }
                                                                            }, (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14406, _, _) => {
                                                                                    ((source_ml_source2_data_property_value14406.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value")))) {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 99, 108, 101, 97, 110, 117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 116, 104, 114, 111, 119, 32, 97, 110, 100, 32, 117, 110, 102, 105, 110, 105, 115, 104, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114]))
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407Next
                                                                            }
                                                                        } else {
                                                                            if !(match (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14407, _, _) => {
                                                                                    ((source_ml_source2_data_property_value14407.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 111, 108, 100, 115, 32, 97, 110, 32, 97, 98, 114, 117, 112, 116, 32, 118, 97, 108, 117, 101, 32, 111, 110, 108, 121, 32, 119, 104, 105, 108, 101, 32, 99, 108, 111, 115, 105, 110, 103, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 116, 101, 114, 97, 116, 111, 114]))
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407Next
                                                                            }
                                                                        };
                                                                        match source_ml_source2_s14385.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407Next => {
                                                                                {
                                                                                    let source_ml_source2_s14428 = if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))) {
                                                                                        if ((((((match (match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14431, _, _, _, _, _, _, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value14431.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match (match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14432, _, _, _, _, _, _, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value14432.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value14433) => {
                                                                                                source_ml_source2_data_property_value14433
                                                                                            }
                                                                                        }) || !(match (match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14434, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value14434.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || !(match (match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14435, _, _, _, _, _, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value14435.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || ((match match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14437, _) => {
                                                                                                ((source_ml_source2_data_property_value14437.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                            }
                                                                                        } { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || {
                                                                                            let source_ml_source2_comparison_operand14439 = match state.clone() {
                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14438, _) => {
                                                                                                    ((source_ml_source2_data_property_value14438.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                }
                                                                                            };
                                                                                            match (source_ml_source2_comparison_operand14439.clone(), self::ml_string::Text::from_units(vec![])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                        }) || ((match match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14442) => {
                                                                                                ((source_ml_source2_data_property_value14442.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                            }
                                                                                        } { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                                                                                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 97, 108, 108, 111, 99, 97, 116, 101, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 98, 101, 102, 111, 114, 101, 32, 99, 108, 111, 115, 105, 110, 103, 32, 116, 104, 101, 32, 117, 110, 102, 105, 110, 105, 115, 104, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114]))
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408Next
                                                                                        }
                                                                                    } else {
                                                                                        if ({
                                                                                            let source_ml_source2_comparison_operand14444 = match state.clone() {
                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14443, _) => {
                                                                                                    ((source_ml_source2_data_property_value14443.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_comparison_operand14445 = None::<()>;
                                                                                                !(match (source_ml_source2_comparison_operand14444.clone(), source_ml_source2_comparison_operand14445.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                            }
                                                                                        } || {
                                                                                            let source_ml_source2_comparison_operand14447 = match state.clone() {
                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14446) => {
                                                                                                    ((source_ml_source2_data_property_value14446.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_comparison_operand14448 = None::<()>;
                                                                                                !(match (source_ml_source2_comparison_operand14447.clone(), source_ml_source2_comparison_operand14448.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                                            }
                                                                                        }) {
                                                                                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 100, 101, 116, 97, 105, 108, 115, 32, 111, 110, 108, 121, 32, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 114, 114, 111, 114, 32, 112, 104, 97, 115, 101]))
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408Next
                                                                                        }
                                                                                    };
                                                                                    match source_ml_source2_s14428.clone() {
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408Next => {
                                                                                            self::read_java_script_array_creation_callback_error(heap.clone(), state.clone())
                                                                                        }
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin408Return(source_ml_source1_temporary_result408) => {
                                                                                            source_ml_source1_temporary_result408.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin407Return(source_ml_source1_temporary_result408_2) => {
                                                                                source_ml_source1_temporary_result408_2.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin406Return(source_ml_source1_temporary_result408_3) => {
                                                                    source_ml_source1_temporary_result408_3.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin405Return(source_ml_source1_temporary_result408_4) => {
                                                source_ml_source1_temporary_result408_4.clone()
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin404Return(source_ml_source1_temporary_result408_5) => {
                                    source_ml_source1_temporary_result408_5.clone()
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403::MlSource1TemporaryReadJavaScriptArrayCreationChildErrorJoin403Return(source_ml_source1_temporary_result408_6) => {
                        source_ml_source1_temporary_result408_6.clone()
                    }
                }
            }
        }
    }
}
