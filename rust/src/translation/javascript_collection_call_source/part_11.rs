// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCollectionMethodCompilation;
use super::JavaScriptPrimitiveMethodOwner;
use super::JavaScriptPrimitiveWrapperRoots;
use super::JavaScriptSemanticValue;
use super::append_java_script_compiled_collection_call_function;
use super::compile_java_script_array_call_function;
use super::compile_java_script_array_concatenation_function;
use super::compile_java_script_array_copy_function;
use super::compile_java_script_array_join_function;
use super::compile_java_script_array_slice_function;
use super::compile_java_script_array_sort_function;
use super::compile_java_script_array_string_function;
use super::compile_java_script_function_invocation_function;
use super::compile_java_script_numeric_intrinsic_function;
use super::compile_java_script_object_assignment_function;
use super::compile_java_script_object_call_function;
use super::compile_java_script_object_string_function;
use super::compile_java_script_reflect_construction_function;
use super::compile_java_script_standard_collection_for_each_function;
use super::compile_java_script_standard_object_from_entries_function;
use super::compile_java_script_string_conversion_function;
use super::compile_java_script_string_method_function;
use super::compile_java_script_symbol_registry_function;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::test_java_script_array_copy_operation;
use super::test_java_script_array_join_operation;
use super::test_java_script_array_sort_operation;
use super::test_java_script_function_invocation_operation;
use super::test_java_script_numeric_intrinsic_operation;
use super::test_java_script_standard_collection_for_each_operation;
use super::test_java_script_standard_object_from_entries_operation;
use super::test_java_script_string_conversion_operation;
use super::test_java_script_string_method_operation;
use super::test_java_script_symbol_registry_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_collection_array_method(method: self::JavaScriptPrimitiveMethodOwner, identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>) -> self::JavaScriptCollectionMethodCompilation {
    if ((self::test_java_script_array_join_operation(&(match method.clone() {
        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2159, _, _, _, _, _, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value2159.clone()
        }
    })) || ((match method.clone() {
        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2161, _, _, _, _, _, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value2161.clone()
        }
    }) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 116, 111, 83, 116, 114, 105, 110, 103])))) || ((match method.clone() {
        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2163, _, _, _, _, _, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value2163.clone()
        }
    }) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 116, 111, 83, 116, 114, 105, 110, 103])))) {
        {
            let fn_ = if self::test_java_script_array_join_operation(&(match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2164, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2164.clone()
                }
            })) {
                self::compile_java_script_array_join_function(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2165, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2165.clone()
                    }
                }, match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2166, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2166.clone()
                    }
                }, match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2167, _, _, _, _) => {
                        ((source_ml_source1_data_property_value2167.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                    }
                })
            } else {
                if ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2169, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2169.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 116, 111, 83, 116, 114, 105, 110, 103]))) {
                    self::compile_java_script_array_string_function(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2170, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2170.clone()
                        }
                    }, match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2171, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2171.clone()
                        }
                    }, match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2172, _, _, _, _) => {
                            ((source_ml_source1_data_property_value2172.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                        }
                    }, match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2173) => {
                            ((source_ml_source1_data_property_value2173.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    })
                } else {
                    self::compile_java_script_object_string_function(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2174, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2174.clone()
                        }
                    }, match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2175, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2175.clone()
                        }
                    }, match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2176, _, _, _, _) => {
                            ((source_ml_source1_data_property_value2176.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                        }
                    }, match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2177, _, _) => {
                            ((source_ml_source1_data_property_value2177.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    })
                }
            };
            if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_.clone())) {
                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 115, 116, 114, 105, 110, 103, 32, 98, 111, 100, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 119, 114, 97, 112, 112, 101, 114, 32, 114, 111, 111, 116, 115, 32, 97, 110, 100, 32, 116, 97, 103, 32, 111, 114, 32, 102, 97, 108, 108, 98, 97, 99, 107, 32, 111, 119, 110, 101, 114, 115]))
            } else {
                {
                    let source_ml_temporary_compile_java_script_collection_array_method_ignored483 = { let ml_method_owner564 = identities.clone(); let ml_method_argument564_0 = match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2178, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2178.clone()
                        }
                    }; let ml_inserted = vec![ml_method_argument564_0]; let mut ml_method_values564 = ml_method_owner564.borrow_mut(); let ml_length = ml_method_values564.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values564.extend(ml_inserted); ml_length as f64 };
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                }
            }
        }
    } else {
        if self::test_java_script_array_sort_operation(&(match method.clone() {
            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2179, _, _, _, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value2179.clone()
            }
        })) {
            {
                let fn_2 = self::compile_java_script_array_sort_function(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2180, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2180.clone()
                    }
                }, match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2181, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2181.clone()
                    }
                }, &(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2182, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2182.clone()
                    }
                }), match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2183, _, _, _, _) => {
                        ((source_ml_source1_data_property_value2183.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                    }
                }, match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, source_ml_source1_data_property_value2184, _, _, _, _, _, _, _, _, _, _) => {
                        ((source_ml_source1_data_property_value2184.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                    }
                });
                if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_2.clone())) {
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 111, 114, 116, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 119, 114, 97, 112, 112, 101, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 114, 111, 111, 116, 115]))
                } else {
                    {
                        let source_ml_temporary_compile_java_script_collection_array_method_ignored484 = { let ml_method_owner565 = identities.clone(); let ml_method_argument565_0 = match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2185, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2185.clone()
                            }
                        }; let ml_inserted = vec![ml_method_argument565_0]; let mut ml_method_values565 = ml_method_owner565.borrow_mut(); let ml_length = ml_method_values565.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values565.extend(ml_inserted); ml_length as f64 };
                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                    }
                }
            }
        } else {
            if self::test_java_script_array_copy_operation(&(match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2186, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2186.clone()
                }
            })) {
                {
                    let fn_3 = self::compile_java_script_array_copy_function(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2187, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2187.clone()
                        }
                    }, match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2188, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2188.clone()
                        }
                    }, &(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2189, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2189.clone()
                        }
                    }), match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2190, _, _, _, _) => {
                            ((source_ml_source1_data_property_value2190.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                        }
                    }, match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, source_ml_source1_data_property_value2191, _, _, _, _, _, _, _, _, _, _) => {
                            ((source_ml_source1_data_property_value2191.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                        }
                    });
                    if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_3.clone())) {
                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 112, 105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 119, 114, 97, 112, 112, 101, 114, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 114, 111, 111, 116, 115]))
                    } else {
                        {
                            let source_ml_temporary_compile_java_script_collection_array_method_ignored485 = { let ml_method_owner566 = identities.clone(); let ml_method_argument566_0 = match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2192, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2192.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument566_0]; let mut ml_method_values566 = ml_method_owner566.borrow_mut(); let ml_length = ml_method_values566.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values566.extend(ml_inserted); ml_length as f64 };
                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                        }
                    }
                }
            } else {
                if ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2194, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2194.clone()
                    }
                }) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 99, 111, 110, 99, 97, 116]))) {
                    {
                        let fn_4 = self::compile_java_script_array_concatenation_function(match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2195, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2195.clone()
                            }
                        }, match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2196, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2196.clone()
                            }
                        }, match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2197, _, _, _, _) => {
                                ((source_ml_source1_data_property_value2197.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                            }
                        }, match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, source_ml_source1_data_property_value2198, _, _, _, _, _, _, _, _, _, _, _) => {
                                ((source_ml_source1_data_property_value2198.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                            }
                        }, match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2199, _) => {
                                ((source_ml_source1_data_property_value2199.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        });
                        if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_4.clone())) {
                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 119, 114, 97, 112, 112, 101, 114, 32, 115, 112, 101, 99, 105, 101, 115, 32, 97, 110, 100, 32, 115, 112, 114, 101, 97, 100, 97, 98, 105, 108, 105, 116, 121, 32, 114, 111, 111, 116, 115]))
                        } else {
                            {
                                let source_ml_temporary_compile_java_script_collection_array_method_ignored486 = { let ml_method_owner567 = identities.clone(); let ml_method_argument567_0 = match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2200, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2200.clone()
                                    }
                                }; let ml_inserted = vec![ml_method_argument567_0]; let mut ml_method_values567 = ml_method_owner567.borrow_mut(); let ml_length = ml_method_values567.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values567.extend(ml_inserted); ml_length as f64 };
                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                            }
                        }
                    }
                } else {
                    if ((match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2202, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2202.clone()
                        }
                    }) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 115, 108, 105, 99, 101]))) {
                        {
                            let fn_5 = self::compile_java_script_array_slice_function(match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2203, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2203.clone()
                                }
                            }, match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2204, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2204.clone()
                                }
                            }, match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2205, _, _, _, _) => {
                                    ((source_ml_source1_data_property_value2205.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                                }
                            }, match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, source_ml_source1_data_property_value2206, _, _, _, _, _, _, _, _, _, _, _) => {
                                    ((source_ml_source1_data_property_value2206.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                }
                            });
                            if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_5.clone())) {
                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 108, 105, 99, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 119, 114, 97, 112, 112, 101, 114, 32, 97, 110, 100, 32, 115, 112, 101, 99, 105, 101, 115, 32, 114, 111, 111, 116, 115]))
                            } else {
                                {
                                    let source_ml_temporary_compile_java_script_collection_array_method_ignored487 = { let ml_method_owner568 = identities.clone(); let ml_method_argument568_0 = match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2207, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2207.clone()
                                        }
                                    }; let ml_inserted = vec![ml_method_argument568_0]; let mut ml_method_values568 = ml_method_owner568.borrow_mut(); let ml_length = ml_method_values568.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values568.extend(ml_inserted); ml_length as f64 };
                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                                }
                            }
                        }
                    } else {
                        if ((match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2209, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2209.clone()
                            }
                        }) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))) {
                            {
                                let source_ml_temporary_compile_java_script_collection_array_method_ignored488 = { let ml_method_owner569 = identities.clone(); let ml_method_argument569_0 = match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2210, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2210.clone()
                                    }
                                }; let ml_inserted = vec![ml_method_argument569_0]; let mut ml_method_values569 = ml_method_owner569.borrow_mut(); let ml_length = ml_method_values569.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values569.extend(ml_inserted); ml_length as f64 };
                                {
                                    let source_ml_temporary_compile_java_script_collection_array_method_ignored489 = { let ml_method_owner570 = functions.clone(); let ml_method_argument570_0 = self::compile_java_script_array_call_function(match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2211, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2211.clone()
                                        }
                                    }, match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2212, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2212.clone()
                                        }
                                    }); let ml_inserted = vec![ml_method_argument570_0]; let mut ml_method_values570 = ml_method_owner570.borrow_mut(); let ml_length = ml_method_values570.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values570.extend(ml_inserted); ml_length as f64 };
                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                                }
                            }
                        } else {
                            if ((match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2214, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2214.clone()
                                }
                            }) == (self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 99, 111, 110, 115, 116, 114, 117, 99, 116]))) {
                                {
                                    let source_ml_temporary_compile_java_script_collection_array_method_ignored490 = { let ml_method_owner571 = identities.clone(); let ml_method_argument571_0 = match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2215, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2215.clone()
                                        }
                                    }; let ml_inserted = vec![ml_method_argument571_0]; let mut ml_method_values571 = ml_method_owner571.borrow_mut(); let ml_length = ml_method_values571.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values571.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let source_ml_temporary_compile_java_script_collection_array_method_ignored491 = { let ml_method_owner572 = functions.clone(); let ml_method_argument572_0 = self::compile_java_script_reflect_construction_function(match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2216, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2216.clone()
                                            }
                                        }, match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2217, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2217.clone()
                                            }
                                        }); let ml_inserted = vec![ml_method_argument572_0]; let mut ml_method_values572 = ml_method_owner572.borrow_mut(); let ml_length = ml_method_values572.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values572.extend(ml_inserted); ml_length as f64 };
                                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                                    }
                                }
                            } else {
                                if self::test_java_script_standard_object_from_entries_operation(&(match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2218, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2218.clone()
                                    }
                                })) {
                                    if {
                                        let source_ml_source1_logical2232 = {
                                            let source_ml_source1_logical2227 = {
                                                let source_ml_source1_comparison_operand2221 = {
                                                    let source_ml_source1_optional_chain_receiver2220 = (match method.clone() {
                                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value2219, _, _, _, _, _, _, _, _, _) => {
                                                            ((source_ml_source1_data_property_value2219.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    },);
                                                    if match (source_ml_source1_optional_chain_receiver2220.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        Some({
                                                            let source_ml_temporary_optional_chain481 = ((source_ml_source1_optional_chain_receiver2220.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_temporary_optional_chain481.clone() {
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
                                                !(match (source_ml_source1_comparison_operand2221.clone(), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                            };
                                            if source_ml_source1_logical2227 {
                                                Some(source_ml_source1_logical2227)
                                            } else {
                                                {
                                                    let source_ml_source1_nullish_field_receiver2224 = (match method.clone() {
                                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value2223, _, _, _, _, _, _, _, _, _) => {
                                                            ((source_ml_source1_data_property_value2223.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    },);
                                                    if match (source_ml_source1_nullish_field_receiver2224.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        {
                                                            let source_ml_source1_nullish_field_present2225 = ((source_ml_source1_nullish_field_receiver2224.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_nullish_field_present2225.clone() {
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
                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value2226) => {
                                                                    Some(source_ml_source1_data_property_value2226)
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                                    }
                                                }
                                            }
                                        };
                                        if match source_ml_source1_logical2232.clone() { Some(ml_present) => ml_present, None => false } {
                                            source_ml_source1_logical2232.clone().expect("a proven present value")
                                        } else {
                                            {
                                                let source_ml_source1_comparison_operand2230 = {
                                                    let source_ml_source1_optional_chain_receiver2229 = (match method.clone() {
                                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2228, _, _, _) => {
                                                            ((source_ml_source1_data_property_value2228.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                        }
                                                    },);
                                                    if match (source_ml_source1_optional_chain_receiver2229.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        Some({
                                                            let source_ml_temporary_optional_chain482 = ((source_ml_source1_optional_chain_receiver2229.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_temporary_optional_chain482.clone() {
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
                                                !(match (source_ml_source1_comparison_operand2230.clone(), self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                            }
                                        }
                                    } {
                                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 79, 98, 106, 101, 99, 116, 32, 114, 111, 111, 116, 32, 97, 110, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 83, 121, 109, 98, 111, 108]))
                                    } else {
                                        {
                                            let fn_6 = self::compile_java_script_standard_object_from_entries_function(match method.clone() {
                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2233, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2233.clone()
                                                }
                                            }, match method.clone() {
                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2234, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2234.clone()
                                                }
                                            }, (match method.clone() {
                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value2235, _, _, _, _, _, _, _, _, _) => {
                                                    ((source_ml_source1_data_property_value2235.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (match method.clone() {
                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2236, _, _, _) => {
                                                    ((source_ml_source1_data_property_value2236.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                }
                                            }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                            if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_6.clone())) {
                                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 102, 114, 111, 109, 69, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                            } else {
                                                {
                                                    let source_ml_temporary_compile_java_script_collection_array_method_ignored492 = { let ml_method_owner573 = identities.clone(); let ml_method_argument573_0 = match method.clone() {
                                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2237, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value2237.clone()
                                                        }
                                                    }; let ml_inserted = vec![ml_method_argument573_0]; let mut ml_method_values573 = ml_method_owner573.borrow_mut(); let ml_length = ml_method_values573.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values573.extend(ml_inserted); ml_length as f64 };
                                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    if ((match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2239, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2239.clone()
                                        }
                                    }) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]))) {
                                        {
                                            let fn_7 = self::compile_java_script_object_call_function(match method.clone() {
                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2240, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2240.clone()
                                                }
                                            }, match method.clone() {
                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2241, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2241.clone()
                                                }
                                            }, match method.clone() {
                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2242, _, _, _, _) => {
                                                    ((source_ml_source1_data_property_value2242.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                                                }
                                            });
                                            if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_7.clone())) {
                                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 67, 97, 108, 108, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101, 114, 32, 114, 111, 111, 116]))
                                            } else {
                                                {
                                                    let source_ml_temporary_compile_java_script_collection_array_method_ignored493 = { let ml_method_owner574 = identities.clone(); let ml_method_argument574_0 = match method.clone() {
                                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2243, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value2243.clone()
                                                        }
                                                    }; let ml_inserted = vec![ml_method_argument574_0]; let mut ml_method_values574 = ml_method_owner574.borrow_mut(); let ml_length = ml_method_values574.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values574.extend(ml_inserted); ml_length as f64 };
                                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
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
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_collection_protocol_method(method: self::JavaScriptPrimitiveMethodOwner, identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>) -> self::JavaScriptCollectionMethodCompilation {
    if self::test_java_script_standard_collection_for_each_operation(&(match method.clone() {
        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2244, _, _, _, _, _, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value2244.clone()
        }
    })) {
        {
            let fn_ = self::compile_java_script_standard_collection_for_each_function(match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2245, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2245.clone()
                }
            }, match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2246, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2246.clone()
                }
            }, &(match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2247, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2247.clone()
                }
            }));
            if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_.clone())) {
                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 109, 101, 116, 104, 111, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
            } else {
                {
                    let source_ml_temporary_compile_java_script_collection_protocol_method_ignored494 = { let ml_method_owner575 = identities.clone(); let ml_method_argument575_0 = match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2248, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2248.clone()
                        }
                    }; let ml_inserted = vec![ml_method_argument575_0]; let mut ml_method_values575 = ml_method_owner575.borrow_mut(); let ml_length = ml_method_values575.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values575.extend(ml_inserted); ml_length as f64 };
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                }
            }
        }
    } else {
        if self::test_java_script_symbol_registry_operation(&(match method.clone() {
            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2249, _, _, _, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value2249.clone()
            }
        })) {
            {
                let fn_2 = self::compile_java_script_symbol_registry_function(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2250, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2250.clone()
                    }
                }, match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2251, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2251.clone()
                    }
                }, &(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2252, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2252.clone()
                    }
                }));
                if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_2.clone())) {
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                } else {
                    {
                        let source_ml_temporary_compile_java_script_collection_protocol_method_ignored495 = { let ml_method_owner576 = identities.clone(); let ml_method_argument576_0 = match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2253, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2253.clone()
                            }
                        }; let ml_inserted = vec![ml_method_argument576_0]; let mut ml_method_values576 = ml_method_owner576.borrow_mut(); let ml_length = ml_method_values576.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values576.extend(ml_inserted); ml_length as f64 };
                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                    }
                }
            }
        } else {
            if self::test_java_script_string_conversion_operation(&(match method.clone() {
                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2254, _, _, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2254.clone()
                }
            })) {
                {
                    let fn_3 = self::compile_java_script_string_conversion_function(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2255, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2255.clone()
                        }
                    }, match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2256, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2256.clone()
                        }
                    }, &(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2257, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2257.clone()
                        }
                    }));
                    if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_3.clone())) {
                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 97, 110, 100, 32, 83, 121, 109, 98, 111, 108, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                    } else {
                        {
                            let source_ml_temporary_compile_java_script_collection_protocol_method_ignored496 = { let ml_method_owner577 = identities.clone(); let ml_method_argument577_0 = match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2258, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2258.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument577_0]; let mut ml_method_values577 = ml_method_owner577.borrow_mut(); let ml_length = ml_method_values577.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values577.extend(ml_inserted); ml_length as f64 };
                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                        }
                    }
                }
            } else {
                if (self::ml_array::includes(&{ let ml_reference_value578 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 116, 111, 76, 111, 119, 101, 114, 67, 97, 115, 101]), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 116, 111, 85, 112, 112, 101, 114, 67, 97, 115, 101])]))).borrow().clone(); ml_reference_value578 }, &match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2259, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2259.clone()
                    }
                }, 0f64, false, |a, b| a == b) && (((match match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2261, _, _, _, _, _) => {
                        ((source_ml_source1_data_property_value2261.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                    }
                } { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (({
                    let source_ml_source1_nullish_field_receiver2266 = (match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2265, _, _, _, _, _) => {
                            ((source_ml_source1_data_property_value2265.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                        }
                    },);
                    if ((match (source_ml_source1_nullish_field_receiver2266.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                        {
                            let source_ml_source1_nullish_field_present2267 = ((source_ml_source1_nullish_field_receiver2266.clone()).0).clone().expect("a proven present value");
                            source_ml_source1_nullish_field_present2267.length()
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                    }
                }) == (0f64)))) {
                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 99, 97, 115, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 85, 110, 105, 99, 111, 100, 101, 32, 112, 114, 111, 102, 105, 108, 101, 59, 32, 117, 110, 99, 111, 118, 101, 114, 101, 100, 32, 112, 114, 111, 102, 105, 108, 101, 115, 32, 114, 101, 112, 111, 114, 116, 32, 111, 110, 32, 117, 115, 101]))
                } else {
                    if self::test_java_script_numeric_intrinsic_operation(match method.clone() {
                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2268, _, _, _, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source1_data_property_value2268.clone()
                        }
                    }) {
                        {
                            let fn_4 = self::compile_java_script_numeric_intrinsic_function(match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2269, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2269.clone()
                                }
                            }, match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2270, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2270.clone()
                                }
                            }, &(match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2271, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2271.clone()
                                }
                            }));
                            if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_4.clone())) {
                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 99, 97, 108, 108, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                            } else {
                                {
                                    let source_ml_temporary_compile_java_script_collection_protocol_method_ignored497 = { let ml_method_owner579 = identities.clone(); let ml_method_argument579_0 = match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2272, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2272.clone()
                                        }
                                    }; let ml_inserted = vec![ml_method_argument579_0]; let mut ml_method_values579 = ml_method_owner579.borrow_mut(); let ml_length = ml_method_values579.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values579.extend(ml_inserted); ml_length as f64 };
                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                                }
                            }
                        }
                    } else {
                        if (self::test_java_script_string_method_operation(match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2273, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2273.clone()
                            }
                        }) && !((((match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2275, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2275.clone()
                            }
                        }) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104]))) && match (match method.clone() {
                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, source_ml_source1_data_property_value2276, _, _, _, _, _, _, _) => {
                                ((source_ml_source1_data_property_value2276.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                            }
                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }))) {
                            {
                                let fn_5 = self::compile_java_script_string_method_function(match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2277, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2277.clone()
                                    }
                                }, match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2278, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2278.clone()
                                    }
                                }, &(match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2279, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2279.clone()
                                    }
                                }), match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, source_ml_source1_data_property_value2280, _, _, _, _, _, _, _) => {
                                        ((source_ml_source1_data_property_value2280.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                    }
                                }, match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2281, _, _, _, _, _) => {
                                        ((source_ml_source1_data_property_value2281.clone().unwrap_or_else(|| (None::<self::ml_string::Text>,))).0).clone()
                                    }
                                });
                                if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_5.clone())) {
                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97, 110, 100, 32, 73, 115, 82, 101, 103, 69, 120, 112, 32, 83, 121, 109, 98, 111, 108, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                } else {
                                    {
                                        let source_ml_temporary_compile_java_script_collection_protocol_method_ignored498 = { let ml_method_owner580 = identities.clone(); let ml_method_argument580_0 = match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2282, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2282.clone()
                                            }
                                        }; let ml_inserted = vec![ml_method_argument580_0]; let mut ml_method_values580 = ml_method_owner580.borrow_mut(); let ml_length = ml_method_values580.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values580.extend(ml_inserted); ml_length as f64 };
                                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                                    }
                                }
                            }
                        } else {
                            if self::test_java_script_function_invocation_operation(&(match method.clone() {
                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2283, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2283.clone()
                                }
                            })) {
                                {
                                    let fn_6 = self::compile_java_script_function_invocation_function(match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2284, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2284.clone()
                                        }
                                    }, match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2285, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2285.clone()
                                        }
                                    }, &(match method.clone() {
                                        self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2286, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2286.clone()
                                        }
                                    }));
                                    if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_6.clone())) {
                                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110, 32, 105, 110, 118, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108, 47, 97, 112, 112, 108, 121, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                    } else {
                                        {
                                            let source_ml_temporary_compile_java_script_collection_protocol_method_ignored499 = { let ml_method_owner581 = identities.clone(); let ml_method_argument581_0 = match method.clone() {
                                                self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2287, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source1_data_property_value2287.clone()
                                                }
                                            }; let ml_inserted = vec![ml_method_argument581_0]; let mut ml_method_values581 = ml_method_owner581.borrow_mut(); let ml_length = ml_method_values581.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values581.extend(ml_inserted); ml_length as f64 };
                                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
                                        }
                                    }
                                }
                            } else {
                                if (((match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value2289, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2289.clone()
                                    }
                                }) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110]))) && !(match (match method.clone() {
                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2290, _, _, _, _) => {
                                        ((source_ml_source1_data_property_value2290.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                                    }
                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                    {
                                        let fn_7 = self::compile_java_script_object_assignment_function(match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2291, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2291.clone()
                                            }
                                        }, match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value2292, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2292.clone()
                                            }
                                        }, match method.clone() {
                                            self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value2293, _, _, _, _) => {
                                                ((source_ml_source1_data_property_value2293.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperRoots>,))).0).clone()
                                            }
                                        });
                                        if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_7.clone())) {
                                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 97, 115, 115, 105, 103, 110, 32, 98, 111, 120, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 119, 114, 97, 112, 112, 101, 114, 32, 114, 111, 111, 116, 115]))
                                        } else {
                                            {
                                                let source_ml_temporary_compile_java_script_collection_protocol_method_ignored500 = { let ml_method_owner582 = identities.clone(); let ml_method_argument582_0 = match method.clone() {
                                                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value2294, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                        source_ml_source1_data_property_value2294.clone()
                                                    }
                                                }; let ml_inserted = vec![ml_method_argument582_0]; let mut ml_method_values582 = ml_method_owner582.borrow_mut(); let ml_length = ml_method_values582.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values582.extend(ml_inserted); ml_length as f64 };
                                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(true, self::ml_string::Text::from_units(vec![]))
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
        }
    }
}
