// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::accept_java_script_array_creation_length;
use super::accept_java_script_array_creation_wrapper;
use super::begin_java_script_primitive_wrapper;
use super::begin_java_script_scalar_protocol;
use super::clear_java_script_array_creation_child;
use super::fail_java_script_array_creation;
use super::ml_string;
use super::select_java_script_array_creation_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_creation_get(
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
            shared_constructor_field31,
            shared_constructor_field32,
            _,
            _,
            _,
            _,
            shared_constructor_field33,
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
        ) => {
            let shared_constructor_field31 = *shared_constructor_field31;
            let shared_constructor_field32 = *shared_constructor_field32;
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
                        {
                            let phase = if shared_constructor_field33.ends_with(
                                &self::ml_string::Text::from_units(vec![
                                    32, 103, 101, 116, 116, 101, 114,
                                ]),
                            ) {
                                {
                                    let source_ml_source2_string_range_operand15554 =
                                        shared_constructor_field33.clone();
                                    {
                                        let source_ml_source2_string_range_operand15555 = 0f64;
                                        {
                                            let source_ml_source2_string_range_operand15556 =
                                                (-7f64);
                                            {
                                                let source_ml_source2_string_range_position15557 =
                                                    source_ml_source2_string_range_operand15555;
                                                {
                                                    let source_ml_source2_string_range_position15558 =
                                                        source_ml_source2_string_range_operand15556;
                                                    source_ml_source2_string_range_operand15554.slice(source_ml_source2_string_range_position15557, source_ml_source2_string_range_position15558)
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                shared_constructor_field33.clone()
                            };
                            if (phase
                                == (self::ml_string::Text::from_units(vec![
                                    109, 101, 116, 104, 111, 100,
                                ])))
                            {
                                if (match value.clone() {
                                    self::JavaScriptSemanticValue::Undefined => true,
                                    _ => false,
                                } || match value.clone() {
                                    self::JavaScriptSemanticValue::Null => true,
                                    _ => false,
                                }) {
                                    if {
                                        let source_ml_source2_s15561 =
                                            shared_constructor_field32.clone();
                                        match source_ml_source2_s15561.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => true,
                                            _ => false,
                                        }
                                    } {
                                        self::JavaScriptArrayCreationTransition::ArrayCreationContinue(Box::new(heap.clone()), Box::new({
                                            let source_ml_source2_nominal_field15567 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field15569 = shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field15571 = shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15573 = shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15575 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15577 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15579 = shared_constructor_field7.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15581 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field15583 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15585 = shared_constructor_field10;
                                                                                {
                                                                                    let source_ml_source2_nominal_field15587 = shared_constructor_field11;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15589 = shared_constructor_field12.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15591 = shared_constructor_field13;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15593 = shared_constructor_field14;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15595 = shared_constructor_field15.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15597 = shared_constructor_field16.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15599 = shared_constructor_field17.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15601 = shared_constructor_field18.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15603 = shared_constructor_field19.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15605 = shared_constructor_field20.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15607 = shared_constructor_field21.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15609 = shared_constructor_field22.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15611 = shared_constructor_field23.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15613 = shared_constructor_field24.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15615 = shared_constructor_field25.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15617 = shared_constructor_field26.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15619 = shared_constructor_field27.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15621 = shared_constructor_field28.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15623 = shared_constructor_field29.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field15625 = shared_constructor_field30.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_override15627 = shared_constructor_field32.clone();
                                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15567.clone(), Box::new(source_ml_source2_nominal_field15569.clone()), source_ml_source2_nominal_field15571.clone(), Box::new(source_ml_source2_nominal_field15573.clone()), Box::new(source_ml_source2_nominal_field15575.clone()), Box::new(source_ml_source2_nominal_field15577.clone()), Box::new(source_ml_source2_nominal_field15579.clone()), Box::new(source_ml_source2_nominal_field15581.clone()), Box::new(source_ml_source2_nominal_field15583.clone()), source_ml_source2_nominal_field15585, source_ml_source2_nominal_field15587, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), source_ml_source2_nominal_field15591, source_ml_source2_nominal_field15593, Box::new(source_ml_source2_nominal_field15595.clone()), Box::new(source_ml_source2_nominal_override15627.clone()), Box::new(source_ml_source2_nominal_field15599.clone()), Box::new(source_ml_source2_nominal_field15601.clone()), Box::new(source_ml_source2_nominal_field15603.clone()), source_ml_source2_nominal_field15605.clone(), source_ml_source2_nominal_field15607.clone(), source_ml_source2_nominal_field15609.clone(), source_ml_source2_nominal_field15611.clone(), source_ml_source2_nominal_field15613.clone(), source_ml_source2_nominal_field15615.clone(), source_ml_source2_nominal_field15617.clone(), source_ml_source2_nominal_field15619.clone(), source_ml_source2_nominal_field15621.clone(), source_ml_source2_nominal_field15623.clone(), source_ml_source2_nominal_field15625.clone())
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                                        if match (match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15629, _, _, _, _, _, _, _, _, _, _) => {
                                                ((source_ml_source2_data_property_value15629.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 84, 111, 79, 98, 106, 101, 99, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
                                        } else {
                                            self::accept_java_script_array_creation_wrapper({
                                                let source_ml_source2_nominal_field15631 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field15633 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15635 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15637 = shared_constructor_field4.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15639 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15641 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15643 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field15645 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15647 = shared_constructor_field9.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field15649 = shared_constructor_field10;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15651 = shared_constructor_field11;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15653 = shared_constructor_field12.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15655 = shared_constructor_field13;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15657 = shared_constructor_field14;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15659 = shared_constructor_field15.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15661 = shared_constructor_field16.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15663 = shared_constructor_field17.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15665 = shared_constructor_field18.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15667 = shared_constructor_field19.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15669 = shared_constructor_field20.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15671 = shared_constructor_field21.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15673 = shared_constructor_field22.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15675 = shared_constructor_field23.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15677 = shared_constructor_field24.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15679 = shared_constructor_field25.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15681 = shared_constructor_field26.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15683 = shared_constructor_field27.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15685 = shared_constructor_field28.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field15687 = shared_constructor_field29.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field15689 = shared_constructor_field30.clone();
                                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15631.clone(), Box::new(source_ml_source2_nominal_field15633.clone()), source_ml_source2_nominal_field15635.clone(), Box::new(source_ml_source2_nominal_field15637.clone()), Box::new(source_ml_source2_nominal_field15639.clone()), Box::new(source_ml_source2_nominal_field15641.clone()), Box::new(source_ml_source2_nominal_field15643.clone()), Box::new(source_ml_source2_nominal_field15645.clone()), Box::new(source_ml_source2_nominal_field15647.clone()), source_ml_source2_nominal_field15649, source_ml_source2_nominal_field15651, self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]), source_ml_source2_nominal_field15655, source_ml_source2_nominal_field15657, Box::new(source_ml_source2_nominal_field15659.clone()), Box::new(source_ml_source2_nominal_field15661.clone()), Box::new(source_ml_source2_nominal_field15663.clone()), Box::new(source_ml_source2_nominal_field15665.clone()), Box::new(source_ml_source2_nominal_field15667.clone()), source_ml_source2_nominal_field15669.clone(), source_ml_source2_nominal_field15671.clone(), source_ml_source2_nominal_field15673.clone(), source_ml_source2_nominal_field15675.clone(), source_ml_source2_nominal_field15677.clone(), source_ml_source2_nominal_field15679.clone(), source_ml_source2_nominal_field15681.clone(), source_ml_source2_nominal_field15683.clone(), source_ml_source2_nominal_field15685.clone(), source_ml_source2_nominal_field15687.clone(), source_ml_source2_nominal_field15689.clone())
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }, self::begin_java_script_primitive_wrapper(heap.clone(), shared_constructor_field32.clone(), (match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value15693, _, _, _, _, _, _, _, _, _, _) => {
                                                    ((source_ml_source2_data_property_value15693.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            }).clone().expect("a proven present value")))
                                        }
                                    }
                                } else {
                                    if (match value.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => true,
                                        _ => false,
                                    } && ((match value.clone() {
                                        self::JavaScriptSemanticValue::Object(
                                            _,
                                            source_ml_source2_data_property_value15696,
                                        ) => Some((source_ml_source2_data_property_value15696,)),
                                        _ => None::<(bool,)>,
                                    })
                                    .clone()
                                    .expect("a proven present value"))
                                    .0)
                                    {
                                        self::JavaScriptArrayCreationTransition::ArrayCreationContinue(Box::new(heap.clone()), Box::new({
                                            let source_ml_source2_nominal_field15698 = shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field15700 = shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field15702 = shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15704 = shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15706 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15708 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15710 = shared_constructor_field7.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15712 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field15714 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15716 = shared_constructor_field10;
                                                                                {
                                                                                    let source_ml_source2_nominal_field15718 = shared_constructor_field11;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15720 = shared_constructor_field12.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15722 = shared_constructor_field13;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15724 = shared_constructor_field14;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15726 = shared_constructor_field15.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15728 = shared_constructor_field16.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15730 = shared_constructor_field17.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15732 = shared_constructor_field18.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15734 = shared_constructor_field19.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15736 = shared_constructor_field20.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15738 = shared_constructor_field21.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15740 = shared_constructor_field22.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15742 = shared_constructor_field23.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15744 = shared_constructor_field24.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15746 = shared_constructor_field25.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15748 = shared_constructor_field26.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15750 = shared_constructor_field27.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15752 = shared_constructor_field28.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15754 = shared_constructor_field29.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field15756 = shared_constructor_field30.clone();
                                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15698.clone(), Box::new(source_ml_source2_nominal_field15700.clone()), source_ml_source2_nominal_field15702.clone(), Box::new(source_ml_source2_nominal_field15704.clone()), Box::new(source_ml_source2_nominal_field15706.clone()), Box::new(source_ml_source2_nominal_field15708.clone()), Box::new(source_ml_source2_nominal_field15710.clone()), Box::new(source_ml_source2_nominal_field15712.clone()), Box::new(source_ml_source2_nominal_field15714.clone()), source_ml_source2_nominal_field15716, true, self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), source_ml_source2_nominal_field15722, source_ml_source2_nominal_field15724, Box::new(value.clone()), Box::new(source_ml_source2_nominal_field15728.clone()), Box::new(source_ml_source2_nominal_field15730.clone()), Box::new(source_ml_source2_nominal_field15732.clone()), Box::new(source_ml_source2_nominal_field15734.clone()), source_ml_source2_nominal_field15736.clone(), source_ml_source2_nominal_field15738.clone(), source_ml_source2_nominal_field15740.clone(), source_ml_source2_nominal_field15742.clone(), source_ml_source2_nominal_field15744.clone(), source_ml_source2_nominal_field15746.clone(), source_ml_source2_nominal_field15748.clone(), source_ml_source2_nominal_field15750.clone(), source_ml_source2_nominal_field15752.clone(), source_ml_source2_nominal_field15754.clone(), source_ml_source2_nominal_field15756.clone())
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                                        self::fail_java_script_array_creation(
                                            heap.clone(),
                                            base.clone(),
                                            &(self::ml_string::Text::from_units(vec![
                                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                                            ])),
                                            &(self::ml_string::Text::from_units(vec![
                                                65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32,
                                                105, 116, 101, 114, 97, 116, 111, 114, 32, 109,
                                                101, 116, 104, 111, 100, 32, 109, 117, 115, 116,
                                                32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108,
                                                101,
                                            ])),
                                        )
                                    }
                                }
                            } else {
                                if (phase
                                    == (self::ml_string::Text::from_units(vec![
                                        108, 101, 110, 103, 116, 104,
                                    ])))
                                {
                                    self::accept_java_script_array_creation_length(
                                        heap.clone(),
                                        {
                                            let source_ml_source2_nominal_field15762 =
                                                shared_constructor_field1.clone();
                                            {
                                                let source_ml_source2_nominal_field15764 =
                                                    shared_constructor_field2.clone();
                                                {
                                                    let source_ml_source2_nominal_field15766 =
                                                        shared_constructor_field3.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15768 =
                                                            shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15770 =
                                                                shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15772 =
                                                                    shared_constructor_field6
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15774 =
                                                                        shared_constructor_field7
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15776 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field15778 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15780 = shared_constructor_field10;
                                                                                {
                                                                                    let source_ml_source2_nominal_field15782 = shared_constructor_field11;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15784 = shared_constructor_field12.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15786 = shared_constructor_field13;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15788 = shared_constructor_field14;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15790 = shared_constructor_field15.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15792 = shared_constructor_field16.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15794 = shared_constructor_field17.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15796 = shared_constructor_field18.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15798 = shared_constructor_field19.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15800 = shared_constructor_field20.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15802 = shared_constructor_field21.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15804 = shared_constructor_field22.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15806 = shared_constructor_field23.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15808 = shared_constructor_field24.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15810 = shared_constructor_field25.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15812 = shared_constructor_field26.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15814 = shared_constructor_field27.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15816 = shared_constructor_field28.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15818 = shared_constructor_field29.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field15820 = shared_constructor_field30.clone();
                                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15762.clone(), Box::new(source_ml_source2_nominal_field15764.clone()), source_ml_source2_nominal_field15766.clone(), Box::new(source_ml_source2_nominal_field15768.clone()), Box::new(source_ml_source2_nominal_field15770.clone()), Box::new(source_ml_source2_nominal_field15772.clone()), Box::new(source_ml_source2_nominal_field15774.clone()), Box::new(source_ml_source2_nominal_field15776.clone()), Box::new(source_ml_source2_nominal_field15778.clone()), source_ml_source2_nominal_field15780, source_ml_source2_nominal_field15782, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), source_ml_source2_nominal_field15786, source_ml_source2_nominal_field15788, Box::new(source_ml_source2_nominal_field15790.clone()), Box::new(source_ml_source2_nominal_field15792.clone()), Box::new(source_ml_source2_nominal_field15794.clone()), Box::new(source_ml_source2_nominal_field15796.clone()), Box::new(value.clone()), source_ml_source2_nominal_field15800.clone(), source_ml_source2_nominal_field15802.clone(), source_ml_source2_nominal_field15804.clone(), source_ml_source2_nominal_field15806.clone(), source_ml_source2_nominal_field15808.clone(), source_ml_source2_nominal_field15810.clone(), source_ml_source2_nominal_field15812.clone(), source_ml_source2_nominal_field15814.clone(), source_ml_source2_nominal_field15816.clone(), source_ml_source2_nominal_field15818.clone(), source_ml_source2_nominal_field15820.clone())
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                                            value.clone(),
                                            shared_constructor_field31.clone(),
                                            &(self::ml_string::Text::from_units(vec![
                                                99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109,
                                                105, 116, 105, 118, 101, 32, 110, 117, 109, 98,
                                                101, 114,
                                            ])),
                                        ),
                                    )
                                } else {
                                    if (phase
                                        == (self::ml_string::Text::from_units(vec![
                                            101, 108, 101, 109, 101, 110, 116,
                                        ])))
                                    {
                                        self::select_java_script_array_creation_value(
                                            heap.clone(),
                                            base.clone(),
                                            value.clone(),
                                        )
                                    } else {
                                        self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 71, 101, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 112, 104, 97, 115, 101]))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
