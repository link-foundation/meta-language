// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextJoin1388;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1387Result;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1389Result;
use super::apply_big_integer_bitwise_operation;
use super::compare_java_script_number_big_integer;
use super::compare_java_script_string_big_integer_units;
use super::construct_java_script_semantic_text_units;
use super::dependency_function7;
use super::divide_java_script_big_integer;
use super::execute_java_script_call_number_bits;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::power_java_script_big_integer;
use super::read_java_script_call_integer_bit_code;
use super::source_ml_source1_temporary_execute_java_script_call_integer_radix_text_loop1387;
use super::source_ml_source1_temporary_execute_java_script_call_integer_radix_text_loop1389;
use super::source_ml_source1_temporary_execute_java_script_call_integer_wrapping_loop1386;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_integer_primitive(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 97,
            100, 105, 120, 32, 116, 101, 120, 116,
        ])))
    {
        self::execute_java_script_call_integer_radix_text(values.clone())
    } else {
        if (((*operation)
            == (self::ml_string::Text::from_units(vec![
                119, 114, 97, 112, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103,
                101, 114,
            ])))
            || ((*operation)
                == (self::ml_string::Text::from_units(vec![
                    119, 114, 97, 112, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105, 110,
                    116, 101, 103, 101, 114,
                ]))))
        {
            self::execute_java_script_call_integer_wrapping(operation, values.clone())
        } else {
            if operation.contains(&self::ml_string::Text::from_units(vec![
                110, 117, 109, 98, 101, 114, 32, 98, 105, 116, 115,
            ])) {
                self::execute_java_script_call_number_bits(operation, values.clone())
            } else {
                if (((*operation)
                    == (self::ml_string::Text::from_units(vec![
                        99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114, 32, 105,
                        110, 116, 101, 103, 101, 114,
                    ])))
                    || ((*operation)
                        == (self::ml_string::Text::from_units(vec![
                            99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32, 105, 110,
                            116, 101, 103, 101, 114,
                        ]))))
                {
                    self::execute_java_script_call_mixed_integer_comparison(
                        operation,
                        values.clone(),
                    )
                } else {
                    {
                        let first = ({
                            let source_ml_source2_array_read_index32620 = 0f64;
                            {
                                let ml_reference_array2040 = values.clone();
                                let ml_reference_position2040 =
                                    source_ml_source2_array_read_index32620;
                                let ml_reference_projection2040 = self::ml_array::optional_at(
                                    &*ml_reference_array2040.borrow(),
                                    self::ml_array::number_index(ml_reference_position2040),
                                );
                                ml_reference_projection2040
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let second = ({
                                let source_ml_source2_array_read_index32622 = 1f64;
                                {
                                    let ml_reference_array2041 = values.clone();
                                    let ml_reference_position2041 =
                                        source_ml_source2_array_read_index32622;
                                    let ml_reference_projection2041 = self::ml_array::optional_at(
                                        &*ml_reference_array2041.borrow(),
                                        self::ml_array::number_index(ml_reference_position2041),
                                    );
                                    ml_reference_projection2041
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| {
                                self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0))
                            });
                            match first.clone() {
                                self::JavaScriptSemanticValue::Integer(first_value) => {
                                    match second.clone() {
                                        self::JavaScriptSemanticValue::Integer(value) => {
                                            if ((((*operation)
                                                == (self::ml_string::Text::from_units(vec![
                                                    100, 105, 118, 105, 100, 101, 32, 105, 110,
                                                    116, 101, 103, 101, 114, 115,
                                                ])))
                                                || ((*operation)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        114, 101, 97, 100, 32, 105, 110, 116, 101,
                                                        103, 101, 114, 32, 114, 101, 109, 97, 105,
                                                        110, 100, 101, 114,
                                                    ]))))
                                                && (value == (self::ml::Big::from_i128(0))))
                                            {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                                    self::ml_string::Text::from_units(vec![
                                                        82, 97, 110, 103, 101, 69, 114, 114, 111,
                                                        114,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        68, 105, 118, 105, 115, 105, 111, 110, 32,
                                                        98, 121, 32, 122, 101, 114, 111,
                                                    ]),
                                                )
                                            } else {
                                                if (((*operation)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        114, 97, 105, 115, 101, 32, 105, 110, 116,
                                                        101, 103, 101, 114, 32, 112, 111, 119, 101,
                                                        114,
                                                    ])))
                                                    && (value < (self::ml::Big::from_i128(0))))
                                                {
                                                    self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![69, 120, 112, 111, 110, 101, 110, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101]))
                                                } else {
                                                    {
                                                        let left_width = self::dependency_function7(
                                                            first_value.clone(),
                                                        );
                                                        {
                                                            let right_width =
                                                                self::dependency_function7(
                                                                    value.clone(),
                                                                );
                                                            if ((left_width < (0f64))
                                                                || (right_width < (0f64)))
                                                            {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 105, 110, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                            } else {
                                                                if ((*operation) == (self::ml_string::Text::from_units(vec![99, 111, 109, 112, 97, 114, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (first_value < value) {
                                                                        (-1f64)
                                                                    } else {
                                                                        if (first_value > value) {
                                                                            1f64
                                                                        } else {
                                                                            0f64
                                                                        }
                                                                    })))
                                                                } else {
                                                                    if (((*operation) == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) && (((left_width + right_width)) > (4096f64))) {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 109, 117, 108, 116, 105, 112, 108, 105, 99, 97, 116, 105, 111, 110, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                    } else {
                                                                        if ((((((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) && (first_value != (self::ml::Big::from_i128(0)))) && (first_value != (self::ml::Big::from_i128(1)))) && (first_value != (self::ml::Big::from_i128(-1)))) && ((value.mul(&{
                                                                            let source_ml_source2_integer_result32626 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(left_width);
                                                                            if (((source_ml_source2_integer_result32626.len() as f64)) == (1f64)) {
                                                                                (self::ml_array::optional_at(&source_ml_source2_integer_result32626, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(left_width)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                            }
                                                                        })) > (self::ml::Big::from_i128(4096)))) {
                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 112, 111, 110, 101, 110, 116, 105, 97, 116, 105, 111, 110, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                        } else {
                                                                            {
                                                                                let source_ml_source2_s32631 = if (((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 114, 105, 103, 104, 116])))) {
                                                                                    {
                                                                                        let growing = if ((*operation) == (self::ml_string::Text::from_units(vec![115, 104, 105, 102, 116, 32, 105, 110, 116, 101, 103, 101, 114, 32, 98, 105, 116, 115, 32, 108, 101, 102, 116]))) {
                                                                                            (value >= (self::ml::Big::from_i128(0)))
                                                                                        } else {
                                                                                            (value < (self::ml::Big::from_i128(0)))
                                                                                        };
                                                                                        {
                                                                                            let count = if (value < (self::ml::Big::from_i128(0))) {
                                                                                                value.neg()
                                                                                            } else {
                                                                                                value.clone()
                                                                                            };
                                                                                            if (((first_value != (self::ml::Big::from_i128(0))) && growing) && (count > ({
                                                                                                let source_ml_source2_integer_operand32634 = (4096f64 - left_width);
                                                                                                {
                                                                                                    let source_ml_source2_integer_result32635 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand32634);
                                                                                                    if (((source_ml_source2_integer_result32635.len() as f64)) == (1f64)) {
                                                                                                        (self::ml_array::optional_at(&source_ml_source2_integer_result32635, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                                                    } else {
                                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (((self::ml_string::Text::from_units(vec![84, 104, 101, 32, 110, 117, 109, 98, 101, 114, 32])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_integer_operand32634)))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 99, 111, 110, 118, 101, 114, 116, 101, 100, 32, 116, 111, 32, 97, 32, 66, 105, 103, 73, 110, 116, 32, 98, 101, 99, 97, 117, 115, 101, 32, 105, 116, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 105, 110, 116, 101, 103, 101, 114]))).0, message_present: true }).raise()
                                                                                                    }
                                                                                                }
                                                                                            }))) {
                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 115, 104, 105, 102, 116, 32, 101, 115, 116, 105, 109, 97, 116, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 117, 116, 112, 117, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))))
                                                                                            } else {
                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384Next
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384Next
                                                                                };
                                                                                match source_ml_source2_s32631.clone() {
                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384Next => {
                                                                                        {
                                                                                            let code = self::read_java_script_call_integer_bit_code(operation);
                                                                                            {
                                                                                                let result = self::ml::Big::from_i128(0);
                                                                                                {
                                                                                                    let source_ml_source2_s32640 = if ((*operation) == (self::ml_string::Text::from_units(vec![97, 100, 100, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                        {
                                                                                                            let result_2 = first_value.add(&value);
                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Next(result_2.clone())
                                                                                                        }
                                                                                                    } else {
                                                                                                        if ((*operation) == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                            {
                                                                                                                let result_3 = first_value.sub(&value);
                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Next(result_3.clone())
                                                                                                            }
                                                                                                        } else {
                                                                                                            if ((*operation) == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                                {
                                                                                                                    let result_4 = first_value.mul(&value);
                                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Next(result_4.clone())
                                                                                                                }
                                                                                                            } else {
                                                                                                                if (((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 114, 101, 109, 97, 105, 110, 100, 101, 114])))) {
                                                                                                                    {
                                                                                                                        let result_5 = ({
                                                                                                                            let source_ml_source2_array_read_value32641 = self::divide_java_script_big_integer(first_value.clone(), value.clone());
                                                                                                                            {
                                                                                                                                let source_ml_source2_array_read_index32642 = if ((*operation) == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101, 32, 105, 110, 116, 101, 103, 101, 114, 115]))) {
                                                                                                                                    0f64
                                                                                                                                } else {
                                                                                                                                    1f64
                                                                                                                                };
                                                                                                                                { let ml_reference_array2042 = source_ml_source2_array_read_value32641.clone(); let ml_reference_position2042 = source_ml_source2_array_read_index32642; let ml_reference_projection2042 = self::ml_array::optional_at(&*ml_reference_array2042.borrow(), self::ml_array::number_index(ml_reference_position2042)); ml_reference_projection2042 }
                                                                                                                            }
                                                                                                                        }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Next(result_5.clone())
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    if ((*operation) == (self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 32, 105, 110, 116, 101, 103, 101, 114, 32, 112, 111, 119, 101, 114]))) {
                                                                                                                        {
                                                                                                                            let result_6 = ({
                                                                                                                                let source_ml_source2_array_read_value32643 = self::power_java_script_big_integer(first_value.clone(), value.clone());
                                                                                                                                {
                                                                                                                                    let source_ml_source2_array_read_index32644 = 0f64;
                                                                                                                                    { let ml_reference_array2043 = source_ml_source2_array_read_value32643.clone(); let ml_reference_position2043 = source_ml_source2_array_read_index32644; let ml_reference_projection2043 = self::ml_array::optional_at(&*ml_reference_array2043.borrow(), self::ml_array::number_index(ml_reference_position2043)); ml_reference_projection2043 }
                                                                                                                                }
                                                                                                                            }).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Next(result_6.clone())
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        if ((*operation) == (self::ml_string::Text::from_units(vec![110, 101, 103, 97, 116, 101, 32, 105, 110, 116, 101, 103, 101, 114]))) {
                                                                                                                            {
                                                                                                                                let result_7 = first_value.neg();
                                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Next(result_7.clone())
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            if (code >= (0f64)) {
                                                                                                                                {
                                                                                                                                    let result_8 = self::apply_big_integer_bitwise_operation(code, first_value.clone(), value.clone());
                                                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Next(result_8.clone())
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))))
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    };
                                                                                                    match source_ml_source2_s32640.clone() {
                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Next(result_9) => {
                                                                                                            if ((self::dependency_function7(result_9.clone())) < (0f64)) {
                                                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 111, 117, 116, 112, 117, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                                                                                            } else {
                                                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(result_9.clone())))
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1385Return(source_ml_source1_temporary_result1385) => {
                                                                                                            let source_ml_source1_temporary_result1385 = *source_ml_source1_temporary_result1385;
                                                                                                            source_ml_source1_temporary_result1385.clone()
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin1384Return(source_ml_source1_temporary_result1385_2) => {
                                                                                        let source_ml_source1_temporary_result1385_2 = *source_ml_source1_temporary_result1385_2;
                                                                                        source_ml_source1_temporary_result1385_2.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                            self::ml_string::Text::from_units(vec![
                                                105, 110, 116, 101, 103, 101, 114, 32, 111, 112,
                                                101, 114, 97, 116, 105, 111, 110, 115, 32, 114,
                                                101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117,
                                                97, 108, 32, 99, 111, 110, 118, 101, 114, 116, 101,
                                                100, 32, 66, 105, 103, 73, 110, 116, 32, 111, 112,
                                                101, 114, 97, 110, 100, 115,
                                            ]),
                                        ),
                                    }
                                }
                                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        105, 110, 116, 101, 103, 101, 114, 32, 111, 112, 101, 114,
                                        97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105,
                                        114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110,
                                        118, 101, 114, 116, 101, 100, 32, 66, 105, 103, 73, 110,
                                        116, 32, 111, 112, 101, 114, 97, 110, 100, 115,
                                    ]),
                                ),
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
pub fn execute_java_script_call_mixed_integer_comparison(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let first = ({
            let source_ml_source2_array_read_index32646 = 0f64;
            {
                let ml_reference_array2044 = values.clone();
                let ml_reference_position2044 = source_ml_source2_array_read_index32646;
                let ml_reference_projection2044 = self::ml_array::optional_at(
                    &*ml_reference_array2044.borrow(),
                    self::ml_array::number_index(ml_reference_position2044),
                );
                ml_reference_projection2044
            }
        })
        .clone()
        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let second = ({
                let source_ml_source2_array_read_index32648 = 1f64;
                {
                    let ml_reference_array2045 = values.clone();
                    let ml_reference_position2045 = source_ml_source2_array_read_index32648;
                    let ml_reference_projection2045 = self::ml_array::optional_at(
                        &*ml_reference_array2045.borrow(),
                        self::ml_array::number_index(ml_reference_position2045),
                    );
                    ml_reference_projection2045
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match second.clone() {
                self::JavaScriptSemanticValue::Integer(second_value) => {
                    if ((self::dependency_function7(second_value.clone())) < (0f64)) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                            self::ml_string::Text::from_units(vec![
                                109, 105, 120, 101, 100, 32, 99, 111, 109, 112, 97, 114, 105, 115,
                                111, 110, 32, 105, 110, 116, 101, 103, 101, 114, 32, 101, 120, 99,
                                101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99,
                                116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111,
                                114, 107, 32, 112, 114, 111, 102, 105, 108, 101,
                            ]),
                        )
                    } else {
                        if ((*operation)
                            == (self::ml_string::Text::from_units(vec![
                                99, 111, 109, 112, 97, 114, 101, 32, 110, 117, 109, 98, 101, 114,
                                32, 105, 110, 116, 101, 103, 101, 114,
                            ])))
                        {
                            match first.clone() {
                                self::JavaScriptSemanticValue::Number(value) => {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(
                                        self::JavaScriptSemanticValue::Number(
                                            self::compare_java_script_number_big_integer(
                                                value,
                                                second_value.clone(),
                                            ),
                                        ),
                                    ))
                                }
                                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        109, 105, 120, 101, 100, 32, 78, 117, 109, 98, 101, 114,
                                        32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32,
                                        114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115,
                                        32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101,
                                        114, 32, 111, 112, 101, 114, 97, 110, 100,
                                    ]),
                                ),
                            }
                        } else {
                            if ((*operation)
                                == (self::ml_string::Text::from_units(vec![
                                    99, 111, 109, 112, 97, 114, 101, 32, 116, 101, 120, 116, 32,
                                    105, 110, 116, 101, 103, 101, 114,
                                ])))
                            {
                                match first.clone() {
                                    self::JavaScriptSemanticValue::SourceText(_, text) => {
                                        if ((text.length()) > (4096f64)) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                        } else {
                                            self::execute_java_script_call_mixed_integer_comparison(operation, std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptSemanticValue::Text(self::construct_java_script_semantic_text_units(first.clone(), 4096f64)), second.clone()])))
                                        }
                                    }
                                    _ => match first.clone() {
                                        self::JavaScriptSemanticValue::Text(units) => {
                                            if (({
                                                let ml_reference_array2046 = units.clone();
                                                let ml_reference_projection2046 =
                                                    ml_reference_array2046.borrow().len() as f64;
                                                ml_reference_projection2046
                                            }) > (4096f64))
                                            {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![109, 105, 120, 101, 100, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 117, 110, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                            } else {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::compare_java_script_string_big_integer_units(units.clone(), second_value.clone()))))
                                            }
                                        }
                                        _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                            self::ml_string::Text::from_units(vec![
                                                109, 105, 120, 101, 100, 32, 116, 101, 120, 116,
                                                32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110,
                                                32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                                                105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                                85, 84, 70, 49, 54, 32, 111, 112, 101, 114, 97,
                                                110, 100,
                                            ]),
                                        ),
                                    },
                                }
                            } else {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101,
                                        114, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110,
                                        32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116,
                                        115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105,
                                        115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97,
                                        116, 105, 111, 110,
                                    ]),
                                )
                            }
                        }
                    }
                }
                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        109, 105, 120, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114, 32, 99,
                        111, 109, 112, 97, 114, 105, 115, 111, 110, 115, 32, 114, 101, 113, 117,
                        105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108,
                        32, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97, 110, 100,
                    ]),
                ),
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_integer_wrapping(
    operation: &self::ml_string::Text,
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let width = ({
            let source_ml_source2_array_read_index32651 = 0f64;
            {
                let ml_reference_array2047 = values.clone();
                let ml_reference_position2047 = source_ml_source2_array_read_index32651;
                let ml_reference_projection2047 = self::ml_array::optional_at(
                    &*ml_reference_array2047.borrow(),
                    self::ml_array::number_index(ml_reference_position2047),
                );
                ml_reference_projection2047
            }
        })
        .clone()
        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let input = ({
                let source_ml_source2_array_read_index32653 = 1f64;
                {
                    let ml_reference_array2048 = values.clone();
                    let ml_reference_position2048 = source_ml_source2_array_read_index32653;
                    let ml_reference_projection2048 = self::ml_array::optional_at(
                        &*ml_reference_array2048.borrow(),
                        self::ml_array::number_index(ml_reference_position2048),
                    );
                    ml_reference_projection2048
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            if !(self::ml_array::includes(
                &{
                    let ml_reference_value2049 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                        self::ml_string::Text::from_units(vec![
                            119, 114, 97, 112, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116,
                            101, 103, 101, 114,
                        ]),
                        self::ml_string::Text::from_units(vec![
                            119, 114, 97, 112, 32, 117, 110, 115, 105, 103, 110, 101, 100, 32, 105,
                            110, 116, 101, 103, 101, 114,
                        ]),
                    ])))
                    .borrow()
                    .clone();
                    ml_reference_value2049
                },
                operation,
                0f64,
                false,
                |a, b| a == b,
            )) {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        105, 110, 116, 101, 103, 101, 114, 32, 119, 114, 97, 112, 112, 105, 110,
                        103, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 105, 116, 115, 32, 99,
                        111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 73, 110, 100, 101,
                        120, 32, 97, 110, 100, 32, 84, 111, 66, 105, 103, 73, 110, 116, 32, 111,
                        112, 101, 114, 97, 110, 100, 115,
                    ]),
                )
            } else {
                match width.clone() {
                    self::JavaScriptSemanticValue::Number(bit_width) => {
                        if (!(self::ml_math::is_safe_integer(bit_width)) || (bit_width < (0f64))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    105, 110, 116, 101, 103, 101, 114, 32, 119, 114, 97, 112, 112,
                                    105, 110, 103, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32,
                                    105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100,
                                    32, 84, 111, 73, 110, 100, 101, 120, 32, 97, 110, 100, 32, 84,
                                    111, 66, 105, 103, 73, 110, 116, 32, 111, 112, 101, 114, 97,
                                    110, 100, 115,
                                ]),
                            )
                        } else {
                            match input.clone() {
                                self::JavaScriptSemanticValue::Integer(integer) => {
                                    if (bit_width == (0f64)) {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(
                                            Box::new(self::JavaScriptSemanticValue::Integer(
                                                self::ml::Big::from_i128(0),
                                            )),
                                        )
                                    } else {
                                        if ((bit_width > (4096f64))
                                            || ((self::dependency_function7(integer.clone()))
                                                < (0f64)))
                                        {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114, 32, 119, 114, 97, 112, 112, 105, 110, 103, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32, 119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101]))
                                        } else {
                                            {
                                                let modulus = self::ml::Big::from_i128(1);
                                                {
                                                    let bit = 0f64;
                                                    {
                                                        let modulus_2 = self::source_ml_source1_temporary_execute_java_script_call_integer_wrapping_loop1386(bit_width, modulus.clone(), bit);
                                                        {
                                                            let result = {
                                                                let source_ml_source2_integer_division32654 = self::ml_big_integer_conversion::divide_java_script_big_integer(integer.clone(),modulus_2.clone());
                                                                if (((source_ml_source2_integer_division32654.len() as f64)) == (2f64)) {
                                                                    (self::ml_array::optional_at(&source_ml_source2_integer_division32654, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                } else {
                                                                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![68, 105, 118, 105, 115, 105, 111, 110, 32, 98, 121, 32, 122, 101, 114, 111])).0, message_present: true }).raise()
                                                                }
                                                            };
                                                            {
                                                                let result_3 = if (result
                                                                    < (self::ml::Big::from_i128(0)))
                                                                {
                                                                    {
                                                                        let result_2 =
                                                                            result.add(&modulus_2);
                                                                        result_2.clone()
                                                                    }
                                                                } else {
                                                                    result.clone()
                                                                };
                                                                {
                                                                    let result_5 = if (((*operation) == (self::ml_string::Text::from_units(vec![119, 114, 97, 112, 32, 115, 105, 103, 110, 101, 100, 32, 105, 110, 116, 101, 103, 101, 114]))) && (result_3 >= (self::ml::divide(&modulus_2, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)))) {
                                                                        {
                                                                            let result_4 = result_3.sub(&modulus_2);
                                                                            result_4.clone()
                                                                        }
                                                                    } else {
                                                                        result_3.clone()
                                                                    };
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(result_5.clone())))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                                    self::ml_string::Text::from_units(vec![
                                        105, 110, 116, 101, 103, 101, 114, 32, 119, 114, 97, 112,
                                        112, 105, 110, 103, 32, 99, 111, 110, 115, 117, 109, 101,
                                        115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101,
                                        116, 101, 100, 32, 84, 111, 73, 110, 100, 101, 120, 32, 97,
                                        110, 100, 32, 84, 111, 66, 105, 103, 73, 110, 116, 32, 111,
                                        112, 101, 114, 97, 110, 100, 115,
                                    ]),
                                ),
                            }
                        }
                    }
                    _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                        self::ml_string::Text::from_units(vec![
                            105, 110, 116, 101, 103, 101, 114, 32, 119, 114, 97, 112, 112, 105,
                            110, 103, 32, 99, 111, 110, 115, 117, 109, 101, 115, 32, 105, 116, 115,
                            32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 84, 111, 73, 110,
                            100, 101, 120, 32, 97, 110, 100, 32, 84, 111, 66, 105, 103, 73, 110,
                            116, 32, 111, 112, 101, 114, 97, 110, 100, 115,
                        ]),
                    ),
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_integer_radix_text(
    values: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallPrimitiveResult {
    {
        let input = ({
            let source_ml_source2_array_read_index32656 = 0f64;
            {
                let ml_reference_array2050 = values.clone();
                let ml_reference_position2050 = source_ml_source2_array_read_index32656;
                let ml_reference_projection2050 = self::ml_array::optional_at(
                    &*ml_reference_array2050.borrow(),
                    self::ml_array::number_index(ml_reference_position2050),
                );
                ml_reference_projection2050
            }
        })
        .clone()
        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        {
            let radix = ({
                let source_ml_source2_array_read_index32658 = 1f64;
                {
                    let ml_reference_array2051 = values.clone();
                    let ml_reference_position2051 = source_ml_source2_array_read_index32658;
                    let ml_reference_projection2051 = self::ml_array::optional_at(
                        &*ml_reference_array2051.borrow(),
                        self::ml_array::number_index(ml_reference_position2051),
                    );
                    ml_reference_projection2051
                }
            })
            .clone()
            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            match input.clone() {
                self::JavaScriptSemanticValue::Integer(value) => match radix.clone() {
                    self::JavaScriptSemanticValue::Number(radix_value) => {
                        let base = if (radix_value.is_nan() || (radix_value == (0f64))) {
                            0f64
                        } else {
                            radix_value.trunc()
                        };
                        if ((base < (2f64)) || (base > (36f64))) {
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                self::ml_string::Text::from_units(vec![
                                    82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    116, 111, 83, 116, 114, 105, 110, 103, 40, 41, 32, 114, 97,
                                    100, 105, 120, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32,
                                    109, 117, 115, 116, 32, 98, 101, 32, 98, 101, 116, 119, 101,
                                    101, 110, 32, 50, 32, 97, 110, 100, 32, 51, 54,
                                ]),
                            )
                        } else {
                            if ((self::dependency_function7(value.clone())) < (0f64)) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                                    self::ml_string::Text::from_units(vec![
                                        105, 110, 116, 101, 103, 101, 114, 32, 114, 97, 100, 105,
                                        120, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101,
                                        100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99,
                                        116, 101, 100, 32, 52, 48, 57, 54, 45, 98, 105, 116, 32,
                                        119, 111, 114, 107, 32, 112, 114, 111, 102, 105, 108, 101,
                                    ]),
                                )
                            } else {
                                {
                                    let remaining = if (value < (self::ml::Big::from_i128(0))) {
                                        value.neg()
                                    } else {
                                        value.clone()
                                    };
                                    {
                                        let divisor = {
                                            let source_ml_source2_integer_result32660 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(base);
                                            if ((source_ml_source2_integer_result32660.len()
                                                as f64)
                                                == (1f64))
                                            {
                                                (self::ml_array::optional_at(
                                                    &source_ml_source2_integer_result32660,
                                                    self::ml_array::number_index(0f64),
                                                ))
                                                .clone()
                                                .unwrap_or_else(|| self::ml::Big::from_i128(0))
                                            } else {
                                                (self::ml_source_exception::SourceException {
                                                    class_name: String::from("RangeError"),
                                                    message_units:
                                                        (((self::ml_string::Text::from_units(
                                                            vec![
                                                                84, 104, 101, 32, 110, 117, 109,
                                                                98, 101, 114, 32,
                                                            ],
                                                        ))
                                                        .concat(&self::ml_string::Text::from_text(
                                                            &self::ml_number::js_number(base),
                                                        )))
                                                        .concat(&self::ml_string::Text::from_units(
                                                            vec![
                                                                32, 99, 97, 110, 110, 111, 116, 32,
                                                                98, 101, 32, 99, 111, 110, 118,
                                                                101, 114, 116, 101, 100, 32, 116,
                                                                111, 32, 97, 32, 66, 105, 103, 73,
                                                                110, 116, 32, 98, 101, 99, 97, 117,
                                                                115, 101, 32, 105, 116, 32, 105,
                                                                115, 32, 110, 111, 116, 32, 97,
                                                                110, 32, 105, 110, 116, 101, 103,
                                                                101, 114,
                                                            ],
                                                        )))
                                                        .0,
                                                    message_present: true,
                                                })
                                                .raise()
                                            }
                                        };
                                        {
                                            let reversed = std::rc::Rc::new(
                                                std::cell::RefCell::new(Vec::<f64>::new()),
                                            );
                                            {
                                                let source_ml_source2_s32661 = self::source_ml_source1_temporary_execute_java_script_call_integer_radix_text_loop1387(remaining.clone(), divisor.clone(), reversed.clone());
                                                match source_ml_source2_s32661.clone() {
                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1387Result::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1387Done => {
                                                                {
                                                                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                    {
                                                                        let source_ml_source2_s32662 = if (value < (self::ml::Big::from_i128(0))) {
                                                                            {
                                                                                let source_ml_source1_temporary_execute_java_script_call_integer_radix_text_ignored1391 = { let ml_method_owner2052 = units.clone(); let ml_method_argument2052_0 = 45f64; let ml_inserted = vec![ml_method_argument2052_0]; let mut ml_method_values2052 = ml_method_owner2052.borrow_mut(); let ml_length = ml_method_values2052.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values2052.extend(ml_inserted); ml_length as f64 };
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextJoin1388::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextJoin1388Next
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextJoin1388::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextJoin1388Next
                                                                        };
                                                                        match source_ml_source2_s32662.clone() {
                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextJoin1388::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextJoin1388Next => {
                                                                                {
                                                                                    let index = ({ let ml_reference_array2053 = reversed.clone(); let ml_reference_projection2053 = ml_reference_array2053.borrow().len() as f64; ml_reference_projection2053 } - 1f64);
                                                                                    {
                                                                                        let source_ml_source2_s32663 = self::source_ml_source1_temporary_execute_java_script_call_integer_radix_text_loop1389(reversed.clone(), units.clone(), index);
                                                                                        match source_ml_source2_s32663.clone() {
                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1389Result::MlSource1TemporaryExecuteJavaScriptCallIntegerRadixTextLoop1389Done => {
                                                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(units.clone())))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                        self::ml_string::Text::from_units(vec![
                            105, 110, 116, 101, 103, 101, 114, 32, 114, 97, 100, 105, 120, 32, 102,
                            111, 114, 109, 97, 116, 116, 105, 110, 103, 32, 114, 101, 113, 117,
                            105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                            32, 66, 105, 103, 73, 110, 116, 32, 97, 110, 100, 32, 99, 111, 109,
                            112, 108, 101, 116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32,
                            114, 97, 100, 105, 120,
                        ]),
                    ),
                },
                _ => self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(
                    self::ml_string::Text::from_units(vec![
                        105, 110, 116, 101, 103, 101, 114, 32, 114, 97, 100, 105, 120, 32, 102,
                        111, 114, 109, 97, 116, 116, 105, 110, 103, 32, 114, 101, 113, 117, 105,
                        114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66,
                        105, 103, 73, 110, 116, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101,
                        116, 101, 100, 32, 110, 117, 109, 101, 114, 105, 99, 32, 114, 97, 100, 105,
                        120,
                    ]),
                ),
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_repetition_length(length: f64, count: f64) -> f64 {
    {
        let integer = if count.is_nan() { 0f64 } else { count.trunc() };
        if ((integer < (0f64)) || (integer == (f64::INFINITY))) {
            (-1f64)
        } else {
            if ((!(self::ml_math::is_safe_integer(length)) || (length < (0f64)))
                || (length > (1048576f64)))
            {
                (-2f64)
            } else {
                if ((length == (0f64)) || (integer == (0f64))) {
                    0f64
                } else {
                    if (integer > ((1048576f64 / length).floor())) {
                        (-2f64)
                    } else {
                        (length * integer)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_padding_length(length: f64, maximum: f64, filler_length: f64) -> f64 {
    if (((((!(self::ml_math::is_safe_integer(length)) || (length < (0f64)))
        || (length > (1048576f64)))
        || !(self::ml_math::is_safe_integer(filler_length)))
        || (filler_length < (0f64)))
        || (filler_length > (1048576f64)))
    {
        (-2f64)
    } else {
        {
            let integer = if maximum.is_nan() {
                0f64
            } else {
                maximum.trunc()
            };
            {
                let target =
                    self::ml_math::min(9007199254740991f64, self::ml_math::max(0f64, integer));
                if ((target <= length) || (filler_length == (0f64))) {
                    length
                } else {
                    if (target > (1048576f64)) {
                        (-2f64)
                    } else {
                        target
                    }
                }
            }
        }
    }
}
