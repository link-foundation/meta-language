// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayIteratorCall;
use super::JavaScriptArrayIteratorState;
use super::JavaScriptArraySpreadState;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionIteratorResultState;
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorSelectedCall;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333;
use super::find_java_script_iterator_region;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_iterator_state_error;
use super::read_java_script_array_spread_state_error;
use super::read_java_script_call_iterator_close_completion;
use super::read_java_script_collection_iterator_result_state_error;
use super::read_java_script_iterator_protocol_state_error;
use super::test_java_script_collection_iterator_intrinsic_mode;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_iterator_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    suspended: bool,
    pending: self::JavaScriptCallPending,
) -> self::ml_string::Text {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            _,
            shared_constructor_field10,
            shared_constructor_field11,
            _,
            _,
            _,
            _,
            shared_constructor_field12,
            _,
            _,
        ) => {
            match frame.clone() {
                self::JavaScriptCallFrame::CallFrame(
                    _,
                    shared_constructor_field8,
                    shared_constructor_field9,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    match pending.clone() {
                        self::JavaScriptCallPending::WaitIteratorProtocol(
                            destination,
                            done,
                            slot,
                            operation,
                            realm,
                            pending_state,
                        ) => {
                            let pending_state = *pending_state;
                            match pending_state.clone() {
                                self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                    shared_constructor_field3,
                                    _,
                                    shared_constructor_field4,
                                    shared_constructor_field5,
                                    shared_constructor_field6,
                                    _,
                                    shared_constructor_field7,
                                    _,
                                    _,
                                ) => {
                                    let shared_constructor_field4 = *shared_constructor_field4;
                                    let shared_constructor_field5 = *shared_constructor_field5;
                                    let shared_constructor_field6 = *shared_constructor_field6;
                                    let shared_constructor_field7 = *shared_constructor_field7;
                                    if (shared_constructor_field8 == (0f64)) {
                                        self::ml_string::Text::from_units(vec![
                                            105, 116, 101, 114, 97, 116, 111, 114, 32, 119, 111,
                                            114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                                            105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                                            111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114,
                                            117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114,
                                            101, 103, 105, 111, 110,
                                        ])
                                    } else {
                                        {
                                            let instruction = {
                                                let source_ml_source2_array_read_value31821 =
                                                    shared_constructor_field12.clone();
                                                {
                                                    let source_ml_source2_array_read_index31822 =
                                                        (shared_constructor_field8 - 1f64);
                                                    {
                                                        let ml_reference_array1965 =
                                                            source_ml_source2_array_read_value31821
                                                                .clone();
                                                        let ml_reference_position1965 =
                                                            source_ml_source2_array_read_index31822;
                                                        let ml_reference_projection1965 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array1965.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position1965,
                                                                ),
                                                            );
                                                        ml_reference_projection1965
                                                    }
                                                }
                                            };
                                            if (match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction.clone().expect("a proven present value") {
                                                self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) {
                                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 103, 105, 111, 110])
                                            } else {
                                                if (((((((((match instruction.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_data_property_value31832, _, _, _, _, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value31832,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0) != destination) || ((((match instruction.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::IteratorProtocol(_, source_ml_source2_data_property_value31834, _, _, _, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value31834,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0) != done)) || ((((match instruction.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value31836, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value31836,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0) != slot)) || (((((match instruction.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, source_ml_source2_data_property_value31838, _) => {
                                                        Some((source_ml_source2_data_property_value31838.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml::Big,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone()) != realm)) || (((((match instruction.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, source_ml_source2_data_property_value31840, _, _) => {
                                                        Some((source_ml_source2_data_property_value31840.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml_string::Text,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone()) != operation)) || (shared_constructor_field3 != operation)) {
                                                    self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 103, 105, 111, 110])
                                                } else {
                                                    {
                                                        let error = self::read_java_script_iterator_protocol_state_error(heap.clone(), pending_state.clone());
                                                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                            error.clone()
                                                        } else {
                                                            if ((operation == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) && (((match pending_state.clone() {
                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, source_ml_source2_data_property_value31845, _) => {
                                                                    ((source_ml_source2_data_property_value31845.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                                                }
                                                            }).clone().unwrap_or_else(|| true)) != (((((match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, _, source_ml_source2_data_property_value31846) => {
                                                                    Some((((source_ml_source2_data_property_value31846.clone().unwrap_or_else(|| (None::<bool>,))).0).clone(),))
                                                                }
                                                                _ => {
                                                                    None::<(Option<bool>,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0).clone()).clone().unwrap_or_else(|| true)))) {
                                                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 100, 118, 97, 110, 99, 101, 109, 101, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 45, 118, 101, 114, 115, 117, 115, 45, 101, 108, 105, 115, 105, 111, 110, 32, 99, 104, 111, 105, 99, 101])
                                                            } else {
                                                                {
                                                                    let source_ml_source2_s31885 = if (operation == (self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]))) {
                                                                        if (!(self::test_java_script_semantic_same_value(shared_constructor_field4.clone(), ({
                                                                            let source_ml_source2_array_read_value31889 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index31890 = ((match instruction.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallInstruction::IteratorProtocol(_, _, source_ml_source2_data_property_value31888, _, _, _, _, _) => {
                                                                                        Some((source_ml_source2_data_property_value31888,))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(f64,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0;
                                                                                { let ml_reference_array1966 = source_ml_source2_array_read_value31889.clone(); let ml_reference_position1966 = source_ml_source2_array_read_index31890; let ml_reference_projection1966 = self::ml_array::optional_at(&*ml_reference_array1966.borrow(), self::ml_array::number_index(ml_reference_position1966)); ml_reference_projection1966 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field5.clone(), ({
                                                                            let source_ml_source2_array_read_value31894 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_array_read_index31895 = ((match instruction.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, source_ml_source2_data_property_value31893, _, _, _, _) => {
                                                                                        Some((source_ml_source2_data_property_value31893,))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(f64,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0;
                                                                                { let ml_reference_array1967 = source_ml_source2_array_read_value31894.clone(); let ml_reference_position1967 = source_ml_source2_array_read_index31895; let ml_reference_projection1967 = self::ml_array::optional_at(&*ml_reference_array1967.borrow(), self::ml_array::number_index(ml_reference_position1967)); ml_reference_projection1967 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) {
                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 104, 101, 108, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 110, 100, 32, 97, 110, 100, 32, 107, 101, 121]))
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333Next
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let regions = (match frame.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value31896, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value31896.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new())));
                                                                            {
                                                                                let region = {
                                                                                    let source_ml_source2_array_read_index31898 = self::find_java_script_iterator_region(regions.clone(), slot);
                                                                                    { let ml_reference_array1968 = regions.clone(); let ml_reference_position1968 = source_ml_source2_array_read_index31898; let ml_reference_projection1968 = self::ml_array::optional_at(&*ml_reference_array1968.borrow(), self::ml_array::number_index(ml_reference_position1968)); ml_reference_projection1968 }
                                                                                };
                                                                                if (((match (region.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_semantic_same_value(match match region.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, source_ml_source2_data_property_value31899) => {
                                                                                        let source_ml_source2_data_property_value31899 = *source_ml_source2_data_property_value31899;
                                                                                        source_ml_source2_data_property_value31899.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value31900, _, _) => {
                                                                                        let source_ml_source2_data_property_value31900 = *source_ml_source2_data_property_value31900;
                                                                                        source_ml_source2_data_property_value31900.clone()
                                                                                    }
                                                                                }, match shared_constructor_field6.clone() {
                                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value31902, _, _) => {
                                                                                        let source_ml_source2_data_property_value31902 = *source_ml_source2_data_property_value31902;
                                                                                        source_ml_source2_data_property_value31902.clone()
                                                                                    }
                                                                                }))) || !(self::test_java_script_semantic_same_value(match match region.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, source_ml_source2_data_property_value31903) => {
                                                                                        let source_ml_source2_data_property_value31903 = *source_ml_source2_data_property_value31903;
                                                                                        source_ml_source2_data_property_value31903.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value31904, _) => {
                                                                                        let source_ml_source2_data_property_value31904 = *source_ml_source2_data_property_value31904;
                                                                                        source_ml_source2_data_property_value31904.clone()
                                                                                    }
                                                                                }, match shared_constructor_field6.clone() {
                                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value31906, _) => {
                                                                                        let source_ml_source2_data_property_value31906 = *source_ml_source2_data_property_value31906;
                                                                                        source_ml_source2_data_property_value31906.clone()
                                                                                    }
                                                                                }))) || ((match match region.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallIteratorRegion::CallIteratorRegion(_, source_ml_source2_data_property_value31911) => {
                                                                                        let source_ml_source2_data_property_value31911 = *source_ml_source2_data_property_value31911;
                                                                                        source_ml_source2_data_property_value31911.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value31912) => {
                                                                                        source_ml_source2_data_property_value31912
                                                                                    }
                                                                                }) != (match shared_constructor_field6.clone() {
                                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value31914) => {
                                                                                        source_ml_source2_data_property_value31914
                                                                                    }
                                                                                }))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 99, 104, 101, 100, 32, 114, 101, 103, 105, 111, 110, 32, 114, 101, 99, 111, 114, 100]))
                                                                                } else {
                                                                                    if (operation == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                                                                                        {
                                                                                            let completion = self::read_java_script_call_iterator_close_completion(frame.clone());
                                                                                            match completion.clone() {
                                                                                                self::JavaScriptIteratorCompletion::IteratorCompletion(shared_constructor_field1, shared_constructor_field2, _) => {
                                                                                                    let shared_constructor_field2 = *shared_constructor_field2;
                                                                                                    if ((shared_constructor_field1 != (match shared_constructor_field7.clone() {
                                                                                                        self::JavaScriptIteratorCompletion::IteratorCompletion(source_ml_source2_data_property_value31920, _, _) => {
                                                                                                            source_ml_source2_data_property_value31920.clone()
                                                                                                        }
                                                                                                    })) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), match shared_constructor_field7.clone() {
                                                                                                        self::JavaScriptIteratorCompletion::IteratorCompletion(_, source_ml_source2_data_property_value31923, _) => {
                                                                                                            let source_ml_source2_data_property_value31923 = *source_ml_source2_data_property_value31923;
                                                                                                            source_ml_source2_data_property_value31923.clone()
                                                                                                        }
                                                                                                    }))) {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 108, 101, 97, 110, 117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 102, 101, 114, 114, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                                                                    } else {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333Next
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333Next
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    match source_ml_source2_s31885.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333Next => {
                                                                            if (suspended && match (match pending_state.clone() {
                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value31924) => {
                                                                                    ((source_ml_source2_data_property_value31924.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100])
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333::MlSource1TemporaryReadJavaScriptCallPendingIteratorErrorJoin1333Return(source_ml_source1_temporary_result1333) => {
                                                                            source_ml_source1_temporary_result1333.clone()
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
                        self::JavaScriptCallPending::WaitArraySpread(
                            destination_2,
                            realm_2,
                            state,
                        ) => {
                            let state = *state;
                            match state.clone() {
                                self::JavaScriptArraySpreadState::ArraySpreadState(
                                    shared_constructor_field1_2,
                                    shared_constructor_field2_2,
                                    shared_constructor_field3_2,
                                    shared_constructor_field4_2,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let shared_constructor_field1_2 = *shared_constructor_field1_2;
                                    let shared_constructor_field2_2 = *shared_constructor_field2_2;
                                    let shared_constructor_field3_2 = *shared_constructor_field3_2;
                                    if (shared_constructor_field8 == (0f64)) {
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100,
                                            32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                            115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117,
                                            114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99,
                                            116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105,
                                            110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114,
                                            101, 97, 108, 109,
                                        ])
                                    } else {
                                        {
                                            let instruction_2 = {
                                                let source_ml_source2_array_read_value31929 =
                                                    shared_constructor_field12.clone();
                                                {
                                                    let source_ml_source2_array_read_index31930 =
                                                        (shared_constructor_field8 - 1f64);
                                                    {
                                                        let ml_reference_array1969 =
                                                            source_ml_source2_array_read_value31929
                                                                .clone();
                                                        let ml_reference_position1969 =
                                                            source_ml_source2_array_read_index31930;
                                                        let ml_reference_projection1969 =
                                                            self::ml_array::optional_at(
                                                                &*ml_reference_array1969.borrow(),
                                                                self::ml_array::number_index(
                                                                    ml_reference_position1969,
                                                                ),
                                                            );
                                                        ml_reference_projection1969
                                                    }
                                                }
                                            };
                                            if (match (instruction_2.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } || match instruction_2
                                                .clone()
                                                .expect("a proven present value")
                                            {
                                                self::JavaScriptCallInstruction::ArraySpread(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => false,
                                                _ => true,
                                            }) {
                                                self::ml_string::Text::from_units(vec![
                                                    65, 114, 114, 97, 121, 32, 115, 112, 114, 101,
                                                    97, 100, 32, 114, 101, 116, 97, 105, 110, 115,
                                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                                    108, 32, 115, 111, 117, 114, 99, 101, 32, 105,
                                                    110, 115, 116, 114, 117, 99, 116, 105, 111,
                                                    110, 44, 32, 100, 101, 115, 116, 105, 110, 97,
                                                    116, 105, 111, 110, 32, 97, 110, 100, 32, 114,
                                                    101, 97, 108, 109,
                                                ])
                                            } else {
                                                if ((((((match instruction_2.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::ArraySpread(source_ml_source2_data_property_value31938, _, _, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value31938,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0) != destination_2) || (((((match instruction_2.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, _, source_ml_source2_data_property_value31940) => {
                                                        Some((source_ml_source2_data_property_value31940.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml::Big,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone()) != realm_2)) || (realm_2 != shared_constructor_field10)) {
                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                } else {
                                                    {
                                                        let error_2 = self::read_java_script_array_spread_state_error(heap.clone(), state.clone());
                                                        if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                            error_2.clone()
                                                        } else {
                                                            if ((!(self::test_java_script_semantic_same_value(({
                                                                let source_ml_source2_array_read_value31945 = shared_constructor_field9.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index31946 = ((match instruction_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallInstruction::ArraySpread(_, source_ml_source2_data_property_value31944, _, _, _, _) => {
                                                                            Some((source_ml_source2_data_property_value31944,))
                                                                        }
                                                                        _ => {
                                                                            None::<(f64,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0;
                                                                    { let ml_reference_array1970 = source_ml_source2_array_read_value31945.clone(); let ml_reference_position1970 = source_ml_source2_array_read_index31946; let ml_reference_projection1970 = self::ml_array::optional_at(&*ml_reference_array1970.borrow(), self::ml_array::number_index(ml_reference_position1970)); ml_reference_projection1970 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field1_2.clone())) || !(self::test_java_script_semantic_same_value(({
                                                                let source_ml_source2_array_read_value31950 = shared_constructor_field9.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index31951 = ((match instruction_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallInstruction::ArraySpread(_, _, source_ml_source2_data_property_value31949, _, _, _) => {
                                                                            Some((source_ml_source2_data_property_value31949,))
                                                                        }
                                                                        _ => {
                                                                            None::<(f64,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0;
                                                                    { let ml_reference_array1971 = source_ml_source2_array_read_value31950.clone(); let ml_reference_position1971 = source_ml_source2_array_read_index31951; let ml_reference_projection1971 = self::ml_array::optional_at(&*ml_reference_array1971.borrow(), self::ml_array::number_index(ml_reference_position1971)); ml_reference_projection1971 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2_2.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                                let source_ml_source2_array_read_value31955 = shared_constructor_field9.clone();
                                                                {
                                                                    let source_ml_source2_array_read_index31956 = ((match instruction_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, source_ml_source2_data_property_value31954, _) => {
                                                                            Some((source_ml_source2_data_property_value31954,))
                                                                        }
                                                                        _ => {
                                                                            None::<(f64,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0;
                                                                    { let ml_reference_array1972 = source_ml_source2_array_read_value31955.clone(); let ml_reference_position1972 = source_ml_source2_array_read_index31956; let ml_reference_projection1972 = self::ml_array::optional_at(&*ml_reference_array1972.borrow(), self::ml_array::number_index(ml_reference_position1972)); ml_reference_projection1972 }
                                                                }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field3_2.clone()))) {
                                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 44, 32, 115, 111, 117, 114, 99, 101, 32, 97, 110, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 107, 101, 121])
                                                            } else {
                                                                {
                                                                    let index = ({
                                                                        let source_ml_source2_array_read_value31960 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index31961 = ((match instruction_2.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallInstruction::ArraySpread(_, _, _, source_ml_source2_data_property_value31959, _, _) => {
                                                                                    Some((source_ml_source2_data_property_value31959,))
                                                                                }
                                                                                _ => {
                                                                                    None::<(f64,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0;
                                                                            { let ml_reference_array1973 = source_ml_source2_array_read_value31960.clone(); let ml_reference_position1973 = source_ml_source2_array_read_index31961; let ml_reference_projection1973 = self::ml_array::optional_at(&*ml_reference_array1973.borrow(), self::ml_array::number_index(ml_reference_position1973)); ml_reference_projection1973 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    if (((match index.clone() {
                                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    } || !(self::ml_math::is_safe_integer(((match index.clone() {
                                                                        self::JavaScriptSemanticValue::Number(source_ml_source2_data_property_value31963) => {
                                                                            Some((source_ml_source2_data_property_value31963,))
                                                                        }
                                                                        _ => {
                                                                            None::<(f64,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0))) || ((((match index.clone() {
                                                                        self::JavaScriptSemanticValue::Number(source_ml_source2_data_property_value31965) => {
                                                                            Some((source_ml_source2_data_property_value31965,))
                                                                        }
                                                                        _ => {
                                                                            None::<(f64,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0) < (0f64))) || (shared_constructor_field4_2 < (((match index.clone() {
                                                                        self::JavaScriptSemanticValue::Number(source_ml_source2_data_property_value31969) => {
                                                                            Some((source_ml_source2_data_property_value31969,))
                                                                        }
                                                                        _ => {
                                                                            None::<(f64,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0))) {
                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 101, 120, 97, 99, 116, 32, 97, 99, 99, 117, 109, 117, 108, 97, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                                                    } else {
                                                                        if (suspended && match ({
                                                                            let source_ml_source2_optional_chain_receiver31971 = (match state.clone() {
                                                                                self::JavaScriptArraySpreadState::ArraySpreadState(_, _, _, _, _, _, source_ml_source2_data_property_value31970, _) => {
                                                                                    ((source_ml_source2_data_property_value31970.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                }
                                                                            },);
                                                                            match if match (source_ml_source2_optional_chain_receiver31971.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                Some({
                                                                                    let source_ml_source1_temporary_optional_chain1332 = ((source_ml_source2_optional_chain_receiver31971.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_temporary_optional_chain1332.clone() {
                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value31972) => {
                                                                                            ((source_ml_source2_data_property_value31972.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                        }
                                                                                    }
                                                                                })
                                                                            } else {
                                                                                None::<self::JavaScriptIteratorSelectedCall>
                                                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108])
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
                        }
                        self::JavaScriptCallPending::WaitArrayIteratorNext(
                            destination_3,
                            realm_3,
                            state_2,
                        ) => {
                            let state_2 = *state_2;
                            match state_2.clone() {
                                self::JavaScriptArrayIteratorState::ArrayIteratorState(
                                    shared_constructor_field1_3,
                                    _,
                                    shared_constructor_field2_3,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let shared_constructor_field1_3 = *shared_constructor_field1_3;
                                    let shared_constructor_field2_3 = *shared_constructor_field2_3;
                                    {
                                        let instruction_3 = {
                                            let source_ml_source2_array_read_value31975 =
                                                shared_constructor_field12.clone();
                                            {
                                                let source_ml_source2_array_read_index31976 =
                                                    shared_constructor_field8;
                                                {
                                                    let ml_reference_array1974 =
                                                        source_ml_source2_array_read_value31975
                                                            .clone();
                                                    let ml_reference_position1974 =
                                                        source_ml_source2_array_read_index31976;
                                                    let ml_reference_projection1974 =
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array1974.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position1974,
                                                            ),
                                                        );
                                                    ml_reference_projection1974
                                                }
                                            }
                                        };
                                        if (((((!(self::test_java_script_collection_iterator_intrinsic_mode(&(shared_constructor_field11))) || (shared_constructor_field10 != realm_3)) || match (instruction_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match instruction_3.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) || (((((match instruction_3.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(_, source_ml_source2_data_property_value31985, _, _) => {
                                                Some((source_ml_source2_data_property_value31985.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml_string::Text,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone()) != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121])))) || ((((match instruction_3.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(source_ml_source2_data_property_value31987, _, _, _) => {
                                                Some((source_ml_source2_data_property_value31987,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != destination_3)) {
                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 101, 120, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                        } else {
                                            {
                                                let error_3 = self::read_java_script_array_iterator_state_error(heap.clone(), state_2.clone());
                                                if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                    error_3.clone()
                                                } else {
                                                    if (!(self::test_java_script_semantic_same_value(shared_constructor_field1_3.clone(), ({
                                                        let source_ml_source2_array_read_value31990 = shared_constructor_field9.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31991 = 0f64;
                                                            { let ml_reference_array1975 = source_ml_source2_array_read_value31990.clone(); let ml_reference_position1975 = source_ml_source2_array_read_index31991; let ml_reference_projection1975 = self::ml_array::optional_at(&*ml_reference_array1975.borrow(), self::ml_array::number_index(ml_reference_position1975)); ml_reference_projection1975 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field2_3.clone(), match program.clone() {
                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value31993, _, _, _) => {
                                                            let source_ml_source2_data_property_value31993 = *source_ml_source2_data_property_value31993;
                                                            source_ml_source2_data_property_value31993.clone()
                                                        }
                                                    }))) {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 101, 120, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108])
                                                    } else {
                                                        if (suspended && match (match state_2.clone() {
                                                            self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value31994) => {
                                                                ((source_ml_source2_data_property_value31994.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,))).0).clone()
                                                            }
                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 110, 101, 120, 116, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])
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
                        self::JavaScriptCallPending::WaitCollectionIteratorResult(
                            destination_4,
                            realm_4,
                            state_3,
                        ) => {
                            let state_3 = *state_3;
                            match state_3.clone() {
                                self::JavaScriptCollectionIteratorResultState::CollectionIteratorResultState(_, shared_constructor_field1_4, shared_constructor_field2_4, shared_constructor_field3_3, shared_constructor_field4_3, _, _, _) => {
                                    let shared_constructor_field1_4 = *shared_constructor_field1_4;
                                    let shared_constructor_field3_3 = *shared_constructor_field3_3;
                                    let shared_constructor_field4_3 = *shared_constructor_field4_3;
                                    {
                                        let instruction_4 = {
                                            let source_ml_source2_array_read_value31997 = shared_constructor_field12.clone();
                                            {
                                                let source_ml_source2_array_read_index31998 = shared_constructor_field8;
                                                { let ml_reference_array1976 = source_ml_source2_array_read_value31997.clone(); let ml_reference_position1976 = source_ml_source2_array_read_index31998; let ml_reference_projection1976 = self::ml_array::optional_at(&*ml_reference_array1976.borrow(), self::ml_array::number_index(ml_reference_position1976)); ml_reference_projection1976 }
                                            }
                                        };
                                        if ((((!(self::test_java_script_collection_iterator_intrinsic_mode(&(shared_constructor_field11))) || (shared_constructor_field10 != realm_4)) || match (instruction_4.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match instruction_4.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) || ((((match instruction_4.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::CollectionIteratorResult(source_ml_source2_data_property_value32007, _, _, _) => {
                                                Some((source_ml_source2_data_property_value32007,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != destination_4)) {
                                            self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                        } else {
                                            {
                                                let error_4 = self::read_java_script_collection_iterator_result_state_error(heap.clone(), state_3.clone());
                                                if (error_4 != (self::ml_string::Text::from_units(vec![]))) {
                                                    error_4.clone()
                                                } else {
                                                    if ((shared_constructor_field2_4 != ((((match instruction_4.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::CollectionIteratorResult(_, source_ml_source2_data_property_value32011, _, _) => {
                                                            Some((source_ml_source2_data_property_value32011.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::ml_string::Text,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone())) || !(self::test_java_script_semantic_same_value(shared_constructor_field1_4.clone(), ({
                                                        let source_ml_source2_array_read_value32014 = shared_constructor_field9.clone();
                                                        {
                                                            let source_ml_source2_array_read_index32015 = 0f64;
                                                            { let ml_reference_array1977 = source_ml_source2_array_read_value32014.clone(); let ml_reference_position1977 = source_ml_source2_array_read_index32015; let ml_reference_projection1977 = self::ml_array::optional_at(&*ml_reference_array1977.borrow(), self::ml_array::number_index(ml_reference_position1977)); ml_reference_projection1977 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) {
                                                        self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 111, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 98, 114, 97, 110, 100])
                                                    } else {
                                                        if (!(self::test_java_script_semantic_same_value(shared_constructor_field3_3.clone(), ({
                                                            let source_ml_source2_array_read_value32019 = shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_array_read_index32020 = ((match instruction_4.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, source_ml_source2_data_property_value32018, _) => {
                                                                        Some((source_ml_source2_data_property_value32018,))
                                                                    }
                                                                    _ => {
                                                                        None::<(f64,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0;
                                                                { let ml_reference_array1978 = source_ml_source2_array_read_value32019.clone(); let ml_reference_position1978 = source_ml_source2_array_read_index32020; let ml_reference_projection1978 = self::ml_array::optional_at(&*ml_reference_array1978.borrow(), self::ml_array::number_index(ml_reference_position1978)); ml_reference_projection1978 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field4_3.clone(), ({
                                                            let source_ml_source2_array_read_value32024 = shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_array_read_index32025 = ((match instruction_4.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, source_ml_source2_data_property_value32023) => {
                                                                        Some((source_ml_source2_data_property_value32023,))
                                                                    }
                                                                    _ => {
                                                                        None::<(f64,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0;
                                                                { let ml_reference_array1979 = source_ml_source2_array_read_value32024.clone(); let ml_reference_position1979 = source_ml_source2_array_read_index32025; let ml_reference_projection1979 = self::ml_array::optional_at(&*ml_reference_array1979.borrow(), self::ml_array::number_index(ml_reference_position1979)); ml_reference_projection1979 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) {
                                                            self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115])
                                                        } else {
                                                            if suspended {
                                                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 104, 105, 100, 101, 32, 97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 115, 111, 117, 114, 99, 101, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108])
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
                        _ => self::ml_string::Text::from_units(vec![]),
                    }
                }
            }
        }
    }
}
