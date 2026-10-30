// Generated from complete js/src/translation/boolean-constructor-match-rules.js. Do not edit by hand.
use super::ml_array;
use super::ml_math;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_closed_constructor_match_sharing_loop8(
    classes: &[f64],
    mut count: f64,
    mut fallback: f64,
    mut plan: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if index < count {
            if {
                let source_ml_source1_comparison_operand5 =
                    self::ml_array::optional_at(classes, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand6 = fallback;
                    !(match (
                        source_ml_source1_comparison_operand5,
                        source_ml_source1_comparison_operand6,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                {
                    let plan_2 = {
                        let (ml_append_right, ml_append_left) = (vec![index], plan);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            (plan, index) = (plan_2, index_2);
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
            plan
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanClosedConstructorMatchSharingLoop5Result {
    MlTemporaryPlanClosedConstructorMatchSharingLoop5Done,
    MlTemporaryPlanClosedConstructorMatchSharingLoop5Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_closed_constructor_match_sharing_loop5(
    owner: &str,
    declared: &[String],
    owners: &[String],
    constructors: &[String],
    classes: &[f64],
    mut count: f64,
    mut covered: Vec<String>,
    mut index: f64,
) -> self::MlTemporaryPlanClosedConstructorMatchSharingLoop5Result {
    loop {
        return if index < count {
            {
                let constructor = (self::ml_array::optional_at(
                    constructors,
                    self::ml_array::number_index(index),
                ))
                .unwrap_or_else(|| String::new());
                {
                    let classification =
                        (self::ml_array::optional_at(classes, self::ml_array::number_index(index)))
                            .unwrap_or_else(|| -2f64);
                    if ({
                        let source_ml_source1_comparison_operand7 = self::ml_array::optional_at(
                            owners,
                            self::ml_array::number_index(index),
                        );
                        {
                            let source_ml_source1_comparison_operand8 = owner.to_owned();
                            !(match (
                                source_ml_source1_comparison_operand7,
                                source_ml_source1_comparison_operand8,
                            ) {
                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                _ => false,
                            })
                        }
                    } || !(self::ml_array::includes(
                        declared,
                        &constructor.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    ))) || self::ml_array::includes(
                        &covered,
                        &constructor.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    ) {
                        self::MlTemporaryPlanClosedConstructorMatchSharingLoop5Result::MlTemporaryPlanClosedConstructorMatchSharingLoop5Return(Vec::<f64>::new())
                    } else {
                        {
                            let covered_2 = {
                                let (ml_append_right, ml_append_left) =
                                    (vec![constructor.clone()], covered);
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            if (!(self::ml_math::is_integer(classification))
                                || (classification < -1f64))
                                || (classification >= count)
                            {
                                self::MlTemporaryPlanClosedConstructorMatchSharingLoop5Result::MlTemporaryPlanClosedConstructorMatchSharingLoop5Return(Vec::<f64>::new())
                            } else {
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (covered, index) = (covered_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanClosedConstructorMatchSharingLoop5Result::MlTemporaryPlanClosedConstructorMatchSharingLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanClosedConstructorMatchSharingLoop4Result {
    MlTemporaryPlanClosedConstructorMatchSharingLoop4Done,
    MlTemporaryPlanClosedConstructorMatchSharingLoop4Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_closed_constructor_match_sharing_loop4(
    declared: &[String],
    mut count: f64,
    mut index: f64,
) -> self::MlTemporaryPlanClosedConstructorMatchSharingLoop4Result {
    loop {
        return if index < count {
            {
                let declaration =
                    (self::ml_array::optional_at(declared, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                if (declaration == "")
                    || ((self::ml_array::index_of(
                        declared,
                        &declaration.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    )) != index)
                {
                    self::MlTemporaryPlanClosedConstructorMatchSharingLoop4Result::MlTemporaryPlanClosedConstructorMatchSharingLoop4Return(Vec::<f64>::new())
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanClosedConstructorMatchSharingLoop4Result::MlTemporaryPlanClosedConstructorMatchSharingLoop4Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_unused_constructor_bindings_loop9(
    bindings: &[String],
    references: &[String],
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (bindings.len() as f64) {
            {
                let name =
                    (self::ml_array::optional_at(bindings, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| String::new());
                if (name != "")
                    && !(self::ml_array::includes(
                        references,
                        &name.clone(),
                        0f64,
                        false,
                        |a, b| a == b,
                    ))
                {
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
                }
                {
                    {
                        let index_3 = index + 1f64;
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanSingleConstructorProjectionsLoop10Result {
    MlTemporaryPlanSingleConstructorProjectionsLoop10Done(f64, Vec<f64>),
    MlTemporaryPlanSingleConstructorProjectionsLoop10Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_single_constructor_projections_loop10(
    counts: &[f64],
    mut total: f64,
    mut result: Vec<f64>,
    mut index: f64,
) -> self::MlTemporaryPlanSingleConstructorProjectionsLoop10Result {
    loop {
        return if index < (counts.len() as f64) {
            {
                let count =
                    (self::ml_array::optional_at(counts, self::ml_array::number_index(index)))
                        .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_safe_integer(count)) || (count < 0f64))
                    || (count > 100_000f64)
                {
                    self::MlTemporaryPlanSingleConstructorProjectionsLoop10Result::MlTemporaryPlanSingleConstructorProjectionsLoop10Return(Vec::<f64>::new())
                } else {
                    {
                        let total_2 = total + count;
                        if count > 0f64 {
                            {
                                let result_2 = {
                                    let (ml_append_right, ml_append_left) = (vec![index], result);
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (total, result, index) = (total_2, result_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let index_3 = index + 1f64;
                                {
                                    (total, index) = (total_2, index_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanSingleConstructorProjectionsLoop10Result::MlTemporaryPlanSingleConstructorProjectionsLoop10Done(total, result)
        };
    }
}
