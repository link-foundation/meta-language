// Generated from complete js/src/translation/rust-unit-value-rules.js. Do not edit by hand.
use super::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result;
use super::ml_array;
use super::source_ml_temporary_find_rust_terminal_return_boundary_loop4;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_rust_terminal_return_boundary_loop3(
    values: &[String],
    mut stack: Vec<String>,
    mut at: f64,
) -> self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result {
    loop {
        return if at < (values.len() as f64) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| String::new());
                if ((value == "(") || (value == "[")) || (value == "{") {
                    {
                        let stack_2 = {
                            let (ml_append_right, ml_append_left) = (vec![value.clone()], stack);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let at_2 = at + 1f64;
                            {
                                (stack, at) = (stack_2, at_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    if ((stack.len() as f64) == 0f64) && ((value == ";") || (value == "}")) {
                        {
                            let end = at;
                            {
                                let end_2 = self::source_ml_temporary_find_rust_terminal_return_boundary_loop4(values, end);
                                if end_2 >= (values.len() as f64) {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(-2f64)
                                } else if {
                                    let source_ml_source1_comparison_operand13 =
                                        self::ml_array::optional_at(
                                            values,
                                            self::ml_array::number_index(end_2),
                                        );
                                    {
                                        let source_ml_source1_comparison_operand14 =
                                            String::from("}");
                                        match (
                                            source_ml_source1_comparison_operand13,
                                            source_ml_source1_comparison_operand14,
                                        ) {
                                            (Some(ml_left), ml_right) => ml_left == ml_right,
                                            _ => false,
                                        }
                                    }
                                } {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(end_2)
                                } else {
                                    self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(-1f64)
                                }
                            }
                        }
                    } else if ((value == ")") || (value == "]")) || (value == "}") {
                        {
                            let previous = (self::ml_array::optional_at(
                                &stack,
                                self::ml_array::number_index((stack.len() as f64) - 1f64),
                            ))
                            .unwrap_or_else(|| String::new());
                            if (((value == ")") && (previous != "("))
                                || ((value == "]") && (previous != "[")))
                                || ((value == "}") && (previous != "{"))
                            {
                                self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Return(-1f64)
                            } else {
                                {
                                    let stack_3 = self::ml_array::slice(&stack, 0f64, -1f64);
                                    {
                                        let at_3 = at + 1f64;
                                        {
                                            (stack, at) = (stack_3, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let at_4 = at + 1f64;
                            {
                                at = at_4;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindRustTerminalReturnBoundaryLoop3Result::MlTemporaryFindRustTerminalReturnBoundaryLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustFunctionCompletionPresenceLoop5Result {
    MlTemporaryReadRustFunctionCompletionPresenceLoop5Done,
    MlTemporaryReadRustFunctionCompletionPresenceLoop5Return(f64),
}

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
