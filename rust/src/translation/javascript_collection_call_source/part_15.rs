// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCollectionIteratorCallCompilation;
use super::JavaScriptCollectionIteratorCallableDirectory;
use super::JavaScriptCollectionIteratorMethodOwner;
use super::JavaScriptSemanticValue;
use super::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_collection_iterator_operation_kind;
use super::read_java_script_collection_iterator_operation_strategy;
use super::test_java_script_collection_iterator_method_alias;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop445(mut directory: self::JavaScriptCollectionIteratorCallableDirectory, mut empty: self::JavaScriptCallProgram, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut owners: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionIteratorMethodOwner>>>, mut source_ml_temporary_values443: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionIteratorMethodOwner>>>, mut source_ml_temporary_index443: self::ml::Big) -> self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result {
    loop {
        return if (source_ml_temporary_index443 < ({ let ml_reference_array669 = source_ml_temporary_values443.clone(); let ml_reference_projection669 = self::ml::Big::from_u128(ml_reference_array669.borrow().len() as u128); ml_reference_projection669 })) {
            {
                let owner = { let ml_reference_array670 = source_ml_temporary_values443.clone(); let ml_reference_position670 = source_ml_temporary_index443.clone(); let ml_reference_projection670 = self::ml_array::at(&*ml_reference_array670.borrow(), ml_reference_position670.to_index()); ml_reference_projection670 };
                {
                    let kind = self::read_java_script_collection_iterator_operation_kind(&(match owner.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, source_ml_source1_data_property_value3123, _, _, _) => {
                            source_ml_source1_data_property_value3123.clone()
                        }
                    }));
                    {
                        let strategy = self::read_java_script_collection_iterator_operation_strategy(&(match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, source_ml_source1_data_property_value3124, _, _, _) => {
                                source_ml_source1_data_property_value3124.clone()
                            }
                        }));
                        if (((((kind == (self::ml_string::Text::from_units(vec![]))) || (({
                            let source_ml_source1_type_query_operand3128 = match owner.clone() {
                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3127, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value3127.clone()
                                }
                            };
                            self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                        }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])))) || ((match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3130, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3130.clone()
                            }
                        }) <= (self::ml::Big::from_i128(0)))) || ((match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3132, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3132.clone()
                            }
                        }) > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::ml_array::includes(&{ let ml_reference_value671 = (match directory.clone() {
                            self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(source_ml_source1_data_property_value3134, _, _) => {
                                source_ml_source1_data_property_value3134.clone()
                            }
                        }).borrow().clone(); ml_reference_value671 }, &match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value3135, _, _, _, _) => {
                                source_ml_source1_data_property_value3135.clone()
                            }
                        }, 0f64, false, |a, b| a == b))) {
                            self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 111, 108, 101]))))
                        } else {
                            {
                                let prior = { let ml_values672 = { let ml_reference_value673 = owners.borrow().clone(); ml_reference_value673 }; let ml_callback672 = { let owner = owner.clone(); std::rc::Rc::new(move |value: self::JavaScriptCollectionIteratorMethodOwner| -> bool { ((match value.clone() {
                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3138, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value3138.clone()
                                    }
                                }) == (match owner.clone() {
                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3139, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value3139.clone()
                                    }
                                })) }) }; ml_values672.iter().cloned().enumerate().find_map(|(ml_index672, ml_value672)| if ml_callback672(ml_value672.clone()) { Some(ml_value672) } else { None }) };
                                if !(match (prior.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    if !(self::test_java_script_collection_iterator_method_alias(prior.clone().expect("a proven present value"), owner.clone())) {
                                        self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![111, 110, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 99, 97, 110, 110, 111, 116, 32, 97, 99, 113, 117, 105, 114, 101, 32, 99, 111, 110, 102, 108, 105, 99, 116, 105, 110, 103, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 115, 101, 109, 97, 110, 116, 105, 99, 115]))))
                                    } else {
                                        {
                                            let source_ml_temporary_index443_2 = source_ml_temporary_index443.add(&self::ml::Big::from_i128(1));
                                            {
                                                source_ml_temporary_index443 = source_ml_temporary_index443_2.clone();
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let owners_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source1_concat_value3141 = std::rc::Rc::new(std::cell::RefCell::new(vec![owner.clone()]));
                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCollectionIteratorMethodOwner>::new(), { let ml_reference_value674 = owners.borrow().clone(); ml_reference_value674 }), { let ml_reference_value675 = source_ml_source1_concat_value3141.borrow().clone(); ml_reference_value675 })
                                        }));
                                        if (kind == (self::ml_string::Text::from_units(vec![115, 101, 108, 102]))) {
                                            {
                                                let functions_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source1_concat_value3153 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                        let source_ml_source1_constructor_field3143 = match owner.clone() {
                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3142, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value3142.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_constructor_field3145 = match owner.clone() {
                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value3144, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value3144.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_constructor_field3147 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_constructor_field3148 = 0f64;
                                                                    {
                                                                        let source_ml_source1_constructor_field3149 = 2f64;
                                                                        {
                                                                            let source_ml_source1_constructor_field3150 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            {
                                                                                let source_ml_source1_constructor_field3151 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Copy(1f64, 0f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                                                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field3143.clone(), source_ml_source1_constructor_field3145.clone(), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), Box::new(source_ml_source1_constructor_field3147.clone()), source_ml_source1_constructor_field3148, source_ml_source1_constructor_field3149, source_ml_source1_constructor_field3150.clone(), source_ml_source1_constructor_field3151.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }]));
                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), { let ml_reference_value676 = functions.borrow().clone(); ml_reference_value676 }), { let ml_reference_value677 = source_ml_source1_concat_value3153.borrow().clone(); ml_reference_value677 })
                                                }));
                                                {
                                                    let source_ml_temporary_index443_3 = source_ml_temporary_index443.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (functions, owners, source_ml_temporary_index443) = (functions_2.clone(), owners_2.clone(), source_ml_temporary_index443_3.clone());
                                                        continue;
                                                    }
                                                }
                                            }
                                        } else {
                                            if ((kind == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))) && (strategy == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115])))) {
                                                if match {
                                                    let source_ml_source1_logical3168 = {
                                                        let source_ml_source1_s3155 = match owner.clone() {
                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3165, _, _) => {
                                                                let source_ml_source1_data_property_value3165 = *source_ml_source1_data_property_value3165;
                                                                source_ml_source1_data_property_value3165.clone()
                                                            }
                                                        };
                                                        match source_ml_source1_s3155.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    };
                                                    if source_ml_source1_logical3168 {
                                                        Some(source_ml_source1_logical3168)
                                                    } else {
                                                        match match owner.clone() {
                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3166, _, _) => {
                                                                let source_ml_source1_data_property_value3166 = *source_ml_source1_data_property_value3166;
                                                                source_ml_source1_data_property_value3166.clone()
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
                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3167) => {
                                                                Some(source_ml_source1_data_property_value3167)
                                                            }
                                                        }
                                                    }
                                                } { Some(ml_present) => ml_present, None => false } {
                                                    self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))))
                                                } else {
                                                    {
                                                        let functions_3 = std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_concat_value3186 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                let source_ml_source1_constructor_field3170 = match owner.clone() {
                                                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3169, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value3169.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_constructor_field3172 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value3171, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value3171.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_constructor_field3174 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_constructor_field3175 = 0f64;
                                                                            {
                                                                                let source_ml_source1_constructor_field3176 = 4f64;
                                                                                {
                                                                                    let source_ml_source1_constructor_field3177 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                    {
                                                                                        let source_ml_source1_constructor_field3184 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 99, 111, 101, 114, 99, 105, 98, 108, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64]))), self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64]))), self::JavaScriptCallInstruction::Constant(2f64, Box::new(match owner.clone() {
                                                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3178, _, _) => {
                                                                                                let source_ml_source1_data_property_value3178 = *source_ml_source1_data_property_value3178;
                                                                                                source_ml_source1_data_property_value3178.clone()
                                                                                            }
                                                                                        })), {
                                                                                            let source_ml_source1_constructor_field3179 = 3f64;
                                                                                            {
                                                                                                let source_ml_source1_constructor_field3180 = 2f64;
                                                                                                {
                                                                                                    let source_ml_source1_constructor_field3183 = Some(1f64);
                                                                                                    self::JavaScriptCallInstruction::CollectionIteratorAllocate(source_ml_source1_constructor_field3179, source_ml_source1_constructor_field3180, self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), Some((source_ml_source1_constructor_field3183.clone(),)))
                                                                                                }
                                                                                            }
                                                                                        }, self::JavaScriptCallInstruction::Return(3f64)]));
                                                                                        self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field3170.clone(), source_ml_source1_constructor_field3172.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field3174.clone()), source_ml_source1_constructor_field3175, source_ml_source1_constructor_field3176, source_ml_source1_constructor_field3177.clone(), source_ml_source1_constructor_field3184.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }]));
                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), { let ml_reference_value678 = functions.borrow().clone(); ml_reference_value678 }), { let ml_reference_value679 = source_ml_source1_concat_value3186.borrow().clone(); ml_reference_value679 })
                                                        }));
                                                        {
                                                            let source_ml_temporary_index443_4 = source_ml_temporary_index443.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                (functions, owners, source_ml_temporary_index443) = (functions_3.clone(), owners_2.clone(), source_ml_temporary_index443_4.clone());
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (strategy != (self::ml_string::Text::from_units(vec![]))) {
                                                    if match {
                                                        let source_ml_source1_logical3201 = {
                                                            let source_ml_source1_s3188 = match owner.clone() {
                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3198, _, _) => {
                                                                    let source_ml_source1_data_property_value3198 = *source_ml_source1_data_property_value3198;
                                                                    source_ml_source1_data_property_value3198.clone()
                                                                }
                                                            };
                                                            match source_ml_source1_s3188.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                        };
                                                        if source_ml_source1_logical3201 {
                                                            Some(source_ml_source1_logical3201)
                                                        } else {
                                                            match match owner.clone() {
                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3199, _, _) => {
                                                                    let source_ml_source1_data_property_value3199 = *source_ml_source1_data_property_value3199;
                                                                    source_ml_source1_data_property_value3199.clone()
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
                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3200) => {
                                                                    Some(source_ml_source1_data_property_value3200)
                                                                }
                                                            }
                                                        }
                                                    } { Some(ml_present) => ml_present, None => false } {
                                                        self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))))
                                                    } else {
                                                        {
                                                            let functions_4 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source1_concat_value3218 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                    let source_ml_source1_constructor_field3203 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3202, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value3202.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_constructor_field3205 = match owner.clone() {
                                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value3204, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value3204.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_constructor_field3207 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source1_constructor_field3208 = 0f64;
                                                                                {
                                                                                    let source_ml_source1_constructor_field3209 = 3f64;
                                                                                    {
                                                                                        let source_ml_source1_constructor_field3210 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                        {
                                                                                            let source_ml_source1_constructor_field3216 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Constant(1f64, Box::new(match owner.clone() {
                                                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3211, _, _) => {
                                                                                                    let source_ml_source1_data_property_value3211 = *source_ml_source1_data_property_value3211;
                                                                                                    source_ml_source1_data_property_value3211.clone()
                                                                                                }
                                                                                            })), {
                                                                                                let source_ml_source1_constructor_field3212 = 2f64;
                                                                                                {
                                                                                                    let source_ml_source1_constructor_field3213 = 1f64;
                                                                                                    self::JavaScriptCallInstruction::CollectionIteratorAllocate(source_ml_source1_constructor_field3212, source_ml_source1_constructor_field3213, kind.clone(), strategy.clone(), None::<(Option<f64>,)>)
                                                                                                }
                                                                                            }, self::JavaScriptCallInstruction::Return(2f64)]));
                                                                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field3203.clone(), source_ml_source1_constructor_field3205.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field3207.clone()), source_ml_source1_constructor_field3208, source_ml_source1_constructor_field3209, source_ml_source1_constructor_field3210.clone(), source_ml_source1_constructor_field3216.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }]));
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), { let ml_reference_value680 = functions.borrow().clone(); ml_reference_value680 }), { let ml_reference_value681 = source_ml_source1_concat_value3218.borrow().clone(); ml_reference_value681 })
                                                            }));
                                                            {
                                                                let source_ml_temporary_index443_5 = source_ml_temporary_index443.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    (functions, owners, source_ml_temporary_index443) = (functions_4.clone(), owners_2.clone(), source_ml_temporary_index443_5.clone());
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if match {
                                                        let source_ml_source1_logical3249 = {
                                                            let source_ml_source1_logical3246 = {
                                                                let source_ml_source1_logical3233 = {
                                                                    let source_ml_source1_s3220 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value3230, _) => {
                                                                            let source_ml_source1_data_property_value3230 = *source_ml_source1_data_property_value3230;
                                                                            source_ml_source1_data_property_value3230.clone()
                                                                        }
                                                                    };
                                                                    match source_ml_source1_s3220.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }
                                                                };
                                                                if source_ml_source1_logical3233 {
                                                                    Some(source_ml_source1_logical3233)
                                                                } else {
                                                                    match match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value3231, _) => {
                                                                            let source_ml_source1_data_property_value3231 = *source_ml_source1_data_property_value3231;
                                                                            source_ml_source1_data_property_value3231.clone()
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
                                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3232) => {
                                                                            Some(source_ml_source1_data_property_value3232)
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            if match source_ml_source1_logical3246.clone() { Some(ml_present) => ml_present, None => false } {
                                                                source_ml_source1_logical3246.clone().expect("a proven present value")
                                                            } else {
                                                                {
                                                                    let source_ml_source1_s3235 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value3245) => {
                                                                            let source_ml_source1_data_property_value3245 = *source_ml_source1_data_property_value3245;
                                                                            source_ml_source1_data_property_value3245.clone()
                                                                        }
                                                                    };
                                                                    match source_ml_source1_s3235.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            false
                                                                        }
                                                                        _ => {
                                                                            true
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        if source_ml_source1_logical3249 {
                                                            Some(source_ml_source1_logical3249)
                                                        } else {
                                                            match match owner.clone() {
                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value3247) => {
                                                                    let source_ml_source1_data_property_value3247 = *source_ml_source1_data_property_value3247;
                                                                    source_ml_source1_data_property_value3247.clone()
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
                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3248) => {
                                                                    Some(source_ml_source1_data_property_value3248)
                                                                }
                                                            }
                                                        }
                                                    } { Some(ml_present) => ml_present, None => false } {
                                                        self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115]))))
                                                    } else {
                                                        {
                                                            let functions_5 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source1_concat_value3263 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                    let source_ml_source1_constructor_field3251 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3250, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value3250.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_constructor_field3253 = match owner.clone() {
                                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value3252, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value3252.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_constructor_field3255 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source1_constructor_field3256 = 0f64;
                                                                                {
                                                                                    let source_ml_source1_constructor_field3257 = 4f64;
                                                                                    {
                                                                                        let source_ml_source1_constructor_field3258 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                        {
                                                                                            let source_ml_source1_constructor_field3261 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Constant(1f64, Box::new(match owner.clone() {
                                                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value3259, _) => {
                                                                                                    let source_ml_source1_data_property_value3259 = *source_ml_source1_data_property_value3259;
                                                                                                    source_ml_source1_data_property_value3259.clone()
                                                                                                }
                                                                                            })), self::JavaScriptCallInstruction::Constant(2f64, Box::new(match owner.clone() {
                                                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value3260) => {
                                                                                                    let source_ml_source1_data_property_value3260 = *source_ml_source1_data_property_value3260;
                                                                                                    source_ml_source1_data_property_value3260.clone()
                                                                                                }
                                                                                            })), self::JavaScriptCallInstruction::CollectionIteratorResult(3f64, kind.clone(), 1f64, 2f64), self::JavaScriptCallInstruction::Return(3f64)]));
                                                                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field3251.clone(), source_ml_source1_constructor_field3253.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field3255.clone()), source_ml_source1_constructor_field3256, source_ml_source1_constructor_field3257, source_ml_source1_constructor_field3258.clone(), source_ml_source1_constructor_field3261.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }]));
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), { let ml_reference_value682 = functions.borrow().clone(); ml_reference_value682 }), { let ml_reference_value683 = source_ml_source1_concat_value3263.borrow().clone(); ml_reference_value683 })
                                                            }));
                                                            {
                                                                let source_ml_temporary_index443_6 = source_ml_temporary_index443.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    (functions, owners, source_ml_temporary_index443) = (functions_5.clone(), owners_2.clone(), source_ml_temporary_index443_6.clone());
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
        } else {
            self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Done(functions.clone(), owners.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Result {
    MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Done(self::ml_string::Text),
    MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Return(self::ml_string::Text),
}
