// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCollectionCallCompilation;
use super::JavaScriptCollectionCallableDirectory;
use super::JavaScriptCollectionConstructorOwner;
use super::JavaScriptCollectionMethodCompilation;
use super::JavaScriptCollectionMethodOwner;
use super::JavaScriptPrimitiveMethodOwner;
use super::JavaScriptSemanticValue;
use super::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result;
use super::append_java_script_compiled_collection_call_function;
use super::compile_java_script_collection_array_method;
use super::compile_java_script_collection_creation_method;
use super::compile_java_script_collection_dispatch_method;
use super::compile_java_script_collection_object_method;
use super::compile_java_script_collection_protocol_method;
use super::compile_java_script_iterable_collection_constructor;
use super::compile_java_script_weak_collection_function;
use super::dependency_function2;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_collection_operation_kind;
use super::test_java_script_array_copy_operation;
use super::test_java_script_array_join_operation;
use super::test_java_script_array_mutation_operation;
use super::test_java_script_array_search_operation;
use super::test_java_script_array_sort_operation;
use super::test_java_script_function_invocation_operation;
use super::test_java_script_numeric_intrinsic_operation;
use super::test_java_script_object_assignment_operation;
use super::test_java_script_object_definition_operation;
use super::test_java_script_standard_array_callback_operation;
use super::test_java_script_standard_array_creation_operation;
use super::test_java_script_standard_array_intrinsic_operation;
use super::test_java_script_standard_array_species_getter_operation;
use super::test_java_script_standard_array_transform_operation;
use super::test_java_script_standard_collection_for_each_operation;
use super::test_java_script_standard_object_enumeration_operation;
use super::test_java_script_standard_object_from_entries_operation;
use super::test_java_script_standard_object_observation_operation;
use super::test_java_script_standard_primitive_method_operation;
use super::test_java_script_standard_property_definitions_operation;
use super::test_java_script_standard_property_reflection_operation;
use super::test_java_script_string_conversion_operation;
use super::test_java_script_string_method_operation;
use super::test_java_script_symbol_registry_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_collection_call_directory_loop526(mut directory: self::JavaScriptCollectionCallableDirectory, mut empty: self::JavaScriptCallProgram, mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut source_ml_temporary_values523: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptPrimitiveMethodOwner>>>, mut source_ml_temporary_index523: self::ml::Big) -> self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result {
    loop {
        return if (source_ml_temporary_index523 < ({ let ml_reference_array694 = source_ml_temporary_values523.clone(); let ml_reference_projection694 = self::ml::Big::from_u128(ml_reference_array694.borrow().len() as u128); ml_reference_projection694 })) {
            {
                let method = { let ml_reference_array695 = source_ml_temporary_values523.clone(); let ml_reference_position695 = source_ml_temporary_index523.clone(); let ml_reference_projection695 = self::ml_array::at(&*ml_reference_array695.borrow(), ml_reference_position695.to_index()); ml_reference_projection695 };
                if ((((((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value3535, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3535.clone()
                    }
                }) <= (self::ml::Big::from_i128(0))) || ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value3537, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3537.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) || self::ml_array::includes(&{ let ml_reference_value696 = identities.borrow().clone(); ml_reference_value696 }, &match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(source_ml_source1_data_property_value3538, _, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3538.clone()
                    }
                }, 0f64, false, |a, b| a == b)) || !(self::ml_array::includes(&{ let ml_reference_value697 = (match directory.clone() {
                    self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(source_ml_source1_data_property_value3540, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3540.clone()
                    }
                }).borrow().clone(); ml_reference_value697 }, &match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, source_ml_source1_data_property_value3541, _, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3541.clone()
                    }
                }, 0f64, false, |a, b| a == b))) || ((((((((((((((((((((((((((((((((!(self::test_java_script_standard_primitive_method_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3542, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3542.clone()
                    }
                }))) && !(self::test_java_script_standard_array_intrinsic_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3543, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3543.clone()
                    }
                })))) && !(self::test_java_script_standard_array_callback_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3544, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3544.clone()
                    }
                })))) && !(self::test_java_script_array_search_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3545, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3545.clone()
                    }
                })))) && !(self::test_java_script_standard_array_species_getter_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3546, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3546.clone()
                    }
                })))) && !(self::test_java_script_standard_array_transform_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3547, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3547.clone()
                    }
                })))) && !(self::test_java_script_array_mutation_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3548, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3548.clone()
                    }
                })))) && !(self::test_java_script_standard_object_enumeration_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3549, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3549.clone()
                    }
                })))) && !(self::test_java_script_standard_object_observation_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3550, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3550.clone()
                    }
                })))) && !(self::test_java_script_object_definition_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3551, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3551.clone()
                    }
                })))) && ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3553, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3553.clone()
                    }
                }) != (self::ml_string::Text::from_units(vec![70, 117, 110, 99, 116, 105, 111, 110, 46, 104, 97, 115, 73, 110, 115, 116, 97, 110, 99, 101])))) && ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3555, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3555.clone()
                    }
                }) != (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 105, 115, 80, 114, 111, 116, 111, 116, 121, 112, 101, 79, 102])))) && !(self::test_java_script_standard_property_reflection_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3556, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3556.clone()
                    }
                })))) && !(self::test_java_script_standard_property_definitions_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3557, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3557.clone()
                    }
                })))) && !(self::test_java_script_object_assignment_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3558, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3558.clone()
                    }
                })))) && !(self::test_java_script_function_invocation_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3559, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3559.clone()
                    }
                })))) && !(self::test_java_script_string_method_operation(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3560, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3560.clone()
                    }
                }))) && !(self::test_java_script_numeric_intrinsic_operation(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3561, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3561.clone()
                    }
                }))) && !(self::test_java_script_string_conversion_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3562, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3562.clone()
                    }
                })))) && !(self::test_java_script_symbol_registry_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3563, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3563.clone()
                    }
                })))) && !(self::test_java_script_standard_collection_for_each_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3564, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3564.clone()
                    }
                })))) && ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3566, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3566.clone()
                    }
                }) != (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116])))) && ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3568, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3568.clone()
                    }
                }) != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121])))) && ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3570, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3570.clone()
                    }
                }) != (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 116, 111, 83, 116, 114, 105, 110, 103])))) && ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3572, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3572.clone()
                    }
                }) != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 116, 111, 83, 116, 114, 105, 110, 103])))) && ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3574, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3574.clone()
                    }
                }) != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 115, 108, 105, 99, 101])))) && ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3576, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3576.clone()
                    }
                }) != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 99, 111, 110, 99, 97, 116])))) && !(self::test_java_script_array_sort_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3577, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3577.clone()
                    }
                })))) && !(self::test_java_script_array_copy_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3578, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3578.clone()
                    }
                })))) && !(self::test_java_script_array_join_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3579, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3579.clone()
                    }
                })))) && !(self::test_java_script_standard_object_from_entries_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3580, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3580.clone()
                    }
                })))) && ((match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3582, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3582.clone()
                    }
                }) != (self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 99, 111, 110, 115, 116, 114, 117, 99, 116])))) && !(self::test_java_script_standard_array_creation_operation(&(match method.clone() {
                    self::JavaScriptPrimitiveMethodOwner::PrimitiveMethodOwner(_, _, source_ml_source1_data_property_value3583, _, _, _, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3583.clone()
                    }
                }))))) {
                    self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))))
                } else {
                    {
                        let creation = self::compile_java_script_collection_creation_method(method.clone(), identities.clone(), functions.clone());
                        if ((match creation.clone() {
                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(_, source_ml_source1_data_property_value3585) => {
                                source_ml_source1_data_property_value3585.clone()
                            }
                        }) != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), match creation.clone() {
                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(_, source_ml_source1_data_property_value3586) => {
                                    source_ml_source1_data_property_value3586.clone()
                                }
                            })))
                        } else {
                            if match creation.clone() {
                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(source_ml_source1_data_property_value3587, _) => {
                                    source_ml_source1_data_property_value3587
                                }
                            } {
                                {
                                    let source_ml_temporary_index523_2 = source_ml_temporary_index523.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index523 = source_ml_temporary_index523_2.clone();
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    let array = self::compile_java_script_collection_array_method(method.clone(), identities.clone(), functions.clone());
                                    if ((match array.clone() {
                                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(_, source_ml_source1_data_property_value3589) => {
                                            source_ml_source1_data_property_value3589.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![]))) {
                                        self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), match array.clone() {
                                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(_, source_ml_source1_data_property_value3590) => {
                                                source_ml_source1_data_property_value3590.clone()
                                            }
                                        })))
                                    } else {
                                        if match array.clone() {
                                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(source_ml_source1_data_property_value3591, _) => {
                                                source_ml_source1_data_property_value3591
                                            }
                                        } {
                                            {
                                                let source_ml_temporary_index523_3 = source_ml_temporary_index523.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index523 = source_ml_temporary_index523_3.clone();
                                                    continue;
                                                }
                                            }
                                        } else {
                                            {
                                                let protocol = self::compile_java_script_collection_protocol_method(method.clone(), identities.clone(), functions.clone());
                                                if ((match protocol.clone() {
                                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(_, source_ml_source1_data_property_value3593) => {
                                                        source_ml_source1_data_property_value3593.clone()
                                                    }
                                                }) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), match protocol.clone() {
                                                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(_, source_ml_source1_data_property_value3594) => {
                                                            source_ml_source1_data_property_value3594.clone()
                                                        }
                                                    })))
                                                } else {
                                                    if match protocol.clone() {
                                                        self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(source_ml_source1_data_property_value3595, _) => {
                                                            source_ml_source1_data_property_value3595
                                                        }
                                                    } {
                                                        {
                                                            let source_ml_temporary_index523_4 = source_ml_temporary_index523.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                source_ml_temporary_index523 = source_ml_temporary_index523_4.clone();
                                                                continue;
                                                            }
                                                        }
                                                    } else {
                                                        {
                                                            let object = self::compile_java_script_collection_object_method(method.clone(), identities.clone(), functions.clone());
                                                            if ((match object.clone() {
                                                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(_, source_ml_source1_data_property_value3597) => {
                                                                    source_ml_source1_data_property_value3597.clone()
                                                                }
                                                            }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), match object.clone() {
                                                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(_, source_ml_source1_data_property_value3598) => {
                                                                        source_ml_source1_data_property_value3598.clone()
                                                                    }
                                                                })))
                                                            } else {
                                                                if match object.clone() {
                                                                    self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(source_ml_source1_data_property_value3599, _) => {
                                                                        source_ml_source1_data_property_value3599
                                                                    }
                                                                } {
                                                                    {
                                                                        let source_ml_temporary_index523_5 = source_ml_temporary_index523.add(&self::ml::Big::from_i128(1));
                                                                        {
                                                                            source_ml_temporary_index523 = source_ml_temporary_index523_5.clone();
                                                                            continue;
                                                                        }
                                                                    }
                                                                } else {
                                                                    {
                                                                        let dispatch = self::compile_java_script_collection_dispatch_method(method.clone(), identities.clone(), functions.clone());
                                                                        if ((match dispatch.clone() {
                                                                            self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(_, source_ml_source1_data_property_value3601) => {
                                                                                source_ml_source1_data_property_value3601.clone()
                                                                            }
                                                                        }) != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), match dispatch.clone() {
                                                                                self::JavaScriptCollectionMethodCompilation::CollectionMethodCompilation(_, source_ml_source1_data_property_value3602) => {
                                                                                    source_ml_source1_data_property_value3602.clone()
                                                                                }
                                                                            })))
                                                                        } else {
                                                                            {
                                                                                let source_ml_temporary_index523_6 = source_ml_temporary_index523.add(&self::ml::Big::from_i128(1));
                                                                                {
                                                                                    source_ml_temporary_index523 = source_ml_temporary_index523_6.clone();
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
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Result {
    MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Done,
    MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Return(Box<self::JavaScriptCollectionCallCompilation>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Result {
    MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Done,
    MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Return(Box<self::JavaScriptCollectionCallCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_collection_call_directory_loop533(mut empty: self::JavaScriptCallProgram, mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut owner: self::JavaScriptCollectionConstructorOwner, mut source_ml_temporary_values522: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionMethodOwner>>>, mut source_ml_temporary_index522: self::ml::Big) -> self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Result {
    loop {
        return if (source_ml_temporary_index522 < ({ let ml_reference_array698 = source_ml_temporary_values522.clone(); let ml_reference_projection698 = self::ml::Big::from_u128(ml_reference_array698.borrow().len() as u128); ml_reference_projection698 })) {
            {
                let method = { let ml_reference_array699 = source_ml_temporary_values522.clone(); let ml_reference_position699 = source_ml_temporary_index522.clone(); let ml_reference_projection699 = self::ml_array::at(&*ml_reference_array699.borrow(), ml_reference_position699.to_index()); ml_reference_projection699 };
                if (((((match method.clone() {
                    self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(source_ml_source1_data_property_value3606, _) => {
                        source_ml_source1_data_property_value3606.clone()
                    }
                }) <= (self::ml::Big::from_i128(0))) || ((match method.clone() {
                    self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(source_ml_source1_data_property_value3608, _) => {
                        source_ml_source1_data_property_value3608.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) || self::ml_array::includes(&{ let ml_reference_value700 = identities.borrow().clone(); ml_reference_value700 }, &match method.clone() {
                    self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(source_ml_source1_data_property_value3609, _) => {
                        source_ml_source1_data_property_value3609.clone()
                    }
                }, 0f64, false, |a, b| a == b)) || ((self::read_java_script_collection_operation_kind(&(match method.clone() {
                    self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(_, source_ml_source1_data_property_value3612) => {
                        source_ml_source1_data_property_value3612.clone()
                    }
                }))) != (match owner.clone() {
                    self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(source_ml_source1_data_property_value3613, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3613.clone()
                    }
                }))) {
                    self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114, 115]))))
                } else {
                    if ((self::dependency_function2(&(match method.clone() {
                        self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(_, source_ml_source1_data_property_value3615) => {
                            source_ml_source1_data_property_value3615.clone()
                        }
                    }))) != (self::ml_string::Text::from_units(vec![]))) {
                        {
                            let fn_ = self::compile_java_script_weak_collection_function(match method.clone() {
                                self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(source_ml_source1_data_property_value3616, _) => {
                                    source_ml_source1_data_property_value3616.clone()
                                }
                            }, match owner.clone() {
                                self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, source_ml_source1_data_property_value3617, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value3617.clone()
                                }
                            }, &(match method.clone() {
                                self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(_, source_ml_source1_data_property_value3618) => {
                                    source_ml_source1_data_property_value3618.clone()
                                }
                            }));
                            if !(self::append_java_script_compiled_collection_call_function(functions.clone(), fn_.clone())) {
                                self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 111, 119, 110, 101, 114, 115]))))
                            } else {
                                {
                                    let source_ml_temporary_compile_java_script_collection_call_directory_ignored534 = { let ml_method_owner701 = identities.clone(); let ml_method_argument701_0 = match method.clone() {
                                        self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(source_ml_source1_data_property_value3619, _) => {
                                            source_ml_source1_data_property_value3619.clone()
                                        }
                                    }; let ml_inserted = vec![ml_method_argument701_0]; let mut ml_method_values701 = ml_method_owner701.borrow_mut(); let ml_length = ml_method_values701.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values701.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let source_ml_temporary_index522_2 = source_ml_temporary_index522.add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_temporary_index522 = source_ml_temporary_index522_2.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_temporary_compile_java_script_collection_call_directory_ignored535 = { let ml_method_owner702 = identities.clone(); let ml_method_argument702_0 = match method.clone() {
                                self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(source_ml_source1_data_property_value3620, _) => {
                                    source_ml_source1_data_property_value3620.clone()
                                }
                            }; let ml_inserted = vec![ml_method_argument702_0]; let mut ml_method_values702 = ml_method_owner702.borrow_mut(); let ml_length = ml_method_values702.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values702.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_temporary_compile_java_script_collection_call_directory_ignored536 = { let ml_method_owner703 = functions.clone(); let ml_method_argument703_0 = {
                                    let source_ml_source1_constructor_field3622 = match method.clone() {
                                        self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(source_ml_source1_data_property_value3621, _) => {
                                            source_ml_source1_data_property_value3621.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source1_constructor_field3624 = match owner.clone() {
                                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, source_ml_source1_data_property_value3623, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value3623.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source1_constructor_field3626 = self::JavaScriptSemanticValue::Undefined;
                                            {
                                                let source_ml_source1_constructor_field3627 = 0f64;
                                                {
                                                    let source_ml_source1_constructor_field3628 = 2f64;
                                                    {
                                                        let source_ml_source1_constructor_field3629 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                        {
                                                            let source_ml_source1_constructor_field3631 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::CollectionOperation(1f64, match method.clone() {
                                                                self::JavaScriptCollectionMethodOwner::CollectionMethodOwner(_, source_ml_source1_data_property_value3630) => {
                                                                    source_ml_source1_data_property_value3630.clone()
                                                                }
                                                            }), self::JavaScriptCallInstruction::Return(1f64)]));
                                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field3622.clone(), source_ml_source1_constructor_field3624.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field3626.clone()), source_ml_source1_constructor_field3627, source_ml_source1_constructor_field3628, source_ml_source1_constructor_field3629.clone(), source_ml_source1_constructor_field3631.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }; let ml_inserted = vec![ml_method_argument703_0]; let mut ml_method_values703 = ml_method_owner703.borrow_mut(); let ml_length = ml_method_values703.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values703.extend(ml_inserted); ml_length as f64 };
                                {
                                    let source_ml_temporary_index522_3 = source_ml_temporary_index522.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index522 = source_ml_temporary_index522_3.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_collection_call_directory_loop525(mut directory: self::JavaScriptCollectionCallableDirectory, mut empty: self::JavaScriptCallProgram, mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut source_ml_temporary_values521: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionConstructorOwner>>>, mut source_ml_temporary_index521: self::ml::Big) -> self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Result {
    loop {
        return if (source_ml_temporary_index521 < ({ let ml_reference_array704 = source_ml_temporary_values521.clone(); let ml_reference_projection704 = self::ml::Big::from_u128(ml_reference_array704.borrow().len() as u128); ml_reference_projection704 })) {
            {
                let owner = { let ml_reference_array705 = source_ml_temporary_values521.clone(); let ml_reference_position705 = source_ml_temporary_index521.clone(); let ml_reference_projection705 = self::ml_array::at(&*ml_reference_array705.borrow(), ml_reference_position705.to_index()); ml_reference_projection705 };
                if {
                    let source_ml_source1_logical3660 = {
                        let source_ml_source1_logical3657 = (((((!(self::ml_array::includes(&{ let ml_reference_value706 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])]))).borrow().clone(); ml_reference_value706 }, &match owner.clone() {
                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(source_ml_source1_data_property_value3634, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3634.clone()
                            }
                        }, 0f64, false, |a, b| a == b)) || ((match owner.clone() {
                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, source_ml_source1_data_property_value3636, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3636.clone()
                            }
                        }) <= (self::ml::Big::from_i128(0)))) || ((match owner.clone() {
                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, source_ml_source1_data_property_value3638, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3638.clone()
                            }
                        }) > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::ml_array::includes(&{ let ml_reference_value707 = (match directory.clone() {
                            self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(source_ml_source1_data_property_value3640, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3640.clone()
                            }
                        }).borrow().clone(); ml_reference_value707 }, &match owner.clone() {
                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, source_ml_source1_data_property_value3641, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3641.clone()
                            }
                        }, 0f64, false, |a, b| a == b))) || self::ml_array::includes(&{ let ml_reference_value708 = identities.borrow().clone(); ml_reference_value708 }, &match owner.clone() {
                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, source_ml_source1_data_property_value3642, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3642.clone()
                            }
                        }, 0f64, false, |a, b| a == b)) || {
                            let source_ml_source1_s3644 = match owner.clone() {
                                self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, _, source_ml_source1_data_property_value3654, _, _, _, _, _, _) => {
                                    let source_ml_source1_data_property_value3654 = *source_ml_source1_data_property_value3654;
                                    source_ml_source1_data_property_value3654.clone()
                                }
                            };
                            match source_ml_source1_s3644.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    false
                                }
                                _ => {
                                    true
                                }
                            }
                        });
                        if source_ml_source1_logical3657 {
                            Some(source_ml_source1_logical3657)
                        } else {
                            match match owner.clone() {
                                self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, _, source_ml_source1_data_property_value3655, _, _, _, _, _, _) => {
                                    let source_ml_source1_data_property_value3655 = *source_ml_source1_data_property_value3655;
                                    source_ml_source1_data_property_value3655.clone()
                                }
                            } {
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
                                self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3656) => {
                                    Some(source_ml_source1_data_property_value3656)
                                }
                            }
                        }
                    };
                    if match source_ml_source1_logical3660.clone() { Some(ml_present) => ml_present, None => false } {
                        source_ml_source1_logical3660.clone().expect("a proven present value")
                    } else {
                        (({ let ml_reference_array709 = match owner.clone() {
                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value3659) => {
                                source_ml_source1_data_property_value3659.clone()
                            }
                        }; let ml_reference_projection709 = ml_reference_array709.borrow().len() as f64; ml_reference_projection709 }) > (16f64))
                    }
                } {
                    self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 105, 113, 117, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 44, 32, 114, 101, 97, 108, 109, 44, 32, 107, 105, 110, 100, 32, 97, 110, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 111, 119, 110, 101, 114, 115]))))
                } else {
                    {
                        let source_ml_temporary_compile_java_script_collection_call_directory_ignored531 = { let ml_method_owner710 = identities.clone(); let ml_method_argument710_0 = match owner.clone() {
                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, source_ml_source1_data_property_value3661, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3661.clone()
                            }
                        }; let ml_inserted = vec![ml_method_argument710_0]; let mut ml_method_values710 = ml_method_owner710.borrow_mut(); let ml_length = ml_method_values710.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values710.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_temporary_compile_java_script_collection_call_directory_ignored532 = { let ml_method_owner711 = functions.clone(); let ml_method_argument711_0 = self::compile_java_script_iterable_collection_constructor(owner.clone()); let ml_inserted = vec![ml_method_argument711_0]; let mut ml_method_values711 = ml_method_owner711.borrow_mut(); let ml_length = ml_method_values711.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values711.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_temporary_values522 = match owner.clone() {
                                    self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, _, _, _, _, _, _, _, source_ml_source1_data_property_value3662) => {
                                        source_ml_source1_data_property_value3662.clone()
                                    }
                                };
                                {
                                    let source_ml_temporary_index522 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s3663 = self::source_ml_temporary_compile_java_script_collection_call_directory_loop533(empty.clone(), identities.clone(), functions.clone(), owner.clone(), source_ml_temporary_values522.clone(), source_ml_temporary_index522.clone());
                                        match source_ml_source1_s3663.clone() {
                                            self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Done => {
                                                {
                                                    let source_ml_temporary_index521_2 = source_ml_temporary_index521.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index521 = source_ml_temporary_index521_2.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                            self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop533Return(source_ml_temporary_result533) => {
                                                let source_ml_temporary_result533 = *source_ml_temporary_result533;
                                                self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Return(Box::new(source_ml_temporary_result533.clone()))
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
            self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop525Done
        };
    }
}
