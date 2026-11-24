// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArgumentRestTransition;
use super::JavaScriptCallFrame;
use super::JavaScriptCallIteratorRegion;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorProtocolTransition;
use super::JavaScriptIteratorRegionsUpdate;
use super::JavaScriptIteratorThrowResolution;
use super::JavaScriptIteratorThrowTransition;
use super::JavaScriptObjectEnumerationTransition;
use super::JavaScriptObjectIntegrityTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::remove_java_script_iterator_region;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::resume_java_script_iterator_protocol_call;
use super::throw_java_script_call_value;
use super::update_java_script_iterator_region;
use super::write_java_script_call_iterator_result;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_iterator_throw(
    machine: self::JavaScriptCallMachine,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptIteratorThrowResolution {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field11 = *shared_constructor_field11;
            {
                let frame = {
                    let source_ml_source2_array_read_value38148 =
                        shared_constructor_field12.clone();
                    {
                        let source_ml_source2_array_read_index38149 = ({
                            let ml_reference_array2469 = shared_constructor_field12.clone();
                            let ml_reference_projection2469 =
                                ml_reference_array2469.borrow().len() as f64;
                            ml_reference_projection2469
                        } - 1f64);
                        {
                            let ml_reference_array2470 =
                                source_ml_source2_array_read_value38148.clone();
                            let ml_reference_position2470 = source_ml_source2_array_read_index38149;
                            let ml_reference_projection2470 = self::ml_array::optional_at(
                                &*ml_reference_array2470.borrow(),
                                self::ml_array::number_index(ml_reference_position2470),
                            );
                            ml_reference_projection2470
                        }
                    }
                };
                if match (frame.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(
                        self::JavaScriptIteratorThrowTransition::MachineInvalid(
                            Box::new(shared_constructor_field11.clone()),
                            self::ml_string::Text::from_units(vec![
                                105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119,
                                32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                                99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101,
                                100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97,
                                116, 105, 111, 110,
                            ]),
                        ),
                    ))
                } else {
                    {
                        let pending = match frame.clone().expect("a proven present value") {
                            self::JavaScriptCallFrame::CallFrame(
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value38151,
                                _,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value38151 =
                                    *source_ml_source2_data_property_value38151;
                                source_ml_source2_data_property_value38151.clone()
                            }
                        };
                        match pending.clone() {
                            self::JavaScriptCallPending::WaitIteratorProtocol(
                                destination,
                                done,
                                slot,
                                operation,
                                _,
                                pending_state,
                            ) => {
                                let pending_state = *pending_state;
                                {
                                    let transition = self::resume_java_script_iterator_protocol_call(shared_constructor_field11.clone(), pending_state.clone(), self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(value.clone())));
                                    {
                                        let regions = (match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, source_ml_source2_data_property_value38154, _, _, _) => {
                                                ((source_ml_source2_data_property_value38154.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallIteratorRegion>>>>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallIteratorRegion>::new())));
                                        if (match transition.clone() {
                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(_, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        } && match transition.clone() {
                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(_, _, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) {
                                            self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(self::JavaScriptIteratorThrowTransition::MachineInvalid(Box::new((((match transition.clone() {
                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(source_ml_source2_data_property_value38161, _) => {
                                                    let source_ml_source2_data_property_value38161 = *source_ml_source2_data_property_value38161;
                                                    Some((source_ml_source2_data_property_value38161.clone(),))
                                                }
                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(source_ml_source2_data_property_value38162, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value38162 = *source_ml_source2_data_property_value38162;
                                                    Some((source_ml_source2_data_property_value38162.clone(),))
                                                }
                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(source_ml_source2_data_property_value38163, _, _, _) => {
                                                    let source_ml_source2_data_property_value38163 = *source_ml_source2_data_property_value38163;
                                                    Some((source_ml_source2_data_property_value38163.clone(),))
                                                }
                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolAcquired(source_ml_source2_data_property_value38164, _) => {
                                                    let source_ml_source2_data_property_value38164 = *source_ml_source2_data_property_value38164;
                                                    Some((source_ml_source2_data_property_value38164.clone(),))
                                                }
                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(source_ml_source2_data_property_value38165, _, _, _) => {
                                                    let source_ml_source2_data_property_value38165 = *source_ml_source2_data_property_value38165;
                                                    Some((source_ml_source2_data_property_value38165.clone(),))
                                                }
                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolError(source_ml_source2_data_property_value38166, _, _, _) => {
                                                    let source_ml_source2_data_property_value38166 = *source_ml_source2_data_property_value38166;
                                                    Some((source_ml_source2_data_property_value38166.clone(),))
                                                }
                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(source_ml_source2_data_property_value38167, _) => {
                                                    let source_ml_source2_data_property_value38167 = *source_ml_source2_data_property_value38167;
                                                    Some((source_ml_source2_data_property_value38167.clone(),))
                                                }
                                                _ => {
                                                    None::<(self::JavaScriptOrdinaryHeap,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone()), match transition.clone() {
                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(_, reason) => {
                                                    reason.clone()
                                                }
                                                _ => {
                                                    self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 111, 114, 32, 115, 117, 112, 112, 114, 101, 115, 115, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110])
                                                }
                                            })))
                                        } else {
                                            {
                                                let update = if (operation == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                                                    self::remove_java_script_iterator_region((((match transition.clone() {
                                                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(source_ml_source2_data_property_value38169, _, _) => {
                                                            let source_ml_source2_data_property_value38169 = *source_ml_source2_data_property_value38169;
                                                            Some((source_ml_source2_data_property_value38169.clone(),))
                                                        }
                                                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(source_ml_source2_data_property_value38170, _, _) => {
                                                            let source_ml_source2_data_property_value38170 = *source_ml_source2_data_property_value38170;
                                                            Some((source_ml_source2_data_property_value38170.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::JavaScriptOrdinaryHeap,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone(), regions.clone(), slot)
                                                } else {
                                                    if (operation == (self::ml_string::Text::from_units(vec![115, 116, 101, 112]))) {
                                                        self::update_java_script_iterator_region((((match transition.clone() {
                                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(source_ml_source2_data_property_value38171, _, _) => {
                                                                let source_ml_source2_data_property_value38171 = *source_ml_source2_data_property_value38171;
                                                                Some((source_ml_source2_data_property_value38171.clone(),))
                                                            }
                                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(source_ml_source2_data_property_value38172, _, _) => {
                                                                let source_ml_source2_data_property_value38172 = *source_ml_source2_data_property_value38172;
                                                                Some((source_ml_source2_data_property_value38172.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::JavaScriptOrdinaryHeap,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone(), regions.clone(), slot, match transition.clone() {
                                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(_, state, _) => {
                                                                let state = *state;
                                                                match state.clone() {
                                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value38175, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value38175 = *source_ml_source2_data_property_value38175;
                                                                        source_ml_source2_data_property_value38175.clone()
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                match pending_state.clone() {
                                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, source_ml_source2_data_property_value38176, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value38176 = *source_ml_source2_data_property_value38176;
                                                                        source_ml_source2_data_property_value38176.clone()
                                                                    }
                                                                }
                                                            }
                                                        })
                                                    } else {
                                                        self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions.clone())
                                                    }
                                                };
                                                match update.clone() {
                                                    self::JavaScriptIteratorRegionsUpdate::IteratorRegionsUpdated(regions_2) => {
                                                        {
                                                            let next = {
                                                                let source_ml_source2_nominal_spread38177 = frame.clone().expect("a proven present value");
                                                                match source_ml_source2_nominal_spread38177.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                        let shared_constructor_field6 = *shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source2_nominal_field38178 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field38180 = shared_constructor_field2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field38182 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field38184 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field38186 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field38188 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field38190 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field38192 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field38194 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field38196 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override38199 = self::JavaScriptCallPending::CallIdle;
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field38178.clone(), source_ml_source2_nominal_field38180, source_ml_source2_nominal_field38182.clone(), source_ml_source2_nominal_field38184.clone(), source_ml_source2_nominal_field38186.clone(), Box::new(source_ml_source2_nominal_override38199.clone()), Some((Some(regions_2.clone()),)), source_ml_source2_nominal_field38192.clone(), source_ml_source2_nominal_field38194.clone(), source_ml_source2_nominal_field38196.clone())
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
                                                                let current = self::replace_java_script_call_frame((((match transition.clone() {
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(source_ml_source2_data_property_value38200, _, _) => {
                                                                        let source_ml_source2_data_property_value38200 = *source_ml_source2_data_property_value38200;
                                                                        Some((source_ml_source2_data_property_value38200.clone(),))
                                                                    }
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(source_ml_source2_data_property_value38201, _, _) => {
                                                                        let source_ml_source2_data_property_value38201 = *source_ml_source2_data_property_value38201;
                                                                        Some((source_ml_source2_data_property_value38201.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::JavaScriptOrdinaryHeap,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone(), shared_constructor_field12.clone(), next.clone());
                                                                match transition.clone() {
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(heap, _) => {
                                                                        let heap = *heap;
                                                                        self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(self::JavaScriptIteratorThrowTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 111, 114, 32, 115, 117, 112, 112, 114, 101, 115, 115, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                                    }
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolCall(heap_2, _, _, _, _) => {
                                                                        let heap_2 = *heap_2;
                                                                        self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(self::JavaScriptIteratorThrowTransition::MachineInvalid(Box::new(heap_2.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 111, 114, 32, 115, 117, 112, 112, 114, 101, 115, 115, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                                    }
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolPrimitiveGet(heap_3, _, _, _) => {
                                                                        let heap_3 = *heap_3;
                                                                        self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(self::JavaScriptIteratorThrowTransition::MachineInvalid(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 111, 114, 32, 115, 117, 112, 112, 114, 101, 115, 115, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                                    }
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolAcquired(heap_4, _) => {
                                                                        let heap_4 = *heap_4;
                                                                        self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(self::JavaScriptIteratorThrowTransition::MachineInvalid(Box::new(heap_4.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 111, 114, 32, 115, 117, 112, 112, 114, 101, 115, 115, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                                    }
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(heap_5, _, _, _) => {
                                                                        let heap_5 = *heap_5;
                                                                        self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(self::JavaScriptIteratorThrowTransition::MachineInvalid(Box::new(heap_5.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 111, 114, 32, 115, 117, 112, 112, 114, 101, 115, 115, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                                    }
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(heap_6, _, _) => {
                                                                        let heap_6 = *heap_6;
                                                                        self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(self::JavaScriptIteratorThrowTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_6.clone(), match current.clone() {
                                                                            self::JavaScriptCallMachine::CallMachine(_, source_ml_source2_data_property_value38223) => {
                                                                                source_ml_source2_data_property_value38223.clone()
                                                                            }
                                                                        }, self::write_java_script_call_iterator_result(next.clone(), destination, done, self::JavaScriptSemanticValue::Undefined, true))))))
                                                                    }
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(_, _, value_2) => {
                                                                        let value_2 = *value_2;
                                                                        self::JavaScriptIteratorThrowResolution::IteratorThrowContinue(Box::new(current.clone()), Box::new(value_2.clone()))
                                                                    }
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolError(heap_7, _, _, _) => {
                                                                        let heap_7 = *heap_7;
                                                                        self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(self::JavaScriptIteratorThrowTransition::MachineInvalid(Box::new(heap_7.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 111, 114, 32, 115, 117, 112, 112, 114, 101, 115, 115, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                                    }
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(heap_8, _) => {
                                                                        let heap_8 = *heap_8;
                                                                        self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(self::JavaScriptIteratorThrowTransition::MachineInvalid(Box::new(heap_8.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 111, 114, 32, 115, 117, 112, 112, 114, 101, 115, 115, 101, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(Box::new(self::JavaScriptIteratorThrowTransition::MachineInvalid(Box::new((((match transition.clone() {
                                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(source_ml_source2_data_property_value38224, _, _) => {
                                                                let source_ml_source2_data_property_value38224 = *source_ml_source2_data_property_value38224;
                                                                Some((source_ml_source2_data_property_value38224.clone(),))
                                                            }
                                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(source_ml_source2_data_property_value38225, _, _) => {
                                                                let source_ml_source2_data_property_value38225 = *source_ml_source2_data_property_value38225;
                                                                Some((source_ml_source2_data_property_value38225.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::JavaScriptOrdinaryHeap,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114, 111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 111, 110, 32, 117, 112, 100, 97, 116, 101]))))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => self::JavaScriptIteratorThrowResolution::IteratorThrowTransition(
                                Box::new(self::JavaScriptIteratorThrowTransition::MachineInvalid(
                                    Box::new(shared_constructor_field11.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        105, 116, 101, 114, 97, 116, 111, 114, 32, 116, 104, 114,
                                        111, 119, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                        116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115,
                                        112, 101, 110, 100, 101, 100, 32, 115, 111, 117, 114, 99,
                                        101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110,
                                    ]),
                                )),
                            ),
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_argument_rest(
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptArgumentRestTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
            let frame = {
                let source_ml_source2_array_read_value38229 = shared_constructor_field1.clone();
                {
                    let source_ml_source2_array_read_index38230 = ({
                        let ml_reference_array2471 = shared_constructor_field1.clone();
                        let ml_reference_projection2471 =
                            ml_reference_array2471.borrow().len() as f64;
                        ml_reference_projection2471
                    } - 1f64);
                    {
                        let ml_reference_array2472 =
                            source_ml_source2_array_read_value38229.clone();
                        let ml_reference_position2472 = source_ml_source2_array_read_index38230;
                        let ml_reference_projection2472 = self::ml_array::optional_at(
                            &*ml_reference_array2472.borrow(),
                            self::ml_array::number_index(ml_reference_position2472),
                        );
                        ml_reference_projection2472
                    }
                }
            };
            if match (frame.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                self::JavaScriptCallMachineTransition::MachineInvalid(
                    Box::new(match transition.clone() {
                        self::JavaScriptArgumentRestTransition::ArgumentRestContinue(
                            source_ml_source2_data_property_value38231,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38231 =
                                *source_ml_source2_data_property_value38231;
                            source_ml_source2_data_property_value38231.clone()
                        }
                        self::JavaScriptArgumentRestTransition::ArgumentRestAllocate(
                            source_ml_source2_data_property_value38232,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38232 =
                                *source_ml_source2_data_property_value38232;
                            source_ml_source2_data_property_value38232.clone()
                        }
                        self::JavaScriptArgumentRestTransition::ArgumentRestCompleted(
                            source_ml_source2_data_property_value38233,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38233 =
                                *source_ml_source2_data_property_value38233;
                            source_ml_source2_data_property_value38233.clone()
                        }
                        self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(
                            source_ml_source2_data_property_value38234,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38234 =
                                *source_ml_source2_data_property_value38234;
                            source_ml_source2_data_property_value38234.clone()
                        }
                        self::JavaScriptArgumentRestTransition::ArgumentRestResourceLimit(
                            source_ml_source2_data_property_value38235,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38235 =
                                *source_ml_source2_data_property_value38235;
                            source_ml_source2_data_property_value38235.clone()
                        }
                    }),
                    self::ml_string::Text::from_units(vec![
                        97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 115, 116, 32, 114,
                        101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                        108, 32, 99, 117, 114, 114, 101, 110, 116, 32, 115, 111, 117, 114, 99, 101,
                        32, 102, 114, 97, 109, 101,
                    ]),
                )
            } else {
                match transition.clone() {
                    self::JavaScriptArgumentRestTransition::ArgumentRestContinue(heap, state) => {
                        let heap = *heap;
                        let state = *state;
                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                            self::replace_java_script_call_frame(
                                heap.clone(),
                                shared_constructor_field1.clone(),
                                self::replace_java_script_call_pending(
                                    frame.clone().expect("a proven present value"),
                                    self::JavaScriptCallPending::WaitArgumentRest(
                                        destination,
                                        realm.clone(),
                                        Box::new(state.clone()),
                                    ),
                                ),
                            ),
                        ))
                    }
                    self::JavaScriptArgumentRestTransition::ArgumentRestAllocate(
                        heap_2,
                        state_2,
                        prototype,
                    ) => {
                        let heap_2 = *heap_2;
                        let state_2 = *state_2;
                        let prototype = *prototype;
                        self::JavaScriptCallMachineTransition::MachineAllocateObject(
                            Box::new(self::replace_java_script_call_frame(
                                heap_2.clone(),
                                shared_constructor_field1.clone(),
                                self::replace_java_script_call_pending(
                                    frame.clone().expect("a proven present value"),
                                    self::JavaScriptCallPending::WaitArgumentRest(
                                        destination,
                                        realm.clone(),
                                        Box::new(state_2.clone()),
                                    ),
                                ),
                            )),
                            Box::new(prototype.clone()),
                        )
                    }
                    self::JavaScriptArgumentRestTransition::ArgumentRestCompleted(
                        heap_3,
                        value,
                    ) => {
                        let heap_3 = *heap_3;
                        let value = *value;
                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                            self::replace_java_script_call_frame(
                                heap_3.clone(),
                                shared_constructor_field1.clone(),
                                self::write_java_script_call_register(
                                    frame.clone().expect("a proven present value"),
                                    destination,
                                    value.clone(),
                                ),
                            ),
                        ))
                    }
                    self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(heap_4, reason) => {
                        let heap_4 = *heap_4;
                        self::JavaScriptCallMachineTransition::MachineInvalid(
                            Box::new(heap_4.clone()),
                            reason.clone(),
                        )
                    }
                    self::JavaScriptArgumentRestTransition::ArgumentRestResourceLimit(
                        heap_5,
                        state_3,
                        reason_2,
                    ) => {
                        let heap_5 = *heap_5;
                        let state_3 = *state_3;
                        self::JavaScriptCallMachineTransition::MachineResourceLimit(
                            Box::new(self::replace_java_script_call_frame(
                                heap_5.clone(),
                                shared_constructor_field1.clone(),
                                self::replace_java_script_call_pending(
                                    frame.clone().expect("a proven present value"),
                                    self::JavaScriptCallPending::WaitArgumentRest(
                                        destination,
                                        realm.clone(),
                                        Box::new(state_3.clone()),
                                    ),
                                ),
                            )),
                            reason_2.clone(),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_object_enumeration(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptObjectEnumerationTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field2) => {
            let frame = {
                let source_ml_source2_array_read_value38242 = shared_constructor_field2.clone();
                {
                    let source_ml_source2_array_read_index38243 = ({
                        let ml_reference_array2473 = shared_constructor_field2.clone();
                        let ml_reference_projection2473 =
                            ml_reference_array2473.borrow().len() as f64;
                        ml_reference_projection2473
                    } - 1f64);
                    {
                        let ml_reference_array2474 =
                            source_ml_source2_array_read_value38242.clone();
                        let ml_reference_position2474 = source_ml_source2_array_read_index38243;
                        let ml_reference_projection2474 = self::ml_array::optional_at(
                            &*ml_reference_array2474.borrow(),
                            self::ml_array::number_index(ml_reference_position2474),
                        );
                        ml_reference_projection2474
                    }
                }
            };
            if match (frame.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(source_ml_source2_data_property_value38244, _) => {
                            let source_ml_source2_data_property_value38244 = *source_ml_source2_data_property_value38244;
                            source_ml_source2_data_property_value38244.clone()
                        }
                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationCall(source_ml_source2_data_property_value38245, _, _, _, _) => {
                            let source_ml_source2_data_property_value38245 = *source_ml_source2_data_property_value38245;
                            source_ml_source2_data_property_value38245.clone()
                        }
                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationAllocate(source_ml_source2_data_property_value38246, _, _, _) => {
                            let source_ml_source2_data_property_value38246 = *source_ml_source2_data_property_value38246;
                            source_ml_source2_data_property_value38246.clone()
                        }
                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationCompleted(source_ml_source2_data_property_value38247, _) => {
                            let source_ml_source2_data_property_value38247 = *source_ml_source2_data_property_value38247;
                            source_ml_source2_data_property_value38247.clone()
                        }
                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationThrow(source_ml_source2_data_property_value38248, _) => {
                            let source_ml_source2_data_property_value38248 = *source_ml_source2_data_property_value38248;
                            source_ml_source2_data_property_value38248.clone()
                        }
                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationError(source_ml_source2_data_property_value38249, _, _) => {
                            let source_ml_source2_data_property_value38249 = *source_ml_source2_data_property_value38249;
                            source_ml_source2_data_property_value38249.clone()
                        }
                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(source_ml_source2_data_property_value38250, _) => {
                            let source_ml_source2_data_property_value38250 = *source_ml_source2_data_property_value38250;
                            source_ml_source2_data_property_value38250.clone()
                        }
                        self::JavaScriptObjectEnumerationTransition::ObjectEnumerationResourceLimit(source_ml_source2_data_property_value38251, _, _) => {
                            let source_ml_source2_data_property_value38251 = *source_ml_source2_data_property_value38251;
                            source_ml_source2_data_property_value38251.clone()
                        }
                    }), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 117, 114, 114, 101, 110, 116, 32, 115, 111, 117, 114, 99, 101, 32, 102, 114, 97, 109, 101]))
            } else {
                {
                    let current = self::JavaScriptCallMachine::CallMachine(Box::new(match transition.clone() {
                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(source_ml_source2_data_property_value38252, _) => {
                                let source_ml_source2_data_property_value38252 = *source_ml_source2_data_property_value38252;
                                source_ml_source2_data_property_value38252.clone()
                            }
                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationCall(source_ml_source2_data_property_value38253, _, _, _, _) => {
                                let source_ml_source2_data_property_value38253 = *source_ml_source2_data_property_value38253;
                                source_ml_source2_data_property_value38253.clone()
                            }
                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationAllocate(source_ml_source2_data_property_value38254, _, _, _) => {
                                let source_ml_source2_data_property_value38254 = *source_ml_source2_data_property_value38254;
                                source_ml_source2_data_property_value38254.clone()
                            }
                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationCompleted(source_ml_source2_data_property_value38255, _) => {
                                let source_ml_source2_data_property_value38255 = *source_ml_source2_data_property_value38255;
                                source_ml_source2_data_property_value38255.clone()
                            }
                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationThrow(source_ml_source2_data_property_value38256, _) => {
                                let source_ml_source2_data_property_value38256 = *source_ml_source2_data_property_value38256;
                                source_ml_source2_data_property_value38256.clone()
                            }
                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationError(source_ml_source2_data_property_value38257, _, _) => {
                                let source_ml_source2_data_property_value38257 = *source_ml_source2_data_property_value38257;
                                source_ml_source2_data_property_value38257.clone()
                            }
                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(source_ml_source2_data_property_value38258, _) => {
                                let source_ml_source2_data_property_value38258 = *source_ml_source2_data_property_value38258;
                                source_ml_source2_data_property_value38258.clone()
                            }
                            self::JavaScriptObjectEnumerationTransition::ObjectEnumerationResourceLimit(source_ml_source2_data_property_value38259, _, _) => {
                                let source_ml_source2_data_property_value38259 = *source_ml_source2_data_property_value38259;
                                source_ml_source2_data_property_value38259.clone()
                            }
                        }), shared_constructor_field2.clone());
                    match current.clone() {
                            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                                match transition.clone() {
                                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationContinue(heap, state) => {
                                        let heap = *heap;
                                        let state = *state;
                                        {
                                            let next = self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectEnumeration(destination, realm.clone(), Box::new(state.clone()))));
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(next.clone()))
                                        }
                                    }
                                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationCall(heap_2, state_2, source_ml_source2_f38261, source_ml_source2_f38262, source_ml_source2_f38263) => {
                                        let heap_2 = *heap_2;
                                        let state_2 = *state_2;
                                        let source_ml_source2_f38261 = *source_ml_source2_f38261;
                                        let source_ml_source2_f38262 = *source_ml_source2_f38262;
                                        {
                                            let next_2 = self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectEnumeration(destination, realm.clone(), Box::new(state_2.clone()))));
                                            self::push_java_script_call(program.clone(), next_2.clone(), source_ml_source2_f38261.clone(), source_ml_source2_f38262.clone(), source_ml_source2_f38263.clone())
                                        }
                                    }
                                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationAllocate(heap_3, state_3, source_ml_source2_f38264, _) => {
                                        let heap_3 = *heap_3;
                                        let state_3 = *state_3;
                                        let source_ml_source2_f38264 = *source_ml_source2_f38264;
                                        {
                                            let next_3 = self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectEnumeration(destination, realm.clone(), Box::new(state_3.clone()))));
                                            self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(next_3.clone()), Box::new(source_ml_source2_f38264.clone()))
                                        }
                                    }
                                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationCompleted(heap_4, value) => {
                                        let heap_4 = *heap_4;
                                        let value = *value;
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_4.clone(), shared_constructor_field1.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                                    }
                                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationThrow(_, value_2) => {
                                        let value_2 = *value_2;
                                        self::throw_java_script_call_value(current.clone(), value_2.clone())
                                    }
                                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationError(_, error_class, reason) => {
                                        self::request_java_script_call_error(current.clone(), realm.clone(), &(error_class), &(reason))
                                    }
                                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationInvalid(heap_5, reason_2) => {
                                        let heap_5 = *heap_5;
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_5.clone()), reason_2.clone())
                                    }
                                    self::JavaScriptObjectEnumerationTransition::ObjectEnumerationResourceLimit(heap_6, state_4, source_ml_source2_f38268) => {
                                        let heap_6 = *heap_6;
                                        let state_4 = *state_4;
                                        {
                                            let next_4 = self::replace_java_script_call_frame(heap_6.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectEnumeration(destination, realm.clone(), Box::new(state_4.clone()))));
                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(next_4.clone()), source_ml_source2_f38268.clone())
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
pub fn accept_java_script_call_object_integrity(
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptObjectIntegrityTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field2) => {
            let frame = {
                let source_ml_source2_array_read_value38300 = shared_constructor_field2.clone();
                {
                    let source_ml_source2_array_read_index38301 = ({
                        let ml_reference_array2475 = shared_constructor_field2.clone();
                        let ml_reference_projection2475 =
                            ml_reference_array2475.borrow().len() as f64;
                        ml_reference_projection2475
                    } - 1f64);
                    {
                        let ml_reference_array2476 =
                            source_ml_source2_array_read_value38300.clone();
                        let ml_reference_position2476 = source_ml_source2_array_read_index38301;
                        let ml_reference_projection2476 = self::ml_array::optional_at(
                            &*ml_reference_array2476.borrow(),
                            self::ml_array::number_index(ml_reference_position2476),
                        );
                        ml_reference_projection2476
                    }
                }
            };
            if match (frame.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                self::JavaScriptCallMachineTransition::MachineInvalid(
                    Box::new(match transition.clone() {
                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityContinue(
                            source_ml_source2_data_property_value38302,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38302 =
                                *source_ml_source2_data_property_value38302;
                            source_ml_source2_data_property_value38302.clone()
                        }
                        self::JavaScriptObjectIntegrityTransition::ObjectPrototypeMetadata(
                            source_ml_source2_data_property_value38303,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38303 =
                                *source_ml_source2_data_property_value38303;
                            source_ml_source2_data_property_value38303.clone()
                        }
                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(
                            source_ml_source2_data_property_value38304,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38304 =
                                *source_ml_source2_data_property_value38304;
                            source_ml_source2_data_property_value38304.clone()
                        }
                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(
                            source_ml_source2_data_property_value38305,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38305 =
                                *source_ml_source2_data_property_value38305;
                            source_ml_source2_data_property_value38305.clone()
                        }
                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(
                            source_ml_source2_data_property_value38306,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38306 =
                                *source_ml_source2_data_property_value38306;
                            source_ml_source2_data_property_value38306.clone()
                        }
                        self::JavaScriptObjectIntegrityTransition::ObjectIntegrityResourceLimit(
                            source_ml_source2_data_property_value38307,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value38307 =
                                *source_ml_source2_data_property_value38307;
                            source_ml_source2_data_property_value38307.clone()
                        }
                    }),
                    self::ml_string::Text::from_units(vec![
                        79, 98, 106, 101, 99, 116, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105,
                        111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                        99, 116, 117, 97, 108, 32, 99, 117, 114, 114, 101, 110, 116, 32, 115, 111,
                        117, 114, 99, 101, 32, 102, 114, 97, 109, 101,
                    ]),
                )
            } else {
                {
                    let current = self::JavaScriptCallMachine::CallMachine(Box::new(match transition.clone() {
                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityContinue(source_ml_source2_data_property_value38308, _) => {
                                let source_ml_source2_data_property_value38308 = *source_ml_source2_data_property_value38308;
                                source_ml_source2_data_property_value38308.clone()
                            }
                            self::JavaScriptObjectIntegrityTransition::ObjectPrototypeMetadata(source_ml_source2_data_property_value38309, _) => {
                                let source_ml_source2_data_property_value38309 = *source_ml_source2_data_property_value38309;
                                source_ml_source2_data_property_value38309.clone()
                            }
                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(source_ml_source2_data_property_value38310, _) => {
                                let source_ml_source2_data_property_value38310 = *source_ml_source2_data_property_value38310;
                                source_ml_source2_data_property_value38310.clone()
                            }
                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(source_ml_source2_data_property_value38311, _, _) => {
                                let source_ml_source2_data_property_value38311 = *source_ml_source2_data_property_value38311;
                                source_ml_source2_data_property_value38311.clone()
                            }
                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(source_ml_source2_data_property_value38312, _) => {
                                let source_ml_source2_data_property_value38312 = *source_ml_source2_data_property_value38312;
                                source_ml_source2_data_property_value38312.clone()
                            }
                            self::JavaScriptObjectIntegrityTransition::ObjectIntegrityResourceLimit(source_ml_source2_data_property_value38313, _, _) => {
                                let source_ml_source2_data_property_value38313 = *source_ml_source2_data_property_value38313;
                                source_ml_source2_data_property_value38313.clone()
                            }
                        }), shared_constructor_field2.clone());
                    match current.clone() {
                            self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                                match transition.clone() {
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityContinue(heap, state) => {
                                        let heap = *heap;
                                        let state = *state;
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectIntegrity(destination, realm.clone(), Box::new(state.clone()))))))
                                    }
                                    self::JavaScriptObjectIntegrityTransition::ObjectPrototypeMetadata(heap_2, input) => {
                                        let heap_2 = *heap_2;
                                        let input = *input;
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectPrototype(destination, realm.clone(), Box::new(input.clone()))))))
                                    }
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityCompleted(heap_3, value) => {
                                        let heap_3 = *heap_3;
                                        let value = *value;
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                                    }
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityError(_, error_class, reason) => {
                                        self::request_java_script_call_error(current.clone(), realm.clone(), &(error_class), &(reason))
                                    }
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityInvalid(heap_4, reason_2) => {
                                        let heap_4 = *heap_4;
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_4.clone()), reason_2.clone())
                                    }
                                    self::JavaScriptObjectIntegrityTransition::ObjectIntegrityResourceLimit(heap_5, state_2, reason_3) => {
                                        let heap_5 = *heap_5;
                                        let state_2 = *state_2;
                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(heap_5.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitObjectIntegrity(destination, realm.clone(), Box::new(state_2.clone()))))), reason_3.clone())
                                    }
                                }
                            }
                        }
                }
            }
        }
    }
}
