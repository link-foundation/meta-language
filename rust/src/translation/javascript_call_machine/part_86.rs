// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptIndexedPropertyCreation;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::accept_java_script_array_creation_length_set;
use super::begin_java_script_array_assignment;
use super::construct_java_script_array_creation_index;
use super::create_java_script_indexed_data_property;
use super::fail_java_script_array_creation;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_set_with_array_definition;
use super::read_java_script_array_creation_property;
use super::request_java_script_array_creation_call;
use super::select_java_script_array_creation_value;
use super::step_java_script_array_assignment;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_creation_result(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
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
            if (shared_constructor_field12
                == (self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116])))
            {
                if (shared_constructor_field13 >= shared_constructor_field14) {
                    self::JavaScriptArrayCreationTransition::ArrayCreationContinue(
                        Box::new(heap.clone()),
                        Box::new({
                            let source_ml_source2_nominal_field16652 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field16654 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field16656 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field16658 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field16660 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field16662 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field16664 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16666 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16668 =
                                                                shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16670 =
                                                                    shared_constructor_field10;
                                                                {
                                                                    let source_ml_source2_nominal_field16672 =
                                                                        shared_constructor_field11;
                                                                    {
                                                                        let source_ml_source2_nominal_field16674 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field16676 = shared_constructor_field13;
                                                                            {
                                                                                let source_ml_source2_nominal_field16678 = shared_constructor_field14;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16680 = shared_constructor_field15.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16682 = shared_constructor_field16.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16684 = shared_constructor_field17.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16686 = shared_constructor_field18.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16688 = shared_constructor_field19.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16690 = shared_constructor_field20.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16692 = shared_constructor_field21.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16694 = shared_constructor_field22.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16696 = shared_constructor_field23.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16698 = shared_constructor_field24.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16700 = shared_constructor_field25.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16702 = shared_constructor_field26.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16704 = shared_constructor_field27.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16706 = shared_constructor_field28.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16708 = shared_constructor_field29.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16710 = shared_constructor_field30.clone();
                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16652.clone(), Box::new(source_ml_source2_nominal_field16654.clone()), source_ml_source2_nominal_field16656.clone(), Box::new(source_ml_source2_nominal_field16658.clone()), Box::new(source_ml_source2_nominal_field16660.clone()), Box::new(source_ml_source2_nominal_field16662.clone()), Box::new(source_ml_source2_nominal_field16664.clone()), Box::new(source_ml_source2_nominal_field16666.clone()), Box::new(source_ml_source2_nominal_field16668.clone()), source_ml_source2_nominal_field16670, source_ml_source2_nominal_field16672, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116]), source_ml_source2_nominal_field16676, source_ml_source2_nominal_field16678, Box::new(source_ml_source2_nominal_field16680.clone()), Box::new(source_ml_source2_nominal_field16682.clone()), Box::new(source_ml_source2_nominal_field16684.clone()), Box::new(source_ml_source2_nominal_field16686.clone()), Box::new(source_ml_source2_nominal_field16688.clone()), source_ml_source2_nominal_field16690.clone(), source_ml_source2_nominal_field16692.clone(), source_ml_source2_nominal_field16694.clone(), source_ml_source2_nominal_field16696.clone(), source_ml_source2_nominal_field16698.clone(), source_ml_source2_nominal_field16700.clone(), source_ml_source2_nominal_field16702.clone(), source_ml_source2_nominal_field16704.clone(), source_ml_source2_nominal_field16706.clone(), source_ml_source2_nominal_field16708.clone(), source_ml_source2_nominal_field16710.clone())
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                    if (shared_constructor_field1
                        == (self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 46, 111, 102,
                        ])))
                    {
                        self::select_java_script_array_creation_value(
                            heap.clone(),
                            state.clone(),
                            ({
                                let source_ml_source2_array_read_value16717 =
                                    shared_constructor_field3.clone();
                                {
                                    let source_ml_source2_array_read_index16718 =
                                        shared_constructor_field13;
                                    {
                                        let ml_reference_array862 =
                                            source_ml_source2_array_read_value16717.clone();
                                        let ml_reference_position862 =
                                            source_ml_source2_array_read_index16718;
                                        let ml_reference_projection862 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array862.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position862,
                                                ),
                                            );
                                        ml_reference_projection862
                                    }
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                        )
                    } else {
                        self::read_java_script_array_creation_property(
                            heap.clone(),
                            state.clone(),
                            shared_constructor_field16.clone(),
                            self::construct_java_script_array_creation_index(
                                shared_constructor_field13,
                            ),
                            &(self::ml_string::Text::from_units(vec![
                                101, 108, 101, 109, 101, 110, 116,
                            ])),
                            shared_constructor_field16.clone(),
                        )
                    }
                }
            } else {
                if (shared_constructor_field12
                    == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101])))
                {
                    {
                        let written = self::create_java_script_indexed_data_property(
                            heap.clone(),
                            shared_constructor_field17.clone(),
                            shared_constructor_field13,
                            shared_constructor_field18.clone(),
                        );
                        match written.clone() {
                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyCreated(heap_2) => {
                                let heap_2 = *heap_2;
                                self::JavaScriptArrayCreationTransition::ArrayCreationContinue(Box::new(heap_2.clone()), Box::new({
                                    let source_ml_source2_nominal_field16730 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field16732 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field16734 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field16736 = shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field16738 = shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16740 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16742 = shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16744 = shared_constructor_field8.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16746 = shared_constructor_field9.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field16748 = shared_constructor_field10;
                                                                        {
                                                                            let source_ml_source2_nominal_field16750 = shared_constructor_field11;
                                                                            {
                                                                                let source_ml_source2_nominal_field16752 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field16754 = shared_constructor_field13;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16756 = shared_constructor_field14;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16758 = shared_constructor_field15.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16760 = shared_constructor_field16.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16762 = shared_constructor_field17.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16764 = shared_constructor_field18.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16766 = shared_constructor_field19.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16768 = shared_constructor_field20.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16770 = shared_constructor_field21.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16772 = shared_constructor_field22.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16774 = shared_constructor_field23.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16776 = shared_constructor_field24.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16778 = shared_constructor_field25.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16780 = shared_constructor_field26.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16782 = shared_constructor_field27.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16784 = shared_constructor_field28.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16786 = shared_constructor_field29.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field16788 = shared_constructor_field30.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_override16790 = (shared_constructor_field13 + 1f64);
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_override16791 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_override16792 = if shared_constructor_field11 {
                                                                                                                                                                        self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 101])
                                                                                                                                                                    } else {
                                                                                                                                                                        self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116])
                                                                                                                                                                    };
                                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16730.clone(), Box::new(source_ml_source2_nominal_field16732.clone()), source_ml_source2_nominal_field16734.clone(), Box::new(source_ml_source2_nominal_field16736.clone()), Box::new(source_ml_source2_nominal_field16738.clone()), Box::new(source_ml_source2_nominal_field16740.clone()), Box::new(source_ml_source2_nominal_field16742.clone()), Box::new(source_ml_source2_nominal_field16744.clone()), Box::new(source_ml_source2_nominal_field16746.clone()), source_ml_source2_nominal_field16748, source_ml_source2_nominal_field16750, source_ml_source2_nominal_override16792.clone(), source_ml_source2_nominal_override16790, source_ml_source2_nominal_field16756, Box::new(source_ml_source2_nominal_field16758.clone()), Box::new(source_ml_source2_nominal_field16760.clone()), Box::new(source_ml_source2_nominal_field16762.clone()), Box::new(source_ml_source2_nominal_override16791.clone()), Box::new(source_ml_source2_nominal_field16766.clone()), source_ml_source2_nominal_field16768.clone(), source_ml_source2_nominal_field16770.clone(), source_ml_source2_nominal_field16772.clone(), source_ml_source2_nominal_field16774.clone(), source_ml_source2_nominal_field16776.clone(), source_ml_source2_nominal_field16778.clone(), source_ml_source2_nominal_field16780.clone(), source_ml_source2_nominal_field16782.clone(), source_ml_source2_nominal_field16784.clone(), source_ml_source2_nominal_field16786.clone(), source_ml_source2_nominal_field16788.clone())
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                                self::fail_java_script_array_creation(heap_3.clone(), state.clone(), &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(reason))
                            }
                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(heap_4, reason_2) => {
                                let heap_4 = *heap_4;
                                self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap_4.clone()), reason_2.clone())
                            }
                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(heap_5, reason_3) => {
                                let heap_5 = *heap_5;
                                self::JavaScriptArrayCreationTransition::ArrayCreationResourceLimit(Box::new(heap_5.clone()), Box::new(state.clone()), reason_3.clone())
                            }
                        }
                    }
                } else {
                    if (shared_constructor_field12
                        == (self::ml_string::Text::from_units(vec![
                            108, 101, 110, 103, 116, 104, 32, 115, 101, 116,
                        ])))
                    {
                        if {
                            let source_ml_source2_s16796 = shared_constructor_field17.clone();
                            match source_ml_source2_s16796.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => false,
                                _ => true,
                            }
                        } {
                            self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111,
                                    110, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116, 32, 110,
                                    101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                    97, 108, 32, 114, 101, 115, 117, 108, 116, 32, 79, 98, 106,
                                    101, 99, 116,
                                ]),
                            )
                        } else {
                            {
                                let key = self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                    std::cell::RefCell::new(vec![
                                        108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
                                    ]),
                                ));
                                {
                                    let value = self::JavaScriptSemanticValue::Number(
                                        if shared_constructor_field11 {
                                            shared_constructor_field13
                                        } else {
                                            shared_constructor_field14
                                        },
                                    );
                                    {
                                        let plan = self::plan_java_script_ordinary_set_with_array_definition(heap.clone(), (((match shared_constructor_field17.clone() {
                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value16804, _) => {
                                                Some((source_ml_source2_data_property_value16804.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml::Big,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone(), key.clone(), value.clone(), shared_constructor_field17.clone());
                                        if (((({
                                            let ml_reference_value863 = plan.borrow().clone();
                                            ml_reference_value863
                                        })
                                        .0)
                                            .4)
                                            != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            if (((({
                                                let ml_reference_value864 = plan.borrow().clone();
                                                ml_reference_value864
                                            })
                                            .0)
                                                .4)
                                                == (self::ml_string::Text::from_units(vec![
                                                    111, 114, 100, 105, 110, 97, 114, 121, 32, 112,
                                                    114, 111, 112, 101, 114, 116, 105, 101, 115,
                                                    32, 101, 120, 99, 101, 101, 100, 32, 116, 104,
                                                    101, 105, 114, 32, 101, 120, 112, 108, 105, 99,
                                                    105, 116, 32, 111, 114, 100, 101, 114, 101,
                                                    100, 32, 115, 116, 111, 114, 97, 103, 101, 32,
                                                    98, 117, 100, 103, 101, 116,
                                                ])))
                                            {
                                                self::JavaScriptArrayCreationTransition::ArrayCreationResourceLimit(Box::new((({ let ml_reference_value865 = plan.borrow().clone(); ml_reference_value865 }).0).5), Box::new(state.clone()), (({ let ml_reference_value866 = plan.borrow().clone(); ml_reference_value866 }).0).4)
                                            } else {
                                                self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new((({ let ml_reference_value867 = plan.borrow().clone(); ml_reference_value867 }).0).5), (({ let ml_reference_value868 = plan.borrow().clone(); ml_reference_value868 }).0).4)
                                            }
                                        } else {
                                            {
                                                let definition = ((((({ let ml_reference_value869 = plan.borrow().clone(); ml_reference_value869 }).0).3).clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), Vec<f64>)>>>,))).0).clone();
                                                if ((((({
                                                    let ml_reference_value870 =
                                                        plan.borrow().clone();
                                                    ml_reference_value870
                                                })
                                                .0)
                                                    .0)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        97, 114, 114, 97, 121, 32, 100, 101, 102,
                                                        105, 110, 105, 116, 105, 111, 110,
                                                    ])))
                                                    && !(match (definition.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    }))
                                                {
                                                    self::accept_java_script_array_creation_length_set(state.clone(), self::begin_java_script_array_assignment((({ let ml_reference_value871 = plan.borrow().clone(); ml_reference_value871 }).0).5, (((match shared_constructor_field17.clone() {
                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value16807, _) => {
                                                            Some((source_ml_source2_data_property_value16807.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::ml::Big,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone(), key.clone(), definition.clone().expect("a proven present value"), shared_constructor_field6.clone()))
                                                } else {
                                                    if (((({
                                                        let ml_reference_value872 =
                                                            plan.borrow().clone();
                                                        ml_reference_value872
                                                    })
                                                    .0)
                                                        .0)
                                                        == (self::ml_string::Text::from_units(
                                                            vec![99, 97, 108, 108],
                                                        )))
                                                    {
                                                        self::request_java_script_array_creation_call((({ let ml_reference_value873 = plan.borrow().clone(); ml_reference_value873 }).0).5, {
                                                            let source_ml_source2_nominal_field16810 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16812 = shared_constructor_field2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16814 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field16816 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field16818 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field16820 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field16822 = shared_constructor_field7.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16824 = shared_constructor_field8.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16826 = shared_constructor_field9.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16828 = shared_constructor_field10;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16830 = shared_constructor_field11;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16832 = shared_constructor_field12.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16834 = shared_constructor_field13;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16836 = shared_constructor_field14;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16838 = shared_constructor_field15.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16840 = shared_constructor_field16.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16842 = shared_constructor_field17.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16844 = shared_constructor_field18.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16846 = shared_constructor_field19.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16848 = shared_constructor_field20.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16850 = shared_constructor_field21.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16852 = shared_constructor_field22.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16854 = shared_constructor_field23.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field16856 = shared_constructor_field24.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field16858 = shared_constructor_field25.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field16860 = shared_constructor_field26.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field16862 = shared_constructor_field27.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field16864 = shared_constructor_field28.clone();
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field16866 = shared_constructor_field29.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_field16868 = shared_constructor_field30.clone();
                                                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16810.clone(), Box::new(source_ml_source2_nominal_field16812.clone()), source_ml_source2_nominal_field16814.clone(), Box::new(source_ml_source2_nominal_field16816.clone()), Box::new(source_ml_source2_nominal_field16818.clone()), Box::new(source_ml_source2_nominal_field16820.clone()), Box::new(source_ml_source2_nominal_field16822.clone()), Box::new(source_ml_source2_nominal_field16824.clone()), Box::new(source_ml_source2_nominal_field16826.clone()), source_ml_source2_nominal_field16828, source_ml_source2_nominal_field16830, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field16834, source_ml_source2_nominal_field16836, Box::new(source_ml_source2_nominal_field16838.clone()), Box::new(source_ml_source2_nominal_field16840.clone()), Box::new(source_ml_source2_nominal_field16842.clone()), Box::new(source_ml_source2_nominal_field16844.clone()), Box::new(source_ml_source2_nominal_field16846.clone()), source_ml_source2_nominal_field16848.clone(), source_ml_source2_nominal_field16850.clone(), source_ml_source2_nominal_field16852.clone(), source_ml_source2_nominal_field16854.clone(), source_ml_source2_nominal_field16856.clone(), source_ml_source2_nominal_field16858.clone(), source_ml_source2_nominal_field16860.clone(), source_ml_source2_nominal_field16862.clone(), source_ml_source2_nominal_field16864.clone(), source_ml_source2_nominal_field16866.clone(), source_ml_source2_nominal_field16868.clone())
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }, (({ let ml_reference_value874 = plan.borrow().clone(); ml_reference_value874 }).0).2, (({ let ml_reference_value875 = plan.borrow().clone(); ml_reference_value875 }).0).6, (({ let ml_reference_value876 = plan.borrow().clone(); ml_reference_value876 }).0).1)
                                                    } else {
                                                        if (((({
                                                            let ml_reference_value877 =
                                                                plan.borrow().clone();
                                                            ml_reference_value877
                                                        })
                                                        .0)
                                                            .0)
                                                            != (self::ml_string::Text::from_units(
                                                                vec![
                                                                    99, 111, 109, 112, 108, 101,
                                                                    116, 101,
                                                                ],
                                                            )))
                                                        {
                                                            self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new((({ let ml_reference_value878 = plan.borrow().clone(); ml_reference_value878 }).0).5), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 102, 105, 110, 97, 108, 32, 108, 101, 110, 103, 116, 104, 32, 83, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 47, 65, 114, 114, 97, 121, 32, 111, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 115, 101, 116, 116, 101, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
                                                        } else {
                                                            if (({
                                                                let ml_reference_value879 =
                                                                    plan.borrow().clone();
                                                                ml_reference_value879
                                                            })
                                                            .0)
                                                                .7
                                                            {
                                                                self::JavaScriptArrayCreationTransition::ArrayCreationCompleted(Box::new((({ let ml_reference_value880 = plan.borrow().clone(); ml_reference_value880 }).0).5), Box::new(shared_constructor_field17.clone()))
                                                            } else {
                                                                self::fail_java_script_array_creation((({ let ml_reference_value881 = plan.borrow().clone(); ml_reference_value881 }).0).5, state.clone(), &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 111, 117, 108, 100, 32, 110, 111, 116, 32, 115, 101, 116, 32, 114, 101, 115, 117, 108, 116, 32, 108, 101, 110, 103, 116, 104])))
                                                            }
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
                        if ((shared_constructor_field12
                            == (self::ml_string::Text::from_units(vec![
                                97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104,
                            ])))
                            && !(match (
                                match state.clone() {
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
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value16874,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value16874
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArrayAssignmentState>,)
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
                            self::accept_java_script_array_creation_length_set(
                                state.clone(),
                                self::step_java_script_array_assignment(
                                    heap.clone(),
                                    (match state.clone() {
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
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value16875,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => ((source_ml_source2_data_property_value16875
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptArrayAssignmentState>,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value"),
                                ),
                            )
                        } else {
                            self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111,
                                    110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 108,
                                    121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                                    101, 108, 101, 99, 116, 101, 100, 32, 99, 111, 110, 116, 105,
                                    110, 117, 97, 116, 105, 111, 110,
                                ]),
                            )
                        }
                    }
                }
            }
        }
    }
}
