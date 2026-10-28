// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_empty_descriptor;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_descriptor_conversion_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop121;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_descriptor_field(state: self::JavaScriptDescriptorConversionState, value: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    if ((((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value934, _, _) => {
            source_ml_source2_data_property_value934
        }
    }) == (4f64)) || ((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value936, _, _) => {
            source_ml_source2_data_property_value936
        }
    }) == (5f64))) && !(self::test_java_script_semantic_accessor(value.clone()))) {
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
    } else {
        {
            let before = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value937, _) => {
                    source_ml_source2_data_property_value937.clone()
                }
            };
            {
                let descriptor = if ((match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value939, _, _) => {
                        source_ml_source2_data_property_value939
                    }
                }) == (0f64)) {
                    {
                        let source_ml_source2_spread940 = before.clone();
                        {
                            let source_ml_source2_spread_field941 = (source_ml_source2_spread940.clone()).0;
                            {
                                let source_ml_source2_spread_field942 = (source_ml_source2_spread940.clone()).1;
                                {
                                    let source_ml_source2_spread_field943 = (source_ml_source2_spread940.clone()).2;
                                    {
                                        let source_ml_source2_spread_field944 = (source_ml_source2_spread940.clone()).3;
                                        {
                                            let source_ml_source2_spread_field945 = (source_ml_source2_spread940.clone()).4;
                                            {
                                                let source_ml_source2_spread_field946 = (source_ml_source2_spread940.clone()).5;
                                                {
                                                    let source_ml_source2_spread_field947 = (source_ml_source2_spread940.clone()).6;
                                                    {
                                                        let source_ml_source2_spread_field948 = (source_ml_source2_spread940.clone()).7;
                                                        {
                                                            let source_ml_source2_spread_field949 = (source_ml_source2_spread940.clone()).8;
                                                            {
                                                                let source_ml_source2_spread_field950 = (source_ml_source2_spread940.clone()).9;
                                                                {
                                                                    let source_ml_source2_spread_field951 = (source_ml_source2_spread940.clone()).10;
                                                                    {
                                                                        let source_ml_source2_spread_field952 = (source_ml_source2_spread940.clone()).11;
                                                                        {
                                                                            let source_ml_source2_field953 = true;
                                                                            {
                                                                                let source_ml_source2_field954 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                (source_ml_source2_spread_field941, source_ml_source2_spread_field942, source_ml_source2_field954, source_ml_source2_field953, source_ml_source2_spread_field945.clone(), source_ml_source2_spread_field946, source_ml_source2_spread_field947.clone(), source_ml_source2_spread_field948, source_ml_source2_spread_field949.clone(), source_ml_source2_spread_field950, source_ml_source2_spread_field951, source_ml_source2_spread_field952)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                    if ((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value956, _, _) => {
                            source_ml_source2_data_property_value956
                        }
                    }) == (1f64)) {
                        {
                            let source_ml_source2_spread957 = before.clone();
                            {
                                let source_ml_source2_spread_field958 = (source_ml_source2_spread957.clone()).0;
                                {
                                    let source_ml_source2_spread_field959 = (source_ml_source2_spread957.clone()).1;
                                    {
                                        let source_ml_source2_spread_field960 = (source_ml_source2_spread957.clone()).2;
                                        {
                                            let source_ml_source2_spread_field961 = (source_ml_source2_spread957.clone()).3;
                                            {
                                                let source_ml_source2_spread_field962 = (source_ml_source2_spread957.clone()).4;
                                                {
                                                    let source_ml_source2_spread_field963 = (source_ml_source2_spread957.clone()).5;
                                                    {
                                                        let source_ml_source2_spread_field964 = (source_ml_source2_spread957.clone()).6;
                                                        {
                                                            let source_ml_source2_spread_field965 = (source_ml_source2_spread957.clone()).7;
                                                            {
                                                                let source_ml_source2_spread_field966 = (source_ml_source2_spread957.clone()).8;
                                                                {
                                                                    let source_ml_source2_spread_field967 = (source_ml_source2_spread957.clone()).9;
                                                                    {
                                                                        let source_ml_source2_spread_field968 = (source_ml_source2_spread957.clone()).10;
                                                                        {
                                                                            let source_ml_source2_spread_field969 = (source_ml_source2_spread957.clone()).11;
                                                                            {
                                                                                let source_ml_source2_field970 = true;
                                                                                {
                                                                                    let source_ml_source2_field971 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                    (source_ml_source2_field971, source_ml_source2_field970, source_ml_source2_spread_field960, source_ml_source2_spread_field961, source_ml_source2_spread_field962.clone(), source_ml_source2_spread_field963, source_ml_source2_spread_field964.clone(), source_ml_source2_spread_field965, source_ml_source2_spread_field966.clone(), source_ml_source2_spread_field967, source_ml_source2_spread_field968, source_ml_source2_spread_field969)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
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
                        if ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value973, _, _) => {
                                source_ml_source2_data_property_value973
                            }
                        }) == (2f64)) {
                            {
                                let source_ml_source2_spread974 = before.clone();
                                {
                                    let source_ml_source2_spread_field975 = (source_ml_source2_spread974.clone()).0;
                                    {
                                        let source_ml_source2_spread_field976 = (source_ml_source2_spread974.clone()).1;
                                        {
                                            let source_ml_source2_spread_field977 = (source_ml_source2_spread974.clone()).2;
                                            {
                                                let source_ml_source2_spread_field978 = (source_ml_source2_spread974.clone()).3;
                                                {
                                                    let source_ml_source2_spread_field979 = (source_ml_source2_spread974.clone()).4;
                                                    {
                                                        let source_ml_source2_spread_field980 = (source_ml_source2_spread974.clone()).5;
                                                        {
                                                            let source_ml_source2_spread_field981 = (source_ml_source2_spread974.clone()).6;
                                                            {
                                                                let source_ml_source2_spread_field982 = (source_ml_source2_spread974.clone()).7;
                                                                {
                                                                    let source_ml_source2_spread_field983 = (source_ml_source2_spread974.clone()).8;
                                                                    {
                                                                        let source_ml_source2_spread_field984 = (source_ml_source2_spread974.clone()).9;
                                                                        {
                                                                            let source_ml_source2_spread_field985 = (source_ml_source2_spread974.clone()).10;
                                                                            {
                                                                                let source_ml_source2_spread_field986 = (source_ml_source2_spread974.clone()).11;
                                                                                {
                                                                                    let source_ml_source2_field987 = true;
                                                                                    {
                                                                                        let source_ml_source2_field988 = value.clone();
                                                                                        (source_ml_source2_spread_field975, source_ml_source2_spread_field976, source_ml_source2_spread_field977, source_ml_source2_spread_field978, source_ml_source2_spread_field979.clone(), source_ml_source2_spread_field980, source_ml_source2_spread_field981.clone(), source_ml_source2_spread_field982, source_ml_source2_field988.clone(), source_ml_source2_field987, source_ml_source2_spread_field985, source_ml_source2_spread_field986)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                            if ((match state.clone() {
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value990, _, _) => {
                                    source_ml_source2_data_property_value990
                                }
                            }) == (3f64)) {
                                {
                                    let source_ml_source2_spread991 = before.clone();
                                    {
                                        let source_ml_source2_spread_field992 = (source_ml_source2_spread991.clone()).0;
                                        {
                                            let source_ml_source2_spread_field993 = (source_ml_source2_spread991.clone()).1;
                                            {
                                                let source_ml_source2_spread_field994 = (source_ml_source2_spread991.clone()).2;
                                                {
                                                    let source_ml_source2_spread_field995 = (source_ml_source2_spread991.clone()).3;
                                                    {
                                                        let source_ml_source2_spread_field996 = (source_ml_source2_spread991.clone()).4;
                                                        {
                                                            let source_ml_source2_spread_field997 = (source_ml_source2_spread991.clone()).5;
                                                            {
                                                                let source_ml_source2_spread_field998 = (source_ml_source2_spread991.clone()).6;
                                                                {
                                                                    let source_ml_source2_spread_field999 = (source_ml_source2_spread991.clone()).7;
                                                                    {
                                                                        let source_ml_source2_spread_field1000 = (source_ml_source2_spread991.clone()).8;
                                                                        {
                                                                            let source_ml_source2_spread_field1001 = (source_ml_source2_spread991.clone()).9;
                                                                            {
                                                                                let source_ml_source2_spread_field1002 = (source_ml_source2_spread991.clone()).10;
                                                                                {
                                                                                    let source_ml_source2_spread_field1003 = (source_ml_source2_spread991.clone()).11;
                                                                                    {
                                                                                        let source_ml_source2_field1004 = true;
                                                                                        {
                                                                                            let source_ml_source2_field1005 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                            (source_ml_source2_spread_field992, source_ml_source2_spread_field993, source_ml_source2_spread_field994, source_ml_source2_spread_field995, source_ml_source2_spread_field996.clone(), source_ml_source2_spread_field997, source_ml_source2_spread_field998.clone(), source_ml_source2_spread_field999, source_ml_source2_spread_field1000.clone(), source_ml_source2_spread_field1001, source_ml_source2_field1005, source_ml_source2_field1004)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                                if ((match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value1007, _, _) => {
                                        source_ml_source2_data_property_value1007
                                    }
                                }) == (4f64)) {
                                    {
                                        let source_ml_source2_spread1008 = before.clone();
                                        {
                                            let source_ml_source2_spread_field1009 = (source_ml_source2_spread1008.clone()).0;
                                            {
                                                let source_ml_source2_spread_field1010 = (source_ml_source2_spread1008.clone()).1;
                                                {
                                                    let source_ml_source2_spread_field1011 = (source_ml_source2_spread1008.clone()).2;
                                                    {
                                                        let source_ml_source2_spread_field1012 = (source_ml_source2_spread1008.clone()).3;
                                                        {
                                                            let source_ml_source2_spread_field1013 = (source_ml_source2_spread1008.clone()).4;
                                                            {
                                                                let source_ml_source2_spread_field1014 = (source_ml_source2_spread1008.clone()).5;
                                                                {
                                                                    let source_ml_source2_spread_field1015 = (source_ml_source2_spread1008.clone()).6;
                                                                    {
                                                                        let source_ml_source2_spread_field1016 = (source_ml_source2_spread1008.clone()).7;
                                                                        {
                                                                            let source_ml_source2_spread_field1017 = (source_ml_source2_spread1008.clone()).8;
                                                                            {
                                                                                let source_ml_source2_spread_field1018 = (source_ml_source2_spread1008.clone()).9;
                                                                                {
                                                                                    let source_ml_source2_spread_field1019 = (source_ml_source2_spread1008.clone()).10;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1020 = (source_ml_source2_spread1008.clone()).11;
                                                                                        {
                                                                                            let source_ml_source2_field1021 = true;
                                                                                            {
                                                                                                let source_ml_source2_field1022 = value.clone();
                                                                                                (source_ml_source2_spread_field1009, source_ml_source2_spread_field1010, source_ml_source2_spread_field1011, source_ml_source2_spread_field1012, source_ml_source2_field1022.clone(), source_ml_source2_field1021, source_ml_source2_spread_field1015.clone(), source_ml_source2_spread_field1016, source_ml_source2_spread_field1017.clone(), source_ml_source2_spread_field1018, source_ml_source2_spread_field1019, source_ml_source2_spread_field1020)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                                        let source_ml_source2_spread1023 = before.clone();
                                        {
                                            let source_ml_source2_spread_field1024 = (source_ml_source2_spread1023.clone()).0;
                                            {
                                                let source_ml_source2_spread_field1025 = (source_ml_source2_spread1023.clone()).1;
                                                {
                                                    let source_ml_source2_spread_field1026 = (source_ml_source2_spread1023.clone()).2;
                                                    {
                                                        let source_ml_source2_spread_field1027 = (source_ml_source2_spread1023.clone()).3;
                                                        {
                                                            let source_ml_source2_spread_field1028 = (source_ml_source2_spread1023.clone()).4;
                                                            {
                                                                let source_ml_source2_spread_field1029 = (source_ml_source2_spread1023.clone()).5;
                                                                {
                                                                    let source_ml_source2_spread_field1030 = (source_ml_source2_spread1023.clone()).6;
                                                                    {
                                                                        let source_ml_source2_spread_field1031 = (source_ml_source2_spread1023.clone()).7;
                                                                        {
                                                                            let source_ml_source2_spread_field1032 = (source_ml_source2_spread1023.clone()).8;
                                                                            {
                                                                                let source_ml_source2_spread_field1033 = (source_ml_source2_spread1023.clone()).9;
                                                                                {
                                                                                    let source_ml_source2_spread_field1034 = (source_ml_source2_spread1023.clone()).10;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1035 = (source_ml_source2_spread1023.clone()).11;
                                                                                        {
                                                                                            let source_ml_source2_field1036 = true;
                                                                                            {
                                                                                                let source_ml_source2_field1037 = value.clone();
                                                                                                (source_ml_source2_spread_field1024, source_ml_source2_spread_field1025, source_ml_source2_spread_field1026, source_ml_source2_spread_field1027, source_ml_source2_spread_field1028.clone(), source_ml_source2_spread_field1029, source_ml_source2_field1037.clone(), source_ml_source2_field1036, source_ml_source2_spread_field1032.clone(), source_ml_source2_spread_field1033, source_ml_source2_spread_field1034, source_ml_source2_spread_field1035)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value1038, _, _, _) => {
                        let source_ml_source2_data_property_value1038 = *source_ml_source2_data_property_value1038;
                        source_ml_source2_data_property_value1038.clone()
                    }
                }), (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value1039, _, _) => {
                        source_ml_source2_data_property_value1039
                    }
                } + 1f64), descriptor.clone(), false)))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value1040) => {
                    source_ml_source2_data_property_value1040
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source2_s1041 = self::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop121(heap.clone(), current.clone());
                        match source_ml_source2_s1041.clone() {
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Done(current_2) => {
                                let current_2 = *current_2;
                                if ((self::read_java_script_property_descriptor_error(match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value1043, _) => {
                                        source_ml_source2_data_property_value1043.clone()
                                    }
                                })) != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                } else {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(match current_2.clone() {
                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value1044, _) => {
                                            source_ml_source2_data_property_value1044.clone()
                                        }
                                    })
                                }
                            }
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(source_ml_source1_temporary_result121) => {
                                let source_ml_source1_temporary_result121 = *source_ml_source1_temporary_result121;
                                source_ml_source1_temporary_result121.clone()
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
pub fn begin_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, receiver: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(heap_error.clone())
        } else {
            {
                let error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
                } else {
                    match receiver.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1045) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f1046) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f1047) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f1048) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1049) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f1050, source_ml_source2_f1051) => {
                            self::step_java_script_descriptor_conversion(heap.clone(), self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), 0f64, self::create_java_script_empty_descriptor(), false))
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState, completion: self::JavaScriptSemanticCompletion) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if !(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value1052) => {
                    source_ml_source2_data_property_value1052
                }
            }) {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 71, 101, 116]))
            } else {
                {
                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(source_ml_source2_data_property_value1053) => {
                            let source_ml_source2_data_property_value1053 = *source_ml_source2_data_property_value1053;
                            source_ml_source2_data_property_value1053.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(source_ml_source2_data_property_value1054) => {
                            let source_ml_source2_data_property_value1054 = *source_ml_source2_data_property_value1054;
                            source_ml_source2_data_property_value1054.clone()
                        }
                    });
                    if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(value_error.clone())
                    } else {
                        match completion.clone() {
                            self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
                                let value = *value;
                                {
                                    let applied = self::apply_java_script_descriptor_field(state.clone(), value.clone());
                                    match applied.clone() {
                                        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(state_2) => {
                                            let state_2 = *state_2;
                                            self::step_java_script_descriptor_conversion(heap.clone(), state_2.clone())
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source2_f1055, source_ml_source2_f1056, source_ml_source2_f1057, source_ml_source2_f1058) => {
                                            let source_ml_source2_f1055 = *source_ml_source2_f1055;
                                            let source_ml_source2_f1056 = *source_ml_source2_f1056;
                                            let source_ml_source2_f1057 = *source_ml_source2_f1057;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source2_f1059) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source2_f1060) => {
                                            let source_ml_source2_f1060 = *source_ml_source2_f1060;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source2_f1061, source_ml_source2_f1062) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source2_f1063) => {
                                            applied.clone()
                                        }
                                    }
                                }
                            }
                            self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
                                let value_2 = *value_2;
                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(Box::new(value_2.clone()))
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
pub fn read_java_script_primitive_conversion_spelling(name: &self::ml_string::Text) -> self::JavaScriptSemanticValue {
    if ((*name) == (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) {
        self::JavaScriptSemanticValue::Text(vec![100f64, 101f64, 102f64, 97f64, 117f64, 108f64, 116f64])
    } else {
        if ((*name) == (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]))) {
            self::JavaScriptSemanticValue::Text(vec![110f64, 117f64, 109f64, 98f64, 101f64, 114f64])
        } else {
            if ((*name) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                self::JavaScriptSemanticValue::Text(vec![115f64, 116f64, 114f64, 105f64, 110f64, 103f64])
            } else {
                if ((*name) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102]))) {
                    self::JavaScriptSemanticValue::Text(vec![118f64, 97f64, 108f64, 117f64, 101f64, 79f64, 102f64])
                } else {
                    if ((*name) == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]))) {
                        self::JavaScriptSemanticValue::Text(vec![116f64, 111f64, 83f64, 116f64, 114f64, 105f64, 110f64, 103f64])
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 115, 112, 101, 108, 108, 105, 110, 103, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 110, 97, 109, 101])).0, message_present: true }).raise()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_primitive_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState) -> self::ml_string::Text {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            error.clone()
        } else {
            if {
                let source_ml_source2_s1065 = match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value1073, _, _, _) => {
                        let source_ml_source2_data_property_value1073 = *source_ml_source2_data_property_value1073;
                        source_ml_source2_data_property_value1073.clone()
                    }
                };
                match source_ml_source2_s1065.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1066) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f1067) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f1068) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f1069) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1070) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f1071, source_ml_source2_f1072) => {
                        false
                    }
                }
            } {
                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value1074, _, _, _) => {
                            let source_ml_source2_data_property_value1074 = *source_ml_source2_data_property_value1074;
                            source_ml_source2_data_property_value1074.clone()
                        }
                    });
                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                        receiver_error.clone()
                    } else {
                        if {
                            let source_ml_source2_s1076 = match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value1084, _, _) => {
                                    let source_ml_source2_data_property_value1084 = *source_ml_source2_data_property_value1084;
                                    source_ml_source2_data_property_value1084.clone()
                                }
                            };
                            match source_ml_source2_s1076.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1077) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source2_f1078) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source2_f1079) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source2_f1080) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1081) => {
                                    false
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source2_f1082, source_ml_source2_f1083) => {
                                    true
                                }
                            }
                        } {
                            self::ml_string::Text::from_units(vec![116, 104, 101, 32, 101, 120, 111, 116, 105, 99, 32, 109, 101, 116, 104, 111, 100, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108])
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value1085, _, _) => {
                                        let source_ml_source2_data_property_value1085 = *source_ml_source2_data_property_value1085;
                                        source_ml_source2_data_property_value1085.clone()
                                    }
                                });
                                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                    key_error.clone()
                                } else {
                                    if ((((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1087, _) => {
                                            source_ml_source2_data_property_value1087.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1089, _) => {
                                            source_ml_source2_data_property_value1089.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1091, _) => {
                                            source_ml_source2_data_property_value1091.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116])
                                    } else {
                                        if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])], &match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value1092) => {
                                                source_ml_source2_data_property_value1092.clone()
                                            }
                                        }, 0f64, false, |a, b| a == b)) {
                                            self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 109, 101, 116, 104, 111, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                    }
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
pub fn continue_java_script_primitive_conversion(state: self::JavaScriptPrimitiveConversionState, phase: &self::ml_string::Text) -> self::JavaScriptPrimitiveConversionTransition {
    self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value1093, _, _, _) => {
            let source_ml_source2_data_property_value1093 = *source_ml_source2_data_property_value1093;
            source_ml_source2_data_property_value1093.clone()
        }
    }), Box::new(match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value1094, _, _) => {
            let source_ml_source2_data_property_value1094 = *source_ml_source2_data_property_value1094;
            source_ml_source2_data_property_value1094.clone()
        }
    }), match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1095, _) => {
            source_ml_source2_data_property_value1095.clone()
        }
    }, phase.clone())))
}
