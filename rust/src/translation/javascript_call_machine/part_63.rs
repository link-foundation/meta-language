// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySliceCall;
use super::JavaScriptArraySliceState;
use super::JavaScriptArraySliceTransition;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptArraySpeciesTransition;
use super::JavaScriptOrdinaryHeap;
use super::allocation_request;
use super::copy_java_script_array_constructor_fact;
use super::ml_string;
use super::request_call;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_species(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySliceState,
    transition: self::JavaScriptArraySpeciesTransition,
) -> self::JavaScriptArraySliceTransition {
    {
        let source_ml_source1_temporary_binding363 = state.clone();
        {
            let prior_call = match source_ml_source1_temporary_binding363.clone() {
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
                    source_ml_source2_data_property_value11380,
                ) => ((source_ml_source2_data_property_value11380
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArraySliceCall>,)))
                .0)
                    .clone(),
            };
            {
                let base = match source_ml_source1_temporary_binding363.clone() {
                    self::JavaScriptArraySliceState::ArraySliceState(
                        source_ml_source2_nominal_rest_field11381,
                        source_ml_source2_nominal_rest_field11382,
                        source_ml_source2_nominal_rest_field11383,
                        source_ml_source2_nominal_rest_field11384,
                        source_ml_source2_nominal_rest_field11385,
                        source_ml_source2_nominal_rest_field11386,
                        source_ml_source2_nominal_rest_field11387,
                        source_ml_source2_nominal_rest_field11388,
                        source_ml_source2_nominal_rest_field11389,
                        source_ml_source2_nominal_rest_field11390,
                        source_ml_source2_nominal_rest_field11391,
                        source_ml_source2_nominal_rest_field11392,
                        source_ml_source2_nominal_rest_field11393,
                        source_ml_source2_nominal_rest_field11394,
                        source_ml_source2_nominal_rest_field11395,
                        source_ml_source2_nominal_rest_field11396,
                        source_ml_source2_nominal_rest_field11397,
                        source_ml_source2_nominal_rest_field11398,
                        source_ml_source2_nominal_rest_field11399,
                        source_ml_source2_nominal_rest_field11400,
                        source_ml_source2_nominal_rest_field11401,
                        source_ml_source2_nominal_rest_field11402,
                        _,
                    ) => {
                        let source_ml_source2_nominal_rest_field11381 =
                            *source_ml_source2_nominal_rest_field11381;
                        let source_ml_source2_nominal_rest_field11382 =
                            *source_ml_source2_nominal_rest_field11382;
                        let source_ml_source2_nominal_rest_field11383 =
                            *source_ml_source2_nominal_rest_field11383;
                        let source_ml_source2_nominal_rest_field11385 =
                            *source_ml_source2_nominal_rest_field11385;
                        let source_ml_source2_nominal_rest_field11392 =
                            *source_ml_source2_nominal_rest_field11392;
                        let source_ml_source2_nominal_rest_field11393 =
                            *source_ml_source2_nominal_rest_field11393;
                        let source_ml_source2_nominal_rest_field11394 =
                            *source_ml_source2_nominal_rest_field11394;
                        self::JavaScriptArraySliceState::ArraySliceState(
                            Box::new(source_ml_source2_nominal_rest_field11381.clone()),
                            Box::new(source_ml_source2_nominal_rest_field11382.clone()),
                            Box::new(source_ml_source2_nominal_rest_field11383.clone()),
                            source_ml_source2_nominal_rest_field11384,
                            Box::new(source_ml_source2_nominal_rest_field11385.clone()),
                            source_ml_source2_nominal_rest_field11386,
                            source_ml_source2_nominal_rest_field11387,
                            source_ml_source2_nominal_rest_field11388,
                            source_ml_source2_nominal_rest_field11389,
                            source_ml_source2_nominal_rest_field11390,
                            source_ml_source2_nominal_rest_field11391,
                            Box::new(source_ml_source2_nominal_rest_field11392.clone()),
                            Box::new(source_ml_source2_nominal_rest_field11393.clone()),
                            Box::new(source_ml_source2_nominal_rest_field11394.clone()),
                            source_ml_source2_nominal_rest_field11395.clone(),
                            source_ml_source2_nominal_rest_field11396.clone(),
                            source_ml_source2_nominal_rest_field11397.clone(),
                            source_ml_source2_nominal_rest_field11398.clone(),
                            source_ml_source2_nominal_rest_field11399.clone(),
                            source_ml_source2_nominal_rest_field11400.clone(),
                            source_ml_source2_nominal_rest_field11401.clone(),
                            source_ml_source2_nominal_rest_field11402.clone(),
                            None::<(Option<self::JavaScriptArraySliceCall>,)>,
                        )
                    }
                };
                match base.clone() {
                    self::JavaScriptArraySliceState::ArraySliceState(
                        shared_constructor_field29,
                        shared_constructor_field30,
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
                    ) => {
                        let shared_constructor_field29 = *shared_constructor_field29;
                        let shared_constructor_field30 = *shared_constructor_field30;
                        let shared_constructor_field31 = *shared_constructor_field31;
                        let shared_constructor_field33 = *shared_constructor_field33;
                        let shared_constructor_field40 = *shared_constructor_field40;
                        let shared_constructor_field41 = *shared_constructor_field41;
                        let shared_constructor_field42 = *shared_constructor_field42;
                        match transition.clone() {
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesContinue(
                                heap_2,
                                state_2,
                            ) => {
                                let heap_2 = *heap_2;
                                let state_2 = *state_2;
                                self::JavaScriptArraySliceTransition::ArraySliceContinue(
                                    Box::new(heap_2.clone()),
                                    Box::new({
                                        let source_ml_source2_nominal_field11405 =
                                            shared_constructor_field29.clone();
                                        {
                                            let source_ml_source2_nominal_field11407 =
                                                shared_constructor_field30.clone();
                                            {
                                                let source_ml_source2_nominal_field11409 =
                                                    shared_constructor_field31.clone();
                                                {
                                                    let source_ml_source2_nominal_field11411 =
                                                        shared_constructor_field32;
                                                    {
                                                        let source_ml_source2_nominal_field11413 =
                                                            shared_constructor_field33.clone();
                                                        {
                                                            let source_ml_source2_nominal_field11415 =
                                                                shared_constructor_field34;
                                                            {
                                                                let source_ml_source2_nominal_field11417 =
                                                                    shared_constructor_field35;
                                                                {
                                                                    let source_ml_source2_nominal_field11419 =
                                                                        shared_constructor_field36;
                                                                    {
                                                                        let source_ml_source2_nominal_field11421 = shared_constructor_field37;
                                                                        {
                                                                            let source_ml_source2_nominal_field11423 = shared_constructor_field38;
                                                                            {
                                                                                let source_ml_source2_nominal_field11425 = shared_constructor_field39;
                                                                                {
                                                                                    let source_ml_source2_nominal_field11427 = shared_constructor_field40.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11429 = shared_constructor_field41.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11431 = shared_constructor_field42.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11433 = shared_constructor_field43.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11435 = shared_constructor_field44.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11437 = shared_constructor_field45.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11439 = shared_constructor_field46.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11441 = shared_constructor_field47.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11443 = shared_constructor_field48.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11445 = shared_constructor_field49.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11447 = shared_constructor_field50.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field11449 = shared_constructor_field51.clone();
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11405.clone()), Box::new(source_ml_source2_nominal_field11407.clone()), Box::new(source_ml_source2_nominal_field11409.clone()), source_ml_source2_nominal_field11411, Box::new(source_ml_source2_nominal_field11413.clone()), source_ml_source2_nominal_field11415, source_ml_source2_nominal_field11417, source_ml_source2_nominal_field11419, source_ml_source2_nominal_field11421, source_ml_source2_nominal_field11423, source_ml_source2_nominal_field11425, Box::new(source_ml_source2_nominal_field11427.clone()), Box::new(source_ml_source2_nominal_field11429.clone()), Box::new(source_ml_source2_nominal_field11431.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field11435.clone(), source_ml_source2_nominal_field11437.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field11441.clone(), source_ml_source2_nominal_field11443.clone(), source_ml_source2_nominal_field11445.clone(), source_ml_source2_nominal_field11447.clone(), source_ml_source2_nominal_field11449.clone())
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
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesCall(
                                heap_3,
                                state_3,
                                callee,
                                receiver,
                                arguments,
                            ) => {
                                let heap_3 = *heap_3;
                                let state_3 = *state_3;
                                let callee = *callee;
                                let receiver = *receiver;
                                self::request_call(
                                    heap_3.clone(),
                                    {
                                        let source_ml_source2_nominal_field11454 =
                                            shared_constructor_field29.clone();
                                        {
                                            let source_ml_source2_nominal_field11456 =
                                                shared_constructor_field30.clone();
                                            {
                                                let source_ml_source2_nominal_field11458 =
                                                    shared_constructor_field31.clone();
                                                {
                                                    let source_ml_source2_nominal_field11460 =
                                                        shared_constructor_field32;
                                                    {
                                                        let source_ml_source2_nominal_field11462 =
                                                            shared_constructor_field33.clone();
                                                        {
                                                            let source_ml_source2_nominal_field11464 =
                                                                shared_constructor_field34;
                                                            {
                                                                let source_ml_source2_nominal_field11466 =
                                                                    shared_constructor_field35;
                                                                {
                                                                    let source_ml_source2_nominal_field11468 =
                                                                        shared_constructor_field36;
                                                                    {
                                                                        let source_ml_source2_nominal_field11470 = shared_constructor_field37;
                                                                        {
                                                                            let source_ml_source2_nominal_field11472 = shared_constructor_field38;
                                                                            {
                                                                                let source_ml_source2_nominal_field11474 = shared_constructor_field39;
                                                                                {
                                                                                    let source_ml_source2_nominal_field11476 = shared_constructor_field40.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11478 = shared_constructor_field41.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11480 = shared_constructor_field42.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11482 = shared_constructor_field43.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11484 = shared_constructor_field44.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11486 = shared_constructor_field45.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11488 = shared_constructor_field46.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11490 = shared_constructor_field47.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11492 = shared_constructor_field48.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11494 = shared_constructor_field49.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11496 = shared_constructor_field50.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field11498 = shared_constructor_field51.clone();
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11454.clone()), Box::new(source_ml_source2_nominal_field11456.clone()), Box::new(source_ml_source2_nominal_field11458.clone()), source_ml_source2_nominal_field11460, Box::new(source_ml_source2_nominal_field11462.clone()), source_ml_source2_nominal_field11464, source_ml_source2_nominal_field11466, source_ml_source2_nominal_field11468, source_ml_source2_nominal_field11470, source_ml_source2_nominal_field11472, source_ml_source2_nominal_field11474, Box::new(source_ml_source2_nominal_field11476.clone()), Box::new(source_ml_source2_nominal_field11478.clone()), Box::new(source_ml_source2_nominal_field11480.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field11484.clone(), source_ml_source2_nominal_field11486.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field11490.clone(), source_ml_source2_nominal_field11492.clone(), source_ml_source2_nominal_field11494.clone(), source_ml_source2_nominal_field11496.clone(), source_ml_source2_nominal_field11498.clone())
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
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesNeedAuthority(
                                heap_4,
                                state_4,
                                constructor,
                            ) => {
                                let heap_4 = *heap_4;
                                let state_4 = *state_4;
                                let constructor = *constructor;
                                self::JavaScriptArraySliceTransition::ArraySliceNeedAuthority(
                                    Box::new(heap_4.clone()),
                                    Box::new({
                                        let source_ml_source2_nominal_field11503 =
                                            shared_constructor_field29.clone();
                                        {
                                            let source_ml_source2_nominal_field11505 =
                                                shared_constructor_field30.clone();
                                            {
                                                let source_ml_source2_nominal_field11507 =
                                                    shared_constructor_field31.clone();
                                                {
                                                    let source_ml_source2_nominal_field11509 =
                                                        shared_constructor_field32;
                                                    {
                                                        let source_ml_source2_nominal_field11511 =
                                                            shared_constructor_field33.clone();
                                                        {
                                                            let source_ml_source2_nominal_field11513 =
                                                                shared_constructor_field34;
                                                            {
                                                                let source_ml_source2_nominal_field11515 =
                                                                    shared_constructor_field35;
                                                                {
                                                                    let source_ml_source2_nominal_field11517 =
                                                                        shared_constructor_field36;
                                                                    {
                                                                        let source_ml_source2_nominal_field11519 = shared_constructor_field37;
                                                                        {
                                                                            let source_ml_source2_nominal_field11521 = shared_constructor_field38;
                                                                            {
                                                                                let source_ml_source2_nominal_field11523 = shared_constructor_field39;
                                                                                {
                                                                                    let source_ml_source2_nominal_field11525 = shared_constructor_field40.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11527 = shared_constructor_field41.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11529 = shared_constructor_field42.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11531 = shared_constructor_field43.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11533 = shared_constructor_field44.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11535 = shared_constructor_field45.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11537 = shared_constructor_field46.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11539 = shared_constructor_field47.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11541 = shared_constructor_field48.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11543 = shared_constructor_field49.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11545 = shared_constructor_field50.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field11547 = shared_constructor_field51.clone();
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11503.clone()), Box::new(source_ml_source2_nominal_field11505.clone()), Box::new(source_ml_source2_nominal_field11507.clone()), source_ml_source2_nominal_field11509, Box::new(source_ml_source2_nominal_field11511.clone()), source_ml_source2_nominal_field11513, source_ml_source2_nominal_field11515, source_ml_source2_nominal_field11517, source_ml_source2_nominal_field11519, source_ml_source2_nominal_field11521, source_ml_source2_nominal_field11523, Box::new(source_ml_source2_nominal_field11525.clone()), Box::new(source_ml_source2_nominal_field11527.clone()), Box::new(source_ml_source2_nominal_field11529.clone()), self::ml_string::Text::from_units(vec![115, 112, 101, 99, 105, 101, 115]), source_ml_source2_nominal_field11533.clone(), source_ml_source2_nominal_field11535.clone(), Some((Some(state_4.clone()),)), source_ml_source2_nominal_field11539.clone(), source_ml_source2_nominal_field11541.clone(), source_ml_source2_nominal_field11543.clone(), source_ml_source2_nominal_field11545.clone(), source_ml_source2_nominal_field11547.clone())
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
                                    Box::new(constructor.clone()),
                                )
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesAllocate(
                                heap_5,
                                state_5,
                                prototype,
                                _,
                            ) => {
                                let heap_5 = *heap_5;
                                let state_5 = *state_5;
                                let prototype = *prototype;
                                self::allocation_request(heap_5.clone(), {
                                    let source_ml_source2_nominal_field11552 =
                                        shared_constructor_field29.clone();
                                    {
                                        let source_ml_source2_nominal_field11554 =
                                            shared_constructor_field30.clone();
                                        {
                                            let source_ml_source2_nominal_field11556 =
                                                shared_constructor_field31.clone();
                                            {
                                                let source_ml_source2_nominal_field11558 =
                                                    shared_constructor_field32;
                                                {
                                                    let source_ml_source2_nominal_field11560 =
                                                        shared_constructor_field33.clone();
                                                    {
                                                        let source_ml_source2_nominal_field11562 =
                                                            shared_constructor_field34;
                                                        {
                                                            let source_ml_source2_nominal_field11564 =
                                                                shared_constructor_field35;
                                                            {
                                                                let source_ml_source2_nominal_field11566 =
                                                                    shared_constructor_field36;
                                                                {
                                                                    let source_ml_source2_nominal_field11568 =
                                                                        shared_constructor_field37;
                                                                    {
                                                                        let source_ml_source2_nominal_field11570 = shared_constructor_field38;
                                                                        {
                                                                            let source_ml_source2_nominal_field11572 = shared_constructor_field39;
                                                                            {
                                                                                let source_ml_source2_nominal_field11574 = shared_constructor_field40.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field11576 = shared_constructor_field41.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11578 = shared_constructor_field42.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11580 = shared_constructor_field43.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11582 = shared_constructor_field44.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11584 = shared_constructor_field45.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11586 = shared_constructor_field46.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11588 = shared_constructor_field47.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11590 = shared_constructor_field48.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11592 = shared_constructor_field49.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11594 = shared_constructor_field50.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11596 = shared_constructor_field51.clone();
                                                                                                                            self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11552.clone()), Box::new(source_ml_source2_nominal_field11554.clone()), Box::new(source_ml_source2_nominal_field11556.clone()), source_ml_source2_nominal_field11558, Box::new(source_ml_source2_nominal_field11560.clone()), source_ml_source2_nominal_field11562, source_ml_source2_nominal_field11564, source_ml_source2_nominal_field11566, source_ml_source2_nominal_field11568, source_ml_source2_nominal_field11570, source_ml_source2_nominal_field11572, Box::new(source_ml_source2_nominal_field11574.clone()), Box::new(source_ml_source2_nominal_field11576.clone()), Box::new(source_ml_source2_nominal_field11578.clone()), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), source_ml_source2_nominal_field11582.clone(), source_ml_source2_nominal_field11584.clone(), Some((Some(state_5.clone()),)), Some((Some(prototype.clone()),)), source_ml_source2_nominal_field11590.clone(), source_ml_source2_nominal_field11592.clone(), source_ml_source2_nominal_field11594.clone(), source_ml_source2_nominal_field11596.clone())
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
                                })
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesConstruct(
                                heap_6,
                                state_6,
                                constructor_2,
                                arguments_2,
                                fact,
                            ) => {
                                let heap_6 = *heap_6;
                                let state_6 = *state_6;
                                let constructor_2 = *constructor_2;
                                let fact = *fact;
                                match fact.clone() {
                                    self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                        _,
                                        _,
                                        shared_constructor_field27,
                                        shared_constructor_field28,
                                        _,
                                    ) => match state_6.clone() {
                                        self::JavaScriptArraySpeciesState::ArraySpeciesState(
                                            _,
                                            _,
                                            shared_constructor_field24,
                                            shared_constructor_field25,
                                            shared_constructor_field26,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let shared_constructor_field25 =
                                                *shared_constructor_field25;
                                            let shared_constructor_field26 =
                                                *shared_constructor_field26;
                                            {
                                                let selected = {
                                                    let source_ml_source2_nominal_field11602 =
                                                        shared_constructor_field29.clone();
                                                    {
                                                        let source_ml_source2_nominal_field11604 =
                                                            shared_constructor_field30.clone();
                                                        {
                                                            let source_ml_source2_nominal_field11606 =
                                                                shared_constructor_field31.clone();
                                                            {
                                                                let source_ml_source2_nominal_field11608 =
                                                                    shared_constructor_field32;
                                                                {
                                                                    let source_ml_source2_nominal_field11610 =
                                                                        shared_constructor_field33
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field11612 = shared_constructor_field34;
                                                                        {
                                                                            let source_ml_source2_nominal_field11614 = shared_constructor_field35;
                                                                            {
                                                                                let source_ml_source2_nominal_field11616 = shared_constructor_field36;
                                                                                {
                                                                                    let source_ml_source2_nominal_field11618 = shared_constructor_field37;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11620 = shared_constructor_field38;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11622 = shared_constructor_field39;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11624 = shared_constructor_field40.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11626 = shared_constructor_field41.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11628 = shared_constructor_field42.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11630 = shared_constructor_field43.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11632 = shared_constructor_field44.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11634 = shared_constructor_field45.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11636 = shared_constructor_field46.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11638 = shared_constructor_field47.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field11640 = shared_constructor_field48.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field11642 = shared_constructor_field49.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field11644 = shared_constructor_field50.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field11646 = shared_constructor_field51.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_override11650 = self::copy_java_script_array_constructor_fact(fact.clone());
                                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11602.clone()), Box::new(source_ml_source2_nominal_field11604.clone()), Box::new(source_ml_source2_nominal_field11606.clone()), source_ml_source2_nominal_field11608, Box::new(source_ml_source2_nominal_field11610.clone()), source_ml_source2_nominal_field11612, source_ml_source2_nominal_field11614, source_ml_source2_nominal_field11616, source_ml_source2_nominal_field11618, source_ml_source2_nominal_field11620, source_ml_source2_nominal_field11622, Box::new(source_ml_source2_nominal_field11624.clone()), Box::new(source_ml_source2_nominal_field11626.clone()), Box::new(source_ml_source2_nominal_field11628.clone()), source_ml_source2_nominal_field11630.clone(), source_ml_source2_nominal_field11632.clone(), source_ml_source2_nominal_field11634.clone(), Some((Some(state_6.clone()),)), source_ml_source2_nominal_field11638.clone(), Some((Some(constructor_2.clone()),)), Some((Some(source_ml_source2_nominal_override11650.clone()),)), source_ml_source2_nominal_field11644.clone(), source_ml_source2_nominal_field11646.clone())
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
                                                match selected.clone() {
                                                        self::JavaScriptArraySliceState::ArraySliceState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14, shared_constructor_field15, shared_constructor_field16, shared_constructor_field17, shared_constructor_field18, shared_constructor_field19, shared_constructor_field20, shared_constructor_field21, shared_constructor_field22, shared_constructor_field23) => {
                                                            let shared_constructor_field1 = *shared_constructor_field1;
                                                            let shared_constructor_field2 = *shared_constructor_field2;
                                                            let shared_constructor_field3 = *shared_constructor_field3;
                                                            let shared_constructor_field5 = *shared_constructor_field5;
                                                            let shared_constructor_field12 = *shared_constructor_field12;
                                                            let shared_constructor_field13 = *shared_constructor_field13;
                                                            let shared_constructor_field14 = *shared_constructor_field14;
                                                            if ((shared_constructor_field28 && (shared_constructor_field27 == shared_constructor_field24)) && self::test_java_script_semantic_same_value(constructor_2.clone(), shared_constructor_field25.clone())) {
                                                                self::allocation_request(heap_6.clone(), {
                                                                    let source_ml_source2_nominal_field11659 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field11661 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field11663 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field11665 = shared_constructor_field4;
                                                                                {
                                                                                    let source_ml_source2_nominal_field11667 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11669 = shared_constructor_field6;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11671 = shared_constructor_field7;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11673 = shared_constructor_field8;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11675 = shared_constructor_field9;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11677 = shared_constructor_field10;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11679 = shared_constructor_field11;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11681 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11683 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11685 = shared_constructor_field14.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11687 = shared_constructor_field15.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field11689 = shared_constructor_field16.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field11691 = shared_constructor_field17.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field11693 = shared_constructor_field18.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field11695 = shared_constructor_field19.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field11697 = shared_constructor_field20.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field11699 = shared_constructor_field21.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field11701 = shared_constructor_field22.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field11703 = shared_constructor_field23.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_override11706 = shared_constructor_field26.clone();
                                                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11659.clone()), Box::new(source_ml_source2_nominal_field11661.clone()), Box::new(source_ml_source2_nominal_field11663.clone()), source_ml_source2_nominal_field11665, Box::new(source_ml_source2_nominal_field11667.clone()), source_ml_source2_nominal_field11669, source_ml_source2_nominal_field11671, source_ml_source2_nominal_field11673, source_ml_source2_nominal_field11675, source_ml_source2_nominal_field11677, source_ml_source2_nominal_field11679, Box::new(source_ml_source2_nominal_field11681.clone()), Box::new(source_ml_source2_nominal_field11683.clone()), Box::new(source_ml_source2_nominal_field11685.clone()), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101]), source_ml_source2_nominal_field11689.clone(), source_ml_source2_nominal_field11691.clone(), source_ml_source2_nominal_field11693.clone(), Some((Some(source_ml_source2_nominal_override11706.clone()),)), source_ml_source2_nominal_field11697.clone(), source_ml_source2_nominal_field11699.clone(), source_ml_source2_nominal_field11701.clone(), source_ml_source2_nominal_field11703.clone())
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
                                                                })
                                                            } else {
                                                                self::JavaScriptArraySliceTransition::ArraySliceConstruct(Box::new(heap_6.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field11708 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field11710 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field11712 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field11714 = shared_constructor_field4;
                                                                                {
                                                                                    let source_ml_source2_nominal_field11716 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field11718 = shared_constructor_field6;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field11720 = shared_constructor_field7;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field11722 = shared_constructor_field8;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field11724 = shared_constructor_field9;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field11726 = shared_constructor_field10;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field11728 = shared_constructor_field11;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field11730 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field11732 = shared_constructor_field13.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field11734 = shared_constructor_field14.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field11736 = shared_constructor_field15.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field11738 = shared_constructor_field16.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field11740 = shared_constructor_field17.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field11742 = shared_constructor_field18.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field11744 = shared_constructor_field19.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field11746 = shared_constructor_field20.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field11748 = shared_constructor_field21.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field11750 = shared_constructor_field22.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field11752 = shared_constructor_field23.clone();
                                                                                                                                                            self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field11708.clone()), Box::new(source_ml_source2_nominal_field11710.clone()), Box::new(source_ml_source2_nominal_field11712.clone()), source_ml_source2_nominal_field11714, Box::new(source_ml_source2_nominal_field11716.clone()), source_ml_source2_nominal_field11718, source_ml_source2_nominal_field11720, source_ml_source2_nominal_field11722, source_ml_source2_nominal_field11724, source_ml_source2_nominal_field11726, source_ml_source2_nominal_field11728, Box::new(source_ml_source2_nominal_field11730.clone()), Box::new(source_ml_source2_nominal_field11732.clone()), Box::new(source_ml_source2_nominal_field11734.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), source_ml_source2_nominal_field11738.clone(), source_ml_source2_nominal_field11740.clone(), source_ml_source2_nominal_field11742.clone(), source_ml_source2_nominal_field11744.clone(), source_ml_source2_nominal_field11746.clone(), source_ml_source2_nominal_field11748.clone(), source_ml_source2_nominal_field11750.clone(), source_ml_source2_nominal_field11752.clone())
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
                                                                }), Box::new(constructor_2.clone()), arguments_2.clone(), Box::new(self::copy_java_script_array_constructor_fact(fact.clone())))
                                                            }
                                                        }
                                                    }
                                            }
                                        }
                                    },
                                }
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesThrow(
                                heap_7,
                                value,
                            ) => {
                                let heap_7 = *heap_7;
                                let value = *value;
                                self::JavaScriptArraySliceTransition::ArraySliceThrow(
                                    Box::new(heap_7.clone()),
                                    Box::new(value.clone()),
                                )
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesError(
                                heap_8,
                                error_class,
                                reason,
                            ) => {
                                let heap_8 = *heap_8;
                                self::JavaScriptArraySliceTransition::ArraySliceError(
                                    Box::new(heap_8.clone()),
                                    error_class.clone(),
                                    reason.clone(),
                                )
                            }
                            self::JavaScriptArraySpeciesTransition::ArraySpeciesInvalid(
                                heap_9,
                                reason_2,
                            ) => {
                                let heap_9 = *heap_9;
                                self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                                    Box::new(heap_9.clone()),
                                    reason_2.clone(),
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
