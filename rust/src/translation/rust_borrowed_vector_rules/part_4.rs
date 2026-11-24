// Generated from complete js/src/translation/rust-borrowed-vector-rules.js. Do not edit by hand.
use super::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result;
use super::ml;
use super::ml_array;
use super::ml_math;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_rust_borrowed_vector_inherent_profiles_loop7(
    profiles: &[String],
    mut index: f64,
) -> self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result {
    loop {
        return if index < (profiles.len() as f64) {
            if (index != 2f64) && {
                let source_ml_source2_comparison_operand41 =
                    self::ml_array::optional_at(profiles, self::ml_array::number_index(index));
                {
                    let source_ml_source2_comparison_operand42 = String::new();
                    !(match (
                        source_ml_source2_comparison_operand41,
                        source_ml_source2_comparison_operand42,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Return(false)
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
            self::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Result::MlSource1TemporaryAcceptRustBorrowedVectorInherentProfilesLoop7Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_borrowed_vector_result_parameters_loop8(
    lifetime: &str,
    references: &[bool],
    lifetimes: &[String],
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if index < (references.len() as f64) {
            if match {
                let source_ml_source2_logical45 =
                    self::ml_array::optional_at(references, self::ml_array::number_index(index));
                if (source_ml_source2_logical45.clone()).unwrap_or_default() {
                    Some(if (!(lifetime).is_empty()) && (lifetime != "_") {
                        {
                            let source_ml_source2_comparison_operand43 =
                                self::ml_array::optional_at(
                                    lifetimes,
                                    self::ml_array::number_index(index),
                                );
                            {
                                let source_ml_source2_comparison_operand44 = lifetime.to_owned();
                                match (
                                    source_ml_source2_comparison_operand43,
                                    source_ml_source2_comparison_operand44,
                                ) {
                                    (Some(ml_left), ml_right) => ml_left == ml_right,
                                    _ => false,
                                }
                            }
                        }
                    } else {
                        true
                    })
                } else {
                    source_ml_source2_logical45.clone()
                }
            } {
                Some(ml_present) => ml_present,
                None => false,
            } {
                {
                    let result_2 = {
                        let (ml_append_right, ml_append_left) = (vec![index], result);
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
            } else {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result {
    MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Done,
    MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_rust_borrowed_expression_position_reservation_loop10(
    source_ml_source1_temporary_values9: &[f64],
    mut source_ml_source1_temporary_index9: self::ml::Big,
) -> self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result {
    loop {
        return if source_ml_source1_temporary_index9
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values9.len() as u128))
        {
            {
                let value = self::ml_array::at(
                    source_ml_source1_temporary_values9,
                    source_ml_source1_temporary_index9.to_index(),
                );
                if !(self::ml_math::is_safe_integer(value)) || (value < 0f64) {
                    self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Return(Vec::<f64>::new())
                } else {
                    {
                        let source_ml_source1_temporary_index9_2 =
                            source_ml_source1_temporary_index9.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index9 =
                                source_ml_source1_temporary_index9_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Result::MlSource1TemporaryReadRustBorrowedExpressionPositionReservationLoop10Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result {
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Done(f64, f64),
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop14(
    mut target_boxes: f64,
    mut target_references: f64,
    source_ml_source1_temporary_values12: &[String],
    mut source_ml_source1_temporary_index12: self::ml::Big,
) -> self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result {
    loop {
        return if source_ml_source1_temporary_index12
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values12.len() as u128))
        {
            {
                let layer = self::ml_array::at(
                    source_ml_source1_temporary_values12,
                    source_ml_source1_temporary_index12.to_index(),
                );
                if layer == "box" {
                    {
                        let target_boxes_2 = target_boxes + 1f64;
                        {
                            let source_ml_source1_temporary_index12_2 =
                                source_ml_source1_temporary_index12
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (target_boxes, source_ml_source1_temporary_index12) =
                                    (target_boxes_2, source_ml_source1_temporary_index12_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    if layer == "reference" {
                        {
                            let target_references_2 = target_references + 1f64;
                            {
                                let source_ml_source1_temporary_index12_3 =
                                    source_ml_source1_temporary_index12
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (target_references, source_ml_source1_temporary_index12) = (
                                        target_references_2,
                                        source_ml_source1_temporary_index12_3,
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Return(Vec::<f64>::new())
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop14Done(target_boxes, target_references)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result {
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Done(f64, f64),
    MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_rust_borrowed_inherent_position_loop13(
    mut actual_boxes: f64,
    mut actual_references: f64,
    source_ml_source1_temporary_values11: &[String],
    mut source_ml_source1_temporary_index11: self::ml::Big,
) -> self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result {
    loop {
        return if source_ml_source1_temporary_index11
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values11.len() as u128))
        {
            {
                let layer = self::ml_array::at(
                    source_ml_source1_temporary_values11,
                    source_ml_source1_temporary_index11.to_index(),
                );
                if layer == "box" {
                    {
                        let actual_boxes_2 = actual_boxes + 1f64;
                        {
                            let source_ml_source1_temporary_index11_2 =
                                source_ml_source1_temporary_index11
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                (actual_boxes, source_ml_source1_temporary_index11) =
                                    (actual_boxes_2, source_ml_source1_temporary_index11_2);
                                continue;
                            }
                        }
                    }
                }
                {
                    if layer == "reference" {
                        {
                            let actual_references_2 = actual_references + 1f64;
                            {
                                let source_ml_source1_temporary_index11_3 =
                                    source_ml_source1_temporary_index11
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    (actual_references, source_ml_source1_temporary_index11) = (
                                        actual_references_2,
                                        source_ml_source1_temporary_index11_3,
                                    );
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Return(Vec::<f64>::new())
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Result::MlSource1TemporaryPlanRustBorrowedInherentPositionLoop13Done(actual_boxes, actual_references)
        };
    }
}
