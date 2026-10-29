// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::test_java_script_call_code_coordinate;

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop104(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values99: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index99: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result {
    loop {
        return if (source_ml_source1_temporary_index99 < (self::ml::Big::from_u128(source_ml_source1_temporary_values99.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values99, source_ml_source1_temporary_index99.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    match value.clone() {
                        self::JavaScriptCallSourceResult::SourceResult(shared_constructor_field1, shared_constructor_field2) => {
                            let shared_constructor_field1 = *shared_constructor_field1;
                            {
                                let current_2 = shared_constructor_field1.clone();
                                {
                                    let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![shared_constructor_field2], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                                    {
                                        let source_ml_source1_temporary_index99_2 = source_ml_source1_temporary_index99.add(&self::ml::Big::from_i128(1));
                                        {
                                            (current, arguments_, source_ml_source1_temporary_index99) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index99_2.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop104Done(Box::new(current.clone()), arguments_)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptCallSourceLoop105Result {
    MlSource1TemporaryCompileJavaScriptCallSourceLoop105Done,
    MlSource1TemporaryCompileJavaScriptCallSourceLoop105Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_call_source_loop105(parameters: &[f64], mut index: f64) -> self::MlSource1TemporaryCompileJavaScriptCallSourceLoop105Result {
    loop {
        return if (index < ((parameters.len() as f64))) {
            {
                let owner = (self::ml_array::optional_at(parameters, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || ((self::ml_array::index_of(parameters, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop105Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop105Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 111, 119, 110, 101, 114, 115]))))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompileJavaScriptCallSourceLoop105Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop105Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_literal_error_source_loop106(message: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (message.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand848 = message.clone();
                    {
                        let source_ml_source2_string_unit_operand849 = index;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand855 = {
                                        let source_ml_source2_string_range_operand850 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand851 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand852 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position853 = source_ml_source2_string_range_operand851;
                                                    {
                                                        let source_ml_source2_string_range_position854 = source_ml_source2_string_range_operand852;
                                                        source_ml_source2_string_range_operand850.slice(source_ml_source2_string_range_position853, source_ml_source2_string_range_position854)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand856 = 0f64;
                                        source_ml_source2_string_unit_operand855.code_point_at(source_ml_source2_string_unit_operand856)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand848.clone(), source_ml_source2_string_unit_operand849)
                    }
                }], units); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (units, index) = (units_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            units
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop108Result {
    MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop108Done,
    MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop108Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_natural_source_realm_loop108(first: &self::ml_string::Text, source_ml_source1_temporary_values107: &[self::ml_string::Text], mut source_ml_source1_temporary_index107: self::ml::Big) -> self::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop108Result {
    loop {
        return if (source_ml_source1_temporary_index107 < (self::ml::Big::from_u128(source_ml_source1_temporary_values107.len() as u128))) {
            {
                let realm = self::ml_array::at(source_ml_source1_temporary_values107, source_ml_source1_temporary_index107.to_index());
                if (realm != (*first)) {
                    self::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop108Result::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop108Return(false)
                } else {
                    {
                        let source_ml_source1_temporary_index107_2 = source_ml_source1_temporary_index107.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index107 = source_ml_source1_temporary_index107_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop108Result::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop108Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Result {
    MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Done(Vec<f64>),
    MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop109Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Result {
    MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Done(f64),
    MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_natural_guard_order_loop110(orders: &[f64], mut order: f64, mut found: f64, mut index: f64) -> self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Result {
    loop {
        return if (index < ((orders.len() as f64))) {
            if (((self::ml_array::optional_at(orders, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64))) == order) {
                if (found != ((-1f64))) {
                    self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Return(vec![(-1f64)])
                } else {
                    {
                        let found_2 = index;
                        {
                            let index_2 = (index + 1f64);
                            {
                                (found, index) = (found_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let index_3 = (index + 1f64);
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop110Done(found)
        };
    }
}

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
                        let source_ml_source2_s857 = self::source_ml_source1_temporary_plan_java_script_natural_guard_order_loop110(orders, order, found, index);
                        match source_ml_source2_s857.clone() {
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
