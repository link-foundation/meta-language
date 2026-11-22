// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConcatenationCall;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConcatenationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::ml_string;
use super::read_java_script_array_like_length;
use super::request_java_script_array_concatenation_call;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_concatenation_length(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayConcatenationState,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptArrayConcatenationTransition {
    {
        let source_ml_source1_temporary_binding339 = state.clone();
        {
            let prior_call = match source_ml_source1_temporary_binding339.clone() {
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
                    _,
                    source_ml_source2_data_property_value9419,
                ) => ((source_ml_source2_data_property_value9419
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,)))
                .0)
                    .clone(),
            };
            {
                let prior_conversion = match source_ml_source1_temporary_binding339.clone() {
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
                        source_ml_source2_data_property_value9420,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value9420
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,)))
                    .0)
                        .clone(),
                };
                {
                    let base = match source_ml_source1_temporary_binding339.clone() {
                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
                            source_ml_source2_nominal_rest_field9421,
                            source_ml_source2_nominal_rest_field9422,
                            source_ml_source2_nominal_rest_field9423,
                            source_ml_source2_nominal_rest_field9424,
                            source_ml_source2_nominal_rest_field9425,
                            source_ml_source2_nominal_rest_field9426,
                            source_ml_source2_nominal_rest_field9427,
                            source_ml_source2_nominal_rest_field9428,
                            source_ml_source2_nominal_rest_field9429,
                            source_ml_source2_nominal_rest_field9430,
                            source_ml_source2_nominal_rest_field9431,
                            source_ml_source2_nominal_rest_field9432,
                            source_ml_source2_nominal_rest_field9433,
                            source_ml_source2_nominal_rest_field9434,
                            source_ml_source2_nominal_rest_field9435,
                            _,
                            source_ml_source2_nominal_rest_field9436,
                            source_ml_source2_nominal_rest_field9437,
                            source_ml_source2_nominal_rest_field9438,
                            source_ml_source2_nominal_rest_field9439,
                            source_ml_source2_nominal_rest_field9440,
                            _,
                        ) => {
                            let source_ml_source2_nominal_rest_field9421 =
                                *source_ml_source2_nominal_rest_field9421;
                            let source_ml_source2_nominal_rest_field9423 =
                                *source_ml_source2_nominal_rest_field9423;
                            let source_ml_source2_nominal_rest_field9424 =
                                *source_ml_source2_nominal_rest_field9424;
                            let source_ml_source2_nominal_rest_field9426 =
                                *source_ml_source2_nominal_rest_field9426;
                            let source_ml_source2_nominal_rest_field9432 =
                                *source_ml_source2_nominal_rest_field9432;
                            let source_ml_source2_nominal_rest_field9433 =
                                *source_ml_source2_nominal_rest_field9433;
                            let source_ml_source2_nominal_rest_field9434 =
                                *source_ml_source2_nominal_rest_field9434;
                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
                                Box::new(source_ml_source2_nominal_rest_field9421.clone()),
                                source_ml_source2_nominal_rest_field9422.clone(),
                                Box::new(source_ml_source2_nominal_rest_field9423.clone()),
                                Box::new(source_ml_source2_nominal_rest_field9424.clone()),
                                source_ml_source2_nominal_rest_field9425,
                                Box::new(source_ml_source2_nominal_rest_field9426.clone()),
                                source_ml_source2_nominal_rest_field9427,
                                source_ml_source2_nominal_rest_field9428,
                                source_ml_source2_nominal_rest_field9429,
                                source_ml_source2_nominal_rest_field9430,
                                source_ml_source2_nominal_rest_field9431,
                                Box::new(source_ml_source2_nominal_rest_field9432.clone()),
                                Box::new(source_ml_source2_nominal_rest_field9433.clone()),
                                Box::new(source_ml_source2_nominal_rest_field9434.clone()),
                                source_ml_source2_nominal_rest_field9435.clone(),
                                None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                                source_ml_source2_nominal_rest_field9436.clone(),
                                source_ml_source2_nominal_rest_field9437.clone(),
                                source_ml_source2_nominal_rest_field9438.clone(),
                                source_ml_source2_nominal_rest_field9439.clone(),
                                source_ml_source2_nominal_rest_field9440.clone(),
                                None::<(Option<self::JavaScriptArrayConcatenationCall>,)>,
                            )
                        }
                    };
                    match base.clone() {
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
                            match transition.clone() {
                                self::JavaScriptScalarConversionTransition::ScalarContinuation(state_2) => {
                                    let state_2 = *state_2;
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field9442 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field9444 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field9446 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field9448 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field9450 = shared_constructor_field5;
                                                        {
                                                            let source_ml_source2_nominal_field9452 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field9454 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field9456 = shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field9458 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field9460 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field9462 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field9464 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9466 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9468 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9470 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9472 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9474 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9476 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9478 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9480 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9482 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9484 = shared_constructor_field22.clone();
                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9442.clone()), source_ml_source2_nominal_field9444.clone(), Box::new(source_ml_source2_nominal_field9446.clone()), Box::new(source_ml_source2_nominal_field9448.clone()), source_ml_source2_nominal_field9450, Box::new(source_ml_source2_nominal_field9452.clone()), source_ml_source2_nominal_field9454, source_ml_source2_nominal_field9456, source_ml_source2_nominal_field9458, source_ml_source2_nominal_field9460, source_ml_source2_nominal_field9462, Box::new(source_ml_source2_nominal_field9464.clone()), Box::new(source_ml_source2_nominal_field9466.clone()), Box::new(source_ml_source2_nominal_field9468.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field9474.clone(), source_ml_source2_nominal_field9476.clone(), source_ml_source2_nominal_field9478.clone(), source_ml_source2_nominal_field9480.clone(), source_ml_source2_nominal_field9482.clone(), source_ml_source2_nominal_field9484.clone())
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
                                    }))
                                }
                                self::JavaScriptScalarConversionTransition::ScalarCall(state_3, callee, receiver, arguments) => {
                                    let state_3 = *state_3;
                                    let callee = *callee;
                                    let receiver = *receiver;
                                    self::request_java_script_array_concatenation_call(heap.clone(), {
                                        let source_ml_source2_nominal_field9489 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field9491 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field9493 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field9495 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field9497 = shared_constructor_field5;
                                                        {
                                                            let source_ml_source2_nominal_field9499 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field9501 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field9503 = shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field9505 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field9507 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field9509 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field9511 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9513 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9515 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9517 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9519 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9521 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9523 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9525 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9527 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9529 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9531 = shared_constructor_field22.clone();
                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9489.clone()), source_ml_source2_nominal_field9491.clone(), Box::new(source_ml_source2_nominal_field9493.clone()), Box::new(source_ml_source2_nominal_field9495.clone()), source_ml_source2_nominal_field9497, Box::new(source_ml_source2_nominal_field9499.clone()), source_ml_source2_nominal_field9501, source_ml_source2_nominal_field9503, source_ml_source2_nominal_field9505, source_ml_source2_nominal_field9507, source_ml_source2_nominal_field9509, Box::new(source_ml_source2_nominal_field9511.clone()), Box::new(source_ml_source2_nominal_field9513.clone()), Box::new(source_ml_source2_nominal_field9515.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field9521.clone(), source_ml_source2_nominal_field9523.clone(), source_ml_source2_nominal_field9525.clone(), source_ml_source2_nominal_field9527.clone(), source_ml_source2_nominal_field9529.clone(), source_ml_source2_nominal_field9531.clone())
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
                                    }, callee.clone(), receiver.clone(), arguments.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarCompleted(primitive) => {
                                    let primitive = *primitive;
                                    match primitive.clone() {
                                        self::JavaScriptSemanticValue::Number(value) => {
                                            {
                                                let length = self::read_java_script_array_like_length(value);
                                                if (length > ((9007199254740991f64 - match state.clone() {
                                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9536, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value9536
                                                    }
                                                }))) {
                                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 109, 97, 120, 105, 109, 117, 109, 32, 115, 97, 102, 101, 32, 111, 117, 116, 112, 117, 116, 32, 108, 101, 110, 103, 116, 104]))
                                                } else {
                                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field9538 = shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_nominal_field9540 = shared_constructor_field2.clone();
                                                            {
                                                                let source_ml_source2_nominal_field9542 = shared_constructor_field3.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field9544 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field9546 = shared_constructor_field5;
                                                                        {
                                                                            let source_ml_source2_nominal_field9548 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field9550 = shared_constructor_field7;
                                                                                {
                                                                                    let source_ml_source2_nominal_field9552 = shared_constructor_field8;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9554 = shared_constructor_field9;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9556 = shared_constructor_field10;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9558 = shared_constructor_field11;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9560 = shared_constructor_field12.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9562 = shared_constructor_field13.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9564 = shared_constructor_field14.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9566 = shared_constructor_field15.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9568 = shared_constructor_field16.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9570 = shared_constructor_field17.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9572 = shared_constructor_field18.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field9574 = shared_constructor_field19.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field9576 = shared_constructor_field20.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field9578 = shared_constructor_field21.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field9580 = shared_constructor_field22.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_override9584 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9538.clone()), source_ml_source2_nominal_field9540.clone(), Box::new(source_ml_source2_nominal_field9542.clone()), Box::new(source_ml_source2_nominal_field9544.clone()), source_ml_source2_nominal_field9546, Box::new(source_ml_source2_nominal_field9548.clone()), source_ml_source2_nominal_field9550, length, source_ml_source2_nominal_field9554, source_ml_source2_nominal_field9556, source_ml_source2_nominal_field9558, Box::new(source_ml_source2_nominal_field9560.clone()), Box::new(source_ml_source2_nominal_field9562.clone()), Box::new(source_ml_source2_nominal_override9584.clone()), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field9568.clone(), source_ml_source2_nominal_field9570.clone(), source_ml_source2_nominal_field9572.clone(), source_ml_source2_nominal_field9574.clone(), source_ml_source2_nominal_field9576.clone(), source_ml_source2_nominal_field9578.clone(), source_ml_source2_nominal_field9580.clone())
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
                                                    }))
                                                }
                                            }
                                        }
                                        _ => {
                                            self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                                        }
                                    }
                                }
                                self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                                    let value_2 = *value_2;
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
                                }
                                self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(Box::new(heap.clone()), error_class.clone(), reason.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), reason_2.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
                                    let value_3 = *value_3;
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationResourceLimit(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field9586 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field9588 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field9590 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field9592 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field9594 = shared_constructor_field5;
                                                        {
                                                            let source_ml_source2_nominal_field9596 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field9598 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field9600 = shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field9602 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field9604 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field9606 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field9608 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9610 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9612 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9614 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9616 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9618 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9620 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9622 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9624 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9626 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9628 = shared_constructor_field22.clone();
                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9586.clone()), source_ml_source2_nominal_field9588.clone(), Box::new(source_ml_source2_nominal_field9590.clone()), Box::new(source_ml_source2_nominal_field9592.clone()), source_ml_source2_nominal_field9594, Box::new(source_ml_source2_nominal_field9596.clone()), source_ml_source2_nominal_field9598, source_ml_source2_nominal_field9600, source_ml_source2_nominal_field9602, source_ml_source2_nominal_field9604, source_ml_source2_nominal_field9606, Box::new(source_ml_source2_nominal_field9608.clone()), Box::new(source_ml_source2_nominal_field9610.clone()), Box::new(value_3.clone()), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101]), source_ml_source2_nominal_field9616.clone(), source_ml_source2_nominal_field9618.clone(), source_ml_source2_nominal_field9620.clone(), source_ml_source2_nominal_field9622.clone(), source_ml_source2_nominal_field9624.clone(), source_ml_source2_nominal_field9626.clone(), source_ml_source2_nominal_field9628.clone())
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
                                    }), reason_3.clone())
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
