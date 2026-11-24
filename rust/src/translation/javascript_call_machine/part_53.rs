// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayConcatenationCall;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConcatenationTransition;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptArraySpeciesCall;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptObjectCopyCall;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333;
use super::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334;
use super::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_object_spread_state_error;
use super::source_ml_source1_temporary_read_java_script_array_concatenation_child_error_loop335;
use super::source_ml_source1_temporary_request_java_script_array_concatenation_allocation_loop337;
use super::test_java_script_array_concatenation_call;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_concatenation_child_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayConcatenationState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
            shared_constructor_field8,
            shared_constructor_field9,
            _,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            _,
            _,
            _,
            shared_constructor_field13,
            _,
            shared_constructor_field14,
            _,
            _,
            shared_constructor_field15,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field14 = *shared_constructor_field14;
            if ((shared_constructor_field15
                == (self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104])))
                != (!(match (
                    match state.clone() {
                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
                            source_ml_source2_data_property_value8772,
                            _,
                        ) => ((source_ml_source2_data_property_value8772
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
                    99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 102, 105, 110,
                    97, 108, 32, 83, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 111, 110,
                    108, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 115, 115,
                    105, 103, 110, 109, 101, 110, 116, 32, 99, 104, 105, 108, 100,
                ])
            } else {
                if ((shared_constructor_field15
                    == (self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104])))
                    && (shared_constructor_field11
                        != ({
                            let ml_reference_array690 = shared_constructor_field9.clone();
                            let ml_reference_projection690 =
                                ml_reference_array690.borrow().len() as f64;
                            ml_reference_projection690
                        } + 1f64)))
                {
                    self::ml_string::Text::from_units(vec![
                        99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 102, 105,
                        110, 97, 108, 32, 83, 101, 116, 32, 102, 111, 108, 108, 111, 119, 115, 32,
                        101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 112,
                        117, 116, 32, 105, 116, 101, 109,
                    ])
                } else {
                    {
                        let source_ml_source2_s8815 = if !(match (match state.clone() {
                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8816, _) => {
                                ((source_ml_source2_data_property_value8816.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                            }
                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                            {
                                let child = (match state.clone() {
                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8817, _) => {
                                        ((source_ml_source2_data_property_value8817.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value");
                                match child.clone() {
                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, _, _, _, _, _) => {
                                        let shared_constructor_field1 = *shared_constructor_field1;
                                        let shared_constructor_field2 = *shared_constructor_field2;
                                        let shared_constructor_field7 = *shared_constructor_field7;
                                        {
                                            let error = self::read_java_script_object_spread_state_error(heap.clone(), child.clone());
                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333Return(error.clone())
                                            } else {
                                                if ((((((((({
                                                    let source_ml_source2_comparison_operand8819 = match child.clone() {
                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value8818, _, _, _) => {
                                                            ((source_ml_source2_data_property_value8818.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                        }
                                                    };
                                                    !(match (source_ml_source2_comparison_operand8819.clone(), self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                } || !(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), shared_constructor_field14.clone()))) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field8.clone()))) || !(self::test_java_script_semantic_same_value((match child.clone() {
                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8825, _, _) => {
                                                        ((source_ml_source2_data_property_value8825.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field10.clone()))) || (({ let ml_reference_array691 = shared_constructor_field4.clone(); let ml_reference_projection691 = ml_reference_array691.borrow().len() as f64; ml_reference_projection691 }) != (1f64))) || !(self::test_java_script_semantic_same_value(({
                                                    let source_ml_source2_array_read_value8830 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_array_read_index8831 = 0f64;
                                                        { let ml_reference_array692 = source_ml_source2_array_read_value8830.clone(); let ml_reference_position692 = source_ml_source2_array_read_index8831; let ml_reference_projection692 = self::ml_array::optional_at(&*ml_reference_array692.borrow(), self::ml_array::number_index(ml_reference_position692)); ml_reference_projection692 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64])))))) || (({ let ml_reference_array693 = shared_constructor_field3.clone(); let ml_reference_projection693 = ml_reference_array693.borrow().len() as f64; ml_reference_projection693 }) != (0f64))) || !((((shared_constructor_field5 == (0f64)) && self::ml_array::includes(&{ let ml_reference_value694 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), self::ml_string::Text::from_units(vec![115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value694 }, &shared_constructor_field6.clone(), 0f64, false, |a, b| a == b)) || ((shared_constructor_field5 == (1f64)) && (shared_constructor_field6 == (self::ml_string::Text::from_units(vec![114, 101, 97, 100]))))))) || ((shared_constructor_field5 == (0f64)) && !(self::test_java_script_semantic_same_value(shared_constructor_field7.clone(), self::JavaScriptSemanticValue::Number(shared_constructor_field13))))) || (!(match (match child.clone() {
                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8845, _) => {
                                                        ((source_ml_source2_data_property_value8845.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                    }
                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(self::test_java_script_semantic_same_value((({ let ml_reference_value695 = (match match (match child.clone() {
                                                    self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8846, _) => {
                                                        ((source_ml_source2_data_property_value8846.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(source_ml_source2_data_property_value8847, _) => {
                                                        let source_ml_source2_data_property_value8847 = *source_ml_source2_data_property_value8847;
                                                        source_ml_source2_data_property_value8847.clone()
                                                    }
                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(source_ml_source2_data_property_value8848, _, _) => {
                                                        let source_ml_source2_data_property_value8848 = *source_ml_source2_data_property_value8848;
                                                        source_ml_source2_data_property_value8848.clone()
                                                    }
                                                    self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(source_ml_source2_data_property_value8849, _, _) => {
                                                        let source_ml_source2_data_property_value8849 = *source_ml_source2_data_property_value8849;
                                                        source_ml_source2_data_property_value8849.clone()
                                                    }
                                                } {
                                                    self::JavaScriptArrayDefinitionState::ArrayDefinitionState(_, _, source_ml_source2_data_property_value8850, _, _, _) => {
                                                        source_ml_source2_data_property_value8850.clone()
                                                    }
                                                }).borrow().clone(); ml_reference_value695 }).0).8, self::JavaScriptSemanticValue::Number(shared_constructor_field13))))) {
                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 115, 117, 108, 116, 44, 32, 108, 101, 110, 103, 116, 104, 32, 107, 101, 121, 32, 97, 110, 100, 32, 111, 117, 116, 112, 117, 116, 32, 99, 111, 117, 110, 116]))
                                                } else {
                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333Next
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333Next
                        };
                        match source_ml_source2_s8815.clone() {
                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333Next => {
                                {
                                    let source_ml_source2_s8930 = if !(match (match state.clone() {
                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8931) => {
                                            ((source_ml_source2_data_property_value8931.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                        }
                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let source_ml_source1_temporary_values328 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![match (match state.clone() {
                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8932) => {
                                                    ((source_ml_source2_data_property_value8932.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value") {
                                                self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(source_ml_source2_data_property_value8933, _, _) => {
                                                    let source_ml_source2_data_property_value8933 = *source_ml_source2_data_property_value8933;
                                                    source_ml_source2_data_property_value8933.clone()
                                                }
                                            }, match (match state.clone() {
                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8934) => {
                                                    ((source_ml_source2_data_property_value8934.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value") {
                                                self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, source_ml_source2_data_property_value8935, _) => {
                                                    let source_ml_source2_data_property_value8935 = *source_ml_source2_data_property_value8935;
                                                    source_ml_source2_data_property_value8935.clone()
                                                }
                                            }], { let ml_reference_value696 = (match (match state.clone() {
                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8936) => {
                                                    ((source_ml_source2_data_property_value8936.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value") {
                                                self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, _, source_ml_source2_data_property_value8937) => {
                                                    source_ml_source2_data_property_value8937.clone()
                                                }
                                            }).borrow().clone(); ml_reference_value696 })));
                                            {
                                                let source_ml_source1_temporary_index328 = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s8938 = self::source_ml_source1_temporary_read_java_script_array_concatenation_child_error_loop335(heap.clone(), source_ml_source1_temporary_values328.clone(), source_ml_source1_temporary_index328.clone());
                                                    match source_ml_source2_s8938.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Result::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Done => {
                                                            if ({
                                                                let source_ml_source2_s8941 = match (match state.clone() {
                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8944) => {
                                                                        ((source_ml_source2_data_property_value8944.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                                    }
                                                                }).clone().expect("a proven present value") {
                                                                    self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(source_ml_source2_data_property_value8945, _, _) => {
                                                                        let source_ml_source2_data_property_value8945 = *source_ml_source2_data_property_value8945;
                                                                        source_ml_source2_data_property_value8945.clone()
                                                                    }
                                                                };
                                                                match source_ml_source2_s8941.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                }
                                                            } || !(((match match (match state.clone() {
                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8946) => {
                                                                    ((source_ml_source2_data_property_value8946.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value") {
                                                                self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(source_ml_source2_data_property_value8947, _, _) => {
                                                                    let source_ml_source2_data_property_value8947 = *source_ml_source2_data_property_value8947;
                                                                    source_ml_source2_data_property_value8947.clone()
                                                                }
                                                            } {
                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value8948) => {
                                                                    Some((source_ml_source2_data_property_value8948,))
                                                                }
                                                                _ => {
                                                                    None::<(bool,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0)) {
                                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                                            } else {
                                                                if self::ml_array::includes(&{ let ml_reference_value697 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value697 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b) {
                                                                    if (!(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8950) => {
                                                                            ((source_ml_source2_data_property_value8950.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value") {
                                                                        self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, source_ml_source2_data_property_value8951, _) => {
                                                                            let source_ml_source2_data_property_value8951 = *source_ml_source2_data_property_value8951;
                                                                            source_ml_source2_data_property_value8951.clone()
                                                                        }
                                                                    }, shared_constructor_field12.clone())) || (({ let ml_reference_array698 = match (match state.clone() {
                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8955) => {
                                                                            ((source_ml_source2_data_property_value8955.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value") {
                                                                        self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, _, source_ml_source2_data_property_value8956) => {
                                                                            source_ml_source2_data_property_value8956.clone()
                                                                        }
                                                                    }; let ml_reference_projection698 = ml_reference_array698.borrow().len() as f64; ml_reference_projection698 }) != (0f64))) {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 103, 101, 116, 116, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 109, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Next
                                                                    }
                                                                } else {
                                                                    if (shared_constructor_field15 == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) {
                                                                        {
                                                                            let call = {
                                                                                let source_ml_source2_optional_chain_receiver8960 = (match state.clone() {
                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8959, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value8959.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                                                    }
                                                                                },);
                                                                                match if match (source_ml_source2_optional_chain_receiver8960.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    Some({
                                                                                        let source_ml_source1_temporary_optional_chain329 = ((source_ml_source2_optional_chain_receiver8960.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source1_temporary_optional_chain329.clone() {
                                                                                            self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value8961) => {
                                                                                                ((source_ml_source2_data_property_value8961.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                                                            }
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    None::<self::JavaScriptArraySpeciesCall>
                                                                                } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                                            };
                                                                            if (match (call.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_array_concatenation_call((match state.clone() {
                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8962) => {
                                                                                    ((source_ml_source2_data_property_value8962.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value"), match call.clone().expect("a proven present value") {
                                                                                self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(source_ml_source2_data_property_value8963, _, _) => {
                                                                                    let source_ml_source2_data_property_value8963 = *source_ml_source2_data_property_value8963;
                                                                                    source_ml_source2_data_property_value8963.clone()
                                                                                }
                                                                            }, match call.clone().expect("a proven present value") {
                                                                                self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(_, source_ml_source2_data_property_value8964, _) => {
                                                                                    let source_ml_source2_data_property_value8964 = *source_ml_source2_data_property_value8964;
                                                                                    source_ml_source2_data_property_value8964.clone()
                                                                                }
                                                                            }, match call.clone().expect("a proven present value") {
                                                                                self::JavaScriptArraySpeciesCall::ArraySpeciesSelectedCall(_, _, source_ml_source2_data_property_value8965) => {
                                                                                    source_ml_source2_data_property_value8965.clone()
                                                                                }
                                                                            }))) {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 115, 112, 101, 99, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 110, 101, 115, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114]))
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Next
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if (shared_constructor_field15 == (self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104]))) {
                                                                            {
                                                                                let call_2 = {
                                                                                    let source_ml_source2_optional_chain_receiver8969 = (match state.clone() {
                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8968, _) => {
                                                                                            ((source_ml_source2_data_property_value8968.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                                                        }
                                                                                    },);
                                                                                    match if match (source_ml_source2_optional_chain_receiver8969.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                        Some({
                                                                                            let source_ml_source1_temporary_optional_chain330 = ((source_ml_source2_optional_chain_receiver8969.clone()).0).clone().expect("a proven present value");
                                                                                            match source_ml_source1_temporary_optional_chain330.clone() {
                                                                                                self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8970) => {
                                                                                                    ((source_ml_source2_data_property_value8970.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                                                                                }
                                                                                            }
                                                                                        })
                                                                                    } else {
                                                                                        None::<self::JavaScriptObjectCopyCall>
                                                                                    } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                                                };
                                                                                if (match (call_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_array_concatenation_call((match state.clone() {
                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8971) => {
                                                                                        ((source_ml_source2_data_property_value8971.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value"), match call_2.clone().expect("a proven present value") {
                                                                                    self::JavaScriptObjectCopyCall::ObjectCopyCall(source_ml_source2_data_property_value8972, _, _) => {
                                                                                        let source_ml_source2_data_property_value8972 = *source_ml_source2_data_property_value8972;
                                                                                        source_ml_source2_data_property_value8972.clone()
                                                                                    }
                                                                                }, match call_2.clone().expect("a proven present value") {
                                                                                    self::JavaScriptObjectCopyCall::ObjectCopyCall(_, source_ml_source2_data_property_value8973, _) => {
                                                                                        let source_ml_source2_data_property_value8973 = *source_ml_source2_data_property_value8973;
                                                                                        source_ml_source2_data_property_value8973.clone()
                                                                                    }
                                                                                }, match call_2.clone().expect("a proven present value") {
                                                                                    self::JavaScriptObjectCopyCall::ObjectCopyCall(_, _, source_ml_source2_data_property_value8974) => {
                                                                                        source_ml_source2_data_property_value8974.clone()
                                                                                    }
                                                                                }))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 110, 101, 115, 116, 101, 100, 32, 115, 101, 116, 116, 101, 114]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Next
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if ((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && !(match (match state.clone() {
                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8977, _, _, _, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value8977.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                                {
                                                                                    let child_2 = (match state.clone() {
                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8978, _, _, _, _, _, _) => {
                                                                                            ((source_ml_source2_data_property_value8978.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value");
                                                                                    match child_2.clone() {
                                                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1_2, _, _, shared_constructor_field2_2) => {
                                                                                            let shared_constructor_field1_2 = *shared_constructor_field1_2;
                                                                                            {
                                                                                                let exotic = (shared_constructor_field2_2 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108])));
                                                                                                if (((!(self::ml_array::includes(&{ let ml_reference_value699 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])]))).borrow().clone(); ml_reference_value699 }, &shared_constructor_field2_2.clone(), 0f64, false, |a, b| a == b)) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8982) => {
                                                                                                        ((source_ml_source2_data_property_value8982.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().expect("a proven present value") {
                                                                                                    self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, source_ml_source2_data_property_value8983, _) => {
                                                                                                        let source_ml_source2_data_property_value8983 = *source_ml_source2_data_property_value8983;
                                                                                                        source_ml_source2_data_property_value8983.clone()
                                                                                                    }
                                                                                                }, shared_constructor_field1_2.clone()))) || (({ let ml_reference_array700 = match (match state.clone() {
                                                                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8987) => {
                                                                                                        ((source_ml_source2_data_property_value8987.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                                                                    }
                                                                                                }).clone().expect("a proven present value") {
                                                                                                    self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, _, source_ml_source2_data_property_value8988) => {
                                                                                                        source_ml_source2_data_property_value8988.clone()
                                                                                                    }
                                                                                                }; let ml_reference_projection700 = ml_reference_array700.borrow().len() as f64; ml_reference_projection700 }) != (if exotic {
                                                                                                    1f64
                                                                                                } else {
                                                                                                    0f64
                                                                                                }))) || (exotic && !(self::test_java_script_semantic_same_value(({
                                                                                                    let source_ml_source2_array_read_value8991 = match (match state.clone() {
                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8989) => {
                                                                                                            ((source_ml_source2_data_property_value8989.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().expect("a proven present value") {
                                                                                                        self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, _, source_ml_source2_data_property_value8990) => {
                                                                                                            source_ml_source2_data_property_value8990.clone()
                                                                                                        }
                                                                                                    };
                                                                                                    {
                                                                                                        let source_ml_source2_array_read_index8992 = 0f64;
                                                                                                        { let ml_reference_array701 = source_ml_source2_array_read_value8991.clone(); let ml_reference_position701 = source_ml_source2_array_read_index8992; let ml_reference_projection701 = self::ml_array::optional_at(&*ml_reference_array701.borrow(), self::ml_array::number_index(ml_reference_position701)); ml_reference_projection701 }
                                                                                                    }
                                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![110f64, 117f64, 109f64, 98f64, 101f64, 114f64]))))))) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 115, 99, 97, 108, 97, 114, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Next
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 99, 104, 105, 108, 100, 32, 99, 97, 108, 108, 115, 32, 111, 110, 108, 121, 32, 105, 110, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 112, 104, 97, 115, 101]))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Result::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorLoop335Return(source_ml_source1_temporary_result335) => {
                                                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Return(source_ml_source1_temporary_result335.clone())
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (((self::ml_array::includes(&{ let ml_reference_value702 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value702 }, &shared_constructor_field15.clone(), 0f64, false, |a, b| a == b) || (((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) && !(match (match state.clone() {
                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8996, _, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value8996.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) && self::ml_array::includes(&{ let ml_reference_value703 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])]))).borrow().clone(); ml_reference_value703 }, &match (match state.clone() {
                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value8997, _, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value8997.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value") {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value8998) => {
                                                source_ml_source2_data_property_value8998.clone()
                                            }
                                        }, 0f64, false, |a, b| a == b))) || ((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]))) && !(match ({
                                            let source_ml_source2_optional_chain_receiver9002 = (match state.clone() {
                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9001, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value9001.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesState>,))).0).clone()
                                                }
                                            },);
                                            match if match (source_ml_source2_optional_chain_receiver9002.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_source1_temporary_optional_chain331 = ((source_ml_source2_optional_chain_receiver9002.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_temporary_optional_chain331.clone() {
                                                        self::JavaScriptArraySpeciesState::ArraySpeciesState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value9003) => {
                                                            ((source_ml_source2_data_property_value9003.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesCall>,))).0).clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::JavaScriptArraySpeciesCall>
                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) || ((shared_constructor_field15 == (self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104]))) && !(match ({
                                            let source_ml_source2_optional_chain_receiver9007 = (match state.clone() {
                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9006, _) => {
                                                    ((source_ml_source2_data_property_value9006.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectSpreadState>,))).0).clone()
                                                }
                                            },);
                                            match if match (source_ml_source2_optional_chain_receiver9007.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                Some({
                                                    let source_ml_source1_temporary_optional_chain332 = ((source_ml_source2_optional_chain_receiver9007.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_temporary_optional_chain332.clone() {
                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9008) => {
                                                            ((source_ml_source2_data_property_value9008.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectCopyCall>,))).0).clone()
                                                        }
                                                    }
                                                })
                                            } else {
                                                None::<self::JavaScriptObjectCopyCall>
                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Return(self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100]))
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Next
                                        }
                                    };
                                    match source_ml_source2_s8930.clone() {
                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Next => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin334Return(source_ml_source1_temporary_result334) => {
                                            source_ml_source1_temporary_result334.clone()
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333::MlSource1TemporaryReadJavaScriptArrayConcatenationChildErrorJoin333Return(source_ml_source1_temporary_result335_2) => {
                                source_ml_source1_temporary_result335_2.clone()
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
pub fn request_java_script_array_concatenation_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayConcatenationState,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptArrayConcatenationTransition {
    match state.clone() {
        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
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
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
            let shared_constructor_field14 = *shared_constructor_field14;
            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationCall(
                Box::new(heap.clone()),
                Box::new({
                    let source_ml_source2_nominal_field9010 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_nominal_field9012 = shared_constructor_field2.clone();
                        {
                            let source_ml_source2_nominal_field9014 =
                                shared_constructor_field3.clone();
                            {
                                let source_ml_source2_nominal_field9016 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field9018 =
                                        shared_constructor_field5;
                                    {
                                        let source_ml_source2_nominal_field9020 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field9022 =
                                                shared_constructor_field7;
                                            {
                                                let source_ml_source2_nominal_field9024 =
                                                    shared_constructor_field8;
                                                {
                                                    let source_ml_source2_nominal_field9026 =
                                                        shared_constructor_field9;
                                                    {
                                                        let source_ml_source2_nominal_field9028 =
                                                            shared_constructor_field10;
                                                        {
                                                            let source_ml_source2_nominal_field9030 =
                                                                shared_constructor_field11;
                                                            {
                                                                let source_ml_source2_nominal_field9032 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field9034 =
                                                                        shared_constructor_field13
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field9036 = shared_constructor_field14.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field9038 = shared_constructor_field15.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field9040 = shared_constructor_field16.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field9042 = shared_constructor_field17.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9044 = shared_constructor_field18.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9046 = shared_constructor_field19.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9048 = shared_constructor_field20.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9050 = shared_constructor_field21.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9052 = shared_constructor_field22.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override9054 = self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(Box::new(callee.clone()), Box::new(receiver.clone()), arguments_.clone());
                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9010.clone()), source_ml_source2_nominal_field9012.clone(), Box::new(source_ml_source2_nominal_field9014.clone()), Box::new(source_ml_source2_nominal_field9016.clone()), source_ml_source2_nominal_field9018, Box::new(source_ml_source2_nominal_field9020.clone()), source_ml_source2_nominal_field9022, source_ml_source2_nominal_field9024, source_ml_source2_nominal_field9026, source_ml_source2_nominal_field9028, source_ml_source2_nominal_field9030, Box::new(source_ml_source2_nominal_field9032.clone()), Box::new(source_ml_source2_nominal_field9034.clone()), Box::new(source_ml_source2_nominal_field9036.clone()), source_ml_source2_nominal_field9038.clone(), source_ml_source2_nominal_field9040.clone(), source_ml_source2_nominal_field9042.clone(), source_ml_source2_nominal_field9044.clone(), source_ml_source2_nominal_field9046.clone(), source_ml_source2_nominal_field9048.clone(), source_ml_source2_nominal_field9050.clone(), Some((Some(source_ml_source2_nominal_override9054.clone()),)))
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
pub fn request_java_script_array_concatenation_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayConcatenationState,
) -> self::JavaScriptArrayConcatenationTransition {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let properties = 0f64;
            {
                let source_ml_source1_temporary_values336 = shared_constructor_field1.clone();
                {
                    let source_ml_source1_temporary_index336 = self::ml::Big::from_i128(0);
                    {
                        let properties_2 = self::source_ml_source1_temporary_request_java_script_array_concatenation_allocation_loop337(properties, source_ml_source1_temporary_values336.clone(), source_ml_source1_temporary_index336.clone());
                        if ((({
                            let ml_reference_array704 = shared_constructor_field1.clone();
                            let ml_reference_projection704 =
                                ml_reference_array704.borrow().len() as f64;
                            ml_reference_projection704
                        }) >= (4096f64))
                            || (properties_2 >= (16384f64)))
                        {
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 115, 117, 108, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new((match state.clone() {
                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9058, _, _, _, _) => {
                                        ((source_ml_source2_data_property_value9058.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)), 0f64)
                        }
                    }
                }
            }
        }
    }
}
