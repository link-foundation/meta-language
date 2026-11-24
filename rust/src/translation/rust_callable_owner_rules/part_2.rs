// Generated from complete js/src/translation/rust-callable-owner-rules.js. Do not edit by hand.
use super::MlTemporaryReadRustCallableOwnerErrorLoop6Result;
use super::ml_array;
use super::read_rust_pointer_integer_owner_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_callable_owner_error_loop6(
    actual: &[String],
    expected: &[String],
    mut index: f64,
) -> self::MlTemporaryReadRustCallableOwnerErrorLoop6Result {
    loop {
        return if index < (expected.len() as f64) {
            {
                let path =
                    (self::ml_array::optional_at(expected, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                {
                    let owner = (self::ml_array::optional_at(
                        expected,
                        self::ml_array::number_index(index + 1f64),
                    ))
                    .unwrap_or_else(|| String::new());
                    if {
                        let source_ml_source1_comparison_operand7 = self::ml_array::optional_at(
                            actual,
                            self::ml_array::number_index(index),
                        );
                        {
                            let source_ml_source1_comparison_operand8 = path;
                            !(match (
                                source_ml_source1_comparison_operand7,
                                source_ml_source1_comparison_operand8,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    } {
                        self::MlTemporaryReadRustCallableOwnerErrorLoop6Result::MlTemporaryReadRustCallableOwnerErrorLoop6Return(String::from("the actual callable needs its complete structural source signature"))
                    } else {
                        {
                            let other = (self::ml_array::optional_at(
                                actual,
                                self::ml_array::number_index(index + 1f64),
                            ))
                            .unwrap_or_else(|| String::new());
                            {
                                let error =
                                    self::read_rust_pointer_integer_owner_error(&(other), &(owner));
                                if error.is_empty() {
                                    if (owner != "") && (other == "") {
                                        self::MlTemporaryReadRustCallableOwnerErrorLoop6Result::MlTemporaryReadRustCallableOwnerErrorLoop6Return(String::from("the actual callable needs complete parameter and result source owners"))
                                    } else {
                                        {
                                            let index_2 = index + 2f64;
                                            {
                                                index = index_2;
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    self::MlTemporaryReadRustCallableOwnerErrorLoop6Result::MlTemporaryReadRustCallableOwnerErrorLoop6Return(error.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCallableOwnerErrorLoop6Result::MlTemporaryReadRustCallableOwnerErrorLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustCallableOwnerDescriptorsEqualLoop7Result {
    MlTemporaryAcceptRustCallableOwnerDescriptorsEqualLoop7Done,
    MlTemporaryAcceptRustCallableOwnerDescriptorsEqualLoop7Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_callable_owner_descriptors_equal_loop7(
    left: &[String],
    right: &[String],
    mut at: f64,
) -> self::MlTemporaryAcceptRustCallableOwnerDescriptorsEqualLoop7Result {
    loop {
        return if at < (left.len() as f64) {
            if {
                let source_ml_source1_comparison_operand9 =
                    self::ml_array::optional_at(left, self::ml_array::number_index(at));
                {
                    let source_ml_source1_comparison_operand10 =
                        self::ml_array::optional_at(right, self::ml_array::number_index(at));
                    !(match (
                        source_ml_source1_comparison_operand9,
                        source_ml_source1_comparison_operand10,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryAcceptRustCallableOwnerDescriptorsEqualLoop7Result::MlTemporaryAcceptRustCallableOwnerDescriptorsEqualLoop7Return(false)
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
            self::MlTemporaryAcceptRustCallableOwnerDescriptorsEqualLoop7Result::MlTemporaryAcceptRustCallableOwnerDescriptorsEqualLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCallableOwnerJoinErrorLoop8Result {
    MlTemporaryReadRustCallableOwnerJoinErrorLoop8Done,
    MlTemporaryReadRustCallableOwnerJoinErrorLoop8Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustCallableOwnerJoinErrorLoop9Result {
    MlTemporaryReadRustCallableOwnerJoinErrorLoop9Done,
    MlTemporaryReadRustCallableOwnerJoinErrorLoop9Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_callable_owner_join_error_loop9(
    descriptors: &[String],
    mut index: f64,
    owner: &str,
    mut before: f64,
) -> self::MlTemporaryReadRustCallableOwnerJoinErrorLoop9Result {
    loop {
        return if before < index {
            if {
                let source_ml_source1_comparison_operand11 =
                    self::ml_array::optional_at(descriptors, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand12 = self::ml_array::optional_at(
                        descriptors,
                        self::ml_array::number_index(before),
                    );
                    match (
                        source_ml_source1_comparison_operand11,
                        source_ml_source1_comparison_operand12,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } {
                {
                    let error = self::read_rust_pointer_integer_owner_error(
                        owner,
                        &((self::ml_array::optional_at(
                            descriptors,
                            self::ml_array::number_index(before + 1f64),
                        ))
                        .unwrap_or_else(|| String::new())),
                    );
                    if error.is_empty() {
                        {
                            let before_2 = before + 2f64;
                            {
                                before = before_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadRustCallableOwnerJoinErrorLoop9Result::MlTemporaryReadRustCallableOwnerJoinErrorLoop9Return(error.clone())
                    }
                }
            } else {
                {
                    let before_3 = before + 2f64;
                    {
                        before = before_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCallableOwnerJoinErrorLoop9Result::MlTemporaryReadRustCallableOwnerJoinErrorLoop9Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_callable_owner_join_error_loop8(
    descriptors: &[String],
    mut index: f64,
) -> self::MlTemporaryReadRustCallableOwnerJoinErrorLoop8Result {
    loop {
        return if index < (descriptors.len() as f64) {
            {
                let owner = (self::ml_array::optional_at(
                    descriptors,
                    self::ml_array::number_index(index + 1f64),
                ))
                .unwrap_or_else(|| String::new());
                if !(self::ml_array::includes(
                    &vec![
                        String::from("usize"),
                        String::from("u64"),
                        String::from("isize"),
                        String::from("i64"),
                    ],
                    &owner.clone(),
                    0f64,
                    false,
                    |a, b| a == b,
                )) {
                    {
                        let index_2 = index + 2f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let before = 0f64;
                        {
                            let source_ml_source1_s13 =
                                self::source_ml_temporary_read_rust_callable_owner_join_error_loop9(
                                    descriptors,
                                    index,
                                    &(owner),
                                    before,
                                );
                            match source_ml_source1_s13 {
                                self::MlTemporaryReadRustCallableOwnerJoinErrorLoop9Result::MlTemporaryReadRustCallableOwnerJoinErrorLoop9Done => {
                                    {
                                        let index_3 = index + 2f64;
                                        {
                                            index = index_3;
                                            continue;
                                        }
                                    }
                                }
                                self::MlTemporaryReadRustCallableOwnerJoinErrorLoop9Result::MlTemporaryReadRustCallableOwnerJoinErrorLoop9Return(source_ml_temporary_result9) => {
                                    self::MlTemporaryReadRustCallableOwnerJoinErrorLoop8Result::MlTemporaryReadRustCallableOwnerJoinErrorLoop8Return(source_ml_temporary_result9.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustCallableOwnerJoinErrorLoop8Result::MlTemporaryReadRustCallableOwnerJoinErrorLoop8Done
        };
    }
}
