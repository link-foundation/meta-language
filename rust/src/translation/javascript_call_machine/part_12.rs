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
use super::plan_java_script_ordinary_get;
use super::read_java_script_descriptor_conversion_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop121;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_descriptor_field(state: self::JavaScriptDescriptorConversionState, value: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            if (((shared_constructor_field2 == (4f64)) || (shared_constructor_field2 == (5f64))) && !(self::test_java_script_semantic_accessor(value.clone()))) {
                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
            } else {
                {
                    let before = shared_constructor_field3.clone();
                    {
                        let descriptor = if (shared_constructor_field2 == (0f64)) {
                            {
                                let source_ml_source2_spread944 = before.clone();
                                {
                                    let source_ml_source2_spread_field945 = (source_ml_source2_spread944.clone()).0;
                                    {
                                        let source_ml_source2_spread_field946 = (source_ml_source2_spread944.clone()).1;
                                        {
                                            let source_ml_source2_spread_field947 = (source_ml_source2_spread944.clone()).2;
                                            {
                                                let source_ml_source2_spread_field948 = (source_ml_source2_spread944.clone()).3;
                                                {
                                                    let source_ml_source2_spread_field949 = (source_ml_source2_spread944.clone()).4;
                                                    {
                                                        let source_ml_source2_spread_field950 = (source_ml_source2_spread944.clone()).5;
                                                        {
                                                            let source_ml_source2_spread_field951 = (source_ml_source2_spread944.clone()).6;
                                                            {
                                                                let source_ml_source2_spread_field952 = (source_ml_source2_spread944.clone()).7;
                                                                {
                                                                    let source_ml_source2_spread_field953 = (source_ml_source2_spread944.clone()).8;
                                                                    {
                                                                        let source_ml_source2_spread_field954 = (source_ml_source2_spread944.clone()).9;
                                                                        {
                                                                            let source_ml_source2_spread_field955 = (source_ml_source2_spread944.clone()).10;
                                                                            {
                                                                                let source_ml_source2_spread_field956 = (source_ml_source2_spread944.clone()).11;
                                                                                {
                                                                                    let source_ml_source2_field957 = true;
                                                                                    {
                                                                                        let source_ml_source2_field958 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                        (source_ml_source2_spread_field945, source_ml_source2_spread_field946, source_ml_source2_field958, source_ml_source2_field957, source_ml_source2_spread_field949.clone(), source_ml_source2_spread_field950, source_ml_source2_spread_field951.clone(), source_ml_source2_spread_field952, source_ml_source2_spread_field953.clone(), source_ml_source2_spread_field954, source_ml_source2_spread_field955, source_ml_source2_spread_field956)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                            if (shared_constructor_field2 == (1f64)) {
                                {
                                    let source_ml_source2_spread961 = before.clone();
                                    {
                                        let source_ml_source2_spread_field962 = (source_ml_source2_spread961.clone()).0;
                                        {
                                            let source_ml_source2_spread_field963 = (source_ml_source2_spread961.clone()).1;
                                            {
                                                let source_ml_source2_spread_field964 = (source_ml_source2_spread961.clone()).2;
                                                {
                                                    let source_ml_source2_spread_field965 = (source_ml_source2_spread961.clone()).3;
                                                    {
                                                        let source_ml_source2_spread_field966 = (source_ml_source2_spread961.clone()).4;
                                                        {
                                                            let source_ml_source2_spread_field967 = (source_ml_source2_spread961.clone()).5;
                                                            {
                                                                let source_ml_source2_spread_field968 = (source_ml_source2_spread961.clone()).6;
                                                                {
                                                                    let source_ml_source2_spread_field969 = (source_ml_source2_spread961.clone()).7;
                                                                    {
                                                                        let source_ml_source2_spread_field970 = (source_ml_source2_spread961.clone()).8;
                                                                        {
                                                                            let source_ml_source2_spread_field971 = (source_ml_source2_spread961.clone()).9;
                                                                            {
                                                                                let source_ml_source2_spread_field972 = (source_ml_source2_spread961.clone()).10;
                                                                                {
                                                                                    let source_ml_source2_spread_field973 = (source_ml_source2_spread961.clone()).11;
                                                                                    {
                                                                                        let source_ml_source2_field974 = true;
                                                                                        {
                                                                                            let source_ml_source2_field975 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                            (source_ml_source2_field975, source_ml_source2_field974, source_ml_source2_spread_field964, source_ml_source2_spread_field965, source_ml_source2_spread_field966.clone(), source_ml_source2_spread_field967, source_ml_source2_spread_field968.clone(), source_ml_source2_spread_field969, source_ml_source2_spread_field970.clone(), source_ml_source2_spread_field971, source_ml_source2_spread_field972, source_ml_source2_spread_field973)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                                if (shared_constructor_field2 == (2f64)) {
                                    {
                                        let source_ml_source2_spread978 = before.clone();
                                        {
                                            let source_ml_source2_spread_field979 = (source_ml_source2_spread978.clone()).0;
                                            {
                                                let source_ml_source2_spread_field980 = (source_ml_source2_spread978.clone()).1;
                                                {
                                                    let source_ml_source2_spread_field981 = (source_ml_source2_spread978.clone()).2;
                                                    {
                                                        let source_ml_source2_spread_field982 = (source_ml_source2_spread978.clone()).3;
                                                        {
                                                            let source_ml_source2_spread_field983 = (source_ml_source2_spread978.clone()).4;
                                                            {
                                                                let source_ml_source2_spread_field984 = (source_ml_source2_spread978.clone()).5;
                                                                {
                                                                    let source_ml_source2_spread_field985 = (source_ml_source2_spread978.clone()).6;
                                                                    {
                                                                        let source_ml_source2_spread_field986 = (source_ml_source2_spread978.clone()).7;
                                                                        {
                                                                            let source_ml_source2_spread_field987 = (source_ml_source2_spread978.clone()).8;
                                                                            {
                                                                                let source_ml_source2_spread_field988 = (source_ml_source2_spread978.clone()).9;
                                                                                {
                                                                                    let source_ml_source2_spread_field989 = (source_ml_source2_spread978.clone()).10;
                                                                                    {
                                                                                        let source_ml_source2_spread_field990 = (source_ml_source2_spread978.clone()).11;
                                                                                        {
                                                                                            let source_ml_source2_field991 = true;
                                                                                            {
                                                                                                let source_ml_source2_field992 = value.clone();
                                                                                                (source_ml_source2_spread_field979, source_ml_source2_spread_field980, source_ml_source2_spread_field981, source_ml_source2_spread_field982, source_ml_source2_spread_field983.clone(), source_ml_source2_spread_field984, source_ml_source2_spread_field985.clone(), source_ml_source2_spread_field986, source_ml_source2_field992.clone(), source_ml_source2_field991, source_ml_source2_spread_field989, source_ml_source2_spread_field990)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                                    if (shared_constructor_field2 == (3f64)) {
                                        {
                                            let source_ml_source2_spread995 = before.clone();
                                            {
                                                let source_ml_source2_spread_field996 = (source_ml_source2_spread995.clone()).0;
                                                {
                                                    let source_ml_source2_spread_field997 = (source_ml_source2_spread995.clone()).1;
                                                    {
                                                        let source_ml_source2_spread_field998 = (source_ml_source2_spread995.clone()).2;
                                                        {
                                                            let source_ml_source2_spread_field999 = (source_ml_source2_spread995.clone()).3;
                                                            {
                                                                let source_ml_source2_spread_field1000 = (source_ml_source2_spread995.clone()).4;
                                                                {
                                                                    let source_ml_source2_spread_field1001 = (source_ml_source2_spread995.clone()).5;
                                                                    {
                                                                        let source_ml_source2_spread_field1002 = (source_ml_source2_spread995.clone()).6;
                                                                        {
                                                                            let source_ml_source2_spread_field1003 = (source_ml_source2_spread995.clone()).7;
                                                                            {
                                                                                let source_ml_source2_spread_field1004 = (source_ml_source2_spread995.clone()).8;
                                                                                {
                                                                                    let source_ml_source2_spread_field1005 = (source_ml_source2_spread995.clone()).9;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1006 = (source_ml_source2_spread995.clone()).10;
                                                                                        {
                                                                                            let source_ml_source2_spread_field1007 = (source_ml_source2_spread995.clone()).11;
                                                                                            {
                                                                                                let source_ml_source2_field1008 = true;
                                                                                                {
                                                                                                    let source_ml_source2_field1009 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                                    (source_ml_source2_spread_field996, source_ml_source2_spread_field997, source_ml_source2_spread_field998, source_ml_source2_spread_field999, source_ml_source2_spread_field1000.clone(), source_ml_source2_spread_field1001, source_ml_source2_spread_field1002.clone(), source_ml_source2_spread_field1003, source_ml_source2_spread_field1004.clone(), source_ml_source2_spread_field1005, source_ml_source2_field1009, source_ml_source2_field1008)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
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
                                        if (shared_constructor_field2 == (4f64)) {
                                            {
                                                let source_ml_source2_spread1012 = before.clone();
                                                {
                                                    let source_ml_source2_spread_field1013 = (source_ml_source2_spread1012.clone()).0;
                                                    {
                                                        let source_ml_source2_spread_field1014 = (source_ml_source2_spread1012.clone()).1;
                                                        {
                                                            let source_ml_source2_spread_field1015 = (source_ml_source2_spread1012.clone()).2;
                                                            {
                                                                let source_ml_source2_spread_field1016 = (source_ml_source2_spread1012.clone()).3;
                                                                {
                                                                    let source_ml_source2_spread_field1017 = (source_ml_source2_spread1012.clone()).4;
                                                                    {
                                                                        let source_ml_source2_spread_field1018 = (source_ml_source2_spread1012.clone()).5;
                                                                        {
                                                                            let source_ml_source2_spread_field1019 = (source_ml_source2_spread1012.clone()).6;
                                                                            {
                                                                                let source_ml_source2_spread_field1020 = (source_ml_source2_spread1012.clone()).7;
                                                                                {
                                                                                    let source_ml_source2_spread_field1021 = (source_ml_source2_spread1012.clone()).8;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1022 = (source_ml_source2_spread1012.clone()).9;
                                                                                        {
                                                                                            let source_ml_source2_spread_field1023 = (source_ml_source2_spread1012.clone()).10;
                                                                                            {
                                                                                                let source_ml_source2_spread_field1024 = (source_ml_source2_spread1012.clone()).11;
                                                                                                {
                                                                                                    let source_ml_source2_field1025 = true;
                                                                                                    {
                                                                                                        let source_ml_source2_field1026 = value.clone();
                                                                                                        (source_ml_source2_spread_field1013, source_ml_source2_spread_field1014, source_ml_source2_spread_field1015, source_ml_source2_spread_field1016, source_ml_source2_field1026.clone(), source_ml_source2_field1025, source_ml_source2_spread_field1019.clone(), source_ml_source2_spread_field1020, source_ml_source2_spread_field1021.clone(), source_ml_source2_spread_field1022, source_ml_source2_spread_field1023, source_ml_source2_spread_field1024)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                                                let source_ml_source2_spread1027 = before.clone();
                                                {
                                                    let source_ml_source2_spread_field1028 = (source_ml_source2_spread1027.clone()).0;
                                                    {
                                                        let source_ml_source2_spread_field1029 = (source_ml_source2_spread1027.clone()).1;
                                                        {
                                                            let source_ml_source2_spread_field1030 = (source_ml_source2_spread1027.clone()).2;
                                                            {
                                                                let source_ml_source2_spread_field1031 = (source_ml_source2_spread1027.clone()).3;
                                                                {
                                                                    let source_ml_source2_spread_field1032 = (source_ml_source2_spread1027.clone()).4;
                                                                    {
                                                                        let source_ml_source2_spread_field1033 = (source_ml_source2_spread1027.clone()).5;
                                                                        {
                                                                            let source_ml_source2_spread_field1034 = (source_ml_source2_spread1027.clone()).6;
                                                                            {
                                                                                let source_ml_source2_spread_field1035 = (source_ml_source2_spread1027.clone()).7;
                                                                                {
                                                                                    let source_ml_source2_spread_field1036 = (source_ml_source2_spread1027.clone()).8;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1037 = (source_ml_source2_spread1027.clone()).9;
                                                                                        {
                                                                                            let source_ml_source2_spread_field1038 = (source_ml_source2_spread1027.clone()).10;
                                                                                            {
                                                                                                let source_ml_source2_spread_field1039 = (source_ml_source2_spread1027.clone()).11;
                                                                                                {
                                                                                                    let source_ml_source2_field1040 = true;
                                                                                                    {
                                                                                                        let source_ml_source2_field1041 = value.clone();
                                                                                                        (source_ml_source2_spread_field1028, source_ml_source2_spread_field1029, source_ml_source2_spread_field1030, source_ml_source2_spread_field1031, source_ml_source2_spread_field1032.clone(), source_ml_source2_spread_field1033, source_ml_source2_field1041.clone(), source_ml_source2_field1040, source_ml_source2_spread_field1036.clone(), source_ml_source2_spread_field1037, source_ml_source2_spread_field1038, source_ml_source2_spread_field1039)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                        self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(shared_constructor_field1.clone()), (shared_constructor_field2 + 1f64), descriptor.clone(), false)))
                    }
                }
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value1044) => {
                    source_ml_source2_data_property_value1044
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source2_s1045 = self::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop121(heap.clone(), current.clone());
                        match source_ml_source2_s1045.clone() {
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Done(current_2) => {
                                let current_2 = *current_2;
                                if ((self::read_java_script_property_descriptor_error(match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value1047, _) => {
                                        source_ml_source2_data_property_value1047.clone()
                                    }
                                })) != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                } else {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(match current_2.clone() {
                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value1048, _) => {
                                            source_ml_source2_data_property_value1048.clone()
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
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value1056) => {
                    source_ml_source2_data_property_value1056
                }
            }) {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 71, 101, 116]))
            } else {
                {
                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(source_ml_source2_data_property_value1057) => {
                            let source_ml_source2_data_property_value1057 = *source_ml_source2_data_property_value1057;
                            source_ml_source2_data_property_value1057.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(source_ml_source2_data_property_value1058) => {
                            let source_ml_source2_data_property_value1058 = *source_ml_source2_data_property_value1058;
                            source_ml_source2_data_property_value1058.clone()
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
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCall(_, _, _, _) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(_) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(_) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(_, _) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(_) => {
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
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if {
                        let source_ml_source2_s1069 = shared_constructor_field1.clone();
                        match source_ml_source2_s1069.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                false
                            }
                            _ => {
                                true
                            }
                        }
                    } {
                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if {
                                    let source_ml_source2_s1080 = shared_constructor_field2.clone();
                                    match source_ml_source2_s1080.clone() {
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            false
                                        }
                                        _ => {
                                            true
                                        }
                                    }
                                } {
                                    self::ml_string::Text::from_units(vec![116, 104, 101, 32, 101, 120, 111, 116, 105, 99, 32, 109, 101, 116, 104, 111, 100, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108])
                                } else {
                                    {
                                        let key_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field2.clone());
                                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                            key_error.clone()
                                        } else {
                                            if (((shared_constructor_field3 != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && (shared_constructor_field3 != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && (shared_constructor_field3 != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                                                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116])
                                            } else {
                                                if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])], &shared_constructor_field4.clone(), 0f64, false, |a, b| a == b)) {
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
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_primitive_conversion(state: self::JavaScriptPrimitiveConversionState, phase: &self::ml_string::Text) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, _) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_primitive_conversion_method(state: self::JavaScriptPrimitiveConversionState, method: self::JavaScriptSemanticValue) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let exotic = ((shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) || (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]))));
                {
                    let first = ((shared_constructor_field4 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]))) || (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]))));
                    if (exotic && (match method.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        _ => {
                            false
                        }
                    } || match method.clone() {
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        _ => {
                            false
                        }
                    })) {
                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])))
                    } else {
                        match method.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Null => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Object(_, callable) => {
                                if !callable {
                                    if exotic {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                    } else {
                                        if first {
                                            self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                        } else {
                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                        }
                                    }
                                } else {
                                    {
                                        let phase = if exotic {
                                            self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108])
                                        } else {
                                            if first {
                                                self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108])
                                            } else {
                                                self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])
                                            }
                                        };
                                        {
                                            let arguments_ = if exotic {
                                                vec![self::read_java_script_primitive_conversion_spelling(&(shared_constructor_field3))]
                                            } else {
                                                Vec::<self::JavaScriptSemanticValue>::new()
                                            };
                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())), Box::new(method.clone()), Box::new(shared_constructor_field1.clone()), arguments_)
                                        }
                                    }
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
pub fn step_java_script_primitive_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else {
                    if (((shared_constructor_field4 != (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) && (shared_constructor_field4 != (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])))) && (shared_constructor_field4 != (self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))) {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                    } else {
                        {
                            let receiver = shared_constructor_field1.clone();
                            match receiver.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Text(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                }
                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                    {
                                        let first = (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116])));
                                        {
                                            let method = if (shared_constructor_field3 == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                                                if first {
                                                    self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])
                                                }
                                            } else {
                                                if first {
                                                    self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])
                                                }
                                            };
                                            {
                                                let key = if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) {
                                                    shared_constructor_field2.clone()
                                                } else {
                                                    self::read_java_script_primitive_conversion_spelling(&(method))
                                                };
                                                {
                                                    let plan = self::plan_java_script_ordinary_get(heap.clone(), identity.clone(), key.clone(), receiver.clone());
                                                    if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid((plan.clone()).3)
                                                    } else {
                                                        if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                            {
                                                                let phase = if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) {
                                                                    self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114])
                                                                } else {
                                                                    if first {
                                                                        self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114])
                                                                    }
                                                                };
                                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(receiver.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)
                                                            }
                                                        } else {
                                                            if (((plan.clone()).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114]))
                                                            } else {
                                                                self::apply_java_script_primitive_conversion_method(state.clone(), (plan.clone()).5)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
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
pub fn begin_java_script_primitive_conversion(heap: self::JavaScriptOrdinaryHeap, input: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue, hint: &self::ml_string::Text) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(heap_error.clone())
        } else {
            {
                let error = self::read_java_script_ordinary_value_error(heap.clone(), input.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else {
                    if ((((*hint) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && ((*hint) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && ((*hint) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116]))
                    } else {
                        match input.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(input.clone()))
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                self::step_java_script_primitive_conversion(heap.clone(), self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(input.clone()), Box::new(exotic_key.clone()), hint.clone(), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116])))
                            }
                        }
                    }
                }
            }
        }
    }
}
