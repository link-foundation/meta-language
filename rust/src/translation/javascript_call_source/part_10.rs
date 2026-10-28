// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryCodePointUnitsUtf16TextLoop80Result;
use super::MlSource1TemporaryCompareUtf16TextLoop73Result;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64;
use super::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65;
use super::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result;
use super::MlSource1TemporaryIndexUtf16TextLoop71Result;
use super::MlSource1TemporaryLastIndexUtf16TextLoop72Result;
use super::MlSource1TemporaryMatchUtf16TextLoop70Result;
use super::MlSource1TemporaryPlanUtf16TextSubstitutionLoop84Result;
use super::MlSource1TemporaryReplaceUtf16TextLoop85Result;
use super::MlSource1TemporarySplitUtf16TextLoop69Result;
use super::apply_big_integer_bitwise_operation;
use super::divide_java_script_big_integer;
use super::execute_java_script_call_number_bits;
use super::execute_java_script_call_text_pair;
use super::execute_java_script_call_text_position;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::power_java_script_big_integer;
use super::read_java_script_call_integer_arity;
use super::read_java_script_call_integer_bit_code;
use super::read_java_script_call_integer_width;
use super::read_java_script_utf8_string_units;
use super::source_ml_source1_temporary_character_units_utf16_text_loop79;
use super::source_ml_source1_temporary_code_point_units_utf16_text_loop80;
use super::source_ml_source1_temporary_compare_utf16_text_loop73;
use super::source_ml_source1_temporary_display_points_utf16_text_loop81;
use super::source_ml_source1_temporary_display_utf8_from_utf16_text_loop83;
use super::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop91;
use super::source_ml_source1_temporary_index_utf16_text_loop71;
use super::source_ml_source1_temporary_join_utf16_text_loop74;
use super::source_ml_source1_temporary_last_index_utf16_text_loop72;
use super::source_ml_source1_temporary_match_utf16_text_loop70;
use super::source_ml_source1_temporary_plan_utf16_text_substitution_loop84;
use super::source_ml_source1_temporary_points_utf16_text_loop67;
use super::source_ml_source1_temporary_replace_utf16_text_loop85;
use super::source_ml_source1_temporary_replace_utf16_text_loop86;
use super::source_ml_source1_temporary_slice_utf16_text_loop66;
use super::source_ml_source1_temporary_split_utf16_text_loop68;
use super::source_ml_source1_temporary_split_utf16_text_loop69;
use super::source_ml_source1_temporary_trim_utf16_text_loop77;
use super::source_ml_source1_temporary_trim_utf16_text_loop78;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_integer_primitive(operation: &str, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    if operation.contains("number bits") {
        self::execute_java_script_call_number_bits(operation, values)
    } else {
        {
            let first = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
            {
                let second = (self::ml_array::optional_at(&values, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Integer(self::ml::Big::from_i128(0)));
                match first.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f186) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f187) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                    }
                    self::JavaScriptSemanticValue::Integer(first_value) => {
                        match second.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f192) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source2_f193) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                            }
                            self::JavaScriptSemanticValue::Integer(value) => {
                                {
                                    let left = first_value.clone();
                                    {
                                        let right = value.clone();
                                        if (((operation == "divide integers") || (operation == "read integer remainder")) && (right == (self::ml::Big::from_i128(0)))) {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveError(String::from("RangeError"), String::from("Division by zero"))
                                        } else {
                                            if ((operation == "raise integer power") && (right < (self::ml::Big::from_i128(0)))) {
                                                self::JavaScriptCallPrimitiveResult::PrimitiveError(String::from("RangeError"), String::from("Exponent must be positive"))
                                            } else {
                                                {
                                                    let left_width = self::read_java_script_call_integer_width(left.clone());
                                                    {
                                                        let right_width = self::read_java_script_call_integer_width(right.clone());
                                                        if ((left_width < (0f64)) || (right_width < (0f64))) {
                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(String::from("integer input exceeds the selected 4096-bit work profile"))
                                                        } else {
                                                            if (operation == "compare integers") {
                                                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(if (left < right) {
                                                                    (-1f64)
                                                                } else {
                                                                    if (left > right) {
                                                                        1f64
                                                                    } else {
                                                                        0f64
                                                                    }
                                                                })))
                                                            } else {
                                                                if ((operation == "multiply integers") && (((left_width + right_width)) > (4096f64))) {
                                                                    self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(String::from("integer multiplication estimate exceeds the selected output work profile"))
                                                                } else {
                                                                    if (((((operation == "raise integer power") && (left != (self::ml::Big::from_i128(0)))) && (left != (self::ml::Big::from_i128(1)))) && (left != (self::ml::Big::from_i128(-1)))) && ((right.mul(&{
                                                                        let source_ml_source2_integer_operand200 = left_width;
                                                                        {
                                                                            let source_ml_source2_integer_result201 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand200);
                                                                            if (((source_ml_source2_integer_result201.len() as f64)) == (1f64)) {
                                                                                (self::ml_array::optional_at(&source_ml_source2_integer_result201, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (format!("{}{}", (format!("{}{}", (String::from("The number ")), (self::ml_number::js_number(source_ml_source2_integer_operand200)))), (String::from(" cannot be converted to a BigInt because it is not an integer")))).encode_utf16().collect(), message_present: true }).raise()
                                                                            }
                                                                        }
                                                                    })) > (self::ml::Big::from_i128(4096)))) {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(String::from("integer exponentiation estimate exceeds the selected output work profile"))
                                                                    } else {
                                                                        {
                                                                            let source_ml_source2_s206 = if ((operation == "shift integer bits left") || (operation == "shift integer bits right")) {
                                                                                {
                                                                                    let growing = if (operation == "shift integer bits left") {
                                                                                        (right >= (self::ml::Big::from_i128(0)))
                                                                                    } else {
                                                                                        (right < (self::ml::Big::from_i128(0)))
                                                                                    };
                                                                                    {
                                                                                        let count = if (right < (self::ml::Big::from_i128(0))) {
                                                                                            right.neg()
                                                                                        } else {
                                                                                            right.clone()
                                                                                        };
                                                                                        if (((left != (self::ml::Big::from_i128(0))) && growing) && (count > ({
                                                                                            let source_ml_source2_integer_operand209 = (4096f64 - left_width);
                                                                                            {
                                                                                                let source_ml_source2_integer_result210 = self::ml_big_integer_conversion::convert_java_script_number_big_integer(source_ml_source2_integer_operand209);
                                                                                                if (((source_ml_source2_integer_result210.len() as f64)) == (1f64)) {
                                                                                                    (self::ml_array::optional_at(&source_ml_source2_integer_result210, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0))
                                                                                                } else {
                                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (format!("{}{}", (format!("{}{}", (String::from("The number ")), (self::ml_number::js_number(source_ml_source2_integer_operand209)))), (String::from(" cannot be converted to a BigInt because it is not an integer")))).encode_utf16().collect(), message_present: true }).raise()
                                                                                                }
                                                                                            }
                                                                                        }))) {
                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(String::from("integer shift estimate exceeds the selected output work profile"))))
                                                                                        } else {
                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next
                                                                            };
                                                                            match source_ml_source2_s206.clone() {
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Next => {
                                                                                    {
                                                                                        let code = self::read_java_script_call_integer_bit_code(operation);
                                                                                        {
                                                                                            let result = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source2_s211 = if (operation == "add integers") {
                                                                                                    {
                                                                                                        let result_2 = left.add(&right);
                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Next(result_2.clone())
                                                                                                    }
                                                                                                } else {
                                                                                                    if (operation == "subtract integers") {
                                                                                                        {
                                                                                                            let result_3 = left.sub(&right);
                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Next(result_3.clone())
                                                                                                        }
                                                                                                    } else {
                                                                                                        if (operation == "multiply integers") {
                                                                                                            {
                                                                                                                let result_4 = left.mul(&right);
                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Next(result_4.clone())
                                                                                                            }
                                                                                                        } else {
                                                                                                            if ((operation == "divide integers") || (operation == "read integer remainder")) {
                                                                                                                {
                                                                                                                    let result_5 = (self::ml_array::optional_at(&self::divide_java_script_big_integer(left.clone(), right.clone()), self::ml_array::number_index(if (operation == "divide integers") {
                                                                                                                        0f64
                                                                                                                    } else {
                                                                                                                        1f64
                                                                                                                    }))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Next(result_5.clone())
                                                                                                                }
                                                                                                            } else {
                                                                                                                if (operation == "raise integer power") {
                                                                                                                    {
                                                                                                                        let result_6 = (self::ml_array::optional_at(&self::power_java_script_big_integer(left.clone(), right.clone()), self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0));
                                                                                                                        self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Next(result_6.clone())
                                                                                                                    }
                                                                                                                } else {
                                                                                                                    if (operation == "negate integer") {
                                                                                                                        {
                                                                                                                            let result_7 = left.neg();
                                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Next(result_7.clone())
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        if (code >= (0f64)) {
                                                                                                                            {
                                                                                                                                let result_8 = self::apply_big_integer_bitwise_operation(code, left.clone(), right.clone());
                                                                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Next(result_8.clone())
                                                                                                                            }
                                                                                                                        } else {
                                                                                                                            self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer primitives require their actual registered operation"))))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                };
                                                                                                match source_ml_source2_s211.clone() {
                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Next(result_9) => {
                                                                                                        if ((self::read_java_script_call_integer_width(result_9.clone())) < (0f64)) {
                                                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(String::from("integer output exceeds the selected 4096-bit work profile"))
                                                                                                        } else {
                                                                                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Integer(result_9.clone())))
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin65Return(source_ml_source1_temporary_result65) => {
                                                                                                        let source_ml_source1_temporary_result65 = *source_ml_source1_temporary_result65;
                                                                                                        source_ml_source1_temporary_result65.clone()
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64::MlSource1TemporaryExecuteJavaScriptCallIntegerPrimitiveJoin64Return(source_ml_source1_temporary_result65_2) => {
                                                                                    let source_ml_source1_temporary_result65_2 = *source_ml_source1_temporary_result65_2;
                                                                                    source_ml_source1_temporary_result65_2.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source2_f194) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f195) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                            }
                            self::JavaScriptSemanticValue::Object(source_ml_source2_f196, source_ml_source2_f197) => {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f188) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f189) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f190, source_ml_source2_f191) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("integer operations require actual converted BigInt operands"))
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn dependency_function2(value: f64) -> f64 {
    if (!(value.is_finite()) || (value == (0f64))) {
        0f64
    } else {
        {
            let integral = if (value < (0f64)) {
                (-(((-value)).floor()))
            } else {
                value.floor()
            };
            {
                let remainder = (integral % 4294967296f64);
                if (remainder < (0f64)) {
                    (remainder + 4294967296f64)
                } else {
                    if (remainder == (0f64)) {
                        0f64
                    } else {
                        remainder
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_integer(value: f64) -> f64 {
    if value.is_nan() {
        0f64
    } else {
        value.trunc()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf16_text_slice_position(value: f64, length: f64) -> f64 {
    {
        let integer = self::read_utf16_text_integer(value);
        if (integer < (0f64)) {
            self::ml_math::max((length + integer), 0f64)
        } else {
            self::ml_math::min(integer, length)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn slice_utf16_text(text: &[f64], start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::read_utf16_text_slice_position(start, (text.len() as f64));
        {
            let last = self::read_utf16_text_slice_position(end, (text.len() as f64));
            {
                let result = Vec::<f64>::new();
                {
                    let at = first;
                    {
                        let result_2 = self::source_ml_source1_temporary_slice_utf16_text_loop66(text, last, result, at);
                        result_2
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn substring_utf16_text(text: &[f64], start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(start), 0f64), (text.len() as f64));
        {
            let last = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(end), 0f64), (text.len() as f64));
            self::slice_utf16_text(text, self::ml_math::min(first, last), self::ml_math::max(first, last))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_utf16_text(text: &[f64], position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn relative_character_utf16_text(text: &[f64], position: f64) -> Vec<Vec<f64>> {
    {
        let integer = self::read_utf16_text_integer(position);
        {
            let at = if (integer < (0f64)) {
                ((text.len() as f64) + integer)
            } else {
                integer
            };
            if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
                Vec::<Vec<f64>>::new()
            } else {
                vec![vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn code_point_utf16_text(text: &[f64], position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    vec![if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        (((65536f64 + ((first - 55296f64) * 1024f64)) + second) - 56320f64)
                    } else {
                        first
                    }]
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn points_utf16_text(text: &[f64]) -> Vec<Vec<f64>> {
    {
        let result = Vec::<Vec<f64>>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_points_utf16_text_loop67(text, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn split_utf16_text(text: &[f64], separator: &[f64], limit: f64, missing: bool) -> Vec<Vec<f64>> {
    {
        let count = self::dependency_function2(limit);
        {
            let result = Vec::<Vec<f64>>::new();
            if (count == (0f64)) {
                result
            } else {
                if missing {
                    vec![self::slice_utf16_text(text, 0f64, (text.len() as f64))]
                } else {
                    if (((separator.len() as f64)) == (0f64)) {
                        {
                            let at = 0f64;
                            {
                                let result_2 = self::source_ml_source1_temporary_split_utf16_text_loop68(text, count, result, at);
                                result_2
                            }
                        }
                    } else {
                        {
                            let start = 0f64;
                            {
                                let source_ml_source2_s212 = self::source_ml_source1_temporary_split_utf16_text_loop69(text, separator, count, result, start);
                                match source_ml_source2_s212.clone() {
                                    self::MlSource1TemporarySplitUtf16TextLoop69Result::MlSource1TemporarySplitUtf16TextLoop69Done(result_3, start_2) => {
                                        {
                                            let result_4 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text, start_2, (text.len() as f64))], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                            result_4
                                        }
                                    }
                                    self::MlSource1TemporarySplitUtf16TextLoop69Result::MlSource1TemporarySplitUtf16TextLoop69Return(source_ml_source1_temporary_result69) => {
                                        source_ml_source1_temporary_result69
                                    }
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
pub fn match_utf16_text(text: &[f64], search: &[f64], start: f64) -> bool {
    if ((start < (0f64)) || (((start + (search.len() as f64))) > ((text.len() as f64)))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s213 = self::source_ml_source1_temporary_match_utf16_text_loop70(text, search, start, at);
                match source_ml_source2_s213.clone() {
                    self::MlSource1TemporaryMatchUtf16TextLoop70Result::MlSource1TemporaryMatchUtf16TextLoop70Done => {
                        true
                    }
                    self::MlSource1TemporaryMatchUtf16TextLoop70Result::MlSource1TemporaryMatchUtf16TextLoop70Return(source_ml_source1_temporary_result70) => {
                        source_ml_source1_temporary_result70
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn index_utf16_text(text: &[f64], search: &[f64], position: f64) -> f64 {
    {
        let start = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(position), 0f64), (text.len() as f64));
        {
            let at = start;
            {
                let source_ml_source2_s214 = self::source_ml_source1_temporary_index_utf16_text_loop71(text, search, at);
                match source_ml_source2_s214.clone() {
                    self::MlSource1TemporaryIndexUtf16TextLoop71Result::MlSource1TemporaryIndexUtf16TextLoop71Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryIndexUtf16TextLoop71Result::MlSource1TemporaryIndexUtf16TextLoop71Return(source_ml_source1_temporary_result71) => {
                        source_ml_source1_temporary_result71
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn last_index_utf16_text(text: &[f64], search: &[f64], position: f64) -> f64 {
    {
        let integer = if position.is_nan() {
            f64::INFINITY
        } else {
            self::read_utf16_text_integer(position)
        };
        {
            let start = self::ml_math::min(self::ml_math::max(integer, 0f64), ((text.len() as f64) - (search.len() as f64)));
            {
                let at = start;
                {
                    let source_ml_source2_s215 = self::source_ml_source1_temporary_last_index_utf16_text_loop72(text, search, at);
                    match source_ml_source2_s215.clone() {
                        self::MlSource1TemporaryLastIndexUtf16TextLoop72Result::MlSource1TemporaryLastIndexUtf16TextLoop72Done => {
                            (-1f64)
                        }
                        self::MlSource1TemporaryLastIndexUtf16TextLoop72Result::MlSource1TemporaryLastIndexUtf16TextLoop72Return(source_ml_source1_temporary_result72) => {
                            source_ml_source1_temporary_result72
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_utf16_text(left: &[f64], right: &[f64]) -> f64 {
    {
        let count = self::ml_math::min((left.len() as f64), (right.len() as f64));
        {
            let at = 0f64;
            {
                let source_ml_source2_s216 = self::source_ml_source1_temporary_compare_utf16_text_loop73(left, right, count, at);
                match source_ml_source2_s216.clone() {
                    self::MlSource1TemporaryCompareUtf16TextLoop73Result::MlSource1TemporaryCompareUtf16TextLoop73Done => {
                        if (((left.len() as f64)) == ((right.len() as f64))) {
                            0f64
                        } else {
                            if (((left.len() as f64)) < ((right.len() as f64))) {
                                (-1f64)
                            } else {
                                1f64
                            }
                        }
                    }
                    self::MlSource1TemporaryCompareUtf16TextLoop73Result::MlSource1TemporaryCompareUtf16TextLoop73Return(source_ml_source1_temporary_result73) => {
                        source_ml_source1_temporary_result73
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn starts_utf16_text(text: &[f64], search: &[f64]) -> bool {
    self::match_utf16_text(text, search, 0f64)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ends_utf16_text(text: &[f64], search: &[f64]) -> bool {
    self::match_utf16_text(text, search, ((text.len() as f64) - (search.len() as f64)))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn includes_utf16_text(text: &[f64], search: &[f64]) -> bool {
    ((self::index_utf16_text(text, search, 0f64)) >= (0f64))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn join_utf16_text(values: &[Vec<f64>], separator: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_join_utf16_text_loop74(values, separator, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn trim_utf16_text(text: &[f64], start: bool, end: bool) -> Vec<f64> {
    {
        let first = 0f64;
        {
            let last = (text.len() as f64);
            {
                let first_3 = if start {
                    {
                        let first_2 = self::source_ml_source1_temporary_trim_utf16_text_loop77(text, first, last);
                        first_2
                    }
                } else {
                    first
                };
                {
                    let last_3 = if end {
                        {
                            let last_2 = self::source_ml_source1_temporary_trim_utf16_text_loop78(text, first_3, last);
                            last_2
                        }
                    } else {
                        last
                    };
                    self::slice_utf16_text(text, first_3, last_3)
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn character_units_utf16_text(values: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_character_units_utf16_text_loop79(values, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn code_point_units_utf16_text(values: &[f64]) -> Vec<f64> {
    {
        let result = vec![(-1f64)];
        {
            let at = 0f64;
            {
                let source_ml_source2_s217 = self::source_ml_source1_temporary_code_point_units_utf16_text_loop80(values, result, at);
                match source_ml_source2_s217.clone() {
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop80Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop80Done(result_2) => {
                        result_2
                    }
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop80Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop80Return(source_ml_source1_temporary_result80) => {
                        source_ml_source1_temporary_result80
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_utf8_text_units(bytes: &[f64]) -> Vec<f64> {
    self::read_java_script_utf8_string_units(bytes.to_vec())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn display_points_utf16_text(text: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_display_points_utf16_text_loop81(text, result, at);
                result_2
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn display_utf8_from_utf16_text(text: &[f64]) -> Vec<f64> {
    {
        let bytes = Vec::<f64>::new();
        {
            let points = self::display_points_utf16_text(text);
            {
                let at = 0f64;
                {
                    let bytes_2 = self::source_ml_source1_temporary_display_utf8_from_utf16_text_loop83(bytes, &(points), at);
                    bytes_2
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_utf16_text_substitution(replacement: &[f64]) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let start = 0f64;
            {
                let at = 0f64;
                {
                    let source_ml_source2_s218 = self::source_ml_source1_temporary_plan_utf16_text_substitution_loop84(replacement, result, start, at);
                    match source_ml_source2_s218.clone() {
                        self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop84Result::MlSource1TemporaryPlanUtf16TextSubstitutionLoop84Done(result_2, start_2) => {
                            {
                                let result_4 = if (start_2 < ((replacement.len() as f64))) {
                                    {
                                        let result_3 = { let (ml_append_right, ml_append_left) = (vec![0f64, start_2, (replacement.len() as f64)], result_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                        result_3
                                    }
                                } else {
                                    result_2
                                };
                                result_4
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
pub fn replace_utf16_text(text: &[f64], search: &[f64], replacement: &[f64], all: bool) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let plan = self::plan_utf16_text_substitution(replacement);
            {
                let start = 0f64;
                {
                    let from = 0f64;
                    {
                        let source_ml_source2_s219 = self::source_ml_source1_temporary_replace_utf16_text_loop85(text, search, replacement, all, result, &(plan), start, from);
                        match source_ml_source2_s219.clone() {
                            self::MlSource1TemporaryReplaceUtf16TextLoop85Result::MlSource1TemporaryReplaceUtf16TextLoop85Done(result_2, start_2) => {
                                {
                                    let at = start_2;
                                    {
                                        let result_3 = self::source_ml_source1_temporary_replace_utf16_text_loop86(text, result_2, at);
                                        result_3
                                    }
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
pub fn read_java_script_call_primitive_arity_error(operation: &str, count: f64) -> String {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < (0f64))) || (count > (4096f64))) {
        String::from("primitive operands exceed their explicit call register budget")
    } else {
        {
            let integer_arity = self::read_java_script_call_integer_arity(operation);
            if (integer_arity != (0f64)) {
                if (count == integer_arity) {
                    String::from("")
                } else {
                    String::from("primitive instructions require every actual operand coordinate")
                }
            } else {
                if (((operation == "read minimum number") || (operation == "read maximum number")) || (operation == "construct text units")) {
                    String::from("")
                } else {
                    {
                        let unary = vec![String::from("truncate number"), String::from("read text length"), String::from("trim text"), String::from("trim text start"), String::from("trim text end")];
                        {
                            let binary = vec![String::from("read text code point"), String::from("read text character"), String::from("read relative text character"), String::from("concatenate text"), String::from("compare text"), String::from("test text prefix"), String::from("test text suffix"), String::from("test text inclusion")];
                            {
                                let ternary = vec![String::from("slice text"), String::from("find text"), String::from("find last text")];
                                {
                                    let arity = if self::ml_array::includes(&unary, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                        1f64
                                    } else {
                                        if self::ml_array::includes(&binary, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                            2f64
                                        } else {
                                            if self::ml_array::includes(&ternary, &operation.to_owned(), 0f64, false, |a, b| a == b) {
                                                3f64
                                            } else {
                                                (-1f64)
                                            }
                                        }
                                    };
                                    if (arity < (0f64)) {
                                        String::from("primitive instructions require their actual registered operation")
                                    } else {
                                        if (count != arity) {
                                            String::from("primitive instructions require every actual operand coordinate")
                                        } else {
                                            String::from("")
                                        }
                                    }
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
pub fn execute_java_script_call_numeric_primitive(operation: &str, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let numbers = Vec::<f64>::new();
        {
            let source_ml_source1_temporary_values90 = values;
            {
                let source_ml_source1_temporary_index90 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s220 = self::source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop91(numbers, source_ml_source1_temporary_values90, source_ml_source1_temporary_index90.clone());
                    match source_ml_source2_s220.clone() {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Done(numbers_2) => {
                            if (operation == "truncate number") {
                                self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(((self::ml_array::optional_at(&numbers_2, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| f64::NAN)).trunc())))
                            } else {
                                if (operation == "read minimum number") {
                                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::min_of(&self::ml_array::append(Vec::<f64>::new(), numbers_2)))))
                                } else {
                                    if (operation == "read maximum number") {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number(self::ml_math::max_of(&self::ml_array::append(Vec::<f64>::new(), numbers_2)))))
                                    } else {
                                        if (operation == "construct text units") {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::character_units_utf16_text(&(numbers_2)))))
                                        } else {
                                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("the numeric primitive executor needs its actual registered operation"))
                                        }
                                    }
                                }
                            }
                        }
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Return(source_ml_source1_temporary_result91) => {
                            let source_ml_source1_temporary_result91 = *source_ml_source1_temporary_result91;
                            source_ml_source1_temporary_result91.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_text_primitive(operation: &str, values: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    {
        let receiver = (self::ml_array::optional_at(&values, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
        match receiver.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Null => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f221) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f222) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f223) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Text(text) => {
                if (operation == "read text length") {
                    self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Number((text.len() as f64))))
                } else {
                    if (((operation == "trim text") || (operation == "trim text start")) || (operation == "trim text end")) {
                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(Box::new(self::JavaScriptSemanticValue::Text(self::trim_utf16_text(&(text), (operation != "trim text end"), (operation != "trim text start")))))
                    } else {
                        if ((((operation == "read text code point") || (operation == "read text character")) || (operation == "read relative text character")) || (operation == "slice text")) {
                            self::execute_java_script_call_text_position(operation, &(text), values)
                        } else {
                            self::execute_java_script_call_text_pair(operation, &(text), values)
                        }
                    }
                }
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f224) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
            self::JavaScriptSemanticValue::Object(source_ml_source2_f225, source_ml_source2_f226) => {
                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("text primitives require their actual complete UTF16 receiver"))
            }
        }
    }
}
