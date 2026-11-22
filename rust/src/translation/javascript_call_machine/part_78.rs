// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationCall;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::clear_java_script_array_creation_child;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn fail_java_script_array_creation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    error_class: &self::ml_string::Text,
    reason: &self::ml_string::Text,
) -> self::JavaScriptArrayCreationTransition {
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
                source_ml_source2_data_property_value14772,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => ((source_ml_source2_data_property_value14772
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,)))
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
            source_ml_source2_data_property_value14773,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => ((source_ml_source2_data_property_value14773
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
            source_ml_source2_data_property_value14774,
        ) => source_ml_source2_data_property_value14774,
    }) {
        self::JavaScriptArrayCreationTransition::ArrayCreationError(
            Box::new(heap.clone()),
            error_class.clone(),
            reason.clone(),
        )
    } else {
        self::JavaScriptArrayCreationTransition::ArrayCreationNeedError(
            Box::new(heap.clone()),
            Box::new({
                let source_ml_source2_nominal_spread14775 =
                    self::clear_java_script_array_creation_child(state.clone());
                match source_ml_source2_nominal_spread14775.clone() {
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
                            let source_ml_source2_nominal_field14776 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field14778 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field14780 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field14782 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field14784 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field14786 =
                                                    shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field14788 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_nominal_field14790 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field14792 =
                                                                shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field14794 =
                                                                    shared_constructor_field10;
                                                                {
                                                                    let source_ml_source2_nominal_field14796 =
                                                                        shared_constructor_field11;
                                                                    {
                                                                        let source_ml_source2_nominal_field14798 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field14800 = shared_constructor_field13;
                                                                            {
                                                                                let source_ml_source2_nominal_field14802 = shared_constructor_field14;
                                                                                {
                                                                                    let source_ml_source2_nominal_field14804 = shared_constructor_field15.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field14806 = shared_constructor_field16.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field14808 = shared_constructor_field17.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field14810 = shared_constructor_field18.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field14812 = shared_constructor_field19.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field14814 = shared_constructor_field20.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field14816 = shared_constructor_field21.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field14818 = shared_constructor_field22.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field14820 = shared_constructor_field23.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field14822 = shared_constructor_field24.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field14824 = shared_constructor_field25.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field14826 = shared_constructor_field26.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field14828 = shared_constructor_field27.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field14830 = shared_constructor_field28.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field14832 = shared_constructor_field29.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field14834 = shared_constructor_field30.clone();
                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field14776.clone(), Box::new(source_ml_source2_nominal_field14778.clone()), source_ml_source2_nominal_field14780.clone(), Box::new(source_ml_source2_nominal_field14782.clone()), Box::new(source_ml_source2_nominal_field14784.clone()), Box::new(source_ml_source2_nominal_field14786.clone()), Box::new(source_ml_source2_nominal_field14788.clone()), Box::new(source_ml_source2_nominal_field14790.clone()), Box::new(source_ml_source2_nominal_field14792.clone()), source_ml_source2_nominal_field14794, source_ml_source2_nominal_field14796, self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114]), source_ml_source2_nominal_field14800, source_ml_source2_nominal_field14802, Box::new(source_ml_source2_nominal_field14804.clone()), Box::new(source_ml_source2_nominal_field14806.clone()), Box::new(source_ml_source2_nominal_field14808.clone()), Box::new(source_ml_source2_nominal_field14810.clone()), Box::new(source_ml_source2_nominal_field14812.clone()), source_ml_source2_nominal_field14814.clone(), source_ml_source2_nominal_field14816.clone(), source_ml_source2_nominal_field14818.clone(), source_ml_source2_nominal_field14820.clone(), source_ml_source2_nominal_field14822.clone(), source_ml_source2_nominal_field14824.clone(), source_ml_source2_nominal_field14826.clone(), source_ml_source2_nominal_field14828.clone(), source_ml_source2_nominal_field14830.clone(), Some((Some(error_class.clone()),)), Some((Some(reason.clone()),)))
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
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
            error_class.clone(),
            reason.clone(),
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_array_creation_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
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
            {
                let call = self::JavaScriptArrayCreationCall::ArrayCreationCallRecord(
                    Box::new(callee.clone()),
                    Box::new(receiver.clone()),
                    arguments_.clone(),
                );
                self::JavaScriptArrayCreationTransition::ArrayCreationCall(
                    Box::new(heap.clone()),
                    Box::new({
                        let source_ml_source2_nominal_field14840 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_nominal_field14842 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_nominal_field14844 =
                                    shared_constructor_field3.clone();
                                {
                                    let source_ml_source2_nominal_field14846 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_nominal_field14848 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_nominal_field14850 =
                                                shared_constructor_field6.clone();
                                            {
                                                let source_ml_source2_nominal_field14852 =
                                                    shared_constructor_field7.clone();
                                                {
                                                    let source_ml_source2_nominal_field14854 =
                                                        shared_constructor_field8.clone();
                                                    {
                                                        let source_ml_source2_nominal_field14856 =
                                                            shared_constructor_field9.clone();
                                                        {
                                                            let source_ml_source2_nominal_field14858 =
                                                                shared_constructor_field10;
                                                            {
                                                                let source_ml_source2_nominal_field14860 =
                                                                    shared_constructor_field11;
                                                                {
                                                                    let source_ml_source2_nominal_field14862 =
                                                                        shared_constructor_field12
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field14864 = shared_constructor_field13;
                                                                        {
                                                                            let source_ml_source2_nominal_field14866 = shared_constructor_field14;
                                                                            {
                                                                                let source_ml_source2_nominal_field14868 = shared_constructor_field15.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field14870 = shared_constructor_field16.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field14872 = shared_constructor_field17.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field14874 = shared_constructor_field18.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field14876 = shared_constructor_field19.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field14878 = shared_constructor_field20.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field14880 = shared_constructor_field21.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field14882 = shared_constructor_field22.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field14884 = shared_constructor_field23.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field14886 = shared_constructor_field24.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field14888 = shared_constructor_field25.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field14890 = shared_constructor_field26.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field14892 = shared_constructor_field27.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field14894 = shared_constructor_field28.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field14896 = shared_constructor_field29.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field14898 = shared_constructor_field30.clone();
                                                                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field14840.clone(), Box::new(source_ml_source2_nominal_field14842.clone()), source_ml_source2_nominal_field14844.clone(), Box::new(source_ml_source2_nominal_field14846.clone()), Box::new(source_ml_source2_nominal_field14848.clone()), Box::new(source_ml_source2_nominal_field14850.clone()), Box::new(source_ml_source2_nominal_field14852.clone()), Box::new(source_ml_source2_nominal_field14854.clone()), Box::new(source_ml_source2_nominal_field14856.clone()), source_ml_source2_nominal_field14858, source_ml_source2_nominal_field14860, source_ml_source2_nominal_field14862.clone(), source_ml_source2_nominal_field14864, source_ml_source2_nominal_field14866, Box::new(source_ml_source2_nominal_field14868.clone()), Box::new(source_ml_source2_nominal_field14870.clone()), Box::new(source_ml_source2_nominal_field14872.clone()), Box::new(source_ml_source2_nominal_field14874.clone()), Box::new(source_ml_source2_nominal_field14876.clone()), source_ml_source2_nominal_field14878.clone(), source_ml_source2_nominal_field14880.clone(), source_ml_source2_nominal_field14882.clone(), source_ml_source2_nominal_field14884.clone(), source_ml_source2_nominal_field14886.clone(), source_ml_source2_nominal_field14888.clone(), source_ml_source2_nominal_field14890.clone(), Some((Some(call.clone()),)), source_ml_source2_nominal_field14894.clone(), source_ml_source2_nominal_field14896.clone(), source_ml_source2_nominal_field14898.clone())
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
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
                    Box::new(callee.clone()),
                    Box::new(receiver.clone()),
                    arguments_.clone(),
                )
            }
        }
    }
}
