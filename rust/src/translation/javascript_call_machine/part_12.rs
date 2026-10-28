// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result;
use super::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_empty_descriptor;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop120;
use super::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop121;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::ml_string::Text {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            heap_error.clone()
        } else {
            if {
                let source_ml_source2_s912 = match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value920, _, _, _) => {
                        let source_ml_source2_data_property_value920 = *source_ml_source2_data_property_value920;
                        source_ml_source2_data_property_value920.clone()
                    }
                };
                match source_ml_source2_s912.clone() {
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f918, source_ml_source2_f919) => {
                        false
                    }
                    _ => {
                        true
                    }
                }
            } {
                self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value921, _, _, _) => {
                            let source_ml_source2_data_property_value921 = *source_ml_source2_data_property_value921;
                            source_ml_source2_data_property_value921.clone()
                        }
                    });
                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                        receiver_error.clone()
                    } else {
                        if ((((!(self::ml_math::is_safe_integer(match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value922, _, _) => {
                                source_ml_source2_data_property_value922
                            }
                        })) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value924, _, _) => {
                                source_ml_source2_data_property_value924
                            }
                        }) < (0f64))) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value926, _, _) => {
                                source_ml_source2_data_property_value926
                            }
                        }) > (6f64))) || (((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value928, _, _) => {
                                source_ml_source2_data_property_value928
                            }
                        }) == (0f64)) && (((1f64 / match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value930, _, _) => {
                                source_ml_source2_data_property_value930
                            }
                        })) < (0f64)))) || (match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value931) => {
                                source_ml_source2_data_property_value931
                            }
                        } && ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value933, _, _) => {
                                source_ml_source2_data_property_value933
                            }
                        }) == (6f64)))) {
                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                        } else {
                            {
                                let index = match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value934, _, _) => {
                                        source_ml_source2_data_property_value934
                                    }
                                };
                                {
                                    let source_ml_source2_s935 = self::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop120(state.clone(), index);
                                    match source_ml_source2_s935.clone() {
                                        self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Done => {
                                            {
                                                let descriptor = match state.clone() {
                                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value936, _) => {
                                                        source_ml_source2_data_property_value936.clone()
                                                    }
                                                };
                                                if ((descriptor.clone()).9 && ((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8)) != (self::ml_string::Text::from_units(vec![])))) {
                                                    self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 118, 97, 108, 117, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116])
                                                } else {
                                                    if ((descriptor.clone()).5 && (((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).4)))) {
                                                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                    } else {
                                                        if ((descriptor.clone()).7 && (((self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6)) != (self::ml_string::Text::from_units(vec![]))) || !(self::test_java_script_semantic_accessor((descriptor.clone()).6)))) {
                                                            self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![])
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Return(source_ml_source1_temporary_result120) => {
                                            source_ml_source1_temporary_result120.clone()
                                        }
                                    }
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
pub fn apply_java_script_descriptor_field(state: self::JavaScriptDescriptorConversionState, value: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    if ((((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value938, _, _) => {
            source_ml_source2_data_property_value938
        }
    }) == (4f64)) || ((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value940, _, _) => {
            source_ml_source2_data_property_value940
        }
    }) == (5f64))) && !(self::test_java_script_semantic_accessor(value.clone()))) {
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
    } else {
        {
            let before = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value941, _) => {
                    source_ml_source2_data_property_value941.clone()
                }
            };
            {
                let descriptor = if ((match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value943, _, _) => {
                        source_ml_source2_data_property_value943
                    }
                }) == (0f64)) {
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
                    if ((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value960, _, _) => {
                            source_ml_source2_data_property_value960
                        }
                    }) == (1f64)) {
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
                        if ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value977, _, _) => {
                                source_ml_source2_data_property_value977
                            }
                        }) == (2f64)) {
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
                            if ((match state.clone() {
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value994, _, _) => {
                                    source_ml_source2_data_property_value994
                                }
                            }) == (3f64)) {
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
                                if ((match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value1011, _, _) => {
                                        source_ml_source2_data_property_value1011
                                    }
                                }) == (4f64)) {
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
                self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value1042, _, _, _) => {
                        let source_ml_source2_data_property_value1042 = *source_ml_source2_data_property_value1042;
                        source_ml_source2_data_property_value1042.clone()
                    }
                }), (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value1043, _, _) => {
                        source_ml_source2_data_property_value1043
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f1049) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f1050) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f1051) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f1052) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1053) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f1054, source_ml_source2_f1055) => {
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
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source2_f1059, source_ml_source2_f1060, source_ml_source2_f1061, source_ml_source2_f1062) => {
                                            let source_ml_source2_f1059 = *source_ml_source2_f1059;
                                            let source_ml_source2_f1060 = *source_ml_source2_f1060;
                                            let source_ml_source2_f1061 = *source_ml_source2_f1061;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source2_f1063) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source2_f1064) => {
                                            let source_ml_source2_f1064 = *source_ml_source2_f1064;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source2_f1065, source_ml_source2_f1066) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source2_f1067) => {
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
                let source_ml_source2_s1069 = match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value1077, _, _, _) => {
                        let source_ml_source2_data_property_value1077 = *source_ml_source2_data_property_value1077;
                        source_ml_source2_data_property_value1077.clone()
                    }
                };
                match source_ml_source2_s1069.clone() {
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f1075, source_ml_source2_f1076) => {
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
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value1078, _, _, _) => {
                            let source_ml_source2_data_property_value1078 = *source_ml_source2_data_property_value1078;
                            source_ml_source2_data_property_value1078.clone()
                        }
                    });
                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                        receiver_error.clone()
                    } else {
                        if {
                            let source_ml_source2_s1080 = match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value1088, _, _) => {
                                    let source_ml_source2_data_property_value1088 = *source_ml_source2_data_property_value1088;
                                    source_ml_source2_data_property_value1088.clone()
                                }
                            };
                            match source_ml_source2_s1080.clone() {
                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f1085) => {
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
                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value1089, _, _) => {
                                        let source_ml_source2_data_property_value1089 = *source_ml_source2_data_property_value1089;
                                        source_ml_source2_data_property_value1089.clone()
                                    }
                                });
                                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                    key_error.clone()
                                } else {
                                    if ((((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1091, _) => {
                                            source_ml_source2_data_property_value1091.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1093, _) => {
                                            source_ml_source2_data_property_value1093.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1095, _) => {
                                            source_ml_source2_data_property_value1095.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116])
                                    } else {
                                        if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])], &match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source2_data_property_value1096) => {
                                                source_ml_source2_data_property_value1096.clone()
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
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value1097, _, _, _) => {
            let source_ml_source2_data_property_value1097 = *source_ml_source2_data_property_value1097;
            source_ml_source2_data_property_value1097.clone()
        }
    }), Box::new(match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value1098, _, _) => {
            let source_ml_source2_data_property_value1098 = *source_ml_source2_data_property_value1098;
            source_ml_source2_data_property_value1098.clone()
        }
    }), match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1099, _) => {
            source_ml_source2_data_property_value1099.clone()
        }
    }, phase.clone())))
}
