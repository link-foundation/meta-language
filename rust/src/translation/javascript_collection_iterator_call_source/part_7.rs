// Generated from complete js/src/translation/javascript-collection-iterator-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionIteratorCallCompilation;
use super::JavaScriptCollectionIteratorMethodOwner;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Result;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Result;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Result;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Result;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Result;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::source_ml_temporary_order_java_script_call_code_functions_loop11;
use super::source_ml_temporary_order_java_script_call_code_functions_loop12;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_call_code_functions_loop10(
    mut functions: Vec<self::JavaScriptCallFunction>,
    mut ordered: Vec<self::JavaScriptCallFunction>,
    mut merged: Vec<self::JavaScriptCallFunction>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Result {
    loop {
        return if (left < middle) && (right < end) {
            {
                let first =
                    self::ml_array::optional_at(&ordered, self::ml_array::number_index(left));
                {
                    let second =
                        self::ml_array::optional_at(&ordered, self::ml_array::number_index(right));
                    if match (first.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Return(functions)
                    } else if match (second.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Return(functions)
                    } else {
                        if (match first.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(
                                source_ml_source1_data_property_value705,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value705.clone(),
                        }) <= (match second.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(
                                source_ml_source1_data_property_value706,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value706.clone(),
                        }) {
                            {
                                let merged_2 = {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![first.clone().expect("a proven present value")],
                                        merged,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let left_2 = left + 1f64;
                                    {
                                        (merged, left) = (merged_2, left_2);
                                        continue;
                                    }
                                }
                            }
                        }
                        {
                            {
                                let merged_3 = {
                                    let (ml_append_right, ml_append_left) = (
                                        vec![second.clone().expect("a proven present value")],
                                        merged,
                                    );
                                    self::ml_array::append(ml_append_left, ml_append_right)
                                };
                                {
                                    let right_2 = right + 1f64;
                                    {
                                        (merged, right) = (merged_3, right_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Done(merged, left, right)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_call_code_functions_loop9(
    mut functions: Vec<self::JavaScriptCallFunction>,
    mut ordered: Vec<self::JavaScriptCallFunction>,
    mut width: f64,
    mut merged: Vec<self::JavaScriptCallFunction>,
    mut start: f64,
) -> self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Result {
    loop {
        return if start < (ordered.len() as f64) {
            {
                let middle = self::ml_math::min((start + width), (ordered.len() as f64));
                {
                    let end = self::ml_math::min((start + (2f64 * width)), (ordered.len() as f64));
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let source_ml_source1_s707 = self::source_ml_temporary_order_java_script_call_code_functions_loop10(functions.clone(), ordered.clone(), merged, middle, end, left, right);
                                match source_ml_source1_s707 {
                                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Done(merged_2, left_2, right_2) => {
                                        {
                                            let index = left_2;
                                            {
                                                let source_ml_source1_s708 = self::source_ml_temporary_order_java_script_call_code_functions_loop11(functions.clone(), ordered.clone(), merged_2, middle, index);
                                                match source_ml_source1_s708 {
                                                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Done(merged_3) => {
                                                        {
                                                            let index_2 = right_2;
                                                            {
                                                                let source_ml_source1_s709 = self::source_ml_temporary_order_java_script_call_code_functions_loop12(functions.clone(), ordered.clone(), merged_3, end, index_2);
                                                                match source_ml_source1_s709 {
                                                                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Done(merged_4) => {
                                                                        {
                                                                            let start_2 = start + (2f64 * width);
                                                                            {
                                                                                (merged, start) = (merged_4, start_2);
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Return(source_ml_temporary_result12) => {
                                                                        self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Return(source_ml_temporary_result12)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Return(source_ml_temporary_result12_2) => {
                                                        self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Return(source_ml_temporary_result12_2)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Return(source_ml_temporary_result12_3) => {
                                        self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Return(source_ml_temporary_result12_3)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Done(merged)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_call_code_functions_loop8(
    mut functions: Vec<self::JavaScriptCallFunction>,
    mut ordered: Vec<self::JavaScriptCallFunction>,
    mut width: f64,
) -> self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Result {
    loop {
        return if width < (ordered.len() as f64) {
            {
                let merged = Vec::<self::JavaScriptCallFunction>::new();
                {
                    let start = 0f64;
                    {
                        let source_ml_source1_s710 =
                            self::source_ml_temporary_order_java_script_call_code_functions_loop9(
                                functions.clone(),
                                ordered,
                                width,
                                merged,
                                start,
                            );
                        match source_ml_source1_s710 {
                            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Done(merged_2) => {
                                {
                                    let ordered_2 = merged_2;
                                    {
                                        let width_2 = width * 2f64;
                                        {
                                            (ordered, width) = (ordered_2, width_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Return(source_ml_temporary_result9) => {
                                self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Return(source_ml_temporary_result9)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Done(ordered)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Result {
    MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Done,
    MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Return(
        Box<self::JavaScriptCollectionIteratorCallCompilation>,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop16(
    mut empty: self::JavaScriptCallProgram,
    mut program: self::JavaScriptCallProgram,
    mut source_ml_temporary_values14: Vec<self::JavaScriptCollectionIteratorMethodOwner>,
    mut source_ml_temporary_index14: self::ml::Big,
) -> self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Result {
    loop {
        return if source_ml_temporary_index14
            < (self::ml::Big::from_u128(source_ml_temporary_values14.len() as u128))
        {
            {
                let owner = self::ml_array::at(
                    &source_ml_temporary_values14,
                    source_ml_temporary_index14.to_index(),
                );
                if (self::find_java_script_call_code_function(program.clone(), match owner {
                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value712, _, _, _, _, _) => {
                        source_ml_source1_data_property_value712.clone()
                    }
                })) < 0f64 {
                    self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 119, 110, 101, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 99, 111, 100, 101]))))
                } else {
                    {
                        let source_ml_temporary_index14_2 = source_ml_temporary_index14.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index14 = source_ml_temporary_index14_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop16Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Result {
    MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Done(
        Vec<self::JavaScriptCallFunction>,
        Vec<self::JavaScriptCollectionIteratorMethodOwner>,
    ),
    MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop15Return(
        Box<self::JavaScriptCollectionIteratorCallCompilation>,
    ),
}
