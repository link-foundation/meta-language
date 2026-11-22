// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionIteratorCallCompilation;
use super::JavaScriptCollectionIteratorMethodOwner;
use super::JavaScriptSemanticValue;
use super::MlOptional;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Result;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Result;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Result;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Result;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Result;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_code_destination;
use super::relocate_java_script_array_mutation_instruction;
use super::replace_java_script_boxed_array_receiver_register;
use super::source_ml_temporary_order_java_script_call_code_functions_loop136;
use super::source_ml_temporary_order_java_script_call_code_functions_loop137;
use super::source_ml_temporary_order_java_script_call_code_functions_loop138;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_call_code_functions_loop135(mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut width: f64, mut merged: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut start: f64) -> self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Result {
    loop {
        return if (start < ({ let ml_reference_array636 = ordered.clone(); let ml_reference_projection636 = ml_reference_array636.borrow().len() as f64; ml_reference_projection636 })) {
            {
                let middle = self::ml_math::min((start + width), { let ml_reference_array637 = ordered.clone(); let ml_reference_projection637 = ml_reference_array637.borrow().len() as f64; ml_reference_projection637 });
                {
                    let end = self::ml_math::min((start + (2f64 * width)), { let ml_reference_array638 = ordered.clone(); let ml_reference_projection638 = ml_reference_array638.borrow().len() as f64; ml_reference_projection638 });
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let source_ml_source1_s2698 = self::source_ml_temporary_order_java_script_call_code_functions_loop136(functions.clone(), ordered.clone(), merged.clone(), middle, end, left, right);
                                match source_ml_source1_s2698.clone() {
                                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Done(left_2, right_2) => {
                                        {
                                            let source_ml_source1_s2699 = self::source_ml_temporary_order_java_script_call_code_functions_loop137(functions.clone(), ordered.clone(), merged.clone(), middle, left_2);
                                            match source_ml_source1_s2699.clone() {
                                                self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Done => {
                                                    {
                                                        let source_ml_source1_s2700 = self::source_ml_temporary_order_java_script_call_code_functions_loop138(functions.clone(), ordered.clone(), merged.clone(), end, right_2);
                                                        match source_ml_source1_s2700.clone() {
                                                            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Done => {
                                                                {
                                                                    let start_2 = (start + (2f64 * width));
                                                                    {
                                                                        start = start_2;
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop138Return(source_ml_temporary_result138) => {
                                                                self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Return(source_ml_temporary_result138.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop137Return(source_ml_temporary_result139) => {
                                                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Return(source_ml_temporary_result139.clone())
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop136Return(source_ml_temporary_result140) => {
                                        self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Return(source_ml_temporary_result140.clone())
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_call_code_functions_loop134(mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut ordered: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut width: f64) -> self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Result {
    loop {
        return if (width < ({ let ml_reference_array639 = ordered.clone(); let ml_reference_projection639 = ml_reference_array639.borrow().len() as f64; ml_reference_projection639 })) {
            {
                let merged = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallFunction>::new()));
                {
                    let start = 0f64;
                    {
                        let source_ml_source1_s2701 = self::source_ml_temporary_order_java_script_call_code_functions_loop135(functions.clone(), ordered.clone(), width, merged.clone(), start);
                        match source_ml_source1_s2701.clone() {
                            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Done => {
                                {
                                    let width_2 = (width * 2f64);
                                    {
                                        (ordered, width) = (merged.clone(), width_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop135Return(source_ml_temporary_result135) => {
                                self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Return(source_ml_temporary_result135.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Done(ordered.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptArrayMutationFunctionJoin150 {
    MlTemporaryCompileJavaScriptArrayMutationFunctionJoin150Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptArrayCopyWithinFunctionLoop223Result {
    MlTemporaryCompileJavaScriptArrayCopyWithinFunctionLoop223Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_array_copy_within_function_loop223(mut code: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, mut index: f64) -> self::MlTemporaryCompileJavaScriptArrayCopyWithinFunctionLoop223Result {
    loop {
        return if (index < (2f64)) {
            {
                let destination = (3f64 + index);
                {
                    let source_ml_temporary_compile_java_script_array_copy_within_function_ignored245 = { let ml_method_owner640 = code.clone(); let ml_method_argument640_0 = self::JavaScriptCallInstruction::Argument(destination, index); let ml_method_argument640_1 = self::JavaScriptCallInstruction::Primitive(destination, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![destination]))); let ml_method_argument640_2 = self::JavaScriptCallInstruction::Primitive(destination, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 97, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, destination]))); let ml_inserted = vec![ml_method_argument640_0, ml_method_argument640_1, ml_method_argument640_2]; let mut ml_method_values640 = ml_method_owner640.borrow_mut(); let ml_length = ml_method_values640.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values640.extend(ml_inserted); ml_length as f64 };
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
            self::MlTemporaryCompileJavaScriptArrayCopyWithinFunctionLoop223Result::MlTemporaryCompileJavaScriptArrayCopyWithinFunctionLoop223Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptBoxedArrayMutationFunctionLoop247Result {
    MlTemporaryCompileJavaScriptBoxedArrayMutationFunctionLoop247Done,
    MlTemporaryCompileJavaScriptBoxedArrayMutationFunctionLoop247Return(Option<self::JavaScriptCallFunction>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_boxed_array_mutation_function_loop247(mut held: f64, mut instructions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, mut offset: f64, mut source_ml_temporary_values246: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, mut source_ml_temporary_index246: self::ml::Big) -> self::MlTemporaryCompileJavaScriptBoxedArrayMutationFunctionLoop247Result {
    loop {
        return if (source_ml_temporary_index246 < ({ let ml_reference_array641 = source_ml_temporary_values246.clone(); let ml_reference_projection641 = self::ml::Big::from_u128(ml_reference_array641.borrow().len() as u128); ml_reference_projection641 })) {
            {
                let instruction = { let ml_reference_array642 = source_ml_temporary_values246.clone(); let ml_reference_position642 = source_ml_temporary_index246.clone(); let ml_reference_projection642 = self::ml_array::at(&*ml_reference_array642.borrow(), ml_reference_position642.to_index()); ml_reference_projection642 };
                if ((self::read_java_script_call_code_destination(instruction.clone())) == (0f64)) {
                    self::MlTemporaryCompileJavaScriptBoxedArrayMutationFunctionLoop247Result::MlTemporaryCompileJavaScriptBoxedArrayMutationFunctionLoop247Return(None::<self::JavaScriptCallFunction>)
                } else {
                    {
                        let relocated = self::relocate_java_script_array_mutation_instruction(instruction.clone(), held, offset);
                        if match (relocated.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlTemporaryCompileJavaScriptBoxedArrayMutationFunctionLoop247Result::MlTemporaryCompileJavaScriptBoxedArrayMutationFunctionLoop247Return(None::<self::JavaScriptCallFunction>)
                        } else {
                            {
                                let source_ml_temporary_compile_java_script_boxed_array_mutation_function_ignored248 = { let ml_method_owner643 = instructions.clone(); let ml_method_argument643_0 = relocated.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument643_0]; let mut ml_method_values643 = ml_method_owner643.borrow_mut(); let ml_length = ml_method_values643.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values643.extend(ml_inserted); ml_length as f64 };
                                {
                                    let source_ml_temporary_index246_2 = source_ml_temporary_index246.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index246 = source_ml_temporary_index246_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptBoxedArrayMutationFunctionLoop247Result::MlTemporaryCompileJavaScriptBoxedArrayMutationFunctionLoop247Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRelocateJavaScriptArrayMutationInstructionLoop250Result {
    MlTemporaryRelocateJavaScriptArrayMutationInstructionLoop250Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_relocate_java_script_array_mutation_instruction_loop250(mut held: f64, mut operands: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_values249: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_index249: self::ml::Big) -> self::MlTemporaryRelocateJavaScriptArrayMutationInstructionLoop250Result {
    loop {
        return if (source_ml_temporary_index249 < ({ let ml_reference_array644 = source_ml_temporary_values249.clone(); let ml_reference_projection644 = self::ml::Big::from_u128(ml_reference_array644.borrow().len() as u128); ml_reference_projection644 })) {
            {
                let operand = { let ml_reference_array645 = source_ml_temporary_values249.clone(); let ml_reference_position645 = source_ml_temporary_index249.clone(); let ml_reference_projection645 = self::ml_array::at(&*ml_reference_array645.borrow(), ml_reference_position645.to_index()); ml_reference_projection645 };
                {
                    let source_ml_temporary_relocate_java_script_array_mutation_instruction_ignored251 = { let ml_method_owner646 = operands.clone(); let ml_method_argument646_0 = self::replace_java_script_boxed_array_receiver_register(operand, held); let ml_inserted = vec![ml_method_argument646_0]; let mut ml_method_values646 = ml_method_owner646.borrow_mut(); let ml_length = ml_method_values646.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values646.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_temporary_index249_2 = source_ml_temporary_index249.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index249 = source_ml_temporary_index249_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryRelocateJavaScriptArrayMutationInstructionLoop250Result::MlTemporaryRelocateJavaScriptArrayMutationInstructionLoop250Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258 {
    MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptObjectCallFunctionLoop310Result {
    MlTemporaryCompileJavaScriptObjectCallFunctionLoop310Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_object_call_function_loop310(mut code: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, mut allocation: f64, mut source_ml_temporary_values303: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_index303: self::ml::Big) -> self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop310Result {
    loop {
        return if (source_ml_temporary_index303 < ({ let ml_reference_array647 = source_ml_temporary_values303.clone(); let ml_reference_projection647 = self::ml::Big::from_u128(ml_reference_array647.borrow().len() as u128); ml_reference_projection647 })) {
            {
                let jump = { let ml_reference_array648 = source_ml_temporary_values303.clone(); let ml_reference_position648 = source_ml_temporary_index303.clone(); let ml_reference_projection648 = self::ml_array::at(&*ml_reference_array648.borrow(), ml_reference_position648.to_index()); ml_reference_projection648 };
                {
                    let source_ml_temporary_compile_java_script_object_call_function_ignored311 = { let ml_array_owner649 = code.clone(); let ml_array_argument649_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Jump(allocation); let ml_array_argument649_1: f64 = jump; let ml_array_argument649_2: f64 = (jump + 1f64); let ml_array_length = ml_array_owner649.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument649_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument649_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner649.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument649_0); } } ml_array_owner649 };
                    {
                        let source_ml_temporary_index303_2 = source_ml_temporary_index303.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index303 = source_ml_temporary_index303_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop310Result::MlTemporaryCompileJavaScriptObjectCallFunctionLoop310Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptObjectCallFunctionLoop305Result {
    MlTemporaryCompileJavaScriptObjectCallFunctionLoop305Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptObjectCallFunctionLoop312Result {
    MlTemporaryCompileJavaScriptObjectCallFunctionLoop312Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_object_call_function_loop312(mut choice: std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptSemanticValue)>>, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop312Result {
    loop {
        return if (at < ((({ let ml_reference_value650 = choice.borrow().clone(); ml_reference_value650 }).0).length())) {
            {
                let source_ml_temporary_compile_java_script_object_call_function_ignored317 = { let ml_method_owner651 = units.clone(); let ml_method_argument651_0 = {
                    let source_ml_source1_string_unit_operand3052 = ({ let ml_reference_value652 = choice.borrow().clone(); ml_reference_value652 }).0;
                    {
                        let source_ml_source1_string_unit_operand3053 = at;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index < (0f64)) || (index >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source1_string_unit_operand3059 = {
                                        let source_ml_source1_string_range_operand3054 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand3055 = index;
                                            {
                                                let source_ml_source1_string_range_operand3056 = (index + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position3057 = source_ml_source1_string_range_operand3055;
                                                    {
                                                        let source_ml_source1_string_range_position3058 = source_ml_source1_string_range_operand3056;
                                                        source_ml_source1_string_range_operand3054.slice(source_ml_source1_string_range_position3057, source_ml_source1_string_range_position3058)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand3060 = 0f64;
                                        source_ml_source1_string_unit_operand3059.code_point_at(source_ml_source1_string_unit_operand3060)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand3052.clone(), source_ml_source1_string_unit_operand3053)
                    }
                }; let ml_inserted = vec![ml_method_argument651_0]; let mut ml_method_values651 = ml_method_owner651.borrow_mut(); let ml_length = ml_method_values651.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values651.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop312Result::MlTemporaryCompileJavaScriptObjectCallFunctionLoop312Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_object_call_function_loop305(mut code: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, mut jumps: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_temporary_values302: std::rc::Rc<std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, self::JavaScriptSemanticValue)>>>>>, mut source_ml_temporary_index302: self::ml::Big) -> self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop305Result {
    loop {
        return if (source_ml_temporary_index302 < ({ let ml_reference_array653 = source_ml_temporary_values302.clone(); let ml_reference_projection653 = self::ml::Big::from_u128(ml_reference_array653.borrow().len() as u128); ml_reference_projection653 })) {
            {
                let choice = { let ml_reference_array654 = source_ml_temporary_values302.clone(); let ml_reference_position654 = source_ml_temporary_index302.clone(); let ml_reference_projection654 = self::ml_array::at(&*ml_reference_array654.borrow(), ml_reference_position654.to_index()); ml_reference_projection654 };
                {
                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s3063 = self::source_ml_temporary_compile_java_script_object_call_function_loop312(choice.clone(), units.clone(), at);
                            match source_ml_source1_s3063.clone() {
                                self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop312Result::MlTemporaryCompileJavaScriptObjectCallFunctionLoop312Done => {
                                    {
                                        let source_ml_temporary_compile_java_script_object_call_function_ignored313 = { let ml_method_owner655 = code.clone(); let ml_method_argument655_0 = self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Text(units.clone()))); let ml_method_argument655_1 = self::JavaScriptCallInstruction::StrictEqual(4f64, 2f64, 3f64); let ml_inserted = vec![ml_method_argument655_0, ml_method_argument655_1]; let mut ml_method_values655 = ml_method_owner655.borrow_mut(); let ml_length = ml_method_values655.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values655.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let branch = { let ml_reference_array656 = code.clone(); let ml_reference_projection656 = ml_reference_array656.borrow().len() as f64; ml_reference_projection656 };
                                            {
                                                let source_ml_temporary_compile_java_script_object_call_function_ignored314 = { let ml_method_owner657 = code.clone(); let ml_method_argument657_0 = self::JavaScriptCallInstruction::Branch(4f64, (branch + 1f64), (branch + 3f64)); let ml_method_argument657_1 = self::JavaScriptCallInstruction::Constant(5f64, Box::new(({ let ml_reference_value658 = choice.borrow().clone(); ml_reference_value658 }).1)); let ml_inserted = vec![ml_method_argument657_0, ml_method_argument657_1]; let mut ml_method_values657 = ml_method_owner657.borrow_mut(); let ml_length = ml_method_values657.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values657.extend(ml_inserted); ml_length as f64 };
                                                {
                                                    let source_ml_temporary_compile_java_script_object_call_function_ignored315 = { let ml_method_owner659 = jumps.clone(); let ml_method_argument659_0 = { let ml_reference_array660 = code.clone(); let ml_reference_projection660 = ml_reference_array660.borrow().len() as f64; ml_reference_projection660 }; let ml_inserted = vec![ml_method_argument659_0]; let mut ml_method_values659 = ml_method_owner659.borrow_mut(); let ml_length = ml_method_values659.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values659.extend(ml_inserted); ml_length as f64 };
                                                    {
                                                        let source_ml_temporary_compile_java_script_object_call_function_ignored316 = { let ml_method_owner661 = code.clone(); let ml_method_argument661_0 = self::JavaScriptCallInstruction::Jump(0f64); let ml_inserted = vec![ml_method_argument661_0]; let mut ml_method_values661 = ml_method_owner661.borrow_mut(); let ml_length = ml_method_values661.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values661.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let source_ml_temporary_index302_2 = source_ml_temporary_index302.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                source_ml_temporary_index302 = source_ml_temporary_index302_2.clone();
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
            self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop305Result::MlTemporaryCompileJavaScriptObjectCallFunctionLoop305Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Result {
    MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Done,
    MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Return(Option<self::JavaScriptCallFunction>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_object_call_function_loop304(mut source_ml_temporary_values300: std::rc::Rc<std::cell::RefCell<Vec<self::MlOptional<self::JavaScriptSemanticValue>>>>, mut source_ml_temporary_index300: self::ml::Big) -> self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Result {
    loop {
        return if (source_ml_temporary_index300 < ({ let ml_reference_array662 = source_ml_temporary_values300.clone(); let ml_reference_projection662 = self::ml::Big::from_u128(ml_reference_array662.borrow().len() as u128); ml_reference_projection662 })) {
            {
                let value = { let ml_reference_array663 = source_ml_temporary_values300.clone(); let ml_reference_position663 = source_ml_temporary_index300.clone(); let ml_reference_projection663 = self::ml_array::at(&*ml_reference_array663.borrow(), ml_reference_position663.to_index()); ml_reference_projection663 };
                if match {
                    let source_ml_source1_logical3072 = {
                        let source_ml_source1_comparison_operand3067 = {
                            let source_ml_source1_optional_chain_receiver3066 = (value.clone(),);
                            if match (source_ml_source1_optional_chain_receiver3066.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                Some({
                                    let source_ml_temporary_optional_chain301 = match (source_ml_source1_optional_chain_receiver3066.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                    match source_ml_temporary_optional_chain301.clone() {
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
                        !(match (source_ml_source1_comparison_operand3067.clone(), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                    };
                    if source_ml_source1_logical3072 {
                        Some(source_ml_source1_logical3072)
                    } else {
                        {
                            let source_ml_source1_nullish_field_receiver3069 = (value.clone(),);
                            if match (source_ml_source1_nullish_field_receiver3069.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                {
                                    let source_ml_source1_nullish_field_present3070 = match (source_ml_source1_nullish_field_receiver3069.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                    match source_ml_source1_nullish_field_present3070.clone() {
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
                                        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3071) => {
                                            Some(source_ml_source1_data_property_value3071)
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source1_nullish_field_receiver3069.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])
                                } else {
                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])
                                }).0, message_present: true }).raise()
                            }
                        }
                    }
                } { Some(ml_present) => ml_present, None => false } {
                    self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Result::MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Return(None::<self::JavaScriptCallFunction>)
                } else {
                    {
                        let source_ml_temporary_index300_2 = source_ml_temporary_index300.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index300 = source_ml_temporary_index300_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Result::MlTemporaryCompileJavaScriptObjectCallFunctionLoop304Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptStringConversionFunctionJoin323 {
    MlTemporaryCompileJavaScriptStringConversionFunctionJoin323Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptStringConversionFunctionJoin329 {
    MlTemporaryCompileJavaScriptStringConversionFunctionJoin329Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332 {
    MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin332Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin335 {
    MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin335Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin351 {
    MlTemporaryCompileJavaScriptNumericIntrinsicFunctionJoin351Next,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptStringMethodFunctionJoin357 {
    MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next,
    MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Return(Option<self::JavaScriptCallFunction>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptStringMethodFunctionLoop365Result {
    MlTemporaryCompileJavaScriptStringMethodFunctionLoop365Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_string_method_function_loop365(mut unicode_profile: Option<self::ml_string::Text>, mut profile_units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryCompileJavaScriptStringMethodFunctionLoop365Result {
    loop {
        return if (at < ((unicode_profile.clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))).length())) {
            {
                let source_ml_temporary_compile_java_script_string_method_function_ignored367 = { let ml_method_owner664 = profile_units.clone(); let ml_method_argument664_0 = {
                    let source_ml_source1_string_unit_operand3075 = unicode_profile.clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                    {
                        let source_ml_source1_string_unit_operand3076 = at;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index < (0f64)) || (index >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source1_string_unit_operand3082 = {
                                        let source_ml_source1_string_range_operand3077 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand3078 = index;
                                            {
                                                let source_ml_source1_string_range_operand3079 = (index + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position3080 = source_ml_source1_string_range_operand3078;
                                                    {
                                                        let source_ml_source1_string_range_position3081 = source_ml_source1_string_range_operand3079;
                                                        source_ml_source1_string_range_operand3077.slice(source_ml_source1_string_range_position3080, source_ml_source1_string_range_position3081)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand3083 = 0f64;
                                        source_ml_source1_string_unit_operand3082.code_point_at(source_ml_source1_string_unit_operand3083)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand3075.clone(), source_ml_source1_string_unit_operand3076)
                    }
                }; let ml_inserted = vec![ml_method_argument664_0]; let mut ml_method_values664 = ml_method_owner664.borrow_mut(); let ml_length = ml_method_values664.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values664.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptStringMethodFunctionLoop365Result::MlTemporaryCompileJavaScriptStringMethodFunctionLoop365Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptStringMethodFunctionJoin396 {
    MlTemporaryCompileJavaScriptStringMethodFunctionJoin396Next,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptStringMethodFunctionJoin394 {
    MlTemporaryCompileJavaScriptStringMethodFunctionJoin394Next,
    MlTemporaryCompileJavaScriptStringMethodFunctionJoin394Return(Option<self::JavaScriptCallFunction>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptStringMethodFunctionLoop411Result {
    MlTemporaryCompileJavaScriptStringMethodFunctionLoop411Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_string_method_function_loop411(message: &self::ml_string::Text, mut message_units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryCompileJavaScriptStringMethodFunctionLoop411Result {
    loop {
        return if (at < (message.length())) {
            {
                let source_ml_temporary_compile_java_script_string_method_function_ignored417 = { let ml_method_owner665 = message_units.clone(); let ml_method_argument665_0 = {
                    let source_ml_source1_string_unit_operand3084 = message.clone();
                    {
                        let source_ml_source1_string_unit_operand3085 = at;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index < (0f64)) || (index >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source1_string_unit_operand3091 = {
                                        let source_ml_source1_string_range_operand3086 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand3087 = index;
                                            {
                                                let source_ml_source1_string_range_operand3088 = (index + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position3089 = source_ml_source1_string_range_operand3087;
                                                    {
                                                        let source_ml_source1_string_range_position3090 = source_ml_source1_string_range_operand3088;
                                                        source_ml_source1_string_range_operand3086.slice(source_ml_source1_string_range_position3089, source_ml_source1_string_range_position3090)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand3092 = 0f64;
                                        source_ml_source1_string_unit_operand3091.code_point_at(source_ml_source1_string_unit_operand3092)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand3084.clone(), source_ml_source1_string_unit_operand3085)
                    }
                }; let ml_inserted = vec![ml_method_argument665_0]; let mut ml_method_values665 = ml_method_owner665.borrow_mut(); let ml_length = ml_method_values665.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values665.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptStringMethodFunctionLoop411Result::MlTemporaryCompileJavaScriptStringMethodFunctionLoop411Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptStringMethodFunctionLoop356Result {
    MlTemporaryCompileJavaScriptStringMethodFunctionLoop356Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_string_method_function_loop356(name: &self::ml_string::Text, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlTemporaryCompileJavaScriptStringMethodFunctionLoop356Result {
    loop {
        return if (at < (name.length())) {
            {
                let source_ml_temporary_compile_java_script_string_method_function_ignored418 = { let ml_method_owner666 = units.clone(); let ml_method_argument666_0 = {
                    let source_ml_source1_string_unit_operand3093 = name.clone();
                    {
                        let source_ml_source1_string_unit_operand3094 = at;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index < (0f64)) || (index >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source1_string_unit_operand3100 = {
                                        let source_ml_source1_string_range_operand3095 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand3096 = index;
                                            {
                                                let source_ml_source1_string_range_operand3097 = (index + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position3098 = source_ml_source1_string_range_operand3096;
                                                    {
                                                        let source_ml_source1_string_range_position3099 = source_ml_source1_string_range_operand3097;
                                                        source_ml_source1_string_range_operand3095.slice(source_ml_source1_string_range_position3098, source_ml_source1_string_range_position3099)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand3101 = 0f64;
                                        source_ml_source1_string_unit_operand3100.code_point_at(source_ml_source1_string_unit_operand3101)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand3093.clone(), source_ml_source1_string_unit_operand3094)
                    }
                }; let ml_inserted = vec![ml_method_argument666_0]; let mut ml_method_values666 = ml_method_owner666.borrow_mut(); let ml_length = ml_method_values666.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values666.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptStringMethodFunctionLoop356Result::MlTemporaryCompileJavaScriptStringMethodFunctionLoop356Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin421 {
    MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin421Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin433 {
    MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin433Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin432 {
    MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin432Next,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin427 {
    MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin427Next(f64, f64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419 {
    MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419Next,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Result {
    MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Done,
    MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Return(Box<self::JavaScriptCollectionIteratorCallCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop446(mut empty: self::JavaScriptCallProgram, mut program: self::JavaScriptCallProgram, mut source_ml_temporary_values444: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionIteratorMethodOwner>>>, mut source_ml_temporary_index444: self::ml::Big) -> self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Result {
    loop {
        return if (source_ml_temporary_index444 < ({ let ml_reference_array667 = source_ml_temporary_values444.clone(); let ml_reference_projection667 = self::ml::Big::from_u128(ml_reference_array667.borrow().len() as u128); ml_reference_projection667 })) {
            {
                let owner = { let ml_reference_array668 = source_ml_temporary_values444.clone(); let ml_reference_position668 = source_ml_temporary_index444.clone(); let ml_reference_projection668 = self::ml_array::at(&*ml_reference_array668.borrow(), ml_reference_position668.to_index()); ml_reference_projection668 };
                if ((self::find_java_script_call_code_function(program.clone(), match owner.clone() {
                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3105, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3105.clone()
                    }
                })) < (0f64)) {
                    self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 119, 110, 101, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 99, 111, 100, 101]))))
                } else {
                    {
                        let source_ml_temporary_index444_2 = source_ml_temporary_index444.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index444 = source_ml_temporary_index444_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result {
    MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionIteratorMethodOwner>>>),
    MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(Box<self::JavaScriptCollectionIteratorCallCompilation>),
}
