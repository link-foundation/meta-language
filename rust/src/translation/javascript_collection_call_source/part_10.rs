// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCollectionMethodCompilation;
use super::JavaScriptPrimitiveMethodOwner;
use super::JavaScriptSemanticValue;
use super::append_java_script_compiled_collection_call_function;
use super::compile_java_script_standard_array_creation_function;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::test_java_script_standard_array_creation_operation;
use super::test_java_script_standard_array_transform_operation;
use super::test_java_script_standard_object_enumeration_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_collection_creation_method(method: self::JavaScriptPrimitiveMethodOwner, identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>) -> self::JavaScriptCollectionMethodCompilation {
    if (self::test_java_script_standard_array_transform_operation(&(match method.clone() {
        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2094, _, _, _, _, _, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value2094.clone()
        }
    })) && {
        let source_ml_source1_comparison_operand2099 = {
            let source_ml_source1_optional_chain_receiver2096 = (match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, source_ml_source1_data_property_value2095, _, _, _, _, _, _, _, _, _, _, _) => {
                    ((source_ml_source1_data_property_value2095.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                }
            },);
            if match (source_ml_source1_optional_chain_receiver2096.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                Some({
                    let source_ml_temporary_optional_chain474 = ((source_ml_source1_optional_chain_receiver2096.clone()).0).clone().expect("a proven present value");
                    match source_ml_temporary_optional_chain474.clone() {
                        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(source_ml_source1_data_property_value2097, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2097.clone()
                        }
                    }
                })
            } else {
                None::<self::ml::Big>
            }
        };
        {
            let source_ml_source1_comparison_operand2100 = match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2098, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2098.clone()
                }
            };
            !(match (source_ml_source1_comparison_operand2099.clone(), source_ml_source1_comparison_operand2100.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
        }
    }) {
        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 116, 114, 97, 110, 115, 102, 111, 114, 109, 97, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 111, 119, 110, 101, 114, 32, 114, 101, 97, 108, 109, 32, 115, 112, 101, 99, 105, 101, 115, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
    } else {
        if self::test_java_script_standard_object_enumeration_operation(&(match method.clone() {
            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2101, _, _, _, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value2101.clone()
            }
        })) {
            if match {
                let source_ml_source1_logical2110 = {
                    let source_ml_source1_comparison_operand2104 = {
                        let source_ml_source1_optional_chain_receiver2103 = (match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, source_ml_source1_data_property_value2102, _, _, _, _, _, _, _, _, _, _) => {
                                ((source_ml_source1_data_property_value2102.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        },);
                        if match (source_ml_source1_optional_chain_receiver2103.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            Some({
                                let source_ml_temporary_optional_chain475 = ((source_ml_source1_optional_chain_receiver2103.clone()).0).clone().expect("a proven present value");
                                match source_ml_temporary_optional_chain475.clone() {
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
                    !(match (source_ml_source1_comparison_operand2104.clone(), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                };
                if source_ml_source1_logical2110 {
                    Some(source_ml_source1_logical2110)
                } else {
                    {
                        let source_ml_source1_nullish_field_receiver2107 = (match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, source_ml_source1_data_property_value2106, _, _, _, _, _, _, _, _, _, _) => {
                                ((source_ml_source1_data_property_value2106.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        },);
                        if match (source_ml_source1_nullish_field_receiver2107.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source1_nullish_field_present2108 = ((source_ml_source1_nullish_field_receiver2107.clone()).0).clone().expect("a proven present value");
                                match source_ml_source1_nullish_field_present2108.clone() {
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
                                    self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value2109) => {
                                        Some(source_ml_source1_data_property_value2109)
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                        }
                    }
                }
            } { Some(ml_present) => ml_present, None => false } {
                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))
            } else {
                {
                    let source_ml_temporary_compile_java_script_collection_creation_method_ignored478 = { let ml_method_owner559 = identities.clone(); let ml_method_argument559_0 = match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2111, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2111.clone()
                        }
                    }; let ml_inserted = vec![ml_method_argument559_0]; let mut ml_method_values559 = ml_method_owner559.borrow_mut(); let ml_length = ml_method_values559.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values559.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_temporary_compile_java_script_collection_creation_method_ignored479 = { let ml_method_owner560 = functions.clone(); let ml_method_argument560_0 = {
                            let source_ml_source1_constructor_field2113 = match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2112, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2112.clone()
                                }
                            };
                            {
                                let source_ml_source1_constructor_field2115 = match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2114, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2114.clone()
                                    }
                                };
                                {
                                    let source_ml_source1_constructor_field2117 = self::JavaScriptSemanticValue::Undefined;
                                    {
                                        let source_ml_source1_constructor_field2118 = 0f64;
                                        {
                                            let source_ml_source1_constructor_field2119 = 3f64;
                                            {
                                                let source_ml_source1_constructor_field2120 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                {
                                                    let source_ml_source1_constructor_field2123 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Constant(1f64, Box::new((match method.clone() {
                                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, source_ml_source1_data_property_value2121, _, _, _, _, _, _, _, _, _, _) => {
                                                            ((source_ml_source1_data_property_value2121.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))), self::JavaScriptCallInstruction::ObjectEnumeration(2f64, match method.clone() {
                                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2122, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value2122.clone()
                                                        }
                                                    }, 1f64), self::JavaScriptCallInstruction::Return(2f64)]));
                                                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field2113.clone(), source_ml_source1_constructor_field2115.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field2117.clone()), source_ml_source1_constructor_field2118, source_ml_source1_constructor_field2119, source_ml_source1_constructor_field2120.clone(), source_ml_source1_constructor_field2123.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }; let ml_inserted = vec![ml_method_argument560_0]; let mut ml_method_values560 = ml_method_owner560.borrow_mut(); let ml_length = ml_method_values560.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values560.extend(ml_inserted); ml_length as f64 };
                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                    }
                }
            }
        } else {
            if (self::ml_array::includes(&{ let ml_reference_value561 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value561 }, &match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2124, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2124.clone()
                }
            }, 0f64, false, |a, b| a == b) && {
                let source_ml_source1_comparison_operand2126 = match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value2125, _, _, _, _, _, _) => {
                        ((source_ml_source1_data_property_value2125.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                    }
                };
                !(match (source_ml_source1_comparison_operand2126.clone(), true) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
            }) {
                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 79, 98, 106, 101, 99, 116, 32, 78, 117, 109, 98, 101, 114, 32, 66, 111, 111, 108, 101, 97, 110, 32, 97, 110, 100, 32, 83, 116, 114, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 119, 110, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 105, 110, 100, 101, 112, 101, 110, 100, 101, 110, 116, 108, 121, 32, 111, 102, 32, 67, 97, 108, 108, 32, 115, 117, 112, 112, 111, 114, 116]))
            } else {
                if match {
                    let source_ml_source1_logical2137 = {
                        let source_ml_source1_comparison_operand2129 = match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value2128, _, _, _, _, _, _) => {
                                ((source_ml_source1_data_property_value2128.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                            }
                        };
                        {
                            let source_ml_source1_comparison_operand2130 = None::<()>;
                            !(match (source_ml_source1_comparison_operand2129.clone(), source_ml_source1_comparison_operand2130.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
                        }
                    };
                    if source_ml_source1_logical2137 {
                        {
                            let source_ml_source1_logical2136 = ((match match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value2132, _, _, _, _, _, _) => {
                                    ((source_ml_source1_data_property_value2132.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                }
                            } { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])));
                            if source_ml_source1_logical2136 {
                                Some(source_ml_source1_logical2136)
                            } else {
                                {
                                    let source_ml_source1_logical2135 = match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, source_ml_source1_data_property_value2133, _, _, _, _, _, _) => {
                                            ((source_ml_source1_data_property_value2133.clone().unwrap_or_else(|| (None::<bool>,))).0).clone()
                                        }
                                    };
                                    if match source_ml_source1_logical2135.clone() { Some(ml_present) => ml_present, None => false } {
                                        Some(!(self::ml_array::includes(&{ let ml_reference_value562 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]), self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]), self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103])]))).borrow().clone(); ml_reference_value562 }, &match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2134, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2134.clone()
                                            }
                                        }, 0f64, false, |a, b| a == b)))
                                    } else {
                                        source_ml_source1_logical2135.clone()
                                    }
                                }
                            }
                        }
                    } else {
                        Some(source_ml_source1_logical2137)
                    }
                } { Some(ml_present) => ml_present, None => false } {
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 98, 108, 101, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 99, 97, 108, 108, 32, 111, 119, 110, 101, 114, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 97, 100, 109, 105, 116, 116, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 107, 105, 110, 100]))
                } else {
                    if self::test_java_script_standard_array_creation_operation(&(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2138, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2138.clone()
                        }
                    })) {
                        if {
                            let source_ml_source1_logical2152 = {
                                let source_ml_source1_logical2147 = {
                                    let source_ml_source1_comparison_operand2141 = {
                                        let source_ml_source1_optional_chain_receiver2140 = (match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, source_ml_source1_data_property_value2139, _, _, _, _, _, _, _, _, _, _) => {
                                                ((source_ml_source1_data_property_value2139.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        },);
                                        if match (source_ml_source1_optional_chain_receiver2140.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            Some({
                                                let source_ml_temporary_optional_chain476 = ((source_ml_source1_optional_chain_receiver2140.clone()).0).clone().expect("a proven present value");
                                                match source_ml_temporary_optional_chain476.clone() {
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
                                    !(match (source_ml_source1_comparison_operand2141.clone(), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                };
                                if source_ml_source1_logical2147 {
                                    Some(source_ml_source1_logical2147)
                                } else {
                                    {
                                        let source_ml_source1_nullish_field_receiver2144 = (match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, source_ml_source1_data_property_value2143, _, _, _, _, _, _, _, _, _, _) => {
                                                ((source_ml_source1_data_property_value2143.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        },);
                                        if match (source_ml_source1_nullish_field_receiver2144.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            {
                                                let source_ml_source1_nullish_field_present2145 = ((source_ml_source1_nullish_field_receiver2144.clone()).0).clone().expect("a proven present value");
                                                match source_ml_source1_nullish_field_present2145.clone() {
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
                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value2146) => {
                                                        Some(source_ml_source1_data_property_value2146)
                                                    }
                                                }
                                            }
                                        } else {
                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                        }
                                    }
                                }
                            };
                            if match source_ml_source1_logical2152.clone() { Some(ml_present) => ml_present, None => false } {
                                source_ml_source1_logical2152.clone().expect("a proven present value")
                            } else {
                                {
                                    let source_ml_source1_comparison_operand2150 = {
                                        let source_ml_source1_optional_chain_receiver2149 = (match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2148, _, _, _) => {
                                                ((source_ml_source1_data_property_value2148.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        },);
                                        if match (source_ml_source1_optional_chain_receiver2149.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                            Some({
                                                let source_ml_temporary_optional_chain477 = ((source_ml_source1_optional_chain_receiver2149.clone()).0).clone().expect("a proven present value");
                                                match source_ml_temporary_optional_chain477.clone() {
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
                                    !(match (source_ml_source1_comparison_operand2150.clone(), self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                }
                            }
                        } {
                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 97, 110, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 83, 121, 109, 98, 111, 108]))
                        } else {
                            {
                                let fn_ = self::compile_java_script_standard_array_creation_function(match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2153, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2153.clone()
                                    }
                                }, match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2154, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2154.clone()
                                    }
                                }, &(match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2155, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2155.clone()
                                    }
                                }), (match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, source_ml_source1_data_property_value2156, _, _, _, _, _, _, _, _, _, _) => {
                                        ((source_ml_source1_data_property_value2156.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2157, _, _, _) => {
                                        ((source_ml_source1_data_property_value2157.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                    }
                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_.clone())) {
                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                } else {
                                    {
                                        let source_ml_temporary_compile_java_script_collection_creation_method_ignored480 = { let ml_method_owner563 = identities.clone(); let ml_method_argument563_0 = match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2158, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2158.clone()
                                            }
                                        }; let ml_inserted = vec![ml_method_argument563_0]; let mut ml_method_values563 = ml_method_owner563.borrow_mut(); let ml_length = ml_method_values563.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values563.extend(ml_inserted); ml_length as f64 };
                                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
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
    }
}
