// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySpreadState;
use super::JavaScriptArraySpreadTransition;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorProtocolTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229;
use super::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226;
use super::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227;
use super::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Result;
use super::begin_java_script_synchronous_iterator;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_iterator_protocol_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_synchronous_iterator_record_error;
use super::source_ml_source1_temporary_read_java_script_array_spread_state_error_loop1225;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_spread_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySpreadState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArraySpreadState::ArraySpreadState(
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field7 = *shared_constructor_field7;
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
                            let source_ml_source1_temporary_values1223 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field2.clone(),
                                    shared_constructor_field3.clone(),
                                    shared_constructor_field4.clone(),
                                    shared_constructor_field7.clone(),
                                ]));
                            {
                                let source_ml_source1_temporary_index1223 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s27182 = self::source_ml_source1_temporary_read_java_script_array_spread_state_error_loop1225(heap.clone(), source_ml_source1_temporary_values1223.clone(), source_ml_source1_temporary_index1223.clone());
                                    match source_ml_source2_s27182.clone() {
                                            self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Result::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Done => {
                                                {
                                                    let target = if {
                                                        let source_ml_source2_s27184 = shared_constructor_field2.clone();
                                                        match source_ml_source2_s27184.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }
                                                    } {
                                                        {
                                                            let source_ml_source2_array_read_value27192 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_array_read_index27193 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field2.clone() {
                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value27191, _) => {
                                                                        Some((source_ml_source2_data_property_value27191.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::ml::Big,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone());
                                                                { let ml_reference_array1530 = source_ml_source2_array_read_value27192.clone(); let ml_reference_position1530 = source_ml_source2_array_read_index27193; let ml_reference_projection1530 = self::ml_array::optional_at(&*ml_reference_array1530.borrow(), self::ml_array::number_index(ml_reference_position1530)); ml_reference_projection1530 }
                                                            }
                                                        }
                                                    } else {
                                                        None::<self::JavaScriptOrdinaryObject>
                                                    };
                                                    if ((((((match state.clone() {
                                                        self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, _, _) => {
                                                            false
                                                        }
                                                    } || {
                                                        let source_ml_source2_comparison_operand27203 = {
                                                            let source_ml_source2_optional_chain_receiver27202 = (target.clone(),);
                                                            if match (source_ml_source2_optional_chain_receiver27202.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                Some({
                                                                    let source_ml_source1_temporary_optional_chain1224 = ((source_ml_source2_optional_chain_receiver27202.clone()).0).clone().expect("a proven present value");
                                                                    match source_ml_source1_temporary_optional_chain1224.clone() {
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
                                                                    }
                                                                })
                                                            } else {
                                                                None::<self::ml_string::Text>
                                                            }
                                                        };
                                                        !(match (source_ml_source2_comparison_operand27203.clone(), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                    }) || {
                                                        let source_ml_source2_s27206 = shared_constructor_field4.clone();
                                                        match source_ml_source2_s27206.clone() {
                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) || !(self::ml_math::is_safe_integer(shared_constructor_field5))) || (shared_constructor_field5 < (0f64))) || {
                                                        let source_ml_source2_same_value_operand27213 = shared_constructor_field5;
                                                        {
                                                            let source_ml_source2_same_value_operand27214 = (-0f64);
                                                            (((source_ml_source2_same_value_operand27213 == source_ml_source2_same_value_operand27214) && ((source_ml_source2_same_value_operand27213 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand27213)) == ((1f64 / source_ml_source2_same_value_operand27214))))) || ((source_ml_source2_same_value_operand27213 != source_ml_source2_same_value_operand27213) && (source_ml_source2_same_value_operand27214 != source_ml_source2_same_value_operand27214)))
                                                        }
                                                    }) || !(self::ml_array::includes(&{ let ml_reference_value1531 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114]), self::ml_string::Text::from_units(vec![115, 116, 101, 112]), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])]))).borrow().clone(); ml_reference_value1531 }, &shared_constructor_field6.clone(), 0f64, false, |a, b| a == b))) {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 65, 114, 114, 97, 121, 44, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 107, 101, 121, 44, 32, 105, 110, 100, 101, 120, 32, 97, 110, 100, 32, 112, 104, 97, 115, 101])
                                                    } else {
                                                        {
                                                            let source_ml_source2_s27218 = if !(match (match state.clone() {
                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, _, source_ml_source2_data_property_value27219) => {
                                                                    ((source_ml_source2_data_property_value27219.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                {
                                                                    let invalid = self::read_java_script_synchronous_iterator_record_error(heap.clone(), (match state.clone() {
                                                                        self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, _, source_ml_source2_data_property_value27220) => {
                                                                            ((source_ml_source2_data_property_value27220.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value"));
                                                                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                        self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226Return(invalid.clone())
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226Next
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226Next
                                                            };
                                                            match source_ml_source2_s27218.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226Next => {
                                                                    {
                                                                        let source_ml_source2_s27254 = if (shared_constructor_field6 == (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114]))) {
                                                                            if match (match state.clone() {
                                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value27257, _) => {
                                                                                    ((source_ml_source2_data_property_value27257.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                            } else {
                                                                                {
                                                                                    let invalid_2 = self::read_java_script_iterator_protocol_state_error(heap.clone(), (match state.clone() {
                                                                                        self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value27258, _) => {
                                                                                            ((source_ml_source2_data_property_value27258.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value"));
                                                                                    if (invalid_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                        self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Return(invalid_2.clone())
                                                                                    } else {
                                                                                        if (((match (match state.clone() {
                                                                                            self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value27261, _) => {
                                                                                                ((source_ml_source2_data_property_value27261.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_data_property_value27262, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value27262.clone()
                                                                                            }
                                                                                        }) == (self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]))) && (!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                            self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value27263, _) => {
                                                                                                ((source_ml_source2_data_property_value27263.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, source_ml_source2_data_property_value27264, _, _, _, _, _, _) => {
                                                                                                let source_ml_source2_data_property_value27264 = *source_ml_source2_data_property_value27264;
                                                                                                source_ml_source2_data_property_value27264.clone()
                                                                                            }
                                                                                        }, shared_constructor_field3.clone())) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                            self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value27266, _) => {
                                                                                                ((source_ml_source2_data_property_value27266.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, source_ml_source2_data_property_value27267, _, _, _, _, _) => {
                                                                                                let source_ml_source2_data_property_value27267 = *source_ml_source2_data_property_value27267;
                                                                                                source_ml_source2_data_property_value27267.clone()
                                                                                            }
                                                                                        }, shared_constructor_field4.clone())))) {
                                                                                            self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 32, 97, 110, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 107, 101, 121]))
                                                                                        } else {
                                                                                            if (((match (match state.clone() {
                                                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value27271, _) => {
                                                                                                    ((source_ml_source2_data_property_value27271.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_data_property_value27272, _, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source2_data_property_value27272.clone()
                                                                                                }
                                                                                            }) == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) && ((match (match state.clone() {
                                                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, _, source_ml_source2_data_property_value27273) => {
                                                                                                    ((source_ml_source2_data_property_value27273.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                }
                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, _, source_ml_source2_data_property_value27274) => {
                                                                                                    ((source_ml_source2_data_property_value27274.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value27275, _, _) => {
                                                                                                    let source_ml_source2_data_property_value27275 = *source_ml_source2_data_property_value27275;
                                                                                                    source_ml_source2_data_property_value27275.clone()
                                                                                                }
                                                                                            }, match match (match state.clone() {
                                                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value27276, _) => {
                                                                                                    ((source_ml_source2_data_property_value27276.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value27277, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value27277 = *source_ml_source2_data_property_value27277;
                                                                                                    source_ml_source2_data_property_value27277.clone()
                                                                                                }
                                                                                            } {
                                                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value27278, _, _) => {
                                                                                                    let source_ml_source2_data_property_value27278 = *source_ml_source2_data_property_value27278;
                                                                                                    source_ml_source2_data_property_value27278.clone()
                                                                                                }
                                                                                            }))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, _, source_ml_source2_data_property_value27279) => {
                                                                                                    ((source_ml_source2_data_property_value27279.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value27280, _) => {
                                                                                                    let source_ml_source2_data_property_value27280 = *source_ml_source2_data_property_value27280;
                                                                                                    source_ml_source2_data_property_value27280.clone()
                                                                                                }
                                                                                            }, match match (match state.clone() {
                                                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value27281, _) => {
                                                                                                    ((source_ml_source2_data_property_value27281.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value27282, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value27282 = *source_ml_source2_data_property_value27282;
                                                                                                    source_ml_source2_data_property_value27282.clone()
                                                                                                }
                                                                                            } {
                                                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value27283, _) => {
                                                                                                    let source_ml_source2_data_property_value27283 = *source_ml_source2_data_property_value27283;
                                                                                                    source_ml_source2_data_property_value27283.clone()
                                                                                                }
                                                                                            })))) {
                                                                                                self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 97, 100, 118, 97, 110, 99, 101, 109, 101, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 99, 104, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 99, 111, 114, 100]))
                                                                                            } else {
                                                                                                if !(self::ml_array::includes(&{ let ml_reference_value1532 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]), self::ml_string::Text::from_units(vec![115, 116, 101, 112])]))).borrow().clone(); ml_reference_value1532 }, &match (match state.clone() {
                                                                                                    self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value27284, _) => {
                                                                                                        ((source_ml_source2_data_property_value27284.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().expect("a proven present value") {
                                                                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_data_property_value27285, _, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value27285.clone()
                                                                                                    }
                                                                                                }, 0f64, false, |a, b| a == b)) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 97, 100, 100, 32, 97, 110, 32, 117, 110, 114, 101, 108, 97, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 108, 101, 97, 110, 117, 112, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if (!(match (match state.clone() {
                                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value27286, _) => {
                                                                                    ((source_ml_source2_data_property_value27286.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match (match state.clone() {
                                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, _, source_ml_source2_data_property_value27287) => {
                                                                                    ((source_ml_source2_data_property_value27287.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Return(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 115, 112, 114, 101, 97, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 111, 114, 100, 32, 119, 105, 116, 104, 111, 117, 116, 32, 97, 32, 115, 116, 97, 108, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Next
                                                                            }
                                                                        };
                                                                        match source_ml_source2_s27254.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Next => {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1227Return(source_ml_source1_temporary_result1227) => {
                                                                                source_ml_source1_temporary_result1227.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorJoin1226Return(source_ml_source1_temporary_result1227_2) => {
                                                                    source_ml_source1_temporary_result1227_2.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Result::MlSource1TemporaryReadJavaScriptArraySpreadStateErrorLoop1225Return(source_ml_source1_temporary_result1227_3) => {
                                                source_ml_source1_temporary_result1227_3.clone()
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
pub fn begin_java_script_array_spread(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    input: self::JavaScriptSemanticValue,
    index: f64,
    iterator_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptArraySpreadTransition {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field9, _, _, _, _, _) => {
            let state = {
                let source_ml_source2_constructor_field27293 =
                    self::JavaScriptSemanticValue::Undefined;
                self::JavaScriptArraySpreadState::ArraySpreadState(
                    Box::new(target.clone()),
                    Box::new(input.clone()),
                    Box::new(iterator_key.clone()),
                    index,
                    self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114]),
                    Box::new(source_ml_source2_constructor_field27293.clone()),
                    None::<(Option<self::JavaScriptIteratorProtocolState>,)>,
                    None::<(Option<self::JavaScriptSynchronousIteratorRecord>,)>,
                )
            };
            {
                let acquired = self::begin_java_script_synchronous_iterator(
                    heap.clone(),
                    input.clone(),
                    iterator_key.clone(),
                );
                {
                    let provisional = {
                        let source_ml_source2_nominal_spread27295 = state.clone();
                        match source_ml_source2_nominal_spread27295.clone() {
                            self::JavaScriptArraySpreadState::ArraySpreadState(
                                shared_constructor_field1,
                                shared_constructor_field2,
                                shared_constructor_field3,
                                shared_constructor_field4,
                                shared_constructor_field5,
                                shared_constructor_field6,
                                shared_constructor_field7,
                                shared_constructor_field8,
                            ) => {
                                let shared_constructor_field1 = *shared_constructor_field1;
                                let shared_constructor_field2 = *shared_constructor_field2;
                                let shared_constructor_field3 = *shared_constructor_field3;
                                let shared_constructor_field6 = *shared_constructor_field6;
                                {
                                    let source_ml_source2_nominal_field27296 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field27298 =
                                            shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field27300 =
                                                shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field27302 =
                                                    shared_constructor_field4;
                                                {
                                                    let source_ml_source2_nominal_field27304 =
                                                        shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27306 =
                                                            shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27308 =
                                                                shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27310 =
                                                                    shared_constructor_field8
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_override27312 = match acquired.clone() {
                                                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(_, state_2) => {
                                                                                let state_2 = *state_2;
                                                                                Some(state_2.clone())
                                                                            }
                                                                            _ => {
                                                                                None::<self::JavaScriptIteratorProtocolState>
                                                                            }
                                                                        };
                                                                    self::JavaScriptArraySpreadState::ArraySpreadState(Box::new(source_ml_source2_nominal_field27296.clone()), Box::new(source_ml_source2_nominal_field27298.clone()), Box::new(source_ml_source2_nominal_field27300.clone()), source_ml_source2_nominal_field27302, source_ml_source2_nominal_field27304.clone(), Box::new(source_ml_source2_nominal_field27306.clone()), Some((source_ml_source2_nominal_override27312.clone(),)), source_ml_source2_nominal_field27310.clone())
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
                    };
                    {
                        let source_ml_source2_s27326 = match acquired.clone() {
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(_, _) => {
                                    {
                                        let error = self::read_java_script_array_spread_state_error(heap.clone(), provisional.clone());
                                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                                            self::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229Return(Box::new(self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(Box::new(heap.clone()), error.clone())))
                                        } else {
                                            self::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229Next
                                        }
                                    }
                                }
                                _ => {
                                    {
                                        let invalid = self::read_java_script_ordinary_value_error(heap.clone(), target.clone());
                                        {
                                            let object = match target.clone() {
                                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                                    {
                                                        let source_ml_source2_array_read_value27332 = shared_constructor_field9.clone();
                                                        {
                                                            let source_ml_source2_array_read_index27333 = self::find_java_script_ordinary_object(shared_constructor_field9.clone(), identity.clone());
                                                            { let ml_reference_array1533 = source_ml_source2_array_read_value27332.clone(); let ml_reference_position1533 = source_ml_source2_array_read_index27333; let ml_reference_projection1533 = self::ml_array::optional_at(&*ml_reference_array1533.borrow(), self::ml_array::number_index(ml_reference_position1533)); ml_reference_projection1533 }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    None::<self::JavaScriptOrdinaryObject>
                                                }
                                            };
                                            if (((((invalid != (self::ml_string::Text::from_units(vec![]))) || {
                                                let source_ml_source2_comparison_operand27335 = {
                                                    let source_ml_source2_optional_chain_receiver27334 = (object.clone(),);
                                                    if match (source_ml_source2_optional_chain_receiver27334.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        Some({
                                                            let source_ml_source1_temporary_optional_chain1228 = ((source_ml_source2_optional_chain_receiver27334.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_temporary_optional_chain1228.clone() {
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
                                                            }
                                                        })
                                                    } else {
                                                        None::<self::ml_string::Text>
                                                    }
                                                };
                                                !(match (source_ml_source2_comparison_operand27335.clone(), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                            }) || !(self::ml_math::is_safe_integer(index))) || (index < (0f64))) || {
                                                let source_ml_source2_same_value_operand27338 = (-0f64);
                                                (((index == source_ml_source2_same_value_operand27338) && ((index != (0f64)) || (((1f64 / index)) == ((1f64 / source_ml_source2_same_value_operand27338))))) || ((index != index) && (source_ml_source2_same_value_operand27338 != source_ml_source2_same_value_operand27338)))
                                            }) {
                                                self::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229Return(Box::new(self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(Box::new(heap.clone()), if !invalid.0.is_empty() {
                                                    invalid.clone()
                                                } else {
                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 32, 97, 110, 100, 32, 105, 110, 100, 101, 120])
                                                })))
                                            } else {
                                                self::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229Next
                                            }
                                        }
                                    }
                                }
                            };
                        match source_ml_source2_s27326.clone() {
                                self::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229Next => {
                                    self::continue_java_script_array_spread_iterator(heap.clone(), state.clone(), acquired.clone())
                                }
                                self::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229::MlSource1TemporaryBeginJavaScriptArraySpreadJoin1229Return(source_ml_source1_temporary_result1229) => {
                                    let source_ml_source1_temporary_result1229 = *source_ml_source1_temporary_result1229;
                                    source_ml_source1_temporary_result1229.clone()
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
pub fn continue_java_script_array_spread_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySpreadState,
    transition: self::JavaScriptIteratorProtocolTransition,
) -> self::JavaScriptArraySpreadTransition {
    {
        let source_ml_source1_temporary_binding1230 = state.clone();
        {
            let previous = match source_ml_source1_temporary_binding1230.clone() {
                self::JavaScriptArraySpreadState::ArraySpreadState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value27340,
                    _,
                ) => ((source_ml_source2_data_property_value27340
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,)))
                .0)
                    .clone(),
            };
            {
                let base = match source_ml_source1_temporary_binding1230.clone() {
                    self::JavaScriptArraySpreadState::ArraySpreadState(
                        source_ml_source2_nominal_rest_field27341,
                        source_ml_source2_nominal_rest_field27342,
                        source_ml_source2_nominal_rest_field27343,
                        source_ml_source2_nominal_rest_field27344,
                        source_ml_source2_nominal_rest_field27345,
                        source_ml_source2_nominal_rest_field27346,
                        _,
                        source_ml_source2_nominal_rest_field27347,
                    ) => {
                        let source_ml_source2_nominal_rest_field27341 =
                            *source_ml_source2_nominal_rest_field27341;
                        let source_ml_source2_nominal_rest_field27342 =
                            *source_ml_source2_nominal_rest_field27342;
                        let source_ml_source2_nominal_rest_field27343 =
                            *source_ml_source2_nominal_rest_field27343;
                        let source_ml_source2_nominal_rest_field27346 =
                            *source_ml_source2_nominal_rest_field27346;
                        self::JavaScriptArraySpreadState::ArraySpreadState(
                            Box::new(source_ml_source2_nominal_rest_field27341.clone()),
                            Box::new(source_ml_source2_nominal_rest_field27342.clone()),
                            Box::new(source_ml_source2_nominal_rest_field27343.clone()),
                            source_ml_source2_nominal_rest_field27344,
                            source_ml_source2_nominal_rest_field27345.clone(),
                            Box::new(source_ml_source2_nominal_rest_field27346.clone()),
                            None::<(Option<self::JavaScriptIteratorProtocolState>,)>,
                            source_ml_source2_nominal_rest_field27347.clone(),
                        )
                    }
                };
                match base.clone() {
                    self::JavaScriptArraySpreadState::ArraySpreadState(
                        shared_constructor_field1,
                        shared_constructor_field2,
                        shared_constructor_field3,
                        shared_constructor_field4,
                        shared_constructor_field5,
                        shared_constructor_field6,
                        shared_constructor_field7,
                        shared_constructor_field8,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        let shared_constructor_field3 = *shared_constructor_field3;
                        let shared_constructor_field6 = *shared_constructor_field6;
                        match transition.clone() {
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(heap_2, state_2) => {
                                let heap_2 = *heap_2;
                                let state_2 = *state_2;
                                self::JavaScriptArraySpreadTransition::ArraySpreadContinue(Box::new(heap_2.clone()), Box::new({
                                    let source_ml_source2_nominal_field27353 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field27355 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field27357 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field27359 = shared_constructor_field4;
                                                {
                                                    let source_ml_source2_nominal_field27361 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27363 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27365 = shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27367 = shared_constructor_field8.clone();
                                                                self::JavaScriptArraySpreadState::ArraySpreadState(Box::new(source_ml_source2_nominal_field27353.clone()), Box::new(source_ml_source2_nominal_field27355.clone()), Box::new(source_ml_source2_nominal_field27357.clone()), source_ml_source2_nominal_field27359, self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114]), Box::new(source_ml_source2_nominal_field27363.clone()), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field27367.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }))
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(heap_3, state_3, callee, receiver, arguments) => {
                                let heap_3 = *heap_3;
                                let state_3 = *state_3;
                                let callee = *callee;
                                let receiver = *receiver;
                                self::JavaScriptArraySpreadTransition::ArraySpreadCall(Box::new(heap_3.clone()), Box::new({
                                    let source_ml_source2_nominal_field27372 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field27374 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field27376 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field27378 = shared_constructor_field4;
                                                {
                                                    let source_ml_source2_nominal_field27380 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27382 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27384 = shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27386 = shared_constructor_field8.clone();
                                                                self::JavaScriptArraySpreadState::ArraySpreadState(Box::new(source_ml_source2_nominal_field27372.clone()), Box::new(source_ml_source2_nominal_field27374.clone()), Box::new(source_ml_source2_nominal_field27376.clone()), source_ml_source2_nominal_field27378, self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114]), Box::new(source_ml_source2_nominal_field27382.clone()), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field27386.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }), Box::new(callee.clone()), Box::new(receiver.clone()), arguments.clone())
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(heap_4, state_4, receiver_2, key) => {
                                let heap_4 = *heap_4;
                                let state_4 = *state_4;
                                let receiver_2 = *receiver_2;
                                let key = *key;
                                self::JavaScriptArraySpreadTransition::ArraySpreadPrimitiveGet(Box::new(heap_4.clone()), Box::new({
                                    let source_ml_source2_nominal_field27391 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field27393 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field27395 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field27397 = shared_constructor_field4;
                                                {
                                                    let source_ml_source2_nominal_field27399 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27401 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27403 = shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27405 = shared_constructor_field8.clone();
                                                                self::JavaScriptArraySpreadState::ArraySpreadState(Box::new(source_ml_source2_nominal_field27391.clone()), Box::new(source_ml_source2_nominal_field27393.clone()), Box::new(source_ml_source2_nominal_field27395.clone()), source_ml_source2_nominal_field27397, self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114]), Box::new(source_ml_source2_nominal_field27401.clone()), Some((Some(state_4.clone()),)), source_ml_source2_nominal_field27405.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }), Box::new(receiver_2.clone()), Box::new(key.clone()))
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolAcquired(heap_5, record) => {
                                let heap_5 = *heap_5;
                                let record = *record;
                                self::JavaScriptArraySpreadTransition::ArraySpreadContinue(Box::new(heap_5.clone()), Box::new({
                                    let source_ml_source2_nominal_field27410 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field27412 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field27414 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field27416 = shared_constructor_field4;
                                                {
                                                    let source_ml_source2_nominal_field27418 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27420 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27422 = shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27424 = shared_constructor_field8.clone();
                                                                self::JavaScriptArraySpreadState::ArraySpreadState(Box::new(source_ml_source2_nominal_field27410.clone()), Box::new(source_ml_source2_nominal_field27412.clone()), Box::new(source_ml_source2_nominal_field27414.clone()), source_ml_source2_nominal_field27416, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), Box::new(source_ml_source2_nominal_field27420.clone()), source_ml_source2_nominal_field27422.clone(), Some((Some(record.clone()),)))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }))
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(heap_6, record_2, done, value) => {
                                let heap_6 = *heap_6;
                                let record_2 = *record_2;
                                let value = *value;
                                if done {
                                    self::JavaScriptArraySpreadTransition::ArraySpreadCompleted(Box::new(heap_6.clone()), match state.clone() {
                                        self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, source_ml_source2_data_property_value27428, _, _, _, _) => {
                                            source_ml_source2_data_property_value27428
                                        }
                                    })
                                } else {
                                    self::JavaScriptArraySpreadTransition::ArraySpreadContinue(Box::new(heap_6.clone()), Box::new({
                                        let source_ml_source2_nominal_field27430 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field27432 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field27434 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field27436 = shared_constructor_field4;
                                                    {
                                                        let source_ml_source2_nominal_field27438 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27440 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27442 = shared_constructor_field7.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27444 = shared_constructor_field8.clone();
                                                                    self::JavaScriptArraySpreadState::ArraySpreadState(Box::new(source_ml_source2_nominal_field27430.clone()), Box::new(source_ml_source2_nominal_field27432.clone()), Box::new(source_ml_source2_nominal_field27434.clone()), source_ml_source2_nominal_field27436, self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), Box::new(value.clone()), source_ml_source2_nominal_field27442.clone(), Some((Some(record_2.clone()),)))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }))
                                }
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(heap_7, _, _) => {
                                let heap_7 = *heap_7;
                                self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(Box::new(heap_7.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 32, 99, 108, 101, 97, 110, 117, 112, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 102, 111, 114, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 100, 118, 97, 110, 99, 101, 109, 101, 110, 116]))
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(heap_8, _, value_2) => {
                                let heap_8 = *heap_8;
                                let value_2 = *value_2;
                                self::JavaScriptArraySpreadTransition::ArraySpreadThrow(Box::new(heap_8.clone()), Box::new(value_2.clone()))
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolError(heap_9, _, error_class, reason) => {
                                let heap_9 = *heap_9;
                                self::JavaScriptArraySpreadTransition::ArraySpreadError(Box::new(heap_9.clone()), error_class.clone(), reason.clone())
                            }
                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(heap_10, reason_2) => {
                                let heap_10 = *heap_10;
                                self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(Box::new(heap_10.clone()), reason_2.clone())
                            }
                        }
                    }
                }
            }
        }
    }
}
