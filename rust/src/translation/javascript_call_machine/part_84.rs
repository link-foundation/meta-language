// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentTransition;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop418Result;
use super::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop419Result;
use super::clear_java_script_array_creation_child;
use super::fail_java_script_array_creation;
use super::ml_string;
use super::request_java_script_array_creation_call;
use super::source_ml_source1_temporary_construct_java_script_array_creation_index_loop418;
use super::source_ml_source1_temporary_construct_java_script_array_creation_index_loop419;
use super::throw_java_script_array_creation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_creation_length_set(
    state: self::JavaScriptArrayCreationState,
    transition: self::JavaScriptArrayAssignmentTransition,
) -> self::JavaScriptArrayCreationTransition {
    {
        let base = self::clear_java_script_array_creation_child(state.clone());
        match base.clone() {
            self::JavaScriptArrayCreationState::ArrayCreationState(
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
                shared_constructor_field24,
                shared_constructor_field25,
                shared_constructor_field26,
                shared_constructor_field27,
                shared_constructor_field28,
                shared_constructor_field29,
                shared_constructor_field30,
            ) => {
                let shared_constructor_field2 = *shared_constructor_field2;
                let shared_constructor_field4 = *shared_constructor_field4;
                let shared_constructor_field5 = *shared_constructor_field5;
                let shared_constructor_field6 = *shared_constructor_field6;
                let shared_constructor_field7 = *shared_constructor_field7;
                let shared_constructor_field8 = *shared_constructor_field8;
                let shared_constructor_field9 = *shared_constructor_field9;
                let shared_constructor_field15 = *shared_constructor_field15;
                let shared_constructor_field16 = *shared_constructor_field16;
                let shared_constructor_field17 = *shared_constructor_field17;
                let shared_constructor_field18 = *shared_constructor_field18;
                let shared_constructor_field19 = *shared_constructor_field19;
                match transition.clone() {
                    self::JavaScriptArrayAssignmentTransition::ArrayAssignmentContinue(
                        heap,
                        state_2,
                    ) => {
                        let heap = *heap;
                        let state_2 = *state_2;
                        self::JavaScriptArrayCreationTransition::ArrayCreationContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field16394 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field16396 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field16398 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field16400 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field16402 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field16404 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16406 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16408 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16410 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16412 =
                                                                        shared_constructor_field10;
                                                                    {
                                                                        let source_ml_source2_nominal_field16414 = shared_constructor_field11;
                                                                        {
                                                                            let source_ml_source2_nominal_field16416 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field16418 = shared_constructor_field13;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16420 = shared_constructor_field14;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16422 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16424 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16426 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16428 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16430 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16432 = shared_constructor_field20.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16434 = shared_constructor_field21.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16436 = shared_constructor_field22.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16438 = shared_constructor_field23.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16440 = shared_constructor_field24.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16442 = shared_constructor_field25.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16444 = shared_constructor_field26.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16446 = shared_constructor_field27.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16448 = shared_constructor_field28.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16450 = shared_constructor_field29.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16452 = shared_constructor_field30.clone();
                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16394.clone(), Box::new(source_ml_source2_nominal_field16396.clone()), source_ml_source2_nominal_field16398.clone(), Box::new(source_ml_source2_nominal_field16400.clone()), Box::new(source_ml_source2_nominal_field16402.clone()), Box::new(source_ml_source2_nominal_field16404.clone()), Box::new(source_ml_source2_nominal_field16406.clone()), Box::new(source_ml_source2_nominal_field16408.clone()), Box::new(source_ml_source2_nominal_field16410.clone()), source_ml_source2_nominal_field16412, source_ml_source2_nominal_field16414, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), source_ml_source2_nominal_field16418, source_ml_source2_nominal_field16420, Box::new(source_ml_source2_nominal_field16422.clone()), Box::new(source_ml_source2_nominal_field16424.clone()), Box::new(source_ml_source2_nominal_field16426.clone()), Box::new(source_ml_source2_nominal_field16428.clone()), Box::new(source_ml_source2_nominal_field16430.clone()), source_ml_source2_nominal_field16432.clone(), source_ml_source2_nominal_field16434.clone(), source_ml_source2_nominal_field16436.clone(), source_ml_source2_nominal_field16438.clone(), source_ml_source2_nominal_field16440.clone(), source_ml_source2_nominal_field16442.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field16446.clone(), source_ml_source2_nominal_field16448.clone(), source_ml_source2_nominal_field16450.clone(), source_ml_source2_nominal_field16452.clone())
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
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }),
                        )
                    }
                    self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCall(
                        heap_2,
                        state_3,
                        callee,
                        receiver,
                        arguments,
                    ) => {
                        let heap_2 = *heap_2;
                        let state_3 = *state_3;
                        let callee = *callee;
                        let receiver = *receiver;
                        self::request_java_script_array_creation_call(
                            heap_2.clone(),
                            {
                                let source_ml_source2_nominal_field16457 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field16459 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field16461 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field16463 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field16465 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field16467 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16469 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16471 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16473 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16475 =
                                                                        shared_constructor_field10;
                                                                    {
                                                                        let source_ml_source2_nominal_field16477 = shared_constructor_field11;
                                                                        {
                                                                            let source_ml_source2_nominal_field16479 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field16481 = shared_constructor_field13;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16483 = shared_constructor_field14;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16485 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16487 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16489 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16491 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16493 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16495 = shared_constructor_field20.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16497 = shared_constructor_field21.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16499 = shared_constructor_field22.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16501 = shared_constructor_field23.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16503 = shared_constructor_field24.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16505 = shared_constructor_field25.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16507 = shared_constructor_field26.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16509 = shared_constructor_field27.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16511 = shared_constructor_field28.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16513 = shared_constructor_field29.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16515 = shared_constructor_field30.clone();
                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16457.clone(), Box::new(source_ml_source2_nominal_field16459.clone()), source_ml_source2_nominal_field16461.clone(), Box::new(source_ml_source2_nominal_field16463.clone()), Box::new(source_ml_source2_nominal_field16465.clone()), Box::new(source_ml_source2_nominal_field16467.clone()), Box::new(source_ml_source2_nominal_field16469.clone()), Box::new(source_ml_source2_nominal_field16471.clone()), Box::new(source_ml_source2_nominal_field16473.clone()), source_ml_source2_nominal_field16475, source_ml_source2_nominal_field16477, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), source_ml_source2_nominal_field16481, source_ml_source2_nominal_field16483, Box::new(source_ml_source2_nominal_field16485.clone()), Box::new(source_ml_source2_nominal_field16487.clone()), Box::new(source_ml_source2_nominal_field16489.clone()), Box::new(source_ml_source2_nominal_field16491.clone()), Box::new(source_ml_source2_nominal_field16493.clone()), source_ml_source2_nominal_field16495.clone(), source_ml_source2_nominal_field16497.clone(), source_ml_source2_nominal_field16499.clone(), source_ml_source2_nominal_field16501.clone(), source_ml_source2_nominal_field16503.clone(), source_ml_source2_nominal_field16505.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field16509.clone(), source_ml_source2_nominal_field16511.clone(), source_ml_source2_nominal_field16513.clone(), source_ml_source2_nominal_field16515.clone())
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
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            callee.clone(),
                            receiver.clone(),
                            arguments.clone(),
                        )
                    }
                    self::JavaScriptArrayAssignmentTransition::ArrayAssignmentCompleted(
                        heap_3,
                        success,
                        _,
                    ) => {
                        let heap_3 = *heap_3;
                        if success {
                            self::JavaScriptArrayCreationTransition::ArrayCreationCompleted(
                                Box::new(heap_3.clone()),
                                Box::new(match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                        source_ml_source2_data_property_value16519,
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
                                        let source_ml_source2_data_property_value16519 =
                                            *source_ml_source2_data_property_value16519;
                                        source_ml_source2_data_property_value16519.clone()
                                    }
                                }),
                            )
                        } else {
                            self::fail_java_script_array_creation(
                                heap_3.clone(),
                                base.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ])),
                                &(self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111,
                                    110, 32, 99, 111, 117, 108, 100, 32, 110, 111, 116, 32, 115,
                                    101, 116, 32, 114, 101, 115, 117, 108, 116, 32, 108, 101, 110,
                                    103, 116, 104,
                                ])),
                            )
                        }
                    }
                    self::JavaScriptArrayAssignmentTransition::ArrayAssignmentThrow(
                        heap_4,
                        value,
                    ) => {
                        let heap_4 = *heap_4;
                        let value = *value;
                        self::throw_java_script_array_creation(
                            heap_4.clone(),
                            base.clone(),
                            value.clone(),
                        )
                    }
                    self::JavaScriptArrayAssignmentTransition::ArrayAssignmentError(
                        heap_5,
                        error_class,
                        reason,
                    ) => {
                        let heap_5 = *heap_5;
                        self::fail_java_script_array_creation(
                            heap_5.clone(),
                            base.clone(),
                            &(error_class),
                            &(reason),
                        )
                    }
                    self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                        heap_6,
                        reason_2,
                    ) => {
                        let heap_6 = *heap_6;
                        self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                            Box::new(heap_6.clone()),
                            reason_2.clone(),
                        )
                    }
                    self::JavaScriptArrayAssignmentTransition::ArrayAssignmentResourceLimit(
                        heap_7,
                        state_4,
                        reason_3,
                    ) => {
                        let heap_7 = *heap_7;
                        let state_4 = *state_4;
                        self::JavaScriptArrayCreationTransition::ArrayCreationResourceLimit(
                            Box::new(heap_7.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field16521 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field16523 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field16525 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field16527 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field16529 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field16531 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16533 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16535 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16537 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16539 =
                                                                        shared_constructor_field10;
                                                                    {
                                                                        let source_ml_source2_nominal_field16541 = shared_constructor_field11;
                                                                        {
                                                                            let source_ml_source2_nominal_field16543 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field16545 = shared_constructor_field13;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16547 = shared_constructor_field14;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16549 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16551 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16553 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16555 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16557 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16559 = shared_constructor_field20.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16561 = shared_constructor_field21.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16563 = shared_constructor_field22.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16565 = shared_constructor_field23.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16567 = shared_constructor_field24.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16569 = shared_constructor_field25.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16571 = shared_constructor_field26.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16573 = shared_constructor_field27.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16575 = shared_constructor_field28.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16577 = shared_constructor_field29.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16579 = shared_constructor_field30.clone();
                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16521.clone(), Box::new(source_ml_source2_nominal_field16523.clone()), source_ml_source2_nominal_field16525.clone(), Box::new(source_ml_source2_nominal_field16527.clone()), Box::new(source_ml_source2_nominal_field16529.clone()), Box::new(source_ml_source2_nominal_field16531.clone()), Box::new(source_ml_source2_nominal_field16533.clone()), Box::new(source_ml_source2_nominal_field16535.clone()), Box::new(source_ml_source2_nominal_field16537.clone()), source_ml_source2_nominal_field16539, source_ml_source2_nominal_field16541, self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), source_ml_source2_nominal_field16545, source_ml_source2_nominal_field16547, Box::new(source_ml_source2_nominal_field16549.clone()), Box::new(source_ml_source2_nominal_field16551.clone()), Box::new(source_ml_source2_nominal_field16553.clone()), Box::new(source_ml_source2_nominal_field16555.clone()), Box::new(source_ml_source2_nominal_field16557.clone()), source_ml_source2_nominal_field16559.clone(), source_ml_source2_nominal_field16561.clone(), source_ml_source2_nominal_field16563.clone(), source_ml_source2_nominal_field16565.clone(), source_ml_source2_nominal_field16567.clone(), source_ml_source2_nominal_field16569.clone(), Some((Some(state_4.clone()),)), source_ml_source2_nominal_field16573.clone(), source_ml_source2_nominal_field16575.clone(), source_ml_source2_nominal_field16577.clone(), source_ml_source2_nominal_field16579.clone())
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
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }),
                            reason_3.clone(),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_array_creation_index(index: f64) -> self::JavaScriptSemanticValue {
    {
        let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
        {
            let reversed = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
            {
                let source_ml_source2_s16583 = self::source_ml_source1_temporary_construct_java_script_array_creation_index_loop418(reversed.clone(), index);
                match source_ml_source2_s16583.clone() {
                    self::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop418Result::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop418Done => {
                        {
                            let at = ({ let ml_reference_array861 = reversed.clone(); let ml_reference_projection861 = ml_reference_array861.borrow().len() as f64; ml_reference_projection861 } - 1f64);
                            {
                                let source_ml_source2_s16584 = self::source_ml_source1_temporary_construct_java_script_array_creation_index_loop419(units.clone(), reversed.clone(), at);
                                match source_ml_source2_s16584.clone() {
                                    self::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop419Result::MlSource1TemporaryConstructJavaScriptArrayCreationIndexLoop419Done => {
                                        self::JavaScriptSemanticValue::Text(units.clone())
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
