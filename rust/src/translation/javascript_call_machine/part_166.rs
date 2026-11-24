// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConcatenationCall;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySliceCall;
use super::JavaScriptArraySliceState;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArrayTransformCall;
use super::JavaScriptArrayTransformState;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSpeciesConstructionCall;
use super::JavaScriptSpeciesConstructionState;
use super::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329;
use super::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330;
use super::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_concatenation_state_error;
use super::read_java_script_array_slice_state_error;
use super::read_java_script_array_transform_state_error;
use super::read_java_script_species_construction_state_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_pending_array_species_error(
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
            shared_constructor_field12,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field13,
            _,
            _,
        ) => {
            match frame.clone() {
                self::JavaScriptCallFrame::CallFrame(
                    _,
                    shared_constructor_field9,
                    shared_constructor_field10,
                    shared_constructor_field11,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    match program.clone() {
                        self::JavaScriptCallProgram::CallProgram(
                            _,
                            _,
                            shared_constructor_field8,
                            _,
                            _,
                            _,
                        ) => {
                            let shared_constructor_field8 = *shared_constructor_field8;
                            match pending.clone() {
                                self::JavaScriptCallPending::WaitArrayConcatenation(
                                    destination,
                                    realm,
                                    state,
                                    pending_construction,
                                ) => {
                                    let state = *state;
                                    match state.clone() {
                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, _, _, _, _, _, _, _, _, _, _, shared_constructor_field5, _, _, _, _, _, _, _) => {
                                            let shared_constructor_field1 = *shared_constructor_field1;
                                            let shared_constructor_field3 = *shared_constructor_field3;
                                            let shared_constructor_field4 = *shared_constructor_field4;
                                            {
                                                let invalid_instruction = self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109]);
                                                if !((shared_constructor_field9 > (0f64))) {
                                                    invalid_instruction.clone()
                                                } else {
                                                    {
                                                        let instruction = {
                                                            let source_ml_source2_array_read_value31425 = shared_constructor_field13.clone();
                                                            {
                                                                let source_ml_source2_array_read_index31426 = (shared_constructor_field9 - 1f64);
                                                                { let ml_reference_array1921 = source_ml_source2_array_read_value31425.clone(); let ml_reference_position1921 = source_ml_source2_array_read_index31426; let ml_reference_projection1921 = self::ml_array::optional_at(&*ml_reference_array1921.borrow(), self::ml_array::number_index(ml_reference_position1921)); ml_reference_projection1921 }
                                                            }
                                                        };
                                                        if (match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) {
                                                            invalid_instruction.clone()
                                                        } else {
                                                            if (((((match instruction.clone().expect("a proven present value") {
                                                                self::JavaScriptCallInstruction::ArrayConcatenation(source_ml_source2_data_property_value31432, _, _, _) => {
                                                                    Some((source_ml_source2_data_property_value31432,))
                                                                }
                                                                _ => {
                                                                    None::<(f64,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0) != destination) || (realm != shared_constructor_field12)) {
                                                                invalid_instruction.clone()
                                                            } else {
                                                                {
                                                                    let species = (((match instruction.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, source_ml_source2_data_property_value31435) => {
                                                                            let source_ml_source2_data_property_value31435 = *source_ml_source2_data_property_value31435;
                                                                            Some((source_ml_source2_data_property_value31435.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::JavaScriptArraySpeciesDirectory,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone();
                                                                    if match (species.clone(), None::<()>) { (_ml_left, Some(_ml_right)) => false, _ => false } {
                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121])
                                                                    } else {
                                                                        {
                                                                            let invalid = self::read_java_script_array_concatenation_state_error(heap.clone(), species.clone(), state.clone());
                                                                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                                invalid.clone()
                                                                            } else {
                                                                                if ((((!(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), ({
                                                                                    let source_ml_source2_array_read_value31439 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_array_read_index31440 = ((((match instruction.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallInstruction::ArrayConcatenation(_, source_ml_source2_data_property_value31438, _, _) => {
                                                                                                Some((((source_ml_source2_data_property_value31438.clone().unwrap_or_else(|| (None::<f64>,))).0).clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(Option<f64>,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone()).clone().unwrap_or_else(|| 0f64);
                                                                                        { let ml_reference_array1922 = source_ml_source2_array_read_value31439.clone(); let ml_reference_position1922 = source_ml_source2_array_read_index31440; let ml_reference_projection1922 = self::ml_array::optional_at(&*ml_reference_array1922.borrow(), self::ml_array::number_index(ml_reference_position1922)); ml_reference_projection1922 }
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field3.clone(), ({
                                                                                    let source_ml_source2_array_read_value31444 = shared_constructor_field10.clone();
                                                                                    {
                                                                                        let source_ml_source2_array_read_index31445 = ((match instruction.clone().expect("a proven present value") {
                                                                                            self::JavaScriptCallInstruction::ArrayConcatenation(_, _, source_ml_source2_data_property_value31443, _) => {
                                                                                                Some((source_ml_source2_data_property_value31443,))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(f64,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0;
                                                                                        { let ml_reference_array1923 = source_ml_source2_array_read_value31444.clone(); let ml_reference_position1923 = source_ml_source2_array_read_index31445; let ml_reference_projection1923 = self::ml_array::optional_at(&*ml_reference_array1923.borrow(), self::ml_array::number_index(ml_reference_position1923)); ml_reference_projection1923 }
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(shared_constructor_field4.clone(), shared_constructor_field8.clone()))) || (({ let ml_reference_array1924 = shared_constructor_field2.clone(); let ml_reference_projection1924 = ml_reference_array1924.borrow().len() as f64; ml_reference_projection1924 }) != ({ let ml_reference_array1925 = shared_constructor_field11.clone(); let ml_reference_projection1925 = ml_reference_array1925.borrow().len() as f64; ml_reference_projection1925 }))) || { let ml_values1926 = { let ml_reference_value1927 = shared_constructor_field2.borrow().clone(); ml_reference_value1927 }; let ml_callback1926 = { let frame = frame.clone(); std::rc::Rc::new(move |value: self::JavaScriptSemanticValue, index: f64| -> bool { !(self::test_java_script_semantic_same_value(value.clone(), ({
                                                                                    let source_ml_source2_array_read_value31454 = match frame.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value31453, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value31453.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_array_read_index31455 = index;
                                                                                        { let ml_reference_array1928 = source_ml_source2_array_read_value31454.clone(); let ml_reference_position1928 = source_ml_source2_array_read_index31455; let ml_reference_projection1928 = self::ml_array::optional_at(&*ml_reference_array1928.borrow(), self::ml_array::number_index(ml_reference_position1928)); ml_reference_projection1928 }
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) }) }; ml_values1926.iter().cloned().enumerate().any(|(ml_index1926, ml_value1926)| ml_callback1926(ml_value1926, (ml_index1926 as f64))) }) {
                                                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 97, 110, 100, 32, 83, 121, 109, 98, 111, 108, 115])
                                                                                } else {
                                                                                    {
                                                                                        let construction = ((pending_construction.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                                                                        {
                                                                                            let source_ml_source2_s31471 = if !(match (construction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                {
                                                                                                    let selected = if {
                                                                                                        let source_ml_source2_own_property_operands31472 = (state.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]));
                                                                                                        {
                                                                                                            let source_ml_source2_own_property_key31473 = ((source_ml_source2_own_property_operands31472.clone()).1,);
                                                                                                            match (source_ml_source2_own_property_operands31472.clone()).0 {
                                                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_owned_constructor_field31474, source_ml_source2_owned_constructor_field31475, source_ml_source2_owned_constructor_field31476, source_ml_source2_owned_constructor_field31477, source_ml_source2_owned_constructor_field31478, source_ml_source2_owned_constructor_field31479, source_ml_source2_owned_constructor_field31480) => {
                                                                                                                    (((((((((((((((((((((((((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![36]))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 115])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100, 97, 98, 108, 101, 75, 101, 121])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 75, 101, 121])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![105, 116, 101, 109, 80, 111, 115, 105, 116, 105, 111, 110])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![105, 116, 101, 109])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![105, 116, 101, 109, 83, 116, 97, 114, 116])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116])))) || (((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![112, 104, 97, 115, 101])))) || ((((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && match source_ml_source2_owned_constructor_field31474.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && match source_ml_source2_owned_constructor_field31475.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101]))) && match source_ml_source2_owned_constructor_field31476.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && match source_ml_source2_owned_constructor_field31477.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 70, 97, 99, 116]))) && match source_ml_source2_owned_constructor_field31478.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116]))) && match source_ml_source2_owned_constructor_field31479.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31473.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) && match source_ml_source2_owned_constructor_field31480.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    } {
                                                                                                        match state.clone() {
                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31481, _, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value31481.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                            }
                                                                                                        }
                                                                                                    } else {
                                                                                                        None::<self::JavaScriptSemanticValue>
                                                                                                    };
                                                                                                    if ((((shared_constructor_field5 != (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) || match (selected.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(match construction.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(source_ml_source2_data_property_value31484, _, _, _, _, _, _, _, _, _) => {
                                                                                                            let source_ml_source2_data_property_value31484 = *source_ml_source2_data_property_value31484;
                                                                                                            source_ml_source2_data_property_value31484.clone()
                                                                                                        }
                                                                                                    }, selected.clone().expect("a proven present value")))) || ((match construction.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, source_ml_source2_data_property_value31486, _, _, _, _, _, _, _) => {
                                                                                                            source_ml_source2_data_property_value31486
                                                                                                        }
                                                                                                    }) != (0f64))) {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 122, 101, 114, 111, 45, 108, 101, 110, 103, 116, 104, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                                                                                    } else {
                                                                                                        {
                                                                                                            let source_ml_source1_shadow_invalid_1 = self::read_java_script_species_construction_state_error(program.clone(), heap.clone(), species.clone(), construction.clone().expect("a proven present value"));
                                                                                                            if (source_ml_source1_shadow_invalid_1 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329Return(source_ml_source1_shadow_invalid_1.clone())
                                                                                                            } else {
                                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329Next
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329Next
                                                                                            };
                                                                                            match source_ml_source2_s31471.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329Next => {
                                                                                                    if (suspended && if !(match (construction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                        match (match construction.clone().expect("a proven present value") {
                                                                                                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value31487, _, _) => {
                                                                                                                ((source_ml_source2_data_property_value31487.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                            }
                                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                                    } else {
                                                                                                        match (match state.clone() {
                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31488) => {
                                                                                                                ((source_ml_source2_data_property_value31488.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                                                                            }
                                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                                    }) {
                                                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 98, 97, 99, 107])
                                                                                                    } else {
                                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                                    }
                                                                                                }
                                                                                                self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1329Return(source_ml_source1_temporary_result1329) => {
                                                                                                    source_ml_source1_temporary_result1329.clone()
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
                                            }
                                        }
                                    }
                                }
                                self::JavaScriptCallPending::WaitArraySlice(
                                    destination_2,
                                    realm_2,
                                    state_2,
                                    pending_construction_2,
                                ) => {
                                    let state_2 = *state_2;
                                    match state_2.clone() {
                                        self::JavaScriptArraySliceState::ArraySliceState(
                                            shared_constructor_field1_2,
                                            shared_constructor_field2_2,
                                            shared_constructor_field3_2,
                                            shared_constructor_field4_2,
                                            shared_constructor_field5_2,
                                            _,
                                            _,
                                            _,
                                            shared_constructor_field6,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            shared_constructor_field7,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let shared_constructor_field1_2 =
                                                *shared_constructor_field1_2;
                                            let shared_constructor_field2_2 =
                                                *shared_constructor_field2_2;
                                            let shared_constructor_field3_2 =
                                                *shared_constructor_field3_2;
                                            let shared_constructor_field5_2 =
                                                *shared_constructor_field5_2;
                                            if !(shared_constructor_field9 > (0f64)) {
                                                self::ml_string::Text::from_units(vec![
                                                    65, 114, 114, 97, 121, 32, 115, 108, 105, 99,
                                                    101, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                                                    97, 99, 116, 117, 97, 108, 32, 115, 111, 117,
                                                    114, 99, 101, 32, 105, 110, 115, 116, 114, 117,
                                                    99, 116, 105, 111, 110, 32, 100, 101, 115, 116,
                                                    105, 110, 97, 116, 105, 111, 110, 32, 97, 110,
                                                    100, 32, 114, 101, 97, 108, 109,
                                                ])
                                            } else {
                                                {
                                                    let instruction_2 = {
                                                        let source_ml_source2_array_read_value31493 =
                                                            shared_constructor_field13.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31494 =
                                                                (shared_constructor_field9 - 1f64);
                                                            {
                                                                let ml_reference_array1929 = source_ml_source2_array_read_value31493.clone();
                                                                let ml_reference_position1929 = source_ml_source2_array_read_index31494;
                                                                let ml_reference_projection1929 = self::ml_array::optional_at(&*ml_reference_array1929.borrow(), self::ml_array::number_index(ml_reference_position1929));
                                                                ml_reference_projection1929
                                                            }
                                                        }
                                                    };
                                                    if (match (instruction_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction_2.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ArraySlice(_, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                    } else {
                                                        if (((((match instruction_2.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArraySlice(source_ml_source2_data_property_value31499, _, _) => {
                                                                Some((source_ml_source2_data_property_value31499,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != destination_2) || (realm_2 != shared_constructor_field12)) {
                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                        } else {
                                                            {
                                                                let species_2 = (((match instruction_2.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::ArraySlice(_, _, source_ml_source2_data_property_value31502) => {
                                                                        let source_ml_source2_data_property_value31502 = *source_ml_source2_data_property_value31502;
                                                                        Some((source_ml_source2_data_property_value31502.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::JavaScriptArraySpeciesDirectory,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone();
                                                                if match (species_2.clone(), None::<()>) { (_ml_left, Some(_ml_right)) => false, _ => false } {
                                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121])
                                                                } else {
                                                                    {
                                                                        let invalid_2 = self::read_java_script_array_slice_state_error(heap.clone(), species_2.clone(), state_2.clone());
                                                                        if (invalid_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                            invalid_2.clone()
                                                                        } else {
                                                                            if ((((!(self::test_java_script_semantic_same_value(shared_constructor_field1_2.clone(), ({
                                                                                let source_ml_source2_array_read_value31506 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index31507 = ((((match instruction_2.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallInstruction::ArraySlice(_, source_ml_source2_data_property_value31505, _) => {
                                                                                            Some((((source_ml_source2_data_property_value31505.clone().unwrap_or_else(|| (None::<f64>,))).0).clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(Option<f64>,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone()).clone().unwrap_or_else(|| 0f64);
                                                                                    { let ml_reference_array1930 = source_ml_source2_array_read_value31506.clone(); let ml_reference_position1930 = source_ml_source2_array_read_index31507; let ml_reference_projection1930 = self::ml_array::optional_at(&*ml_reference_array1930.borrow(), self::ml_array::number_index(ml_reference_position1930)); ml_reference_projection1930 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field2_2.clone(), ({
                                                                                let source_ml_source2_array_read_value31510 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index31511 = 0f64;
                                                                                    { let ml_reference_array1931 = source_ml_source2_array_read_value31510.clone(); let ml_reference_position1931 = source_ml_source2_array_read_index31511; let ml_reference_projection1931 = self::ml_array::optional_at(&*ml_reference_array1931.borrow(), self::ml_array::number_index(ml_reference_position1931)); ml_reference_projection1931 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(shared_constructor_field3_2.clone(), ({
                                                                                let source_ml_source2_array_read_value31514 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index31515 = 1f64;
                                                                                    { let ml_reference_array1932 = source_ml_source2_array_read_value31514.clone(); let ml_reference_position1932 = source_ml_source2_array_read_index31515; let ml_reference_projection1932 = self::ml_array::optional_at(&*ml_reference_array1932.borrow(), self::ml_array::number_index(ml_reference_position1932)); ml_reference_projection1932 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || (shared_constructor_field4_2 != ((({ let ml_reference_array1933 = shared_constructor_field11.clone(); let ml_reference_projection1933 = ml_reference_array1933.borrow().len() as f64; ml_reference_projection1933 }) > (1f64))))) || !(self::test_java_script_semantic_same_value(shared_constructor_field5_2.clone(), shared_constructor_field8.clone()))) {
                                                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 98, 111, 117, 110, 100, 115, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 112, 114, 101, 115, 101, 110, 99, 101, 32, 97, 110, 100, 32, 83, 121, 109, 98, 111, 108])
                                                                            } else {
                                                                                {
                                                                                    let construction_2 = ((pending_construction_2.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                                                                    {
                                                                                        let source_ml_source2_s31541 = if !(match (construction_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                            {
                                                                                                let selected_2 = if {
                                                                                                    let source_ml_source2_own_property_operands31542 = (state_2.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]));
                                                                                                    {
                                                                                                        let source_ml_source2_own_property_key31543 = ((source_ml_source2_own_property_operands31542.clone()).1,);
                                                                                                        match (source_ml_source2_own_property_operands31542.clone()).0 {
                                                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_owned_constructor_field31544, source_ml_source2_owned_constructor_field31545, source_ml_source2_owned_constructor_field31546, source_ml_source2_owned_constructor_field31547, source_ml_source2_owned_constructor_field31548, source_ml_source2_owned_constructor_field31549, source_ml_source2_owned_constructor_field31550) => {
                                                                                                                ((((((((((((((((((((((((((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![36]))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![101, 110, 100])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![101, 110, 100, 80, 114, 101, 115, 101, 110, 116])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 75, 101, 121])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![101, 110, 100, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![112, 104, 97, 115, 101])))) || (((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 75, 105, 110, 100])))) || ((((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && match source_ml_source2_owned_constructor_field31544.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && match source_ml_source2_owned_constructor_field31545.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101]))) && match source_ml_source2_owned_constructor_field31546.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && match source_ml_source2_owned_constructor_field31547.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 70, 97, 99, 116]))) && match source_ml_source2_owned_constructor_field31548.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116]))) && match source_ml_source2_owned_constructor_field31549.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31543.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) && match source_ml_source2_owned_constructor_field31550.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }))
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } {
                                                                                                    match state_2.clone() {
                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31551, _, _, _) => {
                                                                                                            ((source_ml_source2_data_property_value31551.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    None::<self::JavaScriptSemanticValue>
                                                                                                };
                                                                                                if ((((shared_constructor_field7 != (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) || match (selected_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(match construction_2.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(source_ml_source2_data_property_value31554, _, _, _, _, _, _, _, _, _) => {
                                                                                                        let source_ml_source2_data_property_value31554 = *source_ml_source2_data_property_value31554;
                                                                                                        source_ml_source2_data_property_value31554.clone()
                                                                                                    }
                                                                                                }, selected_2.clone().expect("a proven present value")))) || ((match construction_2.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, source_ml_source2_data_property_value31557, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value31557
                                                                                                    }
                                                                                                }) != shared_constructor_field6)) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                                                                                } else {
                                                                                                    {
                                                                                                        let source_ml_source1_shadow_invalid_2 = self::read_java_script_species_construction_state_error(program.clone(), heap.clone(), species_2.clone(), construction_2.clone().expect("a proven present value"));
                                                                                                        if (source_ml_source1_shadow_invalid_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330Return(source_ml_source1_shadow_invalid_2.clone())
                                                                                                        } else {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330Next
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330Next
                                                                                        };
                                                                                        match source_ml_source2_s31541.clone() {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330Next => {
                                                                                                if (suspended && if !(match (construction_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                    match (match construction_2.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value31559, _, _) => {
                                                                                                            ((source_ml_source2_data_property_value31559.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                        }
                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                                } else {
                                                                                                    match (match state_2.clone() {
                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31560) => {
                                                                                                            ((source_ml_source2_data_property_value31560.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                                                                        }
                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                                }) {
                                                                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 108, 105, 99, 101, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 98, 97, 99, 107])
                                                                                                } else {
                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1330Return(source_ml_source1_temporary_result1330) => {
                                                                                                source_ml_source1_temporary_result1330.clone()
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
                                        }
                                    }
                                }
                                self::JavaScriptCallPending::WaitArrayTransform(
                                    destination_3,
                                    realm_3,
                                    state_3,
                                    pending_construction_3,
                                ) => {
                                    let state_3 = *state_3;
                                    match state_3.clone() {
                                        self::JavaScriptArrayTransformState::ArrayTransformState(shared_constructor_field1_3, shared_constructor_field2_3, shared_constructor_field3_3, shared_constructor_field4_3, shared_constructor_field5_3, shared_constructor_field6_2, _, _, _, _, _, _, shared_constructor_field7_2, _, _, _, _, _, _) => {
                                            let shared_constructor_field2_3 = *shared_constructor_field2_3;
                                            let shared_constructor_field3_3 = *shared_constructor_field3_3;
                                            let shared_constructor_field4_3 = *shared_constructor_field4_3;
                                            let shared_constructor_field5_3 = *shared_constructor_field5_3;
                                            if !((shared_constructor_field9 > (0f64))) {
                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                            } else {
                                                {
                                                    let instruction_3 = {
                                                        let source_ml_source2_array_read_value31565 = shared_constructor_field13.clone();
                                                        {
                                                            let source_ml_source2_array_read_index31566 = (shared_constructor_field9 - 1f64);
                                                            { let ml_reference_array1934 = source_ml_source2_array_read_value31565.clone(); let ml_reference_position1934 = source_ml_source2_array_read_index31566; let ml_reference_projection1934 = self::ml_array::optional_at(&*ml_reference_array1934.borrow(), self::ml_array::number_index(ml_reference_position1934)); ml_reference_projection1934 }
                                                        }
                                                    };
                                                    if (match (instruction_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match instruction_3.clone().expect("a proven present value") {
                                                        self::JavaScriptCallInstruction::ArrayTransform(_, _, _, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                    } else {
                                                        if ((((((match instruction_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArrayTransform(source_ml_source2_data_property_value31572, _, _, _) => {
                                                                Some((source_ml_source2_data_property_value31572,))
                                                            }
                                                            _ => {
                                                                None::<(f64,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0) != destination_3) || (realm_3 != shared_constructor_field12)) || (shared_constructor_field1_3 != ((((match instruction_3.clone().expect("a proven present value") {
                                                            self::JavaScriptCallInstruction::ArrayTransform(_, source_ml_source2_data_property_value31578, _, _) => {
                                                                Some((source_ml_source2_data_property_value31578.clone(),))
                                                            }
                                                            _ => {
                                                                None::<(self::ml_string::Text,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0).clone()))) {
                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 44, 32, 100, 101, 115, 116, 105, 110, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])
                                                        } else {
                                                            {
                                                                let species_3 = (((match instruction_3.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallInstruction::ArrayTransform(_, _, source_ml_source2_data_property_value31579, _) => {
                                                                        let source_ml_source2_data_property_value31579 = *source_ml_source2_data_property_value31579;
                                                                        Some((source_ml_source2_data_property_value31579.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::JavaScriptArraySpeciesDirectory,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone();
                                                                if match (species_3.clone(), None::<()>) { (_ml_left, Some(_ml_right)) => false, _ => false } {
                                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 112, 101, 99, 105, 101, 115, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121])
                                                                } else {
                                                                    {
                                                                        let error = self::read_java_script_array_transform_state_error(heap.clone(), species_3.clone(), state_3.clone());
                                                                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                            error.clone()
                                                                        } else {
                                                                            if (((!(self::test_java_script_semantic_same_value(shared_constructor_field2_3.clone(), ({
                                                                                let source_ml_source2_array_read_value31583 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index31584 = ((((match instruction_3.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallInstruction::ArrayTransform(_, _, _, source_ml_source2_data_property_value31582) => {
                                                                                            Some((((source_ml_source2_data_property_value31582.clone().unwrap_or_else(|| (None::<f64>,))).0).clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(Option<f64>,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone()).clone().unwrap_or_else(|| 0f64);
                                                                                    { let ml_reference_array1935 = source_ml_source2_array_read_value31583.clone(); let ml_reference_position1935 = source_ml_source2_array_read_index31584; let ml_reference_projection1935 = self::ml_array::optional_at(&*ml_reference_array1935.borrow(), self::ml_array::number_index(ml_reference_position1935)); ml_reference_projection1935 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field3_3.clone(), ({
                                                                                let source_ml_source2_array_read_value31587 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index31588 = 0f64;
                                                                                    { let ml_reference_array1936 = source_ml_source2_array_read_value31587.clone(); let ml_reference_position1936 = source_ml_source2_array_read_index31588; let ml_reference_projection1936 = self::ml_array::optional_at(&*ml_reference_array1936.borrow(), self::ml_array::number_index(ml_reference_position1936)); ml_reference_projection1936 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(shared_constructor_field4_3.clone(), ({
                                                                                let source_ml_source2_array_read_value31591 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index31592 = 1f64;
                                                                                    { let ml_reference_array1937 = source_ml_source2_array_read_value31591.clone(); let ml_reference_position1937 = source_ml_source2_array_read_index31592; let ml_reference_projection1937 = self::ml_array::optional_at(&*ml_reference_array1937.borrow(), self::ml_array::number_index(ml_reference_position1937)); ml_reference_projection1937 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(shared_constructor_field5_3.clone(), shared_constructor_field8.clone()))) {
                                                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 115, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 44, 32, 99, 97, 108, 108, 98, 97, 99, 107, 44, 32, 116, 104, 105, 115, 65, 114, 103, 32, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108])
                                                                            } else {
                                                                                {
                                                                                    let construction_3 = ((pending_construction_3.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone();
                                                                                    {
                                                                                        let source_ml_source2_s31616 = if !(match (construction_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                            {
                                                                                                let selected_3 = if {
                                                                                                    let source_ml_source2_own_property_operands31617 = (state_3.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]));
                                                                                                    {
                                                                                                        let source_ml_source2_own_property_key31618 = ((source_ml_source2_own_property_operands31617.clone()).1,);
                                                                                                        match (source_ml_source2_own_property_operands31617.clone()).0 {
                                                                                                            self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_owned_constructor_field31619, source_ml_source2_owned_constructor_field31620, source_ml_source2_owned_constructor_field31621, source_ml_source2_owned_constructor_field31622, source_ml_source2_owned_constructor_field31623, source_ml_source2_owned_constructor_field31624) => {
                                                                                                                ((((((((((((((((((((((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![36]))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![111, 112, 101, 114, 97, 116, 105, 111, 110])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![114, 101, 99, 101, 105, 118, 101, 114])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![116, 104, 105, 115, 65, 114, 103, 117, 109, 101, 110, 116])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 75, 101, 121])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116, 73, 110, 100, 101, 120])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![105, 110, 112, 117, 116])))) || (((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![112, 104, 97, 115, 101])))) || ((((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && match source_ml_source2_owned_constructor_field31619.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && match source_ml_source2_owned_constructor_field31620.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101]))) && match source_ml_source2_owned_constructor_field31621.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))) && match source_ml_source2_owned_constructor_field31622.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 70, 97, 99, 116]))) && match source_ml_source2_owned_constructor_field31623.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false })) || ((((source_ml_source2_own_property_key31618.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) && match source_ml_source2_owned_constructor_field31624.clone() { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false }))
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                } {
                                                                                                    match state_3.clone() {
                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31625, _, _) => {
                                                                                                            ((source_ml_source2_data_property_value31625.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    None::<self::JavaScriptSemanticValue>
                                                                                                };
                                                                                                if (((((shared_constructor_field7_2 != (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]))) || match (selected_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match (match state_3.clone() {
                                                                                                    self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31628, _) => {
                                                                                                        ((source_ml_source2_data_property_value31628.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConstructorFact>,))).0).clone()
                                                                                                    }
                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(match construction_3.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(source_ml_source2_data_property_value31629, _, _, _, _, _, _, _, _, _) => {
                                                                                                        let source_ml_source2_data_property_value31629 = *source_ml_source2_data_property_value31629;
                                                                                                        source_ml_source2_data_property_value31629.clone()
                                                                                                    }
                                                                                                }, selected_3.clone().expect("a proven present value")))) || ((match construction_3.clone().expect("a proven present value") {
                                                                                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, source_ml_source2_data_property_value31634, _, _, _, _, _, _, _) => {
                                                                                                        source_ml_source2_data_property_value31634
                                                                                                    }
                                                                                                }) != (if (shared_constructor_field1_3 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 109, 97, 112]))) {
                                                                                                    shared_constructor_field6_2
                                                                                                } else {
                                                                                                    0f64
                                                                                                }))) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331Return(self::ml_string::Text::from_units(vec![110, 101, 115, 116, 101, 100, 32, 115, 112, 101, 99, 105, 101, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 115, 101, 108, 101, 99, 116, 105, 111, 110]))
                                                                                                } else {
                                                                                                    {
                                                                                                        let source_ml_source1_shadow_error_3 = self::read_java_script_species_construction_state_error(program.clone(), heap.clone(), species_3.clone(), construction_3.clone().expect("a proven present value"));
                                                                                                        if (source_ml_source1_shadow_error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331Return(source_ml_source1_shadow_error_3.clone())
                                                                                                        } else {
                                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331Next
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331Next
                                                                                        };
                                                                                        match source_ml_source2_s31616.clone() {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331Next => {
                                                                                                if (suspended && if !(match (construction_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                    match (match construction_3.clone().expect("a proven present value") {
                                                                                                        self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, _, source_ml_source2_data_property_value31638, _, _) => {
                                                                                                            ((source_ml_source2_data_property_value31638.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionCall>,))).0).clone()
                                                                                                        }
                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                                } else {
                                                                                                    match (match state_3.clone() {
                                                                                                        self::JavaScriptArrayTransformState::ArrayTransformState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value31639) => {
                                                                                                            ((source_ml_source2_data_property_value31639.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayTransformCall>,))).0).clone()
                                                                                                        }
                                                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                                                                                }) {
                                                                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 32, 99, 104, 105, 108, 100, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108])
                                                                                                } else {
                                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331::MlSource1TemporaryReadJavaScriptCallPendingArraySpeciesErrorJoin1331Return(source_ml_source1_temporary_result1331) => {
                                                                                                source_ml_source1_temporary_result1331.clone()
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
    }
}
