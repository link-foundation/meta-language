// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationCall;
use super::JavaScriptArrayCreationFallback;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallCompletion;
use super::JavaScriptCallCompletionTransition;
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptConstructionState;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorSelectedCall;
use super::JavaScriptIteratorThrowResolution;
use super::JavaScriptIteratorThrowTransition;
use super::JavaScriptObjectFromEntriesCall;
use super::JavaScriptObjectFromEntriesErrorRequest;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptObjectFromEntriesTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497;
use super::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result;
use super::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Result;
use super::finish_java_script_array_iterator_cursor;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::resume_java_script_array_creation_call;
use super::resume_java_script_call_iterator_throw;
use super::resume_java_script_object_from_entries_call;
use super::retain_java_script_call_argument_lists;
use super::source_ml_source1_temporary_unwind_java_script_call_completion_loop1498;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_unwind_java_script_call_completion_loop1496(
    mut completion: self::JavaScriptCallCompletion,
    mut frames: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFrame>>>,
    mut heap: self::JavaScriptOrdinaryHeap,
) -> self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result {
    loop {
        return if (({
            let ml_reference_array3265 = frames.clone();
            let ml_reference_projection3265 = ml_reference_array3265.borrow().len() as f64;
            ml_reference_projection3265
        }) > (0f64))
        {
            {
                let frame = {
                    let source_ml_source2_array_read_index42462 = ({
                        let ml_reference_array3266 = frames.clone();
                        let ml_reference_projection3266 =
                            ml_reference_array3266.borrow().len() as f64;
                        ml_reference_projection3266
                    } - 1f64);
                    {
                        let ml_reference_array3267 = frames.clone();
                        let ml_reference_position3267 = source_ml_source2_array_read_index42462;
                        let ml_reference_projection3267 = self::ml_array::optional_at(
                            &*ml_reference_array3267.borrow(),
                            self::ml_array::number_index(ml_reference_position3267),
                        );
                        ml_reference_projection3267
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Return(Box::new(self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 117, 110, 119, 105, 110, 100, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 99, 107]))))
                } else {
                    {
                        let source_ml_source2_s42654 = if ((match completion.clone() {
                            self::JavaScriptCallCompletion::CallCompletion(
                                source_ml_source2_data_property_value42656,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value42656.clone(),
                        })
                            == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119])))
                        {
                            {
                                let pending = match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value42657,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value42657 =
                                            *source_ml_source2_data_property_value42657;
                                        source_ml_source2_data_property_value42657.clone()
                                    }
                                };
                                match pending.clone() {
                                    self::JavaScriptCallPending::WaitIteratorProtocol(_, _, _, _, _, state) => {
                                        let state = *state;
                                        if match (match state.clone() {
                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value42666) => {
                                                ((source_ml_source2_data_property_value42666.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Next(Box::new(completion.clone()), frames.clone(), Box::new(heap.clone()), Box::new(frame.clone().expect("a proven present value")))
                                        } else {
                                            {
                                                let resolved = self::resume_java_script_call_iterator_throw(self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), frames.clone()), match completion.clone() {
                                                    self::JavaScriptCallCompletion::CallCompletion(_, source_ml_source2_data_property_value42667, _, _) => {
                                                        let source_ml_source2_data_property_value42667 = *source_ml_source2_data_property_value42667;
                                                        source_ml_source2_data_property_value42667.clone()
                                                    }
                                                });
                                                match resolved.clone() {
                                                    self::JavaScriptIteratorThrowResolution::IteratorThrowContinue(machine, value) => {
                                                        let machine = *machine;
                                                        let value = *value;
                                                        match machine.clone() {
                                                            self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                {
                                                                    let frames_2 = shared_constructor_field2.clone();
                                                                    {
                                                                        let heap_2 = shared_constructor_field1.clone();
                                                                        {
                                                                            let completion_2 = {
                                                                                let source_ml_source2_nominal_field42671 = match completion.clone() {
                                                                                    self::JavaScriptCallCompletion::CallCompletion(source_ml_source2_data_property_value42672, _, _, _) => {
                                                                                        source_ml_source2_data_property_value42672.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field42673 = match completion.clone() {
                                                                                        self::JavaScriptCallCompletion::CallCompletion(_, source_ml_source2_data_property_value42674, _, _) => {
                                                                                            let source_ml_source2_data_property_value42674 = *source_ml_source2_data_property_value42674;
                                                                                            source_ml_source2_data_property_value42674.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field42675 = match completion.clone() {
                                                                                            self::JavaScriptCallCompletion::CallCompletion(_, _, source_ml_source2_data_property_value42676, _) => {
                                                                                                source_ml_source2_data_property_value42676
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field42677 = match completion.clone() {
                                                                                                self::JavaScriptCallCompletion::CallCompletion(_, _, _, source_ml_source2_data_property_value42678) => {
                                                                                                    source_ml_source2_data_property_value42678
                                                                                                }
                                                                                            };
                                                                                            self::JavaScriptCallCompletion::CallCompletion(source_ml_source2_nominal_field42671.clone(), Box::new(value.clone()), source_ml_source2_nominal_field42675, source_ml_source2_nominal_field42677)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            };
                                                                            {
                                                                                let frame_2 = {
                                                                                    let source_ml_source2_array_read_index42681 = ({ let ml_reference_array3268 = frames_2.clone(); let ml_reference_projection3268 = ml_reference_array3268.borrow().len() as f64; ml_reference_projection3268 } - 1f64);
                                                                                    { let ml_reference_array3269 = frames_2.clone(); let ml_reference_position3269 = source_ml_source2_array_read_index42681; let ml_reference_projection3269 = self::ml_array::optional_at(&*ml_reference_array3269.borrow(), self::ml_array::number_index(ml_reference_position3269)); ml_reference_projection3269 }
                                                                                };
                                                                                if match (frame_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                    self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap_2.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 101, 114, 32, 102, 114, 97, 109, 101]))))
                                                                                } else {
                                                                                    self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Next(Box::new(completion_2.clone()), frames_2.clone(), Box::new(heap_2.clone()), Box::new(frame_2.clone().expect("a proven present value")))
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(transition) => {
                                                        let transition = *transition;
                                                        self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(match transition.clone() {
                                                            self::JavaScriptIteratorThrowTransition::MachineInvalid(heap_3, reason) => {
                                                                let heap_3 = *heap_3;
                                                                self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap_3.clone()), reason.clone())
                                                            }
                                                            self::JavaScriptIteratorThrowTransition::MachineContinue(machine_2) => {
                                                                let machine_2 = *machine_2;
                                                                self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(machine_2.clone()))
                                                            }
                                                        }))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitArrayCreation(destination, realm, pending_state, construction, _, _) => {
                                        let pending_state = *pending_state;
                                        if (match (match pending_state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value42682, _, _, _) => {
                                                ((source_ml_source2_data_property_value42682.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match (((construction.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Next(Box::new(completion.clone()), frames.clone(), Box::new(heap.clone()), Box::new(frame.clone().expect("a proven present value")))
                                        } else {
                                            {
                                                let resumed = self::resume_java_script_array_creation_call(heap.clone(), pending_state.clone(), self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(match completion.clone() {
                                                    self::JavaScriptCallCompletion::CallCompletion(_, source_ml_source2_data_property_value42683, _, _) => {
                                                        let source_ml_source2_data_property_value42683 = *source_ml_source2_data_property_value42683;
                                                        source_ml_source2_data_property_value42683.clone()
                                                    }
                                                })));
                                                match resumed.clone() {
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationContinue(heap_4, state_2) => {
                                                        let heap_4 = *heap_4;
                                                        let state_2 = *state_2;
                                                        {
                                                            let next = self::replace_java_script_call_frame(heap_4.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(state_2.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(heap_5, state_3, _, _, _) => {
                                                        let heap_5 = *heap_5;
                                                        let state_3 = *state_3;
                                                        {
                                                            let next_2 = self::replace_java_script_call_frame(heap_5.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(state_3.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next_2.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationCall(heap_6, state_4, _, _, _) => {
                                                        let heap_6 = *heap_6;
                                                        let state_4 = *state_4;
                                                        {
                                                            let next_3 = self::replace_java_script_call_frame(heap_6.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(state_4.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next_3.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationAllocate(heap_7, state_5, _) => {
                                                        let heap_7 = *heap_7;
                                                        let state_5 = *state_5;
                                                        {
                                                            let next_4 = self::replace_java_script_call_frame(heap_7.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(state_5.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next_4.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationPrimitiveGet(heap_8, state_6, _, _) => {
                                                        let heap_8 = *heap_8;
                                                        let state_6 = *state_6;
                                                        {
                                                            let next_5 = self::replace_java_script_call_frame(heap_8.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(state_6.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next_5.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationNeedError(heap_9, state_7, _, _) => {
                                                        let heap_9 = *heap_9;
                                                        let state_7 = *state_7;
                                                        {
                                                            let next_6 = self::replace_java_script_call_frame(heap_9.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(state_7.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next_6.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationCompleted(heap_10, _) => {
                                                        let heap_10 = *heap_10;
                                                        self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap_10.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 101, 97, 110, 117, 112, 32, 111, 114, 32, 102, 105, 110, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                    }
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationThrow(heap_11, value_2) => {
                                                        let heap_11 = *heap_11;
                                                        let value_2 = *value_2;
                                                        {
                                                            let frame_3 = {
                                                                let source_ml_source2_nominal_spread42729 = frame.clone().expect("a proven present value");
                                                                match source_ml_source2_nominal_spread42729.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                        let shared_constructor_field6 = *shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source2_nominal_field42730 = shared_constructor_field1_2.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field42732 = shared_constructor_field2_2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field42734 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field42736 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field42738 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field42740 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field42742 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field42744 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field42746 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field42748 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override42750 = self::JavaScriptCallPending::CallIdle;
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field42730.clone(), source_ml_source2_nominal_field42732, source_ml_source2_nominal_field42734.clone(), source_ml_source2_nominal_field42736.clone(), source_ml_source2_nominal_field42738.clone(), Box::new(source_ml_source2_nominal_override42750.clone()), source_ml_source2_nominal_field42742.clone(), source_ml_source2_nominal_field42744.clone(), source_ml_source2_nominal_field42746.clone(), source_ml_source2_nominal_field42748.clone())
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
                                                            };
                                                            {
                                                                let frames_3 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value42751 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value3270 = frames.borrow().clone(); ml_reference_value3270 }, 0f64, (-1f64))));
                                                                    {
                                                                        let source_ml_source2_concat_value42752 = std::rc::Rc::new(std::cell::RefCell::new(vec![frame_3.clone()]));
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), { let ml_reference_value3271 = source_ml_source2_concat_value42751.borrow().clone(); ml_reference_value3271 }), { let ml_reference_value3272 = source_ml_source2_concat_value42752.borrow().clone(); ml_reference_value3272 })
                                                                    }
                                                                }));
                                                                {
                                                                    let completion_3 = {
                                                                        let source_ml_source2_nominal_field42754 = match completion.clone() {
                                                                            self::JavaScriptCallCompletion::CallCompletion(source_ml_source2_data_property_value42755, _, _, _) => {
                                                                                source_ml_source2_data_property_value42755.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field42756 = match completion.clone() {
                                                                                self::JavaScriptCallCompletion::CallCompletion(_, source_ml_source2_data_property_value42757, _, _) => {
                                                                                    let source_ml_source2_data_property_value42757 = *source_ml_source2_data_property_value42757;
                                                                                    source_ml_source2_data_property_value42757.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field42758 = match completion.clone() {
                                                                                    self::JavaScriptCallCompletion::CallCompletion(_, _, source_ml_source2_data_property_value42759, _) => {
                                                                                        source_ml_source2_data_property_value42759
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field42760 = match completion.clone() {
                                                                                        self::JavaScriptCallCompletion::CallCompletion(_, _, _, source_ml_source2_data_property_value42761) => {
                                                                                            source_ml_source2_data_property_value42761
                                                                                        }
                                                                                    };
                                                                                    self::JavaScriptCallCompletion::CallCompletion(source_ml_source2_nominal_field42754.clone(), Box::new(value_2.clone()), source_ml_source2_nominal_field42758, source_ml_source2_nominal_field42760)
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Next(Box::new(completion_3.clone()), frames_3.clone(), Box::new(heap_11.clone()), Box::new(frame_3.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationError(heap_12, _, _) => {
                                                        let heap_12 = *heap_12;
                                                        self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap_12.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 101, 97, 110, 117, 112, 32, 111, 114, 32, 102, 105, 110, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                    }
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(heap_13, reason_2) => {
                                                        let heap_13 = *heap_13;
                                                        self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap_13.clone()), reason_2.clone())))
                                                    }
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationResourceLimit(heap_14, state_8, source_ml_source2_f42698) => {
                                                        let heap_14 = *heap_14;
                                                        let state_8 = *state_8;
                                                        {
                                                            let next_7 = self::replace_java_script_call_frame(heap_14.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitArrayCreation(destination, realm.clone(), Box::new(state_8.clone()), None::<(Option<self::JavaScriptConstructionState>,)>, None::<(Option<self::JavaScriptArrayCreationFallback>,)>, None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineResourceLimit(Box::new(next_7.clone()), source_ml_source2_f42698.clone())))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitObjectFromEntries(destination_2, realm_2, pending_state_2, _) => {
                                        let pending_state_2 = *pending_state_2;
                                        if match (match pending_state_2.clone() {
                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value42768, _, _, _) => {
                                                ((source_ml_source2_data_property_value42768.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Next(Box::new(completion.clone()), frames.clone(), Box::new(heap.clone()), Box::new(frame.clone().expect("a proven present value")))
                                        } else {
                                            {
                                                let resumed_2 = self::resume_java_script_object_from_entries_call(heap.clone(), pending_state_2.clone(), self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(match completion.clone() {
                                                    self::JavaScriptCallCompletion::CallCompletion(_, source_ml_source2_data_property_value42769, _, _) => {
                                                        let source_ml_source2_data_property_value42769 = *source_ml_source2_data_property_value42769;
                                                        source_ml_source2_data_property_value42769.clone()
                                                    }
                                                })));
                                                match resumed_2.clone() {
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(heap_15, state_9) => {
                                                        let heap_15 = *heap_15;
                                                        let state_9 = *state_9;
                                                        {
                                                            let next_8 = self::replace_java_script_call_frame(heap_15.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination_2, realm_2.clone(), Box::new(state_9.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next_8.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCall(heap_16, state_10, _, _, _) => {
                                                        let heap_16 = *heap_16;
                                                        let state_10 = *state_10;
                                                        {
                                                            let next_9 = self::replace_java_script_call_frame(heap_16.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination_2, realm_2.clone(), Box::new(state_10.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next_9.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesAllocate(heap_17, state_11, _) => {
                                                        let heap_17 = *heap_17;
                                                        let state_11 = *state_11;
                                                        {
                                                            let next_10 = self::replace_java_script_call_frame(heap_17.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination_2, realm_2.clone(), Box::new(state_11.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next_10.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesPrimitiveGet(heap_18, state_12, _, _) => {
                                                        let heap_18 = *heap_18;
                                                        let state_12 = *state_12;
                                                        {
                                                            let next_11 = self::replace_java_script_call_frame(heap_18.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination_2, realm_2.clone(), Box::new(state_12.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next_11.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesNeedError(heap_19, state_13, _, _) => {
                                                        let heap_19 = *heap_19;
                                                        let state_13 = *state_13;
                                                        {
                                                            let next_12 = self::replace_java_script_call_frame(heap_19.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination_2, realm_2.clone(), Box::new(state_13.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(next_12.clone()))))
                                                        }
                                                    }
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesCompleted(heap_20, _) => {
                                                        let heap_20 = *heap_20;
                                                        self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap_20.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 116, 104, 114, 111, 119, 32, 114, 101, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 101, 97, 110, 117, 112, 32, 111, 114, 32, 102, 105, 110, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                    }
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesThrow(heap_21, value_3) => {
                                                        let heap_21 = *heap_21;
                                                        let value_3 = *value_3;
                                                        {
                                                            let frame_4 = {
                                                                let source_ml_source2_nominal_spread42807 = frame.clone().expect("a proven present value");
                                                                match source_ml_source2_nominal_spread42807.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_3, shared_constructor_field2_3, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2, shared_constructor_field7_2, shared_constructor_field8_2, shared_constructor_field9_2, shared_constructor_field10_2) => {
                                                                        let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                                                        {
                                                                            let source_ml_source2_nominal_field42808 = shared_constructor_field1_3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field42810 = shared_constructor_field2_3;
                                                                                {
                                                                                    let source_ml_source2_nominal_field42812 = shared_constructor_field3_2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field42814 = shared_constructor_field4_2.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field42816 = shared_constructor_field5_2.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field42818 = shared_constructor_field6_2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field42820 = shared_constructor_field7_2.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field42822 = shared_constructor_field8_2.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field42824 = shared_constructor_field9_2.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field42826 = shared_constructor_field10_2.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override42828 = self::JavaScriptCallPending::CallIdle;
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field42808.clone(), source_ml_source2_nominal_field42810, source_ml_source2_nominal_field42812.clone(), source_ml_source2_nominal_field42814.clone(), source_ml_source2_nominal_field42816.clone(), Box::new(source_ml_source2_nominal_override42828.clone()), source_ml_source2_nominal_field42820.clone(), source_ml_source2_nominal_field42822.clone(), source_ml_source2_nominal_field42824.clone(), source_ml_source2_nominal_field42826.clone())
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
                                                            };
                                                            {
                                                                let frames_4 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                    let source_ml_source2_concat_value42829 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value3273 = frames.borrow().clone(); ml_reference_value3273 }, 0f64, (-1f64))));
                                                                    {
                                                                        let source_ml_source2_concat_value42830 = std::rc::Rc::new(std::cell::RefCell::new(vec![frame_4.clone()]));
                                                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), { let ml_reference_value3274 = source_ml_source2_concat_value42829.borrow().clone(); ml_reference_value3274 }), { let ml_reference_value3275 = source_ml_source2_concat_value42830.borrow().clone(); ml_reference_value3275 })
                                                                    }
                                                                }));
                                                                {
                                                                    let completion_4 = {
                                                                        let source_ml_source2_nominal_field42832 = match completion.clone() {
                                                                            self::JavaScriptCallCompletion::CallCompletion(source_ml_source2_data_property_value42833, _, _, _) => {
                                                                                source_ml_source2_data_property_value42833.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field42834 = match completion.clone() {
                                                                                self::JavaScriptCallCompletion::CallCompletion(_, source_ml_source2_data_property_value42835, _, _) => {
                                                                                    let source_ml_source2_data_property_value42835 = *source_ml_source2_data_property_value42835;
                                                                                    source_ml_source2_data_property_value42835.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field42836 = match completion.clone() {
                                                                                    self::JavaScriptCallCompletion::CallCompletion(_, _, source_ml_source2_data_property_value42837, _) => {
                                                                                        source_ml_source2_data_property_value42837
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field42838 = match completion.clone() {
                                                                                        self::JavaScriptCallCompletion::CallCompletion(_, _, _, source_ml_source2_data_property_value42839) => {
                                                                                            source_ml_source2_data_property_value42839
                                                                                        }
                                                                                    };
                                                                                    self::JavaScriptCallCompletion::CallCompletion(source_ml_source2_nominal_field42832.clone(), Box::new(value_3.clone()), source_ml_source2_nominal_field42836, source_ml_source2_nominal_field42838)
                                                                                }
                                                                            }
                                                                        }
                                                                    };
                                                                    self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Next(Box::new(completion_4.clone()), frames_4.clone(), Box::new(heap_21.clone()), Box::new(frame_4.clone()))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesError(heap_22, _, _) => {
                                                        let heap_22 = *heap_22;
                                                        self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap_22.clone()), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 116, 104, 114, 111, 119, 32, 114, 101, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 101, 97, 110, 117, 112, 32, 111, 114, 32, 102, 105, 110, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                    }
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(heap_23, reason_3) => {
                                                        let heap_23 = *heap_23;
                                                        self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap_23.clone()), reason_3.clone())))
                                                    }
                                                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesResourceLimit(heap_24, state_14, source_ml_source2_f42781) => {
                                                        let heap_24 = *heap_24;
                                                        let state_14 = *state_14;
                                                        {
                                                            let next_13 = self::replace_java_script_call_frame(heap_24.clone(), frames.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectFromEntries(destination_2, realm_2.clone(), Box::new(state_14.clone()), None::<(Option<self::JavaScriptObjectFromEntriesErrorRequest>,)>)));
                                                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(Box::new(self::JavaScriptCallCompletionTransition::MachineResourceLimit(Box::new(next_13.clone()), source_ml_source2_f42781.clone())))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Next(Box::new(completion.clone()), frames.clone(), Box::new(heap.clone()), Box::new(frame.clone().expect("a proven present value")))
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Next(Box::new(completion.clone()), frames.clone(), Box::new(heap.clone()), Box::new(frame.clone().expect("a proven present value")))
                        };
                        match source_ml_source2_s42654.clone() {
                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Next(completion_5, frames_5, heap_25, frame_5) => {
                                let completion_5 = *completion_5;
                                let heap_25 = *heap_25;
                                let frame_5 = *frame_5;
                                match frame_5.clone() {
                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field5_3, shared_constructor_field6_3, shared_constructor_field7_3, shared_constructor_field8_3, shared_constructor_field9_3, shared_constructor_field10_3, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14) => {
                                        let shared_constructor_field10_3 = *shared_constructor_field10_3;
                                        match completion_5.clone() {
                                            self::JavaScriptCallCompletion::CallCompletion(shared_constructor_field1_4, shared_constructor_field2_4, shared_constructor_field3_3, shared_constructor_field4_3) => {
                                                let shared_constructor_field2_4 = *shared_constructor_field2_4;
                                                {
                                                    let handlers = shared_constructor_field9_3.clone();
                                                    if ((shared_constructor_field1_4 == (self::ml_string::Text::from_units(vec![106, 117, 109, 112]))) && (shared_constructor_field4_3 > ({ let ml_reference_array3276 = handlers.clone(); let ml_reference_projection3276 = ml_reference_array3276.borrow().len() as f64; ml_reference_projection3276 }))) {
                                                        self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Return(Box::new(self::JavaScriptCallCompletionTransition::MachineInvalid(Box::new(heap_25.clone()), self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 106, 117, 109, 112, 32, 99, 97, 110, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 97, 110, 32, 101, 110, 99, 108, 111, 115, 105, 110, 103, 32, 104, 97, 110, 100, 108, 101, 114, 32, 100, 101, 112, 116, 104]))))
                                                    } else {
                                                        {
                                                            let source_ml_source2_s42851 = self::source_ml_source1_temporary_unwind_java_script_call_completion_loop1498(completion_5.clone(), frames_5.clone(), heap_25.clone(), frame_5.clone(), handlers.clone());
                                                            match source_ml_source2_s42851.clone() {
                                                                self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Done(handlers_2) => {
                                                                    {
                                                                        let argument_lists = self::retain_java_script_call_argument_lists((match frame_5.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, source_ml_source2_data_property_value42852, _, _) => {
                                                                                ((source_ml_source2_data_property_value42852.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,))).0).clone()
                                                                            }
                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallArgumentList>::new()))), { let ml_reference_array3277 = handlers_2.clone(); let ml_reference_projection3277 = ml_reference_array3277.borrow().len() as f64; ml_reference_projection3277 });
                                                                        {
                                                                            let current = self::replace_java_script_call_frame(heap_25.clone(), frames_5.clone(), {
                                                                                let source_ml_source2_nominal_field42854 = shared_constructor_field5_3.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field42856 = shared_constructor_field6_3;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field42858 = shared_constructor_field7_3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field42860 = shared_constructor_field8_3.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field42862 = shared_constructor_field9_3.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field42864 = shared_constructor_field10_3.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field42866 = shared_constructor_field11.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field42868 = shared_constructor_field12.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field42870 = shared_constructor_field13.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field42872 = shared_constructor_field14.clone();
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field42854.clone(), source_ml_source2_nominal_field42856, source_ml_source2_nominal_field42858.clone(), source_ml_source2_nominal_field42860.clone(), handlers_2.clone(), Box::new(source_ml_source2_nominal_field42864.clone()), source_ml_source2_nominal_field42866.clone(), Some((Some(argument_lists.clone()),)), source_ml_source2_nominal_field42870.clone(), source_ml_source2_nominal_field42872.clone())
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            });
                                                                            if (shared_constructor_field1_4 == (self::ml_string::Text::from_units(vec![106, 117, 109, 112]))) {
                                                                                self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Return(Box::new(self::JavaScriptCallCompletionTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_25.clone(), frames_5.clone(), {
                                                                                    let source_ml_source2_nominal_field42880 = shared_constructor_field5_3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field42882 = shared_constructor_field6_3;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field42884 = shared_constructor_field7_3.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field42886 = shared_constructor_field8_3.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field42888 = shared_constructor_field9_3.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field42890 = shared_constructor_field10_3.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field42892 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field42894 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field42896 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field42898 = shared_constructor_field14.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override42900 = shared_constructor_field3_3;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override42901 = self::JavaScriptCallPending::CallIdle;
                                                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field42880.clone(), source_ml_source2_nominal_override42900, source_ml_source2_nominal_field42884.clone(), source_ml_source2_nominal_field42886.clone(), handlers_2.clone(), Box::new(source_ml_source2_nominal_override42901.clone()), source_ml_source2_nominal_field42892.clone(), Some((Some(argument_lists.clone()),)), source_ml_source2_nominal_field42896.clone(), source_ml_source2_nominal_field42898.clone())
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
                                                                                })))))
                                                                            } else {
                                                                                if (shared_constructor_field1_4 == (self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]))) {
                                                                                    self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Return(Box::new(self::JavaScriptCallCompletionTransition::CallReturn(Box::new(current.clone()), Box::new(shared_constructor_field2_4.clone()))))
                                                                                } else {
                                                                                    {
                                                                                        let final_pending = shared_constructor_field10_3.clone();
                                                                                        {
                                                                                            let heap_27 = match final_pending.clone() {
                                                                                                self::JavaScriptCallPending::WaitArrayIteratorNext(_, _, state_15) => {
                                                                                                    let state_15 = *state_15;
                                                                                                    {
                                                                                                        let heap_26 = self::finish_java_script_array_iterator_cursor(heap_25.clone(), state_15.clone(), true);
                                                                                                        heap_26.clone()
                                                                                                    }
                                                                                                }
                                                                                                _ => {
                                                                                                    heap_25.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let frames_6 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value3278 = frames_5.borrow().clone(); ml_reference_value3278 }, 0f64, (-1f64))));
                                                                                                {
                                                                                                    (completion, frames, heap) = (completion_5.clone(), frames_6.clone(), heap_27.clone());
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
                                                                self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1498Return(source_ml_source1_temporary_result1498) => {
                                                                    let source_ml_source1_temporary_result1498 = *source_ml_source1_temporary_result1498;
                                                                    self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Return(Box::new(source_ml_source1_temporary_result1498.clone()))
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
                            self::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497::MlSource1TemporaryUnwindJavaScriptCallCompletionJoin1497Return(source_ml_source1_temporary_result1498_2) => {
                                let source_ml_source1_temporary_result1498_2 = *source_ml_source1_temporary_result1498_2;
                                self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Return(Box::new(source_ml_source1_temporary_result1498_2.clone()))
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Result::MlSource1TemporaryUnwindJavaScriptCallCompletionLoop1496Done(Box::new(completion.clone()), Box::new(heap.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRequestJavaScriptCallErrorJoin1499 {
    MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Next(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptCallFrame>,
    ),
    MlSource1TemporaryRequestJavaScriptCallErrorJoin1499Return(
        Box<self::JavaScriptCallMachineTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPushJavaScriptCallLoop1502Result {
    MlSource1TemporaryPushJavaScriptCallLoop1502Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_call_loop1503(
    mut argument_slots: f64,
    mut source_ml_source1_temporary_values1501: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>,
    >,
    mut source_ml_source1_temporary_index1501: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1501
            < ({
                let ml_reference_array3279 = source_ml_source1_temporary_values1501.clone();
                let ml_reference_projection3279 =
                    self::ml::Big::from_u128(ml_reference_array3279.borrow().len() as u128);
                ml_reference_projection3279
            })) {
            {
                let list = {
                    let ml_reference_array3280 = source_ml_source1_temporary_values1501.clone();
                    let ml_reference_position3280 = source_ml_source1_temporary_index1501.clone();
                    let ml_reference_projection3280 = self::ml_array::at(
                        &*ml_reference_array3280.borrow(),
                        ml_reference_position3280.to_index(),
                    );
                    ml_reference_projection3280
                };
                {
                    let argument_slots_2 = (argument_slots + {
                        let ml_reference_array3281 = match list.clone() {
                            self::JavaScriptCallArgumentList::CallArgumentList(
                                _,
                                _,
                                source_ml_source2_data_property_value42912,
                            ) => source_ml_source2_data_property_value42912.clone(),
                        };
                        let ml_reference_projection3281 =
                            ml_reference_array3281.borrow().len() as f64;
                        ml_reference_projection3281
                    });
                    {
                        let source_ml_source1_temporary_index1501_2 =
                            source_ml_source1_temporary_index1501.add(&self::ml::Big::from_i128(1));
                        {
                            (argument_slots, source_ml_source1_temporary_index1501) = (
                                argument_slots_2,
                                source_ml_source1_temporary_index1501_2.clone(),
                            );
                            continue;
                        }
                    }
                }
            }
        } else {
            argument_slots
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_push_java_script_call_loop1502(
    mut slots: f64,
    mut argument_slots: f64,
    mut source_ml_source1_temporary_values1500: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallFrame>>,
    >,
    mut source_ml_source1_temporary_index1500: self::ml::Big,
) -> self::MlSource1TemporaryPushJavaScriptCallLoop1502Result {
    loop {
        return if (source_ml_source1_temporary_index1500
            < ({
                let ml_reference_array3282 = source_ml_source1_temporary_values1500.clone();
                let ml_reference_projection3282 =
                    self::ml::Big::from_u128(ml_reference_array3282.borrow().len() as u128);
                ml_reference_projection3282
            })) {
            {
                let active = {
                    let ml_reference_array3283 = source_ml_source1_temporary_values1500.clone();
                    let ml_reference_position3283 = source_ml_source1_temporary_index1500.clone();
                    let ml_reference_projection3283 = self::ml_array::at(
                        &*ml_reference_array3283.borrow(),
                        ml_reference_position3283.to_index(),
                    );
                    ml_reference_projection3283
                };
                match active.clone() {
                    self::JavaScriptCallFrame::CallFrame(
                        _,
                        _,
                        shared_constructor_field1,
                        shared_constructor_field2,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let slots_2 = (slots + {
                            let ml_reference_array3284 = shared_constructor_field1.clone();
                            let ml_reference_projection3284 =
                                ml_reference_array3284.borrow().len() as f64;
                            ml_reference_projection3284
                        });
                        {
                            let argument_slots_2 = (argument_slots + {
                                let ml_reference_array3285 = shared_constructor_field2.clone();
                                let ml_reference_projection3285 =
                                    ml_reference_array3285.borrow().len() as f64;
                                ml_reference_projection3285
                            });
                            {
                                let source_ml_source1_temporary_values1501 =
                                    (match active.clone() {
                                        self::JavaScriptCallFrame::CallFrame(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value42917,
                                            _,
                                            _,
                                        ) => ((source_ml_source2_data_property_value42917
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<
                                                    std::rc::Rc<
                                                        std::cell::RefCell<
                                                            Vec<self::JavaScriptCallArgumentList>,
                                                        >,
                                                    >,
                                                >,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .unwrap_or_else(|| {
                                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                            self::JavaScriptCallArgumentList,
                                        >::new(
                                        )))
                                    });
                                {
                                    let source_ml_source1_temporary_index1501 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let argument_slots_3 = self::source_ml_source1_temporary_push_java_script_call_loop1503(argument_slots_2, source_ml_source1_temporary_values1501.clone(), source_ml_source1_temporary_index1501.clone());
                                        {
                                            let source_ml_source1_temporary_index1500_2 =
                                                source_ml_source1_temporary_index1500
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (
                                                    slots,
                                                    argument_slots,
                                                    source_ml_source1_temporary_index1500,
                                                ) = (
                                                    slots_2,
                                                    argument_slots_3,
                                                    source_ml_source1_temporary_index1500_2.clone(),
                                                );
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
            self::MlSource1TemporaryPushJavaScriptCallLoop1502Result::MlSource1TemporaryPushJavaScriptCallLoop1502Done(slots, argument_slots)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFinishJavaScriptCallReturnJoin1508 {
    MlSource1TemporaryFinishJavaScriptCallReturnJoin1508Next(Box<self::JavaScriptSemanticValue>),
    MlSource1TemporaryFinishJavaScriptCallReturnJoin1508Return(
        Box<self::JavaScriptCallMachineTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFinishJavaScriptCallReturnJoin1509 {
    MlSource1TemporaryFinishJavaScriptCallReturnJoin1509Next(Box<self::JavaScriptSemanticValue>),
    MlSource1TemporaryFinishJavaScriptCallReturnJoin1509Return(
        Box<self::JavaScriptCallMachineTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFinishJavaScriptCallReturnJoin1507 {
    MlSource1TemporaryFinishJavaScriptCallReturnJoin1507Next(Box<self::JavaScriptSemanticValue>),
    MlSource1TemporaryFinishJavaScriptCallReturnJoin1507Return(
        Box<self::JavaScriptCallMachineTransition>,
    ),
}
