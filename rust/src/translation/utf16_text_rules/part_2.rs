// Generated from complete js/src/translation/utf16-text-rules.js. Do not edit by hand.
use super::accept_java_script_whitespace_unit;
use super::match_utf16_text;
use super::ml_array;
use super::ml_math;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_points_utf16_text_loop6(
    text: &[f64],
    mut result: Vec<Vec<f64>>,
    mut at: f64,
) -> Vec<Vec<f64>> {
    loop {
        if at < (text.len() as f64) {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64);
                {
                    let second = (self::ml_array::optional_at(
                        text,
                        self::ml_array::number_index(at + 1f64),
                    ))
                    .unwrap_or_else(|| -1f64);
                    if (((55_296f64..=56_319f64).contains(&first)) && (second >= 56_320f64))
                        && (second <= 57_343f64)
                    {
                        {
                            let result_2 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![vec![first, second]], result);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_2 = at + 1f64;
                                {
                                    let at_3 = at_2 + 1f64;
                                    {
                                        (result, at) = (result_2, at_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    {
                        {
                            let result_3 = {
                                let (ml_append_right, ml_append_left) = (vec![vec![first]], result);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_4 = at + 1f64;
                                {
                                    (result, at) = (result_3, at_4);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryMatchUtf16TextLoop7Result {
    MlTemporaryMatchUtf16TextLoop7Done,
    MlTemporaryMatchUtf16TextLoop7Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_match_utf16_text_loop7(
    text: &[f64],
    search: &[f64],
    mut start: f64,
    mut at: f64,
) -> self::MlTemporaryMatchUtf16TextLoop7Result {
    loop {
        return if at < (search.len() as f64) {
            if (match (
                self::ml_array::optional_at(text, self::ml_array::number_index(start + at)),
                self::ml_array::optional_at(search, self::ml_array::number_index(at)),
            ) {
                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                (None, None) => true,
                _ => false,
            }) {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
            {
                self::MlTemporaryMatchUtf16TextLoop7Result::MlTemporaryMatchUtf16TextLoop7Return(
                    false,
                )
            }
        } else {
            self::MlTemporaryMatchUtf16TextLoop7Result::MlTemporaryMatchUtf16TextLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryIndexUtf16TextLoop8Result {
    MlTemporaryIndexUtf16TextLoop8Done,
    MlTemporaryIndexUtf16TextLoop8Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_index_utf16_text_loop8(
    text: &[f64],
    search: &[f64],
    mut at: f64,
) -> self::MlTemporaryIndexUtf16TextLoop8Result {
    loop {
        return if at <= ((text.len() as f64) - (search.len() as f64)) {
            if self::match_utf16_text(text, search, at) {
                self::MlTemporaryIndexUtf16TextLoop8Result::MlTemporaryIndexUtf16TextLoop8Return(at)
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryIndexUtf16TextLoop8Result::MlTemporaryIndexUtf16TextLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLastIndexUtf16TextLoop9Result {
    MlTemporaryLastIndexUtf16TextLoop9Done,
    MlTemporaryLastIndexUtf16TextLoop9Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_last_index_utf16_text_loop9(
    text: &[f64],
    search: &[f64],
    mut at: f64,
) -> self::MlTemporaryLastIndexUtf16TextLoop9Result {
    loop {
        return if at >= 0f64 {
            if self::match_utf16_text(text, search, at) {
                self::MlTemporaryLastIndexUtf16TextLoop9Result::MlTemporaryLastIndexUtf16TextLoop9Return(at)
            } else {
                {
                    let at_2 = at - 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryLastIndexUtf16TextLoop9Result::MlTemporaryLastIndexUtf16TextLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareUtf16TextLoop10Result {
    MlTemporaryCompareUtf16TextLoop10Done,
    MlTemporaryCompareUtf16TextLoop10Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_utf16_text_loop10(
    left: &[f64],
    right: &[f64],
    mut count: f64,
    mut at: f64,
) -> self::MlTemporaryCompareUtf16TextLoop10Result {
    loop {
        return if at < count {
            {
                let a = (self::ml_array::optional_at(left, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(right, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| 0f64);
                    {
                        let ml_library_inequality_1 = a != b;
                        if ml_library_inequality_1 {
                            self::MlTemporaryCompareUtf16TextLoop10Result::MlTemporaryCompareUtf16TextLoop10Return(if a < b {
                            -1f64
                        } else {
                            1f64
                        })
                        } else {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompareUtf16TextLoop10Result::MlTemporaryCompareUtf16TextLoop10Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop12(
    separator: &[f64],
    mut result: Vec<f64>,
    mut offset: f64,
) -> Vec<f64> {
    loop {
        if offset < (separator.len() as f64) {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (self::ml_array::optional_at(
                                separator,
                                self::ml_array::number_index(offset),
                            ))
                            .unwrap_or_else(|| 0f64),
                        ],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let offset_2 = offset + 1f64;
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop13(
    mut result: Vec<f64>,
    value: &[f64],
    mut offset: f64,
) -> Vec<f64> {
    loop {
        if offset < (value.len() as f64) {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (self::ml_array::optional_at(
                                value,
                                self::ml_array::number_index(offset),
                            ))
                            .unwrap_or_else(|| 0f64),
                        ],
                        result,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let offset_2 = offset + 1f64;
                    {
                        (result, offset) = (result_2, offset_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_join_utf16_text_loop11(
    values: &[Vec<f64>],
    separator: &[f64],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (values.len() as f64) {
            {
                let result_3 = if at == 0f64 {
                    result
                } else {
                    {
                        let offset = 0f64;
                        {
                            self::source_ml_temporary_join_utf16_text_loop12(
                                separator, result, offset,
                            )
                        }
                    }
                };
                {
                    let value =
                        (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                            .unwrap_or_else(|| Vec::<f64>::new());
                    {
                        let offset_2 = 0f64;
                        {
                            let result_4 = self::source_ml_temporary_join_utf16_text_loop13(
                                result_3,
                                &(value),
                                offset_2,
                            );
                            {
                                let at_2 = at + 1f64;
                                {
                                    (result, at) = (result_4, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_trim_utf16_text_loop14(
    text: &[f64],
    mut first: f64,
    mut last: f64,
) -> f64 {
    loop {
        if (first < last)
            && self::accept_java_script_whitespace_unit(
                (self::ml_array::optional_at(text, self::ml_array::number_index(first)))
                    .unwrap_or_else(|| -1f64),
            )
        {
            {
                let first_2 = first + 1f64;
                {
                    first = first_2;
                    continue;
                }
            }
        }
        return { first };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_trim_utf16_text_loop15(
    text: &[f64],
    mut first: f64,
    mut last: f64,
) -> f64 {
    loop {
        if (last > first)
            && self::accept_java_script_whitespace_unit(
                (self::ml_array::optional_at(text, self::ml_array::number_index(last - 1f64)))
                    .unwrap_or_else(|| -1f64),
            )
        {
            {
                let last_2 = last - 1f64;
                {
                    last = last_2;
                    continue;
                }
            }
        }
        return { last };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_character_units_utf16_text_loop16(
    values: &[f64],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (values.len() as f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64);
                {
                    let integer = if value.is_finite() {
                        value.trunc()
                    } else {
                        0f64
                    };
                    {
                        let residue = integer % 65_536f64;
                        {
                            let result_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![if residue < 0f64 {
                                        residue + 65_536f64
                                    } else if residue == 0f64 {
                                        0f64
                                    } else {
                                        residue
                                    }],
                                    result,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_2 = at + 1f64;
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCodePointUnitsUtf16TextLoop17Result {
    MlTemporaryCodePointUnitsUtf16TextLoop17Done(Vec<f64>),
    MlTemporaryCodePointUnitsUtf16TextLoop17Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_code_point_units_utf16_text_loop17(
    values: &[f64],
    mut result: Vec<f64>,
    mut at: f64,
) -> self::MlTemporaryCodePointUnitsUtf16TextLoop17Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(value)) || (value < 0f64)) || (value > 1_114_111f64)
                {
                    self::MlTemporaryCodePointUnitsUtf16TextLoop17Result::MlTemporaryCodePointUnitsUtf16TextLoop17Return(vec![at])
                } else {
                    if value < 65_536f64 {
                        {
                            let result_2 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![if value == 0f64 { 0f64 } else { value }], result);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_2 = at + 1f64;
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let result_3 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        (55_296f64 + ((value - 65_536f64) / 1024f64).floor()),
                                        (56_320f64 + ((value - 65_536f64) % 1024f64)),
                                    ],
                                    result,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_3 = at + 1f64;
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
            self::MlTemporaryCodePointUnitsUtf16TextLoop17Result::MlTemporaryCodePointUnitsUtf16TextLoop17Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDisplayPointsUtf16TextJoin19 {
    MlTemporaryDisplayPointsUtf16TextJoin19Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_display_points_utf16_text_loop18(
    text: &[f64],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        return if at < (text.len() as f64) {
            {
                let point = (self::ml_array::optional_at(text, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 65_533f64);
                {
                    let source_ml_source1_s10 = if (55_296f64..=56_319f64).contains(&point) {
                        {
                            let low = (self::ml_array::optional_at(
                                text,
                                self::ml_array::number_index(at + 1f64),
                            ))
                            .unwrap_or_else(|| -1f64);
                            if (56_320f64..=57_343f64).contains(&low) {
                                {
                                    let point_2 = ((65_536f64 + ((point - 55_296f64) * 1024f64))
                                        + low)
                                        - 56_320f64;
                                    {
                                        let at_2 = at + 1f64;
                                        self::MlTemporaryDisplayPointsUtf16TextJoin19::MlTemporaryDisplayPointsUtf16TextJoin19Next(at_2, point_2)
                                    }
                                }
                            } else {
                                {
                                    let point_3 = 65_533f64;
                                    self::MlTemporaryDisplayPointsUtf16TextJoin19::MlTemporaryDisplayPointsUtf16TextJoin19Next(at, point_3)
                                }
                            }
                        }
                    } else if (56_320f64..=57_343f64).contains(&point) {
                        {
                            let point_4 = 65_533f64;
                            self::MlTemporaryDisplayPointsUtf16TextJoin19::MlTemporaryDisplayPointsUtf16TextJoin19Next(at, point_4)
                        }
                    } else {
                        self::MlTemporaryDisplayPointsUtf16TextJoin19::MlTemporaryDisplayPointsUtf16TextJoin19Next(at, point)
                    };
                    match source_ml_source1_s10 {
                        self::MlTemporaryDisplayPointsUtf16TextJoin19::MlTemporaryDisplayPointsUtf16TextJoin19Next(at_3, point_5) => {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![point_5], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = at_3 + 1f64;
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_display_utf8_from_utf16_text_loop20(
    mut bytes: Vec<f64>,
    points: &[f64],
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (points.len() as f64) {
            {
                let point = (self::ml_array::optional_at(points, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 65_533f64);
                if point < 128f64 {
                    {
                        let bytes_2 = {
                            let (ml_append_right, ml_append_left) = (vec![point], bytes);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                (bytes, at) = (bytes_2, at_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    if point < 2048f64 {
                        {
                            let bytes_3 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        (192f64 + (point / 64f64).floor()),
                                        (128f64 + (point % 64f64)),
                                    ],
                                    bytes,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let at_3 = at + 1f64;
                                {
                                    (bytes, at) = (bytes_3, at_3);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if point < 65_536f64 {
                            {
                                let bytes_4 = {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![
                                            (224f64 + (point / 4096f64).floor()),
                                            (128f64 + ((point / 64f64).floor() % 64f64)),
                                            (128f64 + (point % 64f64)),
                                        ],
                                        bytes,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let at_4 = at + 1f64;
                                    {
                                        (bytes, at) = (bytes_4, at_4);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let bytes_5 = {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![
                                            (240f64 + (point / 262_144f64).floor()),
                                            (128f64 + ((point / 4096f64).floor() % 64f64)),
                                            (128f64 + ((point / 64f64).floor() % 64f64)),
                                            (128f64 + (point % 64f64)),
                                        ],
                                        bytes,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let at_5 = at + 1f64;
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
        }
        return { bytes };
    }
}
