// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptArraySliceCall;
use super::JavaScriptArraySliceState;
use super::JavaScriptArraySliceTransition;
use super::JavaScriptArraySpeciesCall;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptObjectCopyCall;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358;
use super::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359;
use super::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_object_spread_state_error;
use super::same_call;
use super::source_ml_source1_temporary_allocation_request_loop362;
use super::source_ml_source1_temporary_read_java_script_array_slice_child_state_error_loop360;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_slice_child_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySliceState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArraySliceState::ArraySliceState(
            shared_constructor_field8,
            _,
            _,
            _,
            shared_constructor_field9,
            _,
            _,
            _,
            shared_constructor_field10,
            _,
            _,
            shared_constructor_field11,
            _,
            _,
            shared_constructor_field12,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field11 = *shared_constructor_field11;
            if ((shared_constructor_field12
                == (self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104])))
                != (!(match (
                    match state.clone() {
                        self::JavaScriptArraySliceState::ArraySliceState(
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
                            source_ml_source2_data_property_value11094,
                            _,
                        ) => ((source_ml_source2_data_property_value11094
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,)))
                        .0)
                            .clone(),
                    },
                    None::<()>,
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                })))
            {
                self::ml_string::Text::from_units(vec![
                    115, 108, 105, 99, 101, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116, 32, 114,
                    101, 116, 97, 105, 110, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 97,
                    99, 116, 117, 97, 108, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32,
                    99, 104, 105, 108, 100,
                ])
            } else {
                {
                    let source_ml_source2_s11131 = if !(match (
                        match state.clone() {
                            self::JavaScriptArraySliceState::ArraySliceState(
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
                                source_ml_source2_data_property_value11132,
                                _,
                            ) => ((source_ml_source2_data_property_value11132
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let child = (match state.clone() {
                                self::JavaScriptArraySliceState::ArraySliceState(
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
                                    source_ml_source2_data_property_value11133,
                                    _,
                                ) => ((source_ml_source2_data_property_value11133
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptObjectSpreadState>,)
                                    }))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value");
                            match child.clone() {
                                self::JavaScriptObjectSpreadState::ObjectSpreadState(
                                    shared_constructor_field1,
                                    shared_constructor_field2,
                                    shared_constructor_field3,
                                    shared_constructor_field4,
                                    shared_constructor_field5,
                                    shared_constructor_field6,
                                    shared_constructor_field7,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => {
                                    let shared_constructor_field1 = *shared_constructor_field1;
                                    let shared_constructor_field2 = *shared_constructor_field2;
                                    let shared_constructor_field7 = *shared_constructor_field7;
                                    {
                                        let error =
                                            self::read_java_script_object_spread_state_error(
                                                heap.clone(),
                                                child.clone(),
                                            );
                                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                                            self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358Return(error.clone())
                                        } else {
                                            if ((((((((({
                                                let source_ml_source2_comparison_operand11135 = match child.clone() {
                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value11134, _, _, _) => {
                                                        ((source_ml_source2_data_property_value11134.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                    }
                                                };
                                                !(match (source_ml_source2_comparison_operand11135.clone(), self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                            } || !(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), shared_constructor_field11.clone()))) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field8.clone()))) || !(self::test_java_script_semantic_same_value((match child.clone() {
                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11141, _, _) => {
                                                    ((source_ml_source2_data_property_value11141.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field9.clone()))) || (({ let ml_reference_array746 = shared_constructor_field4.clone(); let ml_reference_projection746 = ml_reference_array746.borrow().len() as f64; ml_reference_projection746 }) != (1f64))) || !(self::test_java_script_semantic_same_value(({
                                                let source_ml_source2_array_read_value11146 = shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_array_read_index11147 = 0f64;
                                                    { let ml_reference_array747 = source_ml_source2_array_read_value11146.clone(); let ml_reference_position747 = source_ml_source2_array_read_index11147; let ml_reference_projection747 = self::ml_array::optional_at(&*ml_reference_array747.borrow(), self::ml_array::number_index(ml_reference_position747)); ml_reference_projection747 }
                                                }
                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64])))))) || (({ let ml_reference_array748 = shared_constructor_field3.clone(); let ml_reference_projection748 = ml_reference_array748.borrow().len() as f64; ml_reference_projection748 }) != (0f64))) || !((((shared_constructor_field5 == (0f64)) && self::ml_array::includes(&{ let ml_reference_value749 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value749 }, &shared_constructor_field6.clone(), 0f64, false, |a, b| a == b)) || ((shared_constructor_field5 == (1f64)) && (shared_constructor_field6 == (self::ml_string::Text::from_units(vec![114, 101, 97, 100]))))))) || ((shared_constructor_field5 == (0f64)) && !(self::test_java_script_semantic_same_value(shared_constructor_field7.clone(), self::JavaScriptSemanticValue::Number(shared_constructor_field10))))) || (!(match (match child.clone() {
                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11161, _) => {
                                                    ((source_ml_source2_data_property_value11161.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                }
                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(self::test_java_script_semantic_same_value((({ let ml_reference_value750 = (match match (match child.clone() {
                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11162, _) => {
                                                    ((source_ml_source2_data_property_value11162.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value") {
                                                self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(source_ml_source2_data_property_value11163, _) => {
                                                    let source_ml_source2_data_property_value11163 = *source_ml_source2_data_property_value11163;
                                                    source_ml_source2_data_property_value11163.clone()
                                                }
                                                self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(source_ml_source2_data_property_value11164, _, _) => {
                                                    let source_ml_source2_data_property_value11164 = *source_ml_source2_data_property_value11164;
                                                    source_ml_source2_data_property_value11164.clone()
                                                }
                                                self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(source_ml_source2_data_property_value11165, _, _) => {
                                                    let source_ml_source2_data_property_value11165 = *source_ml_source2_data_property_value11165;
                                                    source_ml_source2_data_property_value11165.clone()
                                                }
                                            } {
                                                self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value11166, _, _, _) => {
                                                    source_ml_source2_data_property_value11166.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value750 }).0).8, self::JavaScriptSemanticValue::Number(shared_constructor_field10))))) {
                                                self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 116, 97, 114, 103, 101, 116, 44, 32, 108, 101, 110, 103, 116, 104, 32, 107, 101, 121, 32, 97, 110, 100, 32, 99, 111, 117, 110, 116]))
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358Next
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358Next
                    };
                    match source_ml_source2_s11131.clone() {
                        self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358Next => {
                            {
                                let source_ml_source2_s11246 = if !(match (match state.clone() {
                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11247) => {
                                        ((source_ml_source2_data_property_value11247.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                    }
                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    {
                                        let source_ml_source1_temporary_values353 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![match (match state.clone() {
                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11248) => {
                                                ((source_ml_source2_data_property_value11248.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value") {
                                            self::JavaScriptArraySliceCall::ArraySliceCall(source_ml_source2_data_property_value11249, _, _) => {
                                                let source_ml_source2_data_property_value11249 = *source_ml_source2_data_property_value11249;
                                                source_ml_source2_data_property_value11249.clone()
                                            }
                                        }, match (match state.clone() {
                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11250) => {
                                                ((source_ml_source2_data_property_value11250.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value") {
                                            self::JavaScriptArraySliceCall::ArraySliceCall(_, source_ml_source2_data_property_value11251, _) => {
                                                let source_ml_source2_data_property_value11251 = *source_ml_source2_data_property_value11251;
                                                source_ml_source2_data_property_value11251.clone()
                                            }
                                        }], { let ml_reference_value751 = (match (match state.clone() {
                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11252) => {
                                                ((source_ml_source2_data_property_value11252.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value") {
                                            self::JavaScriptArraySliceCall::ArraySliceCall(_, _, source_ml_source2_data_property_value11253) => {
                                                source_ml_source2_data_property_value11253.clone()
                                            }
                                        }).borrow().clone(); ml_reference_value751 })));
                                        {
                                            let source_ml_source1_temporary_index353 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s11254 = self::source_ml_source1_temporary_read_java_script_array_slice_child_state_error_loop360(heap.clone(), source_ml_source1_temporary_values353.clone(), source_ml_source1_temporary_index353.clone());
                                                match source_ml_source2_s11254.clone() {
                                                    self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Result::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Done => {
                                                        if ({
                                                            let source_ml_source2_s11257 = match (match state.clone() {
                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11260) => {
                                                                    ((source_ml_source2_data_property_value11260.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value") {
                                                                self::JavaScriptArraySliceCall::ArraySliceCall(source_ml_source2_data_property_value11261, _, _) => {
                                                                    let source_ml_source2_data_property_value11261 = *source_ml_source2_data_property_value11261;
                                                                    source_ml_source2_data_property_value11261.clone()
                                                                }
                                                            };
                                                            match source_ml_source2_s11257.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                        } || !(((match match (match state.clone() {
                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11262) => {
                                                                ((source_ml_source2_data_property_value11262.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                            }
                                                        }).clone().expect("a proven present value") {
                                                            self::JavaScriptArraySliceCall::ArraySliceCall(source_ml_source2_data_property_value11263, _, _) => {
                                                                let source_ml_source2_data_property_value11263 = *source_ml_source2_data_property_value11263;
                                                                source_ml_source2_data_property_value11263.clone()
                                                            }
                                                        } {
                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value11264) => {
                                                                Some((source_ml_source2_data_property_value11264,))
                                                            }
                                                            _ => {
                                                                None::<(bool,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0)) {
                                                            self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101]))
                                                        } else {
                                                            if self::ml_array::includes(&{ let ml_reference_value752 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value752 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b) {
                                                                if (!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11266) => {
                                                                        ((source_ml_source2_data_property_value11266.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value") {
                                                                    self::JavaScriptArraySliceCall::ArraySliceCall(_, source_ml_source2_data_property_value11267, _) => {
                                                                        let source_ml_source2_data_property_value11267 = *source_ml_source2_data_property_value11267;
                                                                        source_ml_source2_data_property_value11267.clone()
                                                                    }
                                                                }, shared_constructor_field8.clone())) || (({ let ml_reference_array753 = match (match state.clone() {
                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11271) => {
                                                                        ((source_ml_source2_data_property_value11271.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value") {
                                                                    self::JavaScriptArraySliceCall::ArraySliceCall(_, _, source_ml_source2_data_property_value11272) => {
                                                                        source_ml_source2_data_property_value11272.clone()
                                                                    }
                                                                }; let ml_reference_projection753 = ml_reference_array753.borrow().len() as f64; ml_reference_projection753 }) != (0f64))) {
                                                                    self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 103, 101, 116, 116, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 98, 111, 120, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 110, 111, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Next
                                                                }
                                                            } else {
                                                                if (shared_constructor_field12 == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) {
                                                                    {
                                                                        let call = {
                                                                            let source_ml_source2_optional_chain_receiver11276 = (match state.clone() {
                                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11275, _, _, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value11275.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                }
                                                                            },);
                                                                            match if match (source_ml_source2_optional_chain_receiver11276.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                Some({
                                                                                    let source_ml_source1_temporary_optional_chain354 = ((source_ml_source2_optional_chain_receiver11276.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_temporary_optional_chain354.clone() {
                                                                                        self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value11277) => {
                                                                                            ((source_ml_source2_data_property_value11277.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                                                        }
                                                                                    }
                                                                                })
                                                                            } else {
                                                                                None::<self::JavaScriptArraySpeciesCall>
                                                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                                        };
                                                                        if (match (call.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::same_call((match state.clone() {
                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11278) => {
                                                                                ((source_ml_source2_data_property_value11278.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value"), match call.clone().expect("a proven present value") {
                                                                            self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(source_ml_source2_data_property_value11279, _, _) => {
                                                                                let source_ml_source2_data_property_value11279 = *source_ml_source2_data_property_value11279;
                                                                                source_ml_source2_data_property_value11279.clone()
                                                                            }
                                                                        }, match call.clone().expect("a proven present value") {
                                                                            self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(_, source_ml_source2_data_property_value11280, _) => {
                                                                                let source_ml_source2_data_property_value11280 = *source_ml_source2_data_property_value11280;
                                                                                source_ml_source2_data_property_value11280.clone()
                                                                            }
                                                                        }, match call.clone().expect("a proven present value") {
                                                                            self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(_, _, source_ml_source2_data_property_value11281) => {
                                                                                source_ml_source2_data_property_value11281.clone()
                                                                            }
                                                                        }))) {
                                                                            self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 115, 112, 101, 99, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 110, 101, 115, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114]))
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Next
                                                                        }
                                                                    }
                                                                } else {
                                                                    if (shared_constructor_field12 == (self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104]))) {
                                                                        {
                                                                            let call_2 = {
                                                                                let source_ml_source2_optional_chain_receiver11285 = (match state.clone() {
                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11284, _) => {
                                                                                        ((source_ml_source2_data_property_value11284.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                                                    }
                                                                                },);
                                                                                match if match (source_ml_source2_optional_chain_receiver11285.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    Some({
                                                                                        let source_ml_source1_temporary_optional_chain355 = ((source_ml_source2_optional_chain_receiver11285.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source1_temporary_optional_chain355.clone() {
                                                                                            self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11286) => {
                                                                                                ((source_ml_source2_data_property_value11286.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                                                                            }
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    None::<self::JavaScriptObjectCopyCall>
                                                                                } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                                            };
                                                                            if (match (call_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::same_call((match state.clone() {
                                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11287) => {
                                                                                    ((source_ml_source2_data_property_value11287.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value"), match call_2.clone().expect("a proven present value") {
                                                                                self::JavaScriptObjectCopyCall::ObjectCopyCall(source_ml_source2_data_property_value11288, _, _) => {
                                                                                    let source_ml_source2_data_property_value11288 = *source_ml_source2_data_property_value11288;
                                                                                    source_ml_source2_data_property_value11288.clone()
                                                                                }
                                                                            }, match call_2.clone().expect("a proven present value") {
                                                                                self::JavaScriptObjectCopyCall::ObjectCopyCall(_, source_ml_source2_data_property_value11289, _) => {
                                                                                    let source_ml_source2_data_property_value11289 = *source_ml_source2_data_property_value11289;
                                                                                    source_ml_source2_data_property_value11289.clone()
                                                                                }
                                                                            }, match call_2.clone().expect("a proven present value") {
                                                                                self::JavaScriptObjectCopyCall::ObjectCopyCall(_, _, source_ml_source2_data_property_value11290) => {
                                                                                    source_ml_source2_data_property_value11290.clone()
                                                                                }
                                                                            }))) {
                                                                                self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 110, 101, 115, 116, 101, 100, 32, 115, 101, 116, 116, 101, 114]))
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Next
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if ((shared_constructor_field12 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && !(match (match state.clone() {
                                                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11293, _, _, _, _, _, _) => {
                                                                                ((source_ml_source2_data_property_value11293.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                            {
                                                                                let child_2 = (match state.clone() {
                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11294, _, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value11294.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value");
                                                                                match child_2.clone() {
                                                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1_2, _, _, shared_constructor_field2_2) => {
                                                                                        let shared_constructor_field1_2 = *shared_constructor_field1_2;
                                                                                        {
                                                                                            let exotic = (shared_constructor_field2_2 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108])));
                                                                                            if (((!(self::ml_array::includes(&{ let ml_reference_value754 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])]))).borrow().clone(); ml_reference_value754 }, &shared_constructor_field2_2.clone(), 0f64, false, |a, b| a == b)) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11298) => {
                                                                                                    ((source_ml_source2_data_property_value11298.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptArraySliceCall::ArraySliceCall(_, source_ml_source2_data_property_value11299, _) => {
                                                                                                    let source_ml_source2_data_property_value11299 = *source_ml_source2_data_property_value11299;
                                                                                                    source_ml_source2_data_property_value11299.clone()
                                                                                                }
                                                                                            }, shared_constructor_field1_2.clone()))) || (({ let ml_reference_array755 = match (match state.clone() {
                                                                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11303) => {
                                                                                                    ((source_ml_source2_data_property_value11303.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptArraySliceCall::ArraySliceCall(_, _, source_ml_source2_data_property_value11304) => {
                                                                                                    source_ml_source2_data_property_value11304.clone()
                                                                                                }
                                                                                            }; let ml_reference_projection755 = ml_reference_array755.borrow().len() as f64; ml_reference_projection755 }) != (if exotic {
                                                                                                1f64
                                                                                            } else {
                                                                                                0f64
                                                                                            }))) || (exotic && !(self::test_java_script_semantic_same_value(({
                                                                                                let source_ml_source2_array_read_value11307 = match (match state.clone() {
                                                                                                    self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11305) => {
                                                                                                        ((source_ml_source2_data_property_value11305.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().expect("a proven present value") {
                                                                                                    self::JavaScriptArraySliceCall::ArraySliceCall(_, _, source_ml_source2_data_property_value11306) => {
                                                                                                        source_ml_source2_data_property_value11306.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_array_read_index11308 = 0f64;
                                                                                                    { let ml_reference_array756 = source_ml_source2_array_read_value11307.clone(); let ml_reference_position756 = source_ml_source2_array_read_index11308; let ml_reference_projection756 = self::ml_array::optional_at(&*ml_reference_array756.borrow(), self::ml_array::number_index(ml_reference_position756)); ml_reference_projection756 }
                                                                                                }
                                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![110f64, 117f64, 109f64, 98f64, 101f64, 114f64]))))))) {
                                                                                                self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 115, 99, 97, 108, 97, 114, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                            } else {
                                                                                                self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Next
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108, 115, 32, 111, 110, 108, 121, 32, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 104, 97, 115, 101]))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Result::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorLoop360Return(source_ml_source1_temporary_result360) => {
                                                        self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Return(source_ml_source1_temporary_result360.clone())
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if (((self::ml_array::includes(&{ let ml_reference_value757 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value757 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b) || (((shared_constructor_field12 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && !(match (match state.clone() {
                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11312, _, _, _, _, _, _) => {
                                            ((source_ml_source2_data_property_value11312.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                        }
                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) && self::ml_array::includes(&{ let ml_reference_value758 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])]))).borrow().clone(); ml_reference_value758 }, &match (match state.clone() {
                                        self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11313, _, _, _, _, _, _) => {
                                            ((source_ml_source2_data_property_value11313.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                        }
                                    }).clone().expect("a proven present value") {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value11314) => {
                                            source_ml_source2_data_property_value11314.clone()
                                        }
                                    }, 0f64, false, |a, b| a == b))) || ((shared_constructor_field12 == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && !(match ({
                                        let source_ml_source2_optional_chain_receiver11318 = (match state.clone() {
                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11317, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value11317.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                            }
                                        },);
                                        match if match (source_ml_source2_optional_chain_receiver11318.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            Some({
                                                let source_ml_source1_temporary_optional_chain356 = ((source_ml_source2_optional_chain_receiver11318.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_temporary_optional_chain356.clone() {
                                                    self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value11319) => {
                                                        ((source_ml_source2_data_property_value11319.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                    }
                                                }
                                            })
                                        } else {
                                            None::<self::JavaScriptArraySpeciesCall>
                                        } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) || ((shared_constructor_field12 == (self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104]))) && !(match ({
                                        let source_ml_source2_optional_chain_receiver11323 = (match state.clone() {
                                            self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11322, _) => {
                                                ((source_ml_source2_data_property_value11322.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                            }
                                        },);
                                        match if match (source_ml_source2_optional_chain_receiver11323.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            Some({
                                                let source_ml_source1_temporary_optional_chain357 = ((source_ml_source2_optional_chain_receiver11323.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_temporary_optional_chain357.clone() {
                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11324) => {
                                                        ((source_ml_source2_data_property_value11324.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                                    }
                                                }
                                            })
                                        } else {
                                            None::<self::JavaScriptObjectCopyCall>
                                        } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                                        self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Return(self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100]))
                                    } else {
                                        self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Next
                                    }
                                };
                                match source_ml_source2_s11246.clone() {
                                    self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Next => {
                                        self::ml_string::Text::from_units(vec![])
                                    }
                                    self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin359Return(source_ml_source1_temporary_result359) => {
                                        source_ml_source1_temporary_result359.clone()
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358::MlSource1TemporaryReadJavaScriptArraySliceChildStateErrorJoin358Return(source_ml_source1_temporary_result360_2) => {
                            source_ml_source1_temporary_result360_2.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySliceState,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptArraySliceTransition {
    match state.clone() {
        self::JavaScriptArraySliceState::ArraySliceState(
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
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
            shared_constructor_field18,
            shared_constructor_field19,
            shared_constructor_field20,
            shared_constructor_field21,
            shared_constructor_field22,
            shared_constructor_field23,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            let shared_constructor_field14 = *shared_constructor_field14;
            self::JavaScriptArraySliceTransition::ArraySliceCall(
                Box::new(heap.clone()),
                Box::new({
                    let source_ml_source2_nominal_field11326 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_nominal_field11328 =
                            shared_constructor_field2.clone();
                        {
                            let source_ml_source2_nominal_field11330 =
                                shared_constructor_field3.clone();
                            {
                                let source_ml_source2_nominal_field11332 =
                                    shared_constructor_field4;
                                {
                                    let source_ml_source2_nominal_field11334 =
                                        shared_constructor_field5.clone();
                                    {
                                        let source_ml_source2_nominal_field11336 =
                                            shared_constructor_field6;
                                        {
                                            let source_ml_source2_nominal_field11338 =
                                                shared_constructor_field7;
                                            {
                                                let source_ml_source2_nominal_field11340 =
                                                    shared_constructor_field8;
                                                {
                                                    let source_ml_source2_nominal_field11342 =
                                                        shared_constructor_field9;
                                                    {
                                                        let source_ml_source2_nominal_field11344 =
                                                            shared_constructor_field10;
                                                        {
                                                            let source_ml_source2_nominal_field11346 =
                                                                shared_constructor_field11;
                                                            {
                                                                let source_ml_source2_nominal_field11348 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field11350 =
                                                                        shared_constructor_field13
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field11352 = shared_constructor_field14.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field11354 = shared_constructor_field15.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field11356 = shared_constructor_field16.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field11358 = shared_constructor_field17.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11360 = shared_constructor_field18.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11362 = shared_constructor_field19.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11364 = shared_constructor_field20.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11366 = shared_constructor_field21.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11368 = shared_constructor_field22.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11370 = shared_constructor_field23.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override11372 = self::JavaScriptArraySliceCall::ArraySliceCall(Box::new(callee.clone()), Box::new(receiver.clone()), arguments_.clone());
                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11326.clone()), Box::new(source_ml_source2_nominal_field11328.clone()), Box::new(source_ml_source2_nominal_field11330.clone()), source_ml_source2_nominal_field11332, Box::new(source_ml_source2_nominal_field11334.clone()), source_ml_source2_nominal_field11336, source_ml_source2_nominal_field11338, source_ml_source2_nominal_field11340, source_ml_source2_nominal_field11342, source_ml_source2_nominal_field11344, source_ml_source2_nominal_field11346, Box::new(source_ml_source2_nominal_field11348.clone()), Box::new(source_ml_source2_nominal_field11350.clone()), Box::new(source_ml_source2_nominal_field11352.clone()), source_ml_source2_nominal_field11354.clone(), source_ml_source2_nominal_field11356.clone(), source_ml_source2_nominal_field11358.clone(), source_ml_source2_nominal_field11360.clone(), source_ml_source2_nominal_field11362.clone(), source_ml_source2_nominal_field11364.clone(), source_ml_source2_nominal_field11366.clone(), source_ml_source2_nominal_field11368.clone(), Some((Some(source_ml_source2_nominal_override11372.clone()),)))
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
                                        }
                                    }
                                }
                            }
                        }
                    }
                }),
                Box::new(callee.clone()),
                Box::new(receiver.clone()),
                arguments_.clone(),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocation_request(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySliceState,
) -> self::JavaScriptArraySliceTransition {
    match state.clone() {
        self::JavaScriptArraySliceState::ArraySliceState(
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field2,
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
        ) => match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (shared_constructor_field2 > (4294967295f64)) {
                    self::JavaScriptArraySliceTransition::ArraySliceError(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108,
                            101, 110, 103, 116, 104,
                        ]),
                    )
                } else {
                    {
                        let properties = 0f64;
                        {
                            let source_ml_source1_temporary_values361 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source1_temporary_index361 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let properties_2 = self::source_ml_source1_temporary_allocation_request_loop362(properties, source_ml_source1_temporary_values361.clone(), source_ml_source1_temporary_index361.clone());
                                    if ((({
                                        let ml_reference_array759 =
                                            shared_constructor_field1.clone();
                                        let ml_reference_projection759 =
                                            ml_reference_array759.borrow().len() as f64;
                                        ml_reference_projection759
                                    }) >= (4096f64))
                                        || (properties_2 >= (16384f64)))
                                    {
                                        self::JavaScriptArraySliceTransition::ArraySliceResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                    } else {
                                        self::JavaScriptArraySliceTransition::ArraySliceAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new((match state.clone() {
                                                self::JavaScriptArraySliceState::ArraySliceState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value11378, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value11378.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)), shared_constructor_field2)
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
