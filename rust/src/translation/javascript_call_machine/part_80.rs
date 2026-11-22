// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::clear_java_script_array_creation_child;
use super::ml_string;
use super::request_java_script_array_creation_call;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_creation_iterator_step(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    record: self::JavaScriptSynchronousIteratorRecord,
    done: bool,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayCreationTransition {
    match state.clone() {
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
            if done {
                self::JavaScriptArrayCreationTransition::ArrayCreationContinue(
                    Box::new(heap.clone()),
                    Box::new({
                        let source_ml_source2_nominal_field15299 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_nominal_field15301 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_nominal_field15303 =
                                    shared_constructor_field3.clone();
                                {
                                    let source_ml_source2_nominal_field15305 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_nominal_field15307 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_nominal_field15309 =
                                                shared_constructor_field6.clone();
                                            {
                                                let source_ml_source2_nominal_field15311 =
                                                    shared_constructor_field7.clone();
                                                {
                                                    let source_ml_source2_nominal_field15313 =
                                                        shared_constructor_field8.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15315 =
                                                            shared_constructor_field9.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15317 =
                                                                shared_constructor_field10;
                                                            {
                                                                let source_ml_source2_nominal_field15319 =
                                                                    shared_constructor_field11;
                                                                {
                                                                    let source_ml_source2_nominal_field15321 =
                                                                        shared_constructor_field12
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15323 = shared_constructor_field13;
                                                                        {
                                                                            let source_ml_source2_nominal_field15325 = shared_constructor_field14;
                                                                            {
                                                                                let source_ml_source2_nominal_field15327 = shared_constructor_field15.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field15329 = shared_constructor_field16.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15331 = shared_constructor_field17.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15333 = shared_constructor_field18.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15335 = shared_constructor_field19.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15337 = shared_constructor_field20.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15339 = shared_constructor_field21.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15341 = shared_constructor_field22.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15343 = shared_constructor_field23.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15345 = shared_constructor_field24.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15347 = shared_constructor_field25.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15349 = shared_constructor_field26.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15351 = shared_constructor_field27.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15353 = shared_constructor_field28.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15355 = shared_constructor_field29.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15357 = shared_constructor_field30.clone();
                                                                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15299.clone(), Box::new(source_ml_source2_nominal_field15301.clone()), source_ml_source2_nominal_field15303.clone(), Box::new(source_ml_source2_nominal_field15305.clone()), Box::new(source_ml_source2_nominal_field15307.clone()), Box::new(source_ml_source2_nominal_field15309.clone()), Box::new(source_ml_source2_nominal_field15311.clone()), Box::new(source_ml_source2_nominal_field15313.clone()), Box::new(source_ml_source2_nominal_field15315.clone()), source_ml_source2_nominal_field15317, source_ml_source2_nominal_field15319, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116]), source_ml_source2_nominal_field15323, source_ml_source2_nominal_field15325, Box::new(source_ml_source2_nominal_field15327.clone()), Box::new(source_ml_source2_nominal_field15329.clone()), Box::new(source_ml_source2_nominal_field15331.clone()), Box::new(source_ml_source2_nominal_field15333.clone()), Box::new(source_ml_source2_nominal_field15335.clone()), source_ml_source2_nominal_field15337.clone(), Some((Some(record.clone()),)), source_ml_source2_nominal_field15341.clone(), source_ml_source2_nominal_field15343.clone(), source_ml_source2_nominal_field15345.clone(), source_ml_source2_nominal_field15347.clone(), source_ml_source2_nominal_field15349.clone(), source_ml_source2_nominal_field15351.clone(), source_ml_source2_nominal_field15353.clone(), source_ml_source2_nominal_field15355.clone(), source_ml_source2_nominal_field15357.clone())
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
            } else {
                self::select_java_script_array_creation_value(
                    heap.clone(),
                    {
                        let source_ml_source2_nominal_field15362 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_nominal_field15364 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_nominal_field15366 =
                                    shared_constructor_field3.clone();
                                {
                                    let source_ml_source2_nominal_field15368 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_nominal_field15370 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_nominal_field15372 =
                                                shared_constructor_field6.clone();
                                            {
                                                let source_ml_source2_nominal_field15374 =
                                                    shared_constructor_field7.clone();
                                                {
                                                    let source_ml_source2_nominal_field15376 =
                                                        shared_constructor_field8.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15378 =
                                                            shared_constructor_field9.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15380 =
                                                                shared_constructor_field10;
                                                            {
                                                                let source_ml_source2_nominal_field15382 =
                                                                    shared_constructor_field11;
                                                                {
                                                                    let source_ml_source2_nominal_field15384 =
                                                                        shared_constructor_field12
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15386 = shared_constructor_field13;
                                                                        {
                                                                            let source_ml_source2_nominal_field15388 = shared_constructor_field14;
                                                                            {
                                                                                let source_ml_source2_nominal_field15390 = shared_constructor_field15.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field15392 = shared_constructor_field16.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15394 = shared_constructor_field17.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15396 = shared_constructor_field18.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15398 = shared_constructor_field19.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15400 = shared_constructor_field20.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15402 = shared_constructor_field21.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15404 = shared_constructor_field22.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15406 = shared_constructor_field23.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15408 = shared_constructor_field24.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15410 = shared_constructor_field25.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15412 = shared_constructor_field26.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15414 = shared_constructor_field27.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15416 = shared_constructor_field28.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15418 = shared_constructor_field29.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15420 = shared_constructor_field30.clone();
                                                                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15362.clone(), Box::new(source_ml_source2_nominal_field15364.clone()), source_ml_source2_nominal_field15366.clone(), Box::new(source_ml_source2_nominal_field15368.clone()), Box::new(source_ml_source2_nominal_field15370.clone()), Box::new(source_ml_source2_nominal_field15372.clone()), Box::new(source_ml_source2_nominal_field15374.clone()), Box::new(source_ml_source2_nominal_field15376.clone()), Box::new(source_ml_source2_nominal_field15378.clone()), source_ml_source2_nominal_field15380, source_ml_source2_nominal_field15382, source_ml_source2_nominal_field15384.clone(), source_ml_source2_nominal_field15386, source_ml_source2_nominal_field15388, Box::new(source_ml_source2_nominal_field15390.clone()), Box::new(source_ml_source2_nominal_field15392.clone()), Box::new(source_ml_source2_nominal_field15394.clone()), Box::new(source_ml_source2_nominal_field15396.clone()), Box::new(source_ml_source2_nominal_field15398.clone()), source_ml_source2_nominal_field15400.clone(), Some((Some(record.clone()),)), source_ml_source2_nominal_field15404.clone(), source_ml_source2_nominal_field15406.clone(), source_ml_source2_nominal_field15408.clone(), source_ml_source2_nominal_field15410.clone(), source_ml_source2_nominal_field15412.clone(), source_ml_source2_nominal_field15414.clone(), source_ml_source2_nominal_field15416.clone(), source_ml_source2_nominal_field15418.clone(), source_ml_source2_nominal_field15420.clone())
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
                    value.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_java_script_array_creation_value(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayCreationTransition {
    match state.clone() {
        self::JavaScriptArrayCreationState::ArrayCreationState(
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field61,
            shared_constructor_field62,
            shared_constructor_field63,
            _,
            _,
            shared_constructor_field64,
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
        ) => {
            let shared_constructor_field61 = *shared_constructor_field61;
            let shared_constructor_field62 = *shared_constructor_field62;
            {
                let base = self::clear_java_script_array_creation_child(state.clone());
                match base.clone() {
                    self::JavaScriptArrayCreationState::ArrayCreationState(
                        shared_constructor_field31,
                        shared_constructor_field32,
                        shared_constructor_field33,
                        shared_constructor_field34,
                        shared_constructor_field35,
                        shared_constructor_field36,
                        shared_constructor_field37,
                        shared_constructor_field38,
                        shared_constructor_field39,
                        shared_constructor_field40,
                        shared_constructor_field41,
                        shared_constructor_field42,
                        shared_constructor_field43,
                        shared_constructor_field44,
                        shared_constructor_field45,
                        shared_constructor_field46,
                        shared_constructor_field47,
                        shared_constructor_field48,
                        shared_constructor_field49,
                        shared_constructor_field50,
                        shared_constructor_field51,
                        shared_constructor_field52,
                        shared_constructor_field53,
                        shared_constructor_field54,
                        shared_constructor_field55,
                        shared_constructor_field56,
                        shared_constructor_field57,
                        shared_constructor_field58,
                        shared_constructor_field59,
                        shared_constructor_field60,
                    ) => {
                        let shared_constructor_field32 = *shared_constructor_field32;
                        let shared_constructor_field34 = *shared_constructor_field34;
                        let shared_constructor_field35 = *shared_constructor_field35;
                        let shared_constructor_field36 = *shared_constructor_field36;
                        let shared_constructor_field37 = *shared_constructor_field37;
                        let shared_constructor_field38 = *shared_constructor_field38;
                        let shared_constructor_field39 = *shared_constructor_field39;
                        let shared_constructor_field45 = *shared_constructor_field45;
                        let shared_constructor_field46 = *shared_constructor_field46;
                        let shared_constructor_field47 = *shared_constructor_field47;
                        let shared_constructor_field48 = *shared_constructor_field48;
                        let shared_constructor_field49 = *shared_constructor_field49;
                        {
                            let next = {
                                let source_ml_source2_nominal_field15424 =
                                    shared_constructor_field31.clone();
                                {
                                    let source_ml_source2_nominal_field15426 =
                                        shared_constructor_field32.clone();
                                    {
                                        let source_ml_source2_nominal_field15428 =
                                            shared_constructor_field33.clone();
                                        {
                                            let source_ml_source2_nominal_field15430 =
                                                shared_constructor_field34.clone();
                                            {
                                                let source_ml_source2_nominal_field15432 =
                                                    shared_constructor_field35.clone();
                                                {
                                                    let source_ml_source2_nominal_field15434 =
                                                        shared_constructor_field36.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15436 =
                                                            shared_constructor_field37.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15438 =
                                                                shared_constructor_field38.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15440 =
                                                                    shared_constructor_field39
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15442 =
                                                                        shared_constructor_field40;
                                                                    {
                                                                        let source_ml_source2_nominal_field15444 = shared_constructor_field41;
                                                                        {
                                                                            let source_ml_source2_nominal_field15446 = shared_constructor_field42.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15448 = shared_constructor_field43;
                                                                                {
                                                                                    let source_ml_source2_nominal_field15450 = shared_constructor_field44;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15452 = shared_constructor_field45.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15454 = shared_constructor_field46.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15456 = shared_constructor_field47.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15458 = shared_constructor_field48.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15460 = shared_constructor_field49.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15462 = shared_constructor_field50.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15464 = shared_constructor_field51.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15466 = shared_constructor_field52.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15468 = shared_constructor_field53.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15470 = shared_constructor_field54.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15472 = shared_constructor_field55.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15474 = shared_constructor_field56.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15476 = shared_constructor_field57.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15478 = shared_constructor_field58.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15480 = shared_constructor_field59.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15482 = shared_constructor_field60.clone();
                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15424.clone(), Box::new(source_ml_source2_nominal_field15426.clone()), source_ml_source2_nominal_field15428.clone(), Box::new(source_ml_source2_nominal_field15430.clone()), Box::new(source_ml_source2_nominal_field15432.clone()), Box::new(source_ml_source2_nominal_field15434.clone()), Box::new(source_ml_source2_nominal_field15436.clone()), Box::new(source_ml_source2_nominal_field15438.clone()), Box::new(source_ml_source2_nominal_field15440.clone()), source_ml_source2_nominal_field15442, source_ml_source2_nominal_field15444, self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]), source_ml_source2_nominal_field15448, source_ml_source2_nominal_field15450, Box::new(source_ml_source2_nominal_field15452.clone()), Box::new(source_ml_source2_nominal_field15454.clone()), Box::new(source_ml_source2_nominal_field15456.clone()), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field15460.clone()), source_ml_source2_nominal_field15462.clone(), source_ml_source2_nominal_field15464.clone(), source_ml_source2_nominal_field15466.clone(), source_ml_source2_nominal_field15468.clone(), source_ml_source2_nominal_field15470.clone(), source_ml_source2_nominal_field15472.clone(), source_ml_source2_nominal_field15474.clone(), source_ml_source2_nominal_field15476.clone(), source_ml_source2_nominal_field15478.clone(), source_ml_source2_nominal_field15480.clone(), source_ml_source2_nominal_field15482.clone())
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
                            };
                            match next.clone() {
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
                                    if shared_constructor_field63 {
                                        self::request_java_script_array_creation_call(
                                            heap.clone(),
                                            {
                                                let source_ml_source2_nominal_field15488 =
                                                    shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field15490 =
                                                        shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15492 =
                                                            shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15494 =
                                                                shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15496 =
                                                                    shared_constructor_field5
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15498 =
                                                                        shared_constructor_field6
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15500 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field15502 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15504 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field15506 = shared_constructor_field10;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15508 = shared_constructor_field11;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15510 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15512 = shared_constructor_field13;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15514 = shared_constructor_field14;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15516 = shared_constructor_field15.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15518 = shared_constructor_field16.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15520 = shared_constructor_field17.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15522 = shared_constructor_field18.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15524 = shared_constructor_field19.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15526 = shared_constructor_field20.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15528 = shared_constructor_field21.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15530 = shared_constructor_field22.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15532 = shared_constructor_field23.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15534 = shared_constructor_field24.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15536 = shared_constructor_field25.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15538 = shared_constructor_field26.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15540 = shared_constructor_field27.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15542 = shared_constructor_field28.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field15544 = shared_constructor_field29.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field15546 = shared_constructor_field30.clone();
                                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15488.clone(), Box::new(source_ml_source2_nominal_field15490.clone()), source_ml_source2_nominal_field15492.clone(), Box::new(source_ml_source2_nominal_field15494.clone()), Box::new(source_ml_source2_nominal_field15496.clone()), Box::new(source_ml_source2_nominal_field15498.clone()), Box::new(source_ml_source2_nominal_field15500.clone()), Box::new(source_ml_source2_nominal_field15502.clone()), Box::new(source_ml_source2_nominal_field15504.clone()), source_ml_source2_nominal_field15506, source_ml_source2_nominal_field15508, self::ml_string::Text::from_units(vec![109, 97, 112]), source_ml_source2_nominal_field15512, source_ml_source2_nominal_field15514, Box::new(source_ml_source2_nominal_field15516.clone()), Box::new(source_ml_source2_nominal_field15518.clone()), Box::new(source_ml_source2_nominal_field15520.clone()), Box::new(source_ml_source2_nominal_field15522.clone()), Box::new(source_ml_source2_nominal_field15524.clone()), source_ml_source2_nominal_field15526.clone(), source_ml_source2_nominal_field15528.clone(), source_ml_source2_nominal_field15530.clone(), source_ml_source2_nominal_field15532.clone(), source_ml_source2_nominal_field15534.clone(), source_ml_source2_nominal_field15536.clone(), source_ml_source2_nominal_field15538.clone(), source_ml_source2_nominal_field15540.clone(), source_ml_source2_nominal_field15542.clone(), source_ml_source2_nominal_field15544.clone(), source_ml_source2_nominal_field15546.clone())
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
                                            shared_constructor_field61.clone(),
                                            shared_constructor_field62.clone(),
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                value.clone(),
                                                self::JavaScriptSemanticValue::Number(
                                                    shared_constructor_field64,
                                                ),
                                            ])),
                                        )
                                    } else {
                                        self::JavaScriptArrayCreationTransition::ArrayCreationContinue(Box::new(heap.clone()), Box::new(next.clone()))
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
