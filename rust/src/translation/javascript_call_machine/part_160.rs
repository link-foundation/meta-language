// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptInstanceOfCall;
use super::JavaScriptInstanceOfState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPropertyDefinitionsCall;
use super::JavaScriptPropertyDefinitionsState;
use super::JavaScriptPropertyReflectionState;
use super::JavaScriptSemanticValue;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_pending_object_observation_error;
use super::read_java_script_instance_of_state_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_definitions_state_error;
use super::read_java_script_property_reflection_state_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_object_error(
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
            shared_constructor_field8,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field9,
            _,
            _,
        ) => match frame.clone() {
            self::JavaScriptCallFrame::CallFrame(
                _,
                shared_constructor_field6,
                shared_constructor_field7,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => match pending.clone() {
                self::JavaScriptCallPending::WaitPropertyDefinitions(
                    destination,
                    realm,
                    pending_state,
                ) => {
                    let pending_state = *pending_state;
                    match pending_state.clone() {
                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
                            shared_constructor_field1,
                            shared_constructor_field2,
                            shared_constructor_field3,
                            shared_constructor_field4,
                            shared_constructor_field5,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let shared_constructor_field2 = *shared_constructor_field2;
                            let shared_constructor_field3 = *shared_constructor_field3;
                            let shared_constructor_field4 = *shared_constructor_field4;
                            let shared_constructor_field5 = *shared_constructor_field5;
                            if !(shared_constructor_field6 > (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105,
                                    111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99,
                                    116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 116,
                                    97, 114, 103, 101, 116, 32, 97, 116, 116, 114, 105, 98, 117,
                                    116, 101, 115, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110,
                                    100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104,
                                    105, 108, 100,
                                ])
                            } else {
                                {
                                    let instruction = {
                                        let source_ml_source2_array_read_value30725 =
                                            shared_constructor_field9.clone();
                                        {
                                            let source_ml_source2_array_read_index30726 =
                                                (shared_constructor_field6 - 1f64);
                                            {
                                                let ml_reference_array1821 =
                                                    source_ml_source2_array_read_value30725.clone();
                                                let ml_reference_position1821 =
                                                    source_ml_source2_array_read_index30726;
                                                let ml_reference_projection1821 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1821.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1821,
                                                        ),
                                                    );
                                                ml_reference_projection1821
                                            }
                                        }
                                    };
                                    {
                                        let selected = if (shared_constructor_field1
                                            == (self::ml_string::Text::from_units(vec![
                                                79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97,
                                                116, 101,
                                            ]))) {
                                            shared_constructor_field4.clone()
                                        } else {
                                            shared_constructor_field2.clone()
                                        };
                                        if (match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::PropertyDefinitions(_, _, _, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }) {
                                                    self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 116, 97, 114, 103, 101, 116, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100])
                                                } else {
                                                    if ((((((((((match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::PropertyDefinitions(source_ml_source2_data_property_value30736, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value30736,))
                                                        }
                                                        _ => {
                                                            None::<(f64,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0) != destination) || (((((match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::PropertyDefinitions(_, source_ml_source2_data_property_value30739, _, _) => {
                                                            Some((source_ml_source2_data_property_value30739.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::ml_string::Text,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field1)) || (realm != shared_constructor_field8)) || !(self::test_java_script_semantic_same_value(({
                                                        let source_ml_source2_array_read_value30745 = shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_array_read_index30746 = ((match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::PropertyDefinitions(_, _, source_ml_source2_data_property_value30744, _) => {
                                                                    Some((source_ml_source2_data_property_value30744,))
                                                                }
                                                                _ => {
                                                                    None::<(f64,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0;
                                                            { let ml_reference_array1822 = source_ml_source2_array_read_value30745.clone(); let ml_reference_position1822 = source_ml_source2_array_read_index30746; let ml_reference_projection1822 = self::ml_array::optional_at(&*ml_reference_array1822.borrow(), self::ml_array::number_index(ml_reference_position1822)); ml_reference_projection1822 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), selected.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                        let source_ml_source2_array_read_value30749 = shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_array_read_index30750 = ((match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::PropertyDefinitions(_, _, _, source_ml_source2_data_property_value30748) => {
                                                                    Some((source_ml_source2_data_property_value30748,))
                                                                }
                                                                _ => {
                                                                    None::<(f64,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0;
                                                            { let ml_reference_array1823 = source_ml_source2_array_read_value30749.clone(); let ml_reference_position1823 = source_ml_source2_array_read_index30750; let ml_reference_projection1823 = self::ml_array::optional_at(&*ml_reference_array1823.borrow(), self::ml_array::number_index(ml_reference_position1823)); ml_reference_projection1823 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field3.clone()))) || !(self::test_java_script_semantic_same_value(match program.clone() {
                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value30752, _, _, _) => {
                                                            let source_ml_source2_data_property_value30752 = *source_ml_source2_data_property_value30752;
                                                            source_ml_source2_data_property_value30752.clone()
                                                        }
                                                    }, shared_constructor_field5.clone()))) || (suspended && match (match pending_state.clone() {
                                                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value30754) => {
                                                            ((source_ml_source2_data_property_value30754.clone().unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionsCall>,))).0).clone()
                                                        }
                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                        self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 116, 97, 114, 103, 101, 116, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115, 32, 83, 121, 109, 98, 111, 108, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100])
                                                    } else {
                                                        {
                                                            let error = self::read_java_script_property_definitions_state_error(heap.clone(), pending_state.clone());
                                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                error.clone()
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
                self::JavaScriptCallPending::WaitPropertyReflection(
                    destination_2,
                    realm_2,
                    pending_state_2,
                ) => {
                    let pending_state_2 = *pending_state_2;
                    match pending_state_2.clone() {
                        self::JavaScriptPropertyReflectionState::PropertyReflection(
                            shared_constructor_field1_2,
                            shared_constructor_field2_2,
                            shared_constructor_field3_2,
                            shared_constructor_field4_2,
                            shared_constructor_field5_2,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let shared_constructor_field2_2 = *shared_constructor_field2_2;
                            let shared_constructor_field3_2 = *shared_constructor_field3_2;
                            let shared_constructor_field4_2 = *shared_constructor_field4_2;
                            let shared_constructor_field5_2 = *shared_constructor_field5_2;
                            if !(shared_constructor_field6 > (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 102, 108,
                                    101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110,
                                    115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99,
                                    101, 32, 105, 110, 112, 117, 116, 32, 107, 101, 121, 32, 114,
                                    101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112,
                                    101, 115, 32, 97, 110, 100, 32, 110, 111, 32, 105, 110, 118,
                                    101, 110, 116, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99,
                                    97, 108, 108,
                                ])
                            } else {
                                {
                                    let instruction_2 = {
                                        let source_ml_source2_array_read_value30759 =
                                            shared_constructor_field9.clone();
                                        {
                                            let source_ml_source2_array_read_index30760 =
                                                (shared_constructor_field6 - 1f64);
                                            {
                                                let ml_reference_array1824 =
                                                    source_ml_source2_array_read_value30759.clone();
                                                let ml_reference_position1824 =
                                                    source_ml_source2_array_read_index30760;
                                                let ml_reference_projection1824 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1824.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1824,
                                                        ),
                                                    );
                                                ml_reference_projection1824
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
                                        self::JavaScriptCallInstruction::PropertyReflection(
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
                                            112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101,
                                            102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101,
                                            116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108,
                                            32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 112,
                                            117, 116, 32, 107, 101, 121, 32, 114, 101, 97, 108,
                                            109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101,
                                            115, 32, 97, 110, 100, 32, 110, 111, 32, 105, 110, 118,
                                            101, 110, 116, 101, 100, 32, 99, 104, 105, 108, 100,
                                            32, 99, 97, 108, 108,
                                        ])
                                    } else {
                                        if (((((((((((match instruction_2.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::PropertyReflection(source_ml_source2_data_property_value30768, _, _, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value30768,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0) != destination_2) || (((((match instruction_2.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::PropertyReflection(_, source_ml_source2_data_property_value30771, _, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value30771.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml_string::Text,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field1_2)) || (realm_2 != shared_constructor_field8)) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value30777 = shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30778 = ((match instruction_2.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::PropertyReflection(_, _, source_ml_source2_data_property_value30776, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value30776,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        { let ml_reference_array1825 = source_ml_source2_array_read_value30777.clone(); let ml_reference_position1825 = source_ml_source2_array_read_index30778; let ml_reference_projection1825 = self::ml_array::optional_at(&*ml_reference_array1825.borrow(), self::ml_array::number_index(ml_reference_position1825)); ml_reference_projection1825 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2_2.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value30782 = shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30783 = ((match instruction_2.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::PropertyReflection(_, _, _, source_ml_source2_data_property_value30781, _, _) => {
                                                                Some((source_ml_source2_data_property_value30781,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        { let ml_reference_array1826 = source_ml_source2_array_read_value30782.clone(); let ml_reference_position1826 = source_ml_source2_array_read_index30783; let ml_reference_projection1826 = self::ml_array::optional_at(&*ml_reference_array1826.borrow(), self::ml_array::number_index(ml_reference_position1826)); ml_reference_projection1826 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field3_2.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value30787 = shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30788 = ((match instruction_2.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::PropertyReflection(_, _, _, _, source_ml_source2_data_property_value30786, _) => {
                                                                Some((source_ml_source2_data_property_value30786,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        { let ml_reference_array1827 = source_ml_source2_array_read_value30787.clone(); let ml_reference_position1827 = source_ml_source2_array_read_index30788; let ml_reference_projection1827 = self::ml_array::optional_at(&*ml_reference_array1827.borrow(), self::ml_array::number_index(ml_reference_position1827)); ml_reference_projection1827 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field4_2.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value30792 = shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30793 = ((match instruction_2.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::PropertyReflection(_, _, _, _, _, source_ml_source2_data_property_value30791) => {
                                                                Some((source_ml_source2_data_property_value30791,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        { let ml_reference_array1828 = source_ml_source2_array_read_value30792.clone(); let ml_reference_position1828 = source_ml_source2_array_read_index30793; let ml_reference_projection1828 = self::ml_array::optional_at(&*ml_reference_array1828.borrow(), self::ml_array::number_index(ml_reference_position1828)); ml_reference_projection1828 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field5_2.clone()))) || suspended) {
                                                    self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 112, 117, 116, 32, 107, 101, 121, 32, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115, 32, 97, 110, 100, 32, 110, 111, 32, 105, 110, 118, 101, 110, 116, 101, 100, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108])
                                                } else {
                                                    {
                                                        let invalid = self::read_java_script_property_reflection_state_error(heap.clone(), pending_state_2.clone());
                                                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                            invalid.clone()
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
                self::JavaScriptCallPending::WaitInstanceOf(
                    destination_3,
                    realm_3,
                    pending_state_3,
                ) => {
                    let pending_state_3 = *pending_state_3;
                    match pending_state_3.clone() {
                        self::JavaScriptInstanceOfState::InstanceOfState(
                            shared_constructor_field1_3,
                            shared_constructor_field2_3,
                            shared_constructor_field3_3,
                            shared_constructor_field4_3,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let shared_constructor_field2_3 = *shared_constructor_field2_3;
                            let shared_constructor_field3_3 = *shared_constructor_field3_3;
                            let shared_constructor_field4_3 = *shared_constructor_field4_3;
                            if !(shared_constructor_field6 > (0f64)) {
                                self::ml_string::Text::from_units(vec![
                                    105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101,
                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                    115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 110,
                                    100, 115, 32, 83, 121, 109, 98, 111, 108, 32, 111, 112, 101,
                                    114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101,
                                    97, 108, 109,
                                ])
                            } else {
                                {
                                    let instruction_3 = {
                                        let source_ml_source2_array_read_value30799 =
                                            shared_constructor_field9.clone();
                                        {
                                            let source_ml_source2_array_read_index30800 =
                                                (shared_constructor_field6 - 1f64);
                                            {
                                                let ml_reference_array1829 =
                                                    source_ml_source2_array_read_value30799.clone();
                                                let ml_reference_position1829 =
                                                    source_ml_source2_array_read_index30800;
                                                let ml_reference_projection1829 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1829.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1829,
                                                        ),
                                                    );
                                                ml_reference_projection1829
                                            }
                                        }
                                    };
                                    if (match (instruction_3.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || match instruction_3
                                        .clone()
                                        .expect("a proven present value")
                                    {
                                        self::JavaScriptCallInstruction::InstanceOf(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => false,
                                        _ => true,
                                    }) {
                                        self::ml_string::Text::from_units(vec![
                                            105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32,
                                            114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                                            32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101,
                                            99, 116, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32,
                                            111, 112, 101, 114, 97, 110, 100, 115, 32, 83, 121,
                                            109, 98, 111, 108, 32, 111, 112, 101, 114, 97, 116,
                                            105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108,
                                            109,
                                        ])
                                    } else {
                                        if ((((((((((match instruction_3.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::InstanceOf(source_ml_source2_data_property_value30807, _, _, _, _) => {
                                                        Some((source_ml_source2_data_property_value30807,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0) != destination_3) || (((((match instruction_3.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::InstanceOf(_, _, _, _, source_ml_source2_data_property_value30810) => {
                                                        Some((source_ml_source2_data_property_value30810.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml_string::Text,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field1_3)) || (realm_3 != shared_constructor_field8)) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value30816 = shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30817 = ((match instruction_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::InstanceOf(_, source_ml_source2_data_property_value30815, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value30815,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        { let ml_reference_array1830 = source_ml_source2_array_read_value30816.clone(); let ml_reference_position1830 = source_ml_source2_array_read_index30817; let ml_reference_projection1830 = self::ml_array::optional_at(&*ml_reference_array1830.borrow(), self::ml_array::number_index(ml_reference_position1830)); ml_reference_projection1830 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2_3.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value30821 = shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30822 = ((match instruction_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::InstanceOf(_, _, source_ml_source2_data_property_value30820, _, _) => {
                                                                Some((source_ml_source2_data_property_value30820,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        { let ml_reference_array1831 = source_ml_source2_array_read_value30821.clone(); let ml_reference_position1831 = source_ml_source2_array_read_index30822; let ml_reference_projection1831 = self::ml_array::optional_at(&*ml_reference_array1831.borrow(), self::ml_array::number_index(ml_reference_position1831)); ml_reference_projection1831 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field3_3.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value30826 = shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_array_read_index30827 = ((match instruction_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::InstanceOf(_, _, _, source_ml_source2_data_property_value30825, _) => {
                                                                Some((source_ml_source2_data_property_value30825,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0;
                                                        { let ml_reference_array1832 = source_ml_source2_array_read_value30826.clone(); let ml_reference_position1832 = source_ml_source2_array_read_index30827; let ml_reference_projection1832 = self::ml_array::optional_at(&*ml_reference_array1832.borrow(), self::ml_array::number_index(ml_reference_position1832)); ml_reference_projection1832 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field4_3.clone()))) || (suspended && match (match pending_state_3.clone() {
                                                    self::JavaScriptInstanceOfState::InstanceOfState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value30829, _, _) => {
                                                        ((source_ml_source2_data_property_value30829.clone().unwrap_or_else(|| (None::<self::JavaScriptInstanceOfCall>,))).0).clone()
                                                    }
                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                    self::ml_string::Text::from_units(vec![105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 83, 121, 109, 98, 111, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                } else {
                                                    {
                                                        let invalid_2 = self::read_java_script_instance_of_state_error(heap.clone(), pending_state_3.clone());
                                                        if (invalid_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                            invalid_2.clone()
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
                self::JavaScriptCallPending::WaitObjectPrototype(destination_4, realm_4, input) => {
                    let input = *input;
                    if !(shared_constructor_field6 > (0f64)) {
                        self::ml_string::Text::from_units(vec![
                            112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116,
                            111, 116, 121, 112, 101, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32,
                            114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                            117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 112, 117,
                            116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100,
                            32, 114, 101, 97, 108, 109,
                        ])
                    } else {
                        {
                            let instruction_4 = {
                                let source_ml_source2_array_read_value30834 =
                                    shared_constructor_field9.clone();
                                {
                                    let source_ml_source2_array_read_index30835 =
                                        (shared_constructor_field6 - 1f64);
                                    {
                                        let ml_reference_array1833 =
                                            source_ml_source2_array_read_value30834.clone();
                                        let ml_reference_position1833 =
                                            source_ml_source2_array_read_index30835;
                                        let ml_reference_projection1833 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array1833.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position1833,
                                                ),
                                            );
                                        ml_reference_projection1833
                                    }
                                }
                            };
                            if (match (instruction_4.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } || match instruction_4.clone().expect("a proven present value") {
                                self::JavaScriptCallInstruction::ObjectObservation(_, _, _, _) => {
                                    false
                                }
                                _ => true,
                            }) {
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111,
                                    116, 111, 116, 121, 112, 101, 32, 109, 101, 116, 97, 100, 97,
                                    116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                    115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99,
                                    101, 32, 105, 110, 112, 117, 116, 32, 111, 112, 101, 114, 97,
                                    116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108,
                                    109,
                                ])
                            } else {
                                if ((((((((((((match instruction_4.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::ObjectObservation(_, source_ml_source2_data_property_value30841, _, _) => {
                                                Some((source_ml_source2_data_property_value30841.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml_string::Text,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone()) != (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102]))) || ((((match instruction_4.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::ObjectObservation(source_ml_source2_data_property_value30843, _, _, _) => {
                                                Some((source_ml_source2_data_property_value30843,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != destination_4)) || (realm_4 != shared_constructor_field8)) || !(self::test_java_script_semantic_same_value(({
                                            let source_ml_source2_array_read_value30848 = shared_constructor_field7.clone();
                                            {
                                                let source_ml_source2_array_read_index30849 = ((match instruction_4.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::ObjectObservation(_, _, source_ml_source2_data_property_value30847, _) => {
                                                        Some((source_ml_source2_data_property_value30847,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0;
                                                { let ml_reference_array1834 = source_ml_source2_array_read_value30848.clone(); let ml_reference_position1834 = source_ml_source2_array_read_index30849; let ml_reference_projection1834 = self::ml_array::optional_at(&*ml_reference_array1834.borrow(), self::ml_array::number_index(ml_reference_position1834)); ml_reference_projection1834 }
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), input.clone()))) || match input.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        }) || match input.clone() {
                                            self::JavaScriptSemanticValue::Null => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        }) || match input.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                true
                                            }
                                            _ => {
                                                false
                                            }
                                        }) || suspended) {
                                            self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 112, 117, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                        } else {
                                            {
                                                let invalid_3 = self::read_java_script_ordinary_value_error(heap.clone(), input.clone());
                                                if (invalid_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                    invalid_3.clone()
                                                } else {
                                                    self::ml_string::Text::from_units(vec![])
                                                }
                                            }
                                        }
                            }
                        }
                    }
                }
                _ => self::read_java_script_call_pending_object_observation_error(
                    program.clone(),
                    heap.clone(),
                    frame.clone(),
                    fn_.clone(),
                    suspended,
                    pending.clone(),
                ),
            },
        },
    }
}
