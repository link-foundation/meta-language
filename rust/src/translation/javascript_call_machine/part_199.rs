// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallCompletion;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallProgram;
use super::JavaScriptNumberCallState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptTextCallState;
use super::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494;
use super::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495;
use super::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Result;
use super::classify_java_script_array_species_constructor;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::project_java_script_call_completion_transition;
use super::read_java_script_known_array_constructor_fact;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::source_ml_source1_temporary_read_java_script_call_frame_control_error_loop1493;
use super::test_java_script_semantic_same_value;
use super::unwind_java_script_call_completion;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_frame_control_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
) -> self::ml_string::Text {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            _,
            _,
            shared_constructor_field4,
            _,
            _,
            _,
            _,
            shared_constructor_field5,
            _,
            _,
        ) => match frame.clone() {
            self::JavaScriptCallFrame::CallFrame(
                _,
                shared_constructor_field1,
                shared_constructor_field2,
                _,
                shared_constructor_field3,
                _,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source1_temporary_values1490 = shared_constructor_field3.clone();
                {
                    let source_ml_source1_temporary_index1490 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s35313 = self::source_ml_source1_temporary_read_java_script_call_frame_control_error_loop1493(heap.clone(), fn_.clone(), source_ml_source1_temporary_values1490.clone(), source_ml_source1_temporary_index1490.clone());
                        match source_ml_source2_s35313.clone() {
                                    self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Result::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Done => {
                                        {
                                            let join_entries = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values2271 = { let ml_reference_value2272 = shared_constructor_field5.borrow().clone(); ml_reference_value2272 }; let ml_callback2271 = std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction| -> bool { match instruction.clone() {
                                                self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            } }); ml_values2271.iter().cloned().enumerate().filter_map(|(ml_index2271, ml_value2271)| if ml_callback2271(ml_value2271.clone()) { Some(ml_value2271) } else { None }).collect::<Vec<_>>() }));
                                            {
                                                let join_entry = {
                                                    let source_ml_source2_array_read_index35318 = 0f64;
                                                    { let ml_reference_array2273 = join_entries.clone(); let ml_reference_position2273 = source_ml_source2_array_read_index35318; let ml_reference_projection2273 = self::ml_array::optional_at(&*ml_reference_array2273.borrow(), self::ml_array::number_index(ml_reference_position2273)); ml_reference_projection2273 }
                                                };
                                                {
                                                    let join_enter = { let ml_values2274 = { let ml_reference_value2275 = shared_constructor_field5.borrow().clone(); ml_reference_value2275 }; let ml_callback2274 = std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction| -> bool { match instruction.clone() {
                                                        self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    } }); ml_values2274.iter().cloned().enumerate().position(|(ml_index2274, ml_value2274)| ml_callback2274(ml_value2274)).map_or(-1f64, |ml_position| ml_position as f64) };
                                                    {
                                                        let join_leave = { let ml_values2276 = { let ml_reference_value2277 = shared_constructor_field5.borrow().clone(); ml_reference_value2277 }; let ml_callback2276 = std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction| -> bool { match instruction.clone() {
                                                            self::JavaScriptCallInstruction::ArrayJoinLeave(_) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        } }); ml_values2276.iter().cloned().enumerate().position(|(ml_index2276, ml_value2276)| ml_callback2276(ml_value2276)).map_or(-1f64, |ml_position| ml_position as f64) };
                                                        {
                                                            let entered = if (match (join_entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match join_entry.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }) {
                                                                None::<self::JavaScriptSemanticValue>
                                                            } else {
                                                                {
                                                                    let source_ml_source2_array_read_value35328 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index35329 = ((match join_entry.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallInstruction::ArrayJoinEnter(source_ml_source2_data_property_value35327, _) => {
                                                                                Some((source_ml_source2_data_property_value35327,))
                                                                            }
                                                                            _ => {
                                                                                None::<(f64,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0;
                                                                        { let ml_reference_array2278 = source_ml_source2_array_read_value35328.clone(); let ml_reference_position2278 = source_ml_source2_array_read_index35329; let ml_reference_projection2278 = self::ml_array::optional_at(&*ml_reference_array2278.borrow(), self::ml_array::number_index(ml_reference_position2278)); ml_reference_projection2278 }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let owns_join = (((((shared_constructor_field1 > join_enter) && (shared_constructor_field1 <= join_leave)) && !(match (entered.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) && match entered.clone().expect("a proven present value") {
                                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                }) && ((match entered.clone().expect("a proven present value") {
                                                                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_data_property_value35335) => {
                                                                        Some((source_ml_source2_data_property_value35335,))
                                                                    }
                                                                    _ => {
                                                                        None::<(bool,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0);
                                                                if (owns_join && match (match frame.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value35336, _) => {
                                                                        ((source_ml_source2_data_property_value35336.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    self::ml_string::Text::from_units(vec![97, 32, 115, 117, 99, 99, 101, 115, 115, 102, 117, 108, 32, 117, 110, 102, 105, 110, 105, 115, 104, 101, 100, 32, 106, 111, 105, 110, 32, 99, 97, 110, 110, 111, 116, 32, 108, 111, 115, 101, 32, 105, 116, 115, 32, 111, 119, 110, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 103, 117, 97, 114, 100])
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_s35357 = if !(match (match frame.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value35358, _) => {
                                                                                ((source_ml_source2_data_property_value35358.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            {
                                                                                let joined = (match frame.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value35359, _) => {
                                                                                        ((source_ml_source2_data_property_value35359.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value");
                                                                                if (match joined.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                } || ((self::read_java_script_ordinary_value_error(heap.clone(), joined.clone())) != (self::ml_string::Text::from_units(vec![])))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 105, 118, 101, 32, 106, 111, 105, 110, 32, 103, 117, 97, 114, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 79, 98, 106, 101, 99, 116]))
                                                                                } else {
                                                                                    if ((((({ let ml_reference_array2279 = join_entries.clone(); let ml_reference_projection2279 = ml_reference_array2279.borrow().len() as f64; ml_reference_projection2279 }) != (1f64)) || {
                                                                                        let source_ml_source2_comparison_operand35363 = {
                                                                                            let source_ml_source2_optional_chain_receiver35362 = (join_entry.clone(),);
                                                                                            if match (source_ml_source2_optional_chain_receiver35362.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                Some({
                                                                                                    let source_ml_source1_temporary_optional_chain1491 = ((source_ml_source2_optional_chain_receiver35362.clone()).0).clone().expect("a proven present value");
                                                                                                    match source_ml_source1_temporary_optional_chain1491.clone() {
                                                                                                        self::JavaScriptCallInstruction::Constant(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 97, 110, 116])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::Copy(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 112, 121])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::Argument(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArgumentCount(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 67, 111, 117, 110, 116])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArgumentAt(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 65, 116])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::FunctionBind(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 66, 105, 110, 100])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::BoundFunctionCall(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 67, 97, 108, 108])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::FunctionToString(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 84, 111, 83, 116, 114, 105, 110, 103])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::PushFinally(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![112, 117, 115, 104, 70, 105, 110, 97, 108, 108, 121])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::FinishFinally => {
                                                                                                            self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104, 70, 105, 110, 97, 108, 108, 121])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::CompleteJump(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 74, 117, 109, 112])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::Jump(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![106, 117, 109, 112])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![98, 114, 97, 110, 99, 104])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 97, 108, 108])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArgumentListOpen(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 76, 105, 115, 116, 79, 112, 101, 110])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArgumentListAppend(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 76, 105, 115, 116, 65, 112, 112, 101, 110, 100])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 76, 105, 115, 116, 67, 97, 108, 108])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::Construct(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ConstructorCheck(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 67, 104, 101, 99, 107])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ReadNewTarget(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![114, 101, 97, 100, 78, 101, 119, 84, 97, 114, 103, 101, 116])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 74, 111, 105, 110, 69, 110, 116, 101, 114])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArrayJoinLeave(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 74, 111, 105, 110, 76, 101, 97, 118, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ConstructErrorClass(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 69, 114, 114, 111, 114, 67, 108, 97, 115, 115])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::CallClassConstructor(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 97, 108, 108, 67, 108, 97, 115, 115, 67, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ConstructCollection(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 67, 111, 108, 108, 101, 99, 116, 105, 111, 110])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::CollectionAllocate(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 65, 108, 108, 111, 99, 97, 116, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 65, 108, 108, 111, 99, 97, 116, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArgumentRest(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 82, 101, 115, 116])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 80, 114, 111, 116, 111, 99, 111, 108])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 82, 101, 115, 117, 108, 116])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::CollectionOperation(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 79, 112, 101, 114, 97, 116, 105, 111, 110])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::PrimitiveMethod(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 77, 101, 116, 104, 111, 100])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::CollectionForEach(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 70, 111, 114, 69, 97, 99, 104])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 67, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArraySlice(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 83, 108, 105, 99, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArrayCreation(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 67, 114, 101, 97, 116, 105, 111, 110])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArrayCallback(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 67, 97, 108, 108, 98, 97, 99, 107])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArraySearch(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 83, 101, 97, 114, 99, 104])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArrayTransform(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 84, 114, 97, 110, 115, 102, 111, 114, 109])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ObjectEnumeration(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 69, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::PropertyDefinitions(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::PropertyReflection(_, _, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 82, 101, 102, 108, 101, 99, 116, 105, 111, 110])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::InstanceOf(_, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 79, 102])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ObjectObservation(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 79, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::CollectionCopy(_, _, _, _, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 67, 111, 112, 121])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 69, 114, 114, 111, 114, 86, 97, 108, 117, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ReadDerivedReceiver(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![114, 101, 97, 100, 68, 101, 114, 105, 118, 101, 100, 82, 101, 99, 101, 105, 118, 101, 114])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::InvokeErrorIntrinsic(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![105, 110, 118, 111, 107, 101, 69, 114, 114, 111, 114, 73, 110, 116, 114, 105, 110, 115, 105, 99])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::InvokeHostCapability(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![105, 110, 118, 111, 107, 101, 72, 111, 115, 116, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::Return(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::Throw(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![112, 117, 115, 104, 72, 97, 110, 100, 108, 101, 114])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::PopHandler => {
                                                                                                            self::ml_string::Text::from_units(vec![112, 111, 112, 72, 97, 110, 100, 108, 101, 114])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114, 66, 105, 110, 97, 114, 121])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![115, 97, 109, 101, 86, 97, 108, 117, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                                                                            self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 87, 114, 105, 116, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::OrdinaryAllocate(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 65, 108, 108, 111, 99, 97, 116, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::LiteralPrototype(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108, 80, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::LiteralAccessor(_, _, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108, 65, 99, 99, 101, 115, 115, 111, 114])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 70, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 87, 114, 97, 112, 112, 101, 114])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::SymbolAllocate(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 65, 108, 108, 111, 99, 97, 116, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::SymbolDescription(_, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 68, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::SymbolRegistry(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108, 82, 101, 103, 105, 115, 116, 114, 121])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::FunctionName(_, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 78, 97, 109, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArrayAllocate(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 65, 108, 108, 111, 99, 97, 116, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 65, 108, 108, 111, 99, 97, 116, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 83, 112, 114, 101, 97, 100])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 83, 112, 114, 101, 97, 100])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::OrdinaryDataProperty(_, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 68, 97, 116, 97, 80, 114, 111, 112, 101, 114, 116, 121])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 71, 101, 116])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::OrdinaryDelete(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 68, 101, 108, 101, 116, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::OrdinaryHas(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 72, 97, 115])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 83, 101, 116])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::OrdinaryDefine(_, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 68, 101, 102, 105, 110, 101])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 75, 101, 121])
                                                                                                        }
                                                                                                        self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _, _) => {
                                                                                                            self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 68, 101, 102, 105, 110, 105, 116, 105, 111, 110])
                                                                                                        }
                                                                                                    }
                                                                                                })
                                                                                            } else {
                                                                                                None::<self::ml_string::Text>
                                                                                            }
                                                                                        };
                                                                                        !(match (source_ml_source2_comparison_operand35363.clone(), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 74, 111, 105, 110, 69, 110, 116, 101, 114])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                    }) || !owns_join) || !(self::test_java_script_semantic_same_value(({
                                                                                        let source_ml_source2_array_read_value35376 = shared_constructor_field2.clone();
                                                                                        {
                                                                                            let source_ml_source2_array_read_index35377 = match {
                                                                                                let source_ml_source2_nullish_field_receiver35366 = (join_entry.clone(),);
                                                                                                if match (source_ml_source2_nullish_field_receiver35366.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                    {
                                                                                                        let source_ml_source2_nullish_field_present35367 = ((source_ml_source2_nullish_field_receiver35366.clone()).0).clone().expect("a proven present value");
                                                                                                        match source_ml_source2_nullish_field_present35367.clone() {
                                                                                                            self::JavaScriptCallInstruction::Call(_, _, source_ml_source2_data_property_value35368, _) => {
                                                                                                                Some(source_ml_source2_data_property_value35368)
                                                                                                            }
                                                                                                            self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, source_ml_source2_data_property_value35369, _) => {
                                                                                                                Some(source_ml_source2_data_property_value35369)
                                                                                                            }
                                                                                                            self::JavaScriptCallInstruction::ArrayJoinEnter(_, source_ml_source2_data_property_value35370) => {
                                                                                                                Some(source_ml_source2_data_property_value35370)
                                                                                                            }
                                                                                                            self::JavaScriptCallInstruction::ArrayJoinLeave(source_ml_source2_data_property_value35371) => {
                                                                                                                Some(source_ml_source2_data_property_value35371)
                                                                                                            }
                                                                                                            self::JavaScriptCallInstruction::CallClassConstructor(_, _, source_ml_source2_data_property_value35372, _) => {
                                                                                                                Some(source_ml_source2_data_property_value35372)
                                                                                                            }
                                                                                                            self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, source_ml_source2_data_property_value35373) => {
                                                                                                                Some(source_ml_source2_data_property_value35373)
                                                                                                            }
                                                                                                            self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, source_ml_source2_data_property_value35374) => {
                                                                                                                Some(source_ml_source2_data_property_value35374)
                                                                                                            }
                                                                                                            self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, source_ml_source2_data_property_value35375) => {
                                                                                                                Some(source_ml_source2_data_property_value35375)
                                                                                                            }
                                                                                                            _ => {
                                                                                                                None::<f64>
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 99, 101, 105, 118, 101, 114, 39, 41])).0, message_present: true }).raise()
                                                                                                }
                                                                                            } { Some(ml_present) => ml_present, None => f64::NAN };
                                                                                            { let ml_reference_array2280 = source_ml_source2_array_read_value35376.clone(); let ml_reference_position2280 = source_ml_source2_array_read_index35377; let ml_reference_projection2280 = self::ml_array::optional_at(&*ml_reference_array2280.borrow(), self::ml_array::number_index(ml_reference_position2280)); ml_reference_projection2280 }
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), joined.clone()))) {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494Return(self::ml_string::Text::from_units(vec![97, 110, 32, 97, 99, 116, 105, 118, 101, 32, 106, 111, 105, 110, 32, 103, 117, 97, 114, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 115, 117, 99, 99, 101, 115, 115, 102, 117, 108, 32, 117, 110, 102, 105, 110, 105, 115, 104, 101, 100, 32, 101, 110, 116, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494Next
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494Next
                                                                        };
                                                                        match source_ml_source2_s35357.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494Next => {
                                                                                if (!(match (match frame.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35378) => {
                                                                                        ((source_ml_source2_data_property_value35378.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                                                                    let source_ml_source2_s35380 = (match frame.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35383) => {
                                                                                            ((source_ml_source2_data_property_value35383.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value");
                                                                                    match source_ml_source2_s35380.clone() {
                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                            false
                                                                                        }
                                                                                        _ => {
                                                                                            true
                                                                                        }
                                                                                    }
                                                                                } || !(((match (match frame.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35384) => {
                                                                                        ((source_ml_source2_data_property_value35384.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value") {
                                                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value35385) => {
                                                                                        Some((source_ml_source2_data_property_value35385,))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(bool,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0)) || ((self::read_java_script_ordinary_value_error(heap.clone(), (match frame.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35387) => {
                                                                                        ((source_ml_source2_data_property_value35387.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value"))) != (self::ml_string::Text::from_units(vec![]))))) {
                                                                                    self::ml_string::Text::from_units(vec![110, 101, 119, 46, 116, 97, 114, 103, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 105, 100, 101, 110, 116, 105, 116, 121])
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source2_s35396 = if (!(match (match frame.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35397) => {
                                                                                                ((source_ml_source2_data_property_value35397.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(match (match program.clone() {
                                                                                            self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value35398, _) => {
                                                                                                ((source_ml_source2_data_property_value35398.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                                            {
                                                                                                let actual = self::classify_java_script_array_species_constructor(heap.clone(), (match program.clone() {
                                                                                                    self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value35399, _) => {
                                                                                                        ((source_ml_source2_data_property_value35399.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().expect("a proven present value"), (match frame.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35400) => {
                                                                                                        ((source_ml_source2_data_property_value35400.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().expect("a proven present value"));
                                                                                                if {
                                                                                                    let source_ml_source2_comparison_operand35403 = {
                                                                                                        let source_ml_source2_optional_chain_receiver35401 = (self::read_java_script_known_array_constructor_fact(actual.clone()),);
                                                                                                        if match (source_ml_source2_optional_chain_receiver35401.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                            Some({
                                                                                                                let source_ml_source1_temporary_optional_chain1492 = ((source_ml_source2_optional_chain_receiver35401.clone()).0).clone().expect("a proven present value");
                                                                                                                match source_ml_source1_temporary_optional_chain1492.clone() {
                                                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value35402, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value35402
                                                                                                                    }
                                                                                                                }
                                                                                                            })
                                                                                                        } else {
                                                                                                            None::<bool>
                                                                                                        }
                                                                                                    };
                                                                                                    !(match (source_ml_source2_comparison_operand35403.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                                } {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495Return(self::ml_string::Text::from_units(vec![110, 101, 119, 46, 116, 97, 114, 103, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 105, 110, 100, 101, 112, 101, 110, 100, 101, 110, 116, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110]))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495Next
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495Next
                                                                                        };
                                                                                        match source_ml_source2_s35396.clone() {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495Next => {
                                                                                                if ((shared_constructor_field4 == (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108]))) && !(self::test_java_script_semantic_same_value((match frame.clone() {
                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35407) => {
                                                                                                        ((source_ml_source2_data_property_value35407.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (match fn_.clone() {
                                                                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value35408, _) => {
                                                                                                        ((source_ml_source2_data_property_value35408.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) {
                                                                                                    self::ml_string::Text::from_units(vec![97, 110, 32, 97, 114, 114, 111, 119, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 108, 101, 120, 105, 99, 97, 108, 32, 110, 101, 119, 46, 116, 97, 114, 103, 101, 116])
                                                                                                } else {
                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1495Return(source_ml_source1_temporary_result1495) => {
                                                                                                source_ml_source1_temporary_result1495.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494::MlSource1TemporaryReadJavaScriptCallFrameControlErrorJoin1494Return(source_ml_source1_temporary_result1495_2) => {
                                                                                source_ml_source1_temporary_result1495_2.clone()
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
                                    self::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Result::MlSource1TemporaryReadJavaScriptCallFrameControlErrorLoop1493Return(source_ml_source1_temporary_result1495_3) => {
                                        source_ml_source1_temporary_result1495_3.clone()
                                    }
                                }
                    }
                }
            }
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_number_call_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptNumberCallState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptNumberCallState::NumberProtocolState(state_state) => {
            let state_state = *state_state;
            match state_state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    shared_constructor_field1,
                    _,
                ) => {
                    if ((shared_constructor_field1
                        != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))
                        && (shared_constructor_field1
                            != (self::ml_string::Text::from_units(vec![
                                100, 101, 102, 97, 117, 108, 116,
                            ]))))
                    {
                        self::ml_string::Text::from_units(vec![
                            78, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105,
                            111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                            97, 99, 116, 117, 97, 108, 32, 110, 117, 109, 101, 114, 105, 99, 32,
                            112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 104, 105, 110, 116,
                        ])
                    } else {
                        self::read_java_script_primitive_conversion_state_error(
                            heap.clone(),
                            state_state.clone(),
                        )
                    }
                }
            }
        }
        self::JavaScriptNumberCallState::NumberPrimitiveValue(value) => {
            let value = *value;
            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone()))
                != (self::ml_string::Text::from_units(vec![])))
            {
                self::ml_string::Text::from_units(vec![
                    97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 98, 101,
                    114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116,
                    97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112,
                    114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101,
                ])
            } else {
                if match value.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => true,
                    _ => false,
                } {
                    self::ml_string::Text::from_units(vec![
                        97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 98,
                        101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114,
                        101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                        108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108,
                        117, 101,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_text_call_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptTextCallState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptTextCallState::TextProtocolState(state_state) => {
            let state_state = *state_state;
            if ((match state_state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    source_ml_source2_data_property_value35416,
                    _,
                ) => source_ml_source2_data_property_value35416.clone(),
            }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
            {
                self::ml_string::Text::from_units(vec![
                    84, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                    114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                    108, 32, 115, 116, 114, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105,
                    118, 101, 32, 104, 105, 110, 116,
                ])
            } else {
                self::read_java_script_primitive_conversion_state_error(
                    heap.clone(),
                    state_state.clone(),
                )
            }
        }
        self::JavaScriptTextCallState::TextPrimitiveValue(value) => {
            let value = *value;
            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone()))
                != (self::ml_string::Text::from_units(vec![])))
            {
                self::ml_string::Text::from_units(vec![
                    97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 116, 101, 120, 116, 32,
                    99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105,
                    110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105,
                    109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101,
                ])
            } else {
                if match value.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => true,
                    _ => false,
                } {
                    self::ml_string::Text::from_units(vec![
                        97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 116, 101, 120, 116,
                        32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97,
                        105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112,
                        114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn throw_java_script_call_value(
    machine: self::JavaScriptCallMachine,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallMachineTransition {
    {
        let transition = self::unwind_java_script_call_completion(
            machine.clone(),
            self::JavaScriptCallCompletion::CallCompletion(
                self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]),
                Box::new(value.clone()),
                (-1f64),
                0f64,
            ),
        );
        self::project_java_script_call_completion_transition(transition.clone())
    }
}
