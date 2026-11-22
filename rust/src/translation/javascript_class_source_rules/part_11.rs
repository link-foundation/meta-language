// Generated from complete js/src/translation/javascript-class-source-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result;
use super::ml_array;
use super::ml_math;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_closed_class_own_field_loop68(
    receiver_fields: &[self::ml_string::Text],
    public_fields: &[self::ml_string::Text],
    field: &self::ml_string::Text,
    mut at: f64,
) -> self::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result {
    loop {
        return if at < (public_fields.len() as f64) {
            if self::ml_array::includes(
                receiver_fields,
                &(self::ml_array::optional_at(public_fields, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                0f64,
                false,
                |a, b| a == b,
            ) && (((self::ml_array::optional_at(
                public_fields,
                self::ml_array::number_index(at + 1f64),
            ))
            .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])))
                == (*field))
            {
                self::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Return(true)
            } else {
                {
                    let at_2 = at + 2f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Result::MlTemporaryAcceptJavaScriptClosedClassOwnFieldLoop68Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryProjectJavaScriptClassSourceIntervalLoop69Result {
    MlTemporaryProjectJavaScriptClassSourceIntervalLoop69Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_project_java_script_class_source_interval_loop69(
    intervals: &[f64],
    mut first: f64,
    mut last: f64,
    mut low: f64,
    mut high: f64,
    mut at: f64,
) -> self::MlTemporaryProjectJavaScriptClassSourceIntervalLoop69Result {
    loop {
        if at < (intervals.len() as f64) {
            {
                let start =
                    (self::ml_array::optional_at(intervals, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| 0f64);
                {
                    let end = (self::ml_array::optional_at(
                        intervals,
                        self::ml_array::number_index(at + 1f64),
                    ))
                    .unwrap_or_else(|| 0f64);
                    if ((end < first) || (start > last)) || (start == end) {
                        {
                            let at_2 = at + 4f64;
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let original_start = (self::ml_array::optional_at(
                                intervals,
                                self::ml_array::number_index(at + 2f64),
                            ))
                            .unwrap_or_else(|| 0f64);
                            {
                                let original_end = (self::ml_array::optional_at(
                                    intervals,
                                    self::ml_array::number_index(at + 3f64),
                                ))
                                .unwrap_or_else(|| 0f64);
                                {
                                    let low_2 = if low < 0f64 {
                                        original_start
                                    } else {
                                        self::ml_math::min(low, original_start)
                                    };
                                    {
                                        let high_2 = self::ml_math::max(high, original_end);
                                        {
                                            let at_3 = at + 4f64;
                                            {
                                                (low, high, at) = (low_2, high_2, at_3);
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
        }
        return {
            self::MlTemporaryProjectJavaScriptClassSourceIntervalLoop69Result::MlTemporaryProjectJavaScriptClassSourceIntervalLoop69Done(low, high)
        };
    }
}
