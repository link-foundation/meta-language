// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptSemanticValue;
use super::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419;
use super::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin421;
use super::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin427;
use super::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin432;
use super::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin433;
use super::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357;
use super::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394;
use super::MlTemporaryCompileJavaScriptStringMethodFunctionJoin396;
use super::MlTemporaryCompileJavaScriptStringMethodFunctionLoop356Result;
use super::MlTemporaryCompileJavaScriptStringMethodFunctionLoop365Result;
use super::MlTemporaryCompileJavaScriptStringMethodFunctionLoop411Result;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_temporary_compile_java_script_string_method_function_loop356;
use super::source_ml_temporary_compile_java_script_string_method_function_loop365;
use super::source_ml_temporary_compile_java_script_string_method_function_loop411;
use super::test_java_script_string_method_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_string_method_function(identity: self::ml::Big, realm: self::ml::Big, operation: &self::ml_string::Text, match_key: Option<self::JavaScriptSemanticValue>, unicode_profile: Option<self::ml_string::Text>) -> Option<self::JavaScriptCallFunction> {
    if !(self::test_java_script_string_method_operation(operation.clone())) {
        None::<self::JavaScriptCallFunction>
    } else {
        if ((((*operation) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 116, 111, 76, 111, 119, 101, 114, 67, 97, 115, 101]))) || ((*operation) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 116, 111, 85, 112, 112, 101, 114, 67, 97, 115, 101])))) && ((((match unicode_profile.clone() { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (({
            let source_ml_source1_nullish_field_receiver1707 = (unicode_profile.clone(),);
            if ((match (source_ml_source1_nullish_field_receiver1707.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                {
                    let source_ml_source1_nullish_field_present1708 = ((source_ml_source1_nullish_field_receiver1707.clone()).0).clone().expect("a proven present value");
                    source_ml_source1_nullish_field_present1708.length()
                }
            } else {
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
            }
        }) == (0f64))) || (({
            let source_ml_source1_nullish_field_receiver1711 = (unicode_profile.clone(),);
            if ((match (source_ml_source1_nullish_field_receiver1711.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                {
                    let source_ml_source1_nullish_field_present1712 = ((source_ml_source1_nullish_field_receiver1711.clone()).0).clone().expect("a proven present value");
                    source_ml_source1_nullish_field_present1712.length()
                }
            } else {
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
            }
        }) > (128f64)))) {
            None::<self::JavaScriptCallFunction>
        } else {
            {
                let name = {
                    let source_ml_source1_string_range_operand1714 = 7f64;
                    {
                        let source_ml_source1_string_range_position1715 = source_ml_source1_string_range_operand1714;
                        {
                            let source_ml_source1_string_range_position1716 = f64::INFINITY;
                            operation.slice(source_ml_source1_string_range_position1715, source_ml_source1_string_range_position1716)
                        }
                    }
                };
                {
                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s1717 = self::source_ml_temporary_compile_java_script_string_method_function_loop356(&(name), units.clone(), at);
                            match source_ml_source1_s1717.clone() {
                                self::MlTemporaryCompileJavaScriptStringMethodFunctionLoop356Result::MlTemporaryCompileJavaScriptStringMethodFunctionLoop356Done => {
                                    {
                                        let code = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
                                        {
                                            let source_ml_source1_s1729 = if ((name == (self::ml_string::Text::from_units(vec![102, 114, 111, 109, 67, 104, 97, 114, 67, 111, 100, 101]))) || (name == (self::ml_string::Text::from_units(vec![102, 114, 111, 109, 67, 111, 100, 101, 80, 111, 105, 110, 116])))) {
                                                {
                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored358 = { let ml_method_owner424 = code.clone(); let ml_method_argument424_0 = self::JavaScriptCallInstruction::Constant(1f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))))); let ml_method_argument424_1 = self::JavaScriptCallInstruction::ArgumentCount(5f64); let ml_method_argument424_2 = self::JavaScriptCallInstruction::Constant(4f64, Box::new(self::JavaScriptSemanticValue::Number(0f64))); let ml_method_argument424_3 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))); let ml_inserted = vec![ml_method_argument424_0, ml_method_argument424_1, ml_method_argument424_2, ml_method_argument424_3]; let mut ml_method_values424 = ml_method_owner424.borrow_mut(); let ml_length = ml_method_values424.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values424.extend(ml_inserted); ml_length as f64 };
                                                    {
                                                        let loop_ = { let ml_reference_array425 = code.clone(); let ml_reference_projection425 = ml_reference_array425.borrow().len() as f64; ml_reference_projection425 };
                                                        {
                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored359 = { let ml_method_owner426 = code.clone(); let ml_method_argument426_0 = self::JavaScriptCallInstruction::NumberBinary(6f64, 4f64, 5f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])); let ml_inserted = vec![ml_method_argument426_0]; let mut ml_method_values426 = ml_method_owner426.borrow_mut(); let ml_length = ml_method_values426.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values426.extend(ml_inserted); ml_length as f64 };
                                                            {
                                                                let branch = { let ml_reference_array427 = code.clone(); let ml_reference_projection427 = ml_reference_array427.borrow().len() as f64; ml_reference_projection427 };
                                                                {
                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored360 = { let ml_method_owner428 = code.clone(); let ml_method_argument428_0 = self::JavaScriptCallInstruction::Branch(6f64, (branch + 1f64), 0f64); let ml_method_argument428_1 = self::JavaScriptCallInstruction::ArgumentAt(7f64, 4f64); let ml_method_argument428_2 = self::JavaScriptCallInstruction::Primitive(7f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![7f64]))); let ml_method_argument428_3 = self::JavaScriptCallInstruction::Primitive(7f64, if (name == (self::ml_string::Text::from_units(vec![102, 114, 111, 109, 67, 104, 97, 114, 67, 111, 100, 101]))) {
                                                                        self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114, 32, 99, 111, 100, 101, 32, 116, 101, 120, 116])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116, 32, 116, 101, 120, 116])
                                                                    }, std::rc::Rc::new(std::cell::RefCell::new(vec![7f64]))); let ml_method_argument428_4 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 7f64]))); let ml_method_argument428_5 = self::JavaScriptCallInstruction::NumberBinary(4f64, 4f64, 8f64, self::ml_string::Text::from_units(vec![97, 100, 100])); let ml_method_argument428_6 = self::JavaScriptCallInstruction::Jump(loop_); let ml_inserted = vec![ml_method_argument428_0, ml_method_argument428_1, ml_method_argument428_2, ml_method_argument428_3, ml_method_argument428_4, ml_method_argument428_5, ml_method_argument428_6]; let mut ml_method_values428 = ml_method_owner428.borrow_mut(); let ml_length = ml_method_values428.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values428.extend(ml_inserted); ml_length as f64 };
                                                                    {
                                                                        let end = { let ml_reference_array429 = code.clone(); let ml_reference_projection429 = ml_reference_array429.borrow().len() as f64; ml_reference_projection429 };
                                                                        {
                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored361 = { let ml_array_owner430 = code.clone(); let ml_array_argument430_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, (branch + 1f64), end); let ml_array_argument430_1: f64 = branch; let ml_array_argument430_2: f64 = (branch + 1f64); let ml_array_length = ml_array_owner430.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument430_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument430_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner430.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument430_0); } } ml_array_owner430 };
                                                                            {
                                                                                let source_ml_temporary_compile_java_script_string_method_function_ignored362 = { let ml_method_owner431 = code.clone(); let ml_method_argument431_0 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument431_0]; let mut ml_method_values431 = ml_method_owner431.borrow_mut(); let ml_length = ml_method_values431.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values431.extend(ml_inserted); ml_length as f64 };
                                                                                self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if ((name == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103]))) || (name == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 79, 102])))) {
                                                    {
                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored363 = { let ml_method_owner432 = code.clone(); let ml_method_argument432_0 = self::JavaScriptCallInstruction::ObjectObservation(1f64, operation.clone(), 0f64, 0f64); let ml_method_argument432_1 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument432_0, ml_method_argument432_1]; let mut ml_method_values432 = ml_method_owner432.borrow_mut(); let ml_length = ml_method_values432.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values432.extend(ml_inserted); ml_length as f64 };
                                                        self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next
                                                    }
                                                } else {
                                                    {
                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored364 = { let ml_method_owner433 = code.clone(); let ml_method_argument433_0 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Text(units.clone()))); let ml_method_argument433_1 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 83, 116, 114, 105, 110, 103, 32, 114, 101, 99, 101, 105, 118, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64, 8f64]))); let ml_method_argument433_2 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_inserted = vec![ml_method_argument433_0, ml_method_argument433_1, ml_method_argument433_2]; let mut ml_method_values433 = ml_method_owner433.borrow_mut(); let ml_length = ml_method_values433.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values433.extend(ml_inserted); ml_length as f64 };
                                                        if ((name == (self::ml_string::Text::from_units(vec![116, 111, 76, 111, 119, 101, 114, 67, 97, 115, 101]))) || (name == (self::ml_string::Text::from_units(vec![116, 111, 85, 112, 112, 101, 114, 67, 97, 115, 101])))) {
                                                            {
                                                                let profile_units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                {
                                                                    let at_2 = 0f64;
                                                                    {
                                                                        let source_ml_source1_s1730 = self::source_ml_temporary_compile_java_script_string_method_function_loop365(unicode_profile.clone(), profile_units.clone(), at_2);
                                                                        match source_ml_source1_s1730.clone() {
                                                                            self::MlTemporaryCompileJavaScriptStringMethodFunctionLoop365Result::MlTemporaryCompileJavaScriptStringMethodFunctionLoop365Done => {
                                                                                {
                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored366 = { let ml_method_owner434 = code.clone(); let ml_method_argument434_0 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Text(profile_units.clone()))); let ml_method_argument434_1 = self::JavaScriptCallInstruction::Primitive(9f64, if (name == (self::ml_string::Text::from_units(vec![116, 111, 76, 111, 119, 101, 114, 67, 97, 115, 101]))) {
                                                                                        self::ml_string::Text::from_units(vec![109, 97, 112, 32, 108, 111, 119, 101, 114, 32, 83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101])
                                                                                    } else {
                                                                                        self::ml_string::Text::from_units(vec![109, 97, 112, 32, 117, 112, 112, 101, 114, 32, 83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 101])
                                                                                    }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 8f64]))); let ml_method_argument434_2 = self::JavaScriptCallInstruction::Return(9f64); let ml_inserted = vec![ml_method_argument434_0, ml_method_argument434_1, ml_method_argument434_2]; let mut ml_method_values434 = ml_method_owner434.borrow_mut(); let ml_length = ml_method_values434.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values434.extend(ml_inserted); ml_length as f64 };
                                                                                    self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if ((name == (self::ml_string::Text::from_units(vec![105, 115, 87, 101, 108, 108, 70, 111, 114, 109, 101, 100]))) || (name == (self::ml_string::Text::from_units(vec![116, 111, 87, 101, 108, 108, 70, 111, 114, 109, 101, 100])))) {
                                                                {
                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored368 = { let ml_method_owner435 = code.clone(); let ml_method_argument435_0 = self::JavaScriptCallInstruction::Primitive(9f64, if (name == (self::ml_string::Text::from_units(vec![105, 115, 87, 101, 108, 108, 70, 111, 114, 109, 101, 100]))) {
                                                                        self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![99, 111, 112, 121, 32, 119, 101, 108, 108, 32, 102, 111, 114, 109, 101, 100, 32, 83, 116, 114, 105, 110, 103])
                                                                    }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument435_1 = self::JavaScriptCallInstruction::Return(9f64); let ml_inserted = vec![ml_method_argument435_0, ml_method_argument435_1]; let mut ml_method_values435 = ml_method_owner435.borrow_mut(); let ml_length = ml_method_values435.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values435.extend(ml_inserted); ml_length as f64 };
                                                                    self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next
                                                                }
                                                            } else {
                                                                if (((name == (self::ml_string::Text::from_units(vec![116, 114, 105, 109]))) || (name == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 83, 116, 97, 114, 116])))) || (name == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 69, 110, 100])))) {
                                                                    {
                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored369 = { let ml_method_owner436 = code.clone(); let ml_method_argument436_0 = self::JavaScriptCallInstruction::Primitive(9f64, if (name == (self::ml_string::Text::from_units(vec![116, 114, 105, 109]))) {
                                                                            self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116])
                                                                        } else {
                                                                            if (name == (self::ml_string::Text::from_units(vec![116, 114, 105, 109, 83, 116, 97, 114, 116]))) {
                                                                                self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![116, 114, 105, 109, 32, 116, 101, 120, 116, 32, 101, 110, 100])
                                                                            }
                                                                        }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument436_1 = self::JavaScriptCallInstruction::Return(9f64); let ml_inserted = vec![ml_method_argument436_0, ml_method_argument436_1]; let mut ml_method_values436 = ml_method_owner436.borrow_mut(); let ml_length = ml_method_values436.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values436.extend(ml_inserted); ml_length as f64 };
                                                                        self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next
                                                                    }
                                                                } else {
                                                                    if (name == (self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116]))) {
                                                                        {
                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored370 = { let ml_method_owner437 = code.clone(); let ml_method_argument437_0 = self::JavaScriptCallInstruction::ArgumentCount(5f64); let ml_method_argument437_1 = self::JavaScriptCallInstruction::Constant(4f64, Box::new(self::JavaScriptSemanticValue::Number(0f64))); let ml_method_argument437_2 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))); let ml_inserted = vec![ml_method_argument437_0, ml_method_argument437_1, ml_method_argument437_2]; let mut ml_method_values437 = ml_method_owner437.borrow_mut(); let ml_length = ml_method_values437.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values437.extend(ml_inserted); ml_length as f64 };
                                                                            {
                                                                                let loop_2 = { let ml_reference_array438 = code.clone(); let ml_reference_projection438 = ml_reference_array438.borrow().len() as f64; ml_reference_projection438 };
                                                                                {
                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored371 = { let ml_method_owner439 = code.clone(); let ml_method_argument439_0 = self::JavaScriptCallInstruction::NumberBinary(6f64, 4f64, 5f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])); let ml_inserted = vec![ml_method_argument439_0]; let mut ml_method_values439 = ml_method_owner439.borrow_mut(); let ml_length = ml_method_values439.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values439.extend(ml_inserted); ml_length as f64 };
                                                                                    {
                                                                                        let branch_2 = { let ml_reference_array440 = code.clone(); let ml_reference_projection440 = ml_reference_array440.borrow().len() as f64; ml_reference_projection440 };
                                                                                        {
                                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored372 = { let ml_method_owner441 = code.clone(); let ml_method_argument441_0 = self::JavaScriptCallInstruction::Branch(6f64, (branch_2 + 1f64), 0f64); let ml_method_argument441_1 = self::JavaScriptCallInstruction::ArgumentAt(7f64, 4f64); let ml_method_argument441_2 = self::JavaScriptCallInstruction::Primitive(7f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![7f64]))); let ml_method_argument441_3 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 7f64]))); let ml_method_argument441_4 = self::JavaScriptCallInstruction::NumberBinary(4f64, 4f64, 8f64, self::ml_string::Text::from_units(vec![97, 100, 100])); let ml_method_argument441_5 = self::JavaScriptCallInstruction::Jump(loop_2); let ml_inserted = vec![ml_method_argument441_0, ml_method_argument441_1, ml_method_argument441_2, ml_method_argument441_3, ml_method_argument441_4, ml_method_argument441_5]; let mut ml_method_values441 = ml_method_owner441.borrow_mut(); let ml_length = ml_method_values441.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values441.extend(ml_inserted); ml_length as f64 };
                                                                                            {
                                                                                                let end_2 = { let ml_reference_array442 = code.clone(); let ml_reference_projection442 = ml_reference_array442.borrow().len() as f64; ml_reference_projection442 };
                                                                                                {
                                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored373 = { let ml_array_owner443 = code.clone(); let ml_array_argument443_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, (branch_2 + 1f64), end_2); let ml_array_argument443_1: f64 = branch_2; let ml_array_argument443_2: f64 = (branch_2 + 1f64); let ml_array_length = ml_array_owner443.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument443_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument443_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner443.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument443_0); } } ml_array_owner443 };
                                                                                                    {
                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored374 = { let ml_method_owner444 = code.clone(); let ml_method_argument444_0 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument444_0]; let mut ml_method_values444 = ml_method_owner444.borrow_mut(); let ml_length = ml_method_values444.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values444.extend(ml_inserted); ml_length as f64 };
                                                                                                        self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if ((name == (self::ml_string::Text::from_units(vec![112, 97, 100, 83, 116, 97, 114, 116]))) || (name == (self::ml_string::Text::from_units(vec![112, 97, 100, 69, 110, 100])))) {
                                                                            {
                                                                                let source_ml_temporary_compile_java_script_string_method_function_ignored375 = { let ml_method_owner445 = code.clone(); let ml_method_argument445_0 = self::JavaScriptCallInstruction::Argument(2f64, 0f64); let ml_method_argument445_1 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument445_2 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 108, 101, 110, 103, 116, 104]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument445_3 = self::JavaScriptCallInstruction::Primitive(5f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument445_4 = self::JavaScriptCallInstruction::NumberBinary(6f64, 5f64, 2f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])); let ml_inserted = vec![ml_method_argument445_0, ml_method_argument445_1, ml_method_argument445_2, ml_method_argument445_3, ml_method_argument445_4]; let mut ml_method_values445 = ml_method_owner445.borrow_mut(); let ml_length = ml_method_values445.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values445.extend(ml_inserted); ml_length as f64 };
                                                                                {
                                                                                    let useful = { let ml_reference_array446 = code.clone(); let ml_reference_projection446 = ml_reference_array446.borrow().len() as f64; ml_reference_projection446 };
                                                                                    {
                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored376 = { let ml_method_owner447 = code.clone(); let ml_method_argument447_0 = self::JavaScriptCallInstruction::Branch(6f64, (useful + 1f64), 0f64); let ml_method_argument447_1 = self::JavaScriptCallInstruction::Argument(3f64, 1f64); let ml_method_argument447_2 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Undefined)); let ml_method_argument447_3 = self::JavaScriptCallInstruction::StrictEqual(6f64, 3f64, 8f64); let ml_inserted = vec![ml_method_argument447_0, ml_method_argument447_1, ml_method_argument447_2, ml_method_argument447_3]; let mut ml_method_values447 = ml_method_owner447.borrow_mut(); let ml_length = ml_method_values447.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values447.extend(ml_inserted); ml_length as f64 };
                                                                                        {
                                                                                            let missing = { let ml_reference_array448 = code.clone(); let ml_reference_projection448 = ml_reference_array448.borrow().len() as f64; ml_reference_projection448 };
                                                                                            {
                                                                                                let source_ml_temporary_compile_java_script_string_method_function_ignored377 = { let ml_method_owner449 = code.clone(); let ml_method_argument449_0 = self::JavaScriptCallInstruction::Branch(6f64, 0f64, (missing + 1f64)); let ml_method_argument449_1 = self::JavaScriptCallInstruction::Primitive(3f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![3f64]))); let ml_inserted = vec![ml_method_argument449_0, ml_method_argument449_1]; let mut ml_method_values449 = ml_method_owner449.borrow_mut(); let ml_length = ml_method_values449.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values449.extend(ml_inserted); ml_length as f64 };
                                                                                                {
                                                                                                    let skip = { let ml_reference_array450 = code.clone(); let ml_reference_projection450 = ml_reference_array450.borrow().len() as f64; ml_reference_projection450 };
                                                                                                    {
                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored378 = { let ml_method_owner451 = code.clone(); let ml_method_argument451_0 = self::JavaScriptCallInstruction::Jump(0f64); let ml_inserted = vec![ml_method_argument451_0]; let mut ml_method_values451 = ml_method_owner451.borrow_mut(); let ml_length = ml_method_values451.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values451.extend(ml_inserted); ml_length as f64 };
                                                                                                        {
                                                                                                            let space = { let ml_reference_array452 = code.clone(); let ml_reference_projection452 = ml_reference_array452.borrow().len() as f64; ml_reference_projection452 };
                                                                                                            {
                                                                                                                let source_ml_temporary_compile_java_script_string_method_function_ignored379 = { let ml_method_owner453 = code.clone(); let ml_method_argument453_0 = self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![32f64]))))); let ml_inserted = vec![ml_method_argument453_0]; let mut ml_method_values453 = ml_method_owner453.borrow_mut(); let ml_length = ml_method_values453.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values453.extend(ml_inserted); ml_length as f64 };
                                                                                                                {
                                                                                                                    let apply = { let ml_reference_array454 = code.clone(); let ml_reference_projection454 = ml_reference_array454.borrow().len() as f64; ml_reference_projection454 };
                                                                                                                    {
                                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored380 = { let ml_method_owner455 = code.clone(); let ml_method_argument455_0 = self::JavaScriptCallInstruction::Primitive(9f64, if (name == (self::ml_string::Text::from_units(vec![112, 97, 100, 83, 116, 97, 114, 116]))) {
                                                                                                                            self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 115, 116, 97, 114, 116])
                                                                                                                        } else {
                                                                                                                            self::ml_string::Text::from_units(vec![112, 97, 100, 32, 116, 101, 120, 116, 32, 101, 110, 100])
                                                                                                                        }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64]))); let ml_method_argument455_1 = self::JavaScriptCallInstruction::Return(9f64); let ml_inserted = vec![ml_method_argument455_0, ml_method_argument455_1]; let mut ml_method_values455 = ml_method_owner455.borrow_mut(); let ml_length = ml_method_values455.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values455.extend(ml_inserted); ml_length as f64 };
                                                                                                                        {
                                                                                                                            let unchanged = { let ml_reference_array456 = code.clone(); let ml_reference_projection456 = ml_reference_array456.borrow().len() as f64; ml_reference_projection456 };
                                                                                                                            {
                                                                                                                                let source_ml_temporary_compile_java_script_string_method_function_ignored381 = { let ml_method_owner457 = code.clone(); let ml_method_argument457_0 = self::JavaScriptCallInstruction::Return(1f64); let ml_inserted = vec![ml_method_argument457_0]; let mut ml_method_values457 = ml_method_owner457.borrow_mut(); let ml_length = ml_method_values457.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values457.extend(ml_inserted); ml_length as f64 };
                                                                                                                                {
                                                                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored382 = { let ml_array_owner458 = code.clone(); let ml_array_argument458_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, (useful + 1f64), unchanged); let ml_array_argument458_1: f64 = useful; let ml_array_argument458_2: f64 = (useful + 1f64); let ml_array_length = ml_array_owner458.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument458_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument458_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner458.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument458_0); } } ml_array_owner458 };
                                                                                                                                    {
                                                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored383 = { let ml_array_owner459 = code.clone(); let ml_array_argument459_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, space, (missing + 1f64)); let ml_array_argument459_1: f64 = missing; let ml_array_argument459_2: f64 = (missing + 1f64); let ml_array_length = ml_array_owner459.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument459_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument459_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner459.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument459_0); } } ml_array_owner459 };
                                                                                                                                        {
                                                                                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored384 = { let ml_array_owner460 = code.clone(); let ml_array_argument460_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Jump(apply); let ml_array_argument460_1: f64 = skip; let ml_array_argument460_2: f64 = (skip + 1f64); let ml_array_length = ml_array_owner460.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument460_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument460_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner460.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument460_0); } } ml_array_owner460 };
                                                                                                                                            self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next
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
                                                                            }
                                                                        } else {
                                                                            if (((name == (self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101]))) || (name == (self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 105, 110, 103])))) || (name == (self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114])))) {
                                                                                {
                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored385 = { let ml_method_owner461 = code.clone(); let ml_method_argument461_0 = self::JavaScriptCallInstruction::Argument(2f64, 0f64); let ml_method_argument461_1 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument461_2 = self::JavaScriptCallInstruction::Argument(3f64, 1f64); let ml_method_argument461_3 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Undefined)); let ml_method_argument461_4 = self::JavaScriptCallInstruction::StrictEqual(6f64, 3f64, 8f64); let ml_inserted = vec![ml_method_argument461_0, ml_method_argument461_1, ml_method_argument461_2, ml_method_argument461_3, ml_method_argument461_4]; let mut ml_method_values461 = ml_method_owner461.borrow_mut(); let ml_length = ml_method_values461.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values461.extend(ml_inserted); ml_length as f64 };
                                                                                    {
                                                                                        let missing_2 = { let ml_reference_array462 = code.clone(); let ml_reference_projection462 = ml_reference_array462.borrow().len() as f64; ml_reference_projection462 };
                                                                                        {
                                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored386 = { let ml_method_owner463 = code.clone(); let ml_method_argument463_0 = self::JavaScriptCallInstruction::Branch(6f64, 0f64, (missing_2 + 1f64)); let ml_method_argument463_1 = self::JavaScriptCallInstruction::Primitive(3f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![3f64]))); let ml_inserted = vec![ml_method_argument463_0, ml_method_argument463_1]; let mut ml_method_values463 = ml_method_owner463.borrow_mut(); let ml_length = ml_method_values463.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values463.extend(ml_inserted); ml_length as f64 };
                                                                                            {
                                                                                                let skip_2 = { let ml_reference_array464 = code.clone(); let ml_reference_projection464 = ml_reference_array464.borrow().len() as f64; ml_reference_projection464 };
                                                                                                {
                                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored387 = { let ml_method_owner465 = code.clone(); let ml_method_argument465_0 = self::JavaScriptCallInstruction::Jump(0f64); let ml_inserted = vec![ml_method_argument465_0]; let mut ml_method_values465 = ml_method_owner465.borrow_mut(); let ml_length = ml_method_values465.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values465.extend(ml_inserted); ml_length as f64 };
                                                                                                    {
                                                                                                        let length = { let ml_reference_array466 = code.clone(); let ml_reference_projection466 = ml_reference_array466.borrow().len() as f64; ml_reference_projection466 };
                                                                                                        {
                                                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored388 = { let ml_method_owner467 = code.clone(); let ml_method_argument467_0 = self::JavaScriptCallInstruction::Primitive(3f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_inserted = vec![ml_method_argument467_0]; let mut ml_method_values467 = ml_method_owner467.borrow_mut(); let ml_length = ml_method_values467.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values467.extend(ml_inserted); ml_length as f64 };
                                                                                                            {
                                                                                                                let apply_2 = { let ml_reference_array468 = code.clone(); let ml_reference_projection468 = ml_reference_array468.borrow().len() as f64; ml_reference_projection468 };
                                                                                                                {
                                                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored389 = { let ml_method_owner469 = code.clone(); let ml_method_argument469_0 = self::JavaScriptCallInstruction::Primitive(9f64, if (name == (self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101]))) {
                                                                                                                        self::ml_string::Text::from_units(vec![115, 108, 105, 99, 101, 32, 116, 101, 120, 116])
                                                                                                                    } else {
                                                                                                                        if (name == (self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 105, 110, 103]))) {
                                                                                                                            self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 105, 110, 103, 32, 83, 116, 114, 105, 110, 103, 32, 117, 110, 105, 116, 115])
                                                                                                                        } else {
                                                                                                                            self::ml_string::Text::from_units(vec![115, 117, 98, 115, 116, 114, 32, 83, 116, 114, 105, 110, 103, 32, 117, 110, 105, 116, 115])
                                                                                                                        }
                                                                                                                    }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64]))); let ml_method_argument469_1 = self::JavaScriptCallInstruction::Return(9f64); let ml_inserted = vec![ml_method_argument469_0, ml_method_argument469_1]; let mut ml_method_values469 = ml_method_owner469.borrow_mut(); let ml_length = ml_method_values469.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values469.extend(ml_inserted); ml_length as f64 };
                                                                                                                    {
                                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored390 = { let ml_array_owner470 = code.clone(); let ml_array_argument470_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, length, (missing_2 + 1f64)); let ml_array_argument470_1: f64 = missing_2; let ml_array_argument470_2: f64 = (missing_2 + 1f64); let ml_array_length = ml_array_owner470.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument470_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument470_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner470.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument470_0); } } ml_array_owner470 };
                                                                                                                        {
                                                                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored391 = { let ml_array_owner471 = code.clone(); let ml_array_argument471_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Jump(apply_2); let ml_array_argument471_1: f64 = skip_2; let ml_array_argument471_2: f64 = (skip_2 + 1f64); let ml_array_length = ml_array_owner471.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument471_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument471_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner471.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument471_0); } } ml_array_owner471 };
                                                                                                                            self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next
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
                                                                                if self::ml_array::includes(&{ let ml_reference_value472 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 104, 97, 114, 65, 116]), self::ml_string::Text::from_units(vec![99, 104, 97, 114, 67, 111, 100, 101, 65, 116]), self::ml_string::Text::from_units(vec![99, 111, 100, 101, 80, 111, 105, 110, 116, 65, 116]), self::ml_string::Text::from_units(vec![97, 116]), self::ml_string::Text::from_units(vec![114, 101, 112, 101, 97, 116])]))).borrow().clone(); ml_reference_value472 }, &name.clone(), 0f64, false, |a, b| a == b) {
                                                                                    {
                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored392 = { let ml_method_owner473 = code.clone(); let ml_method_argument473_0 = self::JavaScriptCallInstruction::Argument(2f64, 0f64); let ml_method_argument473_1 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument473_2 = self::JavaScriptCallInstruction::Primitive(9f64, if (name == (self::ml_string::Text::from_units(vec![99, 104, 97, 114, 65, 116]))) {
                                                                                            self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])
                                                                                        } else {
                                                                                            if (name == (self::ml_string::Text::from_units(vec![99, 104, 97, 114, 67, 111, 100, 101, 65, 116]))) {
                                                                                                self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 83, 116, 114, 105, 110, 103, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116])
                                                                                            } else {
                                                                                                if (name == (self::ml_string::Text::from_units(vec![99, 111, 100, 101, 80, 111, 105, 110, 116, 65, 116]))) {
                                                                                                    self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 99, 111, 100, 101, 32, 112, 111, 105, 110, 116])
                                                                                                } else {
                                                                                                    if (name == (self::ml_string::Text::from_units(vec![97, 116]))) {
                                                                                                        self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 114, 101, 108, 97, 116, 105, 118, 101, 32, 116, 101, 120, 116, 32, 99, 104, 97, 114, 97, 99, 116, 101, 114])
                                                                                                    } else {
                                                                                                        self::ml_string::Text::from_units(vec![114, 101, 112, 101, 97, 116, 32, 116, 101, 120, 116])
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64]))); let ml_method_argument473_3 = self::JavaScriptCallInstruction::Return(9f64); let ml_inserted = vec![ml_method_argument473_0, ml_method_argument473_1, ml_method_argument473_2, ml_method_argument473_3]; let mut ml_method_values473 = ml_method_owner473.borrow_mut(); let ml_length = ml_method_values473.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values473.extend(ml_inserted); ml_length as f64 };
                                                                                        self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next
                                                                                    }
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored393 = { let ml_method_owner474 = code.clone(); let ml_method_argument474_0 = self::JavaScriptCallInstruction::Argument(2f64, 0f64); let ml_inserted = vec![ml_method_argument474_0]; let mut ml_method_values474 = ml_method_owner474.borrow_mut(); let ml_length = ml_method_values474.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values474.extend(ml_inserted); ml_length as f64 };
                                                                                        {
                                                                                            let source_ml_source1_s1735 = if (((name == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116, 115, 87, 105, 116, 104]))) || (name == (self::ml_string::Text::from_units(vec![101, 110, 100, 115, 87, 105, 116, 104])))) || (name == (self::ml_string::Text::from_units(vec![105, 110, 99, 108, 117, 100, 101, 115])))) {
                                                                                                if {
                                                                                                    let source_ml_source1_comparison_operand1737 = {
                                                                                                        let source_ml_source1_optional_chain_receiver1736 = (match_key.clone(),);
                                                                                                        if match (source_ml_source1_optional_chain_receiver1736.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                                            Some({
                                                                                                                let source_ml_temporary_optional_chain355 = ((source_ml_source1_optional_chain_receiver1736.clone()).0).clone().expect("a proven present value");
                                                                                                                match source_ml_temporary_optional_chain355.clone() {
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
                                                                                                    !(match (source_ml_source1_comparison_operand1737.clone(), self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                                                                                } {
                                                                                                    self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394Return(None::<self::JavaScriptCallFunction>)
                                                                                                } else {
                                                                                                    {
                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored405 = { let ml_method_owner475 = code.clone(); let ml_method_argument475_0 = self::JavaScriptCallInstruction::Primitive(6f64, self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_inserted = vec![ml_method_argument475_0]; let mut ml_method_values475 = ml_method_owner475.borrow_mut(); let ml_length = ml_method_values475.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values475.extend(ml_inserted); ml_length as f64 };
                                                                                                        {
                                                                                                            let object = { let ml_reference_array476 = code.clone(); let ml_reference_projection476 = ml_reference_array476.borrow().len() as f64; ml_reference_projection476 };
                                                                                                            {
                                                                                                                let source_ml_temporary_compile_java_script_string_method_function_ignored406 = { let ml_method_owner477 = code.clone(); let ml_method_argument477_0 = self::JavaScriptCallInstruction::Branch(6f64, (object + 1f64), 0f64); let ml_method_argument477_1 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(match_key.clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))); let ml_method_argument477_2 = self::JavaScriptCallInstruction::OrdinaryGet(7f64, 2f64, 8f64, 2f64); let ml_method_argument477_3 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Undefined)); let ml_method_argument477_4 = self::JavaScriptCallInstruction::StrictEqual(6f64, 7f64, 8f64); let ml_inserted = vec![ml_method_argument477_0, ml_method_argument477_1, ml_method_argument477_2, ml_method_argument477_3, ml_method_argument477_4]; let mut ml_method_values477 = ml_method_owner477.borrow_mut(); let ml_length = ml_method_values477.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values477.extend(ml_inserted); ml_length as f64 };
                                                                                                                {
                                                                                                                    let missing_3 = { let ml_reference_array478 = code.clone(); let ml_reference_projection478 = ml_reference_array478.borrow().len() as f64; ml_reference_projection478 };
                                                                                                                    {
                                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored407 = { let ml_method_owner479 = code.clone(); let ml_method_argument479_0 = self::JavaScriptCallInstruction::Branch(6f64, 0f64, (missing_3 + 1f64)); let ml_method_argument479_1 = self::JavaScriptCallInstruction::Copy(6f64, 7f64); let ml_inserted = vec![ml_method_argument479_0, ml_method_argument479_1]; let mut ml_method_values479 = ml_method_owner479.borrow_mut(); let ml_length = ml_method_values479.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values479.extend(ml_inserted); ml_length as f64 };
                                                                                                                        {
                                                                                                                            let skip_3 = { let ml_reference_array480 = code.clone(); let ml_reference_projection480 = ml_reference_array480.borrow().len() as f64; ml_reference_projection480 };
                                                                                                                            {
                                                                                                                                let source_ml_temporary_compile_java_script_string_method_function_ignored408 = { let ml_method_owner481 = code.clone(); let ml_method_argument481_0 = self::JavaScriptCallInstruction::Jump(0f64); let ml_inserted = vec![ml_method_argument481_0]; let mut ml_method_values481 = ml_method_owner481.borrow_mut(); let ml_length = ml_method_values481.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values481.extend(ml_inserted); ml_length as f64 };
                                                                                                                                {
                                                                                                                                    let slot = { let ml_reference_array482 = code.clone(); let ml_reference_projection482 = ml_reference_array482.borrow().len() as f64; ml_reference_projection482 };
                                                                                                                                    {
                                                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored409 = { let ml_method_owner483 = code.clone(); let ml_method_argument483_0 = self::JavaScriptCallInstruction::ObjectObservation(6f64, self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 114, 101, 103, 117, 108, 97, 114, 69, 120, 112, 114, 101, 115, 115, 105, 111, 110, 83, 108, 111, 116]), 2f64, 2f64); let ml_inserted = vec![ml_method_argument483_0]; let mut ml_method_values483 = ml_method_owner483.borrow_mut(); let ml_length = ml_method_values483.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values483.extend(ml_inserted); ml_length as f64 };
                                                                                                                                        {
                                                                                                                                            let test = { let ml_reference_array484 = code.clone(); let ml_reference_projection484 = ml_reference_array484.borrow().len() as f64; ml_reference_projection484 };
                                                                                                                                            {
                                                                                                                                                let source_ml_temporary_compile_java_script_string_method_function_ignored410 = { let ml_method_owner485 = code.clone(); let ml_method_argument485_0 = self::JavaScriptCallInstruction::Branch(6f64, (test + 1f64), 0f64); let ml_inserted = vec![ml_method_argument485_0]; let mut ml_method_values485 = ml_method_owner485.borrow_mut(); let ml_length = ml_method_values485.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values485.extend(ml_inserted); ml_length as f64 };
                                                                                                                                                {
                                                                                                                                                    let message = ((self::ml_string::Text::from_units(vec![70, 105, 114, 115, 116, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 116, 111, 32, 83, 116, 114, 105, 110, 103, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46])).concat(&name)).concat(&self::ml_string::Text::from_units(vec![32, 109, 117, 115, 116, 32, 110, 111, 116, 32, 98, 101, 32, 97, 32, 114, 101, 103, 117, 108, 97, 114, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110]));
                                                                                                                                                    {
                                                                                                                                                        let message_units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                                                                                                                        {
                                                                                                                                                            let at_3 = 0f64;
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_s1739 = self::source_ml_temporary_compile_java_script_string_method_function_loop411(&(message), message_units.clone(), at_3);
                                                                                                                                                                match source_ml_source1_s1739.clone() {
                                                                                                                                                                    self::MlTemporaryCompileJavaScriptStringMethodFunctionLoop411Result::MlTemporaryCompileJavaScriptStringMethodFunctionLoop411Done => {
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored412 = { let ml_method_owner486 = code.clone(); let ml_method_argument486_0 = self::JavaScriptCallInstruction::Constant(7f64, Box::new(self::JavaScriptSemanticValue::Text(message_units.clone()))); let ml_method_argument486_1 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Boolean(true))); let ml_method_argument486_2 = self::JavaScriptCallInstruction::Primitive(9f64, self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![7f64, 8f64]))); let ml_inserted = vec![ml_method_argument486_0, ml_method_argument486_1, ml_method_argument486_2]; let mut ml_method_values486 = ml_method_owner486.borrow_mut(); let ml_length = ml_method_values486.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values486.extend(ml_inserted); ml_length as f64 };
                                                                                                                                                                            {
                                                                                                                                                                                let convert = { let ml_reference_array487 = code.clone(); let ml_reference_projection487 = ml_reference_array487.borrow().len() as f64; ml_reference_projection487 };
                                                                                                                                                                                {
                                                                                                                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored413 = { let ml_array_owner488 = code.clone(); let ml_array_argument488_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, (object + 1f64), convert); let ml_array_argument488_1: f64 = object; let ml_array_argument488_2: f64 = (object + 1f64); let ml_array_length = ml_array_owner488.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument488_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument488_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner488.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument488_0); } } ml_array_owner488 };
                                                                                                                                                                                    {
                                                                                                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored414 = { let ml_array_owner489 = code.clone(); let ml_array_argument489_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, slot, (missing_3 + 1f64)); let ml_array_argument489_1: f64 = missing_3; let ml_array_argument489_2: f64 = (missing_3 + 1f64); let ml_array_length = ml_array_owner489.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument489_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument489_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner489.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument489_0); } } ml_array_owner489 };
                                                                                                                                                                                        {
                                                                                                                                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored415 = { let ml_array_owner490 = code.clone(); let ml_array_argument490_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Jump(test); let ml_array_argument490_1: f64 = skip_3; let ml_array_argument490_2: f64 = (skip_3 + 1f64); let ml_array_length = ml_array_owner490.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument490_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument490_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner490.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument490_0); } } ml_array_owner490 };
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_temporary_compile_java_script_string_method_function_ignored416 = { let ml_array_owner491 = code.clone(); let ml_array_argument491_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, (test + 1f64), convert); let ml_array_argument491_1: f64 = test; let ml_array_argument491_2: f64 = (test + 1f64); let ml_array_length = ml_array_owner491.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument491_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument491_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner491.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument491_0); } } ml_array_owner491 };
                                                                                                                                                                                                self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394Next
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
                                                                                                self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394Next
                                                                                            };
                                                                                            match source_ml_source1_s1735.clone() {
                                                                                                self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394Next => {
                                                                                                    {
                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored395 = { let ml_method_owner492 = code.clone(); let ml_method_argument492_0 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument492_1 = self::JavaScriptCallInstruction::Argument(3f64, 1f64); let ml_inserted = vec![ml_method_argument492_0, ml_method_argument492_1]; let mut ml_method_values492 = ml_method_owner492.borrow_mut(); let ml_length = ml_method_values492.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values492.extend(ml_inserted); ml_length as f64 };
                                                                                                        {
                                                                                                            let source_ml_source1_s1740 = if (name == (self::ml_string::Text::from_units(vec![101, 110, 100, 115, 87, 105, 116, 104]))) {
                                                                                                                {
                                                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored398 = { let ml_method_owner493 = code.clone(); let ml_method_argument493_0 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Undefined)); let ml_method_argument493_1 = self::JavaScriptCallInstruction::StrictEqual(6f64, 3f64, 8f64); let ml_inserted = vec![ml_method_argument493_0, ml_method_argument493_1]; let mut ml_method_values493 = ml_method_owner493.borrow_mut(); let ml_length = ml_method_values493.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values493.extend(ml_inserted); ml_length as f64 };
                                                                                                                    {
                                                                                                                        let missing_4 = { let ml_reference_array494 = code.clone(); let ml_reference_projection494 = ml_reference_array494.borrow().len() as f64; ml_reference_projection494 };
                                                                                                                        {
                                                                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored399 = { let ml_method_owner495 = code.clone(); let ml_method_argument495_0 = self::JavaScriptCallInstruction::Branch(6f64, 0f64, (missing_4 + 1f64)); let ml_method_argument495_1 = self::JavaScriptCallInstruction::Primitive(3f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![3f64]))); let ml_inserted = vec![ml_method_argument495_0, ml_method_argument495_1]; let mut ml_method_values495 = ml_method_owner495.borrow_mut(); let ml_length = ml_method_values495.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values495.extend(ml_inserted); ml_length as f64 };
                                                                                                                            {
                                                                                                                                let skip_4 = { let ml_reference_array496 = code.clone(); let ml_reference_projection496 = ml_reference_array496.borrow().len() as f64; ml_reference_projection496 };
                                                                                                                                {
                                                                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored400 = { let ml_method_owner497 = code.clone(); let ml_method_argument497_0 = self::JavaScriptCallInstruction::Jump(0f64); let ml_inserted = vec![ml_method_argument497_0]; let mut ml_method_values497 = ml_method_owner497.borrow_mut(); let ml_length = ml_method_values497.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values497.extend(ml_inserted); ml_length as f64 };
                                                                                                                                    {
                                                                                                                                        let length_2 = { let ml_reference_array498 = code.clone(); let ml_reference_projection498 = ml_reference_array498.borrow().len() as f64; ml_reference_projection498 };
                                                                                                                                        {
                                                                                                                                            let source_ml_temporary_compile_java_script_string_method_function_ignored401 = { let ml_method_owner499 = code.clone(); let ml_method_argument499_0 = self::JavaScriptCallInstruction::Primitive(3f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 116, 101, 120, 116, 32, 108, 101, 110, 103, 116, 104]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_inserted = vec![ml_method_argument499_0]; let mut ml_method_values499 = ml_method_owner499.borrow_mut(); let ml_length = ml_method_values499.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values499.extend(ml_inserted); ml_length as f64 };
                                                                                                                                            {
                                                                                                                                                let apply_3 = { let ml_reference_array500 = code.clone(); let ml_reference_projection500 = ml_reference_array500.borrow().len() as f64; ml_reference_projection500 };
                                                                                                                                                {
                                                                                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored402 = { let ml_array_owner501 = code.clone(); let ml_array_argument501_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, length_2, (missing_4 + 1f64)); let ml_array_argument501_1: f64 = missing_4; let ml_array_argument501_2: f64 = (missing_4 + 1f64); let ml_array_length = ml_array_owner501.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument501_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument501_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner501.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument501_0); } } ml_array_owner501 };
                                                                                                                                                    {
                                                                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored403 = { let ml_array_owner502 = code.clone(); let ml_array_argument502_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Jump(apply_3); let ml_array_argument502_1: f64 = skip_4; let ml_array_argument502_2: f64 = (skip_4 + 1f64); let ml_array_length = ml_array_owner502.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument502_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument502_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner502.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument502_0); } } ml_array_owner502 };
                                                                                                                                                        self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin396::MlTemporaryCompileJavaScriptStringMethodFunctionJoin396Next
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
                                                                                                                {
                                                                                                                    let source_ml_temporary_compile_java_script_string_method_function_ignored404 = { let ml_method_owner503 = code.clone(); let ml_method_argument503_0 = self::JavaScriptCallInstruction::Primitive(3f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![3f64]))); let ml_inserted = vec![ml_method_argument503_0]; let mut ml_method_values503 = ml_method_owner503.borrow_mut(); let ml_length = ml_method_values503.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values503.extend(ml_inserted); ml_length as f64 };
                                                                                                                    self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin396::MlTemporaryCompileJavaScriptStringMethodFunctionJoin396Next
                                                                                                                }
                                                                                                            };
                                                                                                            match source_ml_source1_s1740.clone() {
                                                                                                                self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin396::MlTemporaryCompileJavaScriptStringMethodFunctionJoin396Next => {
                                                                                                                    {
                                                                                                                        let source_ml_temporary_compile_java_script_string_method_function_ignored397 = { let ml_method_owner504 = code.clone(); let ml_method_argument504_0 = self::JavaScriptCallInstruction::Primitive(9f64, if (name == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 79, 102]))) {
                                                                                                                            self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 116, 101, 120, 116])
                                                                                                                        } else {
                                                                                                                            if (name == (self::ml_string::Text::from_units(vec![108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102]))) {
                                                                                                                                self::ml_string::Text::from_units(vec![102, 105, 110, 100, 32, 108, 97, 115, 116, 32, 116, 101, 120, 116])
                                                                                                                            } else {
                                                                                                                                if (name == (self::ml_string::Text::from_units(vec![115, 116, 97, 114, 116, 115, 87, 105, 116, 104]))) {
                                                                                                                                    self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120, 32, 97, 116])
                                                                                                                                } else {
                                                                                                                                    if (name == (self::ml_string::Text::from_units(vec![101, 110, 100, 115, 87, 105, 116, 104]))) {
                                                                                                                                        self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 115, 117, 102, 102, 105, 120, 32, 97, 116])
                                                                                                                                    } else {
                                                                                                                                        self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 83, 116, 114, 105, 110, 103, 32, 105, 110, 99, 108, 117, 115, 105, 111, 110, 32, 97, 116])
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64, 2f64, 3f64]))); let ml_method_argument504_1 = self::JavaScriptCallInstruction::Return(9f64); let ml_inserted = vec![ml_method_argument504_0, ml_method_argument504_1]; let mut ml_method_values504 = ml_method_owner504.borrow_mut(); let ml_length = ml_method_values504.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values504.extend(ml_inserted); ml_length as f64 };
                                                                                                                        self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                                self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394::MlTemporaryCompileJavaScriptStringMethodFunctionJoin394Return(source_ml_temporary_result404) => {
                                                                                                    self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Return(source_ml_temporary_result404.clone())
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
                                            };
                                            match source_ml_source1_s1729.clone() {
                                                self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Next => {
                                                    Some({
                                                        let source_ml_source1_constructor_field1744 = self::JavaScriptSemanticValue::Undefined;
                                                        {
                                                            let source_ml_source1_constructor_field1745 = 0f64;
                                                            {
                                                                let source_ml_source1_constructor_field1746 = 10f64;
                                                                {
                                                                    let source_ml_source1_constructor_field1747 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                    self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1744.clone()), source_ml_source1_constructor_field1745, source_ml_source1_constructor_field1746, source_ml_source1_constructor_field1747.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                }
                                                            }
                                                        }
                                                    })
                                                }
                                                self::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357::MlTemporaryCompileJavaScriptStringMethodFunctionJoin357Return(source_ml_temporary_result357) => {
                                                    source_ml_temporary_result357.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_function_invocation_operation(operation: &self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value505 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 99, 97, 108, 108]), self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 97, 112, 112, 108, 121]), self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 98, 105, 110, 100]), self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 116, 111, 83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 114, 101, 115, 116, 114, 105, 99, 116, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121]), self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101]), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 97, 112, 112, 108, 121])]))).borrow().clone(); ml_reference_value505 }, operation, 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_function_invocation_function(identity: self::ml::Big, realm: self::ml::Big, operation: &self::ml_string::Text) -> Option<self::JavaScriptCallFunction> {
    if !(self::test_java_script_function_invocation_operation(operation)) {
        None::<self::JavaScriptCallFunction>
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101]))) {
            Some({
                let source_ml_source1_constructor_field1752 = self::JavaScriptSemanticValue::Undefined;
                {
                    let source_ml_source1_constructor_field1753 = 0f64;
                    {
                        let source_ml_source1_constructor_field1754 = 2f64;
                        {
                            let source_ml_source1_constructor_field1755 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                            {
                                let source_ml_source1_constructor_field1756 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Constant(1f64, Box::new(self::JavaScriptSemanticValue::Undefined)), self::JavaScriptCallInstruction::Return(1f64)]));
                                self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1752.clone()), source_ml_source1_constructor_field1753, source_ml_source1_constructor_field1754, source_ml_source1_constructor_field1755.clone(), source_ml_source1_constructor_field1756.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                            }
                        }
                    }
                }
            })
        } else {
            if ((*operation) == (self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 98, 105, 110, 100]))) {
                Some({
                    let source_ml_source1_constructor_field1760 = self::JavaScriptSemanticValue::Undefined;
                    {
                        let source_ml_source1_constructor_field1761 = 0f64;
                        {
                            let source_ml_source1_constructor_field1762 = 2f64;
                            {
                                let source_ml_source1_constructor_field1763 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                {
                                    let source_ml_source1_constructor_field1764 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::FunctionBind(1f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                    self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1760.clone()), source_ml_source1_constructor_field1761, source_ml_source1_constructor_field1762, source_ml_source1_constructor_field1763.clone(), source_ml_source1_constructor_field1764.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                }
                            }
                        }
                    }
                })
            } else {
                if ((*operation) == (self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 116, 111, 83, 116, 114, 105, 110, 103]))) {
                    Some({
                        let source_ml_source1_constructor_field1768 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source1_constructor_field1769 = 0f64;
                            {
                                let source_ml_source1_constructor_field1770 = 2f64;
                                {
                                    let source_ml_source1_constructor_field1771 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                    {
                                        let source_ml_source1_constructor_field1772 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::FunctionToString(1f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                        self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1768.clone()), source_ml_source1_constructor_field1769, source_ml_source1_constructor_field1770, source_ml_source1_constructor_field1771.clone(), source_ml_source1_constructor_field1772.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                    }
                                }
                            }
                        }
                    })
                } else {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 114, 101, 115, 116, 114, 105, 99, 116, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121]))) {
                        Some({
                            let source_ml_source1_constructor_field1776 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_constructor_field1777 = 0f64;
                                {
                                    let source_ml_source1_constructor_field1778 = 2f64;
                                    {
                                        let source_ml_source1_constructor_field1779 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                        {
                                            let source_ml_source1_constructor_field1780 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119, 32, 114, 101, 115, 116, 114, 105, 99, 116, 101, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 97, 99, 99, 101, 115, 115]), std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))), self::JavaScriptCallInstruction::Return(1f64)]));
                                            self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1776.clone()), source_ml_source1_constructor_field1777, source_ml_source1_constructor_field1778, source_ml_source1_constructor_field1779.clone(), source_ml_source1_constructor_field1780.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                        }
                                    }
                                }
                            }
                        })
                    } else {
                        {
                            let instructions = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
                            {
                                let source_ml_source1_s1781 = if ((*operation) == (self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 97, 112, 112, 108, 121]))) {
                                    {
                                        let source_ml_temporary_compile_java_script_function_invocation_function_ignored440 = { let ml_method_owner506 = instructions.clone(); let ml_method_argument506_0 = self::JavaScriptCallInstruction::Argument(1f64, 0f64); let ml_method_argument506_1 = self::JavaScriptCallInstruction::Argument(2f64, 1f64); let ml_method_argument506_2 = self::JavaScriptCallInstruction::Argument(3f64, 2f64); let ml_inserted = vec![ml_method_argument506_0, ml_method_argument506_1, ml_method_argument506_2]; let mut ml_method_values506 = ml_method_owner506.borrow_mut(); let ml_length = ml_method_values506.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values506.extend(ml_inserted); ml_length as f64 };
                                        self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419Next
                                    }
                                } else {
                                    {
                                        let source_ml_temporary_compile_java_script_function_invocation_function_ignored441 = { let ml_method_owner507 = instructions.clone(); let ml_method_argument507_0 = self::JavaScriptCallInstruction::Copy(1f64, 0f64); let ml_method_argument507_1 = self::JavaScriptCallInstruction::Argument(2f64, 0f64); let ml_inserted = vec![ml_method_argument507_0, ml_method_argument507_1]; let mut ml_method_values507 = ml_method_owner507.borrow_mut(); let ml_length = ml_method_values507.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values507.extend(ml_inserted); ml_length as f64 };
                                        if ((*operation) == (self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 97, 112, 112, 108, 121]))) {
                                            {
                                                let source_ml_temporary_compile_java_script_function_invocation_function_ignored442 = { let ml_method_owner508 = instructions.clone(); let ml_method_argument508_0 = self::JavaScriptCallInstruction::Argument(3f64, 1f64); let ml_inserted = vec![ml_method_argument508_0]; let mut ml_method_values508 = ml_method_owner508.borrow_mut(); let ml_length = ml_method_values508.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values508.extend(ml_inserted); ml_length as f64 };
                                                self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419Next
                                            }
                                        } else {
                                            self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419Next
                                        }
                                    }
                                };
                                match source_ml_source1_s1781.clone() {
                                    self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin419Next => {
                                        {
                                            let source_ml_temporary_compile_java_script_function_invocation_function_ignored420 = { let ml_method_owner509 = instructions.clone(); let ml_method_argument509_0 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_method_argument509_1 = self::JavaScriptCallInstruction::ArgumentListOpen(0f64); let ml_method_argument509_2 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))); let ml_inserted = vec![ml_method_argument509_0, ml_method_argument509_1, ml_method_argument509_2]; let mut ml_method_values509 = ml_method_owner509.borrow_mut(); let ml_length = ml_method_values509.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values509.extend(ml_inserted); ml_length as f64 };
                                            {
                                                let source_ml_source1_s1785 = if ((*operation) == (self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 99, 97, 108, 108]))) {
                                                    {
                                                        let source_ml_temporary_compile_java_script_function_invocation_function_ignored422 = { let ml_method_owner510 = instructions.clone(); let ml_method_argument510_0 = self::JavaScriptCallInstruction::ArgumentCount(5f64); let ml_method_argument510_1 = self::JavaScriptCallInstruction::Constant(4f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))); let ml_inserted = vec![ml_method_argument510_0, ml_method_argument510_1]; let mut ml_method_values510 = ml_method_owner510.borrow_mut(); let ml_length = ml_method_values510.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values510.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let loop_ = { let ml_reference_array511 = instructions.clone(); let ml_reference_projection511 = ml_reference_array511.borrow().len() as f64; ml_reference_projection511 };
                                                            {
                                                                let source_ml_temporary_compile_java_script_function_invocation_function_ignored423 = { let ml_method_owner512 = instructions.clone(); let ml_method_argument512_0 = self::JavaScriptCallInstruction::NumberBinary(6f64, 4f64, 5f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])); let ml_inserted = vec![ml_method_argument512_0]; let mut ml_method_values512 = ml_method_owner512.borrow_mut(); let ml_length = ml_method_values512.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values512.extend(ml_inserted); ml_length as f64 };
                                                                {
                                                                    let branch = { let ml_reference_array513 = instructions.clone(); let ml_reference_projection513 = ml_reference_array513.borrow().len() as f64; ml_reference_projection513 };
                                                                    {
                                                                        let source_ml_temporary_compile_java_script_function_invocation_function_ignored424 = { let ml_method_owner514 = instructions.clone(); let ml_method_argument514_0 = self::JavaScriptCallInstruction::Branch(6f64, (branch + 1f64), 0f64); let ml_method_argument514_1 = self::JavaScriptCallInstruction::ArgumentAt(7f64, 4f64); let ml_method_argument514_2 = self::JavaScriptCallInstruction::ArgumentListAppend(0f64, 7f64); let ml_method_argument514_3 = self::JavaScriptCallInstruction::NumberBinary(4f64, 4f64, 8f64, self::ml_string::Text::from_units(vec![97, 100, 100])); let ml_method_argument514_4 = self::JavaScriptCallInstruction::Jump(loop_); let ml_inserted = vec![ml_method_argument514_0, ml_method_argument514_1, ml_method_argument514_2, ml_method_argument514_3, ml_method_argument514_4]; let mut ml_method_values514 = ml_method_owner514.borrow_mut(); let ml_length = ml_method_values514.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values514.extend(ml_inserted); ml_length as f64 };
                                                                        {
                                                                            let call = { let ml_reference_array515 = instructions.clone(); let ml_reference_projection515 = ml_reference_array515.borrow().len() as f64; ml_reference_projection515 };
                                                                            {
                                                                                let source_ml_temporary_compile_java_script_function_invocation_function_ignored425 = { let ml_array_owner516 = instructions.clone(); let ml_array_argument516_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, (branch + 1f64), call); let ml_array_argument516_1: f64 = branch; let ml_array_argument516_2: f64 = (branch + 1f64); let ml_array_length = ml_array_owner516.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument516_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument516_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner516.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument516_0); } } ml_array_owner516 };
                                                                                {
                                                                                    let source_ml_temporary_compile_java_script_function_invocation_function_ignored426 = { let ml_method_owner517 = instructions.clone(); let ml_method_argument517_0 = self::JavaScriptCallInstruction::ArgumentListCall(9f64, 0f64, 1f64, 2f64, self::ml_string::Text::from_units(vec![99, 97, 108, 108])); let ml_method_argument517_1 = self::JavaScriptCallInstruction::Return(9f64); let ml_inserted = vec![ml_method_argument517_0, ml_method_argument517_1]; let mut ml_method_values517 = ml_method_owner517.borrow_mut(); let ml_length = ml_method_values517.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values517.extend(ml_inserted); ml_length as f64 };
                                                                                    self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin421::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin421Next
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let null_branch = (-1f64);
                                                        {
                                                            let undefined_branch = (-1f64);
                                                            {
                                                                let source_ml_source1_s1786 = if ((*operation) == (self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 97, 112, 112, 108, 121]))) {
                                                                    {
                                                                        let source_ml_temporary_compile_java_script_function_invocation_function_ignored437 = { let ml_method_owner518 = instructions.clone(); let ml_method_argument518_0 = self::JavaScriptCallInstruction::Constant(6f64, Box::new(self::JavaScriptSemanticValue::Null)); let ml_method_argument518_1 = self::JavaScriptCallInstruction::StrictEqual(6f64, 3f64, 6f64); let ml_inserted = vec![ml_method_argument518_0, ml_method_argument518_1]; let mut ml_method_values518 = ml_method_owner518.borrow_mut(); let ml_length = ml_method_values518.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values518.extend(ml_inserted); ml_length as f64 };
                                                                        {
                                                                            let null_branch_2 = { let ml_reference_array519 = instructions.clone(); let ml_reference_projection519 = ml_reference_array519.borrow().len() as f64; ml_reference_projection519 };
                                                                            {
                                                                                let source_ml_temporary_compile_java_script_function_invocation_function_ignored438 = { let ml_method_owner520 = instructions.clone(); let ml_method_argument520_0 = self::JavaScriptCallInstruction::Branch(6f64, 0f64, (null_branch_2 + 1f64)); let ml_method_argument520_1 = self::JavaScriptCallInstruction::Constant(6f64, Box::new(self::JavaScriptSemanticValue::Undefined)); let ml_method_argument520_2 = self::JavaScriptCallInstruction::StrictEqual(6f64, 3f64, 6f64); let ml_inserted = vec![ml_method_argument520_0, ml_method_argument520_1, ml_method_argument520_2]; let mut ml_method_values520 = ml_method_owner520.borrow_mut(); let ml_length = ml_method_values520.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values520.extend(ml_inserted); ml_length as f64 };
                                                                                {
                                                                                    let undefined_branch_2 = { let ml_reference_array521 = instructions.clone(); let ml_reference_projection521 = ml_reference_array521.borrow().len() as f64; ml_reference_projection521 };
                                                                                    {
                                                                                        let source_ml_temporary_compile_java_script_function_invocation_function_ignored439 = { let ml_method_owner522 = instructions.clone(); let ml_method_argument522_0 = self::JavaScriptCallInstruction::Branch(6f64, 0f64, (undefined_branch_2 + 1f64)); let ml_inserted = vec![ml_method_argument522_0]; let mut ml_method_values522 = ml_method_owner522.borrow_mut(); let ml_length = ml_method_values522.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values522.extend(ml_inserted); ml_length as f64 };
                                                                                        self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin427::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin427Next(null_branch_2, undefined_branch_2)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin427::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin427Next(null_branch, undefined_branch)
                                                                };
                                                                match source_ml_source1_s1786.clone() {
                                                                    self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin427::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin427Next(null_branch_3, undefined_branch_3) => {
                                                                        {
                                                                            let source_ml_temporary_compile_java_script_function_invocation_function_ignored428 = { let ml_method_owner523 = instructions.clone(); let ml_method_argument523_0 = self::JavaScriptCallInstruction::Primitive(3f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 79, 98, 106, 101, 99, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![3f64]))); let ml_method_argument523_1 = self::JavaScriptCallInstruction::Constant(6f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64]))))); let ml_method_argument523_2 = self::JavaScriptCallInstruction::OrdinaryGet(5f64, 3f64, 6f64, 3f64); let ml_method_argument523_3 = self::JavaScriptCallInstruction::Primitive(5f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![5f64]))); let ml_method_argument523_4 = self::JavaScriptCallInstruction::Primitive(5f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 108, 101, 110, 103, 116, 104]), std::rc::Rc::new(std::cell::RefCell::new(vec![5f64]))); let ml_method_argument523_5 = self::JavaScriptCallInstruction::Constant(4f64, Box::new(self::JavaScriptSemanticValue::Number(0f64))); let ml_inserted = vec![ml_method_argument523_0, ml_method_argument523_1, ml_method_argument523_2, ml_method_argument523_3, ml_method_argument523_4, ml_method_argument523_5]; let mut ml_method_values523 = ml_method_owner523.borrow_mut(); let ml_length = ml_method_values523.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values523.extend(ml_inserted); ml_length as f64 };
                                                                            {
                                                                                let loop_2 = { let ml_reference_array524 = instructions.clone(); let ml_reference_projection524 = ml_reference_array524.borrow().len() as f64; ml_reference_projection524 };
                                                                                {
                                                                                    let source_ml_temporary_compile_java_script_function_invocation_function_ignored429 = { let ml_method_owner525 = instructions.clone(); let ml_method_argument525_0 = self::JavaScriptCallInstruction::NumberBinary(6f64, 4f64, 5f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])); let ml_inserted = vec![ml_method_argument525_0]; let mut ml_method_values525 = ml_method_owner525.borrow_mut(); let ml_length = ml_method_values525.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values525.extend(ml_inserted); ml_length as f64 };
                                                                                    {
                                                                                        let branch_2 = { let ml_reference_array526 = instructions.clone(); let ml_reference_projection526 = ml_reference_array526.borrow().len() as f64; ml_reference_projection526 };
                                                                                        {
                                                                                            let source_ml_temporary_compile_java_script_function_invocation_function_ignored430 = { let ml_method_owner527 = instructions.clone(); let ml_method_argument527_0 = self::JavaScriptCallInstruction::Branch(6f64, (branch_2 + 1f64), 0f64); let ml_method_argument527_1 = self::JavaScriptCallInstruction::PropertyKey(6f64, 4f64, realm.clone()); let ml_method_argument527_2 = self::JavaScriptCallInstruction::OrdinaryGet(7f64, 3f64, 6f64, 3f64); let ml_method_argument527_3 = self::JavaScriptCallInstruction::ArgumentListAppend(0f64, 7f64); let ml_method_argument527_4 = self::JavaScriptCallInstruction::NumberBinary(4f64, 4f64, 8f64, self::ml_string::Text::from_units(vec![97, 100, 100])); let ml_method_argument527_5 = self::JavaScriptCallInstruction::Jump(loop_2); let ml_inserted = vec![ml_method_argument527_0, ml_method_argument527_1, ml_method_argument527_2, ml_method_argument527_3, ml_method_argument527_4, ml_method_argument527_5]; let mut ml_method_values527 = ml_method_owner527.borrow_mut(); let ml_length = ml_method_values527.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values527.extend(ml_inserted); ml_length as f64 };
                                                                                            {
                                                                                                let call_2 = { let ml_reference_array528 = instructions.clone(); let ml_reference_projection528 = ml_reference_array528.borrow().len() as f64; ml_reference_projection528 };
                                                                                                {
                                                                                                    let source_ml_temporary_compile_java_script_function_invocation_function_ignored431 = { let ml_array_owner529 = instructions.clone(); let ml_array_argument529_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, (branch_2 + 1f64), call_2); let ml_array_argument529_1: f64 = branch_2; let ml_array_argument529_2: f64 = (branch_2 + 1f64); let ml_array_length = ml_array_owner529.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument529_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument529_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner529.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument529_0); } } ml_array_owner529 };
                                                                                                    {
                                                                                                        let source_ml_source1_s1787 = if (null_branch_3 >= (0f64)) {
                                                                                                            {
                                                                                                                let source_ml_temporary_compile_java_script_function_invocation_function_ignored436 = { let ml_array_owner530 = instructions.clone(); let ml_array_argument530_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, call_2, (null_branch_3 + 1f64)); let ml_array_argument530_1: f64 = null_branch_3; let ml_array_argument530_2: f64 = (null_branch_3 + 1f64); let ml_array_length = ml_array_owner530.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument530_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument530_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner530.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument530_0); } } ml_array_owner530 };
                                                                                                                self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin432::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin432Next
                                                                                                            }
                                                                                                        } else {
                                                                                                            self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin432::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin432Next
                                                                                                        };
                                                                                                        match source_ml_source1_s1787.clone() {
                                                                                                            self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin432::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin432Next => {
                                                                                                                {
                                                                                                                    let source_ml_source1_s1788 = if (undefined_branch_3 >= (0f64)) {
                                                                                                                        {
                                                                                                                            let source_ml_temporary_compile_java_script_function_invocation_function_ignored435 = { let ml_array_owner531 = instructions.clone(); let ml_array_argument531_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, call_2, (undefined_branch_3 + 1f64)); let ml_array_argument531_1: f64 = undefined_branch_3; let ml_array_argument531_2: f64 = (undefined_branch_3 + 1f64); let ml_array_length = ml_array_owner531.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument531_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument531_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner531.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument531_0); } } ml_array_owner531 };
                                                                                                                            self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin433::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin433Next
                                                                                                                        }
                                                                                                                    } else {
                                                                                                                        self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin433::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin433Next
                                                                                                                    };
                                                                                                                    match source_ml_source1_s1788.clone() {
                                                                                                                        self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin433::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin433Next => {
                                                                                                                            {
                                                                                                                                let source_ml_temporary_compile_java_script_function_invocation_function_ignored434 = { let ml_method_owner532 = instructions.clone(); let ml_method_argument532_0 = self::JavaScriptCallInstruction::ArgumentListCall(9f64, 0f64, 1f64, 2f64, self::ml_string::Text::from_units(vec![99, 97, 108, 108])); let ml_method_argument532_1 = self::JavaScriptCallInstruction::Return(9f64); let ml_inserted = vec![ml_method_argument532_0, ml_method_argument532_1]; let mut ml_method_values532 = ml_method_owner532.borrow_mut(); let ml_length = ml_method_values532.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values532.extend(ml_inserted); ml_length as f64 };
                                                                                                                                self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin421::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin421Next
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
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                match source_ml_source1_s1785.clone() {
                                                    self::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin421::MlTemporaryCompileJavaScriptFunctionInvocationFunctionJoin421Next => {
                                                        Some({
                                                            let source_ml_source1_constructor_field1792 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_constructor_field1793 = 0f64;
                                                                {
                                                                    let source_ml_source1_constructor_field1794 = 10f64;
                                                                    {
                                                                        let source_ml_source1_constructor_field1795 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1792.clone()), source_ml_source1_constructor_field1793, source_ml_source1_constructor_field1794, source_ml_source1_constructor_field1795.clone(), instructions.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                    }
                                                                }
                                                            }
                                                        })
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn normalize_java_script_collection_iterator_operation(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*operation) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 105, 116, 101, 114, 97, 116, 111, 114]))) {
        self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 118, 97, 108, 117, 101, 115])
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![83, 101, 116, 46, 107, 101, 121, 115]))) {
            self::ml_string::Text::from_units(vec![83, 101, 116, 46, 118, 97, 108, 117, 101, 115])
        } else {
            operation.clone()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_operation_kind(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if self::ml_array::includes(&{ let ml_reference_value533 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 105, 116, 101, 114, 97, 116, 111, 114]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 73, 116, 101, 114, 97, 116, 111, 114, 46, 110, 101, 120, 116])]))).borrow().clone(); ml_reference_value533 }, operation, 0f64, false, |a, b| a == b) {
        self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])
    } else {
        if self::ml_array::includes(&{ let ml_reference_value534 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 107, 101, 121, 115]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 101, 110, 116, 114, 105, 101, 115]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 73, 116, 101, 114, 97, 116, 111, 114, 46, 110, 101, 120, 116])]))).borrow().clone(); ml_reference_value534 }, operation, 0f64, false, |a, b| a == b) {
            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121])
        } else {
            if self::ml_array::includes(&{ let ml_reference_value535 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112, 46, 107, 101, 121, 115]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 101, 110, 116, 114, 105, 101, 115]), self::ml_string::Text::from_units(vec![77, 97, 112, 73, 116, 101, 114, 97, 116, 111, 114, 46, 110, 101, 120, 116])]))).borrow().clone(); ml_reference_value535 }, operation, 0f64, false, |a, b| a == b) {
                self::ml_string::Text::from_units(vec![77, 97, 112])
            } else {
                if self::ml_array::includes(&{ let ml_reference_value536 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![83, 101, 116, 46, 107, 101, 121, 115]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 101, 110, 116, 114, 105, 101, 115]), self::ml_string::Text::from_units(vec![83, 101, 116, 73, 116, 101, 114, 97, 116, 111, 114, 46, 110, 101, 120, 116])]))).borrow().clone(); ml_reference_value536 }, operation, 0f64, false, |a, b| a == b) {
                    self::ml_string::Text::from_units(vec![83, 101, 116])
                } else {
                    if ((*operation) == (self::ml_string::Text::from_units(vec![73, 116, 101, 114, 97, 116, 111, 114, 46, 115, 101, 108, 102]))) {
                        self::ml_string::Text::from_units(vec![115, 101, 108, 102])
                    } else {
                        self::ml_string::Text::from_units(vec![])
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_operation_strategy(operation: &self::ml_string::Text) -> self::ml_string::Text {
    {
        let actual = self::normalize_java_script_collection_iterator_operation(operation);
        if actual.ends_with(&self::ml_string::Text::from_units(vec![46, 107, 101, 121, 115])) {
            self::ml_string::Text::from_units(vec![107, 101, 121, 115])
        } else {
            if actual.ends_with(&self::ml_string::Text::from_units(vec![46, 118, 97, 108, 117, 101, 115])) {
                self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])
            } else {
                if actual.ends_with(&self::ml_string::Text::from_units(vec![46, 101, 110, 116, 114, 105, 101, 115])) {
                    self::ml_string::Text::from_units(vec![101, 110, 116, 114, 105, 101, 115])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}
