// Generated from complete js/src/translation/unicode-case-rules.js. Do not edit by hand.
use super::accept_unicode_case_property;
use super::ml_array;
use super::ml_math;
use super::read_unicode_case_mapping;

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadUnicodeCaseMappingLoop2Result {
    MlTemporaryReadUnicodeCaseMappingLoop2Done(f64, f64),
    MlTemporaryReadUnicodeCaseMappingLoop2Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_unicode_case_mapping_loop4(
    values: &[f64],
    mut offset: f64,
    mut count: f64,
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < count {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (self::ml_array::optional_at(
                                values,
                                self::ml_array::number_index(offset + at),
                            ))
                            .unwrap_or_else(|| 0f64),
                        ],
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
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_unicode_case_mapping_loop2(
    mut point: f64,
    indexes: &[f64],
    values: &[f64],
    mut first: f64,
    mut last: f64,
) -> self::MlTemporaryReadUnicodeCaseMappingLoop2Result {
    loop {
        return if first < last {
            {
                let middle = ((first + last) / 2f64).floor();
                {
                    let key = (self::ml_array::optional_at(
                        indexes,
                        self::ml_array::number_index(middle * 3f64),
                    ))
                    .unwrap_or_else(|| -1f64);
                    if point < key {
                        {
                            let last_2 = middle;
                            {
                                last = last_2;
                                continue;
                            }
                        }
                    }
                    {
                        if point > key {
                            {
                                let first_2 = middle + 1f64;
                                {
                                    first = first_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let offset = (self::ml_array::optional_at(
                                    indexes,
                                    self::ml_array::number_index((middle * 3f64) + 1f64),
                                ))
                                .unwrap_or_else(|| 0f64);
                                {
                                    let count = (self::ml_array::optional_at(
                                        indexes,
                                        self::ml_array::number_index((middle * 3f64) + 2f64),
                                    ))
                                    .unwrap_or_else(|| 0f64);
                                    {
                                        let result = Vec::<f64>::new();
                                        {
                                            let at = 0f64;
                                            {
                                                let result_2 = self::source_ml_temporary_read_unicode_case_mapping_loop4(values, offset, count, result, at);
                                                self::MlTemporaryReadUnicodeCaseMappingLoop2Result::MlTemporaryReadUnicodeCaseMappingLoop2Return(result_2)
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
            self::MlTemporaryReadUnicodeCaseMappingLoop2Result::MlTemporaryReadUnicodeCaseMappingLoop2Done(first, last)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_map_unicode_case_utf16_columns_loop5(
    text: &[f64],
    mut points: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        return if at < (text.len() as f64) {
            {
                let first = (self::ml_array::optional_at(text, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                {
                    assert!(
                        !((!(self::ml_math::is_safe_integer(first)) || (first < 0f64))
                            || (first > 65_535f64)),
                        "{}",
                        "Unicode casing requires actual UTF16 units"
                    );

                    {
                        let second = (self::ml_array::optional_at(
                            text,
                            self::ml_array::number_index(at + 1f64),
                        ))
                        .unwrap_or_else(|| -1f64);
                        if ((((55_296f64..=56_319f64).contains(&first))
                            && self::ml_math::is_safe_integer(second))
                            && (second >= 56_320f64))
                            && (second <= 57_343f64)
                        {
                            {
                                let points_2 = {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![
                                            (((65_536f64 + ((first - 55_296f64) * 1024f64))
                                                + second)
                                                - 56_320f64),
                                        ],
                                        points,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let at_2 = at + 1f64;
                                    {
                                        let at_3 = at_2 + 1f64;
                                        {
                                            (points, at) = (points_2, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                        {
                            {
                                let points_3 = {
                                    let (ml_append_right, ml_append_left) = (vec![first], points);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let at_4 = at + 1f64;
                                    {
                                        (points, at) = (points_3, at_4);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            points
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_map_unicode_case_utf16_columns_loop6(
    cased: &[f64],
    ignorable: &[f64],
    points: &[f64],
    mut following: Vec<bool>,
    mut next: bool,
    mut at: f64,
) -> Vec<bool> {
    loop {
        if at >= 0f64 {
            {
                let point = (self::ml_array::optional_at(points, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64);
                {
                    let following_2 = {
                        let (ml_append_right, ml_append_left) = (vec![next], following);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    if !(self::accept_unicode_case_property(point, ignorable)) {
                        {
                            let next_2 = self::accept_unicode_case_property(point, cased);
                            {
                                let at_2 = at - 1f64;
                                {
                                    (following, next, at) = (following_2, next_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let at_3 = at - 1f64;
                            {
                                (following, at) = (following_2, at_3);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { following };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_map_unicode_case_utf16_columns_loop8(
    mut result: Vec<f64>,
    mapped: &[f64],
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (mapped.len() as f64) {
            {
                let value =
                    (self::ml_array::optional_at(mapped, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| 0f64);
                if value < 65_536f64 {
                    {
                        let result_2 = {
                            let (ml_append_right, ml_append_left) = (vec![value], result);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (result, index) = (result_2, index_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let result_3 = {
                            let (ml_append_right, ml_append_left) = (
                                vec![(55_296f64 + ((value - 65_536f64) / 1024f64).floor())],
                                result,
                            );
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let result_4 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![(56_320f64 + ((value - 65_536f64) % 1024f64))],
                                    result_3,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let index_3 = index + 1f64;
                                {
                                    (result, index) = (result_4, index_3);
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_map_unicode_case_utf16_columns_loop7(
    mut upper: bool,
    ranges: &[f64],
    indexes: &[f64],
    values: &[f64],
    cased: &[f64],
    ignorable: &[f64],
    points: &[f64],
    following: &[bool],
    mut result: Vec<f64>,
    mut previous: bool,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (points.len() as f64) {
            {
                let point = (self::ml_array::optional_at(points, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64);
                {
                    let final_sigma = ((!upper && (point == 931f64)) && previous)
                        && !((self::ml_array::optional_at(
                            following,
                            self::ml_array::number_index(((points.len() as f64) - 1f64) - at),
                        ))
                        .unwrap_or_else(|| false));
                    {
                        let mapped = if final_sigma {
                            vec![962f64]
                        } else {
                            self::read_unicode_case_mapping(point, ranges, indexes, values)
                        };
                        {
                            let index = 0f64;
                            {
                                let result_2 =
                                    self::source_ml_temporary_map_unicode_case_utf16_columns_loop8(
                                        result,
                                        &(mapped),
                                        index,
                                    );
                                if !(self::accept_unicode_case_property(point, ignorable)) {
                                    {
                                        let previous_2 =
                                            self::accept_unicode_case_property(point, cased);
                                        {
                                            let at_2 = at + 1f64;
                                            {
                                                (result, previous, at) =
                                                    (result_2, previous_2, at_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                                {
                                    {
                                        let at_3 = at + 1f64;
                                        {
                                            (result, at) = (result_2, at_3);
                                            continue;
                                        }
                                    }
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
