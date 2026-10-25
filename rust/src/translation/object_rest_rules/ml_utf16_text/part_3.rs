// Generated from complete js/src/translation/object-rest-rules.js. Do not edit by hand.
use super::index_utf16_text;
use super::ml_array;

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_display_utf8_from_utf16_text_loop22(
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanUtf16TextSubstitutionLoop23Result {
    MlTemporaryPlanUtf16TextSubstitutionLoop23Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_utf16_text_substitution_loop23(
    replacement: &[f64],
    mut result: Vec<f64>,
    mut start: f64,
    mut at: f64,
) -> self::MlTemporaryPlanUtf16TextSubstitutionLoop23Result {
    loop {
        if at < (replacement.len() as f64) {
            {
                let next = (self::ml_array::optional_at(
                    replacement,
                    self::ml_array::number_index(at + 1f64),
                ))
                .unwrap_or_else(|| -1f64);
                if (((self::ml_array::optional_at(replacement, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| 0f64))
                    == 36f64)
                    && ((((next == 36f64) || (next == 38f64)) || (next == 96f64))
                        || (next == 39f64))
                {
                    {
                        let result_3 = if start < at {
                            {
                                {
                                    let (ml_append_right, ml_append_left) =
                                        (vec![0f64, start, at], result);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                }
                            }
                        } else {
                            result
                        };
                        {
                            let result_6 = if next == 36f64 {
                                {
                                    {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![0f64, at, (at + 1f64)], result_3);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    }
                                }
                            } else {
                                {
                                    {
                                        let (ml_append_right, ml_append_left) = (
                                            vec![
                                                if next == 38f64 {
                                                    1f64
                                                } else if next == 96f64 {
                                                    2f64
                                                } else {
                                                    3f64
                                                },
                                                0f64,
                                                0f64,
                                            ],
                                            result_3,
                                        );
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    }
                                }
                            };
                            {
                                let at_2 = at + 2f64;
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
                }
                {
                    {
                        let at_3 = at + 1f64;
                        {
                            at = at_3;
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryPlanUtf16TextSubstitutionLoop23Result::MlTemporaryPlanUtf16TextSubstitutionLoop23Done(result, start)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop25(
    text: &[f64],
    mut result: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < (text.len() as f64) {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (self::ml_array::optional_at(text, self::ml_array::number_index(at)))
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReplaceUtf16TextLoop24Result {
    MlTemporaryReplaceUtf16TextLoop24Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop26(
    text: &[f64],
    mut result: Vec<f64>,
    mut found: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < found {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![
                            (self::ml_array::optional_at(text, self::ml_array::number_index(at)))
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
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop28(
    text: &[f64],
    replacement: &[f64],
    mut result: Vec<f64>,
    mut kind: f64,
    mut last: f64,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < last {
            {
                let result_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![if kind == 0f64 {
                            (self::ml_array::optional_at(
                                replacement,
                                self::ml_array::number_index(at),
                            ))
                            .unwrap_or_else(|| 0f64)
                        } else {
                            (self::ml_array::optional_at(text, self::ml_array::number_index(at)))
                                .unwrap_or_else(|| 0f64)
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
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_replace_utf16_text_loop27(
    text: &[f64],
    search: &[f64],
    replacement: &[f64],
    mut result: Vec<f64>,
    plan: &[f64],
    mut found: f64,
    mut instruction: f64,
) -> Vec<f64> {
    loop {
        if instruction < (plan.len() as f64) {
            {
                let kind =
                    (self::ml_array::optional_at(plan, self::ml_array::number_index(instruction)))
                        .unwrap_or_else(|| 0f64);
                {
                    let first = if kind == 0f64 {
                        (self::ml_array::optional_at(
                            plan,
                            self::ml_array::number_index(instruction + 1f64),
                        ))
                        .unwrap_or_else(|| 0f64)
                    } else if kind == 2f64 {
                        0f64
                    } else if kind == 1f64 {
                        found
                    } else {
                        found + (search.len() as f64)
                    };
                    {
                        let last = if kind == 0f64 {
                            (self::ml_array::optional_at(
                                plan,
                                self::ml_array::number_index(instruction + 2f64),
                            ))
                            .unwrap_or_else(|| 0f64)
                        } else if kind == 2f64 {
                            found
                        } else if kind == 1f64 {
                            found + (search.len() as f64)
                        } else {
                            text.len() as f64
                        };
                        {
                            let at = first;
                            {
                                let result_2 = self::source_ml_temporary_replace_utf16_text_loop28(
                                    text,
                                    replacement,
                                    result,
                                    kind,
                                    last,
                                    at,
                                );
                                {
                                    let instruction_2 = instruction + 3f64;
                                    {
                                        (result, instruction) = (result_2, instruction_2);
                                        continue;
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

/// Execute the checked shared source algorithm.
#[must_use]
/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_replace_utf16_text_loop24(
    text: &[f64],
    search: &[f64],
    replacement: &[f64],
    mut all: bool,
    mut result: Vec<f64>,
    plan: &[f64],
    mut start: f64,
    mut from: f64,
) -> self::MlTemporaryReplaceUtf16TextLoop24Result {
    loop {
        return {
            let found = self::index_utf16_text(text, search, from);
            if found < 0f64 {
                self::MlTemporaryReplaceUtf16TextLoop24Result::MlTemporaryReplaceUtf16TextLoop24Done(
                    result, start,
                )
            } else {
                {
                    let at = start;
                    {
                        let result_2 = self::source_ml_temporary_replace_utf16_text_loop26(
                            text, result, found, at,
                        );
                        {
                            let instruction = 0f64;
                            {
                                let result_3 = self::source_ml_temporary_replace_utf16_text_loop27(
                                    text,
                                    search,
                                    replacement,
                                    result_2,
                                    plan,
                                    found,
                                    instruction,
                                );
                                {
                                    let start_2 = found + (search.len() as f64);
                                    if !all
                                        || (((search.len() as f64) == 0f64)
                                            && (found == (text.len() as f64)))
                                    {
                                        self::MlTemporaryReplaceUtf16TextLoop24Result::MlTemporaryReplaceUtf16TextLoop24Done(result_3, start_2)
                                    } else {
                                        {
                                            let from_2 = start_2
                                                + if (search.len() as f64) == 0f64 {
                                                    1f64
                                                } else {
                                                    0f64
                                                };
                                            {
                                                (result, start, from) = (result_3, start_2, from_2);
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
        };
    }
}
