// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySearchCall;
use super::JavaScriptArraySearchState;
use super::JavaScriptArraySearchTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::begin_java_script_scalar_protocol;
use super::ml_string;
use super::read_java_script_array_like_length;
use super::read_java_script_array_search_index;
use super::read_java_script_array_search_missing_value;
use super::read_java_script_array_search_state_error;
use super::request_java_script_array_search_call;
use super::test_java_script_call_strict_equal;
use super::test_java_script_collection_key;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_search_number(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySearchState,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptArraySearchTransition {
    {
        let source_ml_source1_temporary_binding1237 = state.clone();
        {
            let prior_call = match source_ml_source1_temporary_binding1237.clone() {
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
                    source_ml_source2_data_property_value27657,
                ) => ((source_ml_source2_data_property_value27657
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,)))
                .0)
                    .clone(),
            };
            {
                let prior_conversion = match source_ml_source1_temporary_binding1237.clone() {
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
                        source_ml_source2_data_property_value27658,
                        _,
                    ) => ((source_ml_source2_data_property_value27658
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,)))
                    .0)
                        .clone(),
                };
                {
                    let base = match source_ml_source1_temporary_binding1237.clone() {
                        self::JavaScriptArraySearchState::ArraySearchState(
                            source_ml_source2_nominal_rest_field27659,
                            source_ml_source2_nominal_rest_field27660,
                            source_ml_source2_nominal_rest_field27661,
                            source_ml_source2_nominal_rest_field27662,
                            source_ml_source2_nominal_rest_field27663,
                            source_ml_source2_nominal_rest_field27664,
                            source_ml_source2_nominal_rest_field27665,
                            source_ml_source2_nominal_rest_field27666,
                            source_ml_source2_nominal_rest_field27667,
                            source_ml_source2_nominal_rest_field27668,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_nominal_rest_field27660 =
                                *source_ml_source2_nominal_rest_field27660;
                            let source_ml_source2_nominal_rest_field27661 =
                                *source_ml_source2_nominal_rest_field27661;
                            let source_ml_source2_nominal_rest_field27662 =
                                *source_ml_source2_nominal_rest_field27662;
                            let source_ml_source2_nominal_rest_field27664 =
                                *source_ml_source2_nominal_rest_field27664;
                            let source_ml_source2_nominal_rest_field27668 =
                                *source_ml_source2_nominal_rest_field27668;
                            self::JavaScriptArraySearchState::ArraySearchState(
                                source_ml_source2_nominal_rest_field27659.clone(),
                                Box::new(source_ml_source2_nominal_rest_field27660.clone()),
                                Box::new(source_ml_source2_nominal_rest_field27661.clone()),
                                Box::new(source_ml_source2_nominal_rest_field27662.clone()),
                                source_ml_source2_nominal_rest_field27663,
                                Box::new(source_ml_source2_nominal_rest_field27664.clone()),
                                source_ml_source2_nominal_rest_field27665,
                                source_ml_source2_nominal_rest_field27666,
                                source_ml_source2_nominal_rest_field27667.clone(),
                                Box::new(source_ml_source2_nominal_rest_field27668.clone()),
                                None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                                None::<(Option<self::JavaScriptArraySearchCall>,)>,
                            )
                        }
                    };
                    match base.clone() {
                        self::JavaScriptArraySearchState::ArraySearchState(
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
                        ) => {
                            let shared_constructor_field14 = *shared_constructor_field14;
                            let shared_constructor_field15 = *shared_constructor_field15;
                            let shared_constructor_field16 = *shared_constructor_field16;
                            let shared_constructor_field18 = *shared_constructor_field18;
                            let shared_constructor_field22 = *shared_constructor_field22;
                            {
                                let length_phase = (((((match state.clone() {
                                    self::JavaScriptArraySearchState::ArraySearchState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value27670,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value27670.clone(),
                                })
                                    == (self::ml_string::Text::from_units(vec![
                                        108, 101, 110, 103, 116, 104,
                                    ])))
                                    || ((match state.clone() {
                                        self::JavaScriptArraySearchState::ArraySearchState(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value27672,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value27672.clone(),
                                    }) == (self::ml_string::Text::from_units(vec![
                                        108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101,
                                        114,
                                    ]))))
                                    || ((match state.clone() {
                                        self::JavaScriptArraySearchState::ArraySearchState(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value27674,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value27674.clone(),
                                    }) == (self::ml_string::Text::from_units(vec![
                                        108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101,
                                        114, 115, 105, 111, 110,
                                    ]))))
                                    || ((match state.clone() {
                                        self::JavaScriptArraySearchState::ArraySearchState(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value27676,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value27676.clone(),
                                    }) == (self::ml_string::Text::from_units(vec![
                                        108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105,
                                        116, 105, 118, 101,
                                    ]))));
                                match transition.clone() {
                                    self::JavaScriptScalarConversionTransition::ScalarContinuation(state_2) => {
                                        let state_2 = *state_2;
                                        self::JavaScriptArraySearchTransition::ArraySearchContinue(Box::new(heap.clone()), Box::new({
                                            let source_ml_source2_nominal_field27678 = shared_constructor_field13.clone();
                                            {
                                                let source_ml_source2_nominal_field27680 = shared_constructor_field14.clone();
                                                {
                                                    let source_ml_source2_nominal_field27682 = shared_constructor_field15.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27684 = shared_constructor_field16.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27686 = shared_constructor_field17;
                                                            {
                                                                let source_ml_source2_nominal_field27688 = shared_constructor_field18.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27690 = shared_constructor_field19;
                                                                    {
                                                                        let source_ml_source2_nominal_field27692 = shared_constructor_field20;
                                                                        {
                                                                            let source_ml_source2_nominal_field27694 = shared_constructor_field21.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field27696 = shared_constructor_field22.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field27698 = shared_constructor_field23.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field27700 = shared_constructor_field24.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override27702 = if length_phase {
                                                                                                self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])
                                                                                            };
                                                                                            self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field27678.clone(), Box::new(source_ml_source2_nominal_field27680.clone()), Box::new(source_ml_source2_nominal_field27682.clone()), Box::new(source_ml_source2_nominal_field27684.clone()), source_ml_source2_nominal_field27686, Box::new(source_ml_source2_nominal_field27688.clone()), source_ml_source2_nominal_field27690, source_ml_source2_nominal_field27692, source_ml_source2_nominal_override27702.clone(), Box::new(source_ml_source2_nominal_field27696.clone()), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field27700.clone())
                                                                                        }
                                                                                    }
                                                                                }
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
                                        self::request_java_script_array_search_call(heap.clone(), {
                                            let source_ml_source2_nominal_field27705 = shared_constructor_field13.clone();
                                            {
                                                let source_ml_source2_nominal_field27707 = shared_constructor_field14.clone();
                                                {
                                                    let source_ml_source2_nominal_field27709 = shared_constructor_field15.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27711 = shared_constructor_field16.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27713 = shared_constructor_field17;
                                                            {
                                                                let source_ml_source2_nominal_field27715 = shared_constructor_field18.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27717 = shared_constructor_field19;
                                                                    {
                                                                        let source_ml_source2_nominal_field27719 = shared_constructor_field20;
                                                                        {
                                                                            let source_ml_source2_nominal_field27721 = shared_constructor_field21.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field27723 = shared_constructor_field22.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field27725 = shared_constructor_field23.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field27727 = shared_constructor_field24.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override27729 = if length_phase {
                                                                                                self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])
                                                                                            };
                                                                                            self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field27705.clone(), Box::new(source_ml_source2_nominal_field27707.clone()), Box::new(source_ml_source2_nominal_field27709.clone()), Box::new(source_ml_source2_nominal_field27711.clone()), source_ml_source2_nominal_field27713, Box::new(source_ml_source2_nominal_field27715.clone()), source_ml_source2_nominal_field27717, source_ml_source2_nominal_field27719, source_ml_source2_nominal_override27729.clone(), Box::new(source_ml_source2_nominal_field27723.clone()), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field27727.clone())
                                                                                        }
                                                                                    }
                                                                                }
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
                                                if length_phase {
                                                    {
                                                        let length = self::read_java_script_array_like_length(value);
                                                        {
                                                            let next = {
                                                                let source_ml_source2_nominal_field27733 = shared_constructor_field13.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27735 = shared_constructor_field14.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field27737 = shared_constructor_field15.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field27739 = shared_constructor_field16.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field27741 = shared_constructor_field17;
                                                                                {
                                                                                    let source_ml_source2_nominal_field27743 = shared_constructor_field18.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field27745 = shared_constructor_field19;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field27747 = shared_constructor_field20;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field27749 = shared_constructor_field21.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field27751 = shared_constructor_field22.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field27753 = shared_constructor_field23.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field27755 = shared_constructor_field24.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override27758 = match state.clone() {
                                                                                                                    self::JavaScriptArraySearchState::ArraySearchState(_, _, _, source_ml_source2_data_property_value27731, _, _, _, _, _, _, _, _) => {
                                                                                                                        let source_ml_source2_data_property_value27731 = *source_ml_source2_data_property_value27731;
                                                                                                                        source_ml_source2_data_property_value27731.clone()
                                                                                                                    }
                                                                                                                };
                                                                                                                self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field27733.clone(), Box::new(source_ml_source2_nominal_field27735.clone()), Box::new(source_ml_source2_nominal_field27737.clone()), Box::new(source_ml_source2_nominal_field27739.clone()), source_ml_source2_nominal_field27741, Box::new(source_ml_source2_nominal_field27743.clone()), length, source_ml_source2_nominal_field27747, self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101]), Box::new(source_ml_source2_nominal_override27758.clone()), source_ml_source2_nominal_field27753.clone(), source_ml_source2_nominal_field27755.clone())
                                                                                                            }
                                                                                                        }
                                                                                                    }
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
                                                                self::JavaScriptArraySearchState::ArraySearchState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12) => {
                                                                    let shared_constructor_field2 = *shared_constructor_field2;
                                                                    let shared_constructor_field3 = *shared_constructor_field3;
                                                                    let shared_constructor_field4 = *shared_constructor_field4;
                                                                    let shared_constructor_field6 = *shared_constructor_field6;
                                                                    let shared_constructor_field10 = *shared_constructor_field10;
                                                                    if ((length == (0f64)) && ((match state.clone() {
                                                                        self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_data_property_value27761, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value27761.clone()
                                                                        }
                                                                    }) != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 97, 116])))) {
                                                                        self::JavaScriptArraySearchTransition::ArraySearchCompleted(Box::new(heap.clone()), Box::new(self::read_java_script_array_search_missing_value(state.clone())))
                                                                    } else {
                                                                        if (!(match state.clone() {
                                                                            self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, source_ml_source2_data_property_value27762, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value27762
                                                                            }
                                                                        }) && ((match state.clone() {
                                                                            self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_data_property_value27764, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value27764.clone()
                                                                            }
                                                                        }) != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 97, 116])))) {
                                                                            self::JavaScriptArraySearchTransition::ArraySearchContinue(Box::new(heap.clone()), Box::new({
                                                                                let source_ml_source2_nominal_field27767 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field27769 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field27771 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field27773 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field27775 = shared_constructor_field5;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field27777 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field27779 = shared_constructor_field7;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field27781 = shared_constructor_field8;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field27783 = shared_constructor_field9.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field27785 = shared_constructor_field10.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field27787 = shared_constructor_field11.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field27789 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override27791 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override27793 = self::read_java_script_array_search_index(&(match state.clone() {
                                                                                                                                        self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_data_property_value27765, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                            source_ml_source2_data_property_value27765.clone()
                                                                                                                                        }
                                                                                                                                    }), length, false, 0f64);
                                                                                                                                    self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field27767.clone(), Box::new(source_ml_source2_nominal_field27769.clone()), Box::new(source_ml_source2_nominal_field27771.clone()), Box::new(source_ml_source2_nominal_field27773.clone()), source_ml_source2_nominal_field27775, Box::new(source_ml_source2_nominal_field27777.clone()), source_ml_source2_nominal_field27779, source_ml_source2_nominal_override27793, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), Box::new(source_ml_source2_nominal_override27791.clone()), source_ml_source2_nominal_field27787.clone(), source_ml_source2_nominal_field27789.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
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
                                                                            self::continue_java_script_array_search_number(heap.clone(), next.clone(), self::begin_java_script_scalar_protocol(heap.clone(), match state.clone() {
                                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, source_ml_source2_data_property_value27794, _, _, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value27794 = *source_ml_source2_data_property_value27794;
                                                                                    source_ml_source2_data_property_value27794.clone()
                                                                                }
                                                                            }, match state.clone() {
                                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, source_ml_source2_data_property_value27795, _, _, _, _, _, _) => {
                                                                                    let source_ml_source2_data_property_value27795 = *source_ml_source2_data_property_value27795;
                                                                                    source_ml_source2_data_property_value27795.clone()
                                                                                }
                                                                            }, &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    self::JavaScriptArraySearchTransition::ArraySearchContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field27800 = shared_constructor_field13.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27802 = shared_constructor_field14.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27804 = shared_constructor_field15.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27806 = shared_constructor_field16.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field27808 = shared_constructor_field17;
                                                                        {
                                                                            let source_ml_source2_nominal_field27810 = shared_constructor_field18.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field27812 = shared_constructor_field19;
                                                                                {
                                                                                    let source_ml_source2_nominal_field27814 = shared_constructor_field20;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field27816 = shared_constructor_field21.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field27818 = shared_constructor_field22.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field27820 = shared_constructor_field23.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field27822 = shared_constructor_field24.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override27825 = self::JavaScriptSemanticValue::Undefined;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override27826 = self::read_java_script_array_search_index(&(match state.clone() {
                                                                                                                self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_data_property_value27796, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value27796.clone()
                                                                                                                }
                                                                                                            }), match state.clone() {
                                                                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, source_ml_source2_data_property_value27797, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value27797
                                                                                                                }
                                                                                                            }, match state.clone() {
                                                                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, source_ml_source2_data_property_value27798, _, _, _, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value27798
                                                                                                                }
                                                                                                            }, value);
                                                                                                            self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field27800.clone(), Box::new(source_ml_source2_nominal_field27802.clone()), Box::new(source_ml_source2_nominal_field27804.clone()), Box::new(source_ml_source2_nominal_field27806.clone()), source_ml_source2_nominal_field27808, Box::new(source_ml_source2_nominal_field27810.clone()), source_ml_source2_nominal_field27812, source_ml_source2_nominal_override27826, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), Box::new(source_ml_source2_nominal_override27825.clone()), source_ml_source2_nominal_field27820.clone(), source_ml_source2_nominal_field27822.clone())
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
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
                                            _ => {
                                                self::JavaScriptArraySearchTransition::ArraySearchInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                                            }
                                        }
                                    }
                                    self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptArraySearchTransition::ArraySearchThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
                                    }
                                    self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
                                        self::JavaScriptArraySearchTransition::ArraySearchError(Box::new(heap.clone()), error_class.clone(), reason.clone())
                                    }
                                    self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                                        self::JavaScriptArraySearchTransition::ArraySearchInvalid(Box::new(heap.clone()), reason_2.clone())
                                    }
                                    self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
                                        let value_3 = *value_3;
                                        self::JavaScriptArraySearchTransition::ArraySearchResourceLimit(Box::new(heap.clone()), Box::new({
                                            let source_ml_source2_nominal_field27828 = shared_constructor_field13.clone();
                                            {
                                                let source_ml_source2_nominal_field27830 = shared_constructor_field14.clone();
                                                {
                                                    let source_ml_source2_nominal_field27832 = shared_constructor_field15.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27834 = shared_constructor_field16.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27836 = shared_constructor_field17;
                                                            {
                                                                let source_ml_source2_nominal_field27838 = shared_constructor_field18.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27840 = shared_constructor_field19;
                                                                    {
                                                                        let source_ml_source2_nominal_field27842 = shared_constructor_field20;
                                                                        {
                                                                            let source_ml_source2_nominal_field27844 = shared_constructor_field21.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field27846 = shared_constructor_field22.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field27848 = shared_constructor_field23.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field27850 = shared_constructor_field24.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override27852 = if length_phase {
                                                                                                self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101])
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101])
                                                                                            };
                                                                                            self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field27828.clone(), Box::new(source_ml_source2_nominal_field27830.clone()), Box::new(source_ml_source2_nominal_field27832.clone()), Box::new(source_ml_source2_nominal_field27834.clone()), source_ml_source2_nominal_field27836, Box::new(source_ml_source2_nominal_field27838.clone()), source_ml_source2_nominal_field27840, source_ml_source2_nominal_field27842, source_ml_source2_nominal_override27852.clone(), Box::new(value_3.clone()), source_ml_source2_nominal_field27848.clone(), source_ml_source2_nominal_field27850.clone())
                                                                                        }
                                                                                    }
                                                                                }
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_array_search(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    receiver: self::JavaScriptSemanticValue,
    search_element: self::JavaScriptSemanticValue,
    from_index: self::JavaScriptSemanticValue,
    from_present: bool,
    exotic_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptArraySearchTransition {
    if (match receiver.clone() {
        self::JavaScriptSemanticValue::Null => true,
        _ => false,
    } || match receiver.clone() {
        self::JavaScriptSemanticValue::Undefined => true,
        _ => false,
    }) {
        self::JavaScriptArraySearchTransition::ArraySearchError(
            Box::new(heap.clone()),
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 99, 97, 108, 108, 101,
                100, 32, 111, 110, 32, 110, 117, 108, 108, 32, 111, 114, 32, 117, 110, 100, 101,
                102, 105, 110, 101, 100,
            ]),
        )
    } else {
        {
            let state = {
                let source_ml_source2_constructor_field27860 = 0f64;
                {
                    let source_ml_source2_constructor_field27861 = 0f64;
                    {
                        let source_ml_source2_constructor_field27863 =
                            self::JavaScriptSemanticValue::Undefined;
                        self::JavaScriptArraySearchState::ArraySearchState(
                            operation.clone(),
                            Box::new(receiver.clone()),
                            Box::new(search_element.clone()),
                            Box::new(from_index.clone()),
                            from_present,
                            Box::new(exotic_key.clone()),
                            source_ml_source2_constructor_field27860,
                            source_ml_source2_constructor_field27861,
                            self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]),
                            Box::new(source_ml_source2_constructor_field27863.clone()),
                            None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                            None::<(Option<self::JavaScriptArraySearchCall>,)>,
                        )
                    }
                }
            };
            {
                let error =
                    self::read_java_script_array_search_state_error(heap.clone(), state.clone());
                if (error == (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArraySearchTransition::ArraySearchContinue(
                        Box::new(heap.clone()),
                        Box::new(state.clone()),
                    )
                } else {
                    self::JavaScriptArraySearchTransition::ArraySearchInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn complete_java_script_array_search_element(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySearchState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArraySearchTransition {
    match state.clone() {
        self::JavaScriptArraySearchState::ArraySearchState(
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
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field10 = *shared_constructor_field10;
            if (shared_constructor_field1
                == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 97, 116])))
            {
                self::JavaScriptArraySearchTransition::ArraySearchCompleted(
                    Box::new(heap.clone()),
                    Box::new(value.clone()),
                )
            } else {
                {
                    let found = if (shared_constructor_field1
                        == (self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 46, 105, 110, 99, 108, 117, 100, 101, 115,
                        ]))) {
                        self::test_java_script_collection_key(
                            value.clone(),
                            shared_constructor_field3.clone(),
                        )
                    } else {
                        self::test_java_script_call_strict_equal(
                            value.clone(),
                            shared_constructor_field3.clone(),
                        )
                    };
                    if found {
                        self::JavaScriptArraySearchTransition::ArraySearchCompleted(
                            Box::new(heap.clone()),
                            Box::new(
                                if (shared_constructor_field1
                                    == (self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 46, 105, 110, 99, 108, 117, 100,
                                        101, 115,
                                    ])))
                                {
                                    self::JavaScriptSemanticValue::Boolean(true)
                                } else {
                                    self::JavaScriptSemanticValue::Number(shared_constructor_field8)
                                },
                            ),
                        )
                    } else {
                        self::JavaScriptArraySearchTransition::ArraySearchContinue(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field27877 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field27879 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field27881 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field27883 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field27885 =
                                                    shared_constructor_field5;
                                                {
                                                    let source_ml_source2_nominal_field27887 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27889 =
                                                            shared_constructor_field7;
                                                        {
                                                            let source_ml_source2_nominal_field27891 =
                                                                shared_constructor_field8;
                                                            {
                                                                let source_ml_source2_nominal_field27893 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27895 =
                                                                        shared_constructor_field10
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field27897 = shared_constructor_field11.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field27899 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override27902 = (shared_constructor_field8 + if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102]))) {
                                                                                (-1f64)
                                                                            } else {
                                                                                1f64
                                                                            });
                                                                                self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field27877.clone(), Box::new(source_ml_source2_nominal_field27879.clone()), Box::new(source_ml_source2_nominal_field27881.clone()), Box::new(source_ml_source2_nominal_field27883.clone()), source_ml_source2_nominal_field27885, Box::new(source_ml_source2_nominal_field27887.clone()), source_ml_source2_nominal_field27889, source_ml_source2_nominal_override27902, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), Box::new(source_ml_source2_nominal_field27895.clone()), source_ml_source2_nominal_field27897.clone(), source_ml_source2_nominal_field27899.clone())
                                                                            }
                                                                        }
                                                                    }
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
                }
            }
        }
    }
}
