// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCollectionMethodCompilation;
use super::JavaScriptPrimitiveMethodOwner;
use super::JavaScriptPrimitiveWrapperRoots;
use super::JavaScriptSemanticValue;
use super::MlTemporaryCompileJavaScriptCollectionObjectMethodJoin507;
use super::append_java_script_compiled_collection_call_function;
use super::compile_java_script_array_mutation_function;
use super::compile_java_script_boxed_array_mutation_function;
use super::compile_java_script_boxed_array_read_function;
use super::compile_java_script_object_definition_function;
use super::compile_java_script_standard_object_observation_function;
use super::ml;
use super::ml_string;
use super::require_java_script_array_species_directory;
use super::test_java_script_array_mutation_operation;
use super::test_java_script_array_search_operation;
use super::test_java_script_object_assignment_operation;
use super::test_java_script_object_definition_operation;
use super::test_java_script_standard_array_callback_operation;
use super::test_java_script_standard_array_species_getter_operation;
use super::test_java_script_standard_array_transform_operation;
use super::test_java_script_standard_object_observation_operation;
use super::test_java_script_standard_property_definitions_operation;
use super::test_java_script_standard_property_reflection_operation;
use super::test_java_script_standard_single_descriptor_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_collection_object_method(method: self::JavaScriptPrimitiveMethodOwner, identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>) -> self::JavaScriptCollectionMethodCompilation {
    if self::test_java_script_object_assignment_operation(&(match method.clone() {
        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2295, _, _, _, _, _, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value2295.clone()
        }
    })) {
        {
            let instructions = if ((match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2297, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2297.clone()
                }
            }) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110]))) {
                std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 79, 98, 106, 101, 99, 116, 32, 97, 115, 115, 105, 103, 110, 32, 116, 97, 114, 103, 101, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))), self::JavaScriptCallInstruction::ArgumentCount(2f64), self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))), self::JavaScriptCallInstruction::Constant(4f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))), self::JavaScriptCallInstruction::NumberBinary(5f64, 3f64, 2f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])), self::JavaScriptCallInstruction::Branch(5f64, 7f64, 11f64), self::JavaScriptCallInstruction::ArgumentAt(6f64, 3f64), {
                    let source_ml_source1_constructor_field2298 = 7f64;
                    {
                        let source_ml_source1_constructor_field2299 = 1f64;
                        {
                            let source_ml_source1_constructor_field2300 = 6f64;
                            {
                                let source_ml_source1_constructor_field2302 = match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2301, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2301.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_constructor_field2303 = Some(self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116]));
                                    self::JavaScriptCallInstruction::ObjectSpread(source_ml_source1_constructor_field2298, source_ml_source1_constructor_field2299, source_ml_source1_constructor_field2300, source_ml_source1_constructor_field2302.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Some((source_ml_source1_constructor_field2303.clone(),)))
                                }
                            }
                        }
                    }
                }, self::JavaScriptCallInstruction::NumberBinary(3f64, 3f64, 4f64, self::ml_string::Text::from_units(vec![97, 100, 100])), self::JavaScriptCallInstruction::Jump(5f64), self::JavaScriptCallInstruction::Return(1f64)]))
            } else {
                if ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2305, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2305.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]))) {
                    std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::ObjectObservation(1f64, self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102]), 0f64, 0f64), self::JavaScriptCallInstruction::Return(1f64)]))
                } else {
                    std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 110, 117, 108, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 110, 112, 117, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64]))), self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64]))), self::JavaScriptCallInstruction::Branch(2f64, 4f64, 10f64), self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))), self::JavaScriptCallInstruction::Branch(2f64, 9f64, 6f64), self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Null)), self::JavaScriptCallInstruction::StrictEqual(2f64, 1f64, 2f64), self::JavaScriptCallInstruction::Branch(2f64, 9f64, 10f64), self::JavaScriptCallInstruction::ObjectObservation(2f64, self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 115, 101, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102]), 0f64, 1f64), self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Undefined)), self::JavaScriptCallInstruction::Return(3f64)]))
                }
            };
            {
                let source_ml_temporary_compile_java_script_collection_object_method_ignored503 = { let ml_method_owner583 = identities.clone(); let ml_method_argument583_0 = match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2306, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2306.clone()
                    }
                }; let ml_inserted = vec![ml_method_argument583_0]; let mut ml_method_values583 = ml_method_owner583.borrow_mut(); let ml_length = ml_method_values583.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values583.extend(ml_inserted); ml_length as f64 };
                {
                    let source_ml_temporary_compile_java_script_collection_object_method_ignored504 = { let ml_method_owner584 = functions.clone(); let ml_method_argument584_0 = {
                        let source_ml_source1_constructor_field2308 = match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2307, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2307.clone()
                            }
                        };
                        {
                            let source_ml_source1_constructor_field2310 = match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2309, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2309.clone()
                                }
                            };
                            {
                                let source_ml_source1_constructor_field2312 = self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source1_constructor_field2313 = 0f64;
                                    {
                                        let source_ml_source1_constructor_field2318 = if ((match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2315, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2315.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110]))) {
                                            8f64
                                        } else {
                                            if ((match method.clone() {
                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2317, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2317.clone()
                                                }
                                            }) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114]))) {
                                                2f64
                                            } else {
                                                4f64
                                            }
                                        };
                                        {
                                            let source_ml_source1_constructor_field2319 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field2308.clone(), source_ml_source1_constructor_field2310.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field2312.clone()), source_ml_source1_constructor_field2313, source_ml_source1_constructor_field2318, source_ml_source1_constructor_field2319.clone(), instructions.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                        }
                                    }
                                }
                            }
                        }
                    }; let ml_inserted = vec![ml_method_argument584_0]; let mut ml_method_values584 = ml_method_owner584.borrow_mut(); let ml_length = ml_method_values584.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values584.extend(ml_inserted); ml_length as f64 };
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                }
            }
        }
    } else {
        if self::test_java_script_standard_property_definitions_operation(&(match method.clone() {
            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2321, _, _, _, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value2321.clone()
            }
        })) {
            {
                let source_ml_temporary_compile_java_script_collection_object_method_ignored505 = { let ml_method_owner585 = identities.clone(); let ml_method_argument585_0 = match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2322, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2322.clone()
                    }
                }; let ml_inserted = vec![ml_method_argument585_0]; let mut ml_method_values585 = ml_method_owner585.borrow_mut(); let ml_length = ml_method_values585.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values585.extend(ml_inserted); ml_length as f64 };
                {
                    let source_ml_temporary_compile_java_script_collection_object_method_ignored506 = { let ml_method_owner586 = functions.clone(); let ml_method_argument586_0 = {
                        let source_ml_source1_constructor_field2324 = match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2323, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2323.clone()
                            }
                        };
                        {
                            let source_ml_source1_constructor_field2326 = match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2325, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2325.clone()
                                }
                            };
                            {
                                let source_ml_source1_constructor_field2328 = self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source1_constructor_field2329 = 0f64;
                                    {
                                        let source_ml_source1_constructor_field2330 = 4f64;
                                        {
                                            let source_ml_source1_constructor_field2331 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                            {
                                                let source_ml_source1_constructor_field2333 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::Argument(2f64, 1f64), self::JavaScriptCallInstruction::PropertyDefinitions(3f64, match method.clone() {
                                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2332, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value2332.clone()
                                                    }
                                                }, 1f64, 2f64), self::JavaScriptCallInstruction::Return(3f64)]));
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field2324.clone(), source_ml_source1_constructor_field2326.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field2328.clone()), source_ml_source1_constructor_field2329, source_ml_source1_constructor_field2330, source_ml_source1_constructor_field2331.clone(), source_ml_source1_constructor_field2333.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }; let ml_inserted = vec![ml_method_argument586_0]; let mut ml_method_values586 = ml_method_owner586.borrow_mut(); let ml_length = ml_method_values586.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values586.extend(ml_inserted); ml_length as f64 };
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                }
            }
        } else {
            if self::test_java_script_standard_property_reflection_operation(&(match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2334, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2334.clone()
                }
            })) {
                if ({
                    let source_ml_source1_comparison_operand2337 = {
                        let source_ml_source1_optional_chain_receiver2336 = (match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value2335, _, _, _, _, _, _, _, _, _) => {
                                ((source_ml_source1_data_property_value2335.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        },);
                        if match (source_ml_source1_optional_chain_receiver2336.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            Some({
                                let source_ml_temporary_optional_chain501 = ((source_ml_source1_optional_chain_receiver2336.clone()).0).clone().expect("a proven present value");
                                match source_ml_temporary_optional_chain501.clone() {
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
                    !(match (source_ml_source1_comparison_operand2337.clone(), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                } || {
                    let source_ml_source1_comparison_operand2341 = {
                        let source_ml_source1_optional_chain_receiver2340 = (match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, source_ml_source1_data_property_value2339, _, _, _, _, _, _, _, _, _, _) => {
                                ((source_ml_source1_data_property_value2339.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        },);
                        if match (source_ml_source1_optional_chain_receiver2340.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            Some({
                                let source_ml_temporary_optional_chain502 = ((source_ml_source1_optional_chain_receiver2340.clone()).0).clone().expect("a proven present value");
                                match source_ml_temporary_optional_chain502.clone() {
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
                    !(match (source_ml_source1_comparison_operand2341.clone(), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                }) {
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 114, 101, 97, 108, 109, 32, 79, 98, 106, 101, 99, 116, 32, 97, 110, 100, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115]))
                } else {
                    {
                        let instructions_2 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::Primitive(1f64, if (match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2343, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2343.clone()
                            }
                        }).starts_with(&self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46])) {
                            self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116])
                        } else {
                            self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 110, 117, 108, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 110, 112, 117, 116])
                        }, std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))), self::JavaScriptCallInstruction::Argument(2f64, 1f64)]));
                        {
                            let source_ml_source1_s2346 = if self::test_java_script_standard_single_descriptor_operation(&(match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2347, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2347.clone()
                                }
                            })) {
                                {
                                    let source_ml_temporary_compile_java_script_collection_object_method_ignored511 = { let ml_method_owner587 = instructions_2.clone(); let ml_method_argument587_0 = self::JavaScriptCallInstruction::PropertyKey(2f64, 2f64, match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2348, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2348.clone()
                                        }
                                    }); let ml_inserted = vec![ml_method_argument587_0]; let mut ml_method_values587 = ml_method_owner587.borrow_mut(); let ml_length = ml_method_values587.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values587.extend(ml_inserted); ml_length as f64 };
                                    self::MlTemporaryCompileJavaScriptCollectionObjectMethodJoin507::MlTemporaryCompileJavaScriptCollectionObjectMethodJoin507Next
                                }
                            } else {
                                self::MlTemporaryCompileJavaScriptCollectionObjectMethodJoin507::MlTemporaryCompileJavaScriptCollectionObjectMethodJoin507Next
                            };
                            match source_ml_source1_s2346.clone() {
                                self::MlTemporaryCompileJavaScriptCollectionObjectMethodJoin507::MlTemporaryCompileJavaScriptCollectionObjectMethodJoin507Next => {
                                    {
                                        let source_ml_temporary_compile_java_script_collection_object_method_ignored508 = { let ml_method_owner588 = instructions_2.clone(); let ml_method_argument588_0 = self::JavaScriptCallInstruction::Constant(3f64, Box::new((match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value2349, _, _, _, _, _, _, _, _, _) => {
                                                ((source_ml_source1_data_property_value2349.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))); let ml_method_argument588_1 = self::JavaScriptCallInstruction::Constant(4f64, Box::new((match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, source_ml_source1_data_property_value2350, _, _, _, _, _, _, _, _, _, _) => {
                                                ((source_ml_source1_data_property_value2350.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))); let ml_method_argument588_2 = self::JavaScriptCallInstruction::PropertyReflection(5f64, match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2351, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2351.clone()
                                            }
                                        }, 1f64, 2f64, 3f64, 4f64); let ml_method_argument588_3 = self::JavaScriptCallInstruction::Return(5f64); let ml_inserted = vec![ml_method_argument588_0, ml_method_argument588_1, ml_method_argument588_2, ml_method_argument588_3]; let mut ml_method_values588 = ml_method_owner588.borrow_mut(); let ml_length = ml_method_values588.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values588.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let source_ml_temporary_compile_java_script_collection_object_method_ignored509 = { let ml_method_owner589 = identities.clone(); let ml_method_argument589_0 = match method.clone() {
                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2352, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2352.clone()
                                                }
                                            }; let ml_inserted = vec![ml_method_argument589_0]; let mut ml_method_values589 = ml_method_owner589.borrow_mut(); let ml_length = ml_method_values589.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values589.extend(ml_inserted); ml_length as f64 };
                                            {
                                                let source_ml_temporary_compile_java_script_collection_object_method_ignored510 = { let ml_method_owner590 = functions.clone(); let ml_method_argument590_0 = {
                                                    let source_ml_source1_constructor_field2354 = match method.clone() {
                                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2353, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value2353.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_constructor_field2356 = match method.clone() {
                                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2355, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value2355.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_constructor_field2358 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_constructor_field2359 = 0f64;
                                                                {
                                                                    let source_ml_source1_constructor_field2360 = 6f64;
                                                                    {
                                                                        let source_ml_source1_constructor_field2361 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field2354.clone(), source_ml_source1_constructor_field2356.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field2358.clone()), source_ml_source1_constructor_field2359, source_ml_source1_constructor_field2360, source_ml_source1_constructor_field2361.clone(), instructions_2.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }; let ml_inserted = vec![ml_method_argument590_0]; let mut ml_method_values590 = ml_method_owner590.borrow_mut(); let ml_length = ml_method_values590.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values590.extend(ml_inserted); ml_length as f64 };
                                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(false, self::ml_string::Text::from_units(vec![]))
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_collection_dispatch_method(method: self::JavaScriptPrimitiveMethodOwner, identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>) -> self::JavaScriptCollectionMethodCompilation {
    if (((match method.clone() {
        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2364, _, _, _, _, _, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value2364.clone()
        }
    }) == (self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 104, 97, 115, 73, 110, 115, 116, 97, 110, 99, 101]))) || ((match method.clone() {
        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2366, _, _, _, _, _, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value2366.clone()
        }
    }) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 105, 115, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102])))) {
        if {
            let source_ml_source1_comparison_operand2369 = {
                let source_ml_source1_optional_chain_receiver2368 = (match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, source_ml_source1_data_property_value2367, _, _, _, _, _, _, _, _) => {
                        ((source_ml_source1_data_property_value2367.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                    }
                },);
                if match (source_ml_source1_optional_chain_receiver2368.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                    Some({
                        let source_ml_temporary_optional_chain512 = ((source_ml_source1_optional_chain_receiver2368.clone()).0).clone().expect("a proven present value");
                        match source_ml_temporary_optional_chain512.clone() {
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
            !(match (source_ml_source1_comparison_operand2369.clone(), self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        } {
            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 32, 104, 97, 115, 73, 110, 115, 116, 97, 110, 99, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108]))
        } else {
            {
                let source_ml_temporary_compile_java_script_collection_dispatch_method_ignored513 = { let ml_method_owner591 = identities.clone(); let ml_method_argument591_0 = match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2371, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2371.clone()
                    }
                }; let ml_inserted = vec![ml_method_argument591_0]; let mut ml_method_values591 = ml_method_owner591.borrow_mut(); let ml_length = ml_method_values591.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values591.extend(ml_inserted); ml_length as f64 };
                {
                    let source_ml_temporary_compile_java_script_collection_dispatch_method_ignored514 = { let ml_method_owner592 = functions.clone(); let ml_method_argument592_0 = {
                        let source_ml_source1_constructor_field2373 = match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2372, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2372.clone()
                            }
                        };
                        {
                            let source_ml_source1_constructor_field2375 = match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2374, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2374.clone()
                                }
                            };
                            {
                                let source_ml_source1_constructor_field2377 = self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source1_constructor_field2378 = 0f64;
                                    {
                                        let source_ml_source1_constructor_field2379 = 4f64;
                                        {
                                            let source_ml_source1_constructor_field2380 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                            {
                                                let source_ml_source1_constructor_field2384 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::Constant(2f64, Box::new((match method.clone() {
                                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, source_ml_source1_data_property_value2381, _, _, _, _, _, _, _, _) => {
                                                        ((source_ml_source1_data_property_value2381.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))), self::JavaScriptCallInstruction::InstanceOf(3f64, 1f64, 0f64, 2f64, if ((match method.clone() {
                                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2383, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value2383.clone()
                                                    }
                                                }) == (self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 104, 97, 115, 73, 110, 115, 116, 97, 110, 99, 101]))) {
                                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 104, 97, 115, 32, 105, 110, 115, 116, 97, 110, 99, 101])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 111, 110, 116, 97, 105, 110, 115])
                                                }), self::JavaScriptCallInstruction::Return(3f64)]));
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field2373.clone(), source_ml_source1_constructor_field2375.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field2377.clone()), source_ml_source1_constructor_field2378, source_ml_source1_constructor_field2379, source_ml_source1_constructor_field2380.clone(), source_ml_source1_constructor_field2384.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }; let ml_inserted = vec![ml_method_argument592_0]; let mut ml_method_values592 = ml_method_owner592.borrow_mut(); let ml_length = ml_method_values592.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values592.extend(ml_inserted); ml_length as f64 };
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                }
            }
        }
    } else {
        if self::test_java_script_object_definition_operation(&(match method.clone() {
            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2385, _, _, _, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value2385.clone()
            }
        })) {
            {
                let fn_ = self::compile_java_script_object_definition_function(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2386, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2386.clone()
                    }
                }, match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2387, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2387.clone()
                    }
                }, Some(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2388, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2388.clone()
                    }
                }));
                if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_.clone())) {
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 114, 32, 82, 101, 102, 108, 101, 99, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                } else {
                    {
                        let source_ml_temporary_compile_java_script_collection_dispatch_method_ignored515 = { let ml_method_owner593 = identities.clone(); let ml_method_argument593_0 = match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2389, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2389.clone()
                            }
                        }; let ml_inserted = vec![ml_method_argument593_0]; let mut ml_method_values593 = ml_method_owner593.borrow_mut(); let ml_length = ml_method_values593.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values593.extend(ml_inserted); ml_length as f64 };
                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                    }
                }
            }
        } else {
            if self::test_java_script_standard_object_observation_operation(&(match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2390, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2390.clone()
                }
            })) {
                {
                    let fn_2 = self::compile_java_script_standard_object_observation_function(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2391, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2391.clone()
                        }
                    }, match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2392, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2392.clone()
                        }
                    }, &(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2393, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2393.clone()
                        }
                    }));
                    if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_2.clone())) {
                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                    } else {
                        {
                            let source_ml_temporary_compile_java_script_collection_dispatch_method_ignored516 = { let ml_method_owner594 = identities.clone(); let ml_method_argument594_0 = match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2394, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2394.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument594_0]; let mut ml_method_values594 = ml_method_owner594.borrow_mut(); let ml_length = ml_method_values594.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values594.extend(ml_inserted); ml_length as f64 };
                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                        }
                    }
                }
            } else {
                if (!(match (match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2395, _, _, _, _) => {
                        ((source_ml_source1_data_property_value2395.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                    }
                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ((self::test_java_script_standard_array_callback_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2396, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2396.clone()
                    }
                })) || self::test_java_script_array_search_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2397, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2397.clone()
                    }
                }))) || self::test_java_script_standard_array_transform_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2398, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2398.clone()
                    }
                })))) {
                    {
                        let fn_3 = self::compile_java_script_boxed_array_read_function(match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2399, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2399.clone()
                            }
                        }, match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2400, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2400.clone()
                            }
                        }, &(match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2401, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2401.clone()
                            }
                        }), match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2402, _, _, _, _) => {
                                ((source_ml_source1_data_property_value2402.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                            }
                        }, match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, source_ml_source1_data_property_value2403, _, _, _, _, _, _, _, _, _, _, _) => {
                                ((source_ml_source1_data_property_value2403.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                            }
                        });
                        if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_3.clone())) {
                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![98, 111, 120, 101, 100, 32, 65, 114, 114, 97, 121, 32, 114, 101, 97, 100, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 119, 114, 97, 112, 112, 101, 114, 32, 114, 111, 111, 116, 115, 32, 97, 110, 100, 32, 115, 112, 101, 99, 105, 101, 115, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                        } else {
                            {
                                let source_ml_temporary_compile_java_script_collection_dispatch_method_ignored517 = { let ml_method_owner595 = identities.clone(); let ml_method_argument595_0 = match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2404, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2404.clone()
                                    }
                                }; let ml_inserted = vec![ml_method_argument595_0]; let mut ml_method_values595 = ml_method_owner595.borrow_mut(); let ml_length = ml_method_values595.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values595.extend(ml_inserted); ml_length as f64 };
                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                            }
                        }
                    }
                } else {
                    if self::test_java_script_array_mutation_operation(&(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2405, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2405.clone()
                        }
                    })) {
                        {
                            let fn_4 = if match (match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2406, _, _, _, _) => {
                                    ((source_ml_source1_data_property_value2406.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                                }
                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::compile_java_script_array_mutation_function(match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2407, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2407.clone()
                                    }
                                }, match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2408, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2408.clone()
                                    }
                                }, &(match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2409, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2409.clone()
                                    }
                                }))
                            } else {
                                self::compile_java_script_boxed_array_mutation_function(match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2410, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2410.clone()
                                    }
                                }, match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2411, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2411.clone()
                                    }
                                }, &(match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2412, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2412.clone()
                                    }
                                }), match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2413, _, _, _, _) => {
                                        ((source_ml_source1_data_property_value2413.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                                    }
                                })
                            };
                            if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_4.clone())) {
                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 109, 117, 116, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                            } else {
                                {
                                    let source_ml_temporary_compile_java_script_collection_dispatch_method_ignored518 = { let ml_method_owner596 = identities.clone(); let ml_method_argument596_0 = match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2414, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2414.clone()
                                        }
                                    }; let ml_inserted = vec![ml_method_argument596_0]; let mut ml_method_values596 = ml_method_owner596.borrow_mut(); let ml_length = ml_method_values596.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values596.extend(ml_inserted); ml_length as f64 };
                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_temporary_compile_java_script_collection_dispatch_method_ignored519 = { let ml_method_owner597 = identities.clone(); let ml_method_argument597_0 = match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2415, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2415.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument597_0]; let mut ml_method_values597 = ml_method_owner597.borrow_mut(); let ml_length = ml_method_values597.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values597.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_temporary_compile_java_script_collection_dispatch_method_ignored520 = { let ml_method_owner598 = functions.clone(); let ml_method_argument598_0 = {
                                    let source_ml_source1_constructor_field2417 = match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2416, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2416.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_constructor_field2419 = match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2418, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2418.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_constructor_field2421 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_constructor_field2422 = 0f64;
                                                {
                                                    let source_ml_source1_constructor_field2423 = 2f64;
                                                    {
                                                        let source_ml_source1_constructor_field2424 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                        {
                                                            let source_ml_source1_constructor_field2442 = if (self::test_java_script_standard_array_transform_operation(&(match method.clone() {
                                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2425, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value2425.clone()
                                                                }
                                                            })) && !(match (match method.clone() {
                                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, source_ml_source1_data_property_value2426, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                    ((source_ml_source1_data_property_value2426.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                    let source_ml_source1_constructor_field2427 = 1f64;
                                                                    {
                                                                        let source_ml_source1_constructor_field2429 = match method.clone() {
                                                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2428, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value2428.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_constructor_field2431 = (match method.clone() {
                                                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, source_ml_source1_data_property_value2430, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    ((source_ml_source1_data_property_value2430.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                                }
                                                                            }).clone().unwrap_or_else(|| self::require_java_script_array_species_directory());
                                                                            self::JavaScriptCallInstruction::ArrayTransform(source_ml_source1_constructor_field2427, source_ml_source1_constructor_field2429.clone(), Box::new(source_ml_source1_constructor_field2431.clone()), None::<(Option<f64>,)>)
                                                                        }
                                                                    }
                                                                }, self::JavaScriptCallInstruction::Return(1f64)]))
                                                            } else {
                                                                if self::test_java_script_standard_array_species_getter_operation(&(match method.clone() {
                                                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2432, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value2432.clone()
                                                                    }
                                                                })) {
                                                                    std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Copy(1f64, 0f64), self::JavaScriptCallInstruction::Return(1f64)]))
                                                                } else {
                                                                    std::rc::Rc::new(std::cell::RefCell::new(vec![if self::test_java_script_standard_array_callback_operation(&(match method.clone() {
                                                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2433, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value2433.clone()
                                                                        }
                                                                    })) {
                                                                        {
                                                                            let source_ml_source1_constructor_field2434 = 1f64;
                                                                            {
                                                                                let source_ml_source1_constructor_field2436 = match method.clone() {
                                                                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2435, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value2435.clone()
                                                                                    }
                                                                                };
                                                                                self::JavaScriptCallInstruction::ArrayCallback(source_ml_source1_constructor_field2434, source_ml_source1_constructor_field2436.clone(), None::<(Option<f64>,)>)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if self::test_java_script_array_search_operation(&(match method.clone() {
                                                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2437, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value2437.clone()
                                                                            }
                                                                        })) {
                                                                            {
                                                                                let source_ml_source1_constructor_field2438 = 1f64;
                                                                                {
                                                                                    let source_ml_source1_constructor_field2440 = match method.clone() {
                                                                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2439, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source1_data_property_value2439.clone()
                                                                                        }
                                                                                    };
                                                                                    self::JavaScriptCallInstruction::ArraySearch(source_ml_source1_constructor_field2438, source_ml_source1_constructor_field2440.clone(), None::<(Option<f64>,)>)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::JavaScriptCallInstruction::PrimitiveMethod(1f64, match method.clone() {
                                                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2441, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source1_data_property_value2441.clone()
                                                                                }
                                                                            })
                                                                        }
                                                                    }, self::JavaScriptCallInstruction::Return(1f64)]))
                                                                }
                                                            };
                                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field2417.clone(), source_ml_source1_constructor_field2419.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field2421.clone()), source_ml_source1_constructor_field2422, source_ml_source1_constructor_field2423, source_ml_source1_constructor_field2424.clone(), source_ml_source1_constructor_field2442.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }; let ml_inserted = vec![ml_method_argument598_0]; let mut ml_method_values598 = ml_method_owner598.borrow_mut(); let ml_length = ml_method_values598.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values598.extend(ml_inserted); ml_length as f64 };
                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                            }
                        }
                    }
                }
            }
        }
    }
}
