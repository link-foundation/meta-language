// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionState;
use super::JavaScriptConstructorRealm;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::JavaScriptSpeciesConstructionCall;
use super::JavaScriptSpeciesConstructionState;
use super::JavaScriptSpeciesConstructionTransition;
use super::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107;
use super::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108;
use super::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Result;
use super::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Result;
use super::adapt_java_script_species_construction;
use super::begin_java_script_construction;
use super::continue_java_script_construction_prototype;
use super::find_java_script_array_constructor_fact;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_species_directory_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_shared_species_construction_state_error;
use super::source_ml_source1_temporary_read_java_script_species_construction_state_error_loop1106;
use super::source_ml_source1_temporary_read_java_script_species_construction_state_error_loop1109;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_species_construction_state_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptSpeciesConstructionState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            match directory.clone() {
                self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(
                    _,
                    _,
                    _,
                    _,
                    shared_constructor_field2,
                    _,
                ) => match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        shared_constructor_field1,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let error = self::read_java_script_array_species_directory_error(
                            heap.clone(),
                            directory.clone(),
                        );
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            error.clone()
                        } else {
                            if ({
                                        let source_ml_source2_comparison_operand24335 = match state.clone() {
                                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value24334, _) => {
                                                ((source_ml_source2_data_property_value24334.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_comparison_operand24336 = None::<()>;
                                            !(match (source_ml_source2_comparison_operand24335.clone(), source_ml_source2_comparison_operand24336.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                        }
                                    } || !(match (match state.clone() {
                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value24337) => {
                                            ((source_ml_source2_data_property_value24337.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone()
                                        }
                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                        self::read_java_script_shared_species_construction_state_error(program.clone(), heap.clone(), directory.clone(), state.clone())
                                    } else {
                                        {
                                            let fact = {
                                                let source_ml_source2_array_read_value24342 = shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_array_read_index24343 = self::find_java_script_array_constructor_fact(shared_constructor_field2.clone(), match shared_constructor_field4.clone() {
                                                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(source_ml_source2_data_property_value24341, _, _, _, _) => {
                                                            source_ml_source2_data_property_value24341.clone()
                                                        }
                                                    });
                                                    { let ml_reference_array1266 = source_ml_source2_array_read_value24342.clone(); let ml_reference_position1266 = source_ml_source2_array_read_index24343; let ml_reference_projection1266 = self::ml_array::optional_at(&*ml_reference_array1266.borrow(), self::ml_array::number_index(ml_reference_position1266)); ml_reference_projection1266 }
                                                }
                                            };
                                            if (((((((match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match fact.clone().expect("a proven present value") {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value24347, _, _) => {
                                                    source_ml_source2_data_property_value24347.clone()
                                                }
                                            }) != (match shared_constructor_field4.clone() {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value24349, _, _) => {
                                                    source_ml_source2_data_property_value24349.clone()
                                                }
                                            }))) || ((match fact.clone().expect("a proven present value") {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value24353, _) => {
                                                    source_ml_source2_data_property_value24353
                                                }
                                            }) != (match shared_constructor_field4.clone() {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value24355, _) => {
                                                    source_ml_source2_data_property_value24355
                                                }
                                            }))) || ((match fact.clone().expect("a proven present value") {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value24359, _, _, _) => {
                                                    source_ml_source2_data_property_value24359
                                                }
                                            }) != (match shared_constructor_field4.clone() {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value24361, _, _, _) => {
                                                    source_ml_source2_data_property_value24361
                                                }
                                            }))) || !(match fact.clone().expect("a proven present value") {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value24362, _, _, _) => {
                                                    source_ml_source2_data_property_value24362
                                                }
                                            })) || {
                                                let source_ml_source2_s24364 = shared_constructor_field3.clone();
                                                match source_ml_source2_s24364.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            }) || !(((match shared_constructor_field3.clone() {
                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value24369) => {
                                                    Some((source_ml_source2_data_property_value24369,))
                                                }
                                                _ => {
                                                    None::<(bool,)>
                                                }
                                            }).clone().expect("a proven present value")).0)) || (((((match shared_constructor_field3.clone() {
                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value24374, _) => {
                                                    Some((source_ml_source2_data_property_value24374.clone(),))
                                                }
                                                _ => {
                                                    None::<(self::ml::Big,)>
                                                }
                                            }).clone().expect("a proven present value")).0).clone()) != (match fact.clone().expect("a proven present value") {
                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(source_ml_source2_data_property_value24375, _, _, _, _) => {
                                                    source_ml_source2_data_property_value24375.clone()
                                                }
                                            }))) {
                                                self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110])
                                            } else {
                                                if (((!(self::ml_math::is_safe_integer(shared_constructor_field5)) || (shared_constructor_field5 < (0f64))) || {
                                                    let source_ml_source2_same_value_operand24380 = shared_constructor_field5;
                                                    {
                                                        let source_ml_source2_same_value_operand24381 = (-0f64);
                                                        (((source_ml_source2_same_value_operand24380 == source_ml_source2_same_value_operand24381) && ((source_ml_source2_same_value_operand24380 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand24380)) == ((1f64 / source_ml_source2_same_value_operand24381))))) || ((source_ml_source2_same_value_operand24380 != source_ml_source2_same_value_operand24380) && (source_ml_source2_same_value_operand24381 != source_ml_source2_same_value_operand24381)))
                                                    }
                                                }) || !(self::ml_array::includes(&{ let ml_reference_value1267 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 102, 97, 108, 108, 98, 97, 99, 107]), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![98, 111, 100, 121])]))).borrow().clone(); ml_reference_value1267 }, &shared_constructor_field6.clone(), 0f64, false, |a, b| a == b))) {
                                                    self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101])
                                                } else {
                                                    {
                                                        let source_ml_source1_temporary_values1103 = std::rc::Rc::new(std::cell::RefCell::new(vec![shared_constructor_field3.clone(), shared_constructor_field8.clone(), shared_constructor_field9.clone()]));
                                                        {
                                                            let source_ml_source1_temporary_index1103 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s24386 = self::source_ml_source1_temporary_read_java_script_species_construction_state_error_loop1106(heap.clone(), source_ml_source1_temporary_values1103.clone(), source_ml_source1_temporary_index1103.clone());
                                                                match source_ml_source2_s24386.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Result::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Done => {
                                                                        {
                                                                            let fn_ = self::read_java_script_executable_call_function(program.clone(), heap.clone(), (((match shared_constructor_field3.clone() {
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value24388, _) => {
                                                                                    Some((source_ml_source2_data_property_value24388.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(self::ml::Big,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone());
                                                                            {
                                                                                let mode = if match fact.clone().expect("a proven present value") {
                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value24389, _) => {
                                                                                        source_ml_source2_data_property_value24389
                                                                                    }
                                                                                } {
                                                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99])
                                                                                } else {
                                                                                    ({
                                                                                        let source_ml_source2_optional_chain_receiver24390 = (fn_.clone(),);
                                                                                        if match (source_ml_source2_optional_chain_receiver24390.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                            Some({
                                                                                                let source_ml_source1_temporary_optional_chain1104 = ((source_ml_source2_optional_chain_receiver24390.clone()).0).clone().expect("a proven present value");
                                                                                                match source_ml_source1_temporary_optional_chain1104.clone() {
                                                                                                    self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value24391, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value24391.clone()
                                                                                                    }
                                                                                                }
                                                                                            })
                                                                                        } else {
                                                                                            None::<self::ml_string::Text>
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))
                                                                                };
                                                                                if ((shared_constructor_field7 != mode) || ((!(match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(match fact.clone().expect("a proven present value") {
                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, _, source_ml_source2_data_property_value24394, _) => {
                                                                                        source_ml_source2_data_property_value24394
                                                                                    }
                                                                                })) && ((match fn_.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value24397, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value24397.clone()
                                                                                    }
                                                                                }) != (match fact.clone().expect("a proven present value") {
                                                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value24398, _, _) => {
                                                                                        source_ml_source2_data_property_value24398.clone()
                                                                                    }
                                                                                })))) {
                                                                                    self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 109, 111, 100, 101, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                                                } else {
                                                                                    if (self::ml_array::includes(&{ let ml_reference_value1268 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), self::ml_string::Text::from_units(vec![98, 111, 100, 121])]))).borrow().clone(); ml_reference_value1268 }, &shared_constructor_field6.clone(), 0f64, false, |a, b| a == b) && {
                                                                                        let source_ml_source2_s24401 = shared_constructor_field8.clone();
                                                                                        match source_ml_source2_s24401.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    }) {
                                                                                        self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source2_s24433 = if (shared_constructor_field6 == (self::ml_string::Text::from_units(vec![98, 111, 100, 121]))) {
                                                                                                if (({
                                                                                                    let source_ml_source2_s24437 = shared_constructor_field9.clone();
                                                                                                    match source_ml_source2_s24437.clone() {
                                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                            false
                                                                                                        }
                                                                                                        _ => {
                                                                                                            true
                                                                                                        }
                                                                                                    }
                                                                                                } || ((match shared_constructor_field9.clone() {
                                                                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value24442) => {
                                                                                                        Some((source_ml_source2_data_property_value24442,))
                                                                                                    }
                                                                                                    _ => {
                                                                                                        None::<(bool,)>
                                                                                                    }
                                                                                                }).clone().expect("a proven present value")).0) || ((shared_constructor_field7 != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) && (shared_constructor_field7 != (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]))))) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107Return(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 98, 111, 100, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 119, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                                                                } else {
                                                                                                    {
                                                                                                        let object = {
                                                                                                            let source_ml_source2_array_read_value24451 = shared_constructor_field1.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_array_read_index24452 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field9.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value24450, _) => {
                                                                                                                        Some((source_ml_source2_data_property_value24450.clone(),))
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        None::<(self::ml::Big,)>
                                                                                                                    }
                                                                                                                }).clone().expect("a proven present value")).0).clone());
                                                                                                                { let ml_reference_array1269 = source_ml_source2_array_read_value24451.clone(); let ml_reference_position1269 = source_ml_source2_array_read_index24452; let ml_reference_projection1269 = self::ml_array::optional_at(&*ml_reference_array1269.borrow(), self::ml_array::number_index(ml_reference_position1269)); ml_reference_projection1269 }
                                                                                                            }
                                                                                                        };
                                                                                                        if (match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match object.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                                                                false
                                                                                                            }
                                                                                                            _ => {
                                                                                                                true
                                                                                                            }
                                                                                                        }) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107Return(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                                                                        } else {
                                                                                                            self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107Next
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107Next
                                                                                            };
                                                                                            match source_ml_source2_s24433.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107Next => {
                                                                                                    {
                                                                                                        let source_ml_source2_s24506 = if !(match (match state.clone() {
                                                                                                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24507, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value24507.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                            }
                                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                            if (((!(self::ml_array::includes(&{ let ml_reference_value1270 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![98, 111, 100, 121])]))).borrow().clone(); ml_reference_value1270 }, &shared_constructor_field6.clone(), 0f64, false, |a, b| a == b)) || {
                                                                                                                let source_ml_source2_s24511 = match (match state.clone() {
                                                                                                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24514, _, _) => {
                                                                                                                        ((source_ml_source2_data_property_value24514.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                                    }
                                                                                                                }).clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(source_ml_source2_data_property_value24515, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value24515 = *source_ml_source2_data_property_value24515;
                                                                                                                        source_ml_source2_data_property_value24515.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                match source_ml_source2_s24511.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                        false
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        true
                                                                                                                    }
                                                                                                                }
                                                                                                            }) || !(((match match (match state.clone() {
                                                                                                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24516, _, _) => {
                                                                                                                    ((source_ml_source2_data_property_value24516.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                                }
                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(source_ml_source2_data_property_value24517, _, _) => {
                                                                                                                    let source_ml_source2_data_property_value24517 = *source_ml_source2_data_property_value24517;
                                                                                                                    source_ml_source2_data_property_value24517.clone()
                                                                                                                }
                                                                                                            } {
                                                                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value24518) => {
                                                                                                                    Some((source_ml_source2_data_property_value24518,))
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    None::<(bool,)>
                                                                                                                }
                                                                                                            }).clone().expect("a proven present value")).0)) || (({ let ml_reference_array1271 = match (match state.clone() {
                                                                                                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24521, _, _) => {
                                                                                                                    ((source_ml_source2_data_property_value24521.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                                }
                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, _, source_ml_source2_data_property_value24522) => {
                                                                                                                    source_ml_source2_data_property_value24522.clone()
                                                                                                                }
                                                                                                            }; let ml_reference_projection1271 = ml_reference_array1271.borrow().len() as f64; ml_reference_projection1271 }) > (2f64))) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108Return(self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 47, 98, 111, 100, 121, 32, 99, 97, 108, 108]))
                                                                                                            } else {
                                                                                                                {
                                                                                                                    let source_ml_source1_temporary_values1105 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![match (match state.clone() {
                                                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24523, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value24523.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                                        }
                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(source_ml_source2_data_property_value24524, _, _) => {
                                                                                                                            let source_ml_source2_data_property_value24524 = *source_ml_source2_data_property_value24524;
                                                                                                                            source_ml_source2_data_property_value24524.clone()
                                                                                                                        }
                                                                                                                    }, match (match state.clone() {
                                                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24525, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value24525.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                                        }
                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, source_ml_source2_data_property_value24526, _) => {
                                                                                                                            let source_ml_source2_data_property_value24526 = *source_ml_source2_data_property_value24526;
                                                                                                                            source_ml_source2_data_property_value24526.clone()
                                                                                                                        }
                                                                                                                    }], { let ml_reference_value1272 = (match (match state.clone() {
                                                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24527, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value24527.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                                        }
                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, _, source_ml_source2_data_property_value24528) => {
                                                                                                                            source_ml_source2_data_property_value24528.clone()
                                                                                                                        }
                                                                                                                    }).borrow().clone(); ml_reference_value1272 })));
                                                                                                                    {
                                                                                                                        let source_ml_source1_temporary_index1105 = self::ml::Big::from_i128(0);
                                                                                                                        {
                                                                                                                            let source_ml_source2_s24529 = self::source_ml_source1_temporary_read_java_script_species_construction_state_error_loop1109(heap.clone(), source_ml_source1_temporary_values1105.clone(), source_ml_source1_temporary_index1105.clone());
                                                                                                                            match source_ml_source2_s24529.clone() {
                                                                                                                                self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Result::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Done => {
                                                                                                                                    if ((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![98, 111, 100, 121]))) && (((!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24532, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value24532.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                                        self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(source_ml_source2_data_property_value24533, _, _) => {
                                                                                                                                            let source_ml_source2_data_property_value24533 = *source_ml_source2_data_property_value24533;
                                                                                                                                            source_ml_source2_data_property_value24533.clone()
                                                                                                                                        }
                                                                                                                                    }, shared_constructor_field3.clone())) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24535, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value24535.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                                        self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, source_ml_source2_data_property_value24536, _) => {
                                                                                                                                            let source_ml_source2_data_property_value24536 = *source_ml_source2_data_property_value24536;
                                                                                                                                            source_ml_source2_data_property_value24536.clone()
                                                                                                                                        }
                                                                                                                                    }, shared_constructor_field9.clone()))) || (({ let ml_reference_array1273 = match (match state.clone() {
                                                                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24540, _, _) => {
                                                                                                                                            ((source_ml_source2_data_property_value24540.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }).clone().expect("a proven present value") {
                                                                                                                                        self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, _, source_ml_source2_data_property_value24541) => {
                                                                                                                                            source_ml_source2_data_property_value24541.clone()
                                                                                                                                        }
                                                                                                                                    }; let ml_reference_projection1273 = ml_reference_array1273.borrow().len() as f64; ml_reference_projection1273 }) != (1f64))) || !(self::test_java_script_semantic_same_value(({
                                                                                                                                        let source_ml_source2_array_read_value24544 = match (match state.clone() {
                                                                                                                                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value24542, _, _) => {
                                                                                                                                                ((source_ml_source2_data_property_value24542.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                                                            }
                                                                                                                                        }).clone().expect("a proven present value") {
                                                                                                                                            self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, _, source_ml_source2_data_property_value24543) => {
                                                                                                                                                source_ml_source2_data_property_value24543.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_array_read_index24545 = 0f64;
                                                                                                                                            { let ml_reference_array1274 = source_ml_source2_array_read_value24544.clone(); let ml_reference_position1274 = source_ml_source2_array_read_index24545; let ml_reference_projection1274 = self::ml_array::optional_at(&*ml_reference_array1274.borrow(), self::ml_array::number_index(ml_reference_position1274)); ml_reference_projection1274 }
                                                                                                                                        }
                                                                                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::JavaScriptSemanticValue::Number(shared_constructor_field5))))) {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 98, 111, 100, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 101, 101, 44, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 108, 101, 110, 103, 116, 104, 32, 97, 114, 103, 117, 109, 101, 110, 116]))
                                                                                                                                    } else {
                                                                                                                                        self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108Next
                                                                                                                                    }
                                                                                                                                }
                                                                                                                                self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Result::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1109Return(source_ml_source1_temporary_result1109) => {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108Return(source_ml_source1_temporary_result1109.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        } else {
                                                                                                            if ((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]))) || (shared_constructor_field6 == (self::ml_string::Text::from_units(vec![98, 111, 100, 121])))) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108Return(self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 47, 98, 111, 100, 121, 32, 112, 104, 97, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108]))
                                                                                                            } else {
                                                                                                                self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108Next
                                                                                                            }
                                                                                                        };
                                                                                                        match source_ml_source2_s24506.clone() {
                                                                                                            self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108Next => {
                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                            }
                                                                                                            self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1108Return(source_ml_source1_temporary_result1108) => {
                                                                                                                source_ml_source1_temporary_result1108.clone()
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorJoin1107Return(source_ml_source1_temporary_result1109_2) => {
                                                                                                    source_ml_source1_temporary_result1109_2.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Result::MlSource1TemporaryReadJavaScriptSpeciesConstructionStateErrorLoop1106Return(source_ml_source1_temporary_result1109_3) => {
                                                                        source_ml_source1_temporary_result1109_3.clone()
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
                },
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_species_construction(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    constructor: self::JavaScriptSemanticValue,
    fact: self::JavaScriptArrayConstructorFact,
    length: f64,
) -> self::JavaScriptSpeciesConstructionTransition {
    match fact.clone() {
        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
        ) => {
            let seed = {
                let source_ml_source2_constructor_field24563 = {
                    let source_ml_source2_nominal_field24553 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_nominal_field24555 = shared_constructor_field2;
                        {
                            let source_ml_source2_nominal_field24557 =
                                shared_constructor_field3.clone();
                            {
                                let source_ml_source2_nominal_field24559 =
                                    shared_constructor_field4;
                                {
                                    let source_ml_source2_nominal_field24561 =
                                        shared_constructor_field5.clone();
                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                        source_ml_source2_nominal_field24553.clone(),
                                        source_ml_source2_nominal_field24555,
                                        source_ml_source2_nominal_field24557.clone(),
                                        source_ml_source2_nominal_field24559,
                                        source_ml_source2_nominal_field24561.clone(),
                                    )
                                }
                            }
                        }
                    }
                };
                {
                    let source_ml_source2_constructor_field24567 =
                        self::JavaScriptSemanticValue::Undefined;
                    {
                        let source_ml_source2_constructor_field24568 =
                            self::JavaScriptSemanticValue::Undefined;
                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                            Box::new(constructor.clone()),
                            Box::new(source_ml_source2_constructor_field24563.clone()),
                            length,
                            self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116]),
                            self::ml_string::Text::from_units(vec![]),
                            Box::new(source_ml_source2_constructor_field24567.clone()),
                            Box::new(source_ml_source2_constructor_field24568.clone()),
                            None::<(Option<self::JavaScriptSpeciesConstructionCall>,)>,
                            None::<(Option<self::ml_string::Text>,)>,
                            None::<(Option<self::JavaScriptConstructionState>,)>,
                        )
                    }
                }
            };
            {
                let transition = self::adapt_java_script_species_construction(
                    seed.clone(),
                    self::begin_java_script_construction(
                        program.clone(),
                        heap.clone(),
                        directory.clone(),
                        constructor.clone(),
                        constructor.clone(),
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            self::JavaScriptSemanticValue::Number(length),
                        ])),
                    ),
                );
                match transition.clone() {
                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionContinue(
                        _,
                        state,
                    ) => {
                        let state = *state;
                        {
                            let error = self::read_java_script_species_construction_state_error(
                                program.clone(),
                                heap.clone(),
                                directory.clone(),
                                state.clone(),
                            );
                            if (error == (self::ml_string::Text::from_units(vec![]))) {
                                transition.clone()
                            } else {
                                self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionInvalid(Box::new(heap.clone()), error.clone())
                            }
                        }
                    }
                    _ => transition.clone(),
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_species_prototype(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptSpeciesConstructionState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptSpeciesConstructionTransition {
    match state.clone() {
        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
            shared_constructor_field11,
            _,
            _,
            _,
            shared_constructor_field12,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field11 = *shared_constructor_field11;
            if !(match (
                match state.clone() {
                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value24570,
                    ) => ((source_ml_source2_data_property_value24570
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                self::adapt_java_script_species_construction(
                    state.clone(),
                    self::continue_java_script_construction_prototype(
                        heap.clone(),
                        directory.clone(),
                        (match state.clone() {
                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value24571,
                            ) => ((source_ml_source2_data_property_value24571
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                            .0)
                                .clone(),
                        })
                        .clone()
                        .expect("a proven present value"),
                        value.clone(),
                    ),
                )
            } else {
                {
                    let source_ml_source1_temporary_binding1110 = state.clone();
                    {
                        let prior_call = match source_ml_source1_temporary_binding1110.clone() {
                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value24572,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value24572
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<self::JavaScriptSpeciesConstructionCall>,)
                                }))
                            .0)
                                .clone(),
                        };
                        {
                            let base = match source_ml_source1_temporary_binding1110.clone() {
                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                    source_ml_source2_nominal_rest_field24573,
                                    source_ml_source2_nominal_rest_field24574,
                                    source_ml_source2_nominal_rest_field24575,
                                    source_ml_source2_nominal_rest_field24576,
                                    source_ml_source2_nominal_rest_field24577,
                                    source_ml_source2_nominal_rest_field24578,
                                    source_ml_source2_nominal_rest_field24579,
                                    _,
                                    source_ml_source2_nominal_rest_field24580,
                                    source_ml_source2_nominal_rest_field24581,
                                ) => {
                                    let source_ml_source2_nominal_rest_field24573 =
                                        *source_ml_source2_nominal_rest_field24573;
                                    let source_ml_source2_nominal_rest_field24574 =
                                        *source_ml_source2_nominal_rest_field24574;
                                    let source_ml_source2_nominal_rest_field24578 =
                                        *source_ml_source2_nominal_rest_field24578;
                                    let source_ml_source2_nominal_rest_field24579 =
                                        *source_ml_source2_nominal_rest_field24579;
                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                        Box::new(source_ml_source2_nominal_rest_field24573.clone()),
                                        Box::new(source_ml_source2_nominal_rest_field24574.clone()),
                                        source_ml_source2_nominal_rest_field24575,
                                        source_ml_source2_nominal_rest_field24576.clone(),
                                        source_ml_source2_nominal_rest_field24577.clone(),
                                        Box::new(source_ml_source2_nominal_rest_field24578.clone()),
                                        Box::new(source_ml_source2_nominal_rest_field24579.clone()),
                                        None::<(Option<self::JavaScriptSpeciesConstructionCall>,)>,
                                        source_ml_source2_nominal_rest_field24580.clone(),
                                        source_ml_source2_nominal_rest_field24581.clone(),
                                    )
                                }
                            };
                            match base.clone() {
                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                    shared_constructor_field1,
                                    shared_constructor_field2,
                                    shared_constructor_field3,
                                    shared_constructor_field4,
                                    shared_constructor_field5,
                                    shared_constructor_field6,
                                    shared_constructor_field7,
                                    shared_constructor_field8,
                                    shared_constructor_field9,
                                    shared_constructor_field10,
                                ) => {
                                    let shared_constructor_field1 = *shared_constructor_field1;
                                    let shared_constructor_field2 = *shared_constructor_field2;
                                    let shared_constructor_field6 = *shared_constructor_field6;
                                    let shared_constructor_field7 = *shared_constructor_field7;
                                    match value.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                            self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionContinue(Box::new(heap.clone()), Box::new({
                                                let source_ml_source2_nominal_field24585 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field24587 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field24589 = shared_constructor_field3;
                                                        {
                                                            let source_ml_source2_nominal_field24591 = shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field24593 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field24595 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24597 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field24599 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field24601 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field24603 = shared_constructor_field10.clone();
                                                                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(Box::new(source_ml_source2_nominal_field24585.clone()), Box::new(source_ml_source2_nominal_field24587.clone()), source_ml_source2_nominal_field24589, self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), source_ml_source2_nominal_field24593.clone(), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field24597.clone()), source_ml_source2_nominal_field24599.clone(), source_ml_source2_nominal_field24601.clone(), source_ml_source2_nominal_field24603.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }))
                                        }
                                        _ => {
                                            {
                                                let root = { let ml_values1275 = { let ml_reference_value1276 = ((match directory.clone() {
                                                    self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(_, _, _, _, _, source_ml_source2_data_property_value24607) => {
                                                        ((source_ml_source2_data_property_value24607.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptConstructorRealm>>>>,))).0).clone()
                                                    }
                                                }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptConstructorRealm>::new())))).borrow().clone(); ml_reference_value1276 }; let ml_callback1275 = { let state = state.clone(); std::rc::Rc::new(move |row: self::JavaScriptConstructorRealm| -> bool { ((match row.clone() {
                                                    self::JavaScriptConstructorRealm::ConstructorRealm(source_ml_source2_data_property_value24611, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value24611.clone()
                                                    }
                                                }) == (match match state.clone() {
                                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, source_ml_source2_data_property_value24612, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value24612 = *source_ml_source2_data_property_value24612;
                                                        source_ml_source2_data_property_value24612.clone()
                                                    }
                                                } {
                                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, _, source_ml_source2_data_property_value24613, _, _) => {
                                                        source_ml_source2_data_property_value24613.clone()
                                                    }
                                                })) }) }; ml_values1275.iter().cloned().enumerate().find_map(|(ml_index1275, ml_value1275)| if ml_callback1275(ml_value1275.clone()) { Some(ml_value1275) } else { None }) };
                                                if match (root.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionNeedAuthority(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field24615 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field24617 = shared_constructor_field2.clone();
                                                            {
                                                                let source_ml_source2_nominal_field24619 = shared_constructor_field3;
                                                                {
                                                                    let source_ml_source2_nominal_field24621 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24623 = shared_constructor_field5.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field24625 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field24627 = shared_constructor_field7.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field24629 = shared_constructor_field8.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field24631 = shared_constructor_field9.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field24633 = shared_constructor_field10.clone();
                                                                                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(Box::new(source_ml_source2_nominal_field24615.clone()), Box::new(source_ml_source2_nominal_field24617.clone()), source_ml_source2_nominal_field24619, self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 102, 97, 108, 108, 98, 97, 99, 107]), source_ml_source2_nominal_field24623.clone(), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field24627.clone()), source_ml_source2_nominal_field24629.clone(), source_ml_source2_nominal_field24631.clone(), source_ml_source2_nominal_field24633.clone())
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }), Box::new(shared_constructor_field11.clone()))
                                                } else {
                                                    self::JavaScriptSpeciesConstructionTransition::SpeciesConstructionContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field24643 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field24645 = shared_constructor_field2.clone();
                                                            {
                                                                let source_ml_source2_nominal_field24647 = shared_constructor_field3;
                                                                {
                                                                    let source_ml_source2_nominal_field24649 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24651 = shared_constructor_field5.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field24653 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field24655 = shared_constructor_field7.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field24657 = shared_constructor_field8.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field24659 = shared_constructor_field9.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field24661 = shared_constructor_field10.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override24663 = if (shared_constructor_field12 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]))) {
                                                                                                    match root.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptConstructorRealm::ConstructorRealm(_, _, source_ml_source2_data_property_value24640, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                            let source_ml_source2_data_property_value24640 = *source_ml_source2_data_property_value24640;
                                                                                                            source_ml_source2_data_property_value24640.clone()
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    match root.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptConstructorRealm::ConstructorRealm(_, source_ml_source2_data_property_value24641, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                            let source_ml_source2_data_property_value24641 = *source_ml_source2_data_property_value24641;
                                                                                                            source_ml_source2_data_property_value24641.clone()
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(Box::new(source_ml_source2_nominal_field24643.clone()), Box::new(source_ml_source2_nominal_field24645.clone()), source_ml_source2_nominal_field24647, self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), source_ml_source2_nominal_field24651.clone(), Box::new(source_ml_source2_nominal_override24663.clone()), Box::new(source_ml_source2_nominal_field24655.clone()), source_ml_source2_nominal_field24657.clone(), source_ml_source2_nominal_field24659.clone(), source_ml_source2_nominal_field24661.clone())
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
                                                    }))
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
