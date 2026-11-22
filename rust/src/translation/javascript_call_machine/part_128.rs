// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCallbackCall;
use super::JavaScriptArrayCallbackState;
use super::JavaScriptArrayIteratorCall;
use super::JavaScriptArrayIteratorState;
use super::JavaScriptArraySearchCall;
use super::JavaScriptArraySearchState;
use super::JavaScriptArraySpreadState;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionForEachCall;
use super::JavaScriptCollectionForEachState;
use super::JavaScriptFunctionBindCall;
use super::JavaScriptFunctionBindState;
use super::JavaScriptInstanceOfCall;
use super::JavaScriptInstanceOfState;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorSelectedCall;
use super::JavaScriptObjectFromEntriesCall;
use super::JavaScriptObjectFromEntriesState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPropertyDefinitionCall;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionsCall;
use super::JavaScriptPropertyDefinitionsState;
use super::ml_string;
use super::read_java_script_selected_call_frame_error;
use super::read_java_script_selected_parent_species_call_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_selected_parent_call_error(
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
                source_ml_source2_data_property_value25180,
                _,
                _,
                _,
                _,
            ) => {
                let source_ml_source2_data_property_value25180 =
                    *source_ml_source2_data_property_value25180;
                source_ml_source2_data_property_value25180.clone()
            }
        };
        match pending.clone() {
            self::JavaScriptCallPending::WaitIteratorProtocol(_, _, _, _, _, state) => {
                let state = *state;
                {
                    let call = match state.clone() {
                        self::JavaScriptIteratorProtocolState::IteratorProtocol(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value25207,
                        ) => ((source_ml_source2_data_property_value25207
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,)))
                        .0)
                            .clone(),
                    };
                    if match (call.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![
                            105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 103, 105, 111,
                            110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115,
                            32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108,
                            101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 98, 97, 99, 107,
                        ])
                    } else {
                        self::read_java_script_selected_call_frame_error(
                            fn_.clone(),
                            frame.clone(),
                            match call.clone().expect("a proven present value") {
                                self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
                                    source_ml_source2_data_property_value25208,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25208 =
                                        *source_ml_source2_data_property_value25208;
                                    source_ml_source2_data_property_value25208.clone()
                                }
                            },
                            match call.clone().expect("a proven present value") {
                                self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
                                    _,
                                    source_ml_source2_data_property_value25209,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25209 =
                                        *source_ml_source2_data_property_value25209;
                                    source_ml_source2_data_property_value25209.clone()
                                }
                            },
                            match call.clone().expect("a proven present value") {
                                self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25210,
                                ) => source_ml_source2_data_property_value25210.clone(),
                            },
                        )
                    }
                }
            }
            self::JavaScriptCallPending::WaitArraySpread(_, _, state_2) => {
                let state_2 = *state_2;
                {
                    let call_2 = {
                        let source_ml_source2_optional_chain_receiver25212 =
                            (match state_2.clone() {
                                self::JavaScriptArraySpreadState::ArraySpreadState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25211,
                                    _,
                                ) => ((source_ml_source2_data_property_value25211
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptIteratorProtocolState>,)
                                    }))
                                .0)
                                    .clone(),
                            },);
                        match if match (source_ml_source2_optional_chain_receiver25212.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            Some({
                                let source_ml_source1_temporary_optional_chain1117 =
                                    ((source_ml_source2_optional_chain_receiver25212.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_temporary_optional_chain1117.clone() {
                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value25213,
                                    ) => ((source_ml_source2_data_property_value25213
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptIteratorSelectedCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                }
                            })
                        } else {
                            None::<self::JavaScriptIteratorSelectedCall>
                        } {
                            Some(ml_nested) => match ml_nested {
                                Some(ml_present) => Some(ml_present),
                                None => None,
                            },
                            None => None,
                        }
                    };
                    if match (call_2.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 115, 112, 114, 101, 97, 100, 32, 99, 104,
                            105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                            32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101,
                            100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108,
                            98, 97, 99, 107,
                        ])
                    } else {
                        self::read_java_script_selected_call_frame_error(
                            fn_.clone(),
                            frame.clone(),
                            match call_2.clone().expect("a proven present value") {
                                self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
                                    source_ml_source2_data_property_value25214,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25214 =
                                        *source_ml_source2_data_property_value25214;
                                    source_ml_source2_data_property_value25214.clone()
                                }
                            },
                            match call_2.clone().expect("a proven present value") {
                                self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
                                    _,
                                    source_ml_source2_data_property_value25215,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25215 =
                                        *source_ml_source2_data_property_value25215;
                                    source_ml_source2_data_property_value25215.clone()
                                }
                            },
                            match call_2.clone().expect("a proven present value") {
                                self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25216,
                                ) => source_ml_source2_data_property_value25216.clone(),
                            },
                        )
                    }
                }
            }
            self::JavaScriptCallPending::WaitArrayIteratorNext(_, _, state_3) => {
                let state_3 = *state_3;
                {
                    let call_3 = match state_3.clone() {
                        self::JavaScriptArrayIteratorState::ArrayIteratorState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value25217,
                        ) => ((source_ml_source2_data_property_value25217
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,)))
                        .0)
                            .clone(),
                    };
                    if match (call_3.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97, 116,
                            105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105,
                            110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                            101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108,
                        ])
                    } else {
                        self::read_java_script_selected_call_frame_error(
                            fn_.clone(),
                            frame.clone(),
                            match call_3.clone().expect("a proven present value") {
                                self::JavaScriptArrayIteratorCall::ArrayIteratorCall(
                                    source_ml_source2_data_property_value25218,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25218 =
                                        *source_ml_source2_data_property_value25218;
                                    source_ml_source2_data_property_value25218.clone()
                                }
                            },
                            match call_3.clone().expect("a proven present value") {
                                self::JavaScriptArrayIteratorCall::ArrayIteratorCall(
                                    _,
                                    source_ml_source2_data_property_value25219,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25219 =
                                        *source_ml_source2_data_property_value25219;
                                    source_ml_source2_data_property_value25219.clone()
                                }
                            },
                            match call_3.clone().expect("a proven present value") {
                                self::JavaScriptArrayIteratorCall::ArrayIteratorCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25220,
                                ) => source_ml_source2_data_property_value25220.clone(),
                            },
                        )
                    }
                }
            }
            self::JavaScriptCallPending::WaitArraySearch(_, _, state_4) => {
                let state_4 = *state_4;
                {
                    let call_4 = match state_4.clone() {
                        self::JavaScriptArraySearchState::ArraySearchState(
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
                            source_ml_source2_data_property_value25221,
                        ) => ((source_ml_source2_data_property_value25221
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,)))
                        .0)
                            .clone(),
                    };
                    if match (call_4.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97, 116,
                            105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105,
                            110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                            101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108,
                        ])
                    } else {
                        self::read_java_script_selected_call_frame_error(
                            fn_.clone(),
                            frame.clone(),
                            match call_4.clone().expect("a proven present value") {
                                self::JavaScriptArraySearchCall::ArraySearchSelectedCall(
                                    source_ml_source2_data_property_value25222,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25222 =
                                        *source_ml_source2_data_property_value25222;
                                    source_ml_source2_data_property_value25222.clone()
                                }
                            },
                            match call_4.clone().expect("a proven present value") {
                                self::JavaScriptArraySearchCall::ArraySearchSelectedCall(
                                    _,
                                    source_ml_source2_data_property_value25223,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25223 =
                                        *source_ml_source2_data_property_value25223;
                                    source_ml_source2_data_property_value25223.clone()
                                }
                            },
                            match call_4.clone().expect("a proven present value") {
                                self::JavaScriptArraySearchCall::ArraySearchSelectedCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25224,
                                ) => source_ml_source2_data_property_value25224.clone(),
                            },
                        )
                    }
                }
            }
            self::JavaScriptCallPending::WaitCollectionForEach(_, _, state_5) => {
                let state_5 = *state_5;
                {
                    let call_5 = match state_5.clone() {
                        self::JavaScriptCollectionForEachState::CollectionForEachState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value25225,
                        ) => ((source_ml_source2_data_property_value25225
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptCollectionForEachCall>,)))
                        .0)
                            .clone(),
                    };
                    if match (call_5.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97, 116,
                            105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105,
                            110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                            101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108,
                        ])
                    } else {
                        self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call_5.clone().expect("a proven present value") {
                            self::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(source_ml_source2_data_property_value25226, _, _) => {
                                let source_ml_source2_data_property_value25226 = *source_ml_source2_data_property_value25226;
                                source_ml_source2_data_property_value25226.clone()
                            }
                        }, match call_5.clone().expect("a proven present value") {
                            self::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(_, source_ml_source2_data_property_value25227, _) => {
                                let source_ml_source2_data_property_value25227 = *source_ml_source2_data_property_value25227;
                                source_ml_source2_data_property_value25227.clone()
                            }
                        }, match call_5.clone().expect("a proven present value") {
                            self::JavaScriptCollectionForEachCall::CollectionForEachSelectedCall(_, _, source_ml_source2_data_property_value25228) => {
                                source_ml_source2_data_property_value25228.clone()
                            }
                        })
                    }
                }
            }
            self::JavaScriptCallPending::WaitArrayCallback(_, _, state_6) => {
                let state_6 = *state_6;
                {
                    let call_6 = match state_6.clone() {
                        self::JavaScriptArrayCallbackState::ArrayCallbackState(
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
                            source_ml_source2_data_property_value25229,
                        ) => ((source_ml_source2_data_property_value25229
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,)))
                        .0)
                            .clone(),
                    };
                    if match (call_6.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97, 116,
                            105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105,
                            110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                            101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108,
                        ])
                    } else {
                        self::read_java_script_selected_call_frame_error(
                            fn_.clone(),
                            frame.clone(),
                            match call_6.clone().expect("a proven present value") {
                                self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(
                                    source_ml_source2_data_property_value25230,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25230 =
                                        *source_ml_source2_data_property_value25230;
                                    source_ml_source2_data_property_value25230.clone()
                                }
                            },
                            match call_6.clone().expect("a proven present value") {
                                self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(
                                    _,
                                    source_ml_source2_data_property_value25231,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25231 =
                                        *source_ml_source2_data_property_value25231;
                                    source_ml_source2_data_property_value25231.clone()
                                }
                            },
                            match call_6.clone().expect("a proven present value") {
                                self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25232,
                                ) => source_ml_source2_data_property_value25232.clone(),
                            },
                        )
                    }
                }
            }
            self::JavaScriptCallPending::WaitPropertyDefinitions(_, _, state_7) => {
                let state_7 = *state_7;
                {
                    let call_7 = match state_7.clone() {
                        self::JavaScriptPropertyDefinitionsState::PropertyDefinitions(
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
                            source_ml_source2_data_property_value25233,
                        ) => ((source_ml_source2_data_property_value25233
                            .clone()
                            .unwrap_or_else(
                                || (None::<self::JavaScriptPropertyDefinitionsCall>,),
                            ))
                        .0)
                            .clone(),
                    };
                    if match (call_7.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![
                            98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111,
                            110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114,
                            32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101,
                            100, 32, 99, 104, 105, 108, 100,
                        ])
                    } else {
                        self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call_7.clone().expect("a proven present value") {
                            self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(source_ml_source2_data_property_value25234, _, _) => {
                                let source_ml_source2_data_property_value25234 = *source_ml_source2_data_property_value25234;
                                source_ml_source2_data_property_value25234.clone()
                            }
                        }, match call_7.clone().expect("a proven present value") {
                            self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(_, source_ml_source2_data_property_value25235, _) => {
                                let source_ml_source2_data_property_value25235 = *source_ml_source2_data_property_value25235;
                                source_ml_source2_data_property_value25235.clone()
                            }
                        }, match call_7.clone().expect("a proven present value") {
                            self::JavaScriptPropertyDefinitionsCall::PropertyDefinitionsCallRecord(_, _, source_ml_source2_data_property_value25236) => {
                                source_ml_source2_data_property_value25236.clone()
                            }
                        })
                    }
                }
            }
            self::JavaScriptCallPending::WaitInstanceOf(_, _, state_8) => {
                let state_8 = *state_8;
                {
                    let call_8 = match state_8.clone() {
                        self::JavaScriptInstanceOfState::InstanceOfState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value25237,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value25237
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptInstanceOfCall>,)))
                        .0)
                            .clone(),
                    };
                    if match (call_8.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![
                            105, 110, 115, 116, 97, 110, 99, 101, 111, 102, 32, 99, 104, 105, 108,
                            100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                            99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                            99, 97, 108, 108,
                        ])
                    } else {
                        self::read_java_script_selected_call_frame_error(
                            fn_.clone(),
                            frame.clone(),
                            match call_8.clone().expect("a proven present value") {
                                self::JavaScriptInstanceOfCall::InstanceOfCall(
                                    source_ml_source2_data_property_value25238,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25238 =
                                        *source_ml_source2_data_property_value25238;
                                    source_ml_source2_data_property_value25238.clone()
                                }
                            },
                            match call_8.clone().expect("a proven present value") {
                                self::JavaScriptInstanceOfCall::InstanceOfCall(
                                    _,
                                    source_ml_source2_data_property_value25239,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25239 =
                                        *source_ml_source2_data_property_value25239;
                                    source_ml_source2_data_property_value25239.clone()
                                }
                            },
                            match call_8.clone().expect("a proven present value") {
                                self::JavaScriptInstanceOfCall::InstanceOfCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25240,
                                ) => source_ml_source2_data_property_value25240.clone(),
                            },
                        )
                    }
                }
            }
            self::JavaScriptCallPending::WaitDefinition(_, _, state_9) => {
                let state_9 = *state_9;
                {
                    let call_9 = match state_9.clone() {
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value25241,
                        ) => ((source_ml_source2_data_property_value25241
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,)))
                        .0)
                            .clone(),
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value25242,
                        ) => ((source_ml_source2_data_property_value25242
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,)))
                        .0)
                            .clone(),
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionArray(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value25243,
                        ) => ((source_ml_source2_data_property_value25243
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,)))
                        .0)
                            .clone(),
                        self::JavaScriptPropertyDefinitionState::PropertyDefinitionReady(
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value25244,
                        ) => ((source_ml_source2_data_property_value25244
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptPropertyDefinitionCall>,)))
                        .0)
                            .clone(),
                    };
                    if match (call_9.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![])
                    } else {
                        self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call_9.clone().expect("a proven present value") {
                            self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(source_ml_source2_data_property_value25245, _, _) => {
                                let source_ml_source2_data_property_value25245 = *source_ml_source2_data_property_value25245;
                                source_ml_source2_data_property_value25245.clone()
                            }
                        }, match call_9.clone().expect("a proven present value") {
                            self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(_, source_ml_source2_data_property_value25246, _) => {
                                let source_ml_source2_data_property_value25246 = *source_ml_source2_data_property_value25246;
                                source_ml_source2_data_property_value25246.clone()
                            }
                        }, match call_9.clone().expect("a proven present value") {
                            self::JavaScriptPropertyDefinitionCall::PropertyDefinitionCallRecord(_, _, source_ml_source2_data_property_value25247) => {
                                source_ml_source2_data_property_value25247.clone()
                            }
                        })
                    }
                }
            }
            self::JavaScriptCallPending::WaitFunctionBind(_, _, state_10) => {
                let state_10 = *state_10;
                {
                    let call_10 = match state_10.clone() {
                        self::JavaScriptFunctionBindState::FunctionBindState(
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value25248,
                        ) => ((source_ml_source2_data_property_value25248
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptFunctionBindCall>,)))
                        .0)
                            .clone(),
                    };
                    if match (call_10.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![
                            97, 32, 98, 105, 110, 100, 105, 110, 103, 32, 99, 104, 105, 108, 100,
                            32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                            116, 117, 97, 108, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 103,
                            101, 116, 116, 101, 114,
                        ])
                    } else {
                        self::read_java_script_selected_call_frame_error(
                            fn_.clone(),
                            frame.clone(),
                            match call_10.clone().expect("a proven present value") {
                                self::JavaScriptFunctionBindCall::FunctionBindCall(
                                    source_ml_source2_data_property_value25249,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25249 =
                                        *source_ml_source2_data_property_value25249;
                                    source_ml_source2_data_property_value25249.clone()
                                }
                            },
                            match call_10.clone().expect("a proven present value") {
                                self::JavaScriptFunctionBindCall::FunctionBindCall(
                                    _,
                                    source_ml_source2_data_property_value25250,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value25250 =
                                        *source_ml_source2_data_property_value25250;
                                    source_ml_source2_data_property_value25250.clone()
                                }
                            },
                            match call_10.clone().expect("a proven present value") {
                                self::JavaScriptFunctionBindCall::FunctionBindCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value25251,
                                ) => source_ml_source2_data_property_value25251.clone(),
                            },
                        )
                    }
                }
            }
            self::JavaScriptCallPending::WaitObjectFromEntries(_, _, state_11, _) => {
                let state_11 = *state_11;
                {
                    let call_11 = match state_11.clone() {
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
                            source_ml_source2_data_property_value25252,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value25252
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesCall>,)))
                        .0)
                            .clone(),
                    };
                    if match (call_11.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 65, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97, 116,
                            105, 111, 110, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105,
                            110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                            101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108,
                        ])
                    } else {
                        self::read_java_script_selected_call_frame_error(fn_.clone(), frame.clone(), match call_11.clone().expect("a proven present value") {
                            self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(source_ml_source2_data_property_value25253, _, _) => {
                                let source_ml_source2_data_property_value25253 = *source_ml_source2_data_property_value25253;
                                source_ml_source2_data_property_value25253.clone()
                            }
                        }, match call_11.clone().expect("a proven present value") {
                            self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(_, source_ml_source2_data_property_value25254, _) => {
                                let source_ml_source2_data_property_value25254 = *source_ml_source2_data_property_value25254;
                                source_ml_source2_data_property_value25254.clone()
                            }
                        }, match call_11.clone().expect("a proven present value") {
                            self::JavaScriptObjectFromEntriesCall::ObjectFromEntriesCallRecord(_, _, source_ml_source2_data_property_value25255) => {
                                source_ml_source2_data_property_value25255.clone()
                            }
                        })
                    }
                }
            }
            _ => self::read_java_script_selected_parent_species_call_error(
                program.clone(),
                heap.clone(),
                parent.clone(),
                fn_.clone(),
                frame.clone(),
            ),
        }
    }
}
