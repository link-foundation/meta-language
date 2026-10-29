// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::test_java_script_call_code_coordinate;

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop103(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values99: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index99: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result {
    loop {
        return if (source_ml_source1_temporary_index99 < (self::ml::Big::from_u128(source_ml_source1_temporary_values99.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values99, source_ml_source1_temporary_index99.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let captured = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value856, _) => {
                                let source_ml_source2_data_property_value856 = *source_ml_source2_data_property_value856;
                                source_ml_source2_data_property_value856.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value857, _) => {
                                    let source_ml_source2_data_property_value857 = *source_ml_source2_data_property_value857;
                                    source_ml_source2_data_property_value857.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value858) => {
                                    source_ml_source2_data_property_value858
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value859) => {
                                    source_ml_source2_data_property_value859
                                }
                            }));
                            {
                                let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match captured.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value860) => {
                                        source_ml_source2_data_property_value860
                                    }
                                }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
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
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop103Done(Box::new(current.clone()), arguments_)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop105(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values100: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index100: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result {
    loop {
        return if (source_ml_source1_temporary_index100 < (self::ml::Big::from_u128(source_ml_source1_temporary_values100.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values100, source_ml_source1_temporary_index100.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value861, _) => {
                                let source_ml_source2_data_property_value861 = *source_ml_source2_data_property_value861;
                                source_ml_source2_data_property_value861.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value862) => {
                                    source_ml_source2_data_property_value862
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index100_2 = source_ml_source1_temporary_index100.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index100) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index100_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Done(Box::new(current.clone()), arguments_)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptCallSourceLoop106Result {
    MlSource1TemporaryCompileJavaScriptCallSourceLoop106Done,
    MlSource1TemporaryCompileJavaScriptCallSourceLoop106Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_call_source_loop106(parameters: &[f64], mut index: f64) -> self::MlSource1TemporaryCompileJavaScriptCallSourceLoop106Result {
    loop {
        return if (index < ((parameters.len() as f64))) {
            {
                let owner = (self::ml_array::optional_at(parameters, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || ((self::ml_array::index_of(parameters, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop106Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop106Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, String::from("source parameters require distinct actual lexical owners"))))
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
            self::MlSource1TemporaryCompileJavaScriptCallSourceLoop106Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop106Done
        };
    }
}
