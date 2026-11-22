// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionCall;
use super::JavaScriptConstructionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSpeciesConstructionState;
use super::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480;
use super::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_collection_constructor_kind;
use super::read_java_script_executable_call_function;
use super::read_java_script_pending_construction;
use super::read_java_script_pending_species_construction;
use super::read_java_script_suspended_constructor_frame_error;
use super::test_java_script_collection_constructor_mode;
use super::test_java_script_ordinary_class_constructor_mode;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_active_construction_frame_error(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    index: f64,
    frame: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
) -> self::ml_string::Text {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            _,
            _,
            shared_constructor_field6,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            match frame.clone() {
                self::JavaScriptCallFrame::CallFrame(
                    shared_constructor_field3,
                    _,
                    shared_constructor_field4,
                    shared_constructor_field5,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    match machine.clone() {
                        self::JavaScriptCallMachine::CallMachine(
                            shared_constructor_field1,
                            shared_constructor_field2,
                        ) => {
                            let shared_constructor_field1 = *shared_constructor_field1;
                            {
                                let source_ml_source2_s34804 = if (!(match (
                                    match frame.clone() {
                                        self::JavaScriptCallFrame::CallFrame(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value34805,
                                        ) => ((source_ml_source2_data_property_value34805
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptSemanticValue>,)
                                            }))
                                        .0)
                                            .clone(),
                                    },
                                    None::<()>,
                                ) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) && (shared_constructor_field6
                                    != (self::ml_string::Text::from_units(vec![
                                        108, 101, 120, 105, 99, 97, 108,
                                    ])))) {
                                    {
                                        let parent = {
                                            let source_ml_source2_array_read_value34809 =
                                                shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_array_read_index34810 =
                                                    (index - 1f64);
                                                {
                                                    let ml_reference_array2256 =
                                                        source_ml_source2_array_read_value34809
                                                            .clone();
                                                    let ml_reference_position2256 =
                                                        source_ml_source2_array_read_index34810;
                                                    let ml_reference_projection2256 =
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array2256.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position2256,
                                                            ),
                                                        );
                                                    ml_reference_projection2256
                                                }
                                            }
                                        };
                                        {
                                            let caller = if match (parent.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                None::<self::JavaScriptCallFunction>
                                            } else {
                                                self::read_java_script_executable_call_function(program.clone(), shared_constructor_field1.clone(), match parent.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value34812, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value34812.clone()
                                                    }
                                                })
                                            };
                                            {
                                                let instruction = if ((match (
                                                    caller.clone(),
                                                    None::<()>,
                                                ) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                } || match (
                                                    parent.clone(),
                                                    None::<()>,
                                                ) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                }) || ((match parent
                                                    .clone()
                                                    .expect("a proven present value")
                                                {
                                                    self::JavaScriptCallFrame::CallFrame(
                                                        _,
                                                        source_ml_source2_data_property_value34814,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source2_data_property_value34814,
                                                }) == (0f64)))
                                                {
                                                    None::<self::JavaScriptCallInstruction>
                                                } else {
                                                    {
                                                        let source_ml_source2_array_read_value34817 = match caller.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value34815, _, _) => {
                                                                source_ml_source2_data_property_value34815.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_array_read_index34818 = (match parent.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value34816, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value34816
                                                                }
                                                            } - 1f64);
                                                            {
                                                                let ml_reference_array2257 = source_ml_source2_array_read_value34817.clone();
                                                                let ml_reference_position2257 = source_ml_source2_array_read_index34818;
                                                                let ml_reference_projection2257 = self::ml_array::optional_at(&*ml_reference_array2257.borrow(), self::ml_array::number_index(ml_reference_position2257));
                                                                ml_reference_projection2257
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let generic = if match (
                                                        parent.clone(),
                                                        None::<()>,
                                                    ) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    } {
                                                        None::<self::JavaScriptConstructionState>
                                                    } else {
                                                        self::read_java_script_pending_construction(match parent.clone().expect("a proven present value") {
                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value34819, _, _, _, _) => {
                                                                let source_ml_source2_data_property_value34819 = *source_ml_source2_data_property_value34819;
                                                                source_ml_source2_data_property_value34819.clone()
                                                            }
                                                        })
                                                    };
                                                    {
                                                        let species = if match (
                                                            parent.clone(),
                                                            None::<()>,
                                                        ) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        } {
                                                            None::<self::JavaScriptSpeciesConstructionState>
                                                        } else {
                                                            self::read_java_script_pending_species_construction(match parent.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value34820, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value34820 = *source_ml_source2_data_property_value34820;
                                                                    source_ml_source2_data_property_value34820.clone()
                                                                }
                                                            })
                                                        };
                                                        if !(match (generic.clone(), None::<()>) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        }) {
                                                            if ((({
                                                                let source_ml_source2_comparison_operand34824 = {
                                                                    let source_ml_source2_optional_chain_receiver34822 = (match generic.clone().expect("a proven present value") {
                                                                        self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value34821, _) => {
                                                                            ((source_ml_source2_data_property_value34821.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                                        }
                                                                    },);
                                                                    if match (source_ml_source2_optional_chain_receiver34822.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                        Some({
                                                                            let source_ml_source1_temporary_optional_chain1478 = ((source_ml_source2_optional_chain_receiver34822.clone()).0).clone().expect("a proven present value");
                                                                            match source_ml_source1_temporary_optional_chain1478.clone() {
                                                                                self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, source_ml_source2_data_property_value34823, _) => {
                                                                                    source_ml_source2_data_property_value34823
                                                                                }
                                                                            }
                                                                        })
                                                                    } else {
                                                                        None::<bool>
                                                                    }
                                                                };
                                                                !(match (source_ml_source2_comparison_operand34824.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                            } || !(self::test_java_script_semantic_same_value(match generic.clone().expect("a proven present value") {
                                                                self::JavaScriptConstructionState::ConstructionState(_, source_ml_source2_data_property_value34826, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value34826 = *source_ml_source2_data_property_value34826;
                                                                    source_ml_source2_data_property_value34826.clone()
                                                                }
                                                            }, (match frame.clone() {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value34827) => {
                                                                    ((source_ml_source2_data_property_value34827.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value")))) || {
                                                                let source_ml_source2_s34829 = match generic.clone().expect("a proven present value") {
                                                                    self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value34832, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value34832 = *source_ml_source2_data_property_value34832;
                                                                        source_ml_source2_data_property_value34832.clone()
                                                                    }
                                                                };
                                                                match source_ml_source2_s34829.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                }
                                                            }) || (((((match match generic.clone().expect("a proven present value") {
                                                                self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value34836, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                    let source_ml_source2_data_property_value34836 = *source_ml_source2_data_property_value34836;
                                                                    source_ml_source2_data_property_value34836.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value34837, _) => {
                                                                    Some((source_ml_source2_data_property_value34837.clone(),))
                                                                }
                                                                _ => {
                                                                    None::<(self::ml::Big,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field3)) {
                                                                self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Return(self::ml_string::Text::from_units(vec![110, 101, 119, 46, 116, 97, 114, 103, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 98, 97, 115, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Next
                                                            }
                                                        } else {
                                                            if (!(match (species.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match species.clone().expect("a proven present value") {
                                                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, source_ml_source2_data_property_value34840, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value34840.clone()
                                                                }
                                                            }) == (self::ml_string::Text::from_units(vec![98, 111, 100, 121])))) {
                                                                if !(self::test_java_script_semantic_same_value(match species.clone().expect("a proven present value") {
                                                                    self::JavaScriptSpeciesConstructionState::SpeciesConstruction(source_ml_source2_data_property_value34841, _, _, _, _, _, _, _, _, _) => {
                                                                        let source_ml_source2_data_property_value34841 = *source_ml_source2_data_property_value34841;
                                                                        source_ml_source2_data_property_value34841.clone()
                                                                    }
                                                                }, (match frame.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value34842) => {
                                                                        ((source_ml_source2_data_property_value34842.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value"))) {
                                                                    self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Return(self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 110, 101, 119, 46, 116, 97, 114, 103, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Next
                                                                }
                                                            } else {
                                                                if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 67, 97, 108, 108, 32, 99, 97, 110, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 119, 46, 116, 97, 114, 103, 101, 116]))
                                                                } else {
                                                                    {
                                                                        let legacy = (((match instruction.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallInstruction::CallClassConstructor(_, _, _, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        } || match instruction.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallInstruction::ConstructCollection(_, _, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        }) || match instruction.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallInstruction::ConstructErrorClass(_, _, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        }) || (match instruction.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, _, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        } && self::ml_array::includes(&{ let ml_reference_value2258 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115]), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114, 32, 99, 108, 97, 115, 115])]))).borrow().clone(); ml_reference_value2258 }, &(((match instruction.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, _, source_ml_source2_data_property_value34858) => {
                                                                                Some((source_ml_source2_data_property_value34858.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(self::ml_string::Text,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone(), 0f64, false, |a, b| a == b)));
                                                                        if ((!legacy || match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(({
                                                                            let source_ml_source2_array_read_value34866 = match parent.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value34859, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value34859.clone()
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_array_read_index34867 = match match instruction.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallInstruction::Call(_, source_ml_source2_data_property_value34860, _, _) => {
                                                                                        Some(source_ml_source2_data_property_value34860)
                                                                                    }
                                                                                    self::JavaScriptCallInstruction::ArgumentListCall(_, _, source_ml_source2_data_property_value34861, _, _) => {
                                                                                        Some(source_ml_source2_data_property_value34861)
                                                                                    }
                                                                                    self::JavaScriptCallInstruction::Construct(_, source_ml_source2_data_property_value34862, _, _) => {
                                                                                        Some(source_ml_source2_data_property_value34862)
                                                                                    }
                                                                                    self::JavaScriptCallInstruction::ConstructErrorClass(_, source_ml_source2_data_property_value34863, _) => {
                                                                                        Some(source_ml_source2_data_property_value34863)
                                                                                    }
                                                                                    self::JavaScriptCallInstruction::CallClassConstructor(_, source_ml_source2_data_property_value34864, _, _) => {
                                                                                        Some(source_ml_source2_data_property_value34864)
                                                                                    }
                                                                                    self::JavaScriptCallInstruction::ConstructCollection(_, source_ml_source2_data_property_value34865, _) => {
                                                                                        Some(source_ml_source2_data_property_value34865)
                                                                                    }
                                                                                    _ => {
                                                                                        None::<f64>
                                                                                    }
                                                                                } { Some(ml_present) => ml_present, None => f64::NAN };
                                                                                { let ml_reference_array2259 = source_ml_source2_array_read_value34866.clone(); let ml_reference_position2259 = source_ml_source2_array_read_index34867; let ml_reference_projection2259 = self::ml_array::optional_at(&*ml_reference_array2259.borrow(), self::ml_array::number_index(ml_reference_position2259)); ml_reference_projection2259 }
                                                                            }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (match frame.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value34868) => {
                                                                                ((source_ml_source2_data_property_value34868.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value")))) {
                                                                            self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 67, 97, 108, 108, 32, 99, 97, 110, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 119, 46, 116, 97, 114, 103, 101, 116]))
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Next
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
                                    self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Next
                                };
                                match source_ml_source2_s34804.clone() {
                                    self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Next => {
                                        {
                                            let source_ml_source2_s34985 = if (self::test_java_script_ordinary_class_constructor_mode(&(shared_constructor_field6)) || self::test_java_script_collection_constructor_mode(&(shared_constructor_field6))) {
                                                {
                                                    let parent_2 = {
                                                        let source_ml_source2_array_read_value34989 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_array_read_index34990 = (index - 1f64);
                                                            { let ml_reference_array2260 = source_ml_source2_array_read_value34989.clone(); let ml_reference_position2260 = source_ml_source2_array_read_index34990; let ml_reference_projection2260 = self::ml_array::optional_at(&*ml_reference_array2260.borrow(), self::ml_array::number_index(ml_reference_position2260)); ml_reference_projection2260 }
                                                        }
                                                    };
                                                    {
                                                        let caller_2 = if match (parent_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            None::<self::JavaScriptCallFunction>
                                                        } else {
                                                            self::read_java_script_executable_call_function(program.clone(), shared_constructor_field1.clone(), match parent_2.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value34992, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value34992.clone()
                                                                }
                                                            })
                                                        };
                                                        {
                                                            let instruction_2 = if ((match (parent_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match (caller_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || ((match parent_2.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value34994, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value34994
                                                                }
                                                            }) == (0f64))) {
                                                                None::<self::JavaScriptCallInstruction>
                                                            } else {
                                                                {
                                                                    let source_ml_source2_array_read_value34997 = match caller_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value34995, _, _) => {
                                                                            source_ml_source2_data_property_value34995.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_array_read_index34998 = (match parent_2.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value34996, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value34996
                                                                            }
                                                                        } - 1f64);
                                                                        { let ml_reference_array2261 = source_ml_source2_array_read_value34997.clone(); let ml_reference_position2261 = source_ml_source2_array_read_index34998; let ml_reference_projection2261 = self::ml_array::optional_at(&*ml_reference_array2261.borrow(), self::ml_array::number_index(ml_reference_position2261)); ml_reference_projection2261 }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let generic_2 = if match (parent_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    None::<self::JavaScriptConstructionState>
                                                                } else {
                                                                    self::read_java_script_pending_construction(match parent_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value34999, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value34999 = *source_ml_source2_data_property_value34999;
                                                                            source_ml_source2_data_property_value34999.clone()
                                                                        }
                                                                    })
                                                                };
                                                                if !(match (generic_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                    if ((((((!((self::test_java_script_ordinary_class_constructor_mode(&(shared_constructor_field6)) || ((self::read_java_script_collection_constructor_kind(fn_.clone())) == (match generic_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptConstructionState::ConstructionState(_, _, _, _, source_ml_source2_data_property_value35002, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value35002.clone()
                                                                        }
                                                                    })))) || ((match generic_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptConstructionState::ConstructionState(_, _, _, source_ml_source2_data_property_value35004, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value35004.clone()
                                                                        }
                                                                    }) != (self::ml_string::Text::from_units(vec![98, 111, 100, 121])))) || {
                                                                        let source_ml_source2_comparison_operand35008 = {
                                                                            let source_ml_source2_optional_chain_receiver35006 = (match generic_2.clone().expect("a proven present value") {
                                                                                self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35005, _) => {
                                                                                    ((source_ml_source2_data_property_value35005.clone().unwrap_or_else(|| (None::<self::JavaScriptConstructionCall>,))).0).clone()
                                                                                }
                                                                            },);
                                                                            if match (source_ml_source2_optional_chain_receiver35006.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                Some({
                                                                                    let source_ml_source1_temporary_optional_chain1479 = ((source_ml_source2_optional_chain_receiver35006.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_temporary_optional_chain1479.clone() {
                                                                                        self::JavaScriptConstructionCall::ConstructionCallRecord(_, _, _, source_ml_source2_data_property_value35007, _) => {
                                                                                            source_ml_source2_data_property_value35007
                                                                                        }
                                                                                    }
                                                                                })
                                                                            } else {
                                                                                None::<bool>
                                                                            }
                                                                        };
                                                                        !(match (source_ml_source2_comparison_operand35008.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                    }) || {
                                                                        let source_ml_source2_s35011 = match generic_2.clone().expect("a proven present value") {
                                                                            self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value35014, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value35014 = *source_ml_source2_data_property_value35014;
                                                                                source_ml_source2_data_property_value35014.clone()
                                                                            }
                                                                        };
                                                                        match source_ml_source2_s35011.clone() {
                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                false
                                                                            }
                                                                            _ => {
                                                                                true
                                                                            }
                                                                        }
                                                                    }) || (((((match match generic_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value35018, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value35018 = *source_ml_source2_data_property_value35018;
                                                                            source_ml_source2_data_property_value35018.clone()
                                                                        }
                                                                    } {
                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value35019, _) => {
                                                                            Some((source_ml_source2_data_property_value35019.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::ml::Big,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field3)) || !(self::test_java_script_semantic_same_value(match generic_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, source_ml_source2_data_property_value35021, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value35021 = *source_ml_source2_data_property_value35021;
                                                                            source_ml_source2_data_property_value35021.clone()
                                                                        }
                                                                    }, ({
                                                                        let source_ml_source2_array_read_value35023 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index35024 = 0f64;
                                                                            { let ml_reference_array2262 = source_ml_source2_array_read_value35023.clone(); let ml_reference_position2262 = source_ml_source2_array_read_index35024; let ml_reference_projection2262 = self::ml_array::optional_at(&*ml_reference_array2262.borrow(), self::ml_array::number_index(ml_reference_position2262)); ml_reference_projection2262 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(match generic_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptConstructionState::ConstructionState(_, source_ml_source2_data_property_value35025, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value35025 = *source_ml_source2_data_property_value35025;
                                                                            source_ml_source2_data_property_value35025.clone()
                                                                        }
                                                                    }, (match frame.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value35026) => {
                                                                            ((source_ml_source2_data_property_value35026.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) {
                                                                        self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Return(self::ml_string::Text::from_units(vec![97, 32, 98, 97, 115, 101, 32, 99, 108, 97, 115, 115, 32, 111, 114, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 110, 101, 114, 105, 99, 32, 67, 111, 110, 115, 116, 114, 117, 99, 116, 32, 99, 97, 108, 108]))
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Next
                                                                    }
                                                                } else {
                                                                    if ((!(match (parent_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                                                        let source_ml_source2_s35028 = match parent_2.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value35033, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value35033 = *source_ml_source2_data_property_value35033;
                                                                                source_ml_source2_data_property_value35033.clone()
                                                                            }
                                                                        };
                                                                        match source_ml_source2_s35028.clone() {
                                                                            self::JavaScriptCallPending::WaitArrayTransform(_, _, _, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        }
                                                                    } || {
                                                                        let source_ml_source2_s35035 = match parent_2.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value35040, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value35040 = *source_ml_source2_data_property_value35040;
                                                                                source_ml_source2_data_property_value35040.clone()
                                                                            }
                                                                        };
                                                                        match source_ml_source2_s35035.clone() {
                                                                            self::JavaScriptCallPending::WaitArraySlice(_, _, _, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        }
                                                                    }) || {
                                                                        let source_ml_source2_s35042 = match parent_2.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value35047, _, _, _, _) => {
                                                                                let source_ml_source2_data_property_value35047 = *source_ml_source2_data_property_value35047;
                                                                                source_ml_source2_data_property_value35047.clone()
                                                                            }
                                                                        };
                                                                        match source_ml_source2_s35042.clone() {
                                                                            self::JavaScriptCallPending::WaitArrayConcatenation(_, _, _, _) => {
                                                                                true
                                                                            }
                                                                            _ => {
                                                                                false
                                                                            }
                                                                        }
                                                                    })) && !(match ((((match match parent_2.clone().expect("a proven present value") {
                                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value35048, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value35048 = *source_ml_source2_data_property_value35048;
                                                                            source_ml_source2_data_property_value35048.clone()
                                                                        }
                                                                    } {
                                                                        self::JavaScriptCallPending::WaitArrayConcatenation(_, _, _, source_ml_source2_data_property_value35049) => {
                                                                            Some((((source_ml_source2_data_property_value35049.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone(),))
                                                                        }
                                                                        self::JavaScriptCallPending::WaitArraySlice(_, _, _, source_ml_source2_data_property_value35050) => {
                                                                            Some((((source_ml_source2_data_property_value35050.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone(),))
                                                                        }
                                                                        self::JavaScriptCallPending::WaitArrayTransform(_, _, _, source_ml_source2_data_property_value35051) => {
                                                                            Some((((source_ml_source2_data_property_value35051.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(Option<self::JavaScriptSpeciesConstructionState>,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                        {
                                                                            let state = (((match match parent_2.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value35052, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value35052 = *source_ml_source2_data_property_value35052;
                                                                                    source_ml_source2_data_property_value35052.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptCallPending::WaitArrayConcatenation(_, _, _, source_ml_source2_data_property_value35053) => {
                                                                                    Some((((source_ml_source2_data_property_value35053.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone(),))
                                                                                }
                                                                                self::JavaScriptCallPending::WaitArraySlice(_, _, _, source_ml_source2_data_property_value35054) => {
                                                                                    Some((((source_ml_source2_data_property_value35054.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone(),))
                                                                                }
                                                                                self::JavaScriptCallPending::WaitArrayTransform(_, _, _, source_ml_source2_data_property_value35055) => {
                                                                                    Some((((source_ml_source2_data_property_value35055.clone().unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,))).0).clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(Option<self::JavaScriptSpeciesConstructionState>,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone();
                                                                            if ((((((!(self::test_java_script_ordinary_class_constructor_mode(&(shared_constructor_field6))) || (({
                                                                                let source_ml_source2_nullish_field_receiver35060 = (state.clone(),);
                                                                                if match (source_ml_source2_nullish_field_receiver35060.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source2_nullish_field_present35061 = ((source_ml_source2_nullish_field_receiver35060.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source2_nullish_field_present35061.clone() {
                                                                                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, source_ml_source2_data_property_value35062, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value35062.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 112, 104, 97, 115, 101, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            }) != (self::ml_string::Text::from_units(vec![98, 111, 100, 121])))) || {
                                                                                let source_ml_source2_s35066 = {
                                                                                    let source_ml_source2_nullish_field_receiver35069 = (state.clone(),);
                                                                                    if match (source_ml_source2_nullish_field_receiver35069.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        {
                                                                                            let source_ml_source2_nullish_field_present35070 = ((source_ml_source2_nullish_field_receiver35069.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source2_nullish_field_present35070.clone() {
                                                                                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(source_ml_source2_data_property_value35071, _, _, _, _, _, _, _, _, _) => {
                                                                                                    let source_ml_source2_data_property_value35071 = *source_ml_source2_data_property_value35071;
                                                                                                    source_ml_source2_data_property_value35071.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                                    }
                                                                                };
                                                                                match source_ml_source2_s35066.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }
                                                                            }) || (((((match {
                                                                                let source_ml_source2_nullish_field_receiver35077 = (state.clone(),);
                                                                                if match (source_ml_source2_nullish_field_receiver35077.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source2_nullish_field_present35078 = ((source_ml_source2_nullish_field_receiver35077.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source2_nullish_field_present35078.clone() {
                                                                                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(source_ml_source2_data_property_value35079, _, _, _, _, _, _, _, _, _) => {
                                                                                                let source_ml_source2_data_property_value35079 = *source_ml_source2_data_property_value35079;
                                                                                                source_ml_source2_data_property_value35079.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value35080, _) => {
                                                                                    Some((source_ml_source2_data_property_value35080.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(self::ml::Big,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field3)) || !(self::test_java_script_semantic_same_value({
                                                                                let source_ml_source2_nullish_field_receiver35082 = (state.clone(),);
                                                                                if match (source_ml_source2_nullish_field_receiver35082.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source2_nullish_field_present35083 = ((source_ml_source2_nullish_field_receiver35082.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source2_nullish_field_present35083.clone() {
                                                                                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, _, _, _, _, source_ml_source2_data_property_value35084, _, _, _) => {
                                                                                                let source_ml_source2_data_property_value35084 = *source_ml_source2_data_property_value35084;
                                                                                                source_ml_source2_data_property_value35084.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 99, 101, 105, 118, 101, 114, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            }, ({
                                                                                let source_ml_source2_array_read_value35086 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index35087 = 0f64;
                                                                                    { let ml_reference_array2263 = source_ml_source2_array_read_value35086.clone(); let ml_reference_position2263 = source_ml_source2_array_read_index35087; let ml_reference_projection2263 = self::ml_array::optional_at(&*ml_reference_array2263.borrow(), self::ml_array::number_index(ml_reference_position2263)); ml_reference_projection2263 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || (({ let ml_reference_array2264 = shared_constructor_field5.clone(); let ml_reference_projection2264 = ml_reference_array2264.borrow().len() as f64; ml_reference_projection2264 }) != (1f64))) || !(self::test_java_script_semantic_same_value(({
                                                                                let source_ml_source2_array_read_value35091 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_array_read_index35092 = 0f64;
                                                                                    { let ml_reference_array2265 = source_ml_source2_array_read_value35091.clone(); let ml_reference_position2265 = source_ml_source2_array_read_index35092; let ml_reference_projection2265 = self::ml_array::optional_at(&*ml_reference_array2265.borrow(), self::ml_array::number_index(ml_reference_position2265)); ml_reference_projection2265 }
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::JavaScriptSemanticValue::Number({
                                                                                let source_ml_source2_nullish_field_receiver35093 = (state.clone(),);
                                                                                if match (source_ml_source2_nullish_field_receiver35093.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source2_nullish_field_present35094 = ((source_ml_source2_nullish_field_receiver35093.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source2_nullish_field_present35094.clone() {
                                                                                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(_, _, source_ml_source2_data_property_value35095, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value35095
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            })))) {
                                                                                self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Return(self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115, 32, 115, 111, 117, 114, 99, 101, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 110, 101, 119, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Next
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if ((match (parent_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || {
                                                                            let source_ml_source2_s35097 = match parent_2.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value35099, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value35099 = *source_ml_source2_data_property_value35099;
                                                                                    source_ml_source2_data_property_value35099.clone()
                                                                                }
                                                                            };
                                                                            match source_ml_source2_s35097.clone() {
                                                                                self::JavaScriptCallPending::WaitCall(_) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        }) || match (instruction_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Return(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 110, 101, 119, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                                                        } else {
                                                                            {
                                                                                let error = self::read_java_script_suspended_constructor_frame_error(parent_2.clone().expect("a proven present value"), frame.clone(), fn_.clone(), instruction_2.clone().expect("a proven present value"), ((match match parent_2.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value35100, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value35100 = *source_ml_source2_data_property_value35100;
                                                                                        source_ml_source2_data_property_value35100.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptCallPending::WaitCall(source_ml_source2_data_property_value35101) => {
                                                                                        Some((source_ml_source2_data_property_value35101,))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(f64,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0);
                                                                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Return(error.clone())
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Next
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
                                                self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Next
                                            };
                                            match source_ml_source2_s34985.clone() {
                                                self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Next => {
                                                    self::ml_string::Text::from_units(vec![])
                                                }
                                                self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1481Return(source_ml_source1_temporary_result1481) => {
                                                    source_ml_source1_temporary_result1481.clone()
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480::MlSource1TemporaryReadJavaScriptActiveConstructionFrameErrorJoin1480Return(source_ml_source1_temporary_result1481_2) => {
                                        source_ml_source1_temporary_result1481_2.clone()
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
