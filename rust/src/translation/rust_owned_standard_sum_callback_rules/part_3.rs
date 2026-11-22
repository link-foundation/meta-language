// Generated from complete js/src/translation/rust-owned-standard-sum-callback-rules.js. Do not edit by hand.
use super::ml_array;

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14 {
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Next(f64),
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin14Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12 {
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Next(f64, f64),
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin12Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15 {
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Next(f64),
    MlTemporaryPlanRustOwnedStandardSumCallbackSelectionJoin15Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_rust_owned_standard_sum_callback_selection_loop16(
    values: &[String],
    mut at: f64,
    mut budget: f64,
) -> f64 {
    loop {
        if ((at < (values.len() as f64)) && {
            let source_ml_source1_comparison_operand83 =
                self::ml_array::optional_at(values, self::ml_array::number_index(at));
            {
                let source_ml_source1_comparison_operand84 = String::from("|");
                !(match (
                    source_ml_source1_comparison_operand83,
                    source_ml_source1_comparison_operand84,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                })
            }
        }) && (budget < 1024f64)
        {
            {
                let at_2 = at + 1f64;
                {
                    let budget_2 = budget + 1f64;
                    {
                        (at, budget) = (at_2, budget_2);
                        continue;
                    }
                }
            }
        }
        return { at };
    }
}
