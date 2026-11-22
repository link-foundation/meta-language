// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayCreationCall;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::accept_java_script_array_creation_iterator;
use super::begin_java_script_synchronous_iterator_close;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn clear_java_script_array_creation_child(
    state: self::JavaScriptArrayCreationState,
) -> self::JavaScriptArrayCreationState {
    {
        let call = match state.clone() {
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
                _,
                source_ml_source2_data_property_value14675,
                _,
                _,
                _,
            ) => ((source_ml_source2_data_property_value14675
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,)))
            .0)
                .clone(),
        };
        {
            let iterator = match state.clone() {
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
                    source_ml_source2_data_property_value14676,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => ((source_ml_source2_data_property_value14676
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,)))
                .0)
                    .clone(),
            };
            {
                let conversion = match state.clone() {
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
                        source_ml_source2_data_property_value14677,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value14677
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,)))
                    .0)
                        .clone(),
                };
                {
                    let converted = match state.clone() {
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
                            source_ml_source2_data_property_value14678,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value14678
                            .clone()
                            .unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,)))
                        .0)
                            .clone(),
                    };
                    {
                        let wrapper = match state.clone() {
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
                                source_ml_source2_data_property_value14679,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value14679
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<self::JavaScriptPrimitiveWrapperState>,)
                                }))
                            .0)
                                .clone(),
                        };
                        {
                            let assignment = match state.clone() {
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
                                    source_ml_source2_data_property_value14680,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value14680
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptArrayAssignmentState>,)
                                    }))
                                .0)
                                    .clone(),
                            };
                            {
                                let abrupt = match state.clone() {
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
                                        _,
                                        _,
                                        source_ml_source2_data_property_value14681,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value14681
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptSemanticValue>,)
                                        }))
                                    .0)
                                        .clone(),
                                };
                                {
                                    let error_class = match state.clone() {
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
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value14682,
                                            _,
                                        ) => ((source_ml_source2_data_property_value14682
                                            .clone()
                                            .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                                        .0)
                                            .clone(),
                                    };
                                    {
                                        let reason = match state.clone() {
                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14683) => {
                                                ((source_ml_source2_data_property_value14683.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                            }
                                        };
                                        {
                                            let base = match state.clone() {
                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_rest_field14684, source_ml_source2_nominal_rest_field14685, source_ml_source2_nominal_rest_field14686, source_ml_source2_nominal_rest_field14687, source_ml_source2_nominal_rest_field14688, source_ml_source2_nominal_rest_field14689, source_ml_source2_nominal_rest_field14690, source_ml_source2_nominal_rest_field14691, source_ml_source2_nominal_rest_field14692, source_ml_source2_nominal_rest_field14693, source_ml_source2_nominal_rest_field14694, source_ml_source2_nominal_rest_field14695, source_ml_source2_nominal_rest_field14696, source_ml_source2_nominal_rest_field14697, source_ml_source2_nominal_rest_field14698, source_ml_source2_nominal_rest_field14699, source_ml_source2_nominal_rest_field14700, source_ml_source2_nominal_rest_field14701, source_ml_source2_nominal_rest_field14702, source_ml_source2_nominal_rest_field14703, source_ml_source2_nominal_rest_field14704, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_nominal_rest_field14685 = *source_ml_source2_nominal_rest_field14685;
                                                    let source_ml_source2_nominal_rest_field14687 = *source_ml_source2_nominal_rest_field14687;
                                                    let source_ml_source2_nominal_rest_field14688 = *source_ml_source2_nominal_rest_field14688;
                                                    let source_ml_source2_nominal_rest_field14689 = *source_ml_source2_nominal_rest_field14689;
                                                    let source_ml_source2_nominal_rest_field14690 = *source_ml_source2_nominal_rest_field14690;
                                                    let source_ml_source2_nominal_rest_field14691 = *source_ml_source2_nominal_rest_field14691;
                                                    let source_ml_source2_nominal_rest_field14692 = *source_ml_source2_nominal_rest_field14692;
                                                    let source_ml_source2_nominal_rest_field14698 = *source_ml_source2_nominal_rest_field14698;
                                                    let source_ml_source2_nominal_rest_field14699 = *source_ml_source2_nominal_rest_field14699;
                                                    let source_ml_source2_nominal_rest_field14700 = *source_ml_source2_nominal_rest_field14700;
                                                    let source_ml_source2_nominal_rest_field14701 = *source_ml_source2_nominal_rest_field14701;
                                                    let source_ml_source2_nominal_rest_field14702 = *source_ml_source2_nominal_rest_field14702;
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_rest_field14684.clone(), Box::new(source_ml_source2_nominal_rest_field14685.clone()), source_ml_source2_nominal_rest_field14686.clone(), Box::new(source_ml_source2_nominal_rest_field14687.clone()), Box::new(source_ml_source2_nominal_rest_field14688.clone()), Box::new(source_ml_source2_nominal_rest_field14689.clone()), Box::new(source_ml_source2_nominal_rest_field14690.clone()), Box::new(source_ml_source2_nominal_rest_field14691.clone()), Box::new(source_ml_source2_nominal_rest_field14692.clone()), source_ml_source2_nominal_rest_field14693, source_ml_source2_nominal_rest_field14694, source_ml_source2_nominal_rest_field14695.clone(), source_ml_source2_nominal_rest_field14696, source_ml_source2_nominal_rest_field14697, Box::new(source_ml_source2_nominal_rest_field14698.clone()), Box::new(source_ml_source2_nominal_rest_field14699.clone()), Box::new(source_ml_source2_nominal_rest_field14700.clone()), Box::new(source_ml_source2_nominal_rest_field14701.clone()), Box::new(source_ml_source2_nominal_rest_field14702.clone()), source_ml_source2_nominal_rest_field14703.clone(), source_ml_source2_nominal_rest_field14704.clone(), None::<(Option<self::JavaScriptIteratorProtocolState>,)>, None::<(Option<self::JavaScriptPrimitiveConversionState>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptPrimitiveWrapperState>,)>, None::<(Option<self::JavaScriptArrayAssignmentState>,)>, None::<(Option<self::JavaScriptArrayCreationCall>,)>, None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::ml_string::Text>,)>, None::<(Option<self::ml_string::Text>,)>)
                                                }
                                            };
                                            base.clone()
                                        }
                                    }
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
pub fn throw_java_script_array_creation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    value: self::JavaScriptSemanticValue,
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
                if (match (
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
                            source_ml_source2_data_property_value14705,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => ((source_ml_source2_data_property_value14705
                            .clone()
                            .unwrap_or_else(|| {
                                (None::<self::JavaScriptSynchronousIteratorRecord>,)
                            }))
                        .0)
                            .clone(),
                    },
                    None::<()>,
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } || match (match state.clone() {
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
                        source_ml_source2_data_property_value14706,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source2_data_property_value14706
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,)))
                    .0)
                        .clone(),
                })
                .clone()
                .expect("a proven present value")
                {
                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
                        _,
                        _,
                        source_ml_source2_data_property_value14707,
                    ) => source_ml_source2_data_property_value14707,
                }) {
                    self::JavaScriptArrayCreationTransition::ArrayCreationThrow(
                        Box::new(heap.clone()),
                        Box::new(value.clone()),
                    )
                } else {
                    self::accept_java_script_array_creation_iterator(
                        {
                            let source_ml_source2_nominal_field14709 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field14711 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field14713 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field14715 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field14717 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field14719 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field14721 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field14723 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field14725 =
                                                                shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field14727 =
                                                                    shared_constructor_field10;
                                                                {
                                                                    let source_ml_source2_nominal_field14729 =
                                                                        shared_constructor_field11;
                                                                    {
                                                                        let source_ml_source2_nominal_field14731 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field14733 = shared_constructor_field13;
                                                                            {
                                                                                let source_ml_source2_nominal_field14735 = shared_constructor_field14;
                                                                                {
                                                                                    let source_ml_source2_nominal_field14737 = shared_constructor_field15.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field14739 = shared_constructor_field16.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field14741 = shared_constructor_field17.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field14743 = shared_constructor_field18.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field14745 = shared_constructor_field19.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field14747 = shared_constructor_field20.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field14749 = shared_constructor_field21.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field14751 = shared_constructor_field22.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field14753 = shared_constructor_field23.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field14755 = shared_constructor_field24.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field14757 = shared_constructor_field25.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field14759 = shared_constructor_field26.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field14761 = shared_constructor_field27.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field14763 = shared_constructor_field28.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field14765 = shared_constructor_field29.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field14767 = shared_constructor_field30.clone();
                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field14709.clone(), Box::new(source_ml_source2_nominal_field14711.clone()), source_ml_source2_nominal_field14713.clone(), Box::new(source_ml_source2_nominal_field14715.clone()), Box::new(source_ml_source2_nominal_field14717.clone()), Box::new(source_ml_source2_nominal_field14719.clone()), Box::new(source_ml_source2_nominal_field14721.clone()), Box::new(source_ml_source2_nominal_field14723.clone()), Box::new(source_ml_source2_nominal_field14725.clone()), source_ml_source2_nominal_field14727, source_ml_source2_nominal_field14729, self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]), source_ml_source2_nominal_field14733, source_ml_source2_nominal_field14735, Box::new(source_ml_source2_nominal_field14737.clone()), Box::new(source_ml_source2_nominal_field14739.clone()), Box::new(source_ml_source2_nominal_field14741.clone()), Box::new(source_ml_source2_nominal_field14743.clone()), Box::new(source_ml_source2_nominal_field14745.clone()), source_ml_source2_nominal_field14747.clone(), source_ml_source2_nominal_field14749.clone(), source_ml_source2_nominal_field14751.clone(), source_ml_source2_nominal_field14753.clone(), source_ml_source2_nominal_field14755.clone(), source_ml_source2_nominal_field14757.clone(), source_ml_source2_nominal_field14759.clone(), source_ml_source2_nominal_field14761.clone(), Some((Some(value.clone()),)), source_ml_source2_nominal_field14765.clone(), source_ml_source2_nominal_field14767.clone())
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                        self::begin_java_script_synchronous_iterator_close(
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
                                    source_ml_source2_data_property_value14771,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value14771
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptSynchronousIteratorRecord>,)
                                    }))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value"),
                            self::JavaScriptIteratorCompletion::IteratorCompletion(
                                self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]),
                                Box::new(value.clone()),
                                (-1f64),
                            ),
                        ),
                    )
                }
            }
        }
    }
}
