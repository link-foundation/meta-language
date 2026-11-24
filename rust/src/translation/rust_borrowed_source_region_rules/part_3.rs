// Generated from complete js/src/translation/rust-borrowed-source-region-rules.js. Do not edit by hand.
use super::ml;
use super::ml_array;
use super::ml_math;
use super::read_rust_owned_binding_read_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_rust_owned_loan_bindings_loop5(
    mut result: Vec<String>,
    source_ml_temporary_values3: &[String],
    mut source_ml_temporary_index3: self::ml::Big,
) -> Vec<String> {
    loop {
        if source_ml_temporary_index3
            < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))
        {
            {
                let identity = self::ml_array::at(
                    source_ml_temporary_values3,
                    source_ml_temporary_index3.to_index(),
                );
                if !(self::ml_array::includes(&result, &identity.clone(), 0f64, false, |a, b| {
                    a == b
                })) {
                    {
                        let result_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![identity.clone()], result);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let source_ml_temporary_index3_2 =
                                source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_temporary_index3) =
                                    (result_2, source_ml_temporary_index3_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_temporary_index3_3 =
                            source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index3 = source_ml_temporary_index3_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustOwnedLoanReadErrorLoop7Result {
    MlTemporaryReadRustOwnedLoanReadErrorLoop7Done,
    MlTemporaryReadRustOwnedLoanReadErrorLoop7Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_owned_loan_read_error_loop7(
    source_ml_temporary_values6: &[String],
    mut source_ml_temporary_index6: self::ml::Big,
) -> self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result {
    loop {
        return if source_ml_temporary_index6
            < (self::ml::Big::from_u128(source_ml_temporary_values6.len() as u128))
        {
            {
                let state = self::ml_array::at(
                    source_ml_temporary_values6,
                    source_ml_temporary_index6.to_index(),
                );
                {
                    let ml_library_inequality_1 = state != "available";
                    if ml_library_inequality_1 {
                        self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result::MlTemporaryReadRustOwnedLoanReadErrorLoop7Return(self::read_rust_owned_binding_read_error(""))
                    } else {
                        {
                            let source_ml_temporary_index6_2 =
                                source_ml_temporary_index6.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index6 = source_ml_temporary_index6_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustOwnedLoanReadErrorLoop7Result::MlTemporaryReadRustOwnedLoanReadErrorLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result {
    MlTemporaryReadRustBorrowedSourceReadErrorLoop10Done,
    MlTemporaryReadRustBorrowedSourceReadErrorLoop10Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_borrowed_source_read_error_loop10(
    source_ml_temporary_values8: &[String],
    mut source_ml_temporary_index8: self::ml::Big,
) -> self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result {
    loop {
        return if source_ml_temporary_index8
            < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))
        {
            {
                let loan = self::ml_array::at(
                    source_ml_temporary_values8,
                    source_ml_temporary_index8.to_index(),
                );
                {
                    let error = self::read_rust_owned_binding_read_error(&(loan));
                    if error.is_empty() {
                        {
                            let source_ml_temporary_index8_2 =
                                source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index8 = source_ml_temporary_index8_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Return(String::from("a source expression uses a region whose actual binder has moved or expired"))
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Result::MlTemporaryReadRustBorrowedSourceReadErrorLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceReadErrorJoin9 {
    MlTemporaryReadRustBorrowedSourceReadErrorJoin9Next,
    MlTemporaryReadRustBorrowedSourceReadErrorJoin9Return(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result {
    MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Done,
    MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_borrowed_source_return_error_loop12(
    allowed: &[f64],
    static_regions: &[f64],
    source_ml_temporary_values11: &[f64],
    mut source_ml_temporary_index11: self::ml::Big,
) -> self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result {
    loop {
        return if source_ml_temporary_index11
            < (self::ml::Big::from_u128(source_ml_temporary_values11.len() as u128))
        {
            {
                let region = self::ml_array::at(
                    source_ml_temporary_values11,
                    source_ml_temporary_index11.to_index(),
                );
                if ((self::ml_array::includes(
                    allowed,
                    &region,
                    0f64,
                    false,
                    |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                )) || (self::ml_array::includes(
                    static_regions,
                    &region,
                    0f64,
                    false,
                    |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                ))) {
                    {
                        let source_ml_temporary_index11_2 =
                            source_ml_temporary_index11.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index11 = source_ml_temporary_index11_2;
                            continue;
                        }
                    }
                }
                {
                    self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Return(String::from("a returned source region is not derived from its actual enclosing parameter interface"))
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Result::MlTemporaryReadRustBorrowedSourceReturnErrorLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result {
    MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Done,
    MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_borrowed_source_lifetime_error_loop14(
    demand: &str,
    source_ml_temporary_values13: &[String],
    mut source_ml_temporary_index13: self::ml::Big,
) -> self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result {
    loop {
        return if source_ml_temporary_index13
            < (self::ml::Big::from_u128(source_ml_temporary_values13.len() as u128))
        {
            {
                let source = self::ml_array::at(
                    source_ml_temporary_values13,
                    source_ml_temporary_index13.to_index(),
                );
                if (source != "static") && (source != demand) {
                    self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Return(String::from("an expression lifetime is not supplied by its actual source binder regions"))
                } else {
                    {
                        let source_ml_temporary_index13_2 =
                            source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index13 = source_ml_temporary_index13_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Result::MlTemporaryReadRustBorrowedSourceLifetimeErrorLoop14Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result {
    MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Done,
    MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_borrowed_source_region_budget_error_loop16(
    source_ml_temporary_values15: &[f64],
    mut source_ml_temporary_index15: self::ml::Big,
) -> self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result {
    loop {
        return if source_ml_temporary_index15
            < (self::ml::Big::from_u128(source_ml_temporary_values15.len() as u128))
        {
            {
                let value = self::ml_array::at(
                    source_ml_temporary_values15,
                    source_ml_temporary_index15.to_index(),
                );
                if (!(self::ml_math::is_safe_integer(value)) || (value < 0f64)) || {
                    let source_ml_source1_same_value_operand27 = value;
                    {
                        let source_ml_source1_same_value_operand28 = -0f64;
                        (((source_ml_source1_same_value_operand27
                            == source_ml_source1_same_value_operand28)
                            && ((source_ml_source1_same_value_operand27 != 0f64)
                                || ((1f64 / source_ml_source1_same_value_operand27)
                                    == (1f64 / source_ml_source1_same_value_operand28))))
                            || ((source_ml_source1_same_value_operand27
                                != source_ml_source1_same_value_operand27)
                                && (source_ml_source1_same_value_operand28
                                    != source_ml_source1_same_value_operand28)))
                    }
                } {
                    self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Return(String::from("invalid source region proof resource count"))
                } else {
                    {
                        let source_ml_temporary_index15_2 =
                            source_ml_temporary_index15.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index15 = source_ml_temporary_index15_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Result::MlTemporaryReadRustBorrowedSourceRegionBudgetErrorLoop16Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReserveRustBorrowedSourcePositionLoop18Result {
    MlTemporaryReserveRustBorrowedSourcePositionLoop18Done,
    MlTemporaryReserveRustBorrowedSourcePositionLoop18Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_reserve_rust_borrowed_source_position_loop18(
    source_ml_temporary_values17: &[f64],
    mut source_ml_temporary_index17: self::ml::Big,
) -> self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result {
    loop {
        return if source_ml_temporary_index17
            < (self::ml::Big::from_u128(source_ml_temporary_values17.len() as u128))
        {
            {
                let value = self::ml_array::at(
                    source_ml_temporary_values17,
                    source_ml_temporary_index17.to_index(),
                );
                if (!(self::ml_math::is_safe_integer(value)) || (value < 0f64)) || {
                    let source_ml_source1_same_value_operand29 = value;
                    {
                        let source_ml_source1_same_value_operand30 = -0f64;
                        (((source_ml_source1_same_value_operand29
                            == source_ml_source1_same_value_operand30)
                            && ((source_ml_source1_same_value_operand29 != 0f64)
                                || ((1f64 / source_ml_source1_same_value_operand29)
                                    == (1f64 / source_ml_source1_same_value_operand30))))
                            || ((source_ml_source1_same_value_operand29
                                != source_ml_source1_same_value_operand29)
                                && (source_ml_source1_same_value_operand30
                                    != source_ml_source1_same_value_operand30)))
                    }
                } {
                    self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Return(Vec::<f64>::new())
                } else {
                    {
                        let source_ml_temporary_index17_2 =
                            source_ml_temporary_index17.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index17 = source_ml_temporary_index17_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReserveRustBorrowedSourcePositionLoop18Result::MlTemporaryReserveRustBorrowedSourcePositionLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result {
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Done,
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_borrowed_source_box_reference_loop20(
    target: &[String],
    mut index: f64,
) -> self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result {
    loop {
        return if index < (target.len() as f64) {
            if {
                let source_ml_source1_comparison_operand31 =
                    self::ml_array::optional_at(target, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand32 = String::from("box");
                    !(match (
                        source_ml_source1_comparison_operand31,
                        source_ml_source1_comparison_operand32,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Return(false)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop20Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result {
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Done,
    MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_borrowed_source_box_reference_loop19(
    source: &[String],
    mut index: f64,
) -> self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result {
    loop {
        return if index < (source.len() as f64) {
            if {
                let source_ml_source1_comparison_operand33 =
                    self::ml_array::optional_at(source, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand34 = String::from("box");
                    !(match (
                        source_ml_source1_comparison_operand33,
                        source_ml_source1_comparison_operand34,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Return(false)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Result::MlTemporaryAcceptRustBorrowedSourceBoxReferenceLoop19Done
        };
    }
}
