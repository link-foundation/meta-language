// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArraySliceState;
use super::JavaScriptArraySliceTransition;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptIndexedPropertyCreation;
use super::JavaScriptObjectCopyCall;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptArraySlicePropertyAccessLoop366Result;
use super::accept_scalar;
use super::begin_java_script_scalar_protocol;
use super::create_java_script_indexed_data_property;
use super::ml;
use super::ml_number;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_indexed_property_presence;
use super::request_call;
use super::source_ml_source1_temporary_step_java_script_array_slice_property_access_loop366;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_slice_property_access(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    state: self::JavaScriptArraySliceState,
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
            if {
                let source_ml_source2_s12373 = shared_constructor_field1.clone();
                match source_ml_source2_s12373.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                }
            } {
                self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        115, 108, 105, 99, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                        105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 79, 98, 106,
                        101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114,
                    ]),
                )
            } else {
                if (shared_constructor_field15
                    == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104])))
                {
                    {
                        let get = self::plan_java_script_ordinary_get(
                            heap.clone(),
                            (((match shared_constructor_field1.clone() {
                                self::JavaScriptSemanticValue::Object(
                                    source_ml_source2_data_property_value12380,
                                    _,
                                ) => Some((source_ml_source2_data_property_value12380.clone(),)),
                                _ => None::<(self::ml::Big,)>,
                            })
                            .clone()
                            .expect("a proven present value"))
                            .0)
                                .clone(),
                            self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                std::cell::RefCell::new(vec![
                                    108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
                                ]),
                            )),
                            shared_constructor_field1.clone(),
                        );
                        if (((({
                            let ml_reference_value760 = get.borrow().clone();
                            ml_reference_value760
                        })
                        .0)
                            .3)
                            != (self::ml_string::Text::from_units(vec![])))
                        {
                            self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                                Box::new(heap.clone()),
                                (({
                                    let ml_reference_value761 = get.borrow().clone();
                                    ml_reference_value761
                                })
                                .0)
                                    .3,
                            )
                        } else {
                            if (((({
                                let ml_reference_value762 = get.borrow().clone();
                                ml_reference_value762
                            })
                            .0)
                                .0)
                                == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))
                            {
                                self::request_call(
                                    heap.clone(),
                                    {
                                        let source_ml_source2_nominal_field12383 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field12385 =
                                                shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field12387 =
                                                    shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field12389 =
                                                        shared_constructor_field4;
                                                    {
                                                        let source_ml_source2_nominal_field12391 =
                                                            shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field12393 =
                                                                shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field12395 =
                                                                    shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field12397 =
                                                                        shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field12399 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field12401 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field12403 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12405 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12407 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12409 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12411 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12413 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12415 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12417 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12419 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12421 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12423 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12425 = shared_constructor_field22.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12427 = shared_constructor_field23.clone();
                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12383.clone()), Box::new(source_ml_source2_nominal_field12385.clone()), Box::new(source_ml_source2_nominal_field12387.clone()), source_ml_source2_nominal_field12389, Box::new(source_ml_source2_nominal_field12391.clone()), source_ml_source2_nominal_field12393, source_ml_source2_nominal_field12395, source_ml_source2_nominal_field12397, source_ml_source2_nominal_field12399, source_ml_source2_nominal_field12401, source_ml_source2_nominal_field12403, Box::new(source_ml_source2_nominal_field12405.clone()), Box::new(source_ml_source2_nominal_field12407.clone()), Box::new(source_ml_source2_nominal_field12409.clone()), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field12413.clone(), source_ml_source2_nominal_field12415.clone(), source_ml_source2_nominal_field12417.clone(), source_ml_source2_nominal_field12419.clone(), source_ml_source2_nominal_field12421.clone(), source_ml_source2_nominal_field12423.clone(), source_ml_source2_nominal_field12425.clone(), source_ml_source2_nominal_field12427.clone())
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
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
                                    (({
                                        let ml_reference_value763 = get.borrow().clone();
                                        ml_reference_value763
                                    })
                                    .0)
                                        .2,
                                    (({
                                        let ml_reference_value764 = get.borrow().clone();
                                        ml_reference_value764
                                    })
                                    .0)
                                        .4,
                                    (({
                                        let ml_reference_value765 = get.borrow().clone();
                                        ml_reference_value765
                                    })
                                    .0)
                                        .1,
                                )
                            } else {
                                if (((({
                                    let ml_reference_value766 = get.borrow().clone();
                                    ml_reference_value766
                                })
                                .0)
                                    .0)
                                    != (self::ml_string::Text::from_units(vec![
                                        118, 97, 108, 117, 101,
                                    ])))
                                {
                                    self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            115, 108, 105, 99, 101, 32, 108, 101, 110, 103, 116,
                                            104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 71, 101,
                                            116, 32, 114, 101, 115, 117, 108, 116,
                                        ]),
                                    )
                                } else {
                                    self::accept_scalar(
                                        heap.clone(),
                                        directory.clone(),
                                        {
                                            let source_ml_source2_nominal_field12431 =
                                                shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field12433 =
                                                    shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field12435 =
                                                        shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field12437 =
                                                            shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field12439 =
                                                                shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field12441 =
                                                                    shared_constructor_field6;
                                                                {
                                                                    let source_ml_source2_nominal_field12443 =
                                                                        shared_constructor_field7;
                                                                    {
                                                                        let source_ml_source2_nominal_field12445 = shared_constructor_field8;
                                                                        {
                                                                            let source_ml_source2_nominal_field12447 = shared_constructor_field9;
                                                                            {
                                                                                let source_ml_source2_nominal_field12449 = shared_constructor_field10;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12451 = shared_constructor_field11;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12453 = shared_constructor_field12.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12455 = shared_constructor_field13.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12457 = shared_constructor_field14.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12459 = shared_constructor_field15.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12461 = shared_constructor_field16.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12463 = shared_constructor_field17.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12465 = shared_constructor_field18.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12467 = shared_constructor_field19.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12469 = shared_constructor_field20.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12471 = shared_constructor_field21.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12473 = shared_constructor_field22.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12475 = shared_constructor_field23.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override12477 = (({ let ml_reference_value767 = get.borrow().clone(); ml_reference_value767 }).0).5;
                                                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12431.clone()), Box::new(source_ml_source2_nominal_field12433.clone()), Box::new(source_ml_source2_nominal_field12435.clone()), source_ml_source2_nominal_field12437, Box::new(source_ml_source2_nominal_field12439.clone()), source_ml_source2_nominal_field12441, source_ml_source2_nominal_field12443, source_ml_source2_nominal_field12445, source_ml_source2_nominal_field12447, source_ml_source2_nominal_field12449, source_ml_source2_nominal_field12451, Box::new(source_ml_source2_nominal_field12453.clone()), Box::new(source_ml_source2_nominal_field12455.clone()), Box::new(source_ml_source2_nominal_override12477.clone()), source_ml_source2_nominal_field12459.clone(), source_ml_source2_nominal_field12461.clone(), source_ml_source2_nominal_field12463.clone(), source_ml_source2_nominal_field12465.clone(), source_ml_source2_nominal_field12467.clone(), source_ml_source2_nominal_field12469.clone(), source_ml_source2_nominal_field12471.clone(), source_ml_source2_nominal_field12473.clone(), source_ml_source2_nominal_field12475.clone())
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                                        self::begin_java_script_scalar_protocol(
                                            heap.clone(),
                                            (({
                                                let ml_reference_value768 = get.borrow().clone();
                                                ml_reference_value768
                                            })
                                            .0)
                                                .5,
                                            shared_constructor_field5.clone(),
                                            &(self::ml_string::Text::from_units(vec![
                                                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109,
                                                105, 116, 105, 118, 101, 32, 110, 117, 109, 98,
                                                101, 114,
                                            ])),
                                        ),
                                    )
                                }
                            }
                        }
                    }
                } else {
                    if (shared_constructor_field15
                        == (self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101])))
                    {
                        {
                            let written = self::create_java_script_indexed_data_property(
                                heap.clone(),
                                shared_constructor_field12.clone(),
                                shared_constructor_field11,
                                shared_constructor_field13.clone(),
                            );
                            match written.clone() {
                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyCreated(heap_2) => {
                                    let heap_2 = *heap_2;
                                    self::JavaScriptArraySliceTransition::ArraySliceContinue(Box::new(heap_2.clone()), Box::new({
                                        let source_ml_source2_nominal_field12487 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field12489 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field12491 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field12493 = shared_constructor_field4;
                                                    {
                                                        let source_ml_source2_nominal_field12495 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field12497 = shared_constructor_field6;
                                                            {
                                                                let source_ml_source2_nominal_field12499 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field12501 = shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field12503 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field12505 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field12507 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12509 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12511 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12513 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12515 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12517 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12519 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12521 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12523 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12525 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12527 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12529 = shared_constructor_field22.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12531 = shared_constructor_field23.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override12534 = (shared_constructor_field10 + 1f64);
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override12535 = (shared_constructor_field11 + 1f64);
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override12536 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                            self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12487.clone()), Box::new(source_ml_source2_nominal_field12489.clone()), Box::new(source_ml_source2_nominal_field12491.clone()), source_ml_source2_nominal_field12493, Box::new(source_ml_source2_nominal_field12495.clone()), source_ml_source2_nominal_field12497, source_ml_source2_nominal_field12499, source_ml_source2_nominal_field12501, source_ml_source2_nominal_field12503, source_ml_source2_nominal_override12534, source_ml_source2_nominal_override12535, Box::new(source_ml_source2_nominal_field12509.clone()), Box::new(source_ml_source2_nominal_override12536.clone()), Box::new(source_ml_source2_nominal_field12513.clone()), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field12517.clone(), source_ml_source2_nominal_field12519.clone(), source_ml_source2_nominal_field12521.clone(), source_ml_source2_nominal_field12523.clone(), source_ml_source2_nominal_field12525.clone(), source_ml_source2_nominal_field12527.clone(), source_ml_source2_nominal_field12529.clone(), source_ml_source2_nominal_field12531.clone())
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
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
                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(heap_3, reason) => {
                                    let heap_3 = *heap_3;
                                    self::JavaScriptArraySliceTransition::ArraySliceError(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), reason.clone())
                                }
                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(heap_4, reason_2) => {
                                    let heap_4 = *heap_4;
                                    self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap_4.clone()), reason_2.clone())
                                }
                                self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(heap_5, reason_3) => {
                                    let heap_5 = *heap_5;
                                    self::JavaScriptArraySliceTransition::ArraySliceResourceLimit(Box::new(heap_5.clone()), Box::new(state.clone()), reason_3.clone())
                                }
                            }
                        }
                    } else {
                        if (shared_constructor_field15
                            != (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121])))
                        {
                            self::JavaScriptArraySliceTransition::ArraySliceInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    115, 108, 105, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115,
                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 104,
                                    105, 108, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111,
                                    110,
                                ]),
                            )
                        } else {
                            if (shared_constructor_field11 == shared_constructor_field9) {
                                {
                                    let assignment = {
                                        let source_ml_source2_constructor_field12544 =
                                            shared_constructor_field12.clone();
                                        {
                                            let source_ml_source2_constructor_field12546 =
                                                shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_constructor_field12547 =
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        Vec::<self::JavaScriptSemanticValue>::new(),
                                                    ));
                                                {
                                                    let source_ml_source2_constructor_field12548 =
                                                        std::rc::Rc::new(std::cell::RefCell::new(
                                                            vec![
                                                                self::JavaScriptSemanticValue::Text(
                                                                    std::rc::Rc::new(
                                                                        std::cell::RefCell::new(
                                                                            vec![
                                                                                108f64, 101f64,
                                                                                110f64, 103f64,
                                                                                116f64, 104f64,
                                                                            ],
                                                                        ),
                                                                    ),
                                                                ),
                                                            ],
                                                        ));
                                                    {
                                                        let source_ml_source2_constructor_field12549 =
                                                            0f64;
                                                        {
                                                            let source_ml_source2_constructor_field12552 = self::JavaScriptSemanticValue::Number(shared_constructor_field9);
                                                            {
                                                                let source_ml_source2_constructor_field12553 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source2_constructor_field12554 = Some(self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116]));
                                                                    {
                                                                        let source_ml_source2_constructor_field12556 = Some(shared_constructor_field5.clone());
                                                                        self::JavaScriptObjectSpreadState::ObjectSpreadState(Box::new(source_ml_source2_constructor_field12544.clone()), Box::new(source_ml_source2_constructor_field12546.clone()), source_ml_source2_constructor_field12547.clone(), source_ml_source2_constructor_field12548.clone(), source_ml_source2_constructor_field12549, self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), Box::new(source_ml_source2_constructor_field12552.clone()), Box::new(source_ml_source2_constructor_field12553.clone()), Some((source_ml_source2_constructor_field12554.clone(),)), Some((source_ml_source2_constructor_field12556.clone(),)), None::<(Option<self::JavaScriptArrayAssignmentState>,)>, None::<(Option<self::JavaScriptObjectCopyCall>,)>)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    self::JavaScriptArraySliceTransition::ArraySliceContinue(
                                        Box::new(heap.clone()),
                                        Box::new({
                                            let source_ml_source2_nominal_field12558 =
                                                shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field12560 =
                                                    shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field12562 =
                                                        shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field12564 =
                                                            shared_constructor_field4;
                                                        {
                                                            let source_ml_source2_nominal_field12566 =
                                                                shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field12568 =
                                                                    shared_constructor_field6;
                                                                {
                                                                    let source_ml_source2_nominal_field12570 =
                                                                        shared_constructor_field7;
                                                                    {
                                                                        let source_ml_source2_nominal_field12572 = shared_constructor_field8;
                                                                        {
                                                                            let source_ml_source2_nominal_field12574 = shared_constructor_field9;
                                                                            {
                                                                                let source_ml_source2_nominal_field12576 = shared_constructor_field10;
                                                                                {
                                                                                    let source_ml_source2_nominal_field12578 = shared_constructor_field11;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12580 = shared_constructor_field12.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12582 = shared_constructor_field13.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12584 = shared_constructor_field14.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12586 = shared_constructor_field15.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12588 = shared_constructor_field16.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12590 = shared_constructor_field17.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12592 = shared_constructor_field18.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12594 = shared_constructor_field19.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12596 = shared_constructor_field20.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12598 = shared_constructor_field21.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12600 = shared_constructor_field22.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12602 = shared_constructor_field23.clone();
                                                                                                                                    self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12558.clone()), Box::new(source_ml_source2_nominal_field12560.clone()), Box::new(source_ml_source2_nominal_field12562.clone()), source_ml_source2_nominal_field12564, Box::new(source_ml_source2_nominal_field12566.clone()), source_ml_source2_nominal_field12568, source_ml_source2_nominal_field12570, source_ml_source2_nominal_field12572, source_ml_source2_nominal_field12574, source_ml_source2_nominal_field12576, source_ml_source2_nominal_field12578, Box::new(source_ml_source2_nominal_field12580.clone()), Box::new(source_ml_source2_nominal_field12582.clone()), Box::new(source_ml_source2_nominal_field12584.clone()), self::ml_string::Text::from_units(vec![102, 105, 110, 105, 115, 104]), source_ml_source2_nominal_field12588.clone(), source_ml_source2_nominal_field12590.clone(), source_ml_source2_nominal_field12592.clone(), source_ml_source2_nominal_field12594.clone(), source_ml_source2_nominal_field12596.clone(), source_ml_source2_nominal_field12598.clone(), Some((Some(assignment.clone()),)), source_ml_source2_nominal_field12602.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                            } else {
                                {
                                    let name = {
                                        let source_ml_source2_string_argument12607 =
                                            shared_constructor_field10;
                                        self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                source_ml_source2_string_argument12607,
                                            ),
                                        )
                                    };
                                    {
                                        let units = std::rc::Rc::new(std::cell::RefCell::new(
                                            Vec::<f64>::new(),
                                        ));
                                        {
                                            let at = 0f64;
                                            {
                                                let source_ml_source2_s12608 = self::source_ml_source1_temporary_step_java_script_array_slice_property_access_loop366(&(name), units.clone(), at);
                                                match source_ml_source2_s12608.clone() {
                                                    self::MlSource1TemporaryStepJavaScriptArraySlicePropertyAccessLoop366Result::MlSource1TemporaryStepJavaScriptArraySlicePropertyAccessLoop366Done => {
                                                        {
                                                            let key = self::JavaScriptSemanticValue::Text(units.clone());
                                                            {
                                                                let presence = self::read_java_script_indexed_property_presence(heap.clone(), (((match shared_constructor_field1.clone() {
                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value12610, _) => {
                                                                        Some((source_ml_source2_data_property_value12610.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::ml::Big,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone(), key.clone());
                                                                if (((({ let ml_reference_value769 = presence.borrow().clone(); ml_reference_value769 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap.clone()), (({ let ml_reference_value770 = presence.borrow().clone(); ml_reference_value770 }).0).0)
                                                                } else {
                                                                    if !((({ let ml_reference_value771 = presence.borrow().clone(); ml_reference_value771 }).0).1) {
                                                                        self::JavaScriptArraySliceTransition::ArraySliceContinue(Box::new(heap.clone()), Box::new({
                                                                            let source_ml_source2_nominal_field12614 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field12616 = shared_constructor_field2.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field12618 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field12620 = shared_constructor_field4;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12622 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12624 = shared_constructor_field6;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12626 = shared_constructor_field7;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12628 = shared_constructor_field8;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12630 = shared_constructor_field9;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12632 = shared_constructor_field10;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12634 = shared_constructor_field11;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12636 = shared_constructor_field12.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12638 = shared_constructor_field13.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12640 = shared_constructor_field14.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12642 = shared_constructor_field15.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12644 = shared_constructor_field16.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field12646 = shared_constructor_field17.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field12648 = shared_constructor_field18.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field12650 = shared_constructor_field19.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field12652 = shared_constructor_field20.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field12654 = shared_constructor_field21.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field12656 = shared_constructor_field22.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field12658 = shared_constructor_field23.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_override12660 = (shared_constructor_field10 + 1f64);
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_override12661 = (shared_constructor_field11 + 1f64);
                                                                                                                                                                            self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12614.clone()), Box::new(source_ml_source2_nominal_field12616.clone()), Box::new(source_ml_source2_nominal_field12618.clone()), source_ml_source2_nominal_field12620, Box::new(source_ml_source2_nominal_field12622.clone()), source_ml_source2_nominal_field12624, source_ml_source2_nominal_field12626, source_ml_source2_nominal_field12628, source_ml_source2_nominal_field12630, source_ml_source2_nominal_override12660, source_ml_source2_nominal_override12661, Box::new(source_ml_source2_nominal_field12636.clone()), Box::new(source_ml_source2_nominal_field12638.clone()), Box::new(source_ml_source2_nominal_field12640.clone()), source_ml_source2_nominal_field12642.clone(), source_ml_source2_nominal_field12644.clone(), source_ml_source2_nominal_field12646.clone(), source_ml_source2_nominal_field12648.clone(), source_ml_source2_nominal_field12650.clone(), source_ml_source2_nominal_field12652.clone(), source_ml_source2_nominal_field12654.clone(), source_ml_source2_nominal_field12656.clone(), source_ml_source2_nominal_field12658.clone())
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
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
                                                                    } else {
                                                                        {
                                                                            let get_2 = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field1.clone() {
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value12663, _) => {
                                                                                    Some((source_ml_source2_data_property_value12663.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(self::ml::Big,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone(), key.clone(), shared_constructor_field1.clone());
                                                                            if (((({ let ml_reference_value772 = get_2.borrow().clone(); ml_reference_value772 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap.clone()), (({ let ml_reference_value773 = get_2.borrow().clone(); ml_reference_value773 }).0).3)
                                                                            } else {
                                                                                if (((({ let ml_reference_value774 = get_2.borrow().clone(); ml_reference_value774 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                                    self::request_call(heap.clone(), {
                                                                                        let source_ml_source2_nominal_field12666 = shared_constructor_field1.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field12668 = shared_constructor_field2.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12670 = shared_constructor_field3.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12672 = shared_constructor_field4;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12674 = shared_constructor_field5.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12676 = shared_constructor_field6;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12678 = shared_constructor_field7;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12680 = shared_constructor_field8;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12682 = shared_constructor_field9;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12684 = shared_constructor_field10;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12686 = shared_constructor_field11;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12688 = shared_constructor_field12.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12690 = shared_constructor_field13.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field12692 = shared_constructor_field14.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field12694 = shared_constructor_field15.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field12696 = shared_constructor_field16.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field12698 = shared_constructor_field17.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field12700 = shared_constructor_field18.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field12702 = shared_constructor_field19.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field12704 = shared_constructor_field20.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field12706 = shared_constructor_field21.clone();
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field12708 = shared_constructor_field22.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_field12710 = shared_constructor_field23.clone();
                                                                                                                                                                                self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12666.clone()), Box::new(source_ml_source2_nominal_field12668.clone()), Box::new(source_ml_source2_nominal_field12670.clone()), source_ml_source2_nominal_field12672, Box::new(source_ml_source2_nominal_field12674.clone()), source_ml_source2_nominal_field12676, source_ml_source2_nominal_field12678, source_ml_source2_nominal_field12680, source_ml_source2_nominal_field12682, source_ml_source2_nominal_field12684, source_ml_source2_nominal_field12686, Box::new(source_ml_source2_nominal_field12688.clone()), Box::new(source_ml_source2_nominal_field12690.clone()), Box::new(source_ml_source2_nominal_field12692.clone()), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field12696.clone(), source_ml_source2_nominal_field12698.clone(), source_ml_source2_nominal_field12700.clone(), source_ml_source2_nominal_field12702.clone(), source_ml_source2_nominal_field12704.clone(), source_ml_source2_nominal_field12706.clone(), source_ml_source2_nominal_field12708.clone(), source_ml_source2_nominal_field12710.clone())
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }, (({ let ml_reference_value775 = get_2.borrow().clone(); ml_reference_value775 }).0).2, (({ let ml_reference_value776 = get_2.borrow().clone(); ml_reference_value776 }).0).4, (({ let ml_reference_value777 = get_2.borrow().clone(); ml_reference_value777 }).0).1)
                                                                                } else {
                                                                                    if (((({ let ml_reference_value778 = get_2.borrow().clone(); ml_reference_value778 }).0).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                                        self::JavaScriptArraySliceTransition::ArraySliceContinue(Box::new(heap.clone()), Box::new({
                                                                                            let source_ml_source2_nominal_field12714 = shared_constructor_field1.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field12716 = shared_constructor_field2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field12718 = shared_constructor_field3.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field12720 = shared_constructor_field4;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field12722 = shared_constructor_field5.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field12724 = shared_constructor_field6;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field12726 = shared_constructor_field7;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field12728 = shared_constructor_field8;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field12730 = shared_constructor_field9;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field12732 = shared_constructor_field10;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field12734 = shared_constructor_field11;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field12736 = shared_constructor_field12.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field12738 = shared_constructor_field13.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field12740 = shared_constructor_field14.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field12742 = shared_constructor_field15.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field12744 = shared_constructor_field16.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field12746 = shared_constructor_field17.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field12748 = shared_constructor_field18.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field12750 = shared_constructor_field19.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field12752 = shared_constructor_field20.clone();
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field12754 = shared_constructor_field21.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_field12756 = shared_constructor_field22.clone();
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_source2_nominal_field12758 = shared_constructor_field23.clone();
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_source2_nominal_override12761 = (({ let ml_reference_value779 = get_2.borrow().clone(); ml_reference_value779 }).0).5;
                                                                                                                                                                                        self::JavaScriptArraySliceState::ArraySliceState(Box::new(source_ml_source2_nominal_field12714.clone()), Box::new(source_ml_source2_nominal_field12716.clone()), Box::new(source_ml_source2_nominal_field12718.clone()), source_ml_source2_nominal_field12720, Box::new(source_ml_source2_nominal_field12722.clone()), source_ml_source2_nominal_field12724, source_ml_source2_nominal_field12726, source_ml_source2_nominal_field12728, source_ml_source2_nominal_field12730, source_ml_source2_nominal_field12732, source_ml_source2_nominal_field12734, Box::new(source_ml_source2_nominal_field12736.clone()), Box::new(source_ml_source2_nominal_override12761.clone()), Box::new(source_ml_source2_nominal_field12740.clone()), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), source_ml_source2_nominal_field12744.clone(), source_ml_source2_nominal_field12746.clone(), source_ml_source2_nominal_field12748.clone(), source_ml_source2_nominal_field12750.clone(), source_ml_source2_nominal_field12752.clone(), source_ml_source2_nominal_field12754.clone(), source_ml_source2_nominal_field12756.clone(), source_ml_source2_nominal_field12758.clone())
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
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
                                                                                    } else {
                                                                                        self::JavaScriptArraySliceTransition::ArraySliceInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 101, 108, 101, 109, 101, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 71, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
