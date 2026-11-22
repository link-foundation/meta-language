// Generated from complete js/src/translation/rust-borrowed-iterator-rules.js. Do not edit by hand.
use super::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result;
use super::ml;
use super::ml_array;
use super::ml_math;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop4(
    mut result: Vec<String>,
    source_ml_source1_temporary_values2: &[String],
    mut source_ml_source1_temporary_index2: self::ml::Big,
) -> Vec<String> {
    loop {
        if source_ml_source1_temporary_index2
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values2.len() as u128))
        {
            {
                let root = self::ml_array::at(
                    source_ml_source1_temporary_values2,
                    source_ml_source1_temporary_index2.to_index(),
                );
                if !(self::ml_array::includes(&result, &root.clone(), 0f64, false, |a, b| a == b)) {
                    {
                        let result_2 = {
                            let (ml_append_right, ml_append_left) = (vec![root.clone()], result);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let source_ml_source1_temporary_index2_2 =
                                source_ml_source1_temporary_index2
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (result, source_ml_source1_temporary_index2) =
                                    (result_2, source_ml_source1_temporary_index2_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    {
                        let source_ml_source1_temporary_index2_3 =
                            source_ml_source1_temporary_index2.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index2 =
                                source_ml_source1_temporary_index2_3;
                            continue;
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
pub fn source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop3(
    existing: &[String],
    local: &[String],
    receiver: &[String],
    mut result: Vec<String>,
    source_ml_source1_temporary_values1: &[String],
    mut source_ml_source1_temporary_index1: self::ml::Big,
) -> self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result {
    loop {
        return if source_ml_source1_temporary_index1
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values1.len() as u128))
        {
            {
                let loan = self::ml_array::at(
                    source_ml_source1_temporary_values1,
                    source_ml_source1_temporary_index1.to_index(),
                );
                if self::ml_array::includes(receiver, &loan.clone(), 0f64, false, |a, b| a == b) {
                    if !(self::ml_array::includes(&result, &loan.clone(), 0f64, false, |a, b| {
                        a == b
                    })) {
                        {
                            let result_2 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![loan.clone()], result);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let source_ml_source1_temporary_index1_2 =
                                    source_ml_source1_temporary_index1
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (result, source_ml_source1_temporary_index1) =
                                        (result_2, source_ml_source1_temporary_index1_2);
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
                {
                    if self::ml_array::includes(local, &loan.clone(), 0f64, false, |a, b| a == b)
                        && !(self::ml_array::includes(
                            existing,
                            &loan.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))
                    {
                        {
                            let source_ml_source1_temporary_values2 = receiver.to_vec();
                            {
                                let source_ml_source1_temporary_index2 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let result_3 = self::source_ml_source1_temporary_project_rust_borrowed_iterator_callback_regions_loop4(result, &(source_ml_source1_temporary_values2), source_ml_source1_temporary_index2);
                                    {
                                        let source_ml_source1_temporary_index1_4 =
                                            source_ml_source1_temporary_index1
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            (result, source_ml_source1_temporary_index1) =
                                                (result_3, source_ml_source1_temporary_index1_4);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {
                        self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Return(Vec::<String>::new())
                    }
                }
            }
        } else {
            self::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Result::MlSource1TemporaryProjectRustBorrowedIteratorCallbackRegionsLoop3Done(result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result {
    MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Done,
    MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_borrowed_iterator_budget_error_loop6(
    source_ml_source1_temporary_values5: &[f64],
    mut source_ml_source1_temporary_index5: self::ml::Big,
) -> self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result {
    loop {
        return if source_ml_source1_temporary_index5
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values5.len() as u128))
        {
            {
                let count = self::ml_array::at(
                    source_ml_source1_temporary_values5,
                    source_ml_source1_temporary_index5.to_index(),
                );
                if (!(self::ml_math::is_safe_integer(count)) || (count < 0f64)) || {
                    let source_ml_source2_same_value_operand21 = count;
                    {
                        let source_ml_source2_same_value_operand22 = -0f64;
                        (((source_ml_source2_same_value_operand21
                            == source_ml_source2_same_value_operand22)
                            && ((source_ml_source2_same_value_operand21 != 0f64)
                                || ((1f64 / source_ml_source2_same_value_operand21)
                                    == (1f64 / source_ml_source2_same_value_operand22))))
                            || ((source_ml_source2_same_value_operand21
                                != source_ml_source2_same_value_operand21)
                                && (source_ml_source2_same_value_operand22
                                    != source_ml_source2_same_value_operand22)))
                    }
                } {
                    self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Return(String::from("invalid readonly iterator source resource count"))
                } else {
                    {
                        let source_ml_source1_temporary_index5_2 =
                            source_ml_source1_temporary_index5.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index5 =
                                source_ml_source1_temporary_index5_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Result::MlSource1TemporaryReadRustBorrowedIteratorBudgetErrorLoop6Done
        };
    }
}
