// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Result;
use super::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Result;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_plan_java_script_natural_guard_order_loop110;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_natural_guard_order_loop109(types: &[self::ml_string::Text], orders: &[f64], mut positions: Vec<f64>, mut order: f64) -> self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Result {
    loop {
        return if (order < ((orders.len() as f64))) {
            {
                let found = (-1f64);
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s851 = self::source_ml_source1_temporary_plan_java_script_natural_guard_order_loop110(orders, order, found, index);
                        match source_ml_source2_s851.clone() {
                            self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Done(found_2) => {
                                if ((found_2 < (0f64)) || (((self::ml_array::optional_at(types, self::ml_array::number_index(found_2))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                                    self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Return(vec![(-1f64)])
                                } else {
                                    {
                                        let positions_2 = { let (ml_append_right, ml_append_left) = (vec![found_2], positions); self::ml_array::append(ml_append_left, ml_append_right) };
                                        {
                                            let order_2 = (order + 1f64);
                                            {
                                                (positions, order) = (positions_2, order_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Return(source_ml_source1_temporary_result110) => {
                                self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Return(source_ml_source1_temporary_result110)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Done(positions)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Result {
    MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Done(f64, f64, f64),
    MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_natural_pattern_branches_loop111(patterns: &[self::ml_string::Text], mut zero: f64, mut succ: f64, mut fallback: f64, mut index: f64) -> self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Result {
    loop {
        return if (index < ((patterns.len() as f64))) {
            {
                let pattern = (self::ml_array::optional_at(patterns, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (pattern == (self::ml_string::Text::from_units(vec![110, 97, 116, 90, 101, 114, 111]))) {
                    if (zero != ((-1f64))) {
                        self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Return(Vec::<f64>::new())
                    } else {
                        {
                            let zero_2 = index;
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (zero, index) = (zero_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    if (pattern == (self::ml_string::Text::from_units(vec![110, 97, 116, 83, 117, 99, 99]))) {
                        if (succ != ((-1f64))) {
                            self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Return(Vec::<f64>::new())
                        } else {
                            {
                                let succ_2 = index;
                                {
                                    let index_3 = (index + 1f64);
                                    {
                                        (succ, index) = (succ_2, index_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        if ((pattern == (self::ml_string::Text::from_units(vec![119, 105, 108, 100]))) || (pattern == (self::ml_string::Text::from_units(vec![98, 105, 110, 100])))) {
                            if (fallback != ((-1f64))) {
                                self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Return(Vec::<f64>::new())
                            } else {
                                {
                                    let fallback_2 = index;
                                    {
                                        let index_4 = (index + 1f64);
                                        {
                                            (fallback, index) = (fallback_2, index_4);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Return(Vec::<f64>::new())
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop111Done(zero, succ, fallback)
        };
    }
}
