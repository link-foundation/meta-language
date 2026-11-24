pub fn source_ml_source1_temporary_test_java_script_semantic_text_equal_loop34(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result {
    loop {
        return if (at < length) {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        (-1f64)
                    }
                    self::JavaScriptSemanticValue::Null => {
                        (-1f64)
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f668) => {
                        {
                            let source_ml_source2_f692 = source_ml_source2_f668;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source2_f669) => {
                        {
                            let source_ml_source2_f701 = source_ml_source2_f669;
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f670) => {
                        {
                            let source_ml_source2_f710 = source_ml_source2_f670.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Text(units) => {
                        (self::ml_array::optional_at(&units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(source_ml_source2_f671, source_ml_source2_f672) => {
                        {
                            let source_ml_source2_f720 = source_ml_source2_f671.clone();
                            {
                                let text = source_ml_source2_f672.clone();
                                {
                                    let source_ml_source2_string_unit_operand724 = text.clone();
                                    {
                                        let source_ml_source2_string_unit_operand725 = at;
                                        (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if ((index < (0f64)) || (index >= (text_2.length()))) {
                                                f64::NAN
                                            } else {
                                                ({
                                                    let source_ml_source2_string_unit_operand731 = {
                                                        let source_ml_source2_string_range_operand726 = text_2.clone();
                                                        {
                                                            let source_ml_source2_string_range_operand727 = index;
                                                            {
                                                                let source_ml_source2_string_range_operand728 = (index + 1f64);
                                                                {
                                                                    let source_ml_source2_string_range_position729 = source_ml_source2_string_range_operand727;
                                                                    {
                                                                        let source_ml_source2_string_range_position730 = source_ml_source2_string_range_operand728;
                                                                        source_ml_source2_string_range_operand726.slice(source_ml_source2_string_range_position729, source_ml_source2_string_range_position730)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_string_unit_operand732 = 0f64;
                                                        source_ml_source2_string_unit_operand731.code_point_at(source_ml_source2_string_unit_operand732)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source2_string_unit_operand724.clone(), source_ml_source2_string_unit_operand725)
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f673) => {
                        {
                            let source_ml_source2_f738 = source_ml_source2_f673.clone();
                            (-1f64)
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f674, source_ml_source2_f675) => {
                        {
                            let source_ml_source2_f747 = source_ml_source2_f674.clone();
                            {
                                let source_ml_source2_f748 = source_ml_source2_f675;
                                (-1f64)
                            }
                        }
                    }
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            (-1f64)
                        }
                        self::JavaScriptSemanticValue::Null => {
                            (-1f64)
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f749) => {
                            {
                                let source_ml_source2_f773 = source_ml_source2_f749;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f750) => {
                            {
                                let source_ml_source2_f782 = source_ml_source2_f750;
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f751) => {
                            {
                                let source_ml_source2_f791 = source_ml_source2_f751.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            (self::ml_array::optional_at(&units_2, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(source_ml_source2_f752, source_ml_source2_f753) => {
                            {
                                let source_ml_source2_f801 = source_ml_source2_f752.clone();
                                {
                                    let text_2 = source_ml_source2_f753.clone();
                                    {
                                        let source_ml_source2_string_unit_operand805 = text_2.clone();
                                        {
                                            let source_ml_source2_string_unit_operand806 = at;
                                            (std::rc::Rc::new(move |text_3: self::ml_string::Text, position: f64| -> f64 { {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if ((index < (0f64)) || (index >= (text_3.length()))) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                        let source_ml_source2_string_unit_operand812 = {
                                                            let source_ml_source2_string_range_operand807 = text_3.clone();
                                                            {
                                                                let source_ml_source2_string_range_operand808 = index;
                                                                {
                                                                    let source_ml_source2_string_range_operand809 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source2_string_range_position810 = source_ml_source2_string_range_operand808;
                                                                        {
                                                                            let source_ml_source2_string_range_position811 = source_ml_source2_string_range_operand809;
                                                                            source_ml_source2_string_range_operand807.slice(source_ml_source2_string_range_position810, source_ml_source2_string_range_position811)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_string_unit_operand813 = 0f64;
                                                            source_ml_source2_string_unit_operand812.code_point_at(source_ml_source2_string_unit_operand813)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source2_string_unit_operand805.clone(), source_ml_source2_string_unit_operand806)
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f754) => {
                            {
                                let source_ml_source2_f819 = source_ml_source2_f754.clone();
                                (-1f64)
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f755, source_ml_source2_f756) => {
                            {
                                let source_ml_source2_f828 = source_ml_source2_f755.clone();
                                {
                                    let source_ml_source2_f829 = source_ml_source2_f756;
                                    (-1f64)
                                }
                            }
                        }
                    };
                    if (a != b) {
                        self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Return(false)
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop34Done
        };
    }
}

pub fn source_ml_source1_temporary_read_java_script_semantic_text_string_loop36(mut text: self::ml_string::Text, mut source_ml_source1_temporary_values35: Vec<f64>, mut source_ml_source1_temporary_index35: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_source1_temporary_index35 < (self::ml::Big::from_u128(source_ml_source1_temporary_values35.len() as u128))) {
            {
                let unit = self::ml_array::at(&source_ml_source1_temporary_values35, source_ml_source1_temporary_index35.to_index());
                {
                    let text_2 = text.concat(&{
                        let source_ml_source2_character_argument830 = unit;
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source2_character_argument830]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                    });
                    {
                        let source_ml_source1_temporary_index35_2 = source_ml_source1_temporary_index35.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_source1_temporary_index35) = (text_2.clone(), source_ml_source1_temporary_index35_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            text.clone()
        };
    }
}

pub fn source_ml_source1_temporary_construct_java_script_semantic_text_units_loop37(mut value: self::JavaScriptSemanticValue, mut length: f64, mut units: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < length) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![self::read_java_script_semantic_text_unit(value.clone(), at)], units); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (units, at) = (units_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            units
        };
    }
}

pub fn source_ml_source1_temporary_slice_utf16_text_loop38(mut text: Vec<f64>, mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_points_utf16_text_loop39(mut text: Vec<f64>, mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let first = (self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(&text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                    if ((((first >= (55296f64)) && (first <= (56319f64))) && (second >= (56320f64))) && (second <= (57343f64))) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![first, second]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    let at_3 = (at_2 + 1f64);
                                    {
                                        (result, at) = (result_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![vec![first]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_4 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_4);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_split_utf16_text_loop40(mut text: Vec<f64>, mut count: f64, mut result: Vec<Vec<f64>>, mut at: f64) -> Vec<Vec<f64>> {
    loop {
        return if ((at < ((text.len() as f64))) && (((result.len() as f64)) < count)) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)]], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporarySplitUtf16TextLoop41Result {
    MlSource1TemporarySplitUtf16TextLoop41Done(Vec<Vec<f64>>, f64),
    MlSource1TemporarySplitUtf16TextLoop41Return(Vec<Vec<f64>>),
}

pub fn source_ml_source1_temporary_split_utf16_text_loop41(mut text: Vec<f64>, mut separator: Vec<f64>, mut count: f64, mut result: Vec<Vec<f64>>, mut start: f64) -> self::MlSource1TemporarySplitUtf16TextLoop41Result {
    loop {
        return {
            let found = self::index_utf16_text(text.clone(), separator.clone(), start);
            if (found < (0f64)) {
                self::MlSource1TemporarySplitUtf16TextLoop41Result::MlSource1TemporarySplitUtf16TextLoop41Done(result, start)
            } else {
                {
                    let result_2 = { let (ml_append_right, ml_append_left) = (vec![self::slice_utf16_text(text.clone(), start, found)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                    if (((result_2.len() as f64)) == count) {
                        self::MlSource1TemporarySplitUtf16TextLoop41Result::MlSource1TemporarySplitUtf16TextLoop41Return(result_2)
                    } else {
                        {
                            let start_2 = (found + (separator.len() as f64));
                            {
                                (result, start) = (result_2, start_2);
                                continue;
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMatchUtf16TextLoop42Result {
    MlSource1TemporaryMatchUtf16TextLoop42Done,
    MlSource1TemporaryMatchUtf16TextLoop42Return(bool),
}

pub fn source_ml_source1_temporary_match_utf16_text_loop42(mut text: Vec<f64>, mut search: Vec<f64>, mut start: f64, mut at: f64) -> self::MlSource1TemporaryMatchUtf16TextLoop42Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source2_comparison_operand831 = self::ml_array::optional_at(&text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source2_comparison_operand832 = self::ml_array::optional_at(&search, self::ml_array::number_index(at));
                    !(match (source_ml_source2_comparison_operand831.clone(), source_ml_source2_comparison_operand832.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlSource1TemporaryMatchUtf16TextLoop42Result::MlSource1TemporaryMatchUtf16TextLoop42Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryMatchUtf16TextLoop42Result::MlSource1TemporaryMatchUtf16TextLoop42Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryIndexUtf16TextLoop43Result {
    MlSource1TemporaryIndexUtf16TextLoop43Done,
    MlSource1TemporaryIndexUtf16TextLoop43Return(f64),
}

pub fn source_ml_source1_temporary_index_utf16_text_loop43(mut text: Vec<f64>, mut search: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryIndexUtf16TextLoop43Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text.clone(), search.clone(), at) {
                self::MlSource1TemporaryIndexUtf16TextLoop43Result::MlSource1TemporaryIndexUtf16TextLoop43Return(at)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryIndexUtf16TextLoop43Result::MlSource1TemporaryIndexUtf16TextLoop43Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLastIndexUtf16TextLoop44Result {
    MlSource1TemporaryLastIndexUtf16TextLoop44Done,
    MlSource1TemporaryLastIndexUtf16TextLoop44Return(f64),
}

pub fn source_ml_source1_temporary_last_index_utf16_text_loop44(mut text: Vec<f64>, mut search: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryLastIndexUtf16TextLoop44Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text.clone(), search.clone(), at) {
                self::MlSource1TemporaryLastIndexUtf16TextLoop44Result::MlSource1TemporaryLastIndexUtf16TextLoop44Return(at)
            } else {
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLastIndexUtf16TextLoop44Result::MlSource1TemporaryLastIndexUtf16TextLoop44Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareUtf16TextLoop45Result {
    MlSource1TemporaryCompareUtf16TextLoop45Done,
    MlSource1TemporaryCompareUtf16TextLoop45Return(f64),
}

pub fn source_ml_source1_temporary_compare_utf16_text_loop45(mut left: Vec<f64>, mut right: Vec<f64>, mut count: f64, mut at: f64) -> self::MlSource1TemporaryCompareUtf16TextLoop45Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(&left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(&right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlSource1TemporaryCompareUtf16TextLoop45Result::MlSource1TemporaryCompareUtf16TextLoop45Return(if (a < b) {
                            (-1f64)
                        } else {
                            1f64
                        })
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompareUtf16TextLoop45Result::MlSource1TemporaryCompareUtf16TextLoop45Done
        };
    }
}

pub fn source_ml_source1_temporary_join_utf16_text_loop47(mut separator: Vec<f64>, mut result: Vec<f64>, mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((separator.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&separator, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_join_utf16_text_loop48(mut result: Vec<f64>, mut value: Vec<f64>, mut offset: f64) -> Vec<f64> {
    loop {
        return if (offset < ((value.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&value, self::ml_array::number_index(offset))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let offset_2 = (offset + 1f64);
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_join_utf16_text_loop46(mut values: Vec<Vec<f64>>, mut separator: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let result_3 = if (at != (0f64)) {
                    {
                        let offset = 0f64;
                        {
                            let result_2 = self::source_ml_source1_temporary_join_utf16_text_loop47(separator.clone(), result, offset);
                            result_2
                        }
                    }
                } else {
                    result
                };
                {
                    let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| Vec::<f64>::new());
                    {
                        let offset_2 = 0f64;
                        {
                            let result_4 = self::source_ml_source1_temporary_join_utf16_text_loop48(result_3, value, offset_2);
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_4, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_trim_utf16_text_loop49(mut text: Vec<f64>, mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((first < last) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(&text, self::ml_array::number_index(first))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let first_2 = (first + 1f64);
                {
                    first = first_2;
                    continue;
                }
            }
        } else {
            first
        };
    }
}

pub fn source_ml_source1_temporary_trim_utf16_text_loop50(mut text: Vec<f64>, mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((last > first) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(&text, self::ml_array::number_index((last - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let last_2 = (last - 1f64);
                {
                    last = last_2;
                    continue;
                }
            }
        } else {
            last
        };
    }
}

pub fn source_ml_source1_temporary_character_units_utf16_text_loop51(mut values: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let integer = if value.is_finite() {
                        value.trunc()
                    } else {
                        0f64
                    };
                    {
                        let residue = (integer % 65536f64);
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (residue < (0f64)) {
                                (residue + 65536f64)
                            } else {
                                if (residue == (0f64)) {
                                    0f64
                                } else {
                                    residue
                                }
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result {
    MlSource1TemporaryCodePointUnitsUtf16TextLoop52Done(Vec<f64>),
    MlSource1TemporaryCodePointUnitsUtf16TextLoop52Return(Vec<f64>),
}

pub fn source_ml_source1_temporary_code_point_units_utf16_text_loop52(mut values: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> self::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(&values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(value)) || (value < (0f64))) || (value > (1114111f64))) {
                    self::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Return(vec![at])
                } else {
                    if (value < (65536f64)) {
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (value == (0f64)) {
                                0f64
                            } else {
                                value
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let result_3 = { let (ml_append_right, ml_append_left) = (vec![(55296f64 + (((value - 65536f64) / 1024f64)).floor()), (56320f64 + ((value - 65536f64) % 1024f64))], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (result, at) = (result_3, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Result::MlSource1TemporaryCodePointUnitsUtf16TextLoop52Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDisplayPointsUtf16TextJoin54 {
    MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(f64, f64),
}

pub fn source_ml_source1_temporary_display_points_utf16_text_loop53(mut text: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                {
                    let source_ml_source2_s833 = if ((point >= (55296f64)) && (point <= (56319f64))) {
                        {
                            let low = (self::ml_array::optional_at(&text, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                            if ((low >= (56320f64)) && (low <= (57343f64))) {
                                {
                                    let point_2 = (((65536f64 + ((point - 55296f64) * 1024f64)) + low) - 56320f64);
                                    {
                                        let at_2 = (at + 1f64);
                                        self::MlSource1TemporaryDisplayPointsUtf16TextJoin54::MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(at_2, point_2)
                                    }
                                }
                            } else {
                                {
                                    let point_3 = 65533f64;
                                    self::MlSource1TemporaryDisplayPointsUtf16TextJoin54::MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(at, point_3)
                                }
                            }
                        }
                    } else {
                        if ((point >= (56320f64)) && (point <= (57343f64))) {
                            {
                                let point_4 = 65533f64;
                                self::MlSource1TemporaryDisplayPointsUtf16TextJoin54::MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(at, point_4)
                            }
                        } else {
                            self::MlSource1TemporaryDisplayPointsUtf16TextJoin54::MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(at, point)
                        }
                    };
                    match source_ml_source2_s833.clone() {
                        self::MlSource1TemporaryDisplayPointsUtf16TextJoin54::MlSource1TemporaryDisplayPointsUtf16TextJoin54Next(at_3, point_5) => {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![point_5], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = (at_3 + 1f64);
                                    {
                                        (result, at) = (result_2, at_4);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_source1_temporary_display_utf8_from_utf16_text_loop55(mut bytes: Vec<f64>, mut points: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((points.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(&points, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                if (point < (128f64)) {
                    {
                        let bytes_2 = { let (ml_append_right, ml_append_left) = (vec![point], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (bytes, at) = (bytes_2, at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    if (point < (2048f64)) {
                        {
                            let bytes_3 = { let (ml_append_right, ml_append_left) = (vec![(192f64 + ((point / 64f64)).floor()), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (bytes, at) = (bytes_3, at_3);
                                    continue;
                                }
                            }
                        }
                    } else {
                        if (point < (65536f64)) {
                            {
                                let bytes_4 = { let (ml_append_right, ml_append_left) = (vec![(224f64 + ((point / 4096f64)).floor()), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_4, at_4);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let bytes_5 = { let (ml_append_right, ml_append_left) = (vec![(240f64 + ((point / 262144f64)).floor()), (128f64 + (((point / 4096f64)).floor() % 64f64)), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_5 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_5, at_5);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            bytes
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Result {
    MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Done(Vec<f64>, f64),
}

pub fn source_ml_source1_temporary_plan_utf16_text_substitution_loop56(mut replacement: Vec<f64>, mut result: Vec<f64>, mut start: f64, mut at: f64) -> self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Result {
    loop {
        return if (at < ((replacement.len() as f64))) {
            {
                let next = (self::ml_array::optional_at(&replacement, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                if ((((self::ml_array::optional_at(&replacement, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)) == (36f64)) && ((((next == (36f64)) || (next == (38f64))) || (next == (96f64))) || (next == (39f64)))) {
                    {
                        let result_3 = if (start < at) {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![0f64, start, at], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let result_6 = if (next == (36f64)) {
                                {
                                    let result_4 = { let (ml_append_right, ml_append_left) = (vec![0f64, at, (at + 1f64)], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                    result_4
                                }
                            } else {
                                {
                                    let result_5 = { let (ml_append_right, ml_append_left) = (vec![if (next == (38f64)) {
                                        1f64
                                    } else {
                                        if (next == (96f64)) {
                                            2f64
                                        } else {
                                            3f64
                                        }
                                    }, 0f64, 0f64], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                    result_5
                                }
                            };
                            {
                                let at_2 = (at + 2f64);
                                {
                                    let start_2 = at_2;
                                    {
                                        (result, start, at) = (result_6, start_2, at_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let at_3 = (at + 1f64);
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Result::MlSource1TemporaryPlanUtf16TextSubstitutionLoop56Done(result, start)
        };
    }
}

pub fn source_ml_source1_temporary_replace_utf16_text_loop58(mut text: Vec<f64>, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(&text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReplaceUtf16TextLoop57Result {
    MlSource1TemporaryReplaceUtf16TextLoop57Done(Vec<f64>, f64),
}
