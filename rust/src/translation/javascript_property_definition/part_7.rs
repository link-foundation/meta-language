// Generated from complete js/src/translation/javascript-property-definition.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Result;
use super::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result;
use super::begin_java_script_primitive_conversion;
use super::ml;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_property_key_spelling;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_semantic_value_error;
use super::resume_java_script_primitive_conversion;
use super::source_ml_temporary_finish_java_script_property_key_conversion_loop29;
use super::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop30;
use super::source_ml_temporary_step_java_script_descriptor_conversion_loop31;
use super::step_java_script_primitive_conversion;
use super::test_java_script_semantic_accessor;

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
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f843) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f844) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f845) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f846) => {
                    true
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f847) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f848, source_ml_source1_f849) => {
                    false
                }
            } || match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f850) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f851) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f852) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f853) => {
                    false
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f854) => {
                    true
                }
                self::JavaScriptSemanticValue::Object(source_ml_source1_f855, source_ml_source1_f856) => {
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
                                            let units_2 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling), units, index);
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
                                            let units_4 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_2), units_3, index_2);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_4)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f857) => {
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
                                            let units_6 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_3), units_5, index_3);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_6)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f858) => {
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
                                            let units_8 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_4), units_7, index_4);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_8)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f859) => {
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
                                            let units_10 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_5), units_9, index_5);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_10)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f860) => {
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
                                            let units_12 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_6), units_11, index_6);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_12)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f861) => {
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
                                            let units_14 = self::source_ml_temporary_finish_java_script_property_key_conversion_loop29(&(spelling_7), units_13, index_7);
                                            self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(Box::new(self::JavaScriptSemanticValue::Text(units_14)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f862, source_ml_source1_f863) => {
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
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value865, _) => {
            source_ml_source1_data_property_value865.clone()
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
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value867, _) => {
            source_ml_source1_data_property_value867.clone()
        }
    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 114, 105, 110, 103, 32, 104, 105, 110, 116]))
    } else {
        self::finish_java_script_property_key_conversion(self::resume_java_script_primitive_conversion(heap.clone(), state.clone(), completion.clone()))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_field(index: f64) -> (self::ml_string::Text, self::ml_string::Text, Vec<f64>) {
    if (index == (0f64)) {
        {
            let source_ml_source1_field868 = self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]);
            {
                let source_ml_source1_field869 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                {
                    let source_ml_source1_field870 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                    (source_ml_source1_field869.clone(), source_ml_source1_field868.clone(), source_ml_source1_field870)
                }
            }
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source1_field871 = self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]);
                {
                    let source_ml_source1_field872 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                    {
                        let source_ml_source1_field873 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                        (source_ml_source1_field872.clone(), source_ml_source1_field871.clone(), source_ml_source1_field873)
                    }
                }
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source1_field874 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                    {
                        let source_ml_source1_field875 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                        {
                            let source_ml_source1_field876 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                            (source_ml_source1_field875.clone(), source_ml_source1_field874.clone(), source_ml_source1_field876)
                        }
                    }
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source1_field877 = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]);
                        {
                            let source_ml_source1_field878 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                            {
                                let source_ml_source1_field879 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                                (source_ml_source1_field878.clone(), source_ml_source1_field877.clone(), source_ml_source1_field879)
                            }
                        }
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source1_field880 = self::ml_string::Text::from_units(vec![103, 101, 116]);
                            {
                                let source_ml_source1_field881 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                {
                                    let source_ml_source1_field882 = vec![103f64, 101f64, 116f64];
                                    (source_ml_source1_field881.clone(), source_ml_source1_field880.clone(), source_ml_source1_field882)
                                }
                            }
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source1_field883 = self::ml_string::Text::from_units(vec![115, 101, 116]);
                                {
                                    let source_ml_source1_field884 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                    {
                                        let source_ml_source1_field885 = vec![115f64, 101f64, 116f64];
                                        (source_ml_source1_field884.clone(), source_ml_source1_field883.clone(), source_ml_source1_field885)
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_field886 = self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source1_field887 = self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]);
                                    {
                                        let source_ml_source1_field888 = Vec::<f64>::new();
                                        (source_ml_source1_field887.clone(), source_ml_source1_field886.clone(), source_ml_source1_field888)
                                    }
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
pub fn convert_java_script_semantic_boolean(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![84, 111, 66, 111, 111, 108, 101, 97, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                false
            }
            self::JavaScriptSemanticValue::Null => {
                false
            }
            self::JavaScriptSemanticValue::Boolean(value_value) => {
                value_value
            }
            self::JavaScriptSemanticValue::Number(value_value_2) => {
                ((value_value_2 != (0f64)) && !(value_value_2.is_nan()))
            }
            self::JavaScriptSemanticValue::Integer(value_value_3) => {
                (value_value_3 != (self::ml::Big::from_i128(0)))
            }
            self::JavaScriptSemanticValue::Text(units) => {
                (((units.len() as f64)) != (0f64))
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f889) => {
                true
            }
            self::JavaScriptSemanticValue::Object(source_ml_source1_f890, source_ml_source1_f891) => {
                true
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source1_field892 = false;
        {
            let source_ml_source1_field893 = false;
            {
                let source_ml_source1_field894 = false;
                {
                    let source_ml_source1_field895 = false;
                    {
                        let source_ml_source1_field896 = false;
                        {
                            let source_ml_source1_field897 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_field898 = false;
                                {
                                    let source_ml_source1_field899 = false;
                                    {
                                        let source_ml_source1_field900 = false;
                                        {
                                            let source_ml_source1_field901 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_field902 = false;
                                                {
                                                    let source_ml_source1_field903 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source1_field893, source_ml_source1_field892, source_ml_source1_field895, source_ml_source1_field894, source_ml_source1_field901.clone(), source_ml_source1_field900, source_ml_source1_field903.clone(), source_ml_source1_field902, source_ml_source1_field897.clone(), source_ml_source1_field896, source_ml_source1_field899, source_ml_source1_field898)
                                                }
                                            }
                                        }
                                    }
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
pub fn test_java_script_descriptor_field_presence(descriptor: (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), index: f64) -> bool {
    if (index == (0f64)) {
        (descriptor.clone()).3
    } else {
        if (index == (1f64)) {
            (descriptor.clone()).1
        } else {
            if (index == (2f64)) {
                (descriptor.clone()).9
            } else {
                if (index == (3f64)) {
                    (descriptor.clone()).11
                } else {
                    if (index == (4f64)) {
                        (descriptor.clone()).5
                    } else {
                        if (index == (5f64)) {
                            (descriptor.clone()).7
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_conversion_state_error(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::ml_string::Text {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            heap_error.clone()
        } else {
            if {
                let source_ml_source1_s905 = match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value913, _, _, _) => {
                        let source_ml_source1_data_property_value913 = *source_ml_source1_data_property_value913;
                        source_ml_source1_data_property_value913.clone()
                    }
                };
                match source_ml_source1_s905.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f906) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f907) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f908) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f909) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f910) => {
                        true
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f911, source_ml_source1_f912) => {
                        false
                    }
                }
            } {
                self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114])
            } else {
                {
                    let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value914, _, _, _) => {
                            let source_ml_source1_data_property_value914 = *source_ml_source1_data_property_value914;
                            source_ml_source1_data_property_value914.clone()
                        }
                    });
                    if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                        receiver_error.clone()
                    } else {
                        if ((((!(self::ml_math::is_safe_integer(match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value915, _, _) => {
                                source_ml_source1_data_property_value915
                            }
                        })) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value917, _, _) => {
                                source_ml_source1_data_property_value917
                            }
                        }) < (0f64))) || ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value919, _, _) => {
                                source_ml_source1_data_property_value919
                            }
                        }) > (6f64))) || (((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value921, _, _) => {
                                source_ml_source1_data_property_value921
                            }
                        }) == (0f64)) && (((1f64 / match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value923, _, _) => {
                                source_ml_source1_data_property_value923
                            }
                        })) < (0f64)))) || (match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value924) => {
                                source_ml_source1_data_property_value924
                            }
                        } && ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value926, _, _) => {
                                source_ml_source1_data_property_value926
                            }
                        }) == (6f64)))) {
                            self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                        } else {
                            {
                                let index = match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value927, _, _) => {
                                        source_ml_source1_data_property_value927
                                    }
                                };
                                {
                                    let source_ml_source1_s928 = self::source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop30(state.clone(), index);
                                    match source_ml_source1_s928.clone() {
                                        self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Done => {
                                            {
                                                let descriptor = match state.clone() {
                                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value929, _) => {
                                                        source_ml_source1_data_property_value929.clone()
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
                                        self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Return(source_ml_temporary_result30) => {
                                            source_ml_temporary_result30.clone()
                                        }
                                    }
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
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value931, _, _) => {
            source_ml_source1_data_property_value931
        }
    }) == (4f64)) || ((match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value933, _, _) => {
            source_ml_source1_data_property_value933
        }
    }) == (5f64))) && !(self::test_java_script_semantic_accessor(value.clone()))) {
        self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
    } else {
        {
            let before = match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value934, _) => {
                    source_ml_source1_data_property_value934.clone()
                }
            };
            {
                let enumerable = ((match state.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value936, _, _) => {
                        source_ml_source1_data_property_value936
                    }
                }) == (0f64));
                {
                    let configurable = ((match state.clone() {
                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value938, _, _) => {
                            source_ml_source1_data_property_value938
                        }
                    }) == (1f64));
                    {
                        let data_value = ((match state.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value940, _, _) => {
                                source_ml_source1_data_property_value940
                            }
                        }) == (2f64));
                        {
                            let writable = ((match state.clone() {
                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value942, _, _) => {
                                    source_ml_source1_data_property_value942
                                }
                            }) == (3f64));
                            {
                                let getter = ((match state.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value944, _, _) => {
                                        source_ml_source1_data_property_value944
                                    }
                                }) == (4f64));
                                {
                                    let setter = !(((((enumerable || configurable) || data_value) || writable) || getter));
                                    {
                                        let flag = if ((enumerable || configurable) || writable) {
                                            self::convert_java_script_semantic_boolean(value.clone())
                                        } else {
                                            false
                                        };
                                        {
                                            let descriptor = {
                                                let source_ml_source1_spread945 = before.clone();
                                                {
                                                    let source_ml_source1_spread_field946 = (source_ml_source1_spread945.clone()).0;
                                                    {
                                                        let source_ml_source1_spread_field947 = (source_ml_source1_spread945.clone()).1;
                                                        {
                                                            let source_ml_source1_spread_field948 = (source_ml_source1_spread945.clone()).2;
                                                            {
                                                                let source_ml_source1_spread_field949 = (source_ml_source1_spread945.clone()).3;
                                                                {
                                                                    let source_ml_source1_spread_field950 = (source_ml_source1_spread945.clone()).4;
                                                                    {
                                                                        let source_ml_source1_spread_field951 = (source_ml_source1_spread945.clone()).5;
                                                                        {
                                                                            let source_ml_source1_spread_field952 = (source_ml_source1_spread945.clone()).6;
                                                                            {
                                                                                let source_ml_source1_spread_field953 = (source_ml_source1_spread945.clone()).7;
                                                                                {
                                                                                    let source_ml_source1_spread_field954 = (source_ml_source1_spread945.clone()).8;
                                                                                    {
                                                                                        let source_ml_source1_spread_field955 = (source_ml_source1_spread945.clone()).9;
                                                                                        {
                                                                                            let source_ml_source1_spread_field956 = (source_ml_source1_spread945.clone()).10;
                                                                                            {
                                                                                                let source_ml_source1_spread_field957 = (source_ml_source1_spread945.clone()).11;
                                                                                                {
                                                                                                    let source_ml_source1_field958 = (enumerable || (before.clone()).3);
                                                                                                    {
                                                                                                        let source_ml_source1_field959 = if enumerable {
                                                                                                            flag
                                                                                                        } else {
                                                                                                            (before.clone()).2
                                                                                                        };
                                                                                                        {
                                                                                                            let source_ml_source1_field960 = (configurable || (before.clone()).1);
                                                                                                            {
                                                                                                                let source_ml_source1_field961 = if configurable {
                                                                                                                    flag
                                                                                                                } else {
                                                                                                                    (before.clone()).0
                                                                                                                };
                                                                                                                {
                                                                                                                    let source_ml_source1_field962 = (data_value || (before.clone()).9);
                                                                                                                    {
                                                                                                                        let source_ml_source1_field963 = if data_value {
                                                                                                                            value.clone()
                                                                                                                        } else {
                                                                                                                            (before.clone()).8
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let source_ml_source1_field964 = (writable || (before.clone()).11);
                                                                                                                            {
                                                                                                                                let source_ml_source1_field965 = if writable {
                                                                                                                                    flag
                                                                                                                                } else {
                                                                                                                                    (before.clone()).10
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source1_field966 = (getter || (before.clone()).5);
                                                                                                                                    {
                                                                                                                                        let source_ml_source1_field967 = if getter {
                                                                                                                                            value.clone()
                                                                                                                                        } else {
                                                                                                                                            (before.clone()).4
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_field968 = (setter || (before.clone()).7);
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_field969 = if setter {
                                                                                                                                                    value.clone()
                                                                                                                                                } else {
                                                                                                                                                    (before.clone()).6
                                                                                                                                                };
                                                                                                                                                (source_ml_source1_field961, source_ml_source1_field960, source_ml_source1_field959, source_ml_source1_field958, source_ml_source1_field967.clone(), source_ml_source1_field966, source_ml_source1_field969.clone(), source_ml_source1_field968, source_ml_source1_field963.clone(), source_ml_source1_field962, source_ml_source1_field965, source_ml_source1_field964)
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
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
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value970, _, _, _) => {
                                                    let source_ml_source1_data_property_value970 = *source_ml_source1_data_property_value970;
                                                    source_ml_source1_data_property_value970.clone()
                                                }
                                            }), (match state.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value971, _, _) => {
                                                    source_ml_source1_data_property_value971
                                                }
                                            } + 1f64), descriptor.clone(), false)))
                                        }
                                    }
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
pub fn step_java_script_descriptor_conversion(heap: self::JavaScriptOrdinaryHeap, state: self::JavaScriptDescriptorConversionState) -> self::JavaScriptDescriptorConversionTransition {
    {
        let error = self::read_java_script_descriptor_conversion_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(error.clone())
        } else {
            if match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source1_data_property_value972) => {
                    source_ml_source1_data_property_value972
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source1_s973 = self::source_ml_temporary_step_java_script_descriptor_conversion_loop31(heap.clone(), current.clone());
                        match source_ml_source1_s973.clone() {
                            self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Done(current_2) => {
                                let current_2 = *current_2;
                                if ((self::read_java_script_property_descriptor_error(match current_2.clone() {
                                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value975, _) => {
                                        source_ml_source1_data_property_value975.clone()
                                    }
                                })) != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 98, 111, 116, 104, 32, 100, 97, 116, 97, 32, 97, 110, 100, 32, 97, 99, 99, 101, 115, 115, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115]))
                                } else {
                                    self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(match current_2.clone() {
                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value976, _) => {
                                            source_ml_source1_data_property_value976.clone()
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f977) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f978) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f979) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f980) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f981) => {
                            self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116]))
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f982, source_ml_source1_f983) => {
                            self::step_java_script_descriptor_conversion(heap.clone(), self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), 0f64, self::create_java_script_empty_descriptor(), false))
                        }
                    }
                }
            }
        }
    }
}
