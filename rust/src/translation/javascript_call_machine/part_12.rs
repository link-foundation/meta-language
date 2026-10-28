// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_empty_descriptor;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_descriptor_conversion_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::source_ml_temporary_step_java_script_descriptor_conversion_loop63;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_descriptor_field(state: self::JavaScriptDescriptorConversionState, value: self::JavaScriptSemanticValue) -> self::JavaScriptDescriptorConversionTransition {
    if ((((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value859, _, _) => {
            source_ml_source1_data_property_value859
        }
    }) == (4f64)) || ((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value861, _, _) => {
            source_ml_source1_data_property_value861
        }
    }) == (5f64))) && !(self::test_java_script_semantic_accessor(value.clone()))) {
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
    } else {
        {
            let before = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value862, _) => {
                    source_ml_source1_data_property_value862.clone()
                }
            };
            {
                let descriptor = if ((match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value864, _, _) => {
                        source_ml_source1_data_property_value864
                    }
                }) == (0f64)) {
                    {
                        let source_ml_source1_spread865 = before.clone();
                        {
                            let source_ml_source1_spread_field866 = (source_ml_source1_spread865.clone()).0;
                            {
                                let source_ml_source1_spread_field867 = (source_ml_source1_spread865.clone()).1;
                                {
                                    let source_ml_source1_spread_field868 = (source_ml_source1_spread865.clone()).2;
                                    {
                                        let source_ml_source1_spread_field869 = (source_ml_source1_spread865.clone()).3;
                                        {
                                            let source_ml_source1_spread_field870 = (source_ml_source1_spread865.clone()).4;
                                            {
                                                let source_ml_source1_spread_field871 = (source_ml_source1_spread865.clone()).5;
                                                {
                                                    let source_ml_source1_spread_field872 = (source_ml_source1_spread865.clone()).6;
                                                    {
                                                        let source_ml_source1_spread_field873 = (source_ml_source1_spread865.clone()).7;
                                                        {
                                                            let source_ml_source1_spread_field874 = (source_ml_source1_spread865.clone()).8;
                                                            {
                                                                let source_ml_source1_spread_field875 = (source_ml_source1_spread865.clone()).9;
                                                                {
                                                                    let source_ml_source1_spread_field876 = (source_ml_source1_spread865.clone()).10;
                                                                    {
                                                                        let source_ml_source1_spread_field877 = (source_ml_source1_spread865.clone()).11;
                                                                        {
                                                                            let source_ml_source1_field878 = true;
                                                                            {
                                                                                let source_ml_source1_field879 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                (source_ml_source1_spread_field866, source_ml_source1_spread_field867, source_ml_source1_field879, source_ml_source1_field878, source_ml_source1_spread_field870.clone(), source_ml_source1_spread_field871, source_ml_source1_spread_field872.clone(), source_ml_source1_spread_field873, source_ml_source1_spread_field874.clone(), source_ml_source1_spread_field875, source_ml_source1_spread_field876, source_ml_source1_spread_field877)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value881, _, _) => {
                            source_ml_source1_data_property_value881
                        }
                    }) == (1f64)) {
                        {
                            let source_ml_source1_spread882 = before.clone();
                            {
                                let source_ml_source1_spread_field883 = (source_ml_source1_spread882.clone()).0;
                                {
                                    let source_ml_source1_spread_field884 = (source_ml_source1_spread882.clone()).1;
                                    {
                                        let source_ml_source1_spread_field885 = (source_ml_source1_spread882.clone()).2;
                                        {
                                            let source_ml_source1_spread_field886 = (source_ml_source1_spread882.clone()).3;
                                            {
                                                let source_ml_source1_spread_field887 = (source_ml_source1_spread882.clone()).4;
                                                {
                                                    let source_ml_source1_spread_field888 = (source_ml_source1_spread882.clone()).5;
                                                    {
                                                        let source_ml_source1_spread_field889 = (source_ml_source1_spread882.clone()).6;
                                                        {
                                                            let source_ml_source1_spread_field890 = (source_ml_source1_spread882.clone()).7;
                                                            {
                                                                let source_ml_source1_spread_field891 = (source_ml_source1_spread882.clone()).8;
                                                                {
                                                                    let source_ml_source1_spread_field892 = (source_ml_source1_spread882.clone()).9;
                                                                    {
                                                                        let source_ml_source1_spread_field893 = (source_ml_source1_spread882.clone()).10;
                                                                        {
                                                                            let source_ml_source1_spread_field894 = (source_ml_source1_spread882.clone()).11;
                                                                            {
                                                                                let source_ml_source1_field895 = true;
                                                                                {
                                                                                    let source_ml_source1_field896 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                    (source_ml_source1_field896, source_ml_source1_field895, source_ml_source1_spread_field885, source_ml_source1_spread_field886, source_ml_source1_spread_field887.clone(), source_ml_source1_spread_field888, source_ml_source1_spread_field889.clone(), source_ml_source1_spread_field890, source_ml_source1_spread_field891.clone(), source_ml_source1_spread_field892, source_ml_source1_spread_field893, source_ml_source1_spread_field894)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
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
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value898, _, _) => {
                                source_ml_source1_data_property_value898
                            }
                        }) == (2f64)) {
                            {
                                let source_ml_source1_spread899 = before.clone();
                                {
                                    let source_ml_source1_spread_field900 = (source_ml_source1_spread899.clone()).0;
                                    {
                                        let source_ml_source1_spread_field901 = (source_ml_source1_spread899.clone()).1;
                                        {
                                            let source_ml_source1_spread_field902 = (source_ml_source1_spread899.clone()).2;
                                            {
                                                let source_ml_source1_spread_field903 = (source_ml_source1_spread899.clone()).3;
                                                {
                                                    let source_ml_source1_spread_field904 = (source_ml_source1_spread899.clone()).4;
                                                    {
                                                        let source_ml_source1_spread_field905 = (source_ml_source1_spread899.clone()).5;
                                                        {
                                                            let source_ml_source1_spread_field906 = (source_ml_source1_spread899.clone()).6;
                                                            {
                                                                let source_ml_source1_spread_field907 = (source_ml_source1_spread899.clone()).7;
                                                                {
                                                                    let source_ml_source1_spread_field908 = (source_ml_source1_spread899.clone()).8;
                                                                    {
                                                                        let source_ml_source1_spread_field909 = (source_ml_source1_spread899.clone()).9;
                                                                        {
                                                                            let source_ml_source1_spread_field910 = (source_ml_source1_spread899.clone()).10;
                                                                            {
                                                                                let source_ml_source1_spread_field911 = (source_ml_source1_spread899.clone()).11;
                                                                                {
                                                                                    let source_ml_source1_field912 = true;
                                                                                    {
                                                                                        let source_ml_source1_field913 = value.clone();
                                                                                        (source_ml_source1_spread_field900, source_ml_source1_spread_field901, source_ml_source1_spread_field902, source_ml_source1_spread_field903, source_ml_source1_spread_field904.clone(), source_ml_source1_spread_field905, source_ml_source1_spread_field906.clone(), source_ml_source1_spread_field907, source_ml_source1_field913.clone(), source_ml_source1_field912, source_ml_source1_spread_field910, source_ml_source1_spread_field911)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value915, _, _) => {
                                    source_ml_source1_data_property_value915
                                }
                            }) == (3f64)) {
                                {
                                    let source_ml_source1_spread916 = before.clone();
                                    {
                                        let source_ml_source1_spread_field917 = (source_ml_source1_spread916.clone()).0;
                                        {
                                            let source_ml_source1_spread_field918 = (source_ml_source1_spread916.clone()).1;
                                            {
                                                let source_ml_source1_spread_field919 = (source_ml_source1_spread916.clone()).2;
                                                {
                                                    let source_ml_source1_spread_field920 = (source_ml_source1_spread916.clone()).3;
                                                    {
                                                        let source_ml_source1_spread_field921 = (source_ml_source1_spread916.clone()).4;
                                                        {
                                                            let source_ml_source1_spread_field922 = (source_ml_source1_spread916.clone()).5;
                                                            {
                                                                let source_ml_source1_spread_field923 = (source_ml_source1_spread916.clone()).6;
                                                                {
                                                                    let source_ml_source1_spread_field924 = (source_ml_source1_spread916.clone()).7;
                                                                    {
                                                                        let source_ml_source1_spread_field925 = (source_ml_source1_spread916.clone()).8;
                                                                        {
                                                                            let source_ml_source1_spread_field926 = (source_ml_source1_spread916.clone()).9;
                                                                            {
                                                                                let source_ml_source1_spread_field927 = (source_ml_source1_spread916.clone()).10;
                                                                                {
                                                                                    let source_ml_source1_spread_field928 = (source_ml_source1_spread916.clone()).11;
                                                                                    {
                                                                                        let source_ml_source1_field929 = true;
                                                                                        {
                                                                                            let source_ml_source1_field930 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                            (source_ml_source1_spread_field917, source_ml_source1_spread_field918, source_ml_source1_spread_field919, source_ml_source1_spread_field920, source_ml_source1_spread_field921.clone(), source_ml_source1_spread_field922, source_ml_source1_spread_field923.clone(), source_ml_source1_spread_field924, source_ml_source1_spread_field925.clone(), source_ml_source1_spread_field926, source_ml_source1_field930, source_ml_source1_field929)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value932, _, _) => {
                                        source_ml_source1_data_property_value932
                                    }
                                }) == (4f64)) {
                                    {
                                        let source_ml_source1_spread933 = before.clone();
                                        {
                                            let source_ml_source1_spread_field934 = (source_ml_source1_spread933.clone()).0;
                                            {
                                                let source_ml_source1_spread_field935 = (source_ml_source1_spread933.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field936 = (source_ml_source1_spread933.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field937 = (source_ml_source1_spread933.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field938 = (source_ml_source1_spread933.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field939 = (source_ml_source1_spread933.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field940 = (source_ml_source1_spread933.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field941 = (source_ml_source1_spread933.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field942 = (source_ml_source1_spread933.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field943 = (source_ml_source1_spread933.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field944 = (source_ml_source1_spread933.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field945 = (source_ml_source1_spread933.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field946 = true;
                                                                                            {
                                                                                                let source_ml_source1_field947 = value.clone();
                                                                                                (source_ml_source1_spread_field934, source_ml_source1_spread_field935, source_ml_source1_spread_field936, source_ml_source1_spread_field937, source_ml_source1_field947.clone(), source_ml_source1_field946, source_ml_source1_spread_field940.clone(), source_ml_source1_spread_field941, source_ml_source1_spread_field942.clone(), source_ml_source1_spread_field943, source_ml_source1_spread_field944, source_ml_source1_spread_field945)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                                        let source_ml_source1_spread948 = before.clone();
                                        {
                                            let source_ml_source1_spread_field949 = (source_ml_source1_spread948.clone()).0;
                                            {
                                                let source_ml_source1_spread_field950 = (source_ml_source1_spread948.clone()).1;
                                                {
                                                    let source_ml_source1_spread_field951 = (source_ml_source1_spread948.clone()).2;
                                                    {
                                                        let source_ml_source1_spread_field952 = (source_ml_source1_spread948.clone()).3;
                                                        {
                                                            let source_ml_source1_spread_field953 = (source_ml_source1_spread948.clone()).4;
                                                            {
                                                                let source_ml_source1_spread_field954 = (source_ml_source1_spread948.clone()).5;
                                                                {
                                                                    let source_ml_source1_spread_field955 = (source_ml_source1_spread948.clone()).6;
                                                                    {
                                                                        let source_ml_source1_spread_field956 = (source_ml_source1_spread948.clone()).7;
                                                                        {
                                                                            let source_ml_source1_spread_field957 = (source_ml_source1_spread948.clone()).8;
                                                                            {
                                                                                let source_ml_source1_spread_field958 = (source_ml_source1_spread948.clone()).9;
                                                                                {
                                                                                    let source_ml_source1_spread_field959 = (source_ml_source1_spread948.clone()).10;
                                                                                    {
                                                                                        let source_ml_source1_spread_field960 = (source_ml_source1_spread948.clone()).11;
                                                                                        {
                                                                                            let source_ml_source1_field961 = true;
                                                                                            {
                                                                                                let source_ml_source1_field962 = value.clone();
                                                                                                (source_ml_source1_spread_field949, source_ml_source1_spread_field950, source_ml_source1_spread_field951, source_ml_source1_spread_field952, source_ml_source1_spread_field953.clone(), source_ml_source1_spread_field954, source_ml_source1_field962.clone(), source_ml_source1_field961, source_ml_source1_spread_field957.clone(), source_ml_source1_spread_field958, source_ml_source1_spread_field959, source_ml_source1_spread_field960)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
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
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value963, _, _, _) => {
                        let source_ml_source1_data_property_value963 = *source_ml_source1_data_property_value963;
                        source_ml_source1_data_property_value963.clone()
                    }
                }), (match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value964, _, _) => {
                        source_ml_source1_data_property_value964
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value965) => {
                    source_ml_source1_data_property_value965
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source1_s966 = self::source_ml_temporary_step_java_script_descriptor_conversion_loop63(heap.clone(), current.clone());
                        match source_ml_source1_s966.clone() {
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Done(current_2) => {
                                let current_2 = *current_2;
                                if ((self::read_java_script_property_descriptor_error(match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value968, _) => {
                                        source_ml_source1_data_property_value968.clone()
                                    }
                                })) != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                } else {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(match current_2.clone() {
                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value969, _) => {
                                            source_ml_source1_data_property_value969.clone()
                                        }
                                    })
                                }
                            }
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(source_ml_temporary_result63) => {
                                let source_ml_temporary_result63 = *source_ml_temporary_result63;
                                source_ml_temporary_result63.clone()
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f970) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f971) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f972) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f973) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f974) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f975, source_ml_source1_f976) => {
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value977) => {
                    source_ml_source1_data_property_value977
                }
            }) {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 71, 101, 116]))
            } else {
                {
                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(source_ml_source1_data_property_value978) => {
                            let source_ml_source1_data_property_value978 = *source_ml_source1_data_property_value978;
                            source_ml_source1_data_property_value978.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(source_ml_source1_data_property_value979) => {
                            let source_ml_source1_data_property_value979 = *source_ml_source1_data_property_value979;
                            source_ml_source1_data_property_value979.clone()
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
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source1_f980, source_ml_source1_f981, source_ml_source1_f982, source_ml_source1_f983) => {
                                            let source_ml_source1_f980 = *source_ml_source1_f980;
                                            let source_ml_source1_f981 = *source_ml_source1_f981;
                                            let source_ml_source1_f982 = *source_ml_source1_f982;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source1_f984) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source1_f985) => {
                                            let source_ml_source1_f985 = *source_ml_source1_f985;
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source1_f986, source_ml_source1_f987) => {
                                            applied.clone()
                                        }
                                        self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source1_f988) => {
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
                let source_ml_source1_s990 = match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value998, _, _, _) => {
                        let source_ml_source1_data_property_value998 = *source_ml_source1_data_property_value998;
                        source_ml_source1_data_property_value998.clone()
                    }
                };
                match source_ml_source1_s990.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f991) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f992) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f993) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f994) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f995) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f996, source_ml_source1_f997) => {
                        false
                    }
                }
            } {
                self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value999, _, _, _) => {
                            let source_ml_source1_data_property_value999 = *source_ml_source1_data_property_value999;
                            source_ml_source1_data_property_value999.clone()
                        }
                    });
                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                        receiver_error.clone()
                    } else {
                        if {
                            let source_ml_source1_s1001 = match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value1009, _, _) => {
                                    let source_ml_source1_data_property_value1009 = *source_ml_source1_data_property_value1009;
                                    source_ml_source1_data_property_value1009.clone()
                                }
                            };
                            match source_ml_source1_s1001.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1002) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Number(source_ml_source1_f1003) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1004) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Text(source_ml_source1_f1005) => {
                                    true
                                }
                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1006) => {
                                    false
                                }
                                self::JavaScriptSemanticValue::Object(source_ml_source1_f1007, source_ml_source1_f1008) => {
                                    true
                                }
                            }
                        } {
                            self::ml_string::Text::from_units(vec![116, 104, 101, 32, 101, 120, 111, 116, 105, 99, 32, 109, 101, 116, 104, 111, 100, 32, 107, 101, 121, 32, 109, 117, 115, 116, 32, 98, 101, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108])
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value1010, _, _) => {
                                        let source_ml_source1_data_property_value1010 = *source_ml_source1_data_property_value1010;
                                        source_ml_source1_data_property_value1010.clone()
                                    }
                                });
                                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                    key_error.clone()
                                } else {
                                    if ((((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value1012, _) => {
                                            source_ml_source1_data_property_value1012.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116]))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value1014, _) => {
                                            source_ml_source1_data_property_value1014.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) && ((match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value1016, _) => {
                                            source_ml_source1_data_property_value1016.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))) {
                                        self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32, 112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112, 101, 32, 104, 105, 110, 116])
                                    } else {
                                        if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]), self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 67, 97, 108, 108])], &match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value1017) => {
                                                source_ml_source1_data_property_value1017.clone()
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
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value1018, _, _, _) => {
            let source_ml_source1_data_property_value1018 = *source_ml_source1_data_property_value1018;
            source_ml_source1_data_property_value1018.clone()
        }
    }), Box::new(match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value1019, _, _) => {
            let source_ml_source1_data_property_value1019 = *source_ml_source1_data_property_value1019;
            source_ml_source1_data_property_value1019.clone()
        }
    }), match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value1020, _) => {
            source_ml_source1_data_property_value1020.clone()
        }
    }, phase.clone())))
}
