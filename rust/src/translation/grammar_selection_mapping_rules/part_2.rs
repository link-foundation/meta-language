// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::UnicodeEscape;
use super::decode_unicode_escape;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_grammar_selection_json_short_escape;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_json_string_part(codes: &[f64], index: f64) -> Vec<f64> {
    {
        let last = (codes.len() as f64) - 1f64;
        if ((((((codes.len() as f64) < 2f64)
            || !(match (
                self::ml_array::optional_at(codes, self::ml_array::number_index(0f64)),
                34f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }))
            || !(match (
                self::ml_array::optional_at(codes, self::ml_array::number_index(last)),
                34f64,
            ) {
                (Some(ml_left), ml_right) => ml_left == ml_right,
                _ => false,
            }))
            || !(self::ml_math::is_integer(index)))
            || (index < 1f64))
            || (index > last)
        {
            vec![1f64, 0f64]
        } else if index == last {
            vec![3f64, index]
        } else {
            {
                let code =
                    (self::ml_array::optional_at(codes, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                if (((!(self::ml_math::is_integer(code)) || (code < 32f64))
                    || (code > 1_114_111f64))
                    || ((55_296f64..=57_343f64).contains(&code)))
                    || (code == 34f64)
                {
                    vec![1f64, index]
                } else {
                    let ml_library_inequality_2 = code != 92f64;
                    if ml_library_inequality_2 {
                        vec![0f64, code, (index + 1f64)]
                    } else {
                        {
                            let escaped = (self::ml_array::optional_at(
                                codes,
                                self::ml_array::number_index(index + 1f64),
                            ))
                            .unwrap_or_else(|| -1f64);
                            {
                                let ml_library_inequality_1 = escaped != 117f64;
                                if ml_library_inequality_1 {
                                    {
                                        let value =
                                            self::read_grammar_selection_json_short_escape(escaped);
                                        if (value < 0f64) || ((index + 2f64) > last) {
                                            vec![1f64, index]
                                        } else {
                                            vec![0f64, value, (index + 2f64)]
                                        }
                                    }
                                } else if match (
                                    self::ml_array::optional_at(
                                        codes,
                                        self::ml_array::number_index(index + 2f64),
                                    ),
                                    123f64,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                } {
                                    vec![1f64, index]
                                } else {
                                    {
                                        let decoded = self::decode_unicode_escape(
                                            &(self::ml_array::slice(codes, index, (index + 12f64))),
                                            true,
                                        );
                                        match decoded {
                                            self::UnicodeEscape::Scalar(code_2, end) => {
                                                if (index + end) > last {
                                                    vec![1f64, index]
                                                } else {
                                                    vec![0f64, code_2, (index + end)]
                                                }
                                            }
                                            self::UnicodeEscape::Malformed => {
                                                vec![1f64, index]
                                            }
                                            self::UnicodeEscape::Unsupported(ml_f9, ml_f10) => {
                                                vec![2f64, index]
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
pub fn read_grammar_selection_mapping_reservation(bytes: f64, steps: bool) -> f64 {
    if (!(self::ml_math::is_integer(bytes)) || (bytes < 0f64)) || (bytes > 4_294_967_295f64) {
        -1f64
    } else {
        (if steps { 32f64 } else { 12f64 } * bytes) + 32f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_mapped_offset(intervals: &[f64], offset: f64) -> f64 {
    if ((((intervals.len() as f64) < 8f64) || (((intervals.len() as f64) % 4f64) != 0f64))
        || !(self::ml_math::is_integer(offset)))
        || (offset < 0f64)
    {
        -1f64
    } else {
        {
            let index = 0f64;
            {
                let ml_s11 =
                    self::ml_read_grammar_selection_mapped_offset_loop11(intervals, offset, index);
                match ml_s11 {
                    self::MlReadGrammarSelectionMappedOffsetLoop11Result::MlReadGrammarSelectionMappedOffsetLoop11Done => {
                        {
                            let last = (intervals.len() as f64) - 4f64;
                            if match (offset, self::ml_array::optional_at(intervals, self::ml_array::number_index(last + 2f64))) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false } {
                                (self::ml_array::optional_at(intervals, self::ml_array::number_index(last))).unwrap_or_else(|| -1f64)
                            } else {
                                -1f64
                            }
                        }
                    }
                    self::MlReadGrammarSelectionMappedOffsetLoop11Result::MlReadGrammarSelectionMappedOffsetLoop11Return(ml_result11) => {
                        ml_result11
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_mapped_cover(
    intervals: &[Vec<f64>],
    start: f64,
    end: f64,
) -> Vec<f64> {
    if (((((intervals.len() as f64) < 2f64) || !(self::ml_math::is_integer(start)))
        || !(self::ml_math::is_integer(end)))
        || (start < 0f64))
        || (end < start)
    {
        Vec::<f64>::new()
    } else {
        {
            let closing = (self::ml_array::optional_at(
                intervals,
                self::ml_array::number_index((intervals.len() as f64) - 1f64),
            ))
            .unwrap_or_else(|| Vec::<f64>::new());
            if ((closing.len() as f64) != 4f64)
                || (end
                    > ((self::ml_array::optional_at(&closing, self::ml_array::number_index(3f64)))
                        .unwrap_or_else(|| -1f64)))
            {
                Vec::<f64>::new()
            } else if (start == end)
                && match (
                    end,
                    self::ml_array::optional_at(&closing, self::ml_array::number_index(3f64)),
                ) {
                    (ml_left, Some(ml_right)) => ml_left == ml_right,
                    _ => false,
                }
            {
                vec![
                    (self::ml_array::optional_at(&closing, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| -1f64),
                    (self::ml_array::optional_at(&closing, self::ml_array::number_index(0f64)))
                        .unwrap_or_else(|| -1f64),
                    (self::ml_array::optional_at(&closing, self::ml_array::number_index(1f64)))
                        .unwrap_or_else(|| -1f64),
                ]
            } else {
                {
                    let low = 0f64;
                    {
                        let high = (intervals.len() as f64) - 1f64;
                        {
                            let ml_s12 = self::ml_read_grammar_selection_mapped_cover_loop12(
                                intervals, start, low, high,
                            );
                            match ml_s12 {
                                    self::MlReadGrammarSelectionMappedCoverLoop12Result::MlReadGrammarSelectionMappedCoverLoop12Done(low_2, high_2) => {
                                        {
                                            let first = (self::ml_array::optional_at(intervals, self::ml_array::number_index(low_2))).unwrap_or_else(|| Vec::<f64>::new());
                                            if (((first.len() as f64) != 4f64) || (((self::ml_array::optional_at(&first, self::ml_array::number_index(2f64))).unwrap_or_else(|| -1f64)) > start)) || (start >= ((self::ml_array::optional_at(&first, self::ml_array::number_index(3f64))).unwrap_or_else(|| -1f64))) {
                                                Vec::<f64>::new()
                                            } else  if start == end {
                                                    vec![(self::ml_array::optional_at(&first, self::ml_array::number_index(0f64))).unwrap_or_else(|| -1f64), (self::ml_array::optional_at(&first, self::ml_array::number_index(0f64))).unwrap_or_else(|| -1f64), (self::ml_array::optional_at(&first, self::ml_array::number_index(1f64))).unwrap_or_else(|| -1f64)]
                                                } else {
                                                    {
                                                        let high_3 = (intervals.len() as f64) - 1f64;
                                                        {
                                                            let low_3 = self::ml_read_grammar_selection_mapped_cover_loop13(intervals, end, low_2, high_3);
                                                            {
                                                                let last = (self::ml_array::optional_at(intervals, self::ml_array::number_index(low_3))).unwrap_or_else(|| Vec::<f64>::new());
                                                                if ((last.len() as f64) != 4f64) || (end > ((self::ml_array::optional_at(&last, self::ml_array::number_index(3f64))).unwrap_or_else(|| -1f64))) {
                                                                    Vec::<f64>::new()
                                                                } else {
                                                                    vec![(self::ml_array::optional_at(&first, self::ml_array::number_index(0f64))).unwrap_or_else(|| -1f64), (self::ml_array::optional_at(&first, self::ml_array::number_index(0f64))).unwrap_or_else(|| -1f64), (self::ml_array::optional_at(&last, self::ml_array::number_index(1f64))).unwrap_or_else(|| -1f64)]
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_mapping_field_order(role: &str) -> String {
    if role == "source" {
        String::from("kind|identity|encoding|byteLength|derivation")
    } else if role == "derivation" {
        String::from("operation|input|start|end")
    } else if role == "mapping" {
        String::from("kind|operation|input|output|intervals")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_source_definition(
    kind: &str,
    identity: f64,
    position: f64,
    encoding: &str,
    length: f64,
    actual_length: f64,
) -> bool {
    ((((((kind == "grammar-source")
        && ((identity).partial_cmp(&(position)) == Some(std::cmp::Ordering::Equal)))
        && (encoding == "UTF-8"))
        && self::ml_math::is_integer(length))
        && (length >= 0f64))
        && (length <= 4_294_967_295f64))
        && ((length).partial_cmp(&(actual_length)) == Some(std::cmp::Ordering::Equal))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_source_derivation(
    operation: &str,
    input: f64,
    output: f64,
    start: f64,
    end: f64,
    input_length: f64,
) -> bool {
    ((((((((operation == "json-string") && self::ml_math::is_integer(input))
        && (input >= 0f64))
        && (input < output))
        && self::ml_math::is_integer(start))
        && self::ml_math::is_integer(end))
        && (start >= 0f64))
        && (start <= end))
        && (end <= input_length)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_source_interval(actual: &[f64], expected: &[f64]) -> bool {
    ((((((actual.len() as f64) == 4f64) && ((expected.len() as f64) == 4f64))
        && match (
            self::ml_array::optional_at(actual, self::ml_array::number_index(0f64)),
            self::ml_array::optional_at(expected, self::ml_array::number_index(0f64)),
        ) {
            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
            (None, None) => true,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(actual, self::ml_array::number_index(1f64)),
            self::ml_array::optional_at(expected, self::ml_array::number_index(1f64)),
        ) {
            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
            (None, None) => true,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(actual, self::ml_array::number_index(2f64)),
            self::ml_array::optional_at(expected, self::ml_array::number_index(2f64)),
        ) {
            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
            (None, None) => true,
            _ => false,
        })
        && match (
            self::ml_array::optional_at(actual, self::ml_array::number_index(3f64)),
            self::ml_array::optional_at(expected, self::ml_array::number_index(3f64)),
        ) {
            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
            (None, None) => true,
            _ => false,
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_source_root(position: f64, absent_derivation: bool) -> bool {
    if position == 0f64 {
        absent_derivation
    } else {
        !absent_derivation
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_mapping_reference(
    input: f64,
    actual_input: f64,
    output: f64,
    count: f64,
) -> bool {
    ((((input).partial_cmp(&(actual_input)) == Some(std::cmp::Ordering::Equal))
        && self::ml_math::is_integer(output))
        && (output > input))
        && (output < count)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_mapping_occurrence(
    input: f64,
    actual_input: f64,
    start: f64,
    actual_start: f64,
    end: f64,
    actual_end: f64,
) -> bool {
    (((input).partial_cmp(&(actual_input)) == Some(std::cmp::Ordering::Equal))
        && ((start).partial_cmp(&(actual_start)) == Some(std::cmp::Ordering::Equal)))
        && ((end).partial_cmp(&(actual_end)) == Some(std::cmp::Ordering::Equal))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_physical_span(
    start: f64,
    end: f64,
    cursor: f64,
    byte_length: f64,
    missing: bool,
) -> bool {
    ((((self::ml_math::is_integer(start) && self::ml_math::is_integer(end)) && (start >= 0f64))
        && (end <= 4_294_967_295f64))
        && ((start).partial_cmp(&(cursor)) == Some(std::cmp::Ordering::Equal)))
        && if missing {
            (end).partial_cmp(&(cursor)) == Some(std::cmp::Ordering::Equal)
        } else {
            (end >= start) && ((end - start) == byte_length)
        }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_grammar_selection_tree_children(kind: &str, mapped: bool) -> String {
    if kind == "node" {
        String::from("children")
    } else if kind == "embed" {
        if mapped {
            String::from("sourceChildren")
        } else {
            String::from("root")
        }
    } else if (kind == "token") || (kind == "error") {
        String::from("leaf")
    } else if kind == "missing" {
        String::from("missing")
    } else {
        String::from("unsupported")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_mapping_profile(
    mapping: &str,
    physical_children: bool,
    source_mapping: bool,
) -> bool {
    if mapping == "identity" {
        !physical_children && !source_mapping
    } else {
        ((mapping == "json-string") && physical_children) && source_mapping
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_grammar_selection_mapping_descriptor(kind: &str, operation: &str) -> bool {
    (kind == "source-mapping") && (operation == "json-string")
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin5 {
    MlDecodeUnicodeEscapeJoin5Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeLoop6Result {
    MlDecodeUnicodeEscapeLoop6Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_decode_unicode_escape_loop6(
    units: &[f64],
    mut index: f64,
    mut low: f64,
    mut offset: f64,
) -> self::MlDecodeUnicodeEscapeLoop6Result {
    loop {
        return if offset < 6f64 {
            {
                let unit = self::ml_array::optional_at(
                    units,
                    self::ml_array::number_index(index + offset),
                );
                {
                    let digit = -1f64;
                    {
                        let digit_5 = if (((unit).unwrap_or(f64::NAN)) >= 48f64)
                            && (((unit).unwrap_or(f64::NAN)) <= 57f64)
                        {
                            { (unit).unwrap_or(f64::NAN) - 48f64 }
                        } else if (((unit).unwrap_or(f64::NAN)) >= 65f64)
                            && (((unit).unwrap_or(f64::NAN)) <= 70f64)
                        {
                            { (unit).unwrap_or(f64::NAN) - 55f64 }
                        } else if (((unit).unwrap_or(f64::NAN)) >= 97f64)
                            && (((unit).unwrap_or(f64::NAN)) <= 102f64)
                        {
                            { (unit).unwrap_or(f64::NAN) - 87f64 }
                        } else {
                            digit
                        };
                        if digit_5 < 0f64 {
                            self::MlDecodeUnicodeEscapeLoop6Result::MlDecodeUnicodeEscapeLoop6Done(
                                low, offset,
                            )
                        } else {
                            {
                                let low_2 = (low * 16f64) + digit_5;
                                {
                                    let offset_2 = offset + 1f64;
                                    {
                                        (low, offset) = (low_2, offset_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlDecodeUnicodeEscapeLoop6Result::MlDecodeUnicodeEscapeLoop6Done(low, offset)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin4 {
    MlDecodeUnicodeEscapeJoin4Next(f64),
    MlDecodeUnicodeEscapeJoin4Return(Box<self::UnicodeEscape>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeLoop3Result {
    MlDecodeUnicodeEscapeLoop3Done(f64, f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_decode_unicode_escape_loop3(
    units: &[f64],
    mut fixed: f64,
    mut index: f64,
    mut code: f64,
    mut digits: f64,
    mut braced: bool,
) -> self::MlDecodeUnicodeEscapeLoop3Result {
    loop {
        return if (index < (units.len() as f64)) && (digits < (if braced { 6f64 } else { fixed })) {
            {
                let unit = self::ml_array::optional_at(units, self::ml_array::number_index(index));
                {
                    let digit = -1f64;
                    {
                        let digit_5 = if (((unit).unwrap_or(f64::NAN)) >= 48f64)
                            && (((unit).unwrap_or(f64::NAN)) <= 57f64)
                        {
                            { (unit).unwrap_or(f64::NAN) - 48f64 }
                        } else if (((unit).unwrap_or(f64::NAN)) >= 65f64)
                            && (((unit).unwrap_or(f64::NAN)) <= 70f64)
                        {
                            { (unit).unwrap_or(f64::NAN) - 55f64 }
                        } else if (((unit).unwrap_or(f64::NAN)) >= 97f64)
                            && (((unit).unwrap_or(f64::NAN)) <= 102f64)
                        {
                            { (unit).unwrap_or(f64::NAN) - 87f64 }
                        } else {
                            digit
                        };
                        if digit_5 < 0f64 {
                            self::MlDecodeUnicodeEscapeLoop3Result::MlDecodeUnicodeEscapeLoop3Done(
                                index, code, digits,
                            )
                        } else {
                            {
                                let code_2 = (code * 16f64) + digit_5;
                                {
                                    let digits_2 = digits + 1f64;
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (index, code, digits) = (index_2, code_2, digits_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlDecodeUnicodeEscapeLoop3Result::MlDecodeUnicodeEscapeLoop3Done(
                index, code, digits,
            )
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin2 {
    MlDecodeUnicodeEscapeJoin2Next(f64, bool),
    MlDecodeUnicodeEscapeJoin2Return(Box<self::UnicodeEscape>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlDecodeUnicodeEscapeJoin1 {
    MlDecodeUnicodeEscapeJoin1Next(f64),
    MlDecodeUnicodeEscapeJoin1Return(Box<self::UnicodeEscape>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_selection_mapping_context_loop9(
    mut mapped: bool,
    ml_values7: &[String],
    mut ml_index7: self::ml::Big,
) -> bool {
    loop {
        if ml_index7 < (self::ml::Big::from_u128(ml_values7.len() as u128)) {
            {
                let mapping = self::ml_array::at(ml_values7, ml_index7.to_index());
                if mapping == "json-string" {
                    {
                        let mapped_2 = true;
                        {
                            let ml_index7_2 = ml_index7.add(&self::ml::Big::from_i128(1));
                            {
                                (mapped, ml_index7) = (mapped_2, ml_index7_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let ml_index7_3 = ml_index7.add(&self::ml::Big::from_i128(1));
                        {
                            ml_index7 = ml_index7_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { mapped };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarSelectionMappingContextLoop10Result {
    MlReadGrammarSelectionMappingContextLoop10Done,
    MlReadGrammarSelectionMappingContextLoop10Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_selection_mapping_context_loop10(
    ml_values8: &[String],
    mut ml_index8: self::ml::Big,
) -> self::MlReadGrammarSelectionMappingContextLoop10Result {
    loop {
        return if ml_index8 < (self::ml::Big::from_u128(ml_values8.len() as u128)) {
            {
                let mapping = self::ml_array::at(ml_values8, ml_index8.to_index());
                {
                    let ml_library_inequality_1 = mapping != "json-string";
                    if ml_library_inequality_1 {
                        self::MlReadGrammarSelectionMappingContextLoop10Result::MlReadGrammarSelectionMappingContextLoop10Return(2f64)
                    } else {
                        {
                            let ml_index8_2 = ml_index8.add(&self::ml::Big::from_i128(1));
                            {
                                ml_index8 = ml_index8_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarSelectionMappingContextLoop10Result::MlReadGrammarSelectionMappingContextLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarSelectionMappedOffsetLoop11Result {
    MlReadGrammarSelectionMappedOffsetLoop11Done,
    MlReadGrammarSelectionMappedOffsetLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_selection_mapped_offset_loop11(
    intervals: &[f64],
    mut offset: f64,
    mut index: f64,
) -> self::MlReadGrammarSelectionMappedOffsetLoop11Result {
    loop {
        return if (index + 3f64) < (intervals.len() as f64) {
            {
                let start = (self::ml_array::optional_at(
                    intervals,
                    self::ml_array::number_index(index + 2f64),
                ))
                .unwrap_or_else(|| -1f64);
                {
                    let end = (self::ml_array::optional_at(
                        intervals,
                        self::ml_array::number_index(index + 3f64),
                    ))
                    .unwrap_or_else(|| -1f64);
                    if (start <= offset) && (offset < end) {
                        self::MlReadGrammarSelectionMappedOffsetLoop11Result::MlReadGrammarSelectionMappedOffsetLoop11Return((self::ml_array::optional_at(intervals, self::ml_array::number_index(index))).unwrap_or_else(|| -1f64))
                    } else {
                        {
                            let index_2 = index + 4f64;
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadGrammarSelectionMappedOffsetLoop11Result::MlReadGrammarSelectionMappedOffsetLoop11Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_selection_mapped_cover_loop13(
    intervals: &[Vec<f64>],
    mut end: f64,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let row = (self::ml_array::optional_at(
                        intervals,
                        self::ml_array::number_index(middle),
                    ))
                    .unwrap_or_else(|| Vec::<f64>::new());
                    if ((self::ml_array::optional_at(&row, self::ml_array::number_index(3f64)))
                        .unwrap_or_else(|| -1f64))
                        < end
                    {
                        {
                            let low_2 = middle + 1f64;
                            {
                                low = low_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let high_2 = middle;
                            {
                                high = high_2;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { low };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadGrammarSelectionMappedCoverLoop12Result {
    MlReadGrammarSelectionMappedCoverLoop12Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_grammar_selection_mapped_cover_loop12(
    intervals: &[Vec<f64>],
    mut start: f64,
    mut low: f64,
    mut high: f64,
) -> self::MlReadGrammarSelectionMappedCoverLoop12Result {
    loop {
        if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let row = (self::ml_array::optional_at(
                        intervals,
                        self::ml_array::number_index(middle),
                    ))
                    .unwrap_or_else(|| Vec::<f64>::new());
                    if ((self::ml_array::optional_at(&row, self::ml_array::number_index(3f64)))
                        .unwrap_or_else(|| -1f64))
                        <= start
                    {
                        {
                            let low_2 = middle + 1f64;
                            {
                                low = low_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let high_2 = middle;
                            {
                                high = high_2;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlReadGrammarSelectionMappedCoverLoop12Result::MlReadGrammarSelectionMappedCoverLoop12Done(low, high)
        };
    }
}
