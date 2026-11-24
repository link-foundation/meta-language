// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallCellInitializer;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceProperty;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Result;
use super::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Result;
use super::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Result;
use super::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_operand;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_order_java_script_call_code_functions_loop12;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_call_code_functions_loop11(mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut middle: f64, mut index: f64) -> self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Result {
    loop {
        return if (index < middle) {
            {
                let value = { let ml_reference_array372 = ordered.clone(); let ml_reference_position372 = index; let ml_reference_projection372 = self::ml_array::optional_at(&*ml_reference_array372.borrow(), self::ml_array::number_index(ml_reference_position372)); ml_reference_projection372 };
                if match (value.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value373 = functions.borrow().clone(); ml_reference_value373 }, 0f64, f64::INFINITY))))
                } else {
                    {
                        let merged_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value374 = merged.borrow().clone(); ml_reference_value374 }, vec![value.clone().expect("a proven present value")])));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (merged, index) = (merged_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Done(merged.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Result {
    MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, f64, f64),
    MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Return(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_call_code_functions_loop10(mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut middle: f64, mut end: f64, mut left: f64, mut right: f64) -> self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Result {
    loop {
        return if ((left < middle) && (right < end)) {
            {
                let first = { let ml_reference_array375 = ordered.clone(); let ml_reference_position375 = left; let ml_reference_projection375 = self::ml_array::optional_at(&*ml_reference_array375.borrow(), self::ml_array::number_index(ml_reference_position375)); ml_reference_projection375 };
                {
                    let second = { let ml_reference_array376 = ordered.clone(); let ml_reference_position376 = right; let ml_reference_projection376 = self::ml_array::optional_at(&*ml_reference_array376.borrow(), self::ml_array::number_index(ml_reference_position376)); ml_reference_projection376 };
                    if match (first.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value377 = functions.borrow().clone(); ml_reference_value377 }, 0f64, f64::INFINITY))))
                    } else {
                        if match (second.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Return(std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value378 = functions.borrow().clone(); ml_reference_value378 }, 0f64, f64::INFINITY))))
                        } else {
                            if ((match first.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value18749, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value18749.clone()
                                }
                            }) <= (match second.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value18750, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value18750.clone()
                                }
                            })) {
                                {
                                    let merged_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value379 = merged.borrow().clone(); ml_reference_value379 }, vec![first.clone().expect("a proven present value")])));
                                    {
                                        let left_2 = (left + 1f64);
                                        {
                                            (merged, left) = (merged_2.clone(), left_2);
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let merged_3 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value380 = merged.borrow().clone(); ml_reference_value380 }, vec![second.clone().expect("a proven present value")])));
                                    {
                                        let right_2 = (right + 1f64);
                                        {
                                            (merged, right) = (merged_3.clone(), right_2);
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
            self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Done(merged.clone(), left, right)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_call_code_functions_loop9(mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut width: f64, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut start: f64) -> self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Result {
    loop {
        return if (start < ({ let ml_reference_array381 = ordered.clone(); let ml_reference_projection381 = ml_reference_array381.borrow().len() as f64; ml_reference_projection381 })) {
            {
                let middle = self::ml_math::min((start + width), { let ml_reference_array382 = ordered.clone(); let ml_reference_projection382 = ml_reference_array382.borrow().len() as f64; ml_reference_projection382 });
                {
                    let end = self::ml_math::min((start + (2f64 * width)), { let ml_reference_array383 = ordered.clone(); let ml_reference_projection383 = ml_reference_array383.borrow().len() as f64; ml_reference_projection383 });
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let source_ml_source2_s18751 = self::source_ml_source1_temporary_order_java_script_call_code_functions_loop10(functions.clone(), ordered.clone(), merged.clone(), middle, end, left, right);
                                match source_ml_source2_s18751.clone() {
                                    self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Done(merged_2, left_2, right_2) => {
                                        {
                                            let source_ml_source2_s18752 = self::source_ml_source1_temporary_order_java_script_call_code_functions_loop11(functions.clone(), ordered.clone(), merged_2.clone(), middle, left_2);
                                            match source_ml_source2_s18752.clone() {
                                                self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Done(merged_3) => {
                                                    {
                                                        let source_ml_source2_s18753 = self::source_ml_source1_temporary_order_java_script_call_code_functions_loop12(functions.clone(), ordered.clone(), merged_3.clone(), end, right_2);
                                                        match source_ml_source2_s18753.clone() {
                                                            self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Done(merged_4) => {
                                                                {
                                                                    let start_2 = (start + (2f64 * width));
                                                                    {
                                                                        (merged, start) = (merged_4.clone(), start_2);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop12Return(source_ml_source1_temporary_result12) => {
                                                                self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Return(source_ml_source1_temporary_result12.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop11Return(source_ml_source1_temporary_result12_2) => {
                                                    self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Return(source_ml_source1_temporary_result12_2.clone())
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop10Return(source_ml_source1_temporary_result12_3) => {
                                        self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Return(source_ml_source1_temporary_result12_3.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Done(merged.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_order_java_script_call_code_functions_loop8(mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut width: f64) -> self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Result {
    loop {
        return if (width < ({ let ml_reference_array384 = ordered.clone(); let ml_reference_projection384 = ml_reference_array384.borrow().len() as f64; ml_reference_projection384 })) {
            {
                let merged = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallFunction>::new()));
                {
                    let start = 0f64;
                    {
                        let source_ml_source2_s18754 = self::source_ml_source1_temporary_order_java_script_call_code_functions_loop9(functions.clone(), ordered.clone(), width, merged.clone(), start);
                        match source_ml_source2_s18754.clone() {
                            self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Done(merged_2) => {
                                {
                                    let width_2 = (width * 2f64);
                                    {
                                        (ordered, width) = (merged_2.clone(), width_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop9Return(source_ml_source1_temporary_result9) => {
                                self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Return(source_ml_source1_temporary_result9.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Result::MlSource1TemporaryOrderJavaScriptCallCodeFunctionsLoop8Done(ordered.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result {
    MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop15(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut snapshot: bool, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values13: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index13: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result {
    loop {
        return if (source_ml_source1_temporary_index13 < ({ let ml_reference_array385 = source_ml_source1_temporary_values13.clone(); let ml_reference_projection385 = self::ml::Big::from_u128(ml_reference_array385.borrow().len() as u128); ml_reference_projection385 })) {
            {
                let argument = { let ml_reference_array386 = source_ml_source1_temporary_values13.clone(); let ml_reference_position386 = source_ml_source1_temporary_index13.clone(); let ml_reference_projection386 = self::ml_array::at(&*ml_reference_array386.borrow(), ml_reference_position386.to_index()); ml_reference_projection386 };
                {
                    let value = self::lower_java_script_call_source_operand(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64), snapshot);
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18757, _) => {
                                let source_ml_source2_data_property_value18757 = *source_ml_source2_data_property_value18757;
                                source_ml_source2_data_property_value18757.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value387 = arguments_.borrow().clone(); ml_reference_value387 }, vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18758) => {
                                    source_ml_source2_data_property_value18758
                                }
                            }])));
                            {
                                let source_ml_source1_temporary_index13_2 = source_ml_source1_temporary_index13.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index13) = (current_2.clone(), arguments__2.clone(), source_ml_source1_temporary_index13_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Result::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop15Done(Box::new(current.clone()), arguments_.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result {
    MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_constructor_expression_loop16(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values14: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index14: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result {
    loop {
        return if (source_ml_source1_temporary_index14 < ({ let ml_reference_array388 = source_ml_source1_temporary_values14.clone(); let ml_reference_projection388 = self::ml::Big::from_u128(ml_reference_array388.borrow().len() as u128); ml_reference_projection388 })) {
            {
                let argument = { let ml_reference_array389 = source_ml_source1_temporary_values14.clone(); let ml_reference_position389 = source_ml_source1_temporary_index14.clone(); let ml_reference_projection389 = self::ml_array::at(&*ml_reference_array389.borrow(), ml_reference_position389.to_index()); ml_reference_projection389 };
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let saved = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18761, _) => {
                                let source_ml_source2_data_property_value18761 = *source_ml_source2_data_property_value18761;
                                source_ml_source2_data_property_value18761.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18762, _) => {
                                    let source_ml_source2_data_property_value18762 = *source_ml_source2_data_property_value18762;
                                    source_ml_source2_data_property_value18762.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18763) => {
                                    source_ml_source2_data_property_value18763
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18764) => {
                                    source_ml_source2_data_property_value18764
                                }
                            }));
                            {
                                let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value390 = arguments_.borrow().clone(); ml_reference_value390 }, vec![match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18765) => {
                                        source_ml_source2_data_property_value18765
                                    }
                                }])));
                                {
                                    let source_ml_source1_temporary_index14_2 = source_ml_source1_temporary_index14.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, arguments_, source_ml_source1_temporary_index14) = (current_2.clone(), arguments__2.clone(), source_ml_source1_temporary_index14_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Result::MlSource1TemporaryLowerJavaScriptCallSourceConstructorExpressionLoop16Done(Box::new(current.clone()), arguments_.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result {
    MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>),
    MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_lexical_cell_expression_loop17(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut realm: self::ml::Big, mut identities: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut existing: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut initializers: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallCellInitializer>>>, mut current: self::JavaScriptCallSourceBuilder, mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut at: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result {
    loop {
        return if (at < ({ let ml_reference_array391 = identities.clone(); let ml_reference_projection391 = ml_reference_array391.borrow().len() as f64; ml_reference_projection391 })) {
            {
                let identity = ({ let ml_reference_array392 = identities.clone(); let ml_reference_position392 = at; let ml_reference_projection392 = self::ml_array::optional_at(&*ml_reference_array392.borrow(), self::ml_array::number_index(ml_reference_position392)); ml_reference_projection392 }).clone().unwrap_or_else(|| (-1f64));
                {
                    let binding = { let ml_values393 = { let ml_reference_value394 = bindings.borrow().clone(); ml_reference_value394 }; let ml_callback393 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                        self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value18769, _, _, _) => {
                            source_ml_source2_data_property_value18769
                        }
                    }) == identity) }) }; ml_values393.iter().cloned().enumerate().find_map(|(ml_index393, ml_value393)| if ml_callback393(ml_value393.clone()) { Some(ml_value393) } else { None }) };
                    {
                        let replace = self::ml_array::includes(&{ let ml_reference_value395 = existing.borrow().clone(); ml_reference_value395 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()));
                        if ((!(self::test_java_script_call_code_coordinate(identity, 65536f64)) || ((self::ml_array::index_of(&{ let ml_reference_value396 = identities.borrow().clone(); ml_reference_value396 }, &identity, 0f64, false, |a, b| a == b)) != at)) || if replace {
                            ((match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match binding.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value18771, _, _) => {
                                    source_ml_source2_data_property_value18771
                                }
                            }) == (0f64))) || !(match binding.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value18772, _) => {
                                    source_ml_source2_data_property_value18772
                                }
                            }))
                        } else {
                            !(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                        }) {
                            self::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 99, 101, 108, 108, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 101, 100, 32, 115, 99, 111, 112, 101, 32, 111, 114, 32, 102, 111, 114, 109, 97, 108, 32, 98, 105, 110, 100, 105, 110, 103, 115])))))
                        } else {
                            {
                                let initial = { let ml_values397 = { let ml_reference_value398 = initializers.borrow().clone(); ml_reference_value398 }; let ml_callback397 = { let identity = identity; std::rc::Rc::new(move |row: self::JavaScriptCallCellInitializer| -> bool { ((match row.clone() {
                                    self::JavaScriptCallCellInitializer::SourceCellInitializer(source_ml_source2_data_property_value18774, _, _) => {
                                        source_ml_source2_data_property_value18774
                                    }
                                }) == identity) }) }; ml_values397.iter().cloned().enumerate().find_map(|(ml_index397, ml_value397)| if ml_callback397(ml_value397.clone()) { Some(ml_value397) } else { None }) };
                                {
                                    let missing = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Undefined));
                                    {
                                        let initial_value = if match (initial.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            missing.clone()
                                        } else {
                                            match initial.clone().expect("a proven present value") {
                                                self::JavaScriptCallCellInitializer::SourceCellInitializer(_, source_ml_source2_data_property_value18775, _) => {
                                                    let source_ml_source2_data_property_value18775 = *source_ml_source2_data_property_value18775;
                                                    source_ml_source2_data_property_value18775.clone()
                                                }
                                            }
                                        };
                                        {
                                            let initialized = if match (initial.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                false
                                            } else {
                                                match initial.clone().expect("a proven present value") {
                                                    self::JavaScriptCallCellInitializer::SourceCellInitializer(_, _, source_ml_source2_data_property_value18776) => {
                                                        source_ml_source2_data_property_value18776
                                                    }
                                                }
                                            };
                                            {
                                                let cell = {
                                                    let source_ml_source2_constructor_field18777 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Null));
                                                    {
                                                        let source_ml_source2_constructor_field18783 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                            let source_ml_source2_constructor_field18779 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![118f64, 97f64, 108f64, 117f64, 101f64])))));
                                                            self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field18779.clone()), Box::new(initial_value.clone()), None::<(Option<bool>,)>)
                                                        }, {
                                                            let source_ml_source2_constructor_field18781 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![105f64, 110f64, 105f64, 116f64, 105f64, 97f64, 108f64, 105f64, 122f64, 101f64, 100f64])))));
                                                            {
                                                                let source_ml_source2_constructor_field18782 = self::JavaScriptCallSource::SourceConstant(Box::new(self::JavaScriptSemanticValue::Boolean(initialized)));
                                                                self::JavaScriptCallSourceProperty::SourceObjectProperty(Box::new(source_ml_source2_constructor_field18781.clone()), Box::new(source_ml_source2_constructor_field18782.clone()), None::<(Option<bool>,)>)
                                                            }
                                                        }]));
                                                        self::JavaScriptCallSource::SourceObject(Box::new(source_ml_source2_constructor_field18777.clone()), source_ml_source2_constructor_field18783.clone(), realm.clone())
                                                    }
                                                };
                                                {
                                                    let allocated = self::lower_java_script_call_source_expression(cell.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                                                    {
                                                        let current_2 = match allocated.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18784, _) => {
                                                                let source_ml_source2_data_property_value18784 = *source_ml_source2_data_property_value18784;
                                                                source_ml_source2_data_property_value18784.clone()
                                                            }
                                                        };
                                                        if (replace && !(match (binding.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                            {
                                                                let current_3 = self::append_java_script_call_source_instruction(current_2.clone(), self::JavaScriptCallInstruction::Copy(match binding.clone().expect("a proven present value") {
                                                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value18785, _, _) => {
                                                                        source_ml_source2_data_property_value18785
                                                                    }
                                                                }, match allocated.clone() {
                                                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18786) => {
                                                                        source_ml_source2_data_property_value18786
                                                                    }
                                                                }));
                                                                {
                                                                    let at_2 = (at + 1f64);
                                                                    {
                                                                        (current, at) = (current_3.clone(), at_2);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let entries_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value399 = entries.borrow().clone(); ml_reference_value399 }, vec![{
                                                                    let source_ml_source2_constructor_field18789 = match allocated.clone() {
                                                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18788) => {
                                                                            source_ml_source2_data_property_value18788
                                                                        }
                                                                    };
                                                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, source_ml_source2_constructor_field18789, false, None::<(Option<bool>,)>)
                                                                }])));
                                                                {
                                                                    let at_3 = (at + 1f64);
                                                                    {
                                                                        (current, entries, at) = (current_2.clone(), entries_2.clone(), at_3);
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
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Result::MlSource1TemporaryLowerJavaScriptCallSourceLexicalCellExpressionLoop17Done(Box::new(current.clone()), entries.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result {
    MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Done,
    MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_binding_read_expression_loop18(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut state: self::JavaScriptCallSourceBuilder, mut identity: f64, mut index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = { let ml_reference_array400 = bindings.clone(); let ml_reference_position400 = index; let ml_reference_projection400 = self::ml_array::optional_at(&*ml_reference_array400.borrow(), self::ml_array::number_index(ml_reference_position400)); ml_reference_projection400 };
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((match entry.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value18794, _, _, _) => {
                        source_ml_source2_data_property_value18794
                    }
                }) == identity)) {
                    if ({
                        let source_ml_source2_comparison_operand18796 = match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, _, source_ml_source2_data_property_value18795) => {
                                ((source_ml_source2_data_property_value18795.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                            }
                        };
                        match (source_ml_source2_comparison_operand18796.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                    } && !(self::ml_array::includes(&{ let ml_reference_value401 = ((match state.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value18799, _, _) => {
                            ((source_ml_source2_data_property_value18799.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))).borrow().clone(); ml_reference_value401 }, &identity, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan())))) {
                        self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box::new(self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![117, 110, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 97, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 109, 112, 111, 114, 97, 108, 45, 100, 101, 97, 100, 45, 122, 111, 110, 101, 32, 101, 110, 118, 105, 114, 111, 110, 109, 101, 110, 116, 32, 112, 114, 111, 116, 111, 99, 111, 108])))))
                    } else {
                        if ({
                            let source_ml_source2_comparison_operand18801 = match state.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value18800, _, _, _) => {
                                    ((source_ml_source2_data_property_value18800.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                }
                            };
                            match (source_ml_source2_comparison_operand18801.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                        } && match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallSourceBinding::SourceBindingRegister(_, _, source_ml_source2_data_property_value18803, _) => {
                                source_ml_source2_data_property_value18803
                            }
                        }) {
                            {
                                let saved = self::allocate_java_script_call_source_register(state.clone());
                                self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box::new({
                                    let source_ml_source2_nominal_field18808 = match saved.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18809, _) => {
                                            let source_ml_source2_data_property_value18809 = *source_ml_source2_data_property_value18809;
                                            source_ml_source2_data_property_value18809.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field18810 = match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18811) => {
                                                source_ml_source2_data_property_value18811
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override18812 = self::append_java_script_call_source_instruction(match saved.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18804, _) => {
                                                    let source_ml_source2_data_property_value18804 = *source_ml_source2_data_property_value18804;
                                                    source_ml_source2_data_property_value18804.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18805) => {
                                                    source_ml_source2_data_property_value18805
                                                }
                                            }, match entry.clone().expect("a proven present value") {
                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value18806, _, _) => {
                                                    source_ml_source2_data_property_value18806
                                                }
                                            }));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override18812.clone()), source_ml_source2_nominal_field18810)
                                        }
                                    }
                                }))
                            }
                        } else {
                            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value18813, _, _) => {
                                    source_ml_source2_data_property_value18813
                                }
                            })))
                        }
                    }
                } else {
                    {
                        let index_2 = (index - 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingReadExpressionLoop18Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Result {
    MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop28(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values26: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index26: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Result {
    loop {
        return if (source_ml_source1_temporary_index26 < ({ let ml_reference_array402 = source_ml_source1_temporary_values26.clone(); let ml_reference_projection402 = self::ml::Big::from_u128(ml_reference_array402.borrow().len() as u128); ml_reference_projection402 })) {
            {
                let argument = { let ml_reference_array403 = source_ml_source1_temporary_values26.clone(); let ml_reference_position403 = source_ml_source1_temporary_index26.clone(); let ml_reference_projection403 = self::ml_array::at(&*ml_reference_array403.borrow(), ml_reference_position403.to_index()); ml_reference_projection403 };
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18816, _) => {
                                let source_ml_source2_data_property_value18816 = *source_ml_source2_data_property_value18816;
                                source_ml_source2_data_property_value18816.clone()
                            }
                        };
                        {
                            let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value404 = arguments_.borrow().clone(); ml_reference_value404 }, vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18817) => {
                                    source_ml_source2_data_property_value18817
                                }
                            }])));
                            {
                                let source_ml_source1_temporary_index26_2 = source_ml_source1_temporary_index26.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index26) = (current_2.clone(), arguments__2.clone(), source_ml_source1_temporary_index26_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Result::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Done(Box::new(current.clone()), arguments_.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop30(mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::JavaScriptCallSourceBuilder {
    loop {
        return if (index < ({ let ml_reference_array405 = arguments_.clone(); let ml_reference_projection405 = ml_reference_array405.borrow().len() as f64; ml_reference_projection405 })) {
            {
                let current_2 = self::append_java_script_call_source_instruction(current.clone(), self::JavaScriptCallInstruction::Copy((index + 1f64), ({ let ml_reference_array406 = arguments_.clone(); let ml_reference_position406 = index; let ml_reference_projection406 = self::ml_array::optional_at(&*ml_reference_array406.borrow(), self::ml_array::number_index(ml_reference_position406)); ml_reference_projection406 }).clone().unwrap_or_else(|| 0f64)));
                {
                    let index_2 = (index + 1f64);
                    {
                        (current, index) = (current_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Result {
    MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Done(Box<self::JavaScriptCallSourceBuilder>, std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop29(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_values27: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSource>>>, mut source_ml_source1_temporary_index27: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Result {
    loop {
        return if (source_ml_source1_temporary_index27 < ({ let ml_reference_array407 = source_ml_source1_temporary_values27.clone(); let ml_reference_projection407 = self::ml::Big::from_u128(ml_reference_array407.borrow().len() as u128); ml_reference_projection407 })) {
            {
                let argument = { let ml_reference_array408 = source_ml_source1_temporary_values27.clone(); let ml_reference_position408 = source_ml_source1_temporary_index27.clone(); let ml_reference_projection408 = self::ml_array::at(&*ml_reference_array408.borrow(), ml_reference_position408.to_index()); ml_reference_projection408 };
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let captured = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18822, _) => {
                                let source_ml_source2_data_property_value18822 = *source_ml_source2_data_property_value18822;
                                source_ml_source2_data_property_value18822.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value18823, _) => {
                                    let source_ml_source2_data_property_value18823 = *source_ml_source2_data_property_value18823;
                                    source_ml_source2_data_property_value18823.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18824) => {
                                    source_ml_source2_data_property_value18824
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18825) => {
                                    source_ml_source2_data_property_value18825
                                }
                            }));
                            {
                                let arguments__2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value409 = arguments_.borrow().clone(); ml_reference_value409 }, vec![match captured.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value18826) => {
                                        source_ml_source2_data_property_value18826
                                    }
                                }])));
                                {
                                    let source_ml_source1_temporary_index27_2 = source_ml_source1_temporary_index27.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, arguments_, source_ml_source1_temporary_index27) = (current_2.clone(), arguments__2.clone(), source_ml_source1_temporary_index27_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Result::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Done(Box::new(current.clone()), arguments_.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Result {
    MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Done(Box<self::JavaScriptCallSourceBuilder>),
    MlSource1TemporaryLowerJavaScriptCallSourceArrayExpressionLoop33Return(Box<self::JavaScriptCallSourceResult>),
}
