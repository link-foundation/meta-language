// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorClassification;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArrayCreationCall;
use super::JavaScriptArrayCreationFallback;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptObjectFromEntriesErrorRequest;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327;
use super::classify_java_script_array_species_constructor;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_creation_state_error;
use super::read_java_script_call_pending_callable_construction_error;
use super::read_java_script_construction_state_error;
use super::read_java_script_known_array_constructor_fact;
use super::step_java_script_array_creation;
use super::test_java_script_pending_call_constructor_failure;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_construction_error(
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
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field11,
            _,
            _,
        ) => match frame.clone() {
            self::JavaScriptCallFrame::CallFrame(
                _,
                shared_constructor_field7,
                shared_constructor_field8,
                shared_constructor_field9,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => match pending.clone() {
                self::JavaScriptCallPending::WaitArrayCreation(
                    destination,
                    realm,
                    state,
                    pending_construction,
                    pending_fallback,
                    error,
                ) => {
                    let state = *state;
                    match state.clone() {
                        self::JavaScriptArrayCreationState::ArrayCreationState(
                            shared_constructor_field1,
                            shared_constructor_field2,
                            shared_constructor_field3,
                            shared_constructor_field4,
                            shared_constructor_field5,
                            shared_constructor_field6,
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
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let shared_constructor_field2 = *shared_constructor_field2;
                            let shared_constructor_field4 = *shared_constructor_field4;
                            let shared_constructor_field5 = *shared_constructor_field5;
                            let shared_constructor_field6 = *shared_constructor_field6;
                            {
                                let invalid_instruction = self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111,
                                    110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                                    32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118,
                                    101, 114, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32,
                                    114, 111, 111, 116, 115, 32, 83, 121, 109, 98, 111, 108, 115,
                                    32, 97, 110, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111,
                                    112, 101, 114, 97, 116, 105, 111, 110,
                                ]);
                                if !(shared_constructor_field7 > (0f64)) {
                                    invalid_instruction.clone()
                                } else {
                                    {
                                        let instruction = {
                                            let source_ml_source2_array_read_value30985 =
                                                shared_constructor_field11.clone();
                                            {
                                                let source_ml_source2_array_read_index30986 =
                                                    (shared_constructor_field7 - 1f64);
                                                {
                                                    let ml_reference_array1850 =
                                                        source_ml_source2_array_read_value30985
                                                            .clone();
                                                    let ml_reference_position1850 =
                                                        source_ml_source2_array_read_index30986;
                                                    let ml_reference_projection1850 =
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array1850.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position1850,
                                                            ),
                                                        );
                                                    ml_reference_projection1850
                                                }
                                            }
                                        };
                                        if (match (instruction.clone(), None::<()>) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        } || match instruction
                                            .clone()
                                            .expect("a proven present value")
                                        {
                                            self::JavaScriptCallInstruction::ArrayCreation(
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => false,
                                            _ => true,
                                        }) {
                                            invalid_instruction.clone()
                                        } else {
                                            if ((((((((((((match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ArrayCreation(source_ml_source2_data_property_value30992, _, _, _) => {
                                                            Some((source_ml_source2_data_property_value30992,))
                                                        }
                                                        _ => {
                                                            None::<(f64,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0) != destination) || (realm != shared_constructor_field10)) || (((((match instruction.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ArrayCreation(_, source_ml_source2_data_property_value30997, _, _) => {
                                                            Some((source_ml_source2_data_property_value30997.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::ml_string::Text,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field1)) || !(self::test_java_script_semantic_same_value(({
                                                        let source_ml_source2_array_read_value31000 = shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31001 = 0f64;
                                                            { let ml_reference_array1851 = source_ml_source2_array_read_value31000.clone(); let ml_reference_position1851 = source_ml_source2_array_read_index31001; let ml_reference_projection1851 = self::ml_array::optional_at(&*ml_reference_array1851.borrow(), self::ml_array::number_index(ml_reference_position1851)); ml_reference_projection1851 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field2.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                        let source_ml_source2_array_read_value31005 = shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31006 = ((match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::ArrayCreation(_, _, source_ml_source2_data_property_value31004, _) => {
                                                                    Some((source_ml_source2_data_property_value31004,))
                                                                }
                                                                _ => {
                                                                    None::<(f64,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0;
                                                            { let ml_reference_array1852 = source_ml_source2_array_read_value31005.clone(); let ml_reference_position1852 = source_ml_source2_array_read_index31006; let ml_reference_projection1852 = self::ml_array::optional_at(&*ml_reference_array1852.borrow(), self::ml_array::number_index(ml_reference_position1852)); ml_reference_projection1852 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field4.clone()))) || !(self::test_java_script_semantic_same_value(({
                                                        let source_ml_source2_array_read_value31010 = shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31011 = ((match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::ArrayCreation(_, _, _, source_ml_source2_data_property_value31009) => {
                                                                    Some((source_ml_source2_data_property_value31009,))
                                                                }
                                                                _ => {
                                                                    None::<(f64,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0;
                                                            { let ml_reference_array1853 = source_ml_source2_array_read_value31010.clone(); let ml_reference_position1853 = source_ml_source2_array_read_index31011; let ml_reference_projection1853 = self::ml_array::optional_at(&*ml_reference_array1853.borrow(), self::ml_array::number_index(ml_reference_position1853)); ml_reference_projection1853 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field5.clone()))) || !(self::test_java_script_semantic_same_value(match program.clone() {
                                                        self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source2_data_property_value31013, _, _, _) => {
                                                            let source_ml_source2_data_property_value31013 = *source_ml_source2_data_property_value31013;
                                                            source_ml_source2_data_property_value31013.clone()
                                                        }
                                                    }, shared_constructor_field6.clone()))) || (({ let ml_reference_array1854 = shared_constructor_field9.clone(); let ml_reference_projection1854 = ml_reference_array1854.borrow().len() as f64; ml_reference_projection1854 }) != ({ let ml_reference_array1855 = shared_constructor_field3.clone(); let ml_reference_projection1855 = ml_reference_array1855.borrow().len() as f64; ml_reference_projection1855 }))) || { let ml_values1856 = { let ml_reference_value1857 = shared_constructor_field9.borrow().clone(); ml_reference_value1857 }; let ml_callback1856 = { let state = state.clone(); std::rc::Rc::new(move |value: self::JavaScriptSemanticValue, index: f64| -> bool { !(self::test_java_script_semantic_same_value(value.clone(), ({
                                                        let source_ml_source2_array_read_value31021 = match state.clone() {
                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, source_ml_source2_data_property_value31020, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source2_data_property_value31020.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_array_read_index31022 = index;
                                                            { let ml_reference_array1858 = source_ml_source2_array_read_value31021.clone(); let ml_reference_position1858 = source_ml_source2_array_read_index31022; let ml_reference_projection1858 = self::ml_array::optional_at(&*ml_reference_array1858.borrow(), self::ml_array::number_index(ml_reference_position1858)); ml_reference_projection1858 }
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) }) }; ml_values1856.iter().cloned().enumerate().any(|(ml_index1856, ml_value1856)| ml_callback1856(ml_value1856, (ml_index1856 as f64))) }) {
                                                        invalid_instruction.clone()
                                                    } else {
                                                        {
                                                            let invalid = self::read_java_script_array_creation_state_error(heap.clone(), state.clone());
                                                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                invalid.clone()
                                                            } else {
                                                                {
                                                                    let source_ml_source2_s31059 = if (!(match (((pending_construction.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(match (((pending_fallback.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationFallback>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                        {
                                                                            let request = self::step_java_script_array_creation(heap.clone(), state.clone());
                                                                            if ((match request.clone() {
                                                                                self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(_, _, _, _, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            } || !(match (((error.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || (!(match (((pending_construction.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(match (((pending_fallback.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationFallback>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101]))
                                                                            } else {
                                                                                {
                                                                                    let construction = ((pending_construction.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone();
                                                                                    if !(match (construction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                        {
                                                                                            let directory = match program.clone() {
                                                                                                self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value31065, _) => {
                                                                                                    ((source_ml_source2_data_property_value31065.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                                                }
                                                                                            };
                                                                                            if match (directory.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                                            } else {
                                                                                                {
                                                                                                    let source_ml_source1_shadow_invalid_1 = self::read_java_script_construction_state_error(program.clone(), heap.clone(), directory.clone().expect("a proven present value"), construction.clone().expect("a proven present value"));
                                                                                                    if (source_ml_source1_shadow_invalid_1 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Return(source_ml_source1_shadow_invalid_1.clone())
                                                                                                    } else {
                                                                                                        if (((!(self::test_java_script_semantic_same_value(match construction.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value31066, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                let source_ml_source2_data_property_value31066 = *source_ml_source2_data_property_value31066;
                                                                                                                source_ml_source2_data_property_value31066.clone()
                                                                                                            }
                                                                                                        }, (((match request.clone() {
                                                                                                            self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(_, _, source_ml_source2_data_property_value31067, _, _) => {
                                                                                                                let source_ml_source2_data_property_value31067 = *source_ml_source2_data_property_value31067;
                                                                                                                Some((source_ml_source2_data_property_value31067.clone(),))
                                                                                                            }
                                                                                                            _ => {
                                                                                                                None::<(self::JavaScriptSemanticValue,)>
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value")).0).clone())) || !(self::test_java_script_semantic_same_value(match construction.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptConstructionState::ConstructionState(_, source_ml_source2_data_property_value31068, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                let source_ml_source2_data_property_value31068 = *source_ml_source2_data_property_value31068;
                                                                                                                source_ml_source2_data_property_value31068.clone()
                                                                                                            }
                                                                                                        }, (((match request.clone() {
                                                                                                            self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(_, _, source_ml_source2_data_property_value31069, _, _) => {
                                                                                                                let source_ml_source2_data_property_value31069 = *source_ml_source2_data_property_value31069;
                                                                                                                Some((source_ml_source2_data_property_value31069.clone(),))
                                                                                                            }
                                                                                                            _ => {
                                                                                                                None::<(self::JavaScriptSemanticValue,)>
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value")).0).clone()))) || (({ let ml_reference_array1859 = match construction.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptConstructionState::ConstructionState(_, _, source_ml_source2_data_property_value31072, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value31072.clone()
                                                                                                            }
                                                                                                        }; let ml_reference_projection1859 = ml_reference_array1859.borrow().len() as f64; ml_reference_projection1859 }) != ({ let ml_reference_array1860 = (((match request.clone() {
                                                                                                            self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(_, _, _, source_ml_source2_data_property_value31073, _) => {
                                                                                                                Some((source_ml_source2_data_property_value31073.clone(),))
                                                                                                            }
                                                                                                            _ => {
                                                                                                                None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,)>
                                                                                                            }
                                                                                                        }).clone().expect("a proven present value")).0).clone(); let ml_reference_projection1860 = ml_reference_array1860.borrow().len() as f64; ml_reference_projection1860 }))) || { let ml_values1861 = { let ml_reference_value1862 = (match construction.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptConstructionState::ConstructionState(_, _, source_ml_source2_data_property_value31074, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                source_ml_source2_data_property_value31074.clone()
                                                                                                            }
                                                                                                        }).borrow().clone(); ml_reference_value1862 }; let ml_callback1861 = { let request = request.clone(); std::rc::Rc::new(move |value: self::JavaScriptSemanticValue, index: f64| -> bool { !(self::test_java_script_semantic_same_value(value.clone(), ({
                                                                                                            let source_ml_source2_array_read_value31076 = (((match request.clone() {
                                                                                                                self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(_, _, _, source_ml_source2_data_property_value31075, _) => {
                                                                                                                    Some((source_ml_source2_data_property_value31075.clone(),))
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,)>
                                                                                                                }
                                                                                                            }).clone().expect("a proven present value")).0).clone();
                                                                                                            {
                                                                                                                let source_ml_source2_array_read_index31077 = index;
                                                                                                                { let ml_reference_array1863 = source_ml_source2_array_read_value31076.clone(); let ml_reference_position1863 = source_ml_source2_array_read_index31077; let ml_reference_projection1863 = self::ml_array::optional_at(&*ml_reference_array1863.borrow(), self::ml_array::number_index(ml_reference_position1863)); ml_reference_projection1863 }
                                                                                                            }
                                                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) }) }; ml_values1861.iter().cloned().enumerate().any(|(ml_index1861, ml_value1861)| ml_callback1861(ml_value1861, (ml_index1861 as f64))) }) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                        } else {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Next
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        {
                                                                                            let fallback = ((pending_fallback.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationFallback>,))).0).clone();
                                                                                            if ((((match (fallback.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match fallback.clone().expect("a proven present value") {
                                                                                                self::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(_, _) => {
                                                                                                    false
                                                                                                }
                                                                                            }) || ((match fallback.clone().expect("a proven present value") {
                                                                                                self::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(_, source_ml_source2_data_property_value31083) => {
                                                                                                    source_ml_source2_data_property_value31083
                                                                                                }
                                                                                            }) != (match (((match request.clone() {
                                                                                                self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(_, _, _, _, source_ml_source2_data_property_value31084) => {
                                                                                                    let source_ml_source2_data_property_value31084 = *source_ml_source2_data_property_value31084;
                                                                                                    Some((source_ml_source2_data_property_value31084.clone(),))
                                                                                                }
                                                                                                _ => {
                                                                                                    None::<(self::JavaScriptArrayCreationFallback,)>
                                                                                                }
                                                                                            }).clone().expect("a proven present value")).0).clone() {
                                                                                                self::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(_, source_ml_source2_data_property_value31085) => {
                                                                                                    source_ml_source2_data_property_value31085
                                                                                                }
                                                                                            }))) || !(self::test_java_script_semantic_same_value(match fallback.clone().expect("a proven present value") {
                                                                                                self::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(source_ml_source2_data_property_value31086, _) => {
                                                                                                    let source_ml_source2_data_property_value31086 = *source_ml_source2_data_property_value31086;
                                                                                                    source_ml_source2_data_property_value31086.clone()
                                                                                                }
                                                                                            }, match (((match request.clone() {
                                                                                                self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(_, _, _, _, source_ml_source2_data_property_value31087) => {
                                                                                                    let source_ml_source2_data_property_value31087 = *source_ml_source2_data_property_value31087;
                                                                                                    Some((source_ml_source2_data_property_value31087.clone(),))
                                                                                                }
                                                                                                _ => {
                                                                                                    None::<(self::JavaScriptArrayCreationFallback,)>
                                                                                                }
                                                                                            }).clone().expect("a proven present value")).0).clone() {
                                                                                                self::JavaScriptArrayCreationFallback::ArrayCreationArrayFallback(source_ml_source2_data_property_value31088, _) => {
                                                                                                    let source_ml_source2_data_property_value31088 = *source_ml_source2_data_property_value31088;
                                                                                                    source_ml_source2_data_property_value31088.clone()
                                                                                                }
                                                                                            }))) || suspended) {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 102, 97, 108, 108, 98, 97, 99, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101, 32, 108, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                                                                            } else {
                                                                                                {
                                                                                                    let directory_2 = match program.clone() {
                                                                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value31089, _) => {
                                                                                                            ((source_ml_source2_data_property_value31089.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                                                        }
                                                                                                    };
                                                                                                    if match (directory_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 102, 97, 108, 108, 98, 97, 99, 107, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                                                                    } else {
                                                                                                        {
                                                                                                            let classified = self::classify_java_script_array_species_constructor(heap.clone(), directory_2.clone().expect("a proven present value"), (((match request.clone() {
                                                                                                                self::JavaScriptArrayCreationTransition::ArrayCreationConstruct(_, _, source_ml_source2_data_property_value31090, _, _) => {
                                                                                                                    let source_ml_source2_data_property_value31090 = *source_ml_source2_data_property_value31090;
                                                                                                                    Some((source_ml_source2_data_property_value31090.clone(),))
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    None::<(self::JavaScriptSemanticValue,)>
                                                                                                                }
                                                                                                            }).clone().expect("a proven present value")).0).clone());
                                                                                                            if (match classified.clone() {
                                                                                                                self::JavaScriptArrayConstructorClassification::ArrayConstructorUnknown(_) => {
                                                                                                                    true
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    false
                                                                                                                }
                                                                                                            } || {
                                                                                                                let source_ml_source2_comparison_operand31094 = {
                                                                                                                    let source_ml_source2_optional_chain_receiver31092 = (self::read_java_script_known_array_constructor_fact(classified.clone()),);
                                                                                                                    if match (source_ml_source2_optional_chain_receiver31092.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                        Some({
                                                                                                                            let source_ml_source1_temporary_optional_chain1324 = ((source_ml_source2_optional_chain_receiver31092.clone()).0).clone().expect("a proven present value");
                                                                                                                            match source_ml_source1_temporary_optional_chain1324.clone() {
                                                                                                                                self::JavaScriptArrayConstructorFact::ArrayConstructorFact(_, source_ml_source2_data_property_value31093, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value31093
                                                                                                                                }
                                                                                                                            }
                                                                                                                        })
                                                                                                                    } else {
                                                                                                                        None::<bool>
                                                                                                                    }
                                                                                                                };
                                                                                                                match (source_ml_source2_comparison_operand31094.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                                                            }) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 102, 97, 108, 108, 98, 97, 99, 107, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                                                                            } else {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Next
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
                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Next
                                                                    };
                                                                    match source_ml_source2_s31059.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Next => {
                                                                            if (suspended && ((match ({
                                                                                let source_ml_source2_optional_chain_receiver31096 = (((pending_construction.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone(),);
                                                                                match if match (source_ml_source2_optional_chain_receiver31096.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    Some({
                                                                                        let source_ml_source1_temporary_optional_chain1325 = ((source_ml_source2_optional_chain_receiver31096.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source1_temporary_optional_chain1325.clone() {
                                                                                            self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31097, _) => {
                                                                                                ((source_ml_source2_data_property_value31097.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                                                            }
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    None::<self::JavaScriptConstructionCall>
                                                                                } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } && match (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31098, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value31098.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(match (((error.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                                                                                self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108])
                                                                            } else {
                                                                                {
                                                                                    let setup = ((error.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone();
                                                                                    if (!(match (setup.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((!(match (((pending_construction.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(match (((pending_fallback.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationFallback>,))).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || !(self::test_java_script_pending_call_constructor_failure(program.clone(), heap.clone(), ({
                                                                                        let source_ml_source2_optional_chain_receiver31100 = (match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31099, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value31099.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,))).0).clone()
                                                                                            }
                                                                                        },);
                                                                                        if match (source_ml_source2_optional_chain_receiver31100.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                            Some({
                                                                                                let source_ml_source1_temporary_optional_chain1326 = ((source_ml_source2_optional_chain_receiver31100.clone()).0).clone().expect("a proven present value");
                                                                                                match source_ml_source1_temporary_optional_chain1326.clone() {
                                                                                                    self::JavaScriptArrayCreationCall::ArrayCreationCallRecord(source_ml_source2_data_property_value31101, _, _) => {
                                                                                                        let source_ml_source2_data_property_value31101 = *source_ml_source2_data_property_value31101;
                                                                                                        source_ml_source2_data_property_value31101.clone()
                                                                                                    }
                                                                                                }
                                                                                            })
                                                                                        } else {
                                                                                            None::<self::JavaScriptSemanticValue>
                                                                                        }
                                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), setup.clone().expect("a proven present value"))))) {
                                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 115, 101, 116, 117, 112, 32, 102, 97, 105, 108, 117, 114, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 101, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 114, 101, 97, 115, 111, 110])
                                                                                    } else {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327::MlSource1TemporaryReadJavaScriptCallPendingConstructionErrorJoin1327Return(source_ml_source1_temporary_result1327) => {
                                                                            source_ml_source1_temporary_result1327.clone()
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
                self::JavaScriptCallPending::WaitConstructorCheck(
                    destination_2,
                    input,
                    required,
                ) => {
                    let input = *input;
                    if !(shared_constructor_field7 > (0f64)) {
                        self::ml_string::Text::from_units(vec![
                            99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 104, 101,
                            99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117,
                            97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 112,
                            117, 116, 32, 112, 114, 101, 100, 105, 99, 97, 116, 101, 32, 109, 111,
                            100, 101, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116,
                            105, 111, 110,
                        ])
                    } else {
                        {
                            let instruction_2 = {
                                let source_ml_source2_array_read_value31106 =
                                    shared_constructor_field11.clone();
                                {
                                    let source_ml_source2_array_read_index31107 =
                                        (shared_constructor_field7 - 1f64);
                                    {
                                        let ml_reference_array1864 =
                                            source_ml_source2_array_read_value31106.clone();
                                        let ml_reference_position1864 =
                                            source_ml_source2_array_read_index31107;
                                        let ml_reference_projection1864 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array1864.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position1864,
                                                ),
                                            );
                                        ml_reference_projection1864
                                    }
                                }
                            };
                            if (match (instruction_2.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } || match instruction_2.clone().expect("a proven present value") {
                                self::JavaScriptCallInstruction::ConstructorCheck(_, _, _) => false,
                                _ => true,
                            }) {
                                self::ml_string::Text::from_units(vec![
                                    99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99,
                                    104, 101, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32,
                                    97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116,
                                    101, 100, 32, 105, 110, 112, 117, 116, 32, 112, 114, 101, 100,
                                    105, 99, 97, 116, 101, 32, 109, 111, 100, 101, 32, 97, 110,
                                    100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
                                ])
                            } else {
                                if (((((((match instruction_2.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::ConstructorCheck(source_ml_source2_data_property_value31112, _, _) => {
                                                Some((source_ml_source2_data_property_value31112,))
                                            }
                                            _ => {
                                                None::<(f64,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != destination_2) || ((((match instruction_2.clone().expect("a proven present value") {
                                            self::JavaScriptCallInstruction::ConstructorCheck(_, _, source_ml_source2_data_property_value31114) => {
                                                Some((source_ml_source2_data_property_value31114,))
                                            }
                                            _ => {
                                                None::<(bool,)>
                                            }
                                        }).clone().expect("a proven present value")).0) != required)) || !(self::test_java_script_semantic_same_value(({
                                            let source_ml_source2_array_read_value31117 = shared_constructor_field8.clone();
                                            {
                                                let source_ml_source2_array_read_index31118 = ((match instruction_2.clone().expect("a proven present value") {
                                                    self::JavaScriptCallInstruction::ConstructorCheck(_, source_ml_source2_data_property_value31116, _) => {
                                                        Some((source_ml_source2_data_property_value31116,))
                                                    }
                                                    _ => {
                                                        None::<(f64,)>
                                                    }
                                                }).clone().expect("a proven present value")).0;
                                                { let ml_reference_array1865 = source_ml_source2_array_read_value31117.clone(); let ml_reference_position1865 = source_ml_source2_array_read_index31118; let ml_reference_projection1865 = self::ml_array::optional_at(&*ml_reference_array1865.borrow(), self::ml_array::number_index(ml_reference_position1865)); ml_reference_projection1865 }
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), input.clone()))) || suspended) {
                                            self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 104, 101, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 112, 117, 116, 32, 112, 114, 101, 100, 105, 99, 97, 116, 101, 32, 109, 111, 100, 101, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110])
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                            }
                        }
                    }
                }
                _ => self::read_java_script_call_pending_callable_construction_error(
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
