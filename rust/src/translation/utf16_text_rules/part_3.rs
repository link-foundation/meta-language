// Generated from complete js/src/translation/utf16-text-rules.js. Do not edit by hand.
use super::MlTemporaryCodePointUnitsUtf16TextLoop21Result;
use super::ml_array;
use super::ml_math;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_code_point_units_utf16_text_loop21(
    values: &[f64],
    mut result: Vec<f64>,
    mut at: f64,
) -> self::MlTemporaryCodePointUnitsUtf16TextLoop21Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(value)) || (value < 0f64)) || (value > 1_114_111f64)
                {
                    self::MlTemporaryCodePointUnitsUtf16TextLoop21Result::MlTemporaryCodePointUnitsUtf16TextLoop21Return(vec![at])
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
            self::MlTemporaryCodePointUnitsUtf16TextLoop21Result::MlTemporaryCodePointUnitsUtf16TextLoop21Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryDisplayPointsUtf16TextJoin23 {
    MlTemporaryDisplayPointsUtf16TextJoin23Next(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_display_points_utf16_text_loop22(
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
                    let source_ml_source1_s13 = if (55_296f64..=56_319f64).contains(&point) {
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
                                        self::MlTemporaryDisplayPointsUtf16TextJoin23::MlTemporaryDisplayPointsUtf16TextJoin23Next(at_2, point_2)
                                    }
                                }
                            } else {
                                {
                                    let point_3 = 65_533f64;
                                    self::MlTemporaryDisplayPointsUtf16TextJoin23::MlTemporaryDisplayPointsUtf16TextJoin23Next(at, point_3)
                                }
                            }
                        }
                    } else if (56_320f64..=57_343f64).contains(&point) {
                        {
                            let point_4 = 65_533f64;
                            self::MlTemporaryDisplayPointsUtf16TextJoin23::MlTemporaryDisplayPointsUtf16TextJoin23Next(at, point_4)
                        }
                    } else {
                        self::MlTemporaryDisplayPointsUtf16TextJoin23::MlTemporaryDisplayPointsUtf16TextJoin23Next(at, point)
                    };
                    match source_ml_source1_s13 {
                        self::MlTemporaryDisplayPointsUtf16TextJoin23::MlTemporaryDisplayPointsUtf16TextJoin23Next(at_3, point_5) => {
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
pub fn source_ml_temporary_display_utf8_from_utf16_text_loop24(
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
