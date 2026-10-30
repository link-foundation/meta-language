// Generated from complete js/src/translation/javascript-property-definition.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result;
use super::begin_java_script_property_key_conversion;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_empty_descriptor;
use super::define_java_script_ordinary_property;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_descriptor_conversion_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::resume_java_script_property_key_conversion;
use super::source_ml_temporary_step_java_script_descriptor_conversion_loop31;
use super::step_java_script_property_key_conversion;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_descriptor_field(state: self::JavaScriptDescriptorConversionState, value: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    if ((((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value997, _, _) => {
            source_ml_source1_data_property_value997
        }
    }) == (4f64)) || ((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value999, _, _) => {
            source_ml_source1_data_property_value999
        }
    }) == (5f64))) && !(self::test_java_script_semantic_accessor(value.clone()))) {
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
    } else {
        {
            let before = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value1000, _) => {
                    source_ml_source1_data_property_value1000.clone()
                }
            };
            {
                let descriptor = if ((match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1002, _, _) => {
                        source_ml_source1_data_property_value1002
                    }
                }) == (0f64)) {
                    {
                        let source_ml_source1_spread1003 = before.clone();
                        {
                            let source_ml_source1_spread_field1004 = (source_ml_source1_spread1003.clone()).0;
                            {
                                let source_ml_source1_spread_field1005 = (source_ml_source1_spread1003.clone()).1;
                                {
                                    let source_ml_source1_spread_field1006 = (source_ml_source1_spread1003.clone()).2;
                                    {
                                        let source_ml_source1_spread_field1007 = (source_ml_source1_spread1003.clone()).3;
                                        {
                                            let source_ml_source1_spread_field1008 = (source_ml_source1_spread1003.clone()).4;
                                            {
                                                let source_ml_source1_spread_field1009 = (source_ml_source1_spread1003.clone()).5;
                                                {
                                                    let source_ml_source1_spread_field1010 = (source_ml_source1_spread1003.clone()).6;
                                                    {
                                                        let source_ml_source1_spread_field1011 = (source_ml_source1_spread1003.clone()).7;
                                                        {
                                                            let source_ml_source1_spread_field1012 = (source_ml_source1_spread1003.clone()).8;
                                                            {
                                                                let source_ml_source1_spread_field1013 = (source_ml_source1_spread1003.clone()).9;
                                                                {
                                                                    let source_ml_source1_spread_field1014 = (source_ml_source1_spread1003.clone()).10;
                                                                    {
                                                                        let source_ml_source1_spread_field1015 = (source_ml_source1_spread1003.clone()).11;
                                                                        {
                                                                            let source_ml_source1_field1016 = true;
                                                                            {
                                                                                let source_ml_source1_field1017 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                (source_ml_source1_spread_field1004, source_ml_source1_spread_field1005, source_ml_source1_field1017, source_ml_source1_field1016, source_ml_source1_spread_field1008.clone(), source_ml_source1_spread_field1009, source_ml_source1_spread_field1010.clone(), source_ml_source1_spread_field1011, source_ml_source1_spread_field1012.clone(), source_ml_source1_spread_field1013, source_ml_source1_spread_field1014, source_ml_source1_spread_field1015)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1019, _, _) => {
                            source_ml_source1_data_property_value1019
                        }
                    }) == (1f64)) {
                        {
                            let source_ml_source1_spread1020 = before.clone();
                            {
                                let source_ml_source1_spread_field1021 = (source_ml_source1_spread1020.clone()).0;
                                {
                                    let source_ml_source1_spread_field1022 = (source_ml_source1_spread1020.clone()).1;
                                    {
                                        let source_ml_source1_spread_field1023 = (source_ml_source1_spread1020.clone()).2;
                                        {
                                            let source_ml_source1_spread_field1024 = (source_ml_source1_spread1020.clone()).3;
                                            {
                                                let source_ml_source1_spread_field1025 = (source_ml_source1_spread1020.clone()).4;
                                                {
                                                    let source_ml_source1_spread_field1026 = (source_ml_source1_spread1020.clone()).5;
                                                    {
                                                        let source_ml_source1_spread_field1027 = (source_ml_source1_spread1020.clone()).6;
                                                        {
                                                            let source_ml_source1_spread_field1028 = (source_ml_source1_spread1020.clone()).7;
                                                            {
                                                                let source_ml_source1_spread_field1029 = (source_ml_source1_spread1020.clone()).8;
                                                                {
                                                                    let source_ml_source1_spread_field1030 = (source_ml_source1_spread1020.clone()).9;
                                                                    {
                                                                        let source_ml_source1_spread_field1031 = (source_ml_source1_spread1020.clone()).10;
                                                                        {
                                                                            let source_ml_source1_spread_field1032 = (source_ml_source1_spread1020.clone()).11;
                                                                            {
                                                                                let source_ml_source1_field1033 = true;
                                                                                {
                                                                                    let source_ml_source1_field1034 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                    (source_ml_source1_field1034, source_ml_source1_field1033, source_ml_source1_spread_field1023, source_ml_source1_spread_field1024, source_ml_source1_spread_field1025.clone(), source_ml_source1_spread_field1026, source_ml_source1_spread_field1027.clone(), source_ml_source1_spread_field1028, source_ml_source1_spread_field1029.clone(), source_ml_source1_spread_field1030, source_ml_source1_spread_field1031, source_ml_source1_spread_field1032)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
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
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1036, _, _) => {
                                source_ml_source1_data_property_value1036
                            }
                        }) == (2f64)) {
                            {
                                let source_ml_source1_spread1037 = before.clone();
                                {
                                    let source_ml_source1_spread_field1038 = (source_ml_source1_spread1037.clone()).0;
                                    {
                                        let source_ml_source1_spread_field1039 = (source_ml_source1_spread1037.clone()).1;
                                        {
                                            let source_ml_source1_spread_field1040 = (source_ml_source1_spread1037.clone()).2;
                                            {
                                                let source_ml_source1_spread_field1041 = (source_ml_source1_spread1037.clone()).3;
                                                {
                                                    let source_ml_source1_spread_field1042 = (source_ml_source1_spread1037.clone()).4;
                                                    {
                                                        let source_ml_source1_spread_field1043 = (source_ml_source1_spread1037.clone()).5;
                                                        {
                                                            let source_ml_source1_spread_field1044 = (source_ml_source1_spread1037.clone()).6;
                                                            {
                                                                let source_ml_source1_spread_field1045 = (source_ml_source1_spread1037.clone()).7;
                                                                {
                                                                    let source_ml_source1_spread_field1046 = (source_ml_source1_spread1037.clone()).8;
                                                                    {
                                                                        let source_ml_source1_spread_field1047 = (source_ml_source1_spread1037.clone()).9;
                                                                        {
                                                                            let source_ml_source1_spread_field1048 = (source_ml_source1_spread1037.clone()).10;
                                                                            {
                                                                                let source_ml_source1_spread_field1049 = (source_ml_source1_spread1037.clone()).11;
                                                                                {
                                                                                    let source_ml_source1_field1050 = true;
                                                                                    {
                                                                                        let source_ml_source1_field1051 = value.clone();
                                                                                        (source_ml_source1_spread_field1038, source_ml_source1_spread_field1039, source_ml_source1_spread_field1040, source_ml_source1_spread_field1041, source_ml_source1_spread_field1042.clone(), source_ml_source1_spread_field1043, source_ml_source1_spread_field1044.clone(), source_ml_source1_spread_field1045, source_ml_source1_field1051.clone(), source_ml_source1_field1050, source_ml_source1_spread_field1048, source_ml_source1_spread_field1049)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1053, _, _) => {
                                    source_ml_source1_data_property_value1053
                                }
                            }) == (3f64)) {
                                {
                                    let source_ml_source1_spread1054 = before.clone();
                                    {
                                        let source_ml_source1_spread_field1055 = (source_ml_source1_spread1054.clone()).0;
                                        {
                                            let source_ml_source1_spread_field1056 = (source_ml_source1_spread1054.clone()).1;
                                            {
                                                let source_ml_source1_spread_field1057 = (source_ml_source1_spread1054.clone()).2;
                                                {
                                                    let source_ml_source1_spread_field1058 = (source_ml_source1_spread1054.clone()).3;
                                                    {
                                                        let source_ml_source1_spread_field1059 = (source_ml_source1_spread1054.clone()).4;
                                                        {
                                                            let source_ml_source1_spread_field1060 = (source_ml_source1_spread1054.clone()).5;
                                                            {
                                                                let source_ml_source1_spread_field1061 = (source_ml_source1_spread1054.clone()).6;
                                                                {
                                                                    let source_ml_source1_spread_field1062 = (source_ml_source1_spread1054.clone()).7;
                                                                    {
                                                                        let source_ml_source1_spread_field1063 = (source_ml_source1_spread1054.clone()).8;
                                                                        {
                                                                            let source_ml_source1_spread_field1064 = (source_ml_source1_spread1054.clone()).9;
                                                                            {
                                                                                let source_ml_source1_spread_field1065 = (source_ml_source1_spread1054.clone()).10;
                                                                                {
                                                                                    let source_ml_source1_spread_field1066 = (source_ml_source1_spread1054.clone()).11;
                                                                                    {
                                                                                        let source_ml_source1_field1067 = true;
                                                                                        {
                                                                                            let source_ml_source1_field1068 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                            (source_ml_source1_spread_field1055, source_ml_source1_spread_field1056, source_ml_source1_spread_field1057, source_ml_source1_spread_field1058, source_ml_source1_spread_field1059.clone(), source_ml_source1_spread_field1060, source_ml_source1_spread_field1061.clone(), source_ml_source1_spread_field1062, source_ml_source1_spread_field1063.clone(), source_ml_source1_spread_field1064, source_ml_source1_field1068, source_ml_source1_field1067)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1070, _, _) => {
                                        source_ml_source1_data_property_value1070
                                    }
                                }) == (4f64)) {
                                    {
                                        let source_ml_source1_spread1071 = before.clone();
                                        {
                                            let source_ml_source1_spread_field1072 = (source_ml_source1_spread1071.clone()).0;
                                            {
                                                let source_ml_source1_spread_field1073 = (source_ml_source1_spread1071.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field1074 = (source_ml_source1_spread1071.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field1075 = (source_ml_source1_spread1071.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field1076 = (source_ml_source1_spread1071.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field1077 = (source_ml_source1_spread1071.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field1078 = (source_ml_source1_spread1071.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field1079 = (source_ml_source1_spread1071.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field1080 = (source_ml_source1_spread1071.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field1081 = (source_ml_source1_spread1071.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field1082 = (source_ml_source1_spread1071.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field1083 = (source_ml_source1_spread1071.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field1084 = true;
                                                                                            {
                                                                                                let source_ml_source1_field1085 = value.clone();
                                                                                                (source_ml_source1_spread_field1072, source_ml_source1_spread_field1073, source_ml_source1_spread_field1074, source_ml_source1_spread_field1075, source_ml_source1_field1085.clone(), source_ml_source1_field1084, source_ml_source1_spread_field1078.clone(), source_ml_source1_spread_field1079, source_ml_source1_spread_field1080.clone(), source_ml_source1_spread_field1081, source_ml_source1_spread_field1082, source_ml_source1_spread_field1083)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                                        let source_ml_source1_spread1086 = before.clone();
                                        {
                                            let source_ml_source1_spread_field1087 = (source_ml_source1_spread1086.clone()).0;
                                            {
                                                let source_ml_source1_spread_field1088 = (source_ml_source1_spread1086.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field1089 = (source_ml_source1_spread1086.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field1090 = (source_ml_source1_spread1086.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field1091 = (source_ml_source1_spread1086.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field1092 = (source_ml_source1_spread1086.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field1093 = (source_ml_source1_spread1086.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field1094 = (source_ml_source1_spread1086.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field1095 = (source_ml_source1_spread1086.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field1096 = (source_ml_source1_spread1086.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field1097 = (source_ml_source1_spread1086.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field1098 = (source_ml_source1_spread1086.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field1099 = true;
                                                                                            {
                                                                                                let source_ml_source1_field1100 = value.clone();
                                                                                                (source_ml_source1_spread_field1087, source_ml_source1_spread_field1088, source_ml_source1_spread_field1089, source_ml_source1_spread_field1090, source_ml_source1_spread_field1091.clone(), source_ml_source1_spread_field1092, source_ml_source1_field1100.clone(), source_ml_source1_field1099, source_ml_source1_spread_field1095.clone(), source_ml_source1_spread_field1096, source_ml_source1_spread_field1097, source_ml_source1_spread_field1098)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value1101, _, _, _) => {
                        let source_ml_source1_data_property_value1101 = *source_ml_source1_data_property_value1101;
                        source_ml_source1_data_property_value1101.clone()
                    }
                }), (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1102, _, _) => {
                        source_ml_source1_data_property_value1102
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value1103) => {
                    source_ml_source1_data_property_value1103
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source1_s1104 = self::source_ml_temporary_step_java_script_descriptor_conversion_loop31(heap.clone(), current.clone());
                        match source_ml_source1_s1104.clone() {
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Done(current_2) => {
                                let current_2 = *current_2;
                                if ((self::read_java_script_property_descriptor_error(match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value1106, _) => {
                                        source_ml_source1_data_property_value1106.clone()
                                    }
                                })) != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                } else {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(match current_2.clone() {
                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value1107, _) => {
                                            source_ml_source1_data_property_value1107.clone()
                                        }
                                    })
                                }
                            }
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(source_ml_temporary_result31) => {
                                let source_ml_temporary_result31 = *source_ml_temporary_result31;
                                source_ml_temporary_result31.clone()
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1108) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1109) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1110) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1111) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1112) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1113, source_ml_source1_f1114) => {
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value1115) => {
                    source_ml_source1_data_property_value1115
                }
            }) {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 71, 101, 116]))
            } else {
                {
                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(source_ml_source1_data_property_value1116) => {
                            let source_ml_source1_data_property_value1116 = *source_ml_source1_data_property_value1116;
                            source_ml_source1_data_property_value1116.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(source_ml_source1_data_property_value1117) => {
                            let source_ml_source1_data_property_value1117 = *source_ml_source1_data_property_value1117;
                            source_ml_source1_data_property_value1117.clone()
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
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source1_f1118, source_ml_source1_f1119, source_ml_source1_f1120, source_ml_source1_f1121) => {
                                            let source_ml_source1_f1118 = *source_ml_source1_f1118;
                                            let source_ml_source1_f1119 = *source_ml_source1_f1119;
                                            let source_ml_source1_f1120 = *source_ml_source1_f1120;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source1_f1122) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source1_f1123) => {
                                            let source_ml_source1_f1123 = *source_ml_source1_f1123;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source1_f1124, source_ml_source1_f1125) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source1_f1126) => {
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
pub fn project_java_script_property_key_completion(completion: self::JavaScriptSemanticCompletion) -> self::JavaScriptPrimitiveConversionCompletion {
    match completion.clone() {
        self::JavaScriptSemanticCompletion::NormalCompletion(value) => {
            let value = *value;
            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone()))
        }
        self::JavaScriptSemanticCompletion::ThrowCompletion(value_2) => {
            let value_2 = *value_2;
            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(Box::new(value_2.clone()))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn complete_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, key: self::JavaScriptSemanticValue, descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool)) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            {
                let value_error = self::read_java_script_ordinary_value_error(heap.clone(), target.clone());
                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(value_error.clone())
                } else {
                    match target.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1127) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1128) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1129) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1130) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1131) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1132) => {
                            {
                                let transition = self::define_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone(), descriptor.clone());
                                if !((transition.clone()).2) {
                                    if (((transition.clone()).0) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid((transition.clone()).0)
                                    } else {
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 68, 101, 102, 105, 110, 101, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 106, 101, 99, 116, 101, 100, 32, 116, 104, 101, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114]))
                                    }
                                } else {
                                    self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(Box::new((transition.clone()).1), Box::new(target.clone()), (transition.clone()).3)
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
pub fn continue_java_script_property_descriptor_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, key: self::JavaScriptSemanticValue, transition: self::JavaScriptDescriptorConversionTransition) -> self::JavaScriptPropertyDefinitionTransition {
    match transition.clone() {
        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(state) => {
            let state = *state;
            self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(Box::new(target.clone()), Box::new(key.clone()), Box::new(state.clone()))))
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorCall(state_2, callee, receiver, arguments) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptPropertyDefinitionTransition::DefinitionCall(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(Box::new(target.clone()), Box::new(key.clone()), Box::new(state_2.clone()))), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(descriptor) => {
            self::complete_java_script_property_definition(heap.clone(), target.clone(), key.clone(), descriptor.clone())
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(value) => {
            let value = *value;
            self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(Box::new(value.clone()))
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(error_class, reason) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class.clone(), reason.clone())
        }
        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(reason_2) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_property_key_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, attributes: self::JavaScriptSemanticValue, transition: self::JavaScriptPropertyKeyConversionTransition) -> self::JavaScriptPropertyDefinitionTransition {
    match transition.clone() {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
            let state = *state;
            self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(Box::new(target.clone()), Box::new(attributes.clone()), Box::new(state.clone()))))
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(state_2, callee, receiver, arguments) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptPropertyDefinitionTransition::DefinitionCall(Box::new(self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(Box::new(target.clone()), Box::new(attributes.clone()), Box::new(state_2.clone()))), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(key) => {
            let key = *key;
            self::continue_java_script_property_descriptor_definition(heap.clone(), target.clone(), key.clone(), self::begin_java_script_descriptor_conversion(heap.clone(), attributes.clone()))
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
            let value = *value;
            self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(Box::new(value.clone()))
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class, reason) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class.clone(), reason.clone())
        }
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2) => {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, target: self::JavaScriptSemanticValue, key_input: self::JavaScriptSemanticValue, attributes: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            {
                let value_error = self::read_java_script_ordinary_value_error(heap.clone(), target.clone());
                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(value_error.clone())
                } else {
                    match target.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1133) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1134) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1135) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1136) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1137) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1138, source_ml_source1_f1139) => {
                            self::continue_java_script_property_key_definition(heap.clone(), target.clone(), attributes.clone(), self::begin_java_script_property_key_conversion(heap.clone(), key_input.clone(), exotic_key.clone()))
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPropertyDefinitionState) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1140, _, _) => {
                let source_ml_source1_data_property_value1140 = *source_ml_source1_data_property_value1140;
                source_ml_source1_data_property_value1140.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1141, _, _) => {
                let source_ml_source1_data_property_value1141 = *source_ml_source1_data_property_value1141;
                source_ml_source1_data_property_value1141.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source1_s1144 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1152, _, _) => {
                        let source_ml_source1_data_property_value1152 = *source_ml_source1_data_property_value1152;
                        source_ml_source1_data_property_value1152.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1153, _, _) => {
                        let source_ml_source1_data_property_value1153 = *source_ml_source1_data_property_value1153;
                        source_ml_source1_data_property_value1153.clone()
                    }
                };
                match source_ml_source1_s1144.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1145) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1146) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1147) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1148) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1149) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1150, source_ml_source1_f1151) => {
                        false
                    }
                }
            } {
                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
            } else {
                match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(target, attributes, conversion) => {
                        let target = *target;
                        let attributes = *attributes;
                        let conversion = *conversion;
                        self::continue_java_script_property_key_definition(heap.clone(), target.clone(), attributes.clone(), self::step_java_script_property_key_conversion(heap.clone(), conversion.clone()))
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(target_2, key, conversion_2) => {
                        let target_2 = *target_2;
                        let key = *key;
                        let conversion_2 = *conversion_2;
                        self::continue_java_script_property_descriptor_definition(heap.clone(), target_2.clone(), key.clone(), self::step_java_script_descriptor_conversion(heap.clone(), conversion_2.clone()))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_property_definition(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPropertyDefinitionState, completion: self::JavaScriptSemanticCompletion) -> self::JavaScriptPropertyDefinitionTransition {
    {
        let error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1154, _, _) => {
                let source_ml_source1_data_property_value1154 = *source_ml_source1_data_property_value1154;
                source_ml_source1_data_property_value1154.clone()
            }
            self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1155, _, _) => {
                let source_ml_source1_data_property_value1155 = *source_ml_source1_data_property_value1155;
                source_ml_source1_data_property_value1155.clone()
            }
        });
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(error.clone())
        } else {
            if {
                let source_ml_source1_s1158 = match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(source_ml_source1_data_property_value1166, _, _) => {
                        let source_ml_source1_data_property_value1166 = *source_ml_source1_data_property_value1166;
                        source_ml_source1_data_property_value1166.clone()
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(source_ml_source1_data_property_value1167, _, _) => {
                        let source_ml_source1_data_property_value1167 = *source_ml_source1_data_property_value1167;
                        source_ml_source1_data_property_value1167.clone()
                    }
                };
                match source_ml_source1_s1158.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1159) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f1160) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f1161) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f1162) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1163) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f1164, source_ml_source1_f1165) => {
                        false
                    }
                }
            } {
                self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
            } else {
                match state.clone() {
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionKey(target, attributes, conversion) => {
                        let target = *target;
                        let attributes = *attributes;
                        let conversion = *conversion;
                        self::continue_java_script_property_key_definition(heap.clone(), target.clone(), attributes.clone(), self::resume_java_script_property_key_conversion(heap.clone(), conversion.clone(), self::project_java_script_property_key_completion(completion.clone())))
                    }
                    self::JavaScriptPropertyDefinitionState::PropertyDefinitionDescriptor(target_2, key, conversion_2) => {
                        let target_2 = *target_2;
                        let key = *key;
                        let conversion_2 = *conversion_2;
                        self::continue_java_script_property_descriptor_definition(heap.clone(), target_2.clone(), key.clone(), self::resume_java_script_descriptor_conversion(heap.clone(), conversion_2.clone(), completion.clone()))
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
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
