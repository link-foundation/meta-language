// Generated from complete js/src/translation/rust-unit-value-rules.js. Do not edit by hand.
use super::MlTemporaryReadRustFunctionCompletionPresenceLoop5Result;
use super::ml_array;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_function_completion_presence_loop5(
    values: &[String],
    mut depth: f64,
    mut index: f64,
) -> self::MlTemporaryReadRustFunctionCompletionPresenceLoop5Result {
    loop {
        return if index < (values.len() as f64) {
            {
                let value =
                    (self::ml_array::optional_at(values, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                if (value == "return") || (value == "abort") {
                    self::MlTemporaryReadRustFunctionCompletionPresenceLoop5Result::MlTemporaryReadRustFunctionCompletionPresenceLoop5Return(1f64)
                } else {
                    {
                        let depth_3 = if value == "{" {
                            { depth + 1f64 }
                        } else {
                            depth
                        };
                        if value == "}" {
                            {
                                let depth_4 = depth_3 - 1f64;
                                if depth_4 == 0f64 {
                                    self::MlTemporaryReadRustFunctionCompletionPresenceLoop5Result::MlTemporaryReadRustFunctionCompletionPresenceLoop5Return(0f64)
                                } else {
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (depth, index) = (depth_4, index_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let index_3 = index + 1f64;
                                {
                                    (depth, index) = (depth_3, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustFunctionCompletionPresenceLoop5Result::MlTemporaryReadRustFunctionCompletionPresenceLoop5Done
        };
    }
}
