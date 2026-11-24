// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorProtocolTransition;
use super::JavaScriptIteratorSelectedCall;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::convert_java_script_semantic_boolean;
use super::fail_java_script_iterator_protocol;
use super::ml_string;
use super::read_java_script_iterator_protocol_state_error;
use super::read_java_script_ordinary_value_error;
use super::request_java_script_iterator_protocol_call;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_iterator_protocol_value(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptIteratorProtocolState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptIteratorProtocolTransition {
    match state.clone() {
        self::JavaScriptIteratorProtocolState::IteratorProtocol(
            _,
            shared_constructor_field13,
            shared_constructor_field14,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field14 = *shared_constructor_field14;
            {
                let previous = match state.clone() {
                    self::JavaScriptIteratorProtocolState::IteratorProtocol(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value13531,
                    ) => ((source_ml_source2_data_property_value13531
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,)))
                    .0)
                        .clone(),
                };
                {
                    let base = match state.clone() {
                        self::JavaScriptIteratorProtocolState::IteratorProtocol(
                            source_ml_source2_nominal_rest_field13532,
                            source_ml_source2_nominal_rest_field13533,
                            source_ml_source2_nominal_rest_field13534,
                            source_ml_source2_nominal_rest_field13535,
                            source_ml_source2_nominal_rest_field13536,
                            source_ml_source2_nominal_rest_field13537,
                            source_ml_source2_nominal_rest_field13538,
                            source_ml_source2_nominal_rest_field13539,
                            _,
                        ) => {
                            let source_ml_source2_nominal_rest_field13534 =
                                *source_ml_source2_nominal_rest_field13534;
                            let source_ml_source2_nominal_rest_field13535 =
                                *source_ml_source2_nominal_rest_field13535;
                            let source_ml_source2_nominal_rest_field13536 =
                                *source_ml_source2_nominal_rest_field13536;
                            let source_ml_source2_nominal_rest_field13537 =
                                *source_ml_source2_nominal_rest_field13537;
                            let source_ml_source2_nominal_rest_field13538 =
                                *source_ml_source2_nominal_rest_field13538;
                            self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                source_ml_source2_nominal_rest_field13532.clone(),
                                source_ml_source2_nominal_rest_field13533.clone(),
                                Box::new(source_ml_source2_nominal_rest_field13534.clone()),
                                Box::new(source_ml_source2_nominal_rest_field13535.clone()),
                                Box::new(source_ml_source2_nominal_rest_field13536.clone()),
                                Box::new(source_ml_source2_nominal_rest_field13537.clone()),
                                Box::new(source_ml_source2_nominal_rest_field13538.clone()),
                                source_ml_source2_nominal_rest_field13539.clone(),
                                None::<(Option<self::JavaScriptIteratorSelectedCall>,)>,
                            )
                        }
                    };
                    match base.clone() {
                        self::JavaScriptIteratorProtocolState::IteratorProtocol(
                            shared_constructor_field4,
                            shared_constructor_field5,
                            shared_constructor_field6,
                            shared_constructor_field7,
                            shared_constructor_field8,
                            shared_constructor_field9,
                            shared_constructor_field10,
                            shared_constructor_field11,
                            shared_constructor_field12,
                        ) => {
                            let shared_constructor_field6 = *shared_constructor_field6;
                            let shared_constructor_field7 = *shared_constructor_field7;
                            let shared_constructor_field8 = *shared_constructor_field8;
                            let shared_constructor_field9 = *shared_constructor_field9;
                            let shared_constructor_field10 = *shared_constructor_field10;
                            {
                                let phase = if shared_constructor_field13.ends_with(
                                    &self::ml_string::Text::from_units(vec![
                                        32, 103, 101, 116, 116, 101, 114,
                                    ]),
                                ) {
                                    {
                                        let source_ml_source2_string_range_operand13542 =
                                            shared_constructor_field13.clone();
                                        {
                                            let source_ml_source2_string_range_operand13543 = 0f64;
                                            {
                                                let source_ml_source2_string_range_operand13544 =
                                                    (-7f64);
                                                {
                                                    let source_ml_source2_string_range_position13545 =
                                                        source_ml_source2_string_range_operand13543;
                                                    {
                                                        let source_ml_source2_string_range_position13546 = source_ml_source2_string_range_operand13544;
                                                        source_ml_source2_string_range_operand13542.slice(source_ml_source2_string_range_position13545, source_ml_source2_string_range_position13546)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    shared_constructor_field13.clone()
                                };
                                if (phase
                                    == (self::ml_string::Text::from_units(vec![
                                        109, 101, 116, 104, 111, 100,
                                    ])))
                                {
                                    self::request_java_script_iterator_protocol_call(
                                        heap.clone(),
                                        base.clone(),
                                        value.clone(),
                                        shared_constructor_field14.clone(),
                                        &(self::ml_string::Text::from_units(vec![
                                            105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108,
                                            108,
                                        ])),
                                    )
                                } else {
                                    if (phase
                                        == (self::ml_string::Text::from_units(vec![
                                            105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108,
                                            108,
                                        ])))
                                    {
                                        if match value.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => false,
                                            _ => true,
                                        } {
                                            self::fail_java_script_iterator_protocol(
                                                heap.clone(),
                                                base.clone(),
                                                &(self::ml_string::Text::from_units(vec![
                                                    105, 116, 101, 114, 97, 116, 111, 114, 32, 109,
                                                    101, 116, 104, 111, 100, 32, 114, 101, 116,
                                                    117, 114, 110, 101, 100, 32, 97, 32, 110, 111,
                                                    110, 45, 79, 98, 106, 101, 99, 116,
                                                ])),
                                            )
                                        } else {
                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(Box::new(heap.clone()), Box::new({
                                                let source_ml_source2_nominal_field13561 = shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field13563 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field13565 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field13567 = shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field13569 = shared_constructor_field8.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field13571 = shared_constructor_field9.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field13573 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field13575 = shared_constructor_field11.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field13577 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_override13580 = {
                                                                                        let source_ml_source2_nominal_spread13552 = shared_constructor_field8.clone();
                                                                                        match source_ml_source2_nominal_spread13552.clone() {
                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field13553 = shared_constructor_field1.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field13555 = shared_constructor_field2.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field13557 = shared_constructor_field3;
                                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(value.clone()), Box::new(source_ml_source2_nominal_field13555.clone()), source_ml_source2_nominal_field13557)
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    };
                                                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_nominal_field13561.clone(), self::ml_string::Text::from_units(vec![110, 101, 120, 116]), Box::new(source_ml_source2_nominal_field13565.clone()), Box::new(source_ml_source2_nominal_field13567.clone()), Box::new(source_ml_source2_nominal_override13580.clone()), Box::new(source_ml_source2_nominal_field13571.clone()), Box::new(source_ml_source2_nominal_field13573.clone()), source_ml_source2_nominal_field13575.clone(), source_ml_source2_nominal_field13577.clone())
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
                                    } else {
                                        if (phase
                                            == (self::ml_string::Text::from_units(vec![
                                                110, 101, 120, 116,
                                            ])))
                                        {
                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolAcquired(Box::new(heap.clone()), Box::new({
                                                let source_ml_source2_nominal_spread13582 = shared_constructor_field8.clone();
                                                match source_ml_source2_nominal_spread13582.clone() {
                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2) => {
                                                        let shared_constructor_field1_2 = *shared_constructor_field1_2;
                                                        let shared_constructor_field2_2 = *shared_constructor_field2_2;
                                                        {
                                                            let source_ml_source2_nominal_field13583 = shared_constructor_field1_2.clone();
                                                            {
                                                                let source_ml_source2_nominal_field13585 = shared_constructor_field2_2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field13587 = shared_constructor_field3_2;
                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(source_ml_source2_nominal_field13583.clone()), Box::new(value.clone()), source_ml_source2_nominal_field13587)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }))
                                        } else {
                                            if (phase
                                                == (self::ml_string::Text::from_units(vec![
                                                    110, 101, 120, 116, 32, 99, 97, 108, 108,
                                                ])))
                                            {
                                                if match value.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => true,
                                                } {
                                                    self::fail_java_script_iterator_protocol(
                                                        heap.clone(),
                                                        base.clone(),
                                                        &(self::ml_string::Text::from_units(vec![
                                                            105, 116, 101, 114, 97, 116, 111, 114,
                                                            32, 110, 101, 120, 116, 32, 114, 101,
                                                            116, 117, 114, 110, 101, 100, 32, 97,
                                                            32, 110, 111, 110, 45, 79, 98, 106,
                                                            101, 99, 116,
                                                        ])),
                                                    )
                                                } else {
                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field13593 = shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field13595 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field13597 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field13599 = shared_constructor_field7.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field13601 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field13603 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field13605 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field13607 = shared_constructor_field11.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field13609 = shared_constructor_field12.clone();
                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_nominal_field13593.clone(), self::ml_string::Text::from_units(vec![100, 111, 110, 101]), Box::new(source_ml_source2_nominal_field13597.clone()), Box::new(source_ml_source2_nominal_field13599.clone()), Box::new(source_ml_source2_nominal_field13601.clone()), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field13605.clone()), source_ml_source2_nominal_field13607.clone(), source_ml_source2_nominal_field13609.clone())
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
                                            } else {
                                                if (phase
                                                    == (self::ml_string::Text::from_units(vec![
                                                        100, 111, 110, 101,
                                                    ])))
                                                {
                                                    if self::convert_java_script_semantic_boolean(
                                                        value.clone(),
                                                    ) {
                                                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(Box::new(heap.clone()), Box::new({
                                                            let source_ml_source2_nominal_spread13614 = shared_constructor_field8.clone();
                                                            match source_ml_source2_nominal_spread13614.clone() {
                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(shared_constructor_field1_3, shared_constructor_field2_3, shared_constructor_field3_3) => {
                                                                    let shared_constructor_field1_3 = *shared_constructor_field1_3;
                                                                    let shared_constructor_field2_3 = *shared_constructor_field2_3;
                                                                    {
                                                                        let source_ml_source2_nominal_field13615 = shared_constructor_field1_3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field13617 = shared_constructor_field2_3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field13619 = shared_constructor_field3_3;
                                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(source_ml_source2_nominal_field13615.clone()), Box::new(source_ml_source2_nominal_field13617.clone()), true)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }), true, Box::new(self::JavaScriptSemanticValue::Undefined))
                                                    } else {
                                                        if {
                                                            let source_ml_source2_comparison_operand13623 = match base.clone() {
                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, source_ml_source2_data_property_value13622, _) => {
                                                                    ((source_ml_source2_data_property_value13622.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                                                }
                                                            };
                                                            match (source_ml_source2_comparison_operand13623.clone(), false) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                        } {
                                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(Box::new(heap.clone()), Box::new(shared_constructor_field8.clone()), false, Box::new(self::JavaScriptSemanticValue::Undefined))
                                                        } else {
                                                            self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(Box::new(heap.clone()), Box::new({
                                                                let source_ml_source2_nominal_field13627 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field13629 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field13631 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field13633 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field13635 = shared_constructor_field8.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field13637 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field13639 = shared_constructor_field10.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field13641 = shared_constructor_field11.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field13643 = shared_constructor_field12.clone();
                                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_nominal_field13627.clone(), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), Box::new(source_ml_source2_nominal_field13631.clone()), Box::new(source_ml_source2_nominal_field13633.clone()), Box::new(source_ml_source2_nominal_field13635.clone()), Box::new(source_ml_source2_nominal_field13637.clone()), Box::new(source_ml_source2_nominal_field13639.clone()), source_ml_source2_nominal_field13641.clone(), source_ml_source2_nominal_field13643.clone())
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
                                                } else {
                                                    if (phase
                                                        == (self::ml_string::Text::from_units(
                                                            vec![118, 97, 108, 117, 101],
                                                        )))
                                                    {
                                                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolStep(Box::new(heap.clone()), Box::new(shared_constructor_field8.clone()), false, Box::new(value.clone()))
                                                    } else {
                                                        if (phase
                                                            == (self::ml_string::Text::from_units(
                                                                vec![114, 101, 116, 117, 114, 110],
                                                            )))
                                                        {
                                                            if (match value.clone() {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            } || match value.clone() {
                                                                self::JavaScriptSemanticValue::Null => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            }) {
                                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(Box::new(heap.clone()), Box::new(shared_constructor_field8.clone()), Box::new(shared_constructor_field10.clone()))
                                                            } else {
                                                                self::request_java_script_iterator_protocol_call(heap.clone(), base.clone(), value.clone(), match shared_constructor_field8.clone() {
                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value13650, _, _) => {
                                                                        let source_ml_source2_data_property_value13650 = *source_ml_source2_data_property_value13650;
                                                                        source_ml_source2_data_property_value13650.clone()
                                                                    }
                                                                }, &(self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110, 32, 99, 97, 108, 108])))
                                                            }
                                                        } else {
                                                            if (phase == (self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110, 32, 99, 97, 108, 108]))) {
                                                                if (((match shared_constructor_field10.clone() {
                                                                    self::JavaScriptIteratorCompletion::IteratorCompletion(source_ml_source2_data_property_value13654, _, _) => {
                                                                        source_ml_source2_data_property_value13654.clone()
                                                                    }
                                                                }) == (self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]))) || match value.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                }) {
                                                                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(Box::new(heap.clone()), Box::new(shared_constructor_field8.clone()), Box::new(shared_constructor_field10.clone()))
                                                                } else {
                                                                    self::fail_java_script_iterator_protocol(heap.clone(), base.clone(), &(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 116, 117, 114, 110, 32, 114, 101, 116, 117, 114, 110, 101, 100, 32, 97, 32, 110, 111, 110, 45, 79, 98, 106, 101, 99, 116])))
                                                                }
                                                            } else {
                                                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_iterator_protocol_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptIteratorProtocolState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptIteratorProtocolTransition {
    {
        let error =
            self::read_java_script_iterator_protocol_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            if match (
                match state.clone() {
                    self::JavaScriptIteratorProtocolState::IteratorProtocol(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value13659,
                    ) => ((source_ml_source2_data_property_value13659
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108, 32, 99, 111,
                        109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114,
                        101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                        108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100,
                    ]),
                )
            } else {
                self::resume_java_script_iterator_protocol_value(
                    heap.clone(),
                    state.clone(),
                    completion.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_iterator_primitive_get(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptIteratorProtocolState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptIteratorProtocolTransition {
    match state.clone() {
        self::JavaScriptIteratorProtocolState::IteratorProtocol(
            _,
            shared_constructor_field1,
            shared_constructor_field2,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error = self::read_java_script_iterator_protocol_state_error(
                    heap.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if (((((shared_constructor_field1
                        != (self::ml_string::Text::from_units(vec![
                            109, 101, 116, 104, 111, 100,
                        ])))
                        || {
                            let source_ml_source2_s13663 = shared_constructor_field2.clone();
                            match source_ml_source2_s13663.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => true,
                                _ => false,
                            }
                        })
                        || {
                            let source_ml_source2_s13668 = shared_constructor_field2.clone();
                            match source_ml_source2_s13668.clone() {
                                self::JavaScriptSemanticValue::Null => true,
                                _ => false,
                            }
                        })
                        || {
                            let source_ml_source2_s13671 = shared_constructor_field2.clone();
                            match source_ml_source2_s13671.clone() {
                                self::JavaScriptSemanticValue::Undefined => true,
                                _ => false,
                            }
                        })
                        || !(match (
                            match state.clone() {
                                self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value13673,
                                ) => ((source_ml_source2_data_property_value13673
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptIteratorSelectedCall>,)
                                    }))
                                .0)
                                    .clone(),
                            },
                            None::<()>,
                        ) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }))
                    {
                        self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 116, 101,
                                114, 97, 116, 111, 114, 32, 71, 101, 116, 32, 114, 101, 113, 117,
                                105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 97, 99,
                                113, 117, 105, 115, 105, 116, 105, 111, 110, 32, 114, 101, 113,
                                117, 101, 115, 116,
                            ]),
                        )
                    } else {
                        self::resume_java_script_iterator_protocol_value(
                            heap.clone(),
                            state.clone(),
                            completion.clone(),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_iterator_protocol_value(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptIteratorProtocolState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptIteratorProtocolTransition {
    match state.clone() {
        self::JavaScriptIteratorProtocolState::IteratorProtocol(
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field10 = *shared_constructor_field10;
            {
                let error = self::read_java_script_ordinary_value_error(
                    heap.clone(),
                    match completion.clone() {
                        self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                            source_ml_source2_data_property_value13674,
                        ) => {
                            let source_ml_source2_data_property_value13674 =
                                *source_ml_source2_data_property_value13674;
                            source_ml_source2_data_property_value13674.clone()
                        }
                        self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                            source_ml_source2_data_property_value13675,
                        ) => {
                            let source_ml_source2_data_property_value13675 =
                                *source_ml_source2_data_property_value13675;
                            source_ml_source2_data_property_value13675.clone()
                        }
                    },
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    match completion.clone() {
                        self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                            let value = *value;
                            self::continue_java_script_iterator_protocol_value(
                                heap.clone(),
                                state.clone(),
                                value.clone(),
                            )
                        }
                        self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                            let value_2 = *value_2;
                            if ((shared_constructor_field4
                                == (self::ml_string::Text::from_units(vec![
                                    99, 108, 111, 115, 101,
                                ])))
                                && ((match shared_constructor_field10.clone() {
                                    self::JavaScriptIteratorCompletion::IteratorCompletion(
                                        source_ml_source2_data_property_value13681,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value13681.clone(),
                                }) == (self::ml_string::Text::from_units(vec![
                                    116, 104, 114, 111, 119,
                                ]))))
                            {
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolClosed(
                                    Box::new(heap.clone()),
                                    Box::new(shared_constructor_field8.clone()),
                                    Box::new(shared_constructor_field10.clone()),
                                )
                            } else {
                                self::JavaScriptIteratorProtocolTransition::IteratorProtocolThrow(
                                    Box::new(heap.clone()),
                                    Box::new(
                                        if (shared_constructor_field4
                                            == (self::ml_string::Text::from_units(vec![
                                                115, 116, 101, 112,
                                            ])))
                                        {
                                            {
                                                let source_ml_source2_nominal_field13696 =
                                                    shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field13698 =
                                                        shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field13700 =
                                                            shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field13702 =
                                                                shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field13704 =
                                                                    shared_constructor_field8
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field13706 =
                                                                        shared_constructor_field9
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field13708 = shared_constructor_field10.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field13710 = shared_constructor_field11.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field13712 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_override13714 = {
                                                                                        let source_ml_source2_nominal_spread13687 = shared_constructor_field8.clone();
                                                                                        match source_ml_source2_nominal_spread13687.clone() {
                                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                                        let shared_constructor_field1 = *shared_constructor_field1;
                                                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field13688 = shared_constructor_field1.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field13690 = shared_constructor_field2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field13692 = shared_constructor_field3;
                                                                                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(source_ml_source2_nominal_field13688.clone()), Box::new(source_ml_source2_nominal_field13690.clone()), true)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                    };
                                                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(source_ml_source2_nominal_field13696.clone(), source_ml_source2_nominal_field13698.clone(), Box::new(source_ml_source2_nominal_field13700.clone()), Box::new(source_ml_source2_nominal_field13702.clone()), Box::new(source_ml_source2_nominal_override13714.clone()), Box::new(source_ml_source2_nominal_field13706.clone()), Box::new(source_ml_source2_nominal_field13708.clone()), source_ml_source2_nominal_field13710.clone(), source_ml_source2_nominal_field13712.clone())
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
                                            state.clone()
                                        },
                                    ),
                                    Box::new(value_2.clone()),
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
