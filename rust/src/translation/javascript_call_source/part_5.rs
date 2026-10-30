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
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop105(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values101: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index101: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop105Result {
    loop {
        return if (source_ml_source1_temporary_index101 < (self::ml::Big::from_u128(source_ml_source1_temporary_values101.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values101, source_ml_source1_temporary_index101.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let captured = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value875, _) => {
                                let source_ml_source2_data_property_value875 = *source_ml_source2_data_property_value875;
                                source_ml_source2_data_property_value875.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value876, _) => {
                                    let source_ml_source2_data_property_value876 = *source_ml_source2_data_property_value876;
                                    source_ml_source2_data_property_value876.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value877) => {
                                    source_ml_source2_data_property_value877
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value878) => {
                                    source_ml_source2_data_property_value878
                                }
                            }));
                            {
                                let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match captured.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value879) => {
                                        source_ml_source2_data_property_value879
                                    }
                                }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let source_ml_source1_temporary_index101_2 = source_ml_source1_temporary_index101.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, arguments_, source_ml_source1_temporary_index101) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index101_2.clone());
                                        continue;
                                    }
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
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop107Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop107Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop107(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values102: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index102: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop107Result {
    loop {
        return if (source_ml_source1_temporary_index102 < (self::ml::Big::from_u128(source_ml_source1_temporary_values102.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values102, source_ml_source1_temporary_index102.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value880, _) => {
                                let source_ml_source2_data_property_value880 = *source_ml_source2_data_property_value880;
                                source_ml_source2_data_property_value880.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value881) => {
                                    source_ml_source2_data_property_value881
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index102_2 = source_ml_source1_temporary_index102.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index102) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index102_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop107Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop107Done(Box::new(current.clone()), arguments_)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptCallSourceLoop108Result {
    MlSource1TemporaryCompileJavaScriptCallSourceLoop108Done,
    MlSource1TemporaryCompileJavaScriptCallSourceLoop108Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_call_source_loop108(parameters: &[f64], mut index: f64) -> self::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Result {
    loop {
        return if (index < ((parameters.len() as f64))) {
            {
                let owner = (self::ml_array::optional_at(parameters, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || ((self::ml_array::index_of(parameters, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, String::from("source parameters require distinct actual lexical owners"))))
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
            self::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop108Done
        };
    }
}
