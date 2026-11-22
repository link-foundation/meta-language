pub fn slice_utf16_text(text: Vec<f64>, start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::read_utf16_text_slice_position(start, (text.len() as f64));
        {
            let last = self::read_utf16_text_slice_position(end, (text.len() as f64));
            {
                let result = Vec::<f64>::new();
                {
                    let at = first;
                    {
                        let result_2 = self::source_ml_source1_temporary_slice_utf16_text_loop38(text, last, result, at);
                        result_2
                    }
                }
            }
        }
    }
}

pub fn substring_utf16_text(text: Vec<f64>, start: f64, end: f64) -> Vec<f64> {
    {
        let first = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(start), 0f64), (text.len() as f64));
        {
            let last = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(end), 0f64), (text.len() as f64));
            self::slice_utf16_text(text, self::ml_math::min(first, last), self::ml_math::max(first, last))
        }
    }
}

pub fn character_utf16_text(text: Vec<f64>, position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]
        }
    }
}

pub fn relative_character_utf16_text(text: Vec<f64>, position: f64) -> Vec<Vec<f64>> {
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
                vec![vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]]
            }
        }
    }
}

pub fn code_point_utf16_text(text: Vec<f64>, position: f64) -> Vec<f64> {
    {
        let at = self::read_utf16_text_integer(position);
        if ((at < (0f64)) || (at >= ((text.len() as f64)))) {
            Vec::<f64>::new()
        } else {
            {
                let first = (self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(&text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
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

pub fn points_utf16_text(text: Vec<f64>) -> Vec<Vec<f64>> {
    {
        let result = Vec::<Vec<f64>>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_points_utf16_text_loop39(text, result, at);
                result_2
            }
        }
    }
}

pub fn split_utf16_text(text: Vec<f64>, separator: Vec<f64>, limit: f64, missing: bool) -> Vec<Vec<f64>> {
    {
        let count = self::normalize_java_script_unsigned_integer(limit);
        {
            let result = Vec::<Vec<f64>>::new();
            if (count == (0f64)) {
                result
            } else {
                if missing {
                    vec![self::slice_utf16_text(text.clone(), 0f64, (text.len() as f64))]
                } else {
                    if (((separator.len() as f64)) == (0f64)) {
                        {
                            let at = 0f64;
                            {
                                let result_2 = self::source_ml_source1_temporary_split_utf16_text_loop40(text, count, result, at);
                                result_2
                            }
                        }
                    } else {
                        {
                            let start = 0f64;
                            {
                                let source_ml_source2_s444 = self::source_ml_source1_temporary_split_utf16_text_loop41(text.clone(), separator, count, result, start);
                                match source_ml_source2_s444.clone() {
                                    self::MlSource1TemporarySplitUtf16TextLoop41Result::MlSource1TemporarySplitUtf16TextLoop41Done(result_3, start_2) => {
                                        {
                                            let result_4 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text.clone(), start_2, (text.len() as f64))], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                            result_4
                                        }
                                    }
                                    self::MlSource1TemporarySplitUtf16TextLoop41Result::MlSource1TemporarySplitUtf16TextLoop41Return(source_ml_source1_temporary_result41) => {
                                        source_ml_source1_temporary_result41
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn match_utf16_text(text: Vec<f64>, search: Vec<f64>, start: f64) -> bool {
    if ((start < (0f64)) || (((start + (search.len() as f64))) > ((text.len() as f64)))) {
        false
    } else {
        {
            let at = 0f64;
            {
                let source_ml_source2_s445 = self::source_ml_source1_temporary_match_utf16_text_loop42(text, search, start, at);
                match source_ml_source2_s445.clone() {
                    self::MlSource1TemporaryMatchUtf16TextLoop42Result::MlSource1TemporaryMatchUtf16TextLoop42Done => {
                        true
                    }
                    self::MlSource1TemporaryMatchUtf16TextLoop42Result::MlSource1TemporaryMatchUtf16TextLoop42Return(source_ml_source1_temporary_result42) => {
                        source_ml_source1_temporary_result42
                    }
                }
            }
        }
    }
}

pub fn index_utf16_text(text: Vec<f64>, search: Vec<f64>, position: f64) -> f64 {
    {
        let start = self::ml_math::min(self::ml_math::max(self::read_utf16_text_integer(position), 0f64), (text.len() as f64));
        {
            let at = start;
            {
                let source_ml_source2_s446 = self::source_ml_source1_temporary_index_utf16_text_loop43(text, search, at);
                match source_ml_source2_s446.clone() {
                    self::MlSource1TemporaryIndexUtf16TextLoop43Result::MlSource1TemporaryIndexUtf16TextLoop43Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryIndexUtf16TextLoop43Result::MlSource1TemporaryIndexUtf16TextLoop43Return(source_ml_source1_temporary_result43) => {
                        source_ml_source1_temporary_result43
                    }
                }
            }
        }
    }
}

pub fn last_index_utf16_text(text: Vec<f64>, search: Vec<f64>, position: f64) -> f64 {
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
                    let source_ml_source2_s447 = self::source_ml_source1_temporary_last_index_utf16_text_loop44(text, search, at);
                    match source_ml_source2_s447.clone() {
                        self::MlSource1TemporaryLastIndexUtf16TextLoop44Result::MlSource1TemporaryLastIndexUtf16TextLoop44Done => {
                            (-1f64)
                        }
                        self::MlSource1TemporaryLastIndexUtf16TextLoop44Result::MlSource1TemporaryLastIndexUtf16TextLoop44Return(source_ml_source1_temporary_result44) => {
                            source_ml_source1_temporary_result44
                        }
                    }
                }
            }
        }
    }
}

pub fn compare_utf16_text(left: Vec<f64>, right: Vec<f64>) -> f64 {
    {
        let count = self::ml_math::min((left.len() as f64), (right.len() as f64));
        {
            let at = 0f64;
            {
                let source_ml_source2_s448 = self::source_ml_source1_temporary_compare_utf16_text_loop45(left.clone(), right.clone(), count, at);
                match source_ml_source2_s448.clone() {
                    self::MlSource1TemporaryCompareUtf16TextLoop45Result::MlSource1TemporaryCompareUtf16TextLoop45Done => {
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
                    self::MlSource1TemporaryCompareUtf16TextLoop45Result::MlSource1TemporaryCompareUtf16TextLoop45Return(source_ml_source1_temporary_result45) => {
                        source_ml_source1_temporary_result45
                    }
                }
            }
        }
    }
}

pub fn starts_utf16_text(text: Vec<f64>, search: Vec<f64>) -> bool {
    self::match_utf16_text(text, search, 0f64)
}

pub fn ends_utf16_text(text: Vec<f64>, search: Vec<f64>) -> bool {
    self::match_utf16_text(text.clone(), search.clone(), ((text.len() as f64) - (search.len() as f64)))
}

pub fn includes_utf16_text(text: Vec<f64>, search: Vec<f64>) -> bool {
    ((self::index_utf16_text(text, search, 0f64)) >= (0f64))
}

pub fn join_utf16_text(values: Vec<Vec<f64>>, separator: Vec<f64>) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_join_utf16_text_loop46(values, separator, result, at);
                result_2
            }
        }
    }
}

pub fn trim_utf16_text(text: Vec<f64>, start: bool, end: bool) -> Vec<f64> {
    {
        let first = 0f64;
        {
            let last = (text.len() as f64);
            {
                let first_3 = if start {
                    {
                        let first_2 = self::source_ml_source1_temporary_trim_utf16_text_loop49(text.clone(), first, last);
                        first_2
                    }
                } else {
                    first
                };
                {
                    let last_3 = if end {
                        {
                            let last_2 = self::source_ml_source1_temporary_trim_utf16_text_loop50(text.clone(), first_3, last);
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

pub fn character_units_utf16_text(values: Vec<f64>) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_character_units_utf16_text_loop51(values, result, at);
                result_2
            }
        }
    }
}

pub fn code_point_units_utf16_text(values: Vec<f64>) -> Vec<f64> {
    {
        let result = vec![(-1f64)];
        {
            let at = 0f64;
            {
                let source_ml_source2_s449 = self::source_ml_source1_temporary_code_point_units_utf16_text_loop52(values, result, at);
                match source_ml_source2_s449.clone() {
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Done(result_2) => {
                        result_2
                    }
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Return(source_ml_source1_temporary_result52) => {
                        source_ml_source1_temporary_result52
                    }
                }
            }
        }
    }
}

pub fn read_utf8_text_units(bytes: Vec<f64>) -> Vec<f64> {
    self::read_java_script_utf8_string_units(bytes)
}

pub fn display_points_utf16_text(text: Vec<f64>) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let at = 0f64;
            {
                let result_2 = self::source_ml_source1_temporary_display_points_utf16_text_loop53(text, result, at);
                result_2
            }
        }
    }
}

pub fn display_utf8_from_utf16_text(text: Vec<f64>) -> Vec<f64> {
    {
        let bytes = Vec::<f64>::new();
        {
            let points = self::display_points_utf16_text(text);
            {
                let at = 0f64;
                {
                    let bytes_2 = self::source_ml_source1_temporary_display_utf8_from_utf16_text_loop55(bytes, points, at);
                    bytes_2
                }
            }
        }
    }
}

pub fn plan_utf16_text_substitution(replacement: Vec<f64>) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let start = 0f64;
            {
                let at = 0f64;
                {
                    let source_ml_source2_s450 = self::source_ml_source1_temporary_plan_utf16_text_substitution_loop56(replacement.clone(), result, start, at);
                    match source_ml_source2_s450.clone() {
                        self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Result::MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Done(result_2, start_2) => {
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

pub fn replace_utf16_text(text: Vec<f64>, search: Vec<f64>, replacement: Vec<f64>, all: bool) -> Vec<f64> {
    {
        let result = Vec::<f64>::new();
        {
            let plan = self::plan_utf16_text_substitution(replacement.clone());
            {
                let start = 0f64;
                {
                    let from = 0f64;
                    {
                        let searching = true;
                        {
                            let source_ml_source2_s451 = self::source_ml_source1_temporary_replace_utf16_text_loop57(text.clone(), search, replacement, all, result, plan, start, from, searching);
                            match source_ml_source2_s451.clone() {
                                self::MlSource1TemporaryReplaceUtf16TextLoop57Result::MlSource1TemporaryReplaceUtf16TextLoop57Done(result_2, start_2) => {
                                    {
                                        let at = start_2;
                                        {
                                            let result_3 = self::source_ml_source1_temporary_replace_utf16_text_loop58(text, result_2, at);
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
}

pub fn test_java_script_primitive_method_operation(operation: self::ml_string::Text) -> bool {
    self::test_java_script_standard_primitive_method_operation(operation.clone())
}

pub fn execute_java_script_primitive_method(receiver: self::JavaScriptSemanticValue, operation: self::ml_string::Text, args: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallPrimitiveResult {
    if !(self::test_java_script_primitive_method_operation(operation.clone())) {
        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114]))
    } else {
        if (((args.len() as f64)) > (4096f64)) {
            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 97, 108, 108, 32, 98, 117, 100, 103, 101, 116]))
        } else {
            if (match receiver.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    false
                }
                self::JavaScriptSemanticValue::Null => {
                    true
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f452) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f453) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f454) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f455) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f456, source_ml_source2_f457) => {
                    false
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f458) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f459, source_ml_source2_f460) => {
                    false
                }
            } || match receiver.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    true
                }
                self::JavaScriptSemanticValue::Null => {
                    false
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f461) => {
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f462) => {
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f463) => {
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f464) => {
                    false
                }
                self::JavaScriptSemanticValue::SourceText(source_ml_source2_f465, source_ml_source2_f466) => {
                    false
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f467) => {
                    false
                }
                self::JavaScriptSemanticValue::Object(source_ml_source2_f468, source_ml_source2_f469) => {
                    false
                }
            }) {
                self::JavaScriptCallPrimitiveResult::PrimitiveError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104, 32, 99, 97, 108, 108, 101, 100, 32, 111, 110, 32, 110, 117, 108, 108, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100]))
            } else {
                match receiver.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        {
                            let receiver_error = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error.clone())
                            } else {
                                {
                                    let text = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                            let value = *value;
                                            self::execute_java_script_primitive_prefix_search(value.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f479) => {
                                            text.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f480, source_ml_source2_f481) => {
                                            text.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f482) => {
                                            text.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Null => {
                        {
                            let receiver_error_2 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_2.clone())
                            } else {
                                {
                                    let text_2 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_2.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_2) => {
                                            let value_2 = *value_2;
                                            self::execute_java_script_primitive_prefix_search(value_2.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f483) => {
                                            text_2.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f484, source_ml_source2_f485) => {
                                            text_2.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f486) => {
                                            text_2.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f470) => {
                        {
                            let receiver_error_3 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_3.clone())
                            } else {
                                {
                                    let text_3 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_3.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_3) => {
                                            let value_3 = *value_3;
                                            self::execute_java_script_primitive_prefix_search(value_3.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f487) => {
                                            text_3.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f488, source_ml_source2_f489) => {
                                            text_3.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f490) => {
                                            text_3.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f471) => {
                        {
                            let receiver_error_4 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_4 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_4.clone())
                            } else {
                                {
                                    let text_4 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_4.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_4) => {
                                            let value_4 = *value_4;
                                            self::execute_java_script_primitive_prefix_search(value_4.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f491) => {
                                            text_4.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f492, source_ml_source2_f493) => {
                                            text_4.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f494) => {
                                            text_4.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f472) => {
                        {
                            let receiver_error_5 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_5 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_5.clone())
                            } else {
                                {
                                    let text_5 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_5.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_5) => {
                                            let value_5 = *value_5;
                                            self::execute_java_script_primitive_prefix_search(value_5.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f495) => {
                                            text_5.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f496, source_ml_source2_f497) => {
                                            text_5.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f498) => {
                                            text_5.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f473) => {
                        {
                            let receiver_error_6 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_6 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_6.clone())
                            } else {
                                {
                                    let text_6 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_6.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_6) => {
                                            let value_6 = *value_6;
                                            self::execute_java_script_primitive_prefix_search(value_6.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f499) => {
                                            text_6.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f500, source_ml_source2_f501) => {
                                            text_6.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f502) => {
                                            text_6.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f474, source_ml_source2_f475) => {
                        {
                            let receiver_error_7 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_7 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_7.clone())
                            } else {
                                {
                                    let text_7 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_7.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_7) => {
                                            let value_7 = *value_7;
                                            self::execute_java_script_primitive_prefix_search(value_7.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f503) => {
                                            text_7.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f504, source_ml_source2_f505) => {
                                            text_7.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f506) => {
                                            text_7.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f476) => {
                        {
                            let receiver_error_8 = self::read_java_script_semantic_value_error(receiver.clone());
                            if (receiver_error_8 != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(receiver_error_8.clone())
                            } else {
                                {
                                    let text_8 = self::execute_java_script_call_text_conversion(receiver.clone());
                                    match text_8.clone() {
                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_8) => {
                                            let value_8 = *value_8;
                                            self::execute_java_script_primitive_prefix_search(value_8.clone(), args)
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(source_ml_source2_f507) => {
                                            text_8.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(source_ml_source2_f508, source_ml_source2_f509) => {
                                            text_8.clone()
                                        }
                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(source_ml_source2_f510) => {
                                            text_8.clone()
                                        }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f477, source_ml_source2_f478) => {
                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![116, 104, 105, 115, 32, 83, 116, 114, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 84, 111, 80, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                    }
                }
            }
        }
    }
}
