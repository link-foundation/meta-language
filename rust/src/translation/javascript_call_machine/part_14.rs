// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptPropertyDefinitionState;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result;
use super::begin_java_script_primitive_conversion;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_empty_descriptor;
use super::define_java_script_ordinary_property;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_descriptor_conversion_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::resume_java_script_primitive_conversion;
use super::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop125;
use super::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop124;
use super::step_java_script_primitive_conversion;
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
                                let source_ml_source2_spread_field1122 = (before.clone()).0;
                                {
                                    let source_ml_source2_spread_field1123 = (before.clone()).1;
                                    {
                                        let source_ml_source2_spread_field1124 = (before.clone()).2;
                                        {
                                            let source_ml_source2_spread_field1125 = (before.clone()).3;
                                            {
                                                let source_ml_source2_spread_field1126 = (before.clone()).4;
                                                {
                                                    let source_ml_source2_spread_field1127 = (before.clone()).5;
                                                    {
                                                        let source_ml_source2_spread_field1128 = (before.clone()).6;
                                                        {
                                                            let source_ml_source2_spread_field1129 = (before.clone()).7;
                                                            {
                                                                let source_ml_source2_spread_field1130 = (before.clone()).8;
                                                                {
                                                                    let source_ml_source2_spread_field1131 = (before.clone()).9;
                                                                    {
                                                                        let source_ml_source2_spread_field1132 = (before.clone()).10;
                                                                        {
                                                                            let source_ml_source2_spread_field1133 = (before.clone()).11;
                                                                            {
                                                                                let source_ml_source2_field1135 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                (source_ml_source2_spread_field1122, source_ml_source2_spread_field1123, source_ml_source2_field1135, true, source_ml_source2_spread_field1126.clone(), source_ml_source2_spread_field1127, source_ml_source2_spread_field1128.clone(), source_ml_source2_spread_field1129, source_ml_source2_spread_field1130.clone(), source_ml_source2_spread_field1131, source_ml_source2_spread_field1132, source_ml_source2_spread_field1133)
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                                    let source_ml_source2_spread_field1139 = (before.clone()).0;
                                    {
                                        let source_ml_source2_spread_field1140 = (before.clone()).1;
                                        {
                                            let source_ml_source2_spread_field1141 = (before.clone()).2;
                                            {
                                                let source_ml_source2_spread_field1142 = (before.clone()).3;
                                                {
                                                    let source_ml_source2_spread_field1143 = (before.clone()).4;
                                                    {
                                                        let source_ml_source2_spread_field1144 = (before.clone()).5;
                                                        {
                                                            let source_ml_source2_spread_field1145 = (before.clone()).6;
                                                            {
                                                                let source_ml_source2_spread_field1146 = (before.clone()).7;
                                                                {
                                                                    let source_ml_source2_spread_field1147 = (before.clone()).8;
                                                                    {
                                                                        let source_ml_source2_spread_field1148 = (before.clone()).9;
                                                                        {
                                                                            let source_ml_source2_spread_field1149 = (before.clone()).10;
                                                                            {
                                                                                let source_ml_source2_spread_field1150 = (before.clone()).11;
                                                                                {
                                                                                    let source_ml_source2_field1152 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                    (source_ml_source2_field1152, true, source_ml_source2_spread_field1141, source_ml_source2_spread_field1142, source_ml_source2_spread_field1143.clone(), source_ml_source2_spread_field1144, source_ml_source2_spread_field1145.clone(), source_ml_source2_spread_field1146, source_ml_source2_spread_field1147.clone(), source_ml_source2_spread_field1148, source_ml_source2_spread_field1149, source_ml_source2_spread_field1150)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                                        let source_ml_source2_spread_field1156 = (before.clone()).0;
                                        {
                                            let source_ml_source2_spread_field1157 = (before.clone()).1;
                                            {
                                                let source_ml_source2_spread_field1158 = (before.clone()).2;
                                                {
                                                    let source_ml_source2_spread_field1159 = (before.clone()).3;
                                                    {
                                                        let source_ml_source2_spread_field1160 = (before.clone()).4;
                                                        {
                                                            let source_ml_source2_spread_field1161 = (before.clone()).5;
                                                            {
                                                                let source_ml_source2_spread_field1162 = (before.clone()).6;
                                                                {
                                                                    let source_ml_source2_spread_field1163 = (before.clone()).7;
                                                                    {
                                                                        let source_ml_source2_spread_field1164 = (before.clone()).8;
                                                                        {
                                                                            let source_ml_source2_spread_field1165 = (before.clone()).9;
                                                                            {
                                                                                let source_ml_source2_spread_field1166 = (before.clone()).10;
                                                                                {
                                                                                    let source_ml_source2_spread_field1167 = (before.clone()).11;
                                                                                    (source_ml_source2_spread_field1156, source_ml_source2_spread_field1157, source_ml_source2_spread_field1158, source_ml_source2_spread_field1159, source_ml_source2_spread_field1160.clone(), source_ml_source2_spread_field1161, source_ml_source2_spread_field1162.clone(), source_ml_source2_spread_field1163, value.clone(), true, source_ml_source2_spread_field1166, source_ml_source2_spread_field1167)
                                                                                }
                                                                            }
                                                                        }
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
                                            let source_ml_source2_spread_field1173 = (before.clone()).0;
                                            {
                                                let source_ml_source2_spread_field1174 = (before.clone()).1;
                                                {
                                                    let source_ml_source2_spread_field1175 = (before.clone()).2;
                                                    {
                                                        let source_ml_source2_spread_field1176 = (before.clone()).3;
                                                        {
                                                            let source_ml_source2_spread_field1177 = (before.clone()).4;
                                                            {
                                                                let source_ml_source2_spread_field1178 = (before.clone()).5;
                                                                {
                                                                    let source_ml_source2_spread_field1179 = (before.clone()).6;
                                                                    {
                                                                        let source_ml_source2_spread_field1180 = (before.clone()).7;
                                                                        {
                                                                            let source_ml_source2_spread_field1181 = (before.clone()).8;
                                                                            {
                                                                                let source_ml_source2_spread_field1182 = (before.clone()).9;
                                                                                {
                                                                                    let source_ml_source2_spread_field1183 = (before.clone()).10;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1184 = (before.clone()).11;
                                                                                        {
                                                                                            let source_ml_source2_field1186 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                            (source_ml_source2_spread_field1173, source_ml_source2_spread_field1174, source_ml_source2_spread_field1175, source_ml_source2_spread_field1176, source_ml_source2_spread_field1177.clone(), source_ml_source2_spread_field1178, source_ml_source2_spread_field1179.clone(), source_ml_source2_spread_field1180, source_ml_source2_spread_field1181.clone(), source_ml_source2_spread_field1182, source_ml_source2_field1186, true)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
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
                                                let source_ml_source2_spread_field1190 = (before.clone()).0;
                                                {
                                                    let source_ml_source2_spread_field1191 = (before.clone()).1;
                                                    {
                                                        let source_ml_source2_spread_field1192 = (before.clone()).2;
                                                        {
                                                            let source_ml_source2_spread_field1193 = (before.clone()).3;
                                                            {
                                                                let source_ml_source2_spread_field1194 = (before.clone()).4;
                                                                {
                                                                    let source_ml_source2_spread_field1195 = (before.clone()).5;
                                                                    {
                                                                        let source_ml_source2_spread_field1196 = (before.clone()).6;
                                                                        {
                                                                            let source_ml_source2_spread_field1197 = (before.clone()).7;
                                                                            {
                                                                                let source_ml_source2_spread_field1198 = (before.clone()).8;
                                                                                {
                                                                                    let source_ml_source2_spread_field1199 = (before.clone()).9;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1200 = (before.clone()).10;
                                                                                        {
                                                                                            let source_ml_source2_spread_field1201 = (before.clone()).11;
                                                                                            (source_ml_source2_spread_field1190, source_ml_source2_spread_field1191, source_ml_source2_spread_field1192, source_ml_source2_spread_field1193, value.clone(), true, source_ml_source2_spread_field1196.clone(), source_ml_source2_spread_field1197, source_ml_source2_spread_field1198.clone(), source_ml_source2_spread_field1199, source_ml_source2_spread_field1200, source_ml_source2_spread_field1201)
                                                                                        }
                                                                                    }
                                                                                }
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
                                                let source_ml_source2_spread_field1205 = (before.clone()).0;
                                                {
                                                    let source_ml_source2_spread_field1206 = (before.clone()).1;
                                                    {
                                                        let source_ml_source2_spread_field1207 = (before.clone()).2;
                                                        {
                                                            let source_ml_source2_spread_field1208 = (before.clone()).3;
                                                            {
                                                                let source_ml_source2_spread_field1209 = (before.clone()).4;
                                                                {
                                                                    let source_ml_source2_spread_field1210 = (before.clone()).5;
                                                                    {
                                                                        let source_ml_source2_spread_field1211 = (before.clone()).6;
                                                                        {
                                                                            let source_ml_source2_spread_field1212 = (before.clone()).7;
                                                                            {
                                                                                let source_ml_source2_spread_field1213 = (before.clone()).8;
                                                                                {
                                                                                    let source_ml_source2_spread_field1214 = (before.clone()).9;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1215 = (before.clone()).10;
                                                                                        {
                                                                                            let source_ml_source2_spread_field1216 = (before.clone()).11;
                                                                                            (source_ml_source2_spread_field1205, source_ml_source2_spread_field1206, source_ml_source2_spread_field1207, source_ml_source2_spread_field1208, source_ml_source2_spread_field1209.clone(), source_ml_source2_spread_field1210, value.clone(), true, source_ml_source2_spread_field1213.clone(), source_ml_source2_spread_field1214, source_ml_source2_spread_field1215, source_ml_source2_spread_field1216)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value1221) => {
                    source_ml_source2_data_property_value1221
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source2_s1222 = self::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop124(heap.clone(), current.clone());
                        match source_ml_source2_s1222.clone() {
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Done(current_2) => {
                                let current_2 = *current_2;
                                match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, shared_constructor_field1, _) => {
                                        if ((self::read_java_script_property_descriptor_error(shared_constructor_field1.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                        } else {
                                            self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(shared_constructor_field1.clone())
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(source_ml_source1_temporary_result124) => {
                                let source_ml_source1_temporary_result124 = *source_ml_source1_temporary_result124;
                                source_ml_source1_temporary_result124.clone()
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value1233) => {
                    source_ml_source2_data_property_value1233
                }
            }) {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 71, 101, 116]))
            } else {
                {
                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(source_ml_source2_data_property_value1234) => {
                            let source_ml_source2_data_property_value1234 = *source_ml_source2_data_property_value1234;
                            source_ml_source2_data_property_value1234.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(source_ml_source2_data_property_value1235) => {
                            let source_ml_source2_data_property_value1235 = *source_ml_source2_data_property_value1235;
                            source_ml_source2_data_property_value1235.clone()
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
pub fn read_java_script_primitive_property_key_spelling(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
        }
        self::JavaScriptSemanticValue::Null => {
            self::ml_string::Text::from_units(vec![110, 117, 108, 108])
        }
        self::JavaScriptSemanticValue::Boolean(value_value) => {
            if value_value {
                self::ml_string::Text::from_units(vec![116, 114, 117, 101])
            } else {
                self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
            }
        }
        self::JavaScriptSemanticValue::Number(value_value_2) => {
            self::ml_string::Text::from_text(&self::ml_number::js_number(value_value_2))
        }
        self::JavaScriptSemanticValue::Integer(value_value_3) => {
            self::ml_string::Text::from_text(&value_value_3.to_string())
        }
        self::JavaScriptSemanticValue::Text(_) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 117, 110, 105, 116, 115])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Symbol(_) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 101, 120, 97, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Object(_, _) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])).0, message_present: true }).raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_property_key_conversion(transition: self::JavaScriptPrimitiveConversionTransition) -> self::JavaScriptPropertyKeyConversionTransition {
    match transition.clone() {
        self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(state) => {
            let state = *state;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(Box::new(state.clone()))
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(state_2, callee, receiver, arguments) => {
            let state_2 = *state_2;
            let callee = *callee;
            let receiver = *receiver;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(Box::new(state_2.clone()), Box::new(callee.clone()), Box::new(receiver.clone()), arguments)
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(value) => {
            let value = *value;
            if (match value.clone() {
                self::JavaScriptSemanticValue::Text(_) => {
                    true
                }
                _ => {
                    false
                }
            } || match value.clone() {
                self::JavaScriptSemanticValue::Symbol(_) => {
                    true
                }
                _ => {
                    false
                }
            }) {
                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(value.clone()))
            } else {
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        {
                            let spelling = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units = Vec::<f64>::new();
                                    {
                                        let index = 0f64;
                                        {
                                            let units_2 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop125(&(spelling), units, index);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_2)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Null => {
                        {
                            let spelling_2 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_2.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_3 = Vec::<f64>::new();
                                    {
                                        let index_2 = 0f64;
                                        {
                                            let units_4 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop125(&(spelling_2), units_3, index_2);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_4)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        {
                            let spelling_3 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_3.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_5 = Vec::<f64>::new();
                                    {
                                        let index_3 = 0f64;
                                        {
                                            let units_6 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop125(&(spelling_3), units_5, index_3);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_6)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        {
                            let spelling_4 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_4.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_7 = Vec::<f64>::new();
                                    {
                                        let index_4 = 0f64;
                                        {
                                            let units_8 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop125(&(spelling_4), units_7, index_4);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_8)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        {
                            let spelling_5 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_5.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_9 = Vec::<f64>::new();
                                    {
                                        let index_5 = 0f64;
                                        {
                                            let units_10 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop125(&(spelling_5), units_9, index_5);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_10)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        {
                            let spelling_6 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_6.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_11 = Vec::<f64>::new();
                                    {
                                        let index_6 = 0f64;
                                        {
                                            let units_12 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop125(&(spelling_6), units_11, index_6);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_12)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        {
                            let spelling_7 = self::read_java_script_primitive_property_key_spelling(value.clone());
                            if ((spelling_7.length()) > (1048576f64)) {
                                self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116]))
                            } else {
                                {
                                    let units_13 = Vec::<f64>::new();
                                    {
                                        let index_7 = 0f64;
                                        {
                                            let units_14 = self::source_ml_source1_temporary_finish_java_script_property_key_conversion_loop125(&(spelling_7), units_13, index_7);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_14)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                    }
                }
            }
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(value_2) => {
            let value_2 = *value_2;
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(Box::new(value_2.clone()))
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(error_class, reason) => {
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class.clone(), reason.clone())
        }
        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(reason_2) => {
            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2.clone())
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_property_key_conversion(heap: self::JavaScriptOrdinaryHeap, input: self::JavaScriptSemanticValue, exotic_key: self::JavaScriptSemanticValue) -> self::JavaScriptPropertyKeyConversionTransition {
    self::finish_java_script_property_key_conversion(self::begin_java_script_primitive_conversion(heap.clone(), input.clone(), exotic_key.clone(), &(self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_property_key_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState) -> self::JavaScriptPropertyKeyConversionTransition {
    if ((match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1273, _) => {
            source_ml_source2_data_property_value1273.clone()
        }
    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110, 103, 32, 104, 105, 110, 116]))
    } else {
        self::finish_java_script_property_key_conversion(self::step_java_script_primitive_conversion(heap.clone(), state.clone()))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_property_key_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptPrimitiveConversionState, completion: self::JavaScriptPrimitiveConversionCompletion) -> self::JavaScriptPropertyKeyConversionTransition {
    if ((match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value1275, _) => {
            source_ml_source2_data_property_value1275.clone()
        }
    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110, 103, 32, 104, 105, 110, 116]))
    } else {
        self::finish_java_script_property_key_conversion(self::resume_java_script_primitive_conversion(heap.clone(), state.clone(), completion.clone()))
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
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(identity, _) => {
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
                        self::JavaScriptSemanticValue::Boolean(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116, 32, 116, 97, 114, 103, 101, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            self::continue_java_script_property_key_definition(heap.clone(), target.clone(), attributes.clone(), self::begin_java_script_property_key_conversion(heap.clone(), key_input.clone(), exotic_key.clone()))
                        }
                    }
                }
            }
        }
    }
}
