// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveWrapperTransition;
use super::JavaScriptSemanticValue;
use super::accept_java_script_array_creation_get;
use super::clear_java_script_array_creation_child;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::request_java_script_array_creation_call;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_creation_property(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    object: self::JavaScriptSemanticValue,
    key: self::JavaScriptSemanticValue,
    phase: &self::ml_string::Text,
    receiver: self::JavaScriptSemanticValue,
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
            match object.clone() {
                self::JavaScriptSemanticValue::Object(identity, _) => {
                    let get = self::plan_java_script_ordinary_get(
                        heap.clone(),
                        identity.clone(),
                        key.clone(),
                        receiver.clone(),
                    );
                    if (((({
                        let ml_reference_value853 = get.borrow().clone();
                        ml_reference_value853
                    })
                    .0)
                        .3)
                        != (self::ml_string::Text::from_units(vec![])))
                    {
                        self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                            Box::new(heap.clone()),
                            (({
                                let ml_reference_value854 = get.borrow().clone();
                                ml_reference_value854
                            })
                            .0)
                                .3,
                        )
                    } else {
                        if (((({
                            let ml_reference_value855 = get.borrow().clone();
                            ml_reference_value855
                        })
                        .0)
                            .0)
                            == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])))
                        {
                            self::accept_java_script_array_creation_get(
                                heap.clone(),
                                {
                                    let source_ml_source2_nominal_field15827 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source2_nominal_field15829 =
                                            shared_constructor_field2.clone();
                                        {
                                            let source_ml_source2_nominal_field15831 =
                                                shared_constructor_field3.clone();
                                            {
                                                let source_ml_source2_nominal_field15833 =
                                                    shared_constructor_field4.clone();
                                                {
                                                    let source_ml_source2_nominal_field15835 =
                                                        shared_constructor_field5.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15837 =
                                                            shared_constructor_field6.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15839 =
                                                                shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15841 =
                                                                    shared_constructor_field8
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15843 =
                                                                        shared_constructor_field9
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15845 = shared_constructor_field10;
                                                                        {
                                                                            let source_ml_source2_nominal_field15847 = shared_constructor_field11;
                                                                            {
                                                                                let source_ml_source2_nominal_field15849 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field15851 = shared_constructor_field13;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15853 = shared_constructor_field14;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15855 = shared_constructor_field15.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15857 = shared_constructor_field16.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15859 = shared_constructor_field17.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15861 = shared_constructor_field18.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15863 = shared_constructor_field19.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15865 = shared_constructor_field20.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15867 = shared_constructor_field21.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15869 = shared_constructor_field22.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15871 = shared_constructor_field23.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15873 = shared_constructor_field24.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15875 = shared_constructor_field25.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15877 = shared_constructor_field26.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15879 = shared_constructor_field27.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15881 = shared_constructor_field28.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15883 = shared_constructor_field29.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15885 = shared_constructor_field30.clone();
                                                                                                                                                        self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15827.clone(), Box::new(source_ml_source2_nominal_field15829.clone()), source_ml_source2_nominal_field15831.clone(), Box::new(source_ml_source2_nominal_field15833.clone()), Box::new(source_ml_source2_nominal_field15835.clone()), Box::new(source_ml_source2_nominal_field15837.clone()), Box::new(source_ml_source2_nominal_field15839.clone()), Box::new(source_ml_source2_nominal_field15841.clone()), Box::new(source_ml_source2_nominal_field15843.clone()), source_ml_source2_nominal_field15845, source_ml_source2_nominal_field15847, phase.clone(), source_ml_source2_nominal_field15851, source_ml_source2_nominal_field15853, Box::new(source_ml_source2_nominal_field15855.clone()), Box::new(source_ml_source2_nominal_field15857.clone()), Box::new(source_ml_source2_nominal_field15859.clone()), Box::new(source_ml_source2_nominal_field15861.clone()), Box::new(source_ml_source2_nominal_field15863.clone()), source_ml_source2_nominal_field15865.clone(), source_ml_source2_nominal_field15867.clone(), source_ml_source2_nominal_field15869.clone(), source_ml_source2_nominal_field15871.clone(), source_ml_source2_nominal_field15873.clone(), source_ml_source2_nominal_field15875.clone(), source_ml_source2_nominal_field15877.clone(), source_ml_source2_nominal_field15879.clone(), source_ml_source2_nominal_field15881.clone(), source_ml_source2_nominal_field15883.clone(), source_ml_source2_nominal_field15885.clone())
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                                    let ml_reference_value856 = get.borrow().clone();
                                    ml_reference_value856
                                })
                                .0)
                                    .5,
                            )
                        } else {
                            if (((({
                                let ml_reference_value857 = get.borrow().clone();
                                ml_reference_value857
                            })
                            .0)
                                .0)
                                == (self::ml_string::Text::from_units(vec![99, 97, 108, 108])))
                            {
                                self::request_java_script_array_creation_call(
                                    heap.clone(),
                                    {
                                        let source_ml_source2_nominal_field15889 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field15891 =
                                                shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field15893 =
                                                    shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field15895 =
                                                        shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15897 =
                                                            shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15899 =
                                                                shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15901 =
                                                                    shared_constructor_field7
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15903 =
                                                                        shared_constructor_field8
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field15905 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field15907 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field15909 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field15911 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15913 = shared_constructor_field13;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15915 = shared_constructor_field14;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15917 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15919 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15921 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15923 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15925 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15927 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15929 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15931 = shared_constructor_field22.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15933 = shared_constructor_field23.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field15935 = shared_constructor_field24.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field15937 = shared_constructor_field25.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field15939 = shared_constructor_field26.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field15941 = shared_constructor_field27.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field15943 = shared_constructor_field28.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field15945 = shared_constructor_field29.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field15947 = shared_constructor_field30.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_override15949 = phase.concat(&self::ml_string::Text::from_units(vec![32, 103, 101, 116, 116, 101, 114]));
                                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15889.clone(), Box::new(source_ml_source2_nominal_field15891.clone()), source_ml_source2_nominal_field15893.clone(), Box::new(source_ml_source2_nominal_field15895.clone()), Box::new(source_ml_source2_nominal_field15897.clone()), Box::new(source_ml_source2_nominal_field15899.clone()), Box::new(source_ml_source2_nominal_field15901.clone()), Box::new(source_ml_source2_nominal_field15903.clone()), Box::new(source_ml_source2_nominal_field15905.clone()), source_ml_source2_nominal_field15907, source_ml_source2_nominal_field15909, source_ml_source2_nominal_override15949.clone(), source_ml_source2_nominal_field15913, source_ml_source2_nominal_field15915, Box::new(source_ml_source2_nominal_field15917.clone()), Box::new(source_ml_source2_nominal_field15919.clone()), Box::new(source_ml_source2_nominal_field15921.clone()), Box::new(source_ml_source2_nominal_field15923.clone()), Box::new(source_ml_source2_nominal_field15925.clone()), source_ml_source2_nominal_field15927.clone(), source_ml_source2_nominal_field15929.clone(), source_ml_source2_nominal_field15931.clone(), source_ml_source2_nominal_field15933.clone(), source_ml_source2_nominal_field15935.clone(), source_ml_source2_nominal_field15937.clone(), source_ml_source2_nominal_field15939.clone(), source_ml_source2_nominal_field15941.clone(), source_ml_source2_nominal_field15943.clone(), source_ml_source2_nominal_field15945.clone(), source_ml_source2_nominal_field15947.clone())
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
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
                                        let ml_reference_value858 = get.borrow().clone();
                                        ml_reference_value858
                                    })
                                    .0)
                                        .2,
                                    (({
                                        let ml_reference_value859 = get.borrow().clone();
                                        ml_reference_value859
                                    })
                                    .0)
                                        .4,
                                    (({
                                        let ml_reference_value860 = get.borrow().clone();
                                        ml_reference_value860
                                    })
                                    .0)
                                        .1,
                                )
                            } else {
                                self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111,
                                        110, 32, 111, 112, 97, 113, 117, 101, 32, 112, 114, 111,
                                        112, 101, 114, 116, 121, 32, 71, 101, 116, 32, 114, 101,
                                        113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97,
                                        99, 116, 117, 97, 108, 32, 104, 111, 115, 116, 32, 112,
                                        114, 111, 116, 111, 99, 111, 108,
                                    ]),
                                )
                            }
                        }
                    }
                }
                _ => self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 112,
                        114, 111, 112, 101, 114, 116, 121, 32, 71, 101, 116, 32, 114, 101, 116, 97,
                        105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108,
                        111, 111, 107, 117, 112, 32, 79, 98, 106, 101, 99, 116,
                    ]),
                ),
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_array_creation_wrapper(
    state: self::JavaScriptArrayCreationState,
    transition: self::JavaScriptPrimitiveWrapperTransition,
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
                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperAllocate(
                        heap,
                        state_2,
                        prototype,
                    ) => {
                        let heap = *heap;
                        let state_2 = *state_2;
                        let prototype = *prototype;
                        self::JavaScriptArrayCreationTransition::ArrayCreationAllocate(
                            Box::new(heap.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field15951 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field15953 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field15955 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field15957 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field15959 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field15961 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field15963 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field15965 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field15967 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field15969 =
                                                                        shared_constructor_field10;
                                                                    {
                                                                        let source_ml_source2_nominal_field15971 = shared_constructor_field11;
                                                                        {
                                                                            let source_ml_source2_nominal_field15973 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field15975 = shared_constructor_field13;
                                                                                {
                                                                                    let source_ml_source2_nominal_field15977 = shared_constructor_field14;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field15979 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field15981 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field15983 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field15985 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field15987 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field15989 = shared_constructor_field20.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field15991 = shared_constructor_field21.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field15993 = shared_constructor_field22.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field15995 = shared_constructor_field23.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field15997 = shared_constructor_field24.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field15999 = shared_constructor_field25.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16001 = shared_constructor_field26.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16003 = shared_constructor_field27.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16005 = shared_constructor_field28.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16007 = shared_constructor_field29.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16009 = shared_constructor_field30.clone();
                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field15951.clone(), Box::new(source_ml_source2_nominal_field15953.clone()), source_ml_source2_nominal_field15955.clone(), Box::new(source_ml_source2_nominal_field15957.clone()), Box::new(source_ml_source2_nominal_field15959.clone()), Box::new(source_ml_source2_nominal_field15961.clone()), Box::new(source_ml_source2_nominal_field15963.clone()), Box::new(source_ml_source2_nominal_field15965.clone()), Box::new(source_ml_source2_nominal_field15967.clone()), source_ml_source2_nominal_field15969, source_ml_source2_nominal_field15971, self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]), source_ml_source2_nominal_field15975, source_ml_source2_nominal_field15977, Box::new(source_ml_source2_nominal_field15979.clone()), Box::new(source_ml_source2_nominal_field15981.clone()), Box::new(source_ml_source2_nominal_field15983.clone()), Box::new(source_ml_source2_nominal_field15985.clone()), Box::new(source_ml_source2_nominal_field15987.clone()), source_ml_source2_nominal_field15989.clone(), source_ml_source2_nominal_field15991.clone(), source_ml_source2_nominal_field15993.clone(), source_ml_source2_nominal_field15995.clone(), source_ml_source2_nominal_field15997.clone(), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field16001.clone(), source_ml_source2_nominal_field16003.clone(), source_ml_source2_nominal_field16005.clone(), source_ml_source2_nominal_field16007.clone(), source_ml_source2_nominal_field16009.clone())
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                            Box::new(prototype.clone()),
                        )
                    }
                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperCompleted(
                        heap_2,
                        value,
                    ) => {
                        let heap_2 = *heap_2;
                        let value = *value;
                        self::JavaScriptArrayCreationTransition::ArrayCreationContinue(
                            Box::new(heap_2.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field16014 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field16016 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field16018 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field16020 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field16022 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field16024 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16026 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16028 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16030 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16032 =
                                                                        shared_constructor_field10;
                                                                    {
                                                                        let source_ml_source2_nominal_field16034 = shared_constructor_field11;
                                                                        {
                                                                            let source_ml_source2_nominal_field16036 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field16038 = shared_constructor_field13;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16040 = shared_constructor_field14;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16042 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16044 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16046 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16048 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16050 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16052 = shared_constructor_field20.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16054 = shared_constructor_field21.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16056 = shared_constructor_field22.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16058 = shared_constructor_field23.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16060 = shared_constructor_field24.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16062 = shared_constructor_field25.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16064 = shared_constructor_field26.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16066 = shared_constructor_field27.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16068 = shared_constructor_field28.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16070 = shared_constructor_field29.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16072 = shared_constructor_field30.clone();
                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16014.clone(), Box::new(source_ml_source2_nominal_field16016.clone()), source_ml_source2_nominal_field16018.clone(), Box::new(source_ml_source2_nominal_field16020.clone()), Box::new(source_ml_source2_nominal_field16022.clone()), Box::new(source_ml_source2_nominal_field16024.clone()), Box::new(source_ml_source2_nominal_field16026.clone()), Box::new(source_ml_source2_nominal_field16028.clone()), Box::new(source_ml_source2_nominal_field16030.clone()), source_ml_source2_nominal_field16032, source_ml_source2_nominal_field16034, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), source_ml_source2_nominal_field16038, source_ml_source2_nominal_field16040, Box::new(source_ml_source2_nominal_field16042.clone()), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field16046.clone()), Box::new(source_ml_source2_nominal_field16048.clone()), Box::new(source_ml_source2_nominal_field16050.clone()), source_ml_source2_nominal_field16052.clone(), source_ml_source2_nominal_field16054.clone(), source_ml_source2_nominal_field16056.clone(), source_ml_source2_nominal_field16058.clone(), source_ml_source2_nominal_field16060.clone(), source_ml_source2_nominal_field16062.clone(), source_ml_source2_nominal_field16064.clone(), source_ml_source2_nominal_field16066.clone(), source_ml_source2_nominal_field16068.clone(), source_ml_source2_nominal_field16070.clone(), source_ml_source2_nominal_field16072.clone())
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperInvalid(
                        heap_3,
                        reason,
                    ) => {
                        let heap_3 = *heap_3;
                        self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                            Box::new(heap_3.clone()),
                            reason.clone(),
                        )
                    }
                    self::JavaScriptPrimitiveWrapperTransition::PrimitiveWrapperResourceLimit(
                        heap_4,
                        state_3,
                        reason_2,
                    ) => {
                        let heap_4 = *heap_4;
                        let state_3 = *state_3;
                        self::JavaScriptArrayCreationTransition::ArrayCreationResourceLimit(
                            Box::new(heap_4.clone()),
                            Box::new({
                                let source_ml_source2_nominal_field16077 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_nominal_field16079 =
                                        shared_constructor_field2.clone();
                                    {
                                        let source_ml_source2_nominal_field16081 =
                                            shared_constructor_field3.clone();
                                        {
                                            let source_ml_source2_nominal_field16083 =
                                                shared_constructor_field4.clone();
                                            {
                                                let source_ml_source2_nominal_field16085 =
                                                    shared_constructor_field5.clone();
                                                {
                                                    let source_ml_source2_nominal_field16087 =
                                                        shared_constructor_field6.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16089 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16091 =
                                                                shared_constructor_field8.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16093 =
                                                                    shared_constructor_field9
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16095 =
                                                                        shared_constructor_field10;
                                                                    {
                                                                        let source_ml_source2_nominal_field16097 = shared_constructor_field11;
                                                                        {
                                                                            let source_ml_source2_nominal_field16099 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field16101 = shared_constructor_field13;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16103 = shared_constructor_field14;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16105 = shared_constructor_field15.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16107 = shared_constructor_field16.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16109 = shared_constructor_field17.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16111 = shared_constructor_field18.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16113 = shared_constructor_field19.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16115 = shared_constructor_field20.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16117 = shared_constructor_field21.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16119 = shared_constructor_field22.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16121 = shared_constructor_field23.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16123 = shared_constructor_field24.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16125 = shared_constructor_field25.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16127 = shared_constructor_field26.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16129 = shared_constructor_field27.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16131 = shared_constructor_field28.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16133 = shared_constructor_field29.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16135 = shared_constructor_field30.clone();
                                                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16077.clone(), Box::new(source_ml_source2_nominal_field16079.clone()), source_ml_source2_nominal_field16081.clone(), Box::new(source_ml_source2_nominal_field16083.clone()), Box::new(source_ml_source2_nominal_field16085.clone()), Box::new(source_ml_source2_nominal_field16087.clone()), Box::new(source_ml_source2_nominal_field16089.clone()), Box::new(source_ml_source2_nominal_field16091.clone()), Box::new(source_ml_source2_nominal_field16093.clone()), source_ml_source2_nominal_field16095, source_ml_source2_nominal_field16097, self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]), source_ml_source2_nominal_field16101, source_ml_source2_nominal_field16103, Box::new(source_ml_source2_nominal_field16105.clone()), Box::new(source_ml_source2_nominal_field16107.clone()), Box::new(source_ml_source2_nominal_field16109.clone()), Box::new(source_ml_source2_nominal_field16111.clone()), Box::new(source_ml_source2_nominal_field16113.clone()), source_ml_source2_nominal_field16115.clone(), source_ml_source2_nominal_field16117.clone(), source_ml_source2_nominal_field16119.clone(), source_ml_source2_nominal_field16121.clone(), source_ml_source2_nominal_field16123.clone(), Some((Some(state_3.clone()),)), source_ml_source2_nominal_field16127.clone(), source_ml_source2_nominal_field16129.clone(), source_ml_source2_nominal_field16131.clone(), source_ml_source2_nominal_field16133.clone(), source_ml_source2_nominal_field16135.clone())
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                            reason_2.clone(),
                        )
                    }
                }
            }
        }
    }
}
