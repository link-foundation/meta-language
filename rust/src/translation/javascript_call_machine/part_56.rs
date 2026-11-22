// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConcatenationCall;
use super::JavaScriptArrayConcatenationState;
use super::JavaScriptArrayConcatenationTransition;
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptArraySpeciesState;
use super::JavaScriptObjectSpreadState;
use super::JavaScriptObjectSpreadTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341;
use super::begin_java_script_array_species_selection;
use super::continue_java_script_array_concatenation_species;
use super::convert_java_script_semantic_boolean;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_concatenation_state_error;
use super::request_java_script_array_concatenation_call;
use super::test_java_script_array_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_concatenation_final_set(
    state: self::JavaScriptArrayConcatenationState,
    transition: self::JavaScriptObjectSpreadTransition,
) -> self::JavaScriptArrayConcatenationTransition {
    {
        let source_ml_source1_temporary_binding340 = state.clone();
        {
            let prior_call = match source_ml_source1_temporary_binding340.clone() {
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
                    source_ml_source2_data_property_value9632,
                ) => ((source_ml_source2_data_property_value9632
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArrayConcatenationCall>,)))
                .0)
                    .clone(),
            };
            {
                let base = match source_ml_source1_temporary_binding340.clone() {
                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
                        source_ml_source2_nominal_rest_field9633,
                        source_ml_source2_nominal_rest_field9634,
                        source_ml_source2_nominal_rest_field9635,
                        source_ml_source2_nominal_rest_field9636,
                        source_ml_source2_nominal_rest_field9637,
                        source_ml_source2_nominal_rest_field9638,
                        source_ml_source2_nominal_rest_field9639,
                        source_ml_source2_nominal_rest_field9640,
                        source_ml_source2_nominal_rest_field9641,
                        source_ml_source2_nominal_rest_field9642,
                        source_ml_source2_nominal_rest_field9643,
                        source_ml_source2_nominal_rest_field9644,
                        source_ml_source2_nominal_rest_field9645,
                        source_ml_source2_nominal_rest_field9646,
                        source_ml_source2_nominal_rest_field9647,
                        source_ml_source2_nominal_rest_field9648,
                        source_ml_source2_nominal_rest_field9649,
                        source_ml_source2_nominal_rest_field9650,
                        source_ml_source2_nominal_rest_field9651,
                        source_ml_source2_nominal_rest_field9652,
                        source_ml_source2_nominal_rest_field9653,
                        _,
                    ) => {
                        let source_ml_source2_nominal_rest_field9633 =
                            *source_ml_source2_nominal_rest_field9633;
                        let source_ml_source2_nominal_rest_field9635 =
                            *source_ml_source2_nominal_rest_field9635;
                        let source_ml_source2_nominal_rest_field9636 =
                            *source_ml_source2_nominal_rest_field9636;
                        let source_ml_source2_nominal_rest_field9638 =
                            *source_ml_source2_nominal_rest_field9638;
                        let source_ml_source2_nominal_rest_field9644 =
                            *source_ml_source2_nominal_rest_field9644;
                        let source_ml_source2_nominal_rest_field9645 =
                            *source_ml_source2_nominal_rest_field9645;
                        let source_ml_source2_nominal_rest_field9646 =
                            *source_ml_source2_nominal_rest_field9646;
                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(
                            Box::new(source_ml_source2_nominal_rest_field9633.clone()),
                            source_ml_source2_nominal_rest_field9634.clone(),
                            Box::new(source_ml_source2_nominal_rest_field9635.clone()),
                            Box::new(source_ml_source2_nominal_rest_field9636.clone()),
                            source_ml_source2_nominal_rest_field9637,
                            Box::new(source_ml_source2_nominal_rest_field9638.clone()),
                            source_ml_source2_nominal_rest_field9639,
                            source_ml_source2_nominal_rest_field9640,
                            source_ml_source2_nominal_rest_field9641,
                            source_ml_source2_nominal_rest_field9642,
                            source_ml_source2_nominal_rest_field9643,
                            Box::new(source_ml_source2_nominal_rest_field9644.clone()),
                            Box::new(source_ml_source2_nominal_rest_field9645.clone()),
                            Box::new(source_ml_source2_nominal_rest_field9646.clone()),
                            source_ml_source2_nominal_rest_field9647.clone(),
                            source_ml_source2_nominal_rest_field9648.clone(),
                            source_ml_source2_nominal_rest_field9649.clone(),
                            source_ml_source2_nominal_rest_field9650.clone(),
                            source_ml_source2_nominal_rest_field9651.clone(),
                            source_ml_source2_nominal_rest_field9652.clone(),
                            source_ml_source2_nominal_rest_field9653.clone(),
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
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadContinue(
                                heap,
                                state_2,
                            ) => {
                                let heap = *heap;
                                let state_2 = *state_2;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                    let source_ml_source2_nominal_field9656 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field9658 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field9660 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field9662 = shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field9664 = shared_constructor_field5;
                                                    {
                                                        let source_ml_source2_nominal_field9666 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field9668 = shared_constructor_field7;
                                                            {
                                                                let source_ml_source2_nominal_field9670 = shared_constructor_field8;
                                                                {
                                                                    let source_ml_source2_nominal_field9672 = shared_constructor_field9;
                                                                    {
                                                                        let source_ml_source2_nominal_field9674 = shared_constructor_field10;
                                                                        {
                                                                            let source_ml_source2_nominal_field9676 = shared_constructor_field11;
                                                                            {
                                                                                let source_ml_source2_nominal_field9678 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field9680 = shared_constructor_field13.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9682 = shared_constructor_field14.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9684 = shared_constructor_field15.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9686 = shared_constructor_field16.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9688 = shared_constructor_field17.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9690 = shared_constructor_field18.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9692 = shared_constructor_field19.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9694 = shared_constructor_field20.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9696 = shared_constructor_field21.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9698 = shared_constructor_field22.clone();
                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9656.clone()), source_ml_source2_nominal_field9658.clone(), Box::new(source_ml_source2_nominal_field9660.clone()), Box::new(source_ml_source2_nominal_field9662.clone()), source_ml_source2_nominal_field9664, Box::new(source_ml_source2_nominal_field9666.clone()), source_ml_source2_nominal_field9668, source_ml_source2_nominal_field9670, source_ml_source2_nominal_field9672, source_ml_source2_nominal_field9674, source_ml_source2_nominal_field9676, Box::new(source_ml_source2_nominal_field9678.clone()), Box::new(source_ml_source2_nominal_field9680.clone()), Box::new(source_ml_source2_nominal_field9682.clone()), source_ml_source2_nominal_field9684.clone(), source_ml_source2_nominal_field9686.clone(), source_ml_source2_nominal_field9688.clone(), source_ml_source2_nominal_field9690.clone(), source_ml_source2_nominal_field9692.clone(), source_ml_source2_nominal_field9694.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field9698.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadCall(
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
                                self::request_java_script_array_concatenation_call(
                                    heap_2.clone(),
                                    {
                                        let source_ml_source2_nominal_field9702 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field9704 =
                                                shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field9706 =
                                                    shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field9708 =
                                                        shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field9710 =
                                                            shared_constructor_field5;
                                                        {
                                                            let source_ml_source2_nominal_field9712 =
                                                                shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field9714 =
                                                                    shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field9716 =
                                                                        shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field9718 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field9720 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field9722 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field9724 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9726 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9728 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9730 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9732 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9734 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9736 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9738 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9740 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9742 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9744 = shared_constructor_field22.clone();
                                                                                                                            self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9702.clone()), source_ml_source2_nominal_field9704.clone(), Box::new(source_ml_source2_nominal_field9706.clone()), Box::new(source_ml_source2_nominal_field9708.clone()), source_ml_source2_nominal_field9710, Box::new(source_ml_source2_nominal_field9712.clone()), source_ml_source2_nominal_field9714, source_ml_source2_nominal_field9716, source_ml_source2_nominal_field9718, source_ml_source2_nominal_field9720, source_ml_source2_nominal_field9722, Box::new(source_ml_source2_nominal_field9724.clone()), Box::new(source_ml_source2_nominal_field9726.clone()), Box::new(source_ml_source2_nominal_field9728.clone()), source_ml_source2_nominal_field9730.clone(), source_ml_source2_nominal_field9732.clone(), source_ml_source2_nominal_field9734.clone(), source_ml_source2_nominal_field9736.clone(), source_ml_source2_nominal_field9738.clone(), source_ml_source2_nominal_field9740.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field9744.clone())
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
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
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadCompleted(
                                heap_3,
                                _,
                            ) => {
                                let heap_3 = *heap_3;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationCompleted(Box::new(heap_3.clone()), Box::new(match state.clone() {
                                    self::JavaScriptArrayConcatenationState::ArrayConcatenationState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value9747, _, _, _, _, _, _, _, _, _, _) => {
                                        let source_ml_source2_data_property_value9747 = *source_ml_source2_data_property_value9747;
                                        source_ml_source2_data_property_value9747.clone()
                                    }
                                }))
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadThrow(
                                heap_4,
                                value,
                            ) => {
                                let heap_4 = *heap_4;
                                let value = *value;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationThrow(Box::new(heap_4.clone()), Box::new(value.clone()))
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadError(
                                heap_5,
                                reason,
                                error_class,
                            ) => {
                                let heap_5 = *heap_5;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(Box::new(heap_5.clone()), (((error_class.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), reason.clone())
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadInvalid(
                                heap_6,
                                reason_2,
                            ) => {
                                let heap_6 = *heap_6;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap_6.clone()), reason_2.clone())
                            }
                            self::JavaScriptObjectSpreadTransition::ObjectSpreadResourceLimit(
                                heap_7,
                                state_4,
                                reason_3,
                            ) => {
                                let heap_7 = *heap_7;
                                let state_4 = *state_4;
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationResourceLimit(Box::new(heap_7.clone()), Box::new({
                                    let source_ml_source2_nominal_field9749 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field9751 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field9753 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field9755 = shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field9757 = shared_constructor_field5;
                                                    {
                                                        let source_ml_source2_nominal_field9759 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field9761 = shared_constructor_field7;
                                                            {
                                                                let source_ml_source2_nominal_field9763 = shared_constructor_field8;
                                                                {
                                                                    let source_ml_source2_nominal_field9765 = shared_constructor_field9;
                                                                    {
                                                                        let source_ml_source2_nominal_field9767 = shared_constructor_field10;
                                                                        {
                                                                            let source_ml_source2_nominal_field9769 = shared_constructor_field11;
                                                                            {
                                                                                let source_ml_source2_nominal_field9771 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field9773 = shared_constructor_field13.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9775 = shared_constructor_field14.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9777 = shared_constructor_field15.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9779 = shared_constructor_field16.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9781 = shared_constructor_field17.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9783 = shared_constructor_field18.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9785 = shared_constructor_field19.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9787 = shared_constructor_field20.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9789 = shared_constructor_field21.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9791 = shared_constructor_field22.clone();
                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9749.clone()), source_ml_source2_nominal_field9751.clone(), Box::new(source_ml_source2_nominal_field9753.clone()), Box::new(source_ml_source2_nominal_field9755.clone()), source_ml_source2_nominal_field9757, Box::new(source_ml_source2_nominal_field9759.clone()), source_ml_source2_nominal_field9761, source_ml_source2_nominal_field9763, source_ml_source2_nominal_field9765, source_ml_source2_nominal_field9767, source_ml_source2_nominal_field9769, Box::new(source_ml_source2_nominal_field9771.clone()), Box::new(source_ml_source2_nominal_field9773.clone()), Box::new(source_ml_source2_nominal_field9775.clone()), source_ml_source2_nominal_field9777.clone(), source_ml_source2_nominal_field9779.clone(), source_ml_source2_nominal_field9781.clone(), source_ml_source2_nominal_field9783.clone(), source_ml_source2_nominal_field9785.clone(), source_ml_source2_nominal_field9787.clone(), Some((Some(state_4.clone()),)), source_ml_source2_nominal_field9791.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_concatenation_spreadability(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayConcatenationState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayConcatenationTransition {
    match state.clone() {
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
            {
                let spread = None::<()>;
                {
                    let source_ml_source2_s9795 = match value.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            let brand = self::test_java_script_array_value(
                                heap.clone(),
                                shared_constructor_field6.clone(),
                            );
                            if (((({
                                let ml_reference_value705 = brand.borrow().clone();
                                ml_reference_value705
                            })
                            .0)
                                .1)
                                != (self::ml_string::Text::from_units(vec![])))
                            {
                                self::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341Return(Box::new(self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(Box::new(heap.clone()), (({ let ml_reference_value706 = brand.borrow().clone(); ml_reference_value706 }).0).1)))
                            } else {
                                {
                                    let spread_2 = (({
                                        let ml_reference_value707 = brand.borrow().clone();
                                        ml_reference_value707
                                    })
                                    .0)
                                        .0;
                                    self::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341Next(spread_2)
                                }
                            }
                        }
                        _ => {
                            let spread_3 =
                                self::convert_java_script_semantic_boolean(value.clone());
                            self::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341Next(spread_3)
                        }
                    };
                    match source_ml_source2_s9795.clone() {
                        self::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341Next(spread_4) => {
                            if !spread_4 {
                                if (shared_constructor_field10 >= (9007199254740991f64)) {
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 109, 97, 120, 105, 109, 117, 109, 32, 115, 97, 102, 101, 32, 111, 117, 116, 112, 117, 116, 32, 108, 101, 110, 103, 116, 104]))
                                } else {
                                    self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field9801 = shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field9803 = shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field9805 = shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field9807 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field9809 = shared_constructor_field5;
                                                        {
                                                            let source_ml_source2_nominal_field9811 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field9813 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field9815 = shared_constructor_field8;
                                                                    {
                                                                        let source_ml_source2_nominal_field9817 = shared_constructor_field9;
                                                                        {
                                                                            let source_ml_source2_nominal_field9819 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field9821 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field9823 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9825 = shared_constructor_field13.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9827 = shared_constructor_field14.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9829 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9831 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9833 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9835 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9837 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9839 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9841 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field9843 = shared_constructor_field22.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override9847 = shared_constructor_field6.clone();
                                                                                                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9801.clone()), source_ml_source2_nominal_field9803.clone(), Box::new(source_ml_source2_nominal_field9805.clone()), Box::new(source_ml_source2_nominal_field9807.clone()), source_ml_source2_nominal_field9809, Box::new(source_ml_source2_nominal_field9811.clone()), source_ml_source2_nominal_field9813, source_ml_source2_nominal_field9815, source_ml_source2_nominal_field9817, source_ml_source2_nominal_field9819, false, Box::new(source_ml_source2_nominal_field9823.clone()), Box::new(source_ml_source2_nominal_override9847.clone()), Box::new(source_ml_source2_nominal_field9827.clone()), self::ml_string::Text::from_units(vec![119, 114, 105, 116, 101]), source_ml_source2_nominal_field9831.clone(), source_ml_source2_nominal_field9833.clone(), source_ml_source2_nominal_field9835.clone(), source_ml_source2_nominal_field9837.clone(), source_ml_source2_nominal_field9839.clone(), source_ml_source2_nominal_field9841.clone(), source_ml_source2_nominal_field9843.clone())
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
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
                                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationContinue(Box::new(heap.clone()), Box::new({
                                    let source_ml_source2_nominal_field9849 = shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field9851 = shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field9853 = shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field9855 = shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field9857 = shared_constructor_field5;
                                                    {
                                                        let source_ml_source2_nominal_field9859 = shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field9861 = shared_constructor_field7;
                                                            {
                                                                let source_ml_source2_nominal_field9863 = shared_constructor_field8;
                                                                {
                                                                    let source_ml_source2_nominal_field9865 = shared_constructor_field9;
                                                                    {
                                                                        let source_ml_source2_nominal_field9867 = shared_constructor_field10;
                                                                        {
                                                                            let source_ml_source2_nominal_field9869 = shared_constructor_field11;
                                                                            {
                                                                                let source_ml_source2_nominal_field9871 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field9873 = shared_constructor_field13.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field9875 = shared_constructor_field14.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field9877 = shared_constructor_field15.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field9879 = shared_constructor_field16.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field9881 = shared_constructor_field17.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field9883 = shared_constructor_field18.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field9885 = shared_constructor_field19.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field9887 = shared_constructor_field20.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field9889 = shared_constructor_field21.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field9891 = shared_constructor_field22.clone();
                                                                                                                        self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(source_ml_source2_nominal_field9849.clone()), source_ml_source2_nominal_field9851.clone(), Box::new(source_ml_source2_nominal_field9853.clone()), Box::new(source_ml_source2_nominal_field9855.clone()), source_ml_source2_nominal_field9857, Box::new(source_ml_source2_nominal_field9859.clone()), source_ml_source2_nominal_field9861, source_ml_source2_nominal_field9863, source_ml_source2_nominal_field9865, source_ml_source2_nominal_field9867, true, Box::new(source_ml_source2_nominal_field9871.clone()), Box::new(source_ml_source2_nominal_field9873.clone()), Box::new(source_ml_source2_nominal_field9875.clone()), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), source_ml_source2_nominal_field9879.clone(), source_ml_source2_nominal_field9881.clone(), source_ml_source2_nominal_field9883.clone(), source_ml_source2_nominal_field9885.clone(), source_ml_source2_nominal_field9887.clone(), source_ml_source2_nominal_field9889.clone(), source_ml_source2_nominal_field9891.clone())
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                        self::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341::MlSource1TemporaryContinueJavaScriptArrayConcatenationSpreadabilityJoin341Return(source_ml_source1_temporary_result341) => {
                            let source_ml_source1_temporary_result341 = *source_ml_source1_temporary_result341;
                            source_ml_source1_temporary_result341.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_array_concatenation(
    heap: self::JavaScriptOrdinaryHeap,
    directory: self::JavaScriptArraySpeciesDirectory,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    spreadable_key: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayConcatenationTransition {
    {
        let state = {
            let source_ml_source2_constructor_field9896 =
                std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(
                    Vec::<self::JavaScriptSemanticValue>::new(),
                    {
                        let ml_reference_value708 = arguments_.borrow().clone();
                        ml_reference_value708
                    },
                )));
            {
                let source_ml_source2_constructor_field9899 = 0f64;
                {
                    let source_ml_source2_constructor_field9900 =
                        self::JavaScriptSemanticValue::Undefined;
                    {
                        let source_ml_source2_constructor_field9901 = 0f64;
                        {
                            let source_ml_source2_constructor_field9902 = 0f64;
                            {
                                let source_ml_source2_constructor_field9903 = 0f64;
                                {
                                    let source_ml_source2_constructor_field9904 = 0f64;
                                    {
                                        let source_ml_source2_constructor_field9906 =
                                            self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source2_constructor_field9907 =
                                                self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_constructor_field9908 =
                                                    self::JavaScriptSemanticValue::Undefined;
                                                self::JavaScriptArrayConcatenationState::ArrayConcatenationState(Box::new(receiver.clone()), source_ml_source2_constructor_field9896.clone(), Box::new(spreadable_key.clone()), Box::new(exotic_key.clone()), source_ml_source2_constructor_field9899, Box::new(source_ml_source2_constructor_field9900.clone()), source_ml_source2_constructor_field9901, source_ml_source2_constructor_field9902, source_ml_source2_constructor_field9903, source_ml_source2_constructor_field9904, false, Box::new(source_ml_source2_constructor_field9906.clone()), Box::new(source_ml_source2_constructor_field9907.clone()), Box::new(source_ml_source2_constructor_field9908.clone()), self::ml_string::Text::from_units(vec![105, 110, 105, 116, 105, 97, 108, 105, 122, 101]), None::<(Option<self::JavaScriptPrimitiveConversionState>,)>, None::<(Option<self::JavaScriptArraySpeciesState>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptArrayConstructorFact>,)>, None::<(Option<self::JavaScriptObjectSpreadState>,)>, None::<(Option<self::JavaScriptArrayConcatenationCall>,)>)
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
        {
            let error = self::read_java_script_array_concatenation_state_error(
                heap.clone(),
                directory.clone(),
                state.clone(),
            );
            if (error == (self::ml_string::Text::from_units(vec![]))) {
                self::continue_java_script_array_concatenation_species(
                    heap.clone(),
                    state.clone(),
                    self::begin_java_script_array_species_selection(
                        heap.clone(),
                        directory.clone(),
                        receiver.clone(),
                        0f64,
                    ),
                )
            } else {
                self::JavaScriptArrayConcatenationTransition::ArrayConcatenationInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            }
        }
    }
}
