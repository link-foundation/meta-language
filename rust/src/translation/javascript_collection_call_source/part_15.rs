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
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, source_ml_source1_data_property_value3108, _, _, _) => {
                            source_ml_source1_data_property_value3108.clone()
                        }
                    }));
                    {
                        let strategy = self::read_java_script_collection_iterator_operation_strategy(&(match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, source_ml_source1_data_property_value3109, _, _, _) => {
                                source_ml_source1_data_property_value3109.clone()
                            }
                        }));
                        if (((((kind == (self::ml_string::Text::from_units(vec![]))) || (({
                            let source_ml_source1_type_query_operand3113 = match owner.clone() {
                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3112, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value3112.clone()
                                }
                            };
                            self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])
                        }) != (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116])))) || ((match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3115, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3115.clone()
                            }
                        }) <= (self::ml::Big::from_i128(0)))) || ((match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3117, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3117.clone()
                            }
                        }) > (self::ml::Big::from_i128(18446744073709551615)))) || !(self::ml_array::includes(&{ let ml_reference_value671 = (match directory.clone() {
                            self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(source_ml_source1_data_property_value3119, _, _) => {
                                source_ml_source1_data_property_value3119.clone()
                            }
                        }).borrow().clone(); ml_reference_value671 }, &match owner.clone() {
                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value3120, _, _, _, _) => {
                                source_ml_source1_data_property_value3120.clone()
                            }
                        }, 0f64, false, |a, b| a == b))) {
                            self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 44, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 111, 108, 101]))))
                        } else {
                            {
                                let prior = { let ml_values672 = { let ml_reference_value673 = owners.borrow().clone(); ml_reference_value673 }; let ml_callback672 = { let owner = owner.clone(); std::rc::Rc::new(move |value: self::JavaScriptCollectionIteratorMethodOwner| -> bool { ((match value.clone() {
                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3123, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value3123.clone()
                                    }
                                }) == (match owner.clone() {
                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3124, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value3124.clone()
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
                                            let source_ml_source1_concat_value3126 = std::rc::Rc::new(std::cell::RefCell::new(vec![owner.clone()]));
                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCollectionIteratorMethodOwner>::new(), { let ml_reference_value674 = owners.borrow().clone(); ml_reference_value674 }), { let ml_reference_value675 = source_ml_source1_concat_value3126.borrow().clone(); ml_reference_value675 })
                                        }));
                                        if (kind == (self::ml_string::Text::from_units(vec![115, 101, 108, 102]))) {
                                            {
                                                let functions_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                                    let source_ml_source1_concat_value3138 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                        let source_ml_source1_constructor_field3128 = match owner.clone() {
                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3127, _, _, _, _, _) => {
                                                                source_ml_source1_data_property_value3127.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_constructor_field3130 = match owner.clone() {
                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value3129, _, _, _, _) => {
                                                                    source_ml_source1_data_property_value3129.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_constructor_field3132 = self::JavaScriptSemanticValue::Undefined;
                                                                {
                                                                    let source_ml_source1_constructor_field3133 = 0f64;
                                                                    {
                                                                        let source_ml_source1_constructor_field3134 = 2f64;
                                                                        {
                                                                            let source_ml_source1_constructor_field3135 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                            {
                                                                                let source_ml_source1_constructor_field3136 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Copy(1f64, 0f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                                                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field3128.clone(), source_ml_source1_constructor_field3130.clone(), self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]), Box::new(source_ml_source1_constructor_field3132.clone()), source_ml_source1_constructor_field3133, source_ml_source1_constructor_field3134, source_ml_source1_constructor_field3135.clone(), source_ml_source1_constructor_field3136.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }]));
                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), { let ml_reference_value676 = functions.borrow().clone(); ml_reference_value676 }), { let ml_reference_value677 = source_ml_source1_concat_value3138.borrow().clone(); ml_reference_value677 })
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
                                                if ({
                                                    let source_ml_source1_s3140 = match owner.clone() {
                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3150, _, _) => {
                                                            let source_ml_source1_data_property_value3150 = *source_ml_source1_data_property_value3150;
                                                            source_ml_source1_data_property_value3150.clone()
                                                        }
                                                    };
                                                    match source_ml_source1_s3140.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }
                                                } || ((match match owner.clone() {
                                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3151, _, _) => {
                                                        let source_ml_source1_data_property_value3151 = *source_ml_source1_data_property_value3151;
                                                        source_ml_source1_data_property_value3151.clone()
                                                    }
                                                } {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        None::<(bool,)>
                                                    }
                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3152) => {
                                                        Some((source_ml_source1_data_property_value3152,))
                                                    }
                                                }).clone().expect("a proven present value")).0) {
                                                    self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))))
                                                } else {
                                                    {
                                                        let functions_3 = std::rc::Rc::new(std::cell::RefCell::new({
                                                            let source_ml_source1_concat_value3170 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                let source_ml_source1_constructor_field3154 = match owner.clone() {
                                                                    self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3153, _, _, _, _, _) => {
                                                                        source_ml_source1_data_property_value3153.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_constructor_field3156 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value3155, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value3155.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_constructor_field3158 = self::JavaScriptSemanticValue::Undefined;
                                                                        {
                                                                            let source_ml_source1_constructor_field3159 = 0f64;
                                                                            {
                                                                                let source_ml_source1_constructor_field3160 = 4f64;
                                                                                {
                                                                                    let source_ml_source1_constructor_field3161 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                    {
                                                                                        let source_ml_source1_constructor_field3168 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 99, 111, 101, 114, 99, 105, 98, 108, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64]))), self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64]))), self::JavaScriptCallInstruction::Constant(2f64, Box::new(match owner.clone() {
                                                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3162, _, _) => {
                                                                                                let source_ml_source1_data_property_value3162 = *source_ml_source1_data_property_value3162;
                                                                                                source_ml_source1_data_property_value3162.clone()
                                                                                            }
                                                                                        })), {
                                                                                            let source_ml_source1_constructor_field3163 = 3f64;
                                                                                            {
                                                                                                let source_ml_source1_constructor_field3164 = 2f64;
                                                                                                {
                                                                                                    let source_ml_source1_constructor_field3167 = Some(1f64);
                                                                                                    self::JavaScriptCallInstruction::CollectionIteratorAllocate(source_ml_source1_constructor_field3163, source_ml_source1_constructor_field3164, self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 115]), Some((source_ml_source1_constructor_field3167.clone(),)))
                                                                                                }
                                                                                            }
                                                                                        }, self::JavaScriptCallInstruction::Return(3f64)]));
                                                                                        self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field3154.clone(), source_ml_source1_constructor_field3156.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field3158.clone()), source_ml_source1_constructor_field3159, source_ml_source1_constructor_field3160, source_ml_source1_constructor_field3161.clone(), source_ml_source1_constructor_field3168.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }]));
                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), { let ml_reference_value678 = functions.borrow().clone(); ml_reference_value678 }), { let ml_reference_value679 = source_ml_source1_concat_value3170.borrow().clone(); ml_reference_value679 })
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
                                                    if ({
                                                        let source_ml_source1_s3172 = match owner.clone() {
                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3182, _, _) => {
                                                                let source_ml_source1_data_property_value3182 = *source_ml_source1_data_property_value3182;
                                                                source_ml_source1_data_property_value3182.clone()
                                                            }
                                                        };
                                                        match source_ml_source1_s3172.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    } || ((match match owner.clone() {
                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3183, _, _) => {
                                                            let source_ml_source1_data_property_value3183 = *source_ml_source1_data_property_value3183;
                                                            source_ml_source1_data_property_value3183.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Null => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3184) => {
                                                            Some((source_ml_source1_data_property_value3184,))
                                                        }
                                                    }).clone().expect("a proven present value")).0) {
                                                        self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101]))))
                                                    } else {
                                                        {
                                                            let functions_4 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source1_concat_value3201 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                    let source_ml_source1_constructor_field3186 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3185, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value3185.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_constructor_field3188 = match owner.clone() {
                                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value3187, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value3187.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_constructor_field3190 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source1_constructor_field3191 = 0f64;
                                                                                {
                                                                                    let source_ml_source1_constructor_field3192 = 3f64;
                                                                                    {
                                                                                        let source_ml_source1_constructor_field3193 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                        {
                                                                                            let source_ml_source1_constructor_field3199 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Constant(1f64, Box::new(match owner.clone() {
                                                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value3194, _, _) => {
                                                                                                    let source_ml_source1_data_property_value3194 = *source_ml_source1_data_property_value3194;
                                                                                                    source_ml_source1_data_property_value3194.clone()
                                                                                                }
                                                                                            })), {
                                                                                                let source_ml_source1_constructor_field3195 = 2f64;
                                                                                                {
                                                                                                    let source_ml_source1_constructor_field3196 = 1f64;
                                                                                                    self::JavaScriptCallInstruction::CollectionIteratorAllocate(source_ml_source1_constructor_field3195, source_ml_source1_constructor_field3196, kind.clone(), strategy.clone(), None::<(Option<f64>,)>)
                                                                                                }
                                                                                            }, self::JavaScriptCallInstruction::Return(2f64)]));
                                                                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field3186.clone(), source_ml_source1_constructor_field3188.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field3190.clone()), source_ml_source1_constructor_field3191, source_ml_source1_constructor_field3192, source_ml_source1_constructor_field3193.clone(), source_ml_source1_constructor_field3199.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }]));
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), { let ml_reference_value680 = functions.borrow().clone(); ml_reference_value680 }), { let ml_reference_value681 = source_ml_source1_concat_value3201.borrow().clone(); ml_reference_value681 })
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
                                                    if ((({
                                                        let source_ml_source1_s3203 = match owner.clone() {
                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value3213, _) => {
                                                                let source_ml_source1_data_property_value3213 = *source_ml_source1_data_property_value3213;
                                                                source_ml_source1_data_property_value3213.clone()
                                                            }
                                                        };
                                                        match source_ml_source1_s3203.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    } || ((match match owner.clone() {
                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value3214, _) => {
                                                            let source_ml_source1_data_property_value3214 = *source_ml_source1_data_property_value3214;
                                                            source_ml_source1_data_property_value3214.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Null => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3215) => {
                                                            Some((source_ml_source1_data_property_value3215,))
                                                        }
                                                    }).clone().expect("a proven present value")).0) || {
                                                        let source_ml_source1_s3217 = match owner.clone() {
                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value3227) => {
                                                                let source_ml_source1_data_property_value3227 = *source_ml_source1_data_property_value3227;
                                                                source_ml_source1_data_property_value3227.clone()
                                                            }
                                                        };
                                                        match source_ml_source1_s3217.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) || ((match match owner.clone() {
                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value3228) => {
                                                            let source_ml_source1_data_property_value3228 = *source_ml_source1_data_property_value3228;
                                                            source_ml_source1_data_property_value3228.clone()
                                                        }
                                                    } {
                                                        self::JavaScriptSemanticValue::Undefined => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Null => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Number(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            None::<(bool,)>
                                                        }
                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value3229) => {
                                                            Some((source_ml_source1_data_property_value3229,))
                                                        }
                                                    }).clone().expect("a proven present value")).0) {
                                                        self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(Box::new(self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 110, 101, 120, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115]))))
                                                    } else {
                                                        {
                                                            let functions_5 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source1_concat_value3243 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                    let source_ml_source1_constructor_field3231 = match owner.clone() {
                                                                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value3230, _, _, _, _, _) => {
                                                                            source_ml_source1_data_property_value3230.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_constructor_field3233 = match owner.clone() {
                                                                            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value3232, _, _, _, _) => {
                                                                                source_ml_source1_data_property_value3232.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source1_constructor_field3235 = self::JavaScriptSemanticValue::Undefined;
                                                                            {
                                                                                let source_ml_source1_constructor_field3236 = 0f64;
                                                                                {
                                                                                    let source_ml_source1_constructor_field3237 = 4f64;
                                                                                    {
                                                                                        let source_ml_source1_constructor_field3238 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                        {
                                                                                            let source_ml_source1_constructor_field3241 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Constant(1f64, Box::new(match owner.clone() {
                                                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value3239, _) => {
                                                                                                    let source_ml_source1_data_property_value3239 = *source_ml_source1_data_property_value3239;
                                                                                                    source_ml_source1_data_property_value3239.clone()
                                                                                                }
                                                                                            })), self::JavaScriptCallInstruction::Constant(2f64, Box::new(match owner.clone() {
                                                                                                self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value3240) => {
                                                                                                    let source_ml_source1_data_property_value3240 = *source_ml_source1_data_property_value3240;
                                                                                                    source_ml_source1_data_property_value3240.clone()
                                                                                                }
                                                                                            })), self::JavaScriptCallInstruction::CollectionIteratorResult(3f64, kind.clone(), 1f64, 2f64), self::JavaScriptCallInstruction::Return(3f64)]));
                                                                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field3231.clone(), source_ml_source1_constructor_field3233.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field3235.clone()), source_ml_source1_constructor_field3236, source_ml_source1_constructor_field3237, source_ml_source1_constructor_field3238.clone(), source_ml_source1_constructor_field3241.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }]));
                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFunction>::new(), { let ml_reference_value682 = functions.borrow().clone(); ml_reference_value682 }), { let ml_reference_value683 = source_ml_source1_concat_value3243.borrow().clone(); ml_reference_value683 })
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
