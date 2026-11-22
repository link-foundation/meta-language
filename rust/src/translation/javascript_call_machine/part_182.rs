// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Result;
use super::construct_java_script_call_primitive_type_error;
use super::count_java_script_leading_zero_bits;
use super::dependency_function3;
use super::execute_java_script_call_numeric_binary;
use super::execute_java_script_call_numeric_update;
use super::execute_java_script_call_primitive_addition;
use super::execute_java_script_call_primitive_scalar;
use super::execute_java_script_call_relational_comparison;
use super::execute_java_script_numeric_parsing;
use super::execute_java_script_string_method_primitive;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::multiply_java_script_signed_integer_bits;
use super::read_java_script_call_primitive_arity_error;
use super::read_java_script_numeric_parsing_arity;
use super::read_java_script_string_method_primitive_arity;
use super::round_java_script_binary32;
use super::source_ml_source1_temporary_execute_java_script_call_primitive_loop1402;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let arity = self::read_java_script_call_primitive_arity_error(operation, {
            let ml_reference_array2098 = values.clone();
            let ml_reference_projection2098 = ml_reference_array2098.borrow().len() as f64;
            ml_reference_projection2098
        });
        if (arity != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(arity.clone())
        } else {
            if ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    116, 104, 114, 111, 119, 32, 114, 101, 115, 116, 114, 105, 99, 116, 101, 100,
                    32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 97, 99, 99, 101, 115, 115,
                ])))
            {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        39, 99, 97, 108, 108, 101, 114, 39, 44, 32, 39, 99, 97, 108, 108, 101, 101,
                        39, 44, 32, 97, 110, 100, 32, 39, 97, 114, 103, 117, 109, 101, 110, 116,
                        115, 39, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 109, 97,
                        121, 32, 110, 111, 116, 32, 98, 101, 32, 97, 99, 99, 101, 115, 115, 101,
                        100, 32, 111, 110, 32, 115, 116, 114, 105, 99, 116, 32, 109, 111, 100, 101,
                        32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 111, 114, 32, 116, 104,
                        101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 111, 98, 106, 101,
                        99, 116, 115, 32, 102, 111, 114, 32, 99, 97, 108, 108, 115, 32, 116, 111,
                        32, 116, 104, 101, 109,
                    ]),
                )
            } else {
                {
                    let source_ml_source1_temporary_index1401 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source2_s32706 = self::source_ml_source1_temporary_execute_java_script_call_primitive_loop1402(values.clone(), source_ml_source1_temporary_index1401.clone());
                        match source_ml_source2_s32706.clone() {
                            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Done => {
                                if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 108, 101, 110, 103, 116, 104]))) {
                                    {
                                        let value = ({
                                            let source_ml_source2_array_read_index32708 = 0f64;
                                            { let ml_reference_array2099 = values.clone(); let ml_reference_position2099 = source_ml_source2_array_read_index32708; let ml_reference_projection2099 = self::ml_array::optional_at(&*ml_reference_array2099.borrow(), self::ml_array::number_index(ml_reference_position2099)); ml_reference_projection2099 }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                        match value.clone() {
                                            self::JavaScriptSemanticValue::Number(value_value) => {
                                                if ((!(self::ml_math::is_safe_integer(value_value)) || (value_value < (0f64))) || {
                                                    let source_ml_source2_same_value_operand32710 = (-0f64);
                                                    (((value_value == source_ml_source2_same_value_operand32710) && ((value_value != (0f64)) || (((1f64 / value_value)) == ((1f64 / source_ml_source2_same_value_operand32710))))) || ((value_value != value_value) && (source_ml_source2_same_value_operand32710 != source_ml_source2_same_value_operand32710)))
                                                }) {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101, 32, 108, 101, 110, 103, 116, 104, 32, 102, 111, 108, 108, 111, 119, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104]))
                                                } else {
                                                    if (value_value > (4294967295f64)) {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]))
                                                    } else {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101, 32, 108, 101, 110, 103, 116, 104, 32, 102, 111, 108, 108, 111, 119, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104]))
                                            }
                                        }
                                    }
                                } else {
                                    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 99, 104, 101, 99, 107, 101, 100, 32, 97, 114, 114, 97, 121, 32, 114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 105, 110, 100, 101, 120]))) {
                                        {
                                            let length = ({
                                                let source_ml_source2_array_read_index32712 = 0f64;
                                                { let ml_reference_array2100 = values.clone(); let ml_reference_position2100 = source_ml_source2_array_read_index32712; let ml_reference_projection2100 = self::ml_array::optional_at(&*ml_reference_array2100.borrow(), self::ml_array::number_index(ml_reference_position2100)); ml_reference_projection2100 }
                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                            {
                                                let value_2 = ({
                                                    let source_ml_source2_array_read_index32714 = 1f64;
                                                    { let ml_reference_array2101 = values.clone(); let ml_reference_position2101 = source_ml_source2_array_read_index32714; let ml_reference_projection2101 = self::ml_array::optional_at(&*ml_reference_array2101.borrow(), self::ml_array::number_index(ml_reference_position2101)); ml_reference_projection2101 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                match length.clone() {
                                                    self::JavaScriptSemanticValue::Number(length_value) => {
                                                        if ((!(self::ml_math::is_safe_integer(length_value)) || (length_value < (0f64))) || {
                                                            let source_ml_source2_same_value_operand32716 = (-0f64);
                                                            (((length_value == source_ml_source2_same_value_operand32716) && ((length_value != (0f64)) || (((1f64 / length_value)) == ((1f64 / source_ml_source2_same_value_operand32716))))) || ((length_value != length_value) && (source_ml_source2_same_value_operand32716 != source_ml_source2_same_value_operand32716)))
                                                        }) {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 105, 110, 100, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                        } else {
                                                            match value_2.clone() {
                                                                self::JavaScriptSemanticValue::Number(value_value_2) => {
                                                                    {
                                                                        let integer = if (value_value_2.is_nan() || (value_value_2 == (0f64))) {
                                                                            0f64
                                                                        } else {
                                                                            value_value_2.trunc()
                                                                        };
                                                                        {
                                                                            let index = if (integer < (0f64)) {
                                                                                (length_value + integer)
                                                                            } else {
                                                                                integer
                                                                            };
                                                                            if ((index < (0f64)) || (index >= length_value)) {
                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![73, 110, 118, 97, 108, 105, 100, 32, 105, 110, 100, 101, 120]))
                                                                            } else {
                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (index == (0f64)) {
                                                                                    0f64
                                                                                } else {
                                                                                    index
                                                                                })))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 105, 110, 100, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![114, 101, 112, 108, 97, 99, 101, 109, 101, 110, 116, 32, 105, 110, 100, 105, 99, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 99, 111, 117, 110, 116]))) {
                                            {
                                                let maximum = ({
                                                    let source_ml_source2_array_read_index32718 = 0f64;
                                                    { let ml_reference_array2102 = values.clone(); let ml_reference_position2102 = source_ml_source2_array_read_index32718; let ml_reference_projection2102 = self::ml_array::optional_at(&*ml_reference_array2102.borrow(), self::ml_array::number_index(ml_reference_position2102)); ml_reference_projection2102 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                {
                                                    let value_3 = ({
                                                        let source_ml_source2_array_read_index32720 = 1f64;
                                                        { let ml_reference_array2103 = values.clone(); let ml_reference_position2103 = source_ml_source2_array_read_index32720; let ml_reference_projection2103 = self::ml_array::optional_at(&*ml_reference_array2103.borrow(), self::ml_array::number_index(ml_reference_position2103)); ml_reference_projection2103 }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    match maximum.clone() {
                                                        self::JavaScriptSemanticValue::Number(maximum_value) => {
                                                            if ((!(self::ml_math::is_safe_integer(maximum_value)) || (maximum_value < (0f64))) || {
                                                                let source_ml_source2_same_value_operand32722 = (-0f64);
                                                                (((maximum_value == source_ml_source2_same_value_operand32722) && ((maximum_value != (0f64)) || (((1f64 / maximum_value)) == ((1f64 / source_ml_source2_same_value_operand32722))))) || ((maximum_value != maximum_value) && (source_ml_source2_same_value_operand32722 != source_ml_source2_same_value_operand32722)))
                                                            }) {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 99, 111, 117, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 114, 101, 109, 97, 105, 110, 105, 110, 103, 32, 108, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                            } else {
                                                                match value_3.clone() {
                                                                    self::JavaScriptSemanticValue::Number(value_value_3) => {
                                                                        {
                                                                            let integer_2 = if (value_value_3.is_nan() || (value_value_3 == (0f64))) {
                                                                                0f64
                                                                            } else {
                                                                                value_value_3.trunc()
                                                                            };
                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::min(self::ml_math::max(integer_2, 0f64), maximum_value))))
                                                                        }
                                                                    }
                                                                    _ => {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 99, 111, 117, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 114, 101, 109, 97, 105, 110, 105, 110, 103, 32, 108, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        _ => {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 100, 101, 108, 101, 116, 105, 111, 110, 32, 99, 111, 117, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 114, 101, 109, 97, 105, 110, 105, 110, 103, 32, 108, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 78, 117, 109, 98, 101, 114]))
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 32, 83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) {
                                                self::dependency_function3(({
                                                    let source_ml_source2_array_read_index32724 = 0f64;
                                                    { let ml_reference_array2104 = values.clone(); let ml_reference_position2104 = source_ml_source2_array_read_index32724; let ml_reference_projection2104 = self::ml_array::optional_at(&*ml_reference_array2104.borrow(), self::ml_array::number_index(ml_reference_position2104)); ml_reference_projection2104 }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                            } else {
                                                if ((self::read_java_script_numeric_parsing_arity(operation)) >= (0f64)) {
                                                    self::execute_java_script_numeric_parsing(operation, values.clone())
                                                } else {
                                                    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 111, 117, 110, 100, 32, 98, 105, 110, 97, 114, 121, 51, 50, 32, 110, 117, 109, 98, 101, 114]))) {
                                                        {
                                                            let value_4 = ({
                                                                let source_ml_source2_array_read_index32726 = 0f64;
                                                                { let ml_reference_array2105 = values.clone(); let ml_reference_position2105 = source_ml_source2_array_read_index32726; let ml_reference_projection2105 = self::ml_array::optional_at(&*ml_reference_array2105.borrow(), self::ml_array::number_index(ml_reference_position2105)); ml_reference_projection2105 }
                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                            match value_4.clone() {
                                                                self::JavaScriptSemanticValue::Number(value_value_4) => {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::round_java_script_binary32(value_value_4))))
                                                                }
                                                                _ => {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121, 51, 50, 32, 114, 111, 117, 110, 100, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 117, 110, 116, 32, 108, 101, 97, 100, 105, 110, 103, 32, 122, 101, 114, 111, 32, 98, 105, 116, 115]))) {
                                                            {
                                                                let value_5 = ({
                                                                    let source_ml_source2_array_read_index32728 = 0f64;
                                                                    { let ml_reference_array2106 = values.clone(); let ml_reference_position2106 = source_ml_source2_array_read_index32728; let ml_reference_projection2106 = self::ml_array::optional_at(&*ml_reference_array2106.borrow(), self::ml_array::number_index(ml_reference_position2106)); ml_reference_projection2106 }
                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                match value_5.clone() {
                                                                    self::JavaScriptSemanticValue::Number(value_value_5) => {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::count_java_script_leading_zero_bits(value_value_5))))
                                                                    }
                                                                    _ => {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![108, 101, 97, 100, 105, 110, 103, 32, 122, 101, 114, 111, 32, 99, 111, 117, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if ((*operation) == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115]))) {
                                                                {
                                                                    let left = ({
                                                                        let source_ml_source2_array_read_index32730 = 0f64;
                                                                        { let ml_reference_array2107 = values.clone(); let ml_reference_position2107 = source_ml_source2_array_read_index32730; let ml_reference_projection2107 = self::ml_array::optional_at(&*ml_reference_array2107.borrow(), self::ml_array::number_index(ml_reference_position2107)); ml_reference_projection2107 }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                    {
                                                                        let right = ({
                                                                            let source_ml_source2_array_read_index32732 = 1f64;
                                                                            { let ml_reference_array2108 = values.clone(); let ml_reference_position2108 = source_ml_source2_array_read_index32732; let ml_reference_projection2108 = self::ml_array::optional_at(&*ml_reference_array2108.borrow(), self::ml_array::number_index(ml_reference_position2108)); ml_reference_projection2108 }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                        match left.clone() {
                                                                            self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                match right.clone() {
                                                                                    self::JavaScriptSemanticValue::Number(value_6) => {
                                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::multiply_java_script_signed_integer_bits(left_value, value_6))))
                                                                                    }
                                                                                    _ => {
                                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                    }
                                                                                }
                                                                            }
                                                                            _ => {
                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                if ((self::read_java_script_string_method_primitive_arity(operation)) >= (0f64)) {
                                                                    self::execute_java_script_string_method_primitive(operation, values.clone())
                                                                } else {
                                                                    if ((((*operation) == (self::ml_string::Text::from_units(vec![105, 110, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101]))) || ((*operation) == (self::ml_string::Text::from_units(vec![100, 101, 99, 114, 101, 109, 101, 110, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101])))) || ((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101])))) {
                                                                        self::execute_java_script_call_numeric_update(operation, ({
                                                                            let source_ml_source2_array_read_index32734 = 0f64;
                                                                            { let ml_reference_array2109 = values.clone(); let ml_reference_position2109 = source_ml_source2_array_read_index32734; let ml_reference_projection2109 = self::ml_array::optional_at(&*ml_reference_array2109.borrow(), self::ml_array::number_index(ml_reference_position2109)); ml_reference_projection2109 }
                                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                    } else {
                                                                        if self::ml_array::includes(&{ let ml_reference_value2110 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 111, 119, 101, 114])]))).borrow().clone(); ml_reference_value2110 }, operation, 0f64, false, |a, b| a == b) {
                                                                            self::execute_java_script_call_numeric_binary(operation, ({
                                                                                let source_ml_source2_array_read_index32736 = 0f64;
                                                                                { let ml_reference_array2111 = values.clone(); let ml_reference_position2111 = source_ml_source2_array_read_index32736; let ml_reference_projection2111 = self::ml_array::optional_at(&*ml_reference_array2111.borrow(), self::ml_array::number_index(ml_reference_position2111)); ml_reference_projection2111 }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                                                let source_ml_source2_array_read_index32738 = 1f64;
                                                                                { let ml_reference_array2112 = values.clone(); let ml_reference_position2112 = source_ml_source2_array_read_index32738; let ml_reference_projection2112 = self::ml_array::optional_at(&*ml_reference_array2112.borrow(), self::ml_array::number_index(ml_reference_position2112)); ml_reference_projection2112 }
                                                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                        } else {
                                                                            if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 114, 101, 108, 97, 116, 105, 111, 110, 97, 108, 32, 118, 97, 108, 117, 101, 115]))) {
                                                                                self::execute_java_script_call_relational_comparison(({
                                                                                    let source_ml_source2_array_read_index32740 = 0f64;
                                                                                    { let ml_reference_array2113 = values.clone(); let ml_reference_position2113 = source_ml_source2_array_read_index32740; let ml_reference_projection2113 = self::ml_array::optional_at(&*ml_reference_array2113.borrow(), self::ml_array::number_index(ml_reference_position2113)); ml_reference_projection2113 }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                                                    let source_ml_source2_array_read_index32742 = 1f64;
                                                                                    { let ml_reference_array2114 = values.clone(); let ml_reference_position2114 = source_ml_source2_array_read_index32742; let ml_reference_projection2114 = self::ml_array::optional_at(&*ml_reference_array2114.borrow(), self::ml_array::number_index(ml_reference_position2114)); ml_reference_projection2114 }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                            } else {
                                                                                if ((*operation) == (self::ml_string::Text::from_units(vec![97, 100, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 118, 97, 108, 117, 101, 115]))) {
                                                                                    self::execute_java_script_call_primitive_addition(({
                                                                                        let source_ml_source2_array_read_index32744 = 0f64;
                                                                                        { let ml_reference_array2115 = values.clone(); let ml_reference_position2115 = source_ml_source2_array_read_index32744; let ml_reference_projection2115 = self::ml_array::optional_at(&*ml_reference_array2115.borrow(), self::ml_array::number_index(ml_reference_position2115)); ml_reference_projection2115 }
                                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), ({
                                                                                        let source_ml_source2_array_read_index32746 = 1f64;
                                                                                        { let ml_reference_array2116 = values.clone(); let ml_reference_position2116 = source_ml_source2_array_read_index32746; let ml_reference_projection2116 = self::ml_array::optional_at(&*ml_reference_array2116.borrow(), self::ml_array::number_index(ml_reference_position2116)); ml_reference_projection2116 }
                                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                                } else {
                                                                                    self::execute_java_script_call_primitive_object_or_scalar(operation, values.clone())
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop1402Return(source_ml_source1_temporary_result1402) => {
                                let source_ml_source1_temporary_result1402 = *source_ml_source1_temporary_result1402;
                                source_ml_source1_temporary_result1402.clone()
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
pub fn execute_java_script_call_primitive_object_or_scalar(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    if (((*operation)
        == (self::ml_string::Text::from_units(vec![
            114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32, 109, 101, 116, 104,
            111, 100, 32, 111, 98, 106, 101, 99, 116,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32,
                112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 98, 106, 101, 99, 116,
            ]))))
    {
        {
            let value = ({
                let source_ml_source2_array_read_index32748 = 0f64;
                {
                    let ml_reference_array2117 = values.clone();
                    let ml_reference_position2117 = source_ml_source2_array_read_index32748;
                    let ml_reference_projection2117 = self::ml_array::optional_at(
                        &*ml_reference_array2117.borrow(),
                        self::ml_array::number_index(ml_reference_position2117),
                    );
                    ml_reference_projection2117
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            if match value.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => true,
                _ => false,
            } {
                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(value.clone()))
            } else {
                if (match value.clone() {
                    self::JavaScriptSemanticValue::Null => true,
                    _ => false,
                } || match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => true,
                    _ => false,
                }) {
                    self::JavaScriptCallPrimitiveResult::PrimitiveError(
                        self::ml_string::Text::from_units(vec![
                            84, 121, 112, 101, 69, 114, 114, 111, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101, 114, 116, 32,
                            117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 111, 114, 32, 110,
                            117, 108, 108, 32, 116, 111, 32, 111, 98, 106, 101, 99, 116,
                        ]),
                    )
                } else {
                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32,
                                109, 101, 116, 104, 111, 100, 32, 111, 98, 106, 101, 99, 116,
                            ])))
                        {
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 109, 101, 116, 104, 111, 100, 32, 114,
                                101, 99, 101, 105, 118, 101, 114, 32, 114, 101, 113, 117, 105, 114,
                                101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 98, 111, 120, 105,
                                110, 103, 32, 112, 114, 111, 116, 111, 99, 111, 108,
                            ])
                        } else {
                            self::ml_string::Text::from_units(vec![
                                111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101,
                                114, 116, 121, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114,
                                101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101,
                                32, 98, 111, 120, 105, 110, 103, 32, 112, 114, 111, 116, 111, 99,
                                111, 108,
                            ])
                        },
                    )
                }
            }
        }
    } else {
        if ((*operation)
            == (self::ml_string::Text::from_units(vec![
                114, 101, 113, 117, 105, 114, 101, 32, 86, 56, 32, 97, 114, 114, 97, 121, 32, 106,
                111, 105, 110, 32, 108, 101, 110, 103, 116, 104,
            ])))
        {
            {
                let value_2 = ({
                    let source_ml_source2_array_read_index32752 = 0f64;
                    {
                        let ml_reference_array2118 = values.clone();
                        let ml_reference_position2118 = source_ml_source2_array_read_index32752;
                        let ml_reference_projection2118 = self::ml_array::optional_at(
                            &*ml_reference_array2118.borrow(),
                            self::ml_array::number_index(ml_reference_position2118),
                        );
                        ml_reference_projection2118
                    }
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                match value_2.clone() {
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        if ((!(self::ml_math::is_safe_integer(value_value))
                            || (value_value < (0f64)))
                            || {
                                let source_ml_source2_same_value_operand32754 = (-0f64);
                                (((value_value == source_ml_source2_same_value_operand32754)
                                    && ((value_value != (0f64))
                                        || ((1f64 / value_value)
                                            == (1f64
                                                / source_ml_source2_same_value_operand32754))))
                                    || ((value_value != value_value)
                                        && (source_ml_source2_same_value_operand32754
                                            != source_ml_source2_same_value_operand32754)))
                            })
                        {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    86, 56, 32, 106, 111, 105, 110, 32, 108, 101, 110, 103, 116,
                                    104, 32, 102, 111, 108, 108, 111, 119, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 84, 111, 76, 101, 110, 103, 116, 104,
                                ]),
                            )
                        } else {
                            if (value_value > (4294967295f64)) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                    self::ml_string::Text::from_units(vec![
                                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                                    ]),
                                    self::ml_string::Text::from_units(vec![
                                        73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121,
                                        32, 108, 101, 110, 103, 116, 104,
                                    ]),
                                )
                            } else {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                    value_2.clone(),
                                ))
                            }
                        }
                    }
                    _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                        self::ml_string::Text::from_units(vec![
                            86, 56, 32, 106, 111, 105, 110, 32, 108, 101, 110, 103, 116, 104, 32,
                            102, 111, 108, 108, 111, 119, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                            84, 111, 76, 101, 110, 103, 116, 104,
                        ]),
                    ),
                }
            }
        } else {
            if ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    114, 101, 113, 117, 105, 114, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32,
                    116, 97, 114, 103, 101, 116,
                ])))
            {
                {
                    let value_3 = ({
                        let source_ml_source2_array_read_index32756 = 0f64;
                        {
                            let ml_reference_array2119 = values.clone();
                            let ml_reference_position2119 = source_ml_source2_array_read_index32756;
                            let ml_reference_projection2119 = self::ml_array::optional_at(
                                &*ml_reference_array2119.borrow(),
                                self::ml_array::number_index(ml_reference_position2119),
                            );
                            ml_reference_projection2119
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    {
                        let invalid = self::construct_java_script_call_primitive_type_error(
                            &(self::ml_string::Text::from_units(vec![
                                70, 117, 110, 99, 116, 105, 111, 110, 32, 105, 110, 118, 111, 99,
                                97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                                32, 97, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114,
                                103, 101, 116,
                            ])),
                        );
                        match value_3.clone() {
                            self::JavaScriptSemanticValue::Object(_, callable) => {
                                if callable {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                        value_3.clone(),
                                    ))
                                } else {
                                    invalid.clone()
                                }
                            }
                            _ => invalid.clone(),
                        }
                    }
                }
            } else {
                if ((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32, 108, 105,
                        107, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 79, 98, 106,
                        101, 99, 116,
                    ])))
                {
                    {
                        let value_4 = ({
                            let source_ml_source2_array_read_index32759 = 0f64;
                            {
                                let ml_reference_array2120 = values.clone();
                                let ml_reference_position2120 =
                                    source_ml_source2_array_read_index32759;
                                let ml_reference_projection2120 = self::ml_array::optional_at(
                                    &*ml_reference_array2120.borrow(),
                                    self::ml_array::number_index(ml_reference_position2120),
                                );
                                ml_reference_projection2120
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        if match value_4.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                value_4.clone(),
                            ))
                        } else {
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    67, 114, 101, 97, 116, 101, 76, 105, 115, 116, 70, 114, 111,
                                    109, 65, 114, 114, 97, 121, 76, 105, 107, 101, 32, 114, 101,
                                    113, 117, 105, 114, 101, 115, 32, 97, 110, 32, 79, 98, 106,
                                    101, 99, 116, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32,
                                    108, 105, 115, 116,
                                ]),
                            )
                        }
                    }
                } else {
                    if ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            114, 101, 113, 117, 105, 114, 101, 32, 79, 98, 106, 101, 99, 116, 32,
                            97, 115, 115, 105, 103, 110, 32, 116, 97, 114, 103, 101, 116,
                        ])))
                    {
                        {
                            let value_5 = ({
                                let source_ml_source2_array_read_index32763 = 0f64;
                                {
                                    let ml_reference_array2121 = values.clone();
                                    let ml_reference_position2121 =
                                        source_ml_source2_array_read_index32763;
                                    let ml_reference_projection2121 = self::ml_array::optional_at(
                                        &*ml_reference_array2121.borrow(),
                                        self::ml_array::number_index(ml_reference_position2121),
                                    );
                                    ml_reference_projection2121
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                            if match value_5.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => true,
                                _ => false,
                            } {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                    value_5.clone(),
                                ))
                            } else {
                                if (match value_5.clone() {
                                    self::JavaScriptSemanticValue::Null => true,
                                    _ => false,
                                } || match value_5.clone() {
                                    self::JavaScriptSemanticValue::Undefined => true,
                                    _ => false,
                                }) {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                        self::ml_string::Text::from_units(vec![
                                            84, 121, 112, 101, 69, 114, 114, 111, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            67, 97, 110, 110, 111, 116, 32, 99, 111, 110, 118, 101,
                                            114, 116, 32, 117, 110, 100, 101, 102, 105, 110, 101,
                                            100, 32, 111, 114, 32, 110, 117, 108, 108, 32, 116,
                                            111, 32, 111, 98, 106, 101, 99, 116,
                                        ]),
                                    )
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                        self::ml_string::Text::from_units(vec![
                                            79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103,
                                            110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101,
                                            32, 116, 97, 114, 103, 101, 116, 115, 32, 114, 101,
                                            113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108,
                                            32, 119, 114, 97, 112, 112, 101, 114, 32, 97, 108, 108,
                                            111, 99, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32,
                                            105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108,
                                            111, 116, 115,
                                        ]),
                                    )
                                }
                            }
                        }
                    } else {
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                114, 101, 113, 117, 105, 114, 101, 32, 79, 98, 106, 101, 99, 116,
                                32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 116, 97,
                                114, 103, 101, 116,
                            ])))
                        {
                            {
                                let value_6 = ({
                                    let source_ml_source2_array_read_index32767 = 0f64;
                                    {
                                        let ml_reference_array2122 = values.clone();
                                        let ml_reference_position2122 =
                                            source_ml_source2_array_read_index32767;
                                        let ml_reference_projection2122 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array2122.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position2122,
                                                ),
                                            );
                                        ml_reference_projection2122
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                if match value_6.clone() {
                                    self::JavaScriptSemanticValue::Object(_, _) => true,
                                    _ => false,
                                } {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                        value_6.clone(),
                                    ))
                                } else {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                        self::ml_string::Text::from_units(vec![
                                            84, 121, 112, 101, 69, 114, 114, 111, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            82, 101, 102, 108, 101, 99, 116, 32, 112, 114, 111,
                                            112, 101, 114, 116, 121, 32, 111, 112, 101, 114, 97,
                                            116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105,
                                            114, 101, 32, 97, 110, 32, 79, 98, 106, 101, 99, 116,
                                            32, 116, 97, 114, 103, 101, 116,
                                        ]),
                                    )
                                }
                            }
                        } else {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99,
                                    116, 32, 102, 111, 114, 32, 105, 110,
                                ])))
                            {
                                {
                                    let value_7 = ({
                                        let source_ml_source2_array_read_index32771 = 0f64;
                                        {
                                            let ml_reference_array2123 = values.clone();
                                            let ml_reference_position2123 =
                                                source_ml_source2_array_read_index32771;
                                            let ml_reference_projection2123 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array2123.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position2123,
                                                    ),
                                                );
                                            ml_reference_projection2123
                                        }
                                    })
                                    .clone()
                                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                    if match value_7.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => true,
                                        _ => false,
                                    } {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                            Box::new(value_7.clone()),
                                        )
                                    } else {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                            self::ml_string::Text::from_units(vec![
                                                84, 121, 112, 101, 69, 114, 114, 111, 114,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                84, 104, 101, 32, 114, 105, 103, 104, 116, 32, 111,
                                                112, 101, 114, 97, 110, 100, 32, 111, 102, 32, 105,
                                                110, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97,
                                                110, 32, 79, 98, 106, 101, 99, 116,
                                            ]),
                                        )
                                    }
                                }
                            } else {
                                if ((*operation)
                                    == (self::ml_string::Text::from_units(vec![
                                        114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 110,
                                        117, 108, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 110,
                                        112, 117, 116,
                                    ])))
                                {
                                    {
                                        let value_8 = ({
                                            let source_ml_source2_array_read_index32775 = 0f64;
                                            {
                                                let ml_reference_array2124 = values.clone();
                                                let ml_reference_position2124 =
                                                    source_ml_source2_array_read_index32775;
                                                let ml_reference_projection2124 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array2124.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position2124,
                                                        ),
                                                    );
                                                ml_reference_projection2124
                                            }
                                        })
                                        .clone()
                                        .unwrap_or_else(|| {
                                            self::JavaScriptSemanticValue::Undefined
                                        });
                                        if (match value_8.clone() {
                                            self::JavaScriptSemanticValue::Null => true,
                                            _ => false,
                                        } || match value_8.clone() {
                                            self::JavaScriptSemanticValue::Undefined => true,
                                            _ => false,
                                        }) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                                self::ml_string::Text::from_units(vec![
                                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    67, 97, 110, 110, 111, 116, 32, 99, 111, 110,
                                                    118, 101, 114, 116, 32, 117, 110, 100, 101,
                                                    102, 105, 110, 101, 100, 32, 111, 114, 32, 110,
                                                    117, 108, 108, 32, 116, 111, 32, 111, 98, 106,
                                                    101, 99, 116,
                                                ]),
                                            )
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                                Box::new(value_8.clone()),
                                            )
                                        }
                                    }
                                } else {
                                    self::execute_java_script_call_primitive_scalar(
                                        operation,
                                        values.clone(),
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
