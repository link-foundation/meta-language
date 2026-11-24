// Generated from complete js/src/translation/rust-owned-record-effect-rules.js. Do not edit by hand.
use super::ml_array;
use super::read_required_number_fact;

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanConditionalMatchArmsLoop3Result {
    MlTemporaryPlanConditionalMatchArmsLoop3Done(Vec<f64>),
    MlTemporaryPlanConditionalMatchArmsLoop3Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_plan_conditional_match_arms_loop3(
    guards: &[f64],
    mut coverage: Vec<f64>,
    mut at: f64,
) -> self::MlTemporaryPlanConditionalMatchArmsLoop3Result {
    loop {
        return if at < (guards.len() as f64) {
            {
                let guard = self::read_required_number_fact(guards, at);
                if (guard != 0f64) && (guard != 1f64) {
                    self::MlTemporaryPlanConditionalMatchArmsLoop3Result::MlTemporaryPlanConditionalMatchArmsLoop3Return(Vec::<f64>::new())
                } else {
                    if guard == 0f64 {
                        {
                            let coverage_2 = {
                                let source_ml_source1_concat_value13 = coverage;
                                {
                                    let source_ml_source1_concat_value14 = vec![at];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value13,
                                        ),
                                        source_ml_source1_concat_value14,
                                    )
                                }
                            };
                            {
                                let at_2 = at + 1f64;
                                {
                                    (coverage, at) = (coverage_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let at_3 = at + 1f64;
                            {
                                at = at_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanConditionalMatchArmsLoop3Result::MlTemporaryPlanConditionalMatchArmsLoop3Done(coverage)
        };
    }
}
