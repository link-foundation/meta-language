// Generated from complete js/src/translation/rust-standard-sum-ownership-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_owned_field_production_error(
    owned: bool,
    transferred: bool,
    available: bool,
    producer: bool,
) -> String {
    if ((!owned || transferred) || available) || producer {
        String::new()
    } else {
        String::from(
            "an owned constructor field needs its actual available binding or source producer contract",
        )
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_owned_field_disposition(
    owned: bool,
    borrowed: bool,
    field: bool,
) -> String {
    if owned {
        if borrowed {
            String::from("borrow")
        } else if field {
            String::from("move")
        } else {
            String::from("plain")
        }
    } else {
        String::from("plain")
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_owned_guard_error(owned: bool, guarded: bool) -> String {
    if owned && guarded {
        String::from("owned sum guards need their delayed payload-move and fallthrough model")
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub const fn accept_rust_standard_sum_owned_call_result(
    owned: bool,
    function_: bool,
    call: bool,
) -> bool {
    (owned && function_) && call
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_owned_pattern_transfer_error(
    required: bool,
    binding: bool,
    producer: bool,
) -> String {
    if (required && !binding) && !producer {
        String::from(
            "an unbound owned pattern field needs its actual fresh parent producer contract",
        )
    } else {
        String::new()
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_ownership_node_limit() -> f64 {
    100_000f64
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_standard_sum_ownership_depth_limit() -> f64 {
    256f64
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(f64),
    MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_source_depth_loop2(
    mut depth: f64,
    source_ml_source1_temporary_values1: &[f64],
    mut source_ml_source1_temporary_index1: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result {
    loop {
        return if source_ml_source1_temporary_index1
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values1.len() as u128))
        {
            {
                let child = self::ml_array::at(
                    source_ml_source1_temporary_values1,
                    source_ml_source1_temporary_index1.to_index(),
                );
                if (!(self::ml_math::is_integer(child)) || (child < 0f64)) || (child > 16f64) {
                    self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Return(-1f64)
                } else {
                    if child > depth {
                        {
                            let depth_2 = child;
                            {
                                let source_ml_source1_temporary_index1_2 =
                                    source_ml_source1_temporary_index1
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (depth, source_ml_source1_temporary_index1) =
                                        (depth_2, source_ml_source1_temporary_index1_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_source1_temporary_index1_3 =
                                source_ml_source1_temporary_index1
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1 =
                                    source_ml_source1_temporary_index1_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Result::MlSource1TemporaryReadRustStandardSumSourceDepthLoop2Done(depth)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done,
    MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_source_materialization_error_loop4(
    source_ml_source1_temporary_values3: &[f64],
    mut source_ml_source1_temporary_index3: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result {
    loop {
        return if source_ml_source1_temporary_index3
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values3.len() as u128))
        {
            {
                let value = self::ml_array::at(
                    source_ml_source1_temporary_values3,
                    source_ml_source1_temporary_index3.to_index(),
                );
                if !(self::ml_math::is_integer(value)) || (value < 0f64) {
                    self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Return(String::from("invalid standard sum source materialization count"))
                } else {
                    {
                        let source_ml_source1_temporary_index3_2 =
                            source_ml_source1_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index3 =
                                source_ml_source1_temporary_index3_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Result::MlSource1TemporaryReadRustStandardSumSourceMaterializationErrorLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done,
    MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_standard_sum_source_arguments_loop5(
    identities: &[String],
    direct_copies: &[bool],
    mut count: f64,
    mut at: f64,
) -> self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result {
    loop {
        return if at < count {
            if (((self::ml_array::optional_at(identities, self::ml_array::number_index(at)))
                .unwrap_or_else(|| String::new()))
                == "")
                || {
                    let source_ml_source2_comparison_operand24 = self::ml_array::optional_at(
                        direct_copies,
                        self::ml_array::number_index(at),
                    );
                    {
                        let source_ml_source2_comparison_operand25 = true;
                        !(match (
                            source_ml_source2_comparison_operand24,
                            source_ml_source2_comparison_operand25,
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    }
                }
            {
                self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Return(false)
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
            self::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Result::MlSource1TemporaryAcceptRustStandardSumSourceArgumentsLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustStandardSumOwnershipProfileLoop7Result {
    MlSource1TemporaryReadRustStandardSumOwnershipProfileLoop7Done(bool),
    MlSource1TemporaryReadRustStandardSumOwnershipProfileLoop7Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_standard_sum_ownership_profile_loop7(
    mut owned: bool,
    source_ml_source1_temporary_values6: &[String],
    mut source_ml_source1_temporary_index6: self::ml::Big,
) -> self::MlSource1TemporaryReadRustStandardSumOwnershipProfileLoop7Result {
    loop {
        return if source_ml_source1_temporary_index6
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values6.len() as u128))
        {
            {
                let role = self::ml_array::at(
                    source_ml_source1_temporary_values6,
                    source_ml_source1_temporary_index6.to_index(),
                );
                if (role != "copy") && (role != "owned") {
                    self::MlSource1TemporaryReadRustStandardSumOwnershipProfileLoop7Result::MlSource1TemporaryReadRustStandardSumOwnershipProfileLoop7Return(String::new())
                } else {
                    if role == "owned" {
                        {
                            let owned_2 = true;
                            {
                                let source_ml_source1_temporary_index6_2 =
                                    source_ml_source1_temporary_index6
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (owned, source_ml_source1_temporary_index6) =
                                        (owned_2, source_ml_source1_temporary_index6_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let source_ml_source1_temporary_index6_3 =
                                source_ml_source1_temporary_index6
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index6 =
                                    source_ml_source1_temporary_index6_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustStandardSumOwnershipProfileLoop7Result::MlSource1TemporaryReadRustStandardSumOwnershipProfileLoop7Done(owned)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptRustStandardSumOwnedArgumentsLoop9Result {
    MlSource1TemporaryAcceptRustStandardSumOwnedArgumentsLoop9Done,
    MlSource1TemporaryAcceptRustStandardSumOwnedArgumentsLoop9Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_standard_sum_owned_arguments_loop9(
    source_ml_source1_temporary_values8: &[String],
    mut source_ml_source1_temporary_index8: self::ml::Big,
) -> self::MlSource1TemporaryAcceptRustStandardSumOwnedArgumentsLoop9Result {
    loop {
        return if source_ml_source1_temporary_index8
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values8.len() as u128))
        {
            {
                let identity = self::ml_array::at(
                    source_ml_source1_temporary_values8,
                    source_ml_source1_temporary_index8.to_index(),
                );
                if identity == "" {
                    self::MlSource1TemporaryAcceptRustStandardSumOwnedArgumentsLoop9Result::MlSource1TemporaryAcceptRustStandardSumOwnedArgumentsLoop9Return(false)
                } else {
                    {
                        let source_ml_source1_temporary_index8_2 =
                            source_ml_source1_temporary_index8.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index8 =
                                source_ml_source1_temporary_index8_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryAcceptRustStandardSumOwnedArgumentsLoop9Result::MlSource1TemporaryAcceptRustStandardSumOwnedArgumentsLoop9Done
        };
    }
}
