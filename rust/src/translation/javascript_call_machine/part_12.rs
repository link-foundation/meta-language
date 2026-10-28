// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result;
use super::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_semantic_value_error;
use super::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop122;
use super::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop123;
use super::test_java_script_semantic_accessor;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_code_registers(instruction: self::JavaScriptCallInstruction) -> Vec<f64> {
    match instruction.clone() {
        self::JavaScriptCallInstruction::Constant(destination, _) => {
            vec![destination]
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            vec![destination_2, source]
        }
        self::JavaScriptCallInstruction::Argument(destination_3, _) => {
            vec![destination_3]
        }
        self::JavaScriptCallInstruction::Jump(_) => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::Branch(condition, _, _) => {
            vec![condition]
        }
        self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
            {
                let source_ml_source2_concat_value1015 = vec![destination_4, callee, receiver];
                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value1015), arguments)
            }
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            vec![source_2]
        }
        self::JavaScriptCallInstruction::Throw(source_3) => {
            vec![source_3]
        }
        self::JavaScriptCallInstruction::PushHandler(destination_5, _) => {
            vec![destination_5]
        }
        self::JavaScriptCallInstruction::PopHandler => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_6, left, right, _) => {
            vec![destination_6, left, right]
        }
        self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
            vec![destination_7, left_2, right_2]
        }
        self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
            vec![destination_8, left_3, right_3]
        }
        self::JavaScriptCallInstruction::RequireWrite(success) => {
            vec![success]
        }
        self::JavaScriptCallInstruction::Primitive(destination_9, _, arguments_2) => {
            {
                let source_ml_source2_concat_value1017 = vec![destination_9];
                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value1017), arguments_2)
            }
        }
        self::JavaScriptCallInstruction::OrdinaryGet(destination_10, object, key, receiver_2) => {
            vec![destination_10, object, key, receiver_2]
        }
        self::JavaScriptCallInstruction::OrdinarySet(destination_11, object_2, key_2, value, receiver_3) => {
            vec![destination_11, object_2, key_2, value, receiver_3]
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_12, input, _) => {
            vec![destination_12, input]
        }
        self::JavaScriptCallInstruction::PropertyDefinition(destination_13, target, key_3, attributes, _) => {
            vec![destination_13, target, key_3, attributes]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_strict_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Null => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Boolean(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Number(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value) => {
                        (left_value == value)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(_) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                self::test_java_script_semantic_same_value(left.clone(), right.clone())
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_descriptor_field(index: f64) -> (self::ml_string::Text, self::ml_string::Text, Vec<f64>) {
    if (index == (0f64)) {
        {
            let source_ml_source2_field1031 = self::ml_string::Text::from_units(vec![101, 110, 117, 109, 101, 114, 97, 98, 108, 101]);
            {
                let source_ml_source2_field1032 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                {
                    let source_ml_source2_field1033 = vec![101f64, 110f64, 117f64, 109f64, 101f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                    (source_ml_source2_field1032.clone(), source_ml_source2_field1031.clone(), source_ml_source2_field1033)
                }
            }
        }
    } else {
        if (index == (1f64)) {
            {
                let source_ml_source2_field1034 = self::ml_string::Text::from_units(vec![99, 111, 110, 102, 105, 103, 117, 114, 97, 98, 108, 101]);
                {
                    let source_ml_source2_field1035 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                    {
                        let source_ml_source2_field1036 = vec![99f64, 111f64, 110f64, 102f64, 105f64, 103f64, 117f64, 114f64, 97f64, 98f64, 108f64, 101f64];
                        (source_ml_source2_field1035.clone(), source_ml_source2_field1034.clone(), source_ml_source2_field1036)
                    }
                }
            }
        } else {
            if (index == (2f64)) {
                {
                    let source_ml_source2_field1037 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                    {
                        let source_ml_source2_field1038 = self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]);
                        {
                            let source_ml_source2_field1039 = vec![118f64, 97f64, 108f64, 117f64, 101f64];
                            (source_ml_source2_field1038.clone(), source_ml_source2_field1037.clone(), source_ml_source2_field1039)
                        }
                    }
                }
            } else {
                if (index == (3f64)) {
                    {
                        let source_ml_source2_field1040 = self::ml_string::Text::from_units(vec![119, 114, 105, 116, 97, 98, 108, 101]);
                        {
                            let source_ml_source2_field1041 = self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]);
                            {
                                let source_ml_source2_field1042 = vec![119f64, 114f64, 105f64, 116f64, 97f64, 98f64, 108f64, 101f64];
                                (source_ml_source2_field1041.clone(), source_ml_source2_field1040.clone(), source_ml_source2_field1042)
                            }
                        }
                    }
                } else {
                    if (index == (4f64)) {
                        {
                            let source_ml_source2_field1043 = self::ml_string::Text::from_units(vec![103, 101, 116]);
                            {
                                let source_ml_source2_field1044 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                {
                                    let source_ml_source2_field1045 = vec![103f64, 101f64, 116f64];
                                    (source_ml_source2_field1044.clone(), source_ml_source2_field1043.clone(), source_ml_source2_field1045)
                                }
                            }
                        }
                    } else {
                        if (index == (5f64)) {
                            {
                                let source_ml_source2_field1046 = self::ml_string::Text::from_units(vec![115, 101, 116]);
                                {
                                    let source_ml_source2_field1047 = self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]);
                                    {
                                        let source_ml_source2_field1048 = vec![115f64, 101f64, 116f64];
                                        (source_ml_source2_field1047.clone(), source_ml_source2_field1046.clone(), source_ml_source2_field1048)
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source2_field1049 = self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source2_field1050 = self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]);
                                    {
                                        let source_ml_source2_field1051 = Vec::<f64>::new();
                                        (source_ml_source2_field1050.clone(), source_ml_source2_field1049.clone(), source_ml_source2_field1051)
                                    }
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
            self::JavaScriptSemanticValue::Symbol(_) => {
                true
            }
            self::JavaScriptSemanticValue::Object(_, _) => {
                true
            }
            _ => {
                false
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_empty_descriptor() -> (bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool) {
    {
        let source_ml_source2_field1055 = false;
        {
            let source_ml_source2_field1056 = false;
            {
                let source_ml_source2_field1057 = false;
                {
                    let source_ml_source2_field1058 = false;
                    {
                        let source_ml_source2_field1059 = false;
                        {
                            let source_ml_source2_field1060 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_field1061 = false;
                                {
                                    let source_ml_source2_field1062 = false;
                                    {
                                        let source_ml_source2_field1063 = false;
                                        {
                                            let source_ml_source2_field1064 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source2_field1065 = false;
                                                {
                                                    let source_ml_source2_field1066 = self::JavaScriptSemanticValue::Undefined;
                                                    (source_ml_source2_field1056, source_ml_source2_field1055, source_ml_source2_field1058, source_ml_source2_field1057, source_ml_source2_field1064.clone(), source_ml_source2_field1063, source_ml_source2_field1066.clone(), source_ml_source2_field1065, source_ml_source2_field1060.clone(), source_ml_source2_field1059, source_ml_source2_field1062, source_ml_source2_field1061)
                                                }
                                            }
                                        }
                                    }
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
    match state.clone() {
        self::JavaScriptDescriptorConversionState::DescriptorConversion(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    heap_error.clone()
                } else {
                    if {
                        let source_ml_source2_s1068 = shared_constructor_field1.clone();
                        match source_ml_source2_s1068.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
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
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                receiver_error.clone()
                            } else {
                                if ((((!(self::ml_math::is_safe_integer(shared_constructor_field2)) || (shared_constructor_field2 < (0f64))) || (shared_constructor_field2 > (6f64))) || ((shared_constructor_field2 == (0f64)) && (((1f64 / shared_constructor_field2)) < (0f64)))) || (shared_constructor_field4 && (shared_constructor_field2 == (6f64)))) {
                                    self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 102, 105, 101, 108, 100, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110])
                                } else {
                                    {
                                        let index = shared_constructor_field2;
                                        {
                                            let source_ml_source2_s1091 = self::source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop122(state.clone(), index);
                                            match source_ml_source2_s1091.clone() {
                                                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Done => {
                                                    {
                                                        let descriptor = shared_constructor_field3.clone();
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
                                                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Return(source_ml_source1_temporary_result122) => {
                                                    source_ml_source1_temporary_result122.clone()
                                                }
                                            }
                                        }
                                    }
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
                                let source_ml_source2_spread_field1101 = (before.clone()).0;
                                {
                                    let source_ml_source2_spread_field1102 = (before.clone()).1;
                                    {
                                        let source_ml_source2_spread_field1103 = (before.clone()).2;
                                        {
                                            let source_ml_source2_spread_field1104 = (before.clone()).3;
                                            {
                                                let source_ml_source2_spread_field1105 = (before.clone()).4;
                                                {
                                                    let source_ml_source2_spread_field1106 = (before.clone()).5;
                                                    {
                                                        let source_ml_source2_spread_field1107 = (before.clone()).6;
                                                        {
                                                            let source_ml_source2_spread_field1108 = (before.clone()).7;
                                                            {
                                                                let source_ml_source2_spread_field1109 = (before.clone()).8;
                                                                {
                                                                    let source_ml_source2_spread_field1110 = (before.clone()).9;
                                                                    {
                                                                        let source_ml_source2_spread_field1111 = (before.clone()).10;
                                                                        {
                                                                            let source_ml_source2_spread_field1112 = (before.clone()).11;
                                                                            {
                                                                                let source_ml_source2_field1113 = true;
                                                                                {
                                                                                    let source_ml_source2_field1114 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                    (source_ml_source2_spread_field1101, source_ml_source2_spread_field1102, source_ml_source2_field1114, source_ml_source2_field1113, source_ml_source2_spread_field1105.clone(), source_ml_source2_spread_field1106, source_ml_source2_spread_field1107.clone(), source_ml_source2_spread_field1108, source_ml_source2_spread_field1109.clone(), source_ml_source2_spread_field1110, source_ml_source2_spread_field1111, source_ml_source2_spread_field1112)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                                    let source_ml_source2_spread_field1118 = (before.clone()).0;
                                    {
                                        let source_ml_source2_spread_field1119 = (before.clone()).1;
                                        {
                                            let source_ml_source2_spread_field1120 = (before.clone()).2;
                                            {
                                                let source_ml_source2_spread_field1121 = (before.clone()).3;
                                                {
                                                    let source_ml_source2_spread_field1122 = (before.clone()).4;
                                                    {
                                                        let source_ml_source2_spread_field1123 = (before.clone()).5;
                                                        {
                                                            let source_ml_source2_spread_field1124 = (before.clone()).6;
                                                            {
                                                                let source_ml_source2_spread_field1125 = (before.clone()).7;
                                                                {
                                                                    let source_ml_source2_spread_field1126 = (before.clone()).8;
                                                                    {
                                                                        let source_ml_source2_spread_field1127 = (before.clone()).9;
                                                                        {
                                                                            let source_ml_source2_spread_field1128 = (before.clone()).10;
                                                                            {
                                                                                let source_ml_source2_spread_field1129 = (before.clone()).11;
                                                                                {
                                                                                    let source_ml_source2_field1130 = true;
                                                                                    {
                                                                                        let source_ml_source2_field1131 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                        (source_ml_source2_field1131, source_ml_source2_field1130, source_ml_source2_spread_field1120, source_ml_source2_spread_field1121, source_ml_source2_spread_field1122.clone(), source_ml_source2_spread_field1123, source_ml_source2_spread_field1124.clone(), source_ml_source2_spread_field1125, source_ml_source2_spread_field1126.clone(), source_ml_source2_spread_field1127, source_ml_source2_spread_field1128, source_ml_source2_spread_field1129)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
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
                                        let source_ml_source2_spread_field1135 = (before.clone()).0;
                                        {
                                            let source_ml_source2_spread_field1136 = (before.clone()).1;
                                            {
                                                let source_ml_source2_spread_field1137 = (before.clone()).2;
                                                {
                                                    let source_ml_source2_spread_field1138 = (before.clone()).3;
                                                    {
                                                        let source_ml_source2_spread_field1139 = (before.clone()).4;
                                                        {
                                                            let source_ml_source2_spread_field1140 = (before.clone()).5;
                                                            {
                                                                let source_ml_source2_spread_field1141 = (before.clone()).6;
                                                                {
                                                                    let source_ml_source2_spread_field1142 = (before.clone()).7;
                                                                    {
                                                                        let source_ml_source2_spread_field1143 = (before.clone()).8;
                                                                        {
                                                                            let source_ml_source2_spread_field1144 = (before.clone()).9;
                                                                            {
                                                                                let source_ml_source2_spread_field1145 = (before.clone()).10;
                                                                                {
                                                                                    let source_ml_source2_spread_field1146 = (before.clone()).11;
                                                                                    {
                                                                                        let source_ml_source2_field1147 = true;
                                                                                        (source_ml_source2_spread_field1135, source_ml_source2_spread_field1136, source_ml_source2_spread_field1137, source_ml_source2_spread_field1138, source_ml_source2_spread_field1139.clone(), source_ml_source2_spread_field1140, source_ml_source2_spread_field1141.clone(), source_ml_source2_spread_field1142, value.clone(), source_ml_source2_field1147, source_ml_source2_spread_field1145, source_ml_source2_spread_field1146)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
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
                                            let source_ml_source2_spread_field1152 = (before.clone()).0;
                                            {
                                                let source_ml_source2_spread_field1153 = (before.clone()).1;
                                                {
                                                    let source_ml_source2_spread_field1154 = (before.clone()).2;
                                                    {
                                                        let source_ml_source2_spread_field1155 = (before.clone()).3;
                                                        {
                                                            let source_ml_source2_spread_field1156 = (before.clone()).4;
                                                            {
                                                                let source_ml_source2_spread_field1157 = (before.clone()).5;
                                                                {
                                                                    let source_ml_source2_spread_field1158 = (before.clone()).6;
                                                                    {
                                                                        let source_ml_source2_spread_field1159 = (before.clone()).7;
                                                                        {
                                                                            let source_ml_source2_spread_field1160 = (before.clone()).8;
                                                                            {
                                                                                let source_ml_source2_spread_field1161 = (before.clone()).9;
                                                                                {
                                                                                    let source_ml_source2_spread_field1162 = (before.clone()).10;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1163 = (before.clone()).11;
                                                                                        {
                                                                                            let source_ml_source2_field1164 = true;
                                                                                            {
                                                                                                let source_ml_source2_field1165 = self::convert_java_script_semantic_boolean(value.clone());
                                                                                                (source_ml_source2_spread_field1152, source_ml_source2_spread_field1153, source_ml_source2_spread_field1154, source_ml_source2_spread_field1155, source_ml_source2_spread_field1156.clone(), source_ml_source2_spread_field1157, source_ml_source2_spread_field1158.clone(), source_ml_source2_spread_field1159, source_ml_source2_spread_field1160.clone(), source_ml_source2_spread_field1161, source_ml_source2_field1165, source_ml_source2_field1164)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
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
                                                let source_ml_source2_spread_field1169 = (before.clone()).0;
                                                {
                                                    let source_ml_source2_spread_field1170 = (before.clone()).1;
                                                    {
                                                        let source_ml_source2_spread_field1171 = (before.clone()).2;
                                                        {
                                                            let source_ml_source2_spread_field1172 = (before.clone()).3;
                                                            {
                                                                let source_ml_source2_spread_field1173 = (before.clone()).4;
                                                                {
                                                                    let source_ml_source2_spread_field1174 = (before.clone()).5;
                                                                    {
                                                                        let source_ml_source2_spread_field1175 = (before.clone()).6;
                                                                        {
                                                                            let source_ml_source2_spread_field1176 = (before.clone()).7;
                                                                            {
                                                                                let source_ml_source2_spread_field1177 = (before.clone()).8;
                                                                                {
                                                                                    let source_ml_source2_spread_field1178 = (before.clone()).9;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1179 = (before.clone()).10;
                                                                                        {
                                                                                            let source_ml_source2_spread_field1180 = (before.clone()).11;
                                                                                            {
                                                                                                let source_ml_source2_field1181 = true;
                                                                                                (source_ml_source2_spread_field1169, source_ml_source2_spread_field1170, source_ml_source2_spread_field1171, source_ml_source2_spread_field1172, value.clone(), source_ml_source2_field1181, source_ml_source2_spread_field1175.clone(), source_ml_source2_spread_field1176, source_ml_source2_spread_field1177.clone(), source_ml_source2_spread_field1178, source_ml_source2_spread_field1179, source_ml_source2_spread_field1180)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
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
                                                let source_ml_source2_spread_field1184 = (before.clone()).0;
                                                {
                                                    let source_ml_source2_spread_field1185 = (before.clone()).1;
                                                    {
                                                        let source_ml_source2_spread_field1186 = (before.clone()).2;
                                                        {
                                                            let source_ml_source2_spread_field1187 = (before.clone()).3;
                                                            {
                                                                let source_ml_source2_spread_field1188 = (before.clone()).4;
                                                                {
                                                                    let source_ml_source2_spread_field1189 = (before.clone()).5;
                                                                    {
                                                                        let source_ml_source2_spread_field1190 = (before.clone()).6;
                                                                        {
                                                                            let source_ml_source2_spread_field1191 = (before.clone()).7;
                                                                            {
                                                                                let source_ml_source2_spread_field1192 = (before.clone()).8;
                                                                                {
                                                                                    let source_ml_source2_spread_field1193 = (before.clone()).9;
                                                                                    {
                                                                                        let source_ml_source2_spread_field1194 = (before.clone()).10;
                                                                                        {
                                                                                            let source_ml_source2_spread_field1195 = (before.clone()).11;
                                                                                            {
                                                                                                let source_ml_source2_field1196 = true;
                                                                                                (source_ml_source2_spread_field1184, source_ml_source2_spread_field1185, source_ml_source2_spread_field1186, source_ml_source2_spread_field1187, source_ml_source2_spread_field1188.clone(), source_ml_source2_spread_field1189, value.clone(), source_ml_source2_field1196, source_ml_source2_spread_field1192.clone(), source_ml_source2_spread_field1193, source_ml_source2_spread_field1194, source_ml_source2_spread_field1195)
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value1200) => {
                    source_ml_source2_data_property_value1200
                }
            } {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 71, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 110, 101, 120, 116, 32, 115, 116, 101, 112]))
            } else {
                {
                    let current = state.clone();
                    {
                        let source_ml_source2_s1201 = self::source_ml_source1_temporary_step_java_script_descriptor_conversion_loop123(heap.clone(), current.clone());
                        match source_ml_source2_s1201.clone() {
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Done(current_2) => {
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
                            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(source_ml_source1_temporary_result123) => {
                                let source_ml_source1_temporary_result123 = *source_ml_source1_temporary_result123;
                                source_ml_source1_temporary_result123.clone()
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, _, source_ml_source2_data_property_value1212) => {
                    source_ml_source2_data_property_value1212
                }
            }) {
                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 71, 101, 116]))
            } else {
                {
                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                        self::JavaScriptSemanticCompletion::NormalCompletion(source_ml_source2_data_property_value1213) => {
                            let source_ml_source2_data_property_value1213 = *source_ml_source2_data_property_value1213;
                            source_ml_source2_data_property_value1213.clone()
                        }
                        self::JavaScriptSemanticCompletion::ThrowCompletion(source_ml_source2_data_property_value1214) => {
                            let source_ml_source2_data_property_value1214 = *source_ml_source2_data_property_value1214;
                            source_ml_source2_data_property_value1214.clone()
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
