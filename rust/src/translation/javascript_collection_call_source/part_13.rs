// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionCallCompilation;
use super::JavaScriptCollectionCallableDirectory;
use super::JavaScriptCollectionIteratorCallCompilation;
use super::JavaScriptCollectionIteratorCallableDirectory;
use super::JavaScriptCollectionIteratorMethodOwner;
use super::JavaScriptSemanticValue;
use super::MlOptional;
use super::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Result;
use super::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result;
use super::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Result;
use super::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Result;
use super::compile_java_script_collection_iterator_call_directory;
use super::create_java_script_collection_call_program;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::order_java_script_call_code_functions;
use super::source_ml_temporary_compile_java_script_collection_call_directory_loop525;
use super::source_ml_temporary_compile_java_script_collection_call_directory_loop526;
use super::source_ml_temporary_compile_java_script_collection_call_directory_loop527;
use super::source_ml_temporary_compile_java_script_collection_call_directory_loop528;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_collection_call_directory(directory: self::JavaScriptCollectionCallableDirectory) -> self::JavaScriptCollectionCallCompilation {
    {
        let empty = self::create_java_script_collection_call_program(directory.clone(), std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallFunction>::new())));
        {
            let owner_budget = (match directory.clone() {
                self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, _, _, source_ml_source1_data_property_value2434, _, _) => {
                    ((source_ml_source1_data_property_value2434.clone().unwrap_or_else(|| (None::<f64>,))).0).clone()
                }
            }).clone().unwrap_or_else(|| 16f64);
            if ((((owner_budget != (16f64)) && (owner_budget != (64f64))) && !(((owner_budget == (128f64)) && {
                let source_ml_source1_comparison_operand2436 = match directory.clone() {
                    self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, _, _, _, source_ml_source1_data_property_value2435, _) => {
                        ((source_ml_source1_data_property_value2435.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                    }
                };
                match (source_ml_source1_comparison_operand2436.clone(), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 99, 97, 116, 97, 108, 111, 103, 32, 49])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }))) && !(((owner_budget == (256f64)) && {
                let source_ml_source1_comparison_operand2439 = match directory.clone() {
                    self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, _, _, _, source_ml_source1_data_property_value2438, _) => {
                        ((source_ml_source1_data_property_value2438.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                    }
                };
                match (source_ml_source1_comparison_operand2439.clone(), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 99, 97, 116, 97, 108, 111, 103, 32, 50])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
            }))) {
                self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 119, 110, 101, 114, 32, 98, 117, 100, 103, 101, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 49, 54, 32, 54, 52, 32, 49, 50, 56, 32, 111, 114, 32, 50, 53, 54, 32, 112, 114, 111, 102, 105, 108, 101]))
            } else {
                if ((({
                    let source_ml_source1_comparison_operand2442 = match directory.clone() {
                        self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, _, _, _, source_ml_source1_data_property_value2441, _) => {
                            ((source_ml_source1_data_property_value2441.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand2443 = None::<()>;
                        !(match (source_ml_source1_comparison_operand2442.clone(), source_ml_source1_comparison_operand2443.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                    }
                } && (((match directory.clone() {
                    self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, _, _, _, source_ml_source1_data_property_value2445, _) => {
                        ((source_ml_source1_data_property_value2445.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                    }
                }).clone().expect("a proven present value")) != (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 99, 97, 116, 97, 108, 111, 103, 32, 49])))) && (((match directory.clone() {
                    self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, _, _, _, source_ml_source1_data_property_value2447, _) => {
                        ((source_ml_source1_data_property_value2447.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                    }
                }).clone().expect("a proven present value")) != (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 99, 97, 116, 97, 108, 111, 103, 32, 50])))) || ({
                    let source_ml_source1_comparison_operand2449 = match directory.clone() {
                        self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, _, _, _, source_ml_source1_data_property_value2448, _) => {
                            ((source_ml_source1_data_property_value2448.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                        }
                    };
                    match (source_ml_source1_comparison_operand2449.clone(), self::ml_string::Text::from_units(vec![112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 99, 97, 116, 97, 108, 111, 103, 32, 50])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                } && (owner_budget != (256f64)))) {
                    self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 99, 97, 116, 97, 108, 111, 103, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 112, 114, 111, 102, 105, 108, 101]))
                } else {
                    if (((({ let ml_reference_array599 = match directory.clone() {
                        self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, source_ml_source1_data_property_value2452, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2452.clone()
                        }
                    }; let ml_reference_projection599 = ml_reference_array599.borrow().len() as f64; ml_reference_projection599 }) > (4f64)) || (({ let ml_reference_array600 = match directory.clone() {
                        self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(source_ml_source1_data_property_value2454, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2454.clone()
                        }
                    }; let ml_reference_projection600 = ml_reference_array600.borrow().len() as f64; ml_reference_projection600 }) > (4096f64))) || (({ let ml_reference_array601 = match directory.clone() {
                        self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, source_ml_source1_data_property_value2456, _, _, _, _) => {
                            source_ml_source1_data_property_value2456.clone()
                        }
                    }; let ml_reference_projection601 = ml_reference_array601.borrow().len() as f64; ml_reference_projection601 }) > owner_budget)) {
                        self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 114, 32, 98, 117, 100, 103, 101, 116]))
                    } else {
                        {
                            let identities = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::ml::Big>::new()));
                            {
                                let functions = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallFunction>::new()));
                                {
                                    let source_ml_temporary_values521 = match directory.clone() {
                                        self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, source_ml_source1_data_property_value2457, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2457.clone()
                                        }
                                    };
                                    {
                                        let source_ml_temporary_index521 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source1_s2458 = self::source_ml_temporary_compile_java_script_collection_call_directory_loop525(directory.clone(), empty.clone(), identities.clone(), functions.clone(), source_ml_temporary_values521.clone(), source_ml_temporary_index521.clone());
                                            match source_ml_source1_s2458.clone() {
                                                self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Done => {
                                                    {
                                                        let source_ml_temporary_values523 = match directory.clone() {
                                                            self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, source_ml_source1_data_property_value2459, _, _, _, _) => {
                                                                source_ml_source1_data_property_value2459.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_temporary_index523 = self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source1_s2460 = self::source_ml_temporary_compile_java_script_collection_call_directory_loop526(directory.clone(), empty.clone(), identities.clone(), functions.clone(), source_ml_temporary_values523.clone(), source_ml_temporary_index523.clone());
                                                                match source_ml_source1_s2460.clone() {
                                                                    self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Done => {
                                                                        {
                                                                            let iterators = self::compile_java_script_collection_iterator_call_directory(self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(match directory.clone() {
                                                                                self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(source_ml_source1_data_property_value2461, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value2461.clone()
                                                                                }
                                                                            }, Box::new(match directory.clone() {
                                                                                self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, source_ml_source1_data_property_value2462, _, _, _, _, _, _) => {
                                                                                    let source_ml_source1_data_property_value2462 = *source_ml_source1_data_property_value2462;
                                                                                    source_ml_source1_data_property_value2462.clone()
                                                                                }
                                                                            }), (match directory.clone() {
                                                                                self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, _, source_ml_source1_data_property_value2463, _, _, _) => {
                                                                                    ((source_ml_source1_data_property_value2463.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionIteratorMethodOwner>>>>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCollectionIteratorMethodOwner>::new())))));
                                                                            if ((match iterators.clone() {
                                                                                self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(_, source_ml_source1_data_property_value2465) => {
                                                                                    source_ml_source1_data_property_value2465.clone()
                                                                                }
                                                                            }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), match iterators.clone() {
                                                                                    self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(_, source_ml_source1_data_property_value2466) => {
                                                                                        source_ml_source1_data_property_value2466.clone()
                                                                                    }
                                                                                })
                                                                            } else {
                                                                                {
                                                                                    let source_ml_temporary_values524 = match match iterators.clone() {
                                                                                        self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(source_ml_source1_data_property_value2467, _) => {
                                                                                            let source_ml_source1_data_property_value2467 = *source_ml_source1_data_property_value2467;
                                                                                            source_ml_source1_data_property_value2467.clone()
                                                                                        }
                                                                                    } {
                                                                                        self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value2468, _, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value2468.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_temporary_index524 = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let source_ml_source1_s2469 = self::source_ml_temporary_compile_java_script_collection_call_directory_loop527(empty.clone(), identities.clone(), functions.clone(), source_ml_temporary_values524.clone(), source_ml_temporary_index524.clone());
                                                                                            match source_ml_source1_s2469.clone() {
                                                                                                self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Done => {
                                                                                                    {
                                                                                                        let functions_2 = self::order_java_script_call_code_functions(functions.clone());
                                                                                                        {
                                                                                                            let program = self::create_java_script_collection_call_program(directory.clone(), functions_2.clone());
                                                                                                            {
                                                                                                                let at = 0f64;
                                                                                                                {
                                                                                                                    let source_ml_source1_s2470 = self::source_ml_temporary_compile_java_script_collection_call_directory_loop528(empty.clone(), functions_2.clone(), program.clone(), at);
                                                                                                                    match source_ml_source1_s2470.clone() {
                                                                                                                        self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Done => {
                                                                                                                            self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(program.clone()), self::ml_string::Text::from_units(vec![]))
                                                                                                                        }
                                                                                                                        self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Return(source_ml_temporary_result528) => {
                                                                                                                            let source_ml_temporary_result528 = *source_ml_temporary_result528;
                                                                                                                            source_ml_temporary_result528.clone()
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Return(source_ml_temporary_result528_2) => {
                                                                                                    let source_ml_temporary_result528_2 = *source_ml_temporary_result528_2;
                                                                                                    source_ml_temporary_result528_2.clone()
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Return(source_ml_temporary_result530) => {
                                                                        let source_ml_temporary_result530 = *source_ml_temporary_result530;
                                                                        source_ml_temporary_result530.clone()
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Return(source_ml_temporary_result530_2) => {
                                                    let source_ml_temporary_result530_2 = *source_ml_temporary_result530_2;
                                                    source_ml_temporary_result530_2.clone()
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin3 {
    MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin3Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin1 {
    MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin1Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptSymbolRegistryFunctionJoin7 {
    MlTemporaryCompileJavaScriptSymbolRegistryFunctionJoin7Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAppendJavaScriptObjectCoercionLoop27Result {
    MlTemporaryAppendJavaScriptObjectCoercionLoop27Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_append_java_script_object_coercion_loop27(mut code: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, mut allocation: f64, mut source_ml_temporary_values16: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_index16: self::ml::Big) -> self::MlTemporaryAppendJavaScriptObjectCoercionLoop27Result {
    loop {
        return if (source_ml_temporary_index16 < ({ let ml_reference_array602 = source_ml_temporary_values16.clone(); let ml_reference_projection602 = self::ml::Big::from_u128(ml_reference_array602.borrow().len() as u128); ml_reference_projection602 })) {
            {
                let jump = { let ml_reference_array603 = source_ml_temporary_values16.clone(); let ml_reference_position603 = source_ml_temporary_index16.clone(); let ml_reference_projection603 = self::ml_array::at(&*ml_reference_array603.borrow(), ml_reference_position603.to_index()); ml_reference_projection603 };
                {
                    let source_ml_temporary_append_java_script_object_coercion_ignored28 = { let ml_array_owner604 = code.clone(); let ml_array_argument604_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Jump(allocation); let ml_array_argument604_1: f64 = jump; let ml_array_argument604_2: f64 = (jump + 1f64); let ml_array_length = ml_array_owner604.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument604_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument604_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner604.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument604_0); } } ml_array_owner604 };
                    {
                        let source_ml_temporary_index16_2 = source_ml_temporary_index16.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index16 = source_ml_temporary_index16_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAppendJavaScriptObjectCoercionLoop27Result::MlTemporaryAppendJavaScriptObjectCoercionLoop27Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAppendJavaScriptObjectCoercionLoop20Result {
    MlTemporaryAppendJavaScriptObjectCoercionLoop20Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAppendJavaScriptObjectCoercionLoop29Result {
    MlTemporaryAppendJavaScriptObjectCoercionLoop29Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_append_java_script_object_coercion_loop29(mut choice: std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptSemanticValue)>>, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlTemporaryAppendJavaScriptObjectCoercionLoop29Result {
    loop {
        return if (index < ((({ let ml_reference_value605 = choice.borrow().clone(); ml_reference_value605 }).0).length())) {
            {
                let source_ml_temporary_append_java_script_object_coercion_ignored34 = { let ml_method_owner606 = units.clone(); let ml_method_argument606_0 = {
                    let source_ml_source1_string_unit_operand2481 = ({ let ml_reference_value607 = choice.borrow().clone(); ml_reference_value607 }).0;
                    {
                        let source_ml_source1_string_unit_operand2482 = index;
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
                                    let source_ml_source1_string_unit_operand2488 = {
                                        let source_ml_source1_string_range_operand2483 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand2484 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand2485 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position2486 = source_ml_source1_string_range_operand2484;
                                                    {
                                                        let source_ml_source1_string_range_position2487 = source_ml_source1_string_range_operand2485;
                                                        source_ml_source1_string_range_operand2483.slice(source_ml_source1_string_range_position2486, source_ml_source1_string_range_position2487)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand2489 = 0f64;
                                        source_ml_source1_string_unit_operand2488.code_point_at(source_ml_source1_string_unit_operand2489)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand2481.clone(), source_ml_source1_string_unit_operand2482)
                    }
                }; let ml_inserted = vec![ml_method_argument606_0]; let mut ml_method_values606 = ml_method_owner606.borrow_mut(); let ml_length = ml_method_values606.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values606.extend(ml_inserted); ml_length as f64 };
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryAppendJavaScriptObjectCoercionLoop29Result::MlTemporaryAppendJavaScriptObjectCoercionLoop29Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_append_java_script_object_coercion_loop20(mut code: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, mut type_register: f64, mut key_register: f64, mut condition_register: f64, mut prototype_register: f64, mut jumps: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_values15: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptSemanticValue)>>>>>, mut source_ml_temporary_index15: self::ml::Big) -> self::MlTemporaryAppendJavaScriptObjectCoercionLoop20Result {
    loop {
        return if (source_ml_temporary_index15 < ({ let ml_reference_array608 = source_ml_temporary_values15.clone(); let ml_reference_projection608 = self::ml::Big::from_u128(ml_reference_array608.borrow().len() as u128); ml_reference_projection608 })) {
            {
                let choice = { let ml_reference_array609 = source_ml_temporary_values15.clone(); let ml_reference_position609 = source_ml_temporary_index15.clone(); let ml_reference_projection609 = self::ml_array::at(&*ml_reference_array609.borrow(), ml_reference_position609.to_index()); ml_reference_projection609 };
                {
                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let index = 0f64;
                        {
                            let source_ml_source1_s2492 = self::source_ml_temporary_append_java_script_object_coercion_loop29(choice.clone(), units.clone(), index);
                            match source_ml_source1_s2492.clone() {
                                self::MlTemporaryAppendJavaScriptObjectCoercionLoop29Result::MlTemporaryAppendJavaScriptObjectCoercionLoop29Done => {
                                    {
                                        let source_ml_temporary_append_java_script_object_coercion_ignored30 = { let ml_method_owner610 = code.clone(); let ml_method_argument610_0 = self::JavaScriptCallInstruction::Constant(key_register, Box::new(self::JavaScriptSemanticValue::Text(units.clone()))); let ml_method_argument610_1 = self::JavaScriptCallInstruction::StrictEqual(condition_register, type_register, key_register); let ml_inserted = vec![ml_method_argument610_0, ml_method_argument610_1]; let mut ml_method_values610 = ml_method_owner610.borrow_mut(); let ml_length = ml_method_values610.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values610.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let branch = { let ml_reference_array611 = code.clone(); let ml_reference_projection611 = ml_reference_array611.borrow().len() as f64; ml_reference_projection611 };
                                            {
                                                let source_ml_temporary_append_java_script_object_coercion_ignored31 = { let ml_method_owner612 = code.clone(); let ml_method_argument612_0 = self::JavaScriptCallInstruction::Branch(condition_register, (branch + 1f64), (branch + 3f64)); let ml_method_argument612_1 = self::JavaScriptCallInstruction::Constant(prototype_register, Box::new(({ let ml_reference_value613 = choice.borrow().clone(); ml_reference_value613 }).1)); let ml_inserted = vec![ml_method_argument612_0, ml_method_argument612_1]; let mut ml_method_values612 = ml_method_owner612.borrow_mut(); let ml_length = ml_method_values612.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values612.extend(ml_inserted); ml_length as f64 };
                                                {
                                                    let source_ml_temporary_append_java_script_object_coercion_ignored32 = { let ml_method_owner614 = jumps.clone(); let ml_method_argument614_0 = { let ml_reference_array615 = code.clone(); let ml_reference_projection615 = ml_reference_array615.borrow().len() as f64; ml_reference_projection615 }; let ml_inserted = vec![ml_method_argument614_0]; let mut ml_method_values614 = ml_method_owner614.borrow_mut(); let ml_length = ml_method_values614.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values614.extend(ml_inserted); ml_length as f64 };
                                                    {
                                                        let source_ml_temporary_append_java_script_object_coercion_ignored33 = { let ml_method_owner616 = code.clone(); let ml_method_argument616_0 = self::JavaScriptCallInstruction::Jump(0f64); let ml_inserted = vec![ml_method_argument616_0]; let mut ml_method_values616 = ml_method_owner616.borrow_mut(); let ml_length = ml_method_values616.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values616.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let source_ml_temporary_index15_2 = source_ml_temporary_index15.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                source_ml_temporary_index15 = source_ml_temporary_index15_2.clone();
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAppendJavaScriptObjectCoercionLoop20Result::MlTemporaryAppendJavaScriptObjectCoercionLoop20Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAppendJavaScriptObjectCoercionLoop17Result {
    MlTemporaryAppendJavaScriptObjectCoercionLoop17Done,
    MlTemporaryAppendJavaScriptObjectCoercionLoop17Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_append_java_script_object_coercion_loop17(mut source_ml_temporary_values13: std::rc::Rc<std::cell::RefCell<Vec<self::MlOptional<self::JavaScriptSemanticValue>>>>, mut source_ml_temporary_index13: self::ml::Big) -> self::MlTemporaryAppendJavaScriptObjectCoercionLoop17Result {
    loop {
        return if (source_ml_temporary_index13 < ({ let ml_reference_array617 = source_ml_temporary_values13.clone(); let ml_reference_projection617 = self::ml::Big::from_u128(ml_reference_array617.borrow().len() as u128); ml_reference_projection617 })) {
            {
                let value = { let ml_reference_array618 = source_ml_temporary_values13.clone(); let ml_reference_position618 = source_ml_temporary_index13.clone(); let ml_reference_projection618 = self::ml_array::at(&*ml_reference_array618.borrow(), ml_reference_position618.to_index()); ml_reference_projection618 };
                if match {
                    let source_ml_source1_logical2501 = {
                        let source_ml_source1_comparison_operand2496 = {
                            let source_ml_source1_optional_chain_receiver2495 = (value.clone(),);
                            if match (source_ml_source1_optional_chain_receiver2495.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                Some({
                                    let source_ml_temporary_optional_chain14 = match (source_ml_source1_optional_chain_receiver2495.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                    match source_ml_temporary_optional_chain14.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                                        }
                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                                        }
                                        self::JavaScriptSemanticValue::Number(_) => {
                                            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                                        }
                                        self::JavaScriptSemanticValue::Integer(_) => {
                                            self::ml_string::Text::from_units(vec![105, 110, 116, 101, 103, 101, 114])
                                        }
                                        self::JavaScriptSemanticValue::Text(_) => {
                                            self::ml_string::Text::from_units(vec![116, 101, 120, 116])
                                        }
                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                            self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 84, 101, 120, 116])
                                        }
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])
                                        }
                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                            self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])
                                        }
                                    }
                                })
                            } else {
                                None::<self::ml_string::Text>
                            }
                        };
                        !(match (source_ml_source1_comparison_operand2496.clone(), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                    };
                    if source_ml_source1_logical2501 {
                        Some(source_ml_source1_logical2501)
                    } else {
                        {
                            let source_ml_source1_nullish_field_receiver2498 = (value.clone(),);
                            if match (source_ml_source1_nullish_field_receiver2498.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                {
                                    let source_ml_source1_nullish_field_present2499 = match (source_ml_source1_nullish_field_receiver2498.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                    match source_ml_source1_nullish_field_present2499.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            None::<bool>
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            None::<bool>
                                        }
                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                            None::<bool>
                                        }
                                        self::JavaScriptSemanticValue::Number(_) => {
                                            None::<bool>
                                        }
                                        self::JavaScriptSemanticValue::Integer(_) => {
                                            None::<bool>
                                        }
                                        self::JavaScriptSemanticValue::Text(_) => {
                                            None::<bool>
                                        }
                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                            None::<bool>
                                        }
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            None::<bool>
                                        }
                                        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value2500) => {
                                            Some(source_ml_source1_data_property_value2500)
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source1_nullish_field_receiver2498.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])
                                } else {
                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])
                                }).0, message_present: true }).raise()
                            }
                        }
                    }
                } { Some(ml_present) => ml_present, None => false } {
                    self::MlTemporaryAppendJavaScriptObjectCoercionLoop17Result::MlTemporaryAppendJavaScriptObjectCoercionLoop17Return(false)
                } else {
                    {
                        let source_ml_temporary_index13_2 = source_ml_temporary_index13.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index13 = source_ml_temporary_index13_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAppendJavaScriptObjectCoercionLoop17Result::MlTemporaryAppendJavaScriptObjectCoercionLoop17Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptArraySortFunctionJoin74 {
    MlTemporaryCompileJavaScriptArraySortFunctionJoin74Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptArraySortFunctionJoin47 {
    MlTemporaryCompileJavaScriptArraySortFunctionJoin47Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptArraySortFunctionJoin40 {
    MlTemporaryCompileJavaScriptArraySortFunctionJoin40Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptArraySortFunctionJoin38 {
    MlTemporaryCompileJavaScriptArraySortFunctionJoin38Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptArrayCopyFunctionJoin120 {
    MlTemporaryCompileJavaScriptArrayCopyFunctionJoin120Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptArrayCopyFunctionJoin110 {
    MlTemporaryCompileJavaScriptArrayCopyFunctionJoin110Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop126Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop126Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop126Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop126(mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop126Result {
    loop {
        return if (at < ({ let ml_reference_array619 = units.clone(); let ml_reference_projection619 = ml_reference_array619.borrow().len() as f64; ml_reference_projection619 })) {
            {
                let unit = ({ let ml_reference_array620 = units.clone(); let ml_reference_position620 = at; let ml_reference_projection620 = self::ml_array::optional_at(&*ml_reference_array620.borrow(), self::ml_array::number_index(ml_reference_position620)); ml_reference_projection620 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop126Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop126Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop126Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop126Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop127Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop127Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop127Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop127(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop127Result {
    loop {
        return if (at < length) {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Text(units) => {
                        ({ let ml_reference_array621 = units.clone(); let ml_reference_position621 = at; let ml_reference_projection621 = self::ml_array::optional_at(&*ml_reference_array621.borrow(), self::ml_array::number_index(ml_reference_position621)); ml_reference_projection621 }).clone().unwrap_or_else(|| (-1f64))
                    }
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f2508) => {
                        {
                            let text = source_ml_source1_f2508.clone();
                            {
                                let source_ml_source1_string_unit_operand2562 = text.clone();
                                {
                                    let source_ml_source1_string_unit_operand2563 = at;
                                    (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if ((index < (0f64)) || (index >= (text_2.length()))) {
                                            f64::NAN
                                        } else {
                                            ({
                                                let source_ml_source1_string_unit_operand2569 = {
                                                    let source_ml_source1_string_range_operand2564 = text_2.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand2565 = index;
                                                        {
                                                            let source_ml_source1_string_range_operand2566 = (index + 1f64);
                                                            {
                                                                let source_ml_source1_string_range_position2567 = source_ml_source1_string_range_operand2565;
                                                                {
                                                                    let source_ml_source1_string_range_position2568 = source_ml_source1_string_range_operand2566;
                                                                    source_ml_source1_string_range_operand2564.slice(source_ml_source1_string_range_position2567, source_ml_source1_string_range_position2568)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source1_string_unit_operand2570 = 0f64;
                                                    source_ml_source1_string_unit_operand2569.code_point_at(source_ml_source1_string_unit_operand2570)
                                                }
                                            }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    } }))(source_ml_source1_string_unit_operand2562.clone(), source_ml_source1_string_unit_operand2563)
                                }
                            }
                        }
                    }
                    _ => {
                        (-1f64)
                    }
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Text(units_2) => {
                            ({ let ml_reference_array622 = units_2.clone(); let ml_reference_position622 = at; let ml_reference_projection622 = self::ml_array::optional_at(&*ml_reference_array622.borrow(), self::ml_array::number_index(ml_reference_position622)); ml_reference_projection622 }).clone().unwrap_or_else(|| (-1f64))
                        }
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_f2591) => {
                            {
                                let text_2 = source_ml_source1_f2591.clone();
                                {
                                    let source_ml_source1_string_unit_operand2645 = text_2.clone();
                                    {
                                        let source_ml_source1_string_unit_operand2646 = at;
                                        (std::rc::Rc::new(move |text_3: self::ml_string::Text, position: f64| -> f64 { {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if ((index < (0f64)) || (index >= (text_3.length()))) {
                                                f64::NAN
                                            } else {
                                                ({
                                                    let source_ml_source1_string_unit_operand2652 = {
                                                        let source_ml_source1_string_range_operand2647 = text_3.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand2648 = index;
                                                            {
                                                                let source_ml_source1_string_range_operand2649 = (index + 1f64);
                                                                {
                                                                    let source_ml_source1_string_range_position2650 = source_ml_source1_string_range_operand2648;
                                                                    {
                                                                        let source_ml_source1_string_range_position2651 = source_ml_source1_string_range_operand2649;
                                                                        source_ml_source1_string_range_operand2647.slice(source_ml_source1_string_range_position2650, source_ml_source1_string_range_position2651)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_string_unit_operand2653 = 0f64;
                                                        source_ml_source1_string_unit_operand2652.code_point_at(source_ml_source1_string_unit_operand2653)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        } }))(source_ml_source1_string_unit_operand2645.clone(), source_ml_source1_string_unit_operand2646)
                                    }
                                }
                            }
                        }
                        _ => {
                            (-1f64)
                        }
                    };
                    if (a != b) {
                        self::MlTemporaryTestJavaScriptSemanticTextEqualLoop127Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop127Return(false)
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop127Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop127Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptCallCodeFunctionLoop133Result {
    MlTemporaryFindJavaScriptCallCodeFunctionLoop133Done,
    MlTemporaryFindJavaScriptCallCodeFunctionLoop133Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_call_code_function_loop133(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = {
                        let source_ml_source1_array_read_value2680 = match program.clone() {
                            self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value2679, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2679.clone()
                            }
                        };
                        { let ml_reference_array623 = source_ml_source1_array_read_value2680.clone(); let ml_reference_position623 = middle; let ml_reference_projection623 = self::ml_array::optional_at(&*ml_reference_array623.borrow(), self::ml_array::number_index(ml_reference_position623)); ml_reference_projection623 }
                    };
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value2683, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2683.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value2685, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2685.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Result {
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Result {
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Done,
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Result {
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Done,
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_call_code_functions_loop138(mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut end: f64, mut index: f64) -> self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Result {
    loop {
        return if (index < end) {
            {
                let value = { let ml_reference_array624 = ordered.clone(); let ml_reference_position624 = index; let ml_reference_projection624 = self::ml_array::optional_at(&*ml_reference_array624.borrow(), self::ml_array::number_index(ml_reference_position624)); ml_reference_projection624 };
                if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value625 = functions.borrow().clone(); ml_reference_value625 }, 0f64, f64::INFINITY))))
                } else {
                    {
                        let source_ml_temporary_order_java_script_call_code_functions_ignored139 = { let ml_method_owner626 = merged.clone(); let ml_method_argument626_0 = value.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument626_0]; let mut ml_method_values626 = ml_method_owner626.borrow_mut(); let ml_length = ml_method_values626.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values626.extend(ml_inserted); ml_length as f64 };
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Result {
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Done,
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_call_code_functions_loop137(mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut middle: f64, mut index: f64) -> self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Result {
    loop {
        return if (index < middle) {
            {
                let value = { let ml_reference_array627 = ordered.clone(); let ml_reference_position627 = index; let ml_reference_projection627 = self::ml_array::optional_at(&*ml_reference_array627.borrow(), self::ml_array::number_index(ml_reference_position627)); ml_reference_projection627 };
                if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value628 = functions.borrow().clone(); ml_reference_value628 }, 0f64, f64::INFINITY))))
                } else {
                    {
                        let source_ml_temporary_order_java_script_call_code_functions_ignored140 = { let ml_method_owner629 = merged.clone(); let ml_method_argument629_0 = value.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument629_0]; let mut ml_method_values629 = ml_method_owner629.borrow_mut(); let ml_length = ml_method_values629.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values629.extend(ml_inserted); ml_length as f64 };
                        {
                            let index_2 = (index + 1f64);
                            {
                                index = index_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Result {
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Done(f64, f64),
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_call_code_functions_loop136(mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut middle: f64, mut end: f64, mut left: f64, mut right: f64) -> self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Result {
    loop {
        return if ((left < middle) && (right < end)) {
            {
                let first = { let ml_reference_array630 = ordered.clone(); let ml_reference_position630 = left; let ml_reference_projection630 = self::ml_array::optional_at(&*ml_reference_array630.borrow(), self::ml_array::number_index(ml_reference_position630)); ml_reference_projection630 };
                {
                    let second = { let ml_reference_array631 = ordered.clone(); let ml_reference_position631 = right; let ml_reference_projection631 = self::ml_array::optional_at(&*ml_reference_array631.borrow(), self::ml_array::number_index(ml_reference_position631)); ml_reference_projection631 };
                    if match (first.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value632 = functions.borrow().clone(); ml_reference_value632 }, 0f64, f64::INFINITY))))
                    } else {
                        if match (second.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value633 = functions.borrow().clone(); ml_reference_value633 }, 0f64, f64::INFINITY))))
                        } else {
                            if ((match first.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value2696, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2696.clone()
                                }
                            }) <= (match second.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value2697, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2697.clone()
                                }
                            })) {
                                {
                                    let source_ml_temporary_order_java_script_call_code_functions_ignored141 = { let ml_method_owner634 = merged.clone(); let ml_method_argument634_0 = first.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument634_0]; let mut ml_method_values634 = ml_method_owner634.borrow_mut(); let ml_length = ml_method_values634.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values634.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let left_2 = (left + 1f64);
                                        {
                                            left = left_2;
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let source_ml_temporary_order_java_script_call_code_functions_ignored142 = { let ml_method_owner635 = merged.clone(); let ml_method_argument635_0 = second.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument635_0]; let mut ml_method_values635 = ml_method_owner635.borrow_mut(); let ml_length = ml_method_values635.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values635.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let right_2 = (right + 1f64);
                                        {
                                            right = right_2;
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
            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Done(left, right)
        };
    }
}
