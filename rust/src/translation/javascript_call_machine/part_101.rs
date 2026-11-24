// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorSelectedCall;
use super::JavaScriptObjectFromEntriesCall;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptObjectFromEntriesTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004;
use super::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005;
use super::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006;
use super::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007;
use super::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008;
use super::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Result;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_object_from_entries_state_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_state_error;
use super::source_ml_source1_temporary_read_java_script_object_from_entries_entry_state_error_loop1009;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_object_from_entries_entry_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptObjectFromEntriesState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
            _,
            _,
            _,
            shared_constructor_field4,
            _,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            _,
            shared_constructor_field8,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            {
                let source_ml_source2_s20216 = if !(match (
                    match state.clone() {
                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                            source_ml_source2_data_property_value20217,
                            _,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value20217
                            .clone()
                            .unwrap_or_else(|| {
                                (None::<self::JavaScriptPrimitiveConversionState>,)
                            }))
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
                        let invalid = self::read_java_script_primitive_conversion_state_error(
                            heap.clone(),
                            (match state.clone() {
                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                                    source_ml_source2_data_property_value20218,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value20218
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptPrimitiveConversionState>,)
                                    }))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value"),
                        );
                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004Return(invalid.clone())
                        } else {
                            if ((((shared_constructor_field5 != (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 107, 101, 121]))) || ((match (match state.clone() {
                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20223, _, _, _, _) => {
                                    ((source_ml_source2_data_property_value20223.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                }
                            }).clone().expect("a proven present value") {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value20224, _) => {
                                    source_ml_source2_data_property_value20224.clone()
                                }
                            }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20225, _, _, _, _) => {
                                    ((source_ml_source2_data_property_value20225.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                }
                            }).clone().expect("a proven present value") {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value20226, _, _, _) => {
                                    let source_ml_source2_data_property_value20226 = *source_ml_source2_data_property_value20226;
                                    source_ml_source2_data_property_value20226.clone()
                                }
                            }, shared_constructor_field7.clone()))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20228, _, _, _, _) => {
                                    ((source_ml_source2_data_property_value20228.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                }
                            }).clone().expect("a proven present value") {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value20229, _, _) => {
                                    let source_ml_source2_data_property_value20229 = *source_ml_source2_data_property_value20229;
                                    source_ml_source2_data_property_value20229.clone()
                                }
                            }, shared_constructor_field4.clone()))) {
                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 110, 116, 114, 121, 32, 107, 101, 121, 32, 115, 116, 114, 105, 110, 103, 45, 104, 105, 110, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))
                            } else {
                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004Next
                            }
                        }
                    }
                } else {
                    if (shared_constructor_field5
                        == (self::ml_string::Text::from_units(vec![
                            99, 111, 110, 118, 101, 114, 116, 32, 107, 101, 121,
                        ])))
                    {
                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 115, 116, 101, 100, 32, 115, 116, 97, 116, 101]))
                    } else {
                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004Next
                    }
                };
                match source_ml_source2_s20216.clone() {
                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004Next => {
                        if (self::ml_array::includes(&{ let ml_reference_value1045 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 107, 101, 121]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 107, 101, 121, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 118, 97, 108, 117, 101]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 118, 97, 108, 117, 101, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 107, 101, 121]), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101])]))).borrow().clone(); ml_reference_value1045 }, &shared_constructor_field5.clone(), 0f64, false, |a, b| a == b) && {
                            let source_ml_source2_s20235 = shared_constructor_field6.clone();
                            match source_ml_source2_s20235.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    false
                                }
                                _ => {
                                    true
                                }
                            }
                        }) {
                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 97, 99, 99, 101, 112, 116, 115, 32, 111, 110, 108, 121, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 101, 110, 116, 114, 121, 32, 118, 97, 108, 117, 101, 115])
                        } else {
                            {
                                let source_ml_source2_s20242 = if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]))) {
                                    {
                                        let invalid_2 = self::read_java_script_ordinary_key_error(heap.clone(), shared_constructor_field8.clone());
                                        if (invalid_2 != (self::ml_string::Text::from_units(vec![]))) {
                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005Return(invalid_2.clone())
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005Next
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005Next
                                };
                                match source_ml_source2_s20242.clone() {
                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005Next => {
                                        {
                                            let source_ml_source2_s20265 = if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]))) {
                                                if (((((match (match state.clone() {
                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20268, _, _) => {
                                                        ((source_ml_source2_data_property_value20268.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match (match state.clone() {
                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20269, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value20269.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                    }
                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || match (match state.clone() {
                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20270, _, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value20270.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value20271) => {
                                                        source_ml_source2_data_property_value20271
                                                    }
                                                }) || match (match state.clone() {
                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20272, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value20272.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                    }
                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || ((match match (match state.clone() {
                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20276, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value20276.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, source_ml_source2_data_property_value20277, _, _) => {
                                                        let source_ml_source2_data_property_value20277 = *source_ml_source2_data_property_value20277;
                                                        source_ml_source2_data_property_value20277.clone()
                                                    }
                                                } {
                                                    self::JavaScriptIteratorCompletion::IteratorCompletion(source_ml_source2_data_property_value20278, _, _) => {
                                                        source_ml_source2_data_property_value20278.clone()
                                                    }
                                                }) != (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119])))) || !(self::test_java_script_semantic_same_value(match match (match state.clone() {
                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20279, _, _, _, _, _) => {
                                                        ((source_ml_source2_data_property_value20279.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value") {
                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, source_ml_source2_data_property_value20280, _, _) => {
                                                        let source_ml_source2_data_property_value20280 = *source_ml_source2_data_property_value20280;
                                                        source_ml_source2_data_property_value20280.clone()
                                                    }
                                                } {
                                                    self::JavaScriptIteratorCompletion::IteratorCompletion(_, source_ml_source2_data_property_value20281, _) => {
                                                        let source_ml_source2_data_property_value20281 = *source_ml_source2_data_property_value20281;
                                                        source_ml_source2_data_property_value20281.clone()
                                                    }
                                                }, (match state.clone() {
                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20282, _, _) => {
                                                        ((source_ml_source2_data_property_value20282.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                }).clone().expect("a proven present value")))) {
                                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 99, 108, 101, 97, 110, 117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 116, 104, 114, 111, 119, 32, 98, 101, 102, 111, 114, 101, 32, 99, 108, 111, 115, 101]))
                                                } else {
                                                    {
                                                        let invalid_3 = self::read_java_script_ordinary_value_error(heap.clone(), (match state.clone() {
                                                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20283, _, _) => {
                                                                ((source_ml_source2_data_property_value20283.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                            }
                                                        }).clone().expect("a proven present value"));
                                                        if (invalid_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006Return(invalid_3.clone())
                                                        } else {
                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006Next
                                                        }
                                                    }
                                                }
                                            } else {
                                                if !(match (match state.clone() {
                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20284, _, _) => {
                                                        ((source_ml_source2_data_property_value20284.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 104, 111, 108, 100, 115, 32, 97, 110, 32, 97, 98, 114, 117, 112, 116, 32, 118, 97, 108, 117, 101, 32, 111, 110, 108, 121, 32, 100, 117, 114, 105, 110, 103, 32, 97, 99, 116, 117, 97, 108, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 108, 101, 97, 110, 117, 112]))
                                                } else {
                                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006Next
                                                }
                                            };
                                            match source_ml_source2_s20265.clone() {
                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006Next => {
                                                    {
                                                        let source_ml_source2_s20306 = if (shared_constructor_field5 == (self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]))) {
                                                            if (((((((match (match state.clone() {
                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20309, _, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value20309.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match (match state.clone() {
                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20310, _, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value20310.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value") {
                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value20311) => {
                                                                    source_ml_source2_data_property_value20311
                                                                }
                                                            }) || !(match (match state.clone() {
                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20312, _, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value20312.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || !(match (match state.clone() {
                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20313, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value20313.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || !(match (match state.clone() {
                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20314, _, _, _, _) => {
                                                                    ((source_ml_source2_data_property_value20314.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || ((match match state.clone() {
                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20316, _) => {
                                                                    ((source_ml_source2_data_property_value20316.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                }
                                                            } { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) || {
                                                                let source_ml_source2_comparison_operand20318 = match state.clone() {
                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20317, _) => {
                                                                        ((source_ml_source2_data_property_value20317.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                    }
                                                                };
                                                                match (source_ml_source2_comparison_operand20318.clone(), self::ml_string::Text::from_units(vec![])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                            }) || ((match match state.clone() {
                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20321) => {
                                                                    ((source_ml_source2_data_property_value20321.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                }
                                                            } { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 97, 105, 108, 117, 114, 101, 115, 32, 97, 108, 108, 111, 99, 97, 116, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 69, 114, 114, 111, 114, 32, 98, 101, 102, 111, 114, 101, 32, 99, 108, 111, 115, 105, 110, 103, 32, 116, 104, 101, 32, 117, 110, 102, 105, 110, 105, 115, 104, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114]))
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007Next
                                                            }
                                                        } else {
                                                            if ({
                                                                let source_ml_source2_comparison_operand20323 = match state.clone() {
                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20322, _) => {
                                                                        ((source_ml_source2_data_property_value20322.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_comparison_operand20324 = None::<()>;
                                                                    !(match (source_ml_source2_comparison_operand20323.clone(), source_ml_source2_comparison_operand20324.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                }
                                                            } || {
                                                                let source_ml_source2_comparison_operand20326 = match state.clone() {
                                                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20325) => {
                                                                        ((source_ml_source2_data_property_value20325.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_comparison_operand20327 = None::<()>;
                                                                    !(match (source_ml_source2_comparison_operand20326.clone(), source_ml_source2_comparison_operand20327.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                                                                }
                                                            }) {
                                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 114, 114, 111, 114, 45, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 100, 101, 116, 97, 105, 108, 115, 32, 111, 110, 108, 121, 32, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 112, 104, 97, 115, 101]))
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007Next
                                                            }
                                                        };
                                                        match source_ml_source2_s20306.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007Next => {
                                                                {
                                                                    let source_ml_source2_s20373 = if !(match (match state.clone() {
                                                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20374, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value20374.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                        {
                                                                            let call = (match state.clone() {
                                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20375, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value20375.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value");
                                                                            match call.clone() {
                                                                                self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                                    let shared_constructor_field1 = *shared_constructor_field1;
                                                                                    let shared_constructor_field2 = *shared_constructor_field2;
                                                                                    if ((((match call.clone() {
                                                                                        self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(_, _, _) => {
                                                                                            false
                                                                                        }
                                                                                    } || {
                                                                                        let source_ml_source2_s20380 = shared_constructor_field1.clone();
                                                                                        match source_ml_source2_s20380.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    }) || !(((match shared_constructor_field1.clone() {
                                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value20385) => {
                                                                                            Some((source_ml_source2_data_property_value20385,))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(bool,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0)) || (({ let ml_reference_array1046 = shared_constructor_field3.clone(); let ml_reference_projection1046 = ml_reference_array1046.borrow().len() as f64; ml_reference_projection1046 }) > (1f64))) || !(self::ml_array::includes(&{ let ml_reference_value1047 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]), self::ml_string::Text::from_units(vec![115, 116, 101, 112]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 107, 101, 121, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 118, 97, 108, 117, 101, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 107, 101, 121]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101])]))).borrow().clone(); ml_reference_value1047 }, &shared_constructor_field5.clone(), 0f64, false, |a, b| a == b))) {
                                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100, 32, 99, 97, 108, 108]))
                                                                                    } else {
                                                                                        {
                                                                                            let source_ml_source1_temporary_values1001 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![shared_constructor_field1.clone(), shared_constructor_field2.clone()], { let ml_reference_value1048 = shared_constructor_field3.borrow().clone(); ml_reference_value1048 })));
                                                                                            {
                                                                                                let source_ml_source1_temporary_index1001 = self::ml::Big::from_i128(0);
                                                                                                {
                                                                                                    let source_ml_source2_s20392 = self::source_ml_source1_temporary_read_java_script_object_from_entries_entry_state_error_loop1009(heap.clone(), source_ml_source1_temporary_values1001.clone(), source_ml_source1_temporary_index1001.clone());
                                                                                                    match source_ml_source2_s20392.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Result::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Done => {
                                                                                                            if (shared_constructor_field5.ends_with(&self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114])) && (!(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), shared_constructor_field6.clone())) || (({ let ml_reference_array1049 = shared_constructor_field3.clone(); let ml_reference_projection1049 = ml_reference_array1049.borrow().len() as f64; ml_reference_projection1049 }) != (0f64)))) {
                                                                                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 101, 110, 116, 114, 121, 32, 103, 101, 116, 116, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 101, 109, 112, 116, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                                                                                            } else {
                                                                                                                if !(match ({
                                                                                                                    let source_ml_source2_optional_chain_receiver20399 = (match state.clone() {
                                                                                                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20398, _, _, _, _, _) => {
                                                                                                                            ((source_ml_source2_data_property_value20398.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                                        }
                                                                                                                    },);
                                                                                                                    match if match (source_ml_source2_optional_chain_receiver20399.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                        Some({
                                                                                                                            let source_ml_source1_temporary_optional_chain1002 = ((source_ml_source2_optional_chain_receiver20399.clone()).0).clone().expect("a proven present value");
                                                                                                                            match source_ml_source1_temporary_optional_chain1002.clone() {
                                                                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20400) => {
                                                                                                                                    ((source_ml_source2_data_property_value20400.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                                                                }
                                                                                                                            }
                                                                                                                        })
                                                                                                                    } else {
                                                                                                                        None::<self::JavaScriptIteratorSelectedCall>
                                                                                                                    } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                                                    {
                                                                                                                        let nested = {
                                                                                                                            let source_ml_source2_nullish_field_receiver20402 = (match state.clone() {
                                                                                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20401, _, _, _, _, _) => {
                                                                                                                                    ((source_ml_source2_data_property_value20401.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                                                                }
                                                                                                                            },);
                                                                                                                            if match (source_ml_source2_nullish_field_receiver20402.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nullish_field_present20403 = ((source_ml_source2_nullish_field_receiver20402.clone()).0).clone().expect("a proven present value");
                                                                                                                                    match source_ml_source2_nullish_field_present20403.clone() {
                                                                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20404) => {
                                                                                                                                            ((source_ml_source2_data_property_value20404.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 39, 41])).0, message_present: true }).raise()
                                                                                                                            }
                                                                                                                        };
                                                                                                                        if ((!(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), {
                                                                                                                            let source_ml_source2_nullish_field_receiver20406 = (nested.clone(),);
                                                                                                                            if match (source_ml_source2_nullish_field_receiver20406.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nullish_field_present20407 = ((source_ml_source2_nullish_field_receiver20406.clone()).0).clone().expect("a proven present value");
                                                                                                                                    match source_ml_source2_nullish_field_present20407.clone() {
                                                                                                                                        self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(source_ml_source2_data_property_value20408, _, _) => {
                                                                                                                                            let source_ml_source2_data_property_value20408 = *source_ml_source2_data_property_value20408;
                                                                                                                                            source_ml_source2_data_property_value20408.clone()
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 101, 101, 39, 41])).0, message_present: true }).raise()
                                                                                                                            }
                                                                                                                        })) || !(self::test_java_script_semantic_same_value(shared_constructor_field2.clone(), {
                                                                                                                            let source_ml_source2_nullish_field_receiver20410 = (nested.clone(),);
                                                                                                                            if match (source_ml_source2_nullish_field_receiver20410.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nullish_field_present20411 = ((source_ml_source2_nullish_field_receiver20410.clone()).0).clone().expect("a proven present value");
                                                                                                                                    match source_ml_source2_nullish_field_present20411.clone() {
                                                                                                                                        self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(_, source_ml_source2_data_property_value20412, _) => {
                                                                                                                                            let source_ml_source2_data_property_value20412 = *source_ml_source2_data_property_value20412;
                                                                                                                                            source_ml_source2_data_property_value20412.clone()
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 99, 101, 105, 118, 101, 114, 39, 41])).0, message_present: true }).raise()
                                                                                                                            }
                                                                                                                        }))) || (({ let ml_reference_array1050 = shared_constructor_field3.clone(); let ml_reference_projection1050 = ml_reference_array1050.borrow().len() as f64; ml_reference_projection1050 }) != (0f64))) {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100]))
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Next
                                                                                                                        }
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Next
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Result::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorLoop1009Return(source_ml_source1_temporary_result1009) => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Return(source_ml_source1_temporary_result1009.clone())
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
                                                                        if (shared_constructor_field5.ends_with(&self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114])) || !(match ({
                                                                            let source_ml_source2_optional_chain_receiver20417 = (match state.clone() {
                                                                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value20416, _, _, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value20416.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,))).0).clone()
                                                                                }
                                                                            },);
                                                                            match if match (source_ml_source2_optional_chain_receiver20417.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                Some({
                                                                                    let source_ml_source1_temporary_optional_chain1003 = ((source_ml_source2_optional_chain_receiver20417.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_temporary_optional_chain1003.clone() {
                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value20418) => {
                                                                                            ((source_ml_source2_data_property_value20418.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                        }
                                                                                    }
                                                                                })
                                                                            } else {
                                                                                None::<self::JavaScriptIteratorSelectedCall>
                                                                            } { Some(ml_nested) => match ml_nested { Some(ml_present) => Some(ml_present), None => None }, None => None }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100, 32, 99, 97, 110, 110, 111, 116, 32, 108, 111, 115, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108]))
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Next
                                                                        }
                                                                    };
                                                                    match source_ml_source2_s20373.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Next => {
                                                                            self::ml_string::Text::from_units(vec![])
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1008Return(source_ml_source1_temporary_result1008) => {
                                                                            source_ml_source1_temporary_result1008.clone()
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1007Return(source_ml_source1_temporary_result1009_2) => {
                                                                source_ml_source1_temporary_result1009_2.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1006Return(source_ml_source1_temporary_result1009_3) => {
                                                    source_ml_source1_temporary_result1009_3.clone()
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1005Return(source_ml_source1_temporary_result1009_4) => {
                                        source_ml_source1_temporary_result1009_4.clone()
                                    }
                                }
                            }
                        }
                    }
                    self::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004::MlSource1TemporaryReadJavaScriptObjectFromEntriesEntryStateErrorJoin1004Return(source_ml_source1_temporary_result1009_5) => {
                        source_ml_source1_temporary_result1009_5.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_object_from_entries(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    prototype: self::JavaScriptSemanticValue,
    iterator_key: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptObjectFromEntriesTransition {
    {
        let state = {
            let source_ml_source2_constructor_field20423 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source2_constructor_field20425 =
                    self::JavaScriptSemanticValue::Undefined;
                {
                    let source_ml_source2_constructor_field20426 =
                        self::JavaScriptSemanticValue::Undefined;
                    {
                        let source_ml_source2_constructor_field20427 =
                            self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source2_constructor_field20428 =
                                self::JavaScriptSemanticValue::Undefined;
                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
                                Box::new(input.clone()),
                                Box::new(prototype.clone()),
                                Box::new(iterator_key.clone()),
                                Box::new(exotic_key.clone()),
                                Box::new(source_ml_source2_constructor_field20423.clone()),
                                self::ml_string::Text::from_units(vec![
                                    97, 108, 108, 111, 99, 97, 116, 101,
                                ]),
                                Box::new(source_ml_source2_constructor_field20425.clone()),
                                Box::new(source_ml_source2_constructor_field20426.clone()),
                                Box::new(source_ml_source2_constructor_field20427.clone()),
                                Box::new(source_ml_source2_constructor_field20428.clone()),
                                None::<(Option<self::JavaScriptSynchronousIteratorRecord>,)>,
                                None::<(Option<self::JavaScriptIteratorProtocolState>,)>,
                                None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                                None::<(Option<self::JavaScriptObjectFromEntriesCall>,)>,
                                None::<(Option<self::JavaScriptSemanticValue>,)>,
                                None::<(Option<self::ml_string::Text>,)>,
                                None::<(Option<self::ml_string::Text>,)>,
                            )
                        }
                    }
                }
            }
        };
        {
            let error =
                self::read_java_script_object_from_entries_state_error(heap.clone(), state.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                if (match input.clone() {
                    self::JavaScriptSemanticValue::Null => true,
                    _ => false,
                } || match input.clone() {
                    self::JavaScriptSemanticValue::Undefined => true,
                    _ => false,
                }) {
                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesError(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            84, 121, 112, 101, 69, 114, 114, 111, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114,
                            105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110,
                            32, 111, 98, 106, 101, 99, 116, 45, 99, 111, 101, 114, 99, 105, 98,
                            108, 101, 32, 105, 116, 101, 114, 97, 98, 108, 101,
                        ]),
                    )
                } else {
                    self::JavaScriptObjectFromEntriesTransition::ObjectFromEntriesContinue(
                        Box::new(heap.clone()),
                        Box::new(state.clone()),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn clear_java_script_object_from_entries_child(
    state: self::JavaScriptObjectFromEntriesState,
) -> self::JavaScriptObjectFromEntriesState {
    {
        let call = match state.clone() {
            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                source_ml_source2_data_property_value20429,
                _,
                _,
                _,
            ) => ((source_ml_source2_data_property_value20429
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,)))
            .0)
                .clone(),
        };
        {
            let conversion = match state.clone() {
                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                    source_ml_source2_data_property_value20430,
                    _,
                    _,
                    _,
                    _,
                ) => ((source_ml_source2_data_property_value20430
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,)))
                .0)
                    .clone(),
            };
            {
                let iterator = match state.clone() {
                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                        source_ml_source2_data_property_value20431,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value20431
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,)))
                    .0)
                        .clone(),
                };
                {
                    let error_class = match state.clone() {
                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                            source_ml_source2_data_property_value20432,
                            _,
                        ) => ((source_ml_source2_data_property_value20432
                            .clone()
                            .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                        .0)
                            .clone(),
                    };
                    {
                        let reason = match state.clone() {
                            self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                                source_ml_source2_data_property_value20433,
                            ) => ((source_ml_source2_data_property_value20433
                                .clone()
                                .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                            .0)
                                .clone(),
                        };
                        {
                            let abrupt = match state.clone() {
                                self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(
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
                                    source_ml_source2_data_property_value20434,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value20434
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                                .0)
                                    .clone(),
                            };
                            {
                                let base = match state.clone() {
                                    self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(source_ml_source2_nominal_rest_field20435, source_ml_source2_nominal_rest_field20436, source_ml_source2_nominal_rest_field20437, source_ml_source2_nominal_rest_field20438, source_ml_source2_nominal_rest_field20439, source_ml_source2_nominal_rest_field20440, source_ml_source2_nominal_rest_field20441, source_ml_source2_nominal_rest_field20442, source_ml_source2_nominal_rest_field20443, source_ml_source2_nominal_rest_field20444, source_ml_source2_nominal_rest_field20445, _, _, _, _, _, _) => {
                                        let source_ml_source2_nominal_rest_field20435 = *source_ml_source2_nominal_rest_field20435;
                                        let source_ml_source2_nominal_rest_field20436 = *source_ml_source2_nominal_rest_field20436;
                                        let source_ml_source2_nominal_rest_field20437 = *source_ml_source2_nominal_rest_field20437;
                                        let source_ml_source2_nominal_rest_field20438 = *source_ml_source2_nominal_rest_field20438;
                                        let source_ml_source2_nominal_rest_field20439 = *source_ml_source2_nominal_rest_field20439;
                                        let source_ml_source2_nominal_rest_field20441 = *source_ml_source2_nominal_rest_field20441;
                                        let source_ml_source2_nominal_rest_field20442 = *source_ml_source2_nominal_rest_field20442;
                                        let source_ml_source2_nominal_rest_field20443 = *source_ml_source2_nominal_rest_field20443;
                                        let source_ml_source2_nominal_rest_field20444 = *source_ml_source2_nominal_rest_field20444;
                                        self::JavaScriptObjectFromEntriesState::ObjectFromEntriesState(Box::new(source_ml_source2_nominal_rest_field20435.clone()), Box::new(source_ml_source2_nominal_rest_field20436.clone()), Box::new(source_ml_source2_nominal_rest_field20437.clone()), Box::new(source_ml_source2_nominal_rest_field20438.clone()), Box::new(source_ml_source2_nominal_rest_field20439.clone()), source_ml_source2_nominal_rest_field20440.clone(), Box::new(source_ml_source2_nominal_rest_field20441.clone()), Box::new(source_ml_source2_nominal_rest_field20442.clone()), Box::new(source_ml_source2_nominal_rest_field20443.clone()), Box::new(source_ml_source2_nominal_rest_field20444.clone()), source_ml_source2_nominal_rest_field20445.clone(), None::<(Option<self::JavaScriptIteratorProtocolState>,)>, None::<(Option<self::JavaScriptPrimitiveConversionState>,)>, None::<(Option<self::JavaScriptObjectFromEntriesCall>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::ml_string::Text>,)>)
                                    }
                                };
                                base.clone()
                            }
                        }
                    }
                }
            }
        }
    }
}
