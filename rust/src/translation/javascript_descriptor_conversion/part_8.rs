// Generated from complete js/src/translation/javascript-descriptor-conversion.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_empty_descriptor;
use super::ml_array;
use super::ml_math;
use super::read_java_script_descriptor_conversion_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::source_ml_temporary_step_java_script_descriptor_conversion_loop30;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_descriptor_field(
    state: self::JavaScriptDescriptorConversionState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptDescriptorConversionTransition {
    if (((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(
            _,
            source_ml_source1_data_property_value845,
            _,
            _,
        ) => source_ml_source1_data_property_value845,
    }) == 4f64)
        || ((match state.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                _,
                source_ml_source1_data_property_value847,
                _,
                _,
            ) => source_ml_source1_data_property_value847,
        }) == 5f64))
        && !(self::test_java_script_semantic_accessor(value.clone()))
    {
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
            String::from("TypeError"),
            String::from("a descriptor accessor must be callable or undefined"),
        )
    } else {
        {
            let before = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(
                    _,
                    _,
                    source_ml_source1_data_property_value848,
                    _,
                ) => source_ml_source1_data_property_value848.clone(),
            };
            {
                let descriptor = if (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value850,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value850,
                }) == 0f64
                {
                    {
                        let source_ml_source1_spread851 = before.clone();
                        {
                            let source_ml_source1_spread_field852 =
                                (source_ml_source1_spread851.clone()).0;
                            {
                                let source_ml_source1_spread_field853 =
                                    (source_ml_source1_spread851.clone()).1;
                                {
                                    let source_ml_source1_spread_field854 =
                                        (source_ml_source1_spread851.clone()).2;
                                    {
                                        let source_ml_source1_spread_field855 =
                                            (source_ml_source1_spread851.clone()).3;
                                        {
                                            let source_ml_source1_spread_field856 =
                                                (source_ml_source1_spread851.clone()).4;
                                            {
                                                let source_ml_source1_spread_field857 =
                                                    (source_ml_source1_spread851.clone()).5;
                                                {
                                                    let source_ml_source1_spread_field858 =
                                                        (source_ml_source1_spread851.clone()).6;
                                                    {
                                                        let source_ml_source1_spread_field859 =
                                                            (source_ml_source1_spread851.clone()).7;
                                                        {
                                                            let source_ml_source1_spread_field860 =
                                                                (source_ml_source1_spread851
                                                                    .clone())
                                                                .8;
                                                            {
                                                                let source_ml_source1_spread_field861 =
                                                                    (source_ml_source1_spread851
                                                                        .clone())
                                                                    .9;
                                                                {
                                                                    let source_ml_source1_spread_field862 = (source_ml_source1_spread851.clone()).10;
                                                                    {
                                                                        let source_ml_source1_spread_field863 = (source_ml_source1_spread851.clone()).11;
                                                                        {
                                                                            let source_ml_source1_field864 =
                                                                                true;
                                                                            {
                                                                                let source_ml_source1_field865 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                (source_ml_source1_spread_field852, source_ml_source1_spread_field853, source_ml_source1_field865, source_ml_source1_field864, source_ml_source1_spread_field856, source_ml_source1_spread_field857, source_ml_source1_spread_field858, source_ml_source1_spread_field859, source_ml_source1_spread_field860, source_ml_source1_spread_field861, source_ml_source1_spread_field862, source_ml_source1_spread_field863)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value867,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value867,
                }) == 1f64
                {
                    {
                        let source_ml_source1_spread868 = before.clone();
                        {
                            let source_ml_source1_spread_field869 =
                                (source_ml_source1_spread868.clone()).0;
                            {
                                let source_ml_source1_spread_field870 =
                                    (source_ml_source1_spread868.clone()).1;
                                {
                                    let source_ml_source1_spread_field871 =
                                        (source_ml_source1_spread868.clone()).2;
                                    {
                                        let source_ml_source1_spread_field872 =
                                            (source_ml_source1_spread868.clone()).3;
                                        {
                                            let source_ml_source1_spread_field873 =
                                                (source_ml_source1_spread868.clone()).4;
                                            {
                                                let source_ml_source1_spread_field874 =
                                                    (source_ml_source1_spread868.clone()).5;
                                                {
                                                    let source_ml_source1_spread_field875 =
                                                        (source_ml_source1_spread868.clone()).6;
                                                    {
                                                        let source_ml_source1_spread_field876 =
                                                            (source_ml_source1_spread868.clone()).7;
                                                        {
                                                            let source_ml_source1_spread_field877 =
                                                                (source_ml_source1_spread868
                                                                    .clone())
                                                                .8;
                                                            {
                                                                let source_ml_source1_spread_field878 =
                                                                    (source_ml_source1_spread868
                                                                        .clone())
                                                                    .9;
                                                                {
                                                                    let source_ml_source1_spread_field879 = (source_ml_source1_spread868.clone()).10;
                                                                    {
                                                                        let source_ml_source1_spread_field880 = (source_ml_source1_spread868.clone()).11;
                                                                        {
                                                                            let source_ml_source1_field881 =
                                                                                true;
                                                                            {
                                                                                let source_ml_source1_field882 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                (source_ml_source1_field882, source_ml_source1_field881, source_ml_source1_spread_field871, source_ml_source1_spread_field872, source_ml_source1_spread_field873, source_ml_source1_spread_field874, source_ml_source1_spread_field875, source_ml_source1_spread_field876, source_ml_source1_spread_field877, source_ml_source1_spread_field878, source_ml_source1_spread_field879, source_ml_source1_spread_field880)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value884,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value884,
                }) == 2f64
                {
                    {
                        let source_ml_source1_spread885 = before.clone();
                        {
                            let source_ml_source1_spread_field886 =
                                (source_ml_source1_spread885.clone()).0;
                            {
                                let source_ml_source1_spread_field887 =
                                    (source_ml_source1_spread885.clone()).1;
                                {
                                    let source_ml_source1_spread_field888 =
                                        (source_ml_source1_spread885.clone()).2;
                                    {
                                        let source_ml_source1_spread_field889 =
                                            (source_ml_source1_spread885.clone()).3;
                                        {
                                            let source_ml_source1_spread_field890 =
                                                (source_ml_source1_spread885.clone()).4;
                                            {
                                                let source_ml_source1_spread_field891 =
                                                    (source_ml_source1_spread885.clone()).5;
                                                {
                                                    let source_ml_source1_spread_field892 =
                                                        (source_ml_source1_spread885.clone()).6;
                                                    {
                                                        let source_ml_source1_spread_field893 =
                                                            (source_ml_source1_spread885.clone()).7;
                                                        {
                                                            let source_ml_source1_spread_field894 =
                                                                (source_ml_source1_spread885
                                                                    .clone())
                                                                .8;
                                                            {
                                                                let source_ml_source1_spread_field895 =
                                                                    (source_ml_source1_spread885
                                                                        .clone())
                                                                    .9;
                                                                {
                                                                    let source_ml_source1_spread_field896 = (source_ml_source1_spread885.clone()).10;
                                                                    {
                                                                        let source_ml_source1_spread_field897 = (source_ml_source1_spread885.clone()).11;
                                                                        {
                                                                            let source_ml_source1_field898 =
                                                                                true;
                                                                            {
                                                                                let source_ml_source1_field899 =
                                                                                    value.clone();
                                                                                (source_ml_source1_spread_field886, source_ml_source1_spread_field887, source_ml_source1_spread_field888, source_ml_source1_spread_field889, source_ml_source1_spread_field890, source_ml_source1_spread_field891, source_ml_source1_spread_field892, source_ml_source1_spread_field893, source_ml_source1_field899, source_ml_source1_field898, source_ml_source1_spread_field896, source_ml_source1_spread_field897)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value901,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value901,
                }) == 3f64
                {
                    {
                        let source_ml_source1_spread902 = before.clone();
                        {
                            let source_ml_source1_spread_field903 =
                                (source_ml_source1_spread902.clone()).0;
                            {
                                let source_ml_source1_spread_field904 =
                                    (source_ml_source1_spread902.clone()).1;
                                {
                                    let source_ml_source1_spread_field905 =
                                        (source_ml_source1_spread902.clone()).2;
                                    {
                                        let source_ml_source1_spread_field906 =
                                            (source_ml_source1_spread902.clone()).3;
                                        {
                                            let source_ml_source1_spread_field907 =
                                                (source_ml_source1_spread902.clone()).4;
                                            {
                                                let source_ml_source1_spread_field908 =
                                                    (source_ml_source1_spread902.clone()).5;
                                                {
                                                    let source_ml_source1_spread_field909 =
                                                        (source_ml_source1_spread902.clone()).6;
                                                    {
                                                        let source_ml_source1_spread_field910 =
                                                            (source_ml_source1_spread902.clone()).7;
                                                        {
                                                            let source_ml_source1_spread_field911 =
                                                                (source_ml_source1_spread902
                                                                    .clone())
                                                                .8;
                                                            {
                                                                let source_ml_source1_spread_field912 =
                                                                    (source_ml_source1_spread902
                                                                        .clone())
                                                                    .9;
                                                                {
                                                                    let source_ml_source1_spread_field913 = (source_ml_source1_spread902.clone()).10;
                                                                    {
                                                                        let source_ml_source1_spread_field914 = (source_ml_source1_spread902.clone()).11;
                                                                        {
                                                                            let source_ml_source1_field915 =
                                                                                true;
                                                                            {
                                                                                let source_ml_source1_field916 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                (source_ml_source1_spread_field903, source_ml_source1_spread_field904, source_ml_source1_spread_field905, source_ml_source1_spread_field906, source_ml_source1_spread_field907, source_ml_source1_spread_field908, source_ml_source1_spread_field909, source_ml_source1_spread_field910, source_ml_source1_spread_field911, source_ml_source1_spread_field912, source_ml_source1_field916, source_ml_source1_field915)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else if (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        _,
                        source_ml_source1_data_property_value918,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value918,
                }) == 4f64
                {
                    {
                        let source_ml_source1_spread919 = before.clone();
                        {
                            let source_ml_source1_spread_field920 =
                                (source_ml_source1_spread919.clone()).0;
                            {
                                let source_ml_source1_spread_field921 =
                                    (source_ml_source1_spread919.clone()).1;
                                {
                                    let source_ml_source1_spread_field922 =
                                        (source_ml_source1_spread919.clone()).2;
                                    {
                                        let source_ml_source1_spread_field923 =
                                            (source_ml_source1_spread919.clone()).3;
                                        {
                                            let source_ml_source1_spread_field924 =
                                                (source_ml_source1_spread919.clone()).4;
                                            {
                                                let source_ml_source1_spread_field925 =
                                                    (source_ml_source1_spread919.clone()).5;
                                                {
                                                    let source_ml_source1_spread_field926 =
                                                        (source_ml_source1_spread919.clone()).6;
                                                    {
                                                        let source_ml_source1_spread_field927 =
                                                            (source_ml_source1_spread919.clone()).7;
                                                        {
                                                            let source_ml_source1_spread_field928 =
                                                                (source_ml_source1_spread919
                                                                    .clone())
                                                                .8;
                                                            {
                                                                let source_ml_source1_spread_field929 =
                                                                    (source_ml_source1_spread919
                                                                        .clone())
                                                                    .9;
                                                                {
                                                                    let source_ml_source1_spread_field930 = (source_ml_source1_spread919.clone()).10;
                                                                    {
                                                                        let source_ml_source1_spread_field931 = (source_ml_source1_spread919.clone()).11;
                                                                        {
                                                                            let source_ml_source1_field932 =
                                                                                true;
                                                                            {
                                                                                let source_ml_source1_field933 =
                                                                                    value.clone();
                                                                                (source_ml_source1_spread_field920, source_ml_source1_spread_field921, source_ml_source1_spread_field922, source_ml_source1_spread_field923, source_ml_source1_field933, source_ml_source1_field932, source_ml_source1_spread_field926, source_ml_source1_spread_field927, source_ml_source1_spread_field928, source_ml_source1_spread_field929, source_ml_source1_spread_field930, source_ml_source1_spread_field931)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                    {
                        let source_ml_source1_spread934 = before.clone();
                        {
                            let source_ml_source1_spread_field935 =
                                (source_ml_source1_spread934.clone()).0;
                            {
                                let source_ml_source1_spread_field936 =
                                    (source_ml_source1_spread934.clone()).1;
                                {
                                    let source_ml_source1_spread_field937 =
                                        (source_ml_source1_spread934.clone()).2;
                                    {
                                        let source_ml_source1_spread_field938 =
                                            (source_ml_source1_spread934.clone()).3;
                                        {
                                            let source_ml_source1_spread_field939 =
                                                (source_ml_source1_spread934.clone()).4;
                                            {
                                                let source_ml_source1_spread_field940 =
                                                    (source_ml_source1_spread934.clone()).5;
                                                {
                                                    let source_ml_source1_spread_field941 =
                                                        (source_ml_source1_spread934.clone()).6;
                                                    {
                                                        let source_ml_source1_spread_field942 =
                                                            (source_ml_source1_spread934.clone()).7;
                                                        {
                                                            let source_ml_source1_spread_field943 =
                                                                (source_ml_source1_spread934
                                                                    .clone())
                                                                .8;
                                                            {
                                                                let source_ml_source1_spread_field944 =
                                                                    (source_ml_source1_spread934
                                                                        .clone())
                                                                    .9;
                                                                {
                                                                    let source_ml_source1_spread_field945 = (source_ml_source1_spread934.clone()).10;
                                                                    {
                                                                        let source_ml_source1_spread_field946 = (source_ml_source1_spread934.clone()).11;
                                                                        {
                                                                            let source_ml_source1_field947 =
                                                                                true;
                                                                            {
                                                                                let source_ml_source1_field948 =
                                                                                    value.clone();
                                                                                (source_ml_source1_spread_field935, source_ml_source1_spread_field936, source_ml_source1_spread_field937, source_ml_source1_spread_field938, source_ml_source1_spread_field939, source_ml_source1_spread_field940, source_ml_source1_field948, source_ml_source1_field947, source_ml_source1_spread_field943, source_ml_source1_spread_field944, source_ml_source1_spread_field945, source_ml_source1_spread_field946)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(
                        Box::new(match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                source_ml_source1_data_property_value949,
                                _,
                                _,
                                _,
                            ) => {
                                let source_ml_source1_data_property_value949 =
                                    *source_ml_source1_data_property_value949;
                                source_ml_source1_data_property_value949.clone()
                            }
                        }),
                        (match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                _,
                                source_ml_source1_data_property_value950,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value950,
                        } + 1f64),
                        descriptor,
                        false,
                    ),
                ))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_descriptor_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptDescriptorConversionState,
) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error =
            self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if !(error).is_empty() {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else if match state.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                _,
                _,
                _,
                source_ml_source1_data_property_value951,
            ) => source_ml_source1_data_property_value951,
        } {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from(
                "a pending descriptor Get must be completed before its next step",
            ))
        } else {
            {
                let current = state.clone();
                {
                    let source_ml_source1_s952 =
                        self::source_ml_temporary_step_java_script_descriptor_conversion_loop30(
                            heap.clone(),
                            current,
                        );
                    match source_ml_source1_s952 {
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Done(current_2) => {
                                let current_2 = *current_2;
                                if (self::read_java_script_property_descriptor_error(match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value954, _) => {
                                        source_ml_source1_data_property_value954.clone()
                                    }
                                })) != "" {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(String::from("TypeError"), String::from("a descriptor cannot contain both data and accessor attributes"))
                                } else {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(match current_2.clone() {
                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value955, _) => {
                                            source_ml_source1_data_property_value955.clone()
                                        }
                                    })
                                }
                            }
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop30Result::MlTemporaryStepJavaScriptDescriptorConversionLoop30Return(source_ml_temporary_result30) => {
                                let source_ml_temporary_result30 = *source_ml_temporary_result30;
                                source_ml_temporary_result30.clone()
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_descriptor_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
) -> self::JavaScriptDescriptorConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(heap_error).is_empty() {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(heap_error.clone())
        } else {
            {
                let error =
                    self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                if !(error).is_empty() {
                    self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
                } else {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f956) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f957) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f958) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f959) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f960) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(
                                String::from("TypeError"),
                                String::from("a property descriptor source must be an Object"),
                            )
                        }
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f961,
                            source_ml_source1_f962,
                        ) => self::step_java_script_descriptor_conversion(
                            heap.clone(),
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                                Box::new(receiver.clone()),
                                0f64,
                                self::create_java_script_empty_descriptor(),
                                false,
                            ),
                        ),
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_descriptor_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptDescriptorConversionState,
    completion: self::JavaScriptSemanticCompletion,
) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error =
            self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if !(error).is_empty() {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else if (match state.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(
                _,
                _,
                _,
                source_ml_source1_data_property_value963,
            ) => source_ml_source1_data_property_value963,
        }) {
            {
                let value_error = self::read_java_script_ordinary_value_error(
                    heap.clone(),
                    match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(
                            source_ml_source1_data_property_value964,
                        ) => {
                            let source_ml_source1_data_property_value964 =
                                *source_ml_source1_data_property_value964;
                            source_ml_source1_data_property_value964.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(
                            source_ml_source1_data_property_value965,
                        ) => {
                            let source_ml_source1_data_property_value965 =
                                *source_ml_source1_data_property_value965;
                            source_ml_source1_data_property_value965.clone()
                        }
                    },
                );
                if !(value_error).is_empty() {
                    self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(
                        value_error.clone(),
                    )
                } else {
                    match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
                            let value = *value;
                            {
                                let applied = self::apply_java_script_descriptor_field(
                                    state.clone(),
                                    value.clone(),
                                );
                                match applied.clone() {
                                        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(state_2) => {
                                            let state_2 = *state_2;
                                            self::step_java_script_descriptor_conversion(heap.clone(), state_2.clone())
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source1_f966, source_ml_source1_f967, source_ml_source1_f968, source_ml_source1_f969) => {
                                            let source_ml_source1_f966 = *source_ml_source1_f966;
                                            let source_ml_source1_f967 = *source_ml_source1_f967;
                                            let source_ml_source1_f968 = *source_ml_source1_f968;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source1_f970) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source1_f971) => {
                                            let source_ml_source1_f971 = *source_ml_source1_f971;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source1_f972, source_ml_source1_f973) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source1_f974) => {
                                            applied.clone()
                                        }
                                    }
                            }
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
                            let value_2 = *value_2;
                            self::JavaScriptDescriptorConversionTransition::DescriptorThrow(
                                Box::new(value_2.clone()),
                            )
                        }
                    }
                }
            }
        } else {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(String::from(
                "descriptor resumption needs its actual pending Get",
            ))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(
    units: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String::from("semantic text requires complete ordered UTF16 code units"))
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_same_value_loop2(
    left_units: &[f64],
    right_units: &[f64],
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if at < (left_units.len() as f64) {
            {
                let ml_library_inequality_1 =
                    ((self::ml_array::optional_at(left_units, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| -1f64))
                        != ((self::ml_array::optional_at(
                            right_units,
                            self::ml_array::number_index(at),
                        ))
                        .unwrap_or_else(|| -1f64));
                if ml_library_inequality_1 {
                    self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin3 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(
        (
            (
                bool,
                bool,
                bool,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                bool,
                bool,
            ),
            String,
            String,
            Vec<String>,
        ),
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop4(
    key: &[f64],
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result {
    loop {
        return if at < (key.len() as f64) {
            {
                let digit = (self::ml_array::optional_at(key, self::ml_array::number_index(at)))
                    .unwrap_or(f64::NAN)
                    - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(-1f64)
                } else {
                    {
                        let value_2 = (value * 10f64) + digit;
                        {
                            let at_2 = at + 1f64;
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done,
    MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(f64),
}
