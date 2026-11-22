// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConcatenationCall;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArraySliceCall;
use super::JavaScriptArraySliceState;
use super::JavaScriptArrayTransformCall;
use super::JavaScriptArrayTransformState;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSpeciesConstructionCall;
use super::JavaScriptSpeciesConstructionState;
use super::ml_string;
use super::read_java_script_selected_call_frame_error;
use super::read_java_script_selected_parent_special_call_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_selected_parent_species_call_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    parent: self::JavaScriptCallFrame,
    fn_: self::JavaScriptCallFunction,
    frame: self::JavaScriptCallFrame,
) -> self::ml_string::Text {
    {
        let pending = match parent.clone() {
            self::JavaScriptCallFrame::CallFrame(
                _,
                _,
                _,
                _,
                _,
                source_ml_source2_data_property_value25256,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value25256 =
                    *source_ml_source2_data_property_value25256;
                source_ml_source2_data_property_value25256.clone()
            }
        };
        match pending.clone() {
            self::JavaScriptCallPending::WaitArrayConcatenation(
                _,
                _,
                state,
                pending_construction,
            ) => {
                let state = *state;
                {
                    let construction = ((pending_construction
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,)))
                    .0)
                        .clone();
                    if !(match (construction.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let call = match construction.clone().expect("a proven present value") {
                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25263,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value25263
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptSpeciesConstructionCall>,)
                                    }))
                                .0)
                                    .clone(),
                            };
                            if match (call.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97,
                                    116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101,
                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                    99, 97, 108, 108,
                                ])
                            } else {
                                self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call.clone().expect("a proven present value") {
                                    self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(source_ml_source2_data_property_value25264, _, _) => {
                                        let source_ml_source2_data_property_value25264 = *source_ml_source2_data_property_value25264;
                                        source_ml_source2_data_property_value25264.clone()
                                    }
                                }, match call.clone().expect("a proven present value") {
                                    self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, source_ml_source2_data_property_value25265, _) => {
                                        let source_ml_source2_data_property_value25265 = *source_ml_source2_data_property_value25265;
                                        source_ml_source2_data_property_value25265.clone()
                                    }
                                }, match call.clone().expect("a proven present value") {
                                    self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, _, source_ml_source2_data_property_value25266) => {
                                        source_ml_source2_data_property_value25266.clone()
                                    }
                                })
                            }
                        }
                    } else {
                        {
                            let call_2 = match state.clone() {
                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value25267) => {
                                    ((source_ml_source2_data_property_value25267.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,))).0).clone()
                                }
                            };
                            if match (call_2.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97,
                                    116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101,
                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                    99, 97, 108, 108,
                                ])
                            } else {
                                self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call_2.clone().expect("a proven present value") {
                                    self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(source_ml_source2_data_property_value25268, _, _) => {
                                        let source_ml_source2_data_property_value25268 = *source_ml_source2_data_property_value25268;
                                        source_ml_source2_data_property_value25268.clone()
                                    }
                                }, match call_2.clone().expect("a proven present value") {
                                    self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, source_ml_source2_data_property_value25269, _) => {
                                        let source_ml_source2_data_property_value25269 = *source_ml_source2_data_property_value25269;
                                        source_ml_source2_data_property_value25269.clone()
                                    }
                                }, match call_2.clone().expect("a proven present value") {
                                    self::JavaScriptArrayConcatenationCall::ArrayConcatenationCall(_, _, source_ml_source2_data_property_value25270) => {
                                        source_ml_source2_data_property_value25270.clone()
                                    }
                                })
                            }
                        }
                    }
                }
            }
            self::JavaScriptCallPending::WaitArraySlice(_, _, state_2, pending_construction_2) => {
                let state_2 = *state_2;
                {
                    let construction_2 = ((pending_construction_2
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,)))
                    .0)
                        .clone();
                    if !(match (construction_2.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let call_3 = match construction_2
                                .clone()
                                .expect("a proven present value")
                            {
                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25271,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value25271
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptSpeciesConstructionCall>,)
                                    }))
                                .0)
                                    .clone(),
                            };
                            if match (call_3.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97,
                                    116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101,
                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                    99, 97, 108, 108,
                                ])
                            } else {
                                self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call_3.clone().expect("a proven present value") {
                                    self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(source_ml_source2_data_property_value25272, _, _) => {
                                        let source_ml_source2_data_property_value25272 = *source_ml_source2_data_property_value25272;
                                        source_ml_source2_data_property_value25272.clone()
                                    }
                                }, match call_3.clone().expect("a proven present value") {
                                    self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, source_ml_source2_data_property_value25273, _) => {
                                        let source_ml_source2_data_property_value25273 = *source_ml_source2_data_property_value25273;
                                        source_ml_source2_data_property_value25273.clone()
                                    }
                                }, match call_3.clone().expect("a proven present value") {
                                    self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, _, source_ml_source2_data_property_value25274) => {
                                        source_ml_source2_data_property_value25274.clone()
                                    }
                                })
                            }
                        }
                    } else {
                        {
                            let call_4 = match state_2.clone() {
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
                                    _,
                                    source_ml_source2_data_property_value25275,
                                ) => ((source_ml_source2_data_property_value25275
                                    .clone()
                                    .unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,)))
                                .0)
                                    .clone(),
                            };
                            if match (call_4.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97,
                                    116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101,
                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                    99, 97, 108, 108,
                                ])
                            } else {
                                self::read_java_script_selected_call_frame_error(
                                    fn_.clone(),
                                    frame.clone(),
                                    match call_4.clone().expect("a proven present value") {
                                        self::JavaScriptArraySliceCall::ArraySliceCall(
                                            source_ml_source2_data_property_value25276,
                                            _,
                                            _,
                                        ) => {
                                            let source_ml_source2_data_property_value25276 =
                                                *source_ml_source2_data_property_value25276;
                                            source_ml_source2_data_property_value25276.clone()
                                        }
                                    },
                                    match call_4.clone().expect("a proven present value") {
                                        self::JavaScriptArraySliceCall::ArraySliceCall(
                                            _,
                                            source_ml_source2_data_property_value25277,
                                            _,
                                        ) => {
                                            let source_ml_source2_data_property_value25277 =
                                                *source_ml_source2_data_property_value25277;
                                            source_ml_source2_data_property_value25277.clone()
                                        }
                                    },
                                    match call_4.clone().expect("a proven present value") {
                                        self::JavaScriptArraySliceCall::ArraySliceCall(
                                            _,
                                            _,
                                            source_ml_source2_data_property_value25278,
                                        ) => source_ml_source2_data_property_value25278.clone(),
                                    },
                                )
                            }
                        }
                    }
                }
            }
            self::JavaScriptCallPending::WaitArrayTransform(
                _,
                _,
                state_3,
                pending_construction_3,
            ) => {
                let state_3 = *state_3;
                {
                    let construction_3 = ((pending_construction_3
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,)))
                    .0)
                        .clone();
                    if !(match (construction_3.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let call_5 = match construction_3
                                .clone()
                                .expect("a proven present value")
                            {
                                self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25279,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value25279
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptSpeciesConstructionCall>,)
                                    }))
                                .0)
                                    .clone(),
                            };
                            if match (call_5.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97,
                                    116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101,
                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                    99, 97, 108, 108,
                                ])
                            } else {
                                self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call_5.clone().expect("a proven present value") {
                                    self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(source_ml_source2_data_property_value25280, _, _) => {
                                        let source_ml_source2_data_property_value25280 = *source_ml_source2_data_property_value25280;
                                        source_ml_source2_data_property_value25280.clone()
                                    }
                                }, match call_5.clone().expect("a proven present value") {
                                    self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, source_ml_source2_data_property_value25281, _) => {
                                        let source_ml_source2_data_property_value25281 = *source_ml_source2_data_property_value25281;
                                        source_ml_source2_data_property_value25281.clone()
                                    }
                                }, match call_5.clone().expect("a proven present value") {
                                    self::JavaScriptSpeciesConstructionCall::SpeciesConstructionCall(_, _, source_ml_source2_data_property_value25282) => {
                                        source_ml_source2_data_property_value25282.clone()
                                    }
                                })
                            }
                        }
                    } else {
                        {
                            let call_6 = match state_3.clone() {
                                self::JavaScriptArrayTransformState::ArrayTransformState(
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
                                    source_ml_source2_data_property_value25283,
                                ) => ((source_ml_source2_data_property_value25283
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptArrayTransformCall>,)
                                    }))
                                .0)
                                    .clone(),
                            };
                            if match (call_6.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::ml_string::Text::from_units(vec![
                                    97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97,
                                    116, 105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101,
                                    116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                    99, 97, 108, 108,
                                ])
                            } else {
                                self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call_6.clone().expect("a proven present value") {
                                    self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(source_ml_source2_data_property_value25284, _, _) => {
                                        let source_ml_source2_data_property_value25284 = *source_ml_source2_data_property_value25284;
                                        source_ml_source2_data_property_value25284.clone()
                                    }
                                }, match call_6.clone().expect("a proven present value") {
                                    self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, source_ml_source2_data_property_value25285, _) => {
                                        let source_ml_source2_data_property_value25285 = *source_ml_source2_data_property_value25285;
                                        source_ml_source2_data_property_value25285.clone()
                                    }
                                }, match call_6.clone().expect("a proven present value") {
                                    self::JavaScriptArrayTransformCall::ArrayTransformSelectedCall(_, _, source_ml_source2_data_property_value25286) => {
                                        source_ml_source2_data_property_value25286.clone()
                                    }
                                })
                            }
                        }
                    }
                }
            }
            _ => self::read_java_script_selected_parent_special_call_error(
                program.clone(),
                heap.clone(),
                parent.clone(),
                fn_.clone(),
                frame.clone(),
            ),
        }
    }
}
