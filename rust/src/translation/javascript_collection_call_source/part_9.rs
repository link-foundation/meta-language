// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptCallSourceUnit;
use super::JavaScriptCallTemplate;
use super::JavaScriptCollectionCallableDirectory;
use super::JavaScriptCollectionConstructorOwner;
use super::JavaScriptCollectionIteratorCallCompilation;
use super::JavaScriptCollectionIteratorCallableDirectory;
use super::JavaScriptCollectionIteratorMethodOwner;
use super::JavaScriptSemanticValue;
use super::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result;
use super::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Result;
use super::MlTemporaryCompileJavaScriptIterableCollectionConstructorJoin462;
use super::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Result;
use super::dependency_function1;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::normalize_java_script_collection_iterator_operation;
use super::order_java_script_call_code_functions;
use super::read_java_script_collection_iterator_operation_strategy;
use super::require_java_script_array_species_directory;
use super::source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop445;
use super::source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop446;
use super::source_ml_temporary_read_java_script_collection_constructor_kind_loop459;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_collection_iterator_method_alias(left: self::JavaScriptCollectionIteratorMethodOwner, right: self::JavaScriptCollectionIteratorMethodOwner) -> bool {
    {
        let operation = self::normalize_java_script_collection_iterator_operation(&(match left.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, source_ml_source1_data_property_value1797, _, _, _) => {
                source_ml_source1_data_property_value1797.clone()
            }
        }));
        if ((((match left.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value1800, _, _, _, _, _) => {
                source_ml_source1_data_property_value1800.clone()
            }
        }) != (match right.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(source_ml_source1_data_property_value1801, _, _, _, _, _) => {
                source_ml_source1_data_property_value1801.clone()
            }
        })) || ((match left.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value1804, _, _, _, _) => {
                source_ml_source1_data_property_value1804.clone()
            }
        }) != (match right.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, source_ml_source1_data_property_value1805, _, _, _, _) => {
                source_ml_source1_data_property_value1805.clone()
            }
        }))) || (operation != (self::normalize_java_script_collection_iterator_operation(&(match right.clone() {
            self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, source_ml_source1_data_property_value1807, _, _, _) => {
                source_ml_source1_data_property_value1807.clone()
            }
        }))))) {
            false
        } else {
            if (operation == (self::ml_string::Text::from_units(vec![73, 116, 101, 114, 97, 116, 111, 114, 46, 115, 101, 108, 102]))) {
                true
            } else {
                if ((self::read_java_script_collection_iterator_operation_strategy(&(operation))) != (self::ml_string::Text::from_units(vec![]))) {
                    self::test_java_script_semantic_same_value(match left.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value1808, _, _) => {
                            let source_ml_source1_data_property_value1808 = *source_ml_source1_data_property_value1808;
                            source_ml_source1_data_property_value1808.clone()
                        }
                    }, match right.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, source_ml_source1_data_property_value1809, _, _) => {
                            let source_ml_source1_data_property_value1809 = *source_ml_source1_data_property_value1809;
                            source_ml_source1_data_property_value1809.clone()
                        }
                    })
                } else {
                    (self::test_java_script_semantic_same_value(match left.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value1810, _) => {
                            let source_ml_source1_data_property_value1810 = *source_ml_source1_data_property_value1810;
                            source_ml_source1_data_property_value1810.clone()
                        }
                    }, match right.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, source_ml_source1_data_property_value1811, _) => {
                            let source_ml_source1_data_property_value1811 = *source_ml_source1_data_property_value1811;
                            source_ml_source1_data_property_value1811.clone()
                        }
                    }) && self::test_java_script_semantic_same_value(match left.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value1812) => {
                            let source_ml_source1_data_property_value1812 = *source_ml_source1_data_property_value1812;
                            source_ml_source1_data_property_value1812.clone()
                        }
                    }, match right.clone() {
                        self::JavaScriptCollectionIteratorMethodOwner::CollectionIteratorMethodOwner(_, _, _, _, _, source_ml_source1_data_property_value1813) => {
                            let source_ml_source1_data_property_value1813 = *source_ml_source1_data_property_value1813;
                            source_ml_source1_data_property_value1813.clone()
                        }
                    }))
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_collection_iterator_call_directory(directory: self::JavaScriptCollectionIteratorCallableDirectory) -> self::JavaScriptCollectionIteratorCallCompilation {
    {
        let empty = {
            let source_ml_source1_constructor_field1814 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallFunction>::new()));
            {
                let source_ml_source1_constructor_field1816 = match directory.clone() {
                    self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(source_ml_source1_data_property_value1815, _, _) => {
                        source_ml_source1_data_property_value1815.clone()
                    }
                };
                {
                    let source_ml_source1_constructor_field1818 = match directory.clone() {
                        self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(_, source_ml_source1_data_property_value1817, _) => {
                            let source_ml_source1_data_property_value1817 = *source_ml_source1_data_property_value1817;
                            source_ml_source1_data_property_value1817.clone()
                        }
                    };
                    self::JavaScriptCallProgram::CallProgram(source_ml_source1_constructor_field1814.clone(), source_ml_source1_constructor_field1816.clone(), Box::new(source_ml_source1_constructor_field1818.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)>, None::<(Option<self::JavaScriptArraySpeciesDirectory>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>)
                }
            }
        };
        if ((({ let ml_reference_array537 = match directory.clone() {
            self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(_, _, source_ml_source1_data_property_value1820) => {
                source_ml_source1_data_property_value1820.clone()
            }
        }; let ml_reference_projection537 = ml_reference_array537.borrow().len() as f64; ml_reference_projection537 }) > (64f64)) || (({ let ml_reference_array538 = match directory.clone() {
            self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(source_ml_source1_data_property_value1822, _, _) => {
                source_ml_source1_data_property_value1822.clone()
            }
        }; let ml_reference_projection538 = ml_reference_array538.borrow().len() as f64; ml_reference_projection538 }) > (4096f64))) {
            self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 119, 110, 101, 114, 32, 98, 117, 100, 103, 101, 116]))
        } else {
            {
                let functions = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallFunction>::new()));
                {
                    let owners = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCollectionIteratorMethodOwner>::new()));
                    {
                        let source_ml_temporary_values443 = match directory.clone() {
                            self::JavaScriptCollectionIteratorCallableDirectory::CollectionIteratorCallableDirectory(_, _, source_ml_source1_data_property_value1823) => {
                                source_ml_source1_data_property_value1823.clone()
                            }
                        };
                        {
                            let source_ml_temporary_index443 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source1_s1824 = self::source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop445(directory.clone(), empty.clone(), functions.clone(), owners.clone(), source_ml_temporary_values443.clone(), source_ml_temporary_index443.clone());
                                match source_ml_source1_s1824.clone() {
                                    self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Done(functions_2, owners_2) => {
                                        {
                                            let ordered = self::order_java_script_call_code_functions(functions_2.clone());
                                            {
                                                let program = {
                                                    let source_ml_source1_nominal_field1826 = match empty.clone() {
                                                        self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value1827, _, _, _, _, _) => {
                                                            source_ml_source1_data_property_value1827.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_nominal_field1828 = match empty.clone() {
                                                            self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value1829, _, _, _, _) => {
                                                                source_ml_source1_data_property_value1829.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_nominal_field1830 = match empty.clone() {
                                                                self::JavaScriptCallProgram::CallProgram(_, _, source_ml_source1_data_property_value1831, _, _, _) => {
                                                                    let source_ml_source1_data_property_value1831 = *source_ml_source1_data_property_value1831;
                                                                    source_ml_source1_data_property_value1831.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_nominal_field1832 = match empty.clone() {
                                                                    self::JavaScriptCallProgram::CallProgram(_, _, _, source_ml_source1_data_property_value1833, _, _) => {
                                                                        source_ml_source1_data_property_value1833.clone()
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source1_nominal_field1834 = match empty.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source1_data_property_value1835, _) => {
                                                                            source_ml_source1_data_property_value1835.clone()
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source1_nominal_field1836 = match empty.clone() {
                                                                            self::JavaScriptCallProgram::CallProgram(_, _, _, _, _, source_ml_source1_data_property_value1837) => {
                                                                                source_ml_source1_data_property_value1837.clone()
                                                                            }
                                                                        };
                                                                        self::JavaScriptCallProgram::CallProgram(ordered.clone(), source_ml_source1_nominal_field1828.clone(), Box::new(source_ml_source1_nominal_field1830.clone()), source_ml_source1_nominal_field1832.clone(), source_ml_source1_nominal_field1834.clone(), source_ml_source1_nominal_field1836.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_temporary_index444 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s1839 = self::source_ml_temporary_compile_java_script_collection_iterator_call_directory_loop446(empty.clone(), program.clone(), owners_2.clone(), source_ml_temporary_index444.clone());
                                                        match source_ml_source1_s1839.clone() {
                                                            self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Done => {
                                                                self::JavaScriptCollectionIteratorCallCompilation::CollectionIteratorCallCompilation(Box::new(program.clone()), self::ml_string::Text::from_units(vec![]))
                                                            }
                                                            self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop446Return(source_ml_temporary_result446) => {
                                                                let source_ml_temporary_result446 = *source_ml_temporary_result446;
                                                                source_ml_temporary_result446.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Result::MlTemporaryCompileJavaScriptCollectionIteratorCallDirectoryLoop445Return(source_ml_temporary_result446_2) => {
                                        let source_ml_temporary_result446_2 = *source_ml_temporary_result446_2;
                                        source_ml_temporary_result446_2.clone()
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
pub fn read_java_script_collection_operation_kind(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if self::ml_array::includes(&{ let ml_reference_value539 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112, 46, 103, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 115, 101, 116]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 104, 97, 115]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 100, 101, 108, 101, 116, 101]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 99, 108, 101, 97, 114]), self::ml_string::Text::from_units(vec![77, 97, 112, 46, 115, 105, 122, 101])]))).borrow().clone(); ml_reference_value539 }, operation, 0f64, false, |a, b| a == b) {
        self::ml_string::Text::from_units(vec![77, 97, 112])
    } else {
        if self::ml_array::includes(&{ let ml_reference_value540 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![83, 101, 116, 46, 97, 100, 100]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 104, 97, 115]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 100, 101, 108, 101, 116, 101]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 99, 108, 101, 97, 114]), self::ml_string::Text::from_units(vec![83, 101, 116, 46, 115, 105, 122, 101])]))).borrow().clone(); ml_reference_value540 }, operation, 0f64, false, |a, b| a == b) {
            self::ml_string::Text::from_units(vec![83, 101, 116])
        } else {
            self::dependency_function1(operation)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_object_assignment_operation(operation: &self::ml_string::Text) -> bool {
    ((((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 97, 115, 115, 105, 103, 110]))) || ((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 103, 101, 116, 116, 101, 114])))) || ((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 115, 101, 116, 116, 101, 114]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_object_definition_operation(operation: &self::ml_string::Text) -> bool {
    (((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121]))) || ((*operation) == (self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_object_definition_function(identity: self::ml::Big, realm: self::ml::Big, operation: Option<self::ml_string::Text>) -> Option<self::JavaScriptCallFunction> {
    {
        let selected = operation.clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
        if ({
            let source_ml_source1_comparison_operand2004 = None::<()>;
            !(match (operation.clone(), source_ml_source1_comparison_operand2004.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })
        } && !(self::test_java_script_object_definition_operation(&(selected)))) {
            None::<self::JavaScriptCallFunction>
        } else {
            Some({
                let instructions = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::Argument(2f64, 1f64), self::JavaScriptCallInstruction::Argument(3f64, 2f64), if (selected == (self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 121]))) {
                    {
                        let source_ml_source1_constructor_field2005 = 4f64;
                        {
                            let source_ml_source1_constructor_field2006 = 1f64;
                            {
                                let source_ml_source1_constructor_field2007 = 2f64;
                                {
                                    let source_ml_source1_constructor_field2008 = 3f64;
                                    {
                                        let source_ml_source1_constructor_field2010 = Some(selected.clone());
                                        self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source1_constructor_field2005, source_ml_source1_constructor_field2006, source_ml_source1_constructor_field2007, source_ml_source1_constructor_field2008, realm.clone(), Some((source_ml_source1_constructor_field2010.clone(),)))
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_constructor_field2011 = 4f64;
                        {
                            let source_ml_source1_constructor_field2012 = 1f64;
                            {
                                let source_ml_source1_constructor_field2013 = 2f64;
                                {
                                    let source_ml_source1_constructor_field2014 = 3f64;
                                    self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source1_constructor_field2011, source_ml_source1_constructor_field2012, source_ml_source1_constructor_field2013, source_ml_source1_constructor_field2014, realm.clone(), None::<(Option<self::ml_string::Text>,)>)
                                }
                            }
                        }
                    }
                }, self::JavaScriptCallInstruction::Return(4f64)]));
                {
                    let source_ml_source1_constructor_field2019 = self::JavaScriptSemanticValue::Undefined;
                    {
                        let source_ml_source1_constructor_field2020 = 0f64;
                        {
                            let source_ml_source1_constructor_field2021 = 5f64;
                            {
                                let source_ml_source1_constructor_field2022 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field2019.clone()), source_ml_source1_constructor_field2020, source_ml_source1_constructor_field2021, source_ml_source1_constructor_field2022.clone(), instructions.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                            }
                        }
                    }
                }
            })
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_collection_constructor_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode) == (self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_collection_intrinsic_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode) == (self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_primitive_intrinsic_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode) == (self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_constructor_kind(fn_: self::JavaScriptCallFunction) -> self::ml_string::Text {
    if !(self::test_java_script_collection_constructor_mode(&(match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source1_data_property_value2024, _, _, _, _, _, _, _) => {
            source_ml_source1_data_property_value2024.clone()
        }
    }))) {
        self::ml_string::Text::from_units(vec![])
    } else {
        {
            let kind = self::ml_string::Text::from_units(vec![]);
            {
                let source_ml_temporary_values458 = match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2025, _, _) => {
                        source_ml_source1_data_property_value2025.clone()
                    }
                };
                {
                    let source_ml_temporary_index458 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s2026 = self::source_ml_temporary_read_java_script_collection_constructor_kind_loop459(kind.clone(), source_ml_temporary_values458.clone(), source_ml_temporary_index458.clone());
                        match source_ml_source1_s2026.clone() {
                            self::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Result::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Done(kind_2) => {
                                kind_2.clone()
                            }
                            self::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Result::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Return(source_ml_temporary_result459) => {
                                source_ml_temporary_result459.clone()
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
pub fn compile_java_script_iterable_collection_constructor(owner: self::JavaScriptCollectionConstructorOwner) -> self::JavaScriptCallFunction {
    {
        let instructions = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::Constant(2f64, Box::new(match owner.clone() {
            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, _, source_ml_source1_data_property_value2027, _, _, _, _, _, _) => {
                let source_ml_source1_data_property_value2027 = *source_ml_source1_data_property_value2027;
                source_ml_source1_data_property_value2027.clone()
            }
        })), self::JavaScriptCallInstruction::CollectionAllocate(3f64, 2f64, match owner.clone() {
            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(source_ml_source1_data_property_value2028, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value2028.clone()
            }
        }), self::JavaScriptCallInstruction::Constant(7f64, Box::new(self::JavaScriptSemanticValue::Undefined)), self::JavaScriptCallInstruction::StrictEqual(8f64, 1f64, 7f64), self::JavaScriptCallInstruction::Branch(8f64, 0f64, 6f64), self::JavaScriptCallInstruction::Constant(7f64, Box::new(self::JavaScriptSemanticValue::Null)), self::JavaScriptCallInstruction::StrictEqual(8f64, 1f64, 7f64), self::JavaScriptCallInstruction::Branch(8f64, 0f64, 9f64), self::JavaScriptCallInstruction::Constant(4f64, Box::new(self::JavaScriptSemanticValue::Text(if (((match owner.clone() {
            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(source_ml_source1_data_property_value2030, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value2030.clone()
            }
        }) == (self::ml_string::Text::from_units(vec![77, 97, 112]))) || ((match owner.clone() {
            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(source_ml_source1_data_property_value2032, _, _, _, _, _, _, _, _, _) => {
                source_ml_source1_data_property_value2032.clone()
            }
        }) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112])))) {
            std::rc::Rc::new(std::cell::RefCell::new(vec![115f64, 101f64, 116f64]))
        } else {
            std::rc::Rc::new(std::cell::RefCell::new(vec![97f64, 100f64, 100f64]))
        }))), self::JavaScriptCallInstruction::OrdinaryGet(5f64, 3f64, 4f64, 3f64), self::JavaScriptCallInstruction::Primitive(5f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 97, 100, 100, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![5f64]))), self::JavaScriptCallInstruction::Constant(6f64, Box::new(match owner.clone() {
            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, _, _, _, source_ml_source1_data_property_value2033, _, _, _, _) => {
                let source_ml_source1_data_property_value2033 = *source_ml_source1_data_property_value2033;
                source_ml_source1_data_property_value2033.clone()
            }
        })), {
            let source_ml_source1_constructor_field2034 = 7f64;
            {
                let source_ml_source1_constructor_field2035 = 8f64;
                {
                    let source_ml_source1_constructor_field2036 = 1f64;
                    {
                        let source_ml_source1_constructor_field2037 = 6f64;
                        {
                            let source_ml_source1_constructor_field2038 = 0f64;
                            {
                                let source_ml_source1_constructor_field2041 = match owner.clone() {
                                    self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, source_ml_source1_data_property_value2040, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2040.clone()
                                    }
                                };
                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source1_constructor_field2034, source_ml_source1_constructor_field2035, source_ml_source1_constructor_field2036, source_ml_source1_constructor_field2037, source_ml_source1_constructor_field2038, self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]), source_ml_source1_constructor_field2041.clone(), None::<(Option<bool>,)>)
                            }
                        }
                    }
                }
            }
        }, self::JavaScriptCallInstruction::PushFinally(0f64)]));
        {
            let start = { let ml_reference_array541 = instructions.clone(); let ml_reference_projection541 = ml_reference_array541.borrow().len() as f64; ml_reference_projection541 };
            {
                let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored460 = { let ml_method_owner542 = instructions.clone(); let ml_method_argument542_0 = {
                    let source_ml_source1_constructor_field2042 = 9f64;
                    {
                        let source_ml_source1_constructor_field2043 = 8f64;
                        {
                            let source_ml_source1_constructor_field2044 = 0f64;
                            {
                                let source_ml_source1_constructor_field2045 = 0f64;
                                {
                                    let source_ml_source1_constructor_field2046 = 0f64;
                                    {
                                        let source_ml_source1_constructor_field2049 = match owner.clone() {
                                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, source_ml_source1_data_property_value2048, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2048.clone()
                                            }
                                        };
                                        self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source1_constructor_field2042, source_ml_source1_constructor_field2043, source_ml_source1_constructor_field2044, source_ml_source1_constructor_field2045, source_ml_source1_constructor_field2046, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), source_ml_source1_constructor_field2049.clone(), None::<(Option<bool>,)>)
                                    }
                                }
                            }
                        }
                    }
                }; let ml_inserted = vec![ml_method_argument542_0]; let mut ml_method_values542 = ml_method_owner542.borrow_mut(); let ml_length = ml_method_values542.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values542.extend(ml_inserted); ml_length as f64 };
                {
                    let exhausted = { let ml_reference_array543 = instructions.clone(); let ml_reference_projection543 = ml_reference_array543.borrow().len() as f64; ml_reference_projection543 };
                    {
                        let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored461 = { let ml_method_owner544 = instructions.clone(); let ml_method_argument544_0 = self::JavaScriptCallInstruction::Branch(8f64, 0f64, (exhausted + 1f64)); let ml_inserted = vec![ml_method_argument544_0]; let mut ml_method_values544 = ml_method_owner544.borrow_mut(); let ml_length = ml_method_values544.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values544.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_source1_s2054 = if (((match owner.clone() {
                                self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(source_ml_source1_data_property_value2056, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2056.clone()
                                }
                            }) == (self::ml_string::Text::from_units(vec![77, 97, 112]))) || ((match owner.clone() {
                                self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(source_ml_source1_data_property_value2058, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value2058.clone()
                                }
                            }) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112])))) {
                                {
                                    let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored472 = { let ml_method_owner545 = instructions.clone(); let ml_method_argument545_0 = self::JavaScriptCallInstruction::Primitive(9f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 77, 97, 112, 32, 101, 110, 116, 114, 121, 32, 111, 98, 106, 101, 99, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![9f64]))); let ml_method_argument545_1 = self::JavaScriptCallInstruction::Constant(13f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![48f64]))))); let ml_method_argument545_2 = self::JavaScriptCallInstruction::OrdinaryGet(10f64, 9f64, 13f64, 9f64); let ml_method_argument545_3 = self::JavaScriptCallInstruction::Constant(13f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![49f64]))))); let ml_method_argument545_4 = self::JavaScriptCallInstruction::OrdinaryGet(11f64, 9f64, 13f64, 9f64); let ml_inserted = vec![ml_method_argument545_0, ml_method_argument545_1, ml_method_argument545_2, ml_method_argument545_3, ml_method_argument545_4]; let mut ml_method_values545 = ml_method_owner545.borrow_mut(); let ml_length = ml_method_values545.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values545.extend(ml_inserted); ml_length as f64 };
                                    self::MlTemporaryCompileJavaScriptIterableCollectionConstructorJoin462::MlTemporaryCompileJavaScriptIterableCollectionConstructorJoin462Next
                                }
                            } else {
                                self::MlTemporaryCompileJavaScriptIterableCollectionConstructorJoin462::MlTemporaryCompileJavaScriptIterableCollectionConstructorJoin462Next
                            };
                            match source_ml_source1_s2054.clone() {
                                self::MlTemporaryCompileJavaScriptIterableCollectionConstructorJoin462::MlTemporaryCompileJavaScriptIterableCollectionConstructorJoin462Next => {
                                    {
                                        let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored463 = { let ml_method_owner546 = instructions.clone(); let ml_method_argument546_0 = self::JavaScriptCallInstruction::Call(12f64, 5f64, 3f64, if (((match owner.clone() {
                                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(source_ml_source1_data_property_value2060, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2060.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![77, 97, 112]))) || ((match owner.clone() {
                                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(source_ml_source1_data_property_value2062, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source1_data_property_value2062.clone()
                                            }
                                        }) == (self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112])))) {
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![10f64, 11f64]))
                                        } else {
                                            std::rc::Rc::new(std::cell::RefCell::new(vec![9f64]))
                                        }); let ml_method_argument546_1 = self::JavaScriptCallInstruction::Jump(start); let ml_inserted = vec![ml_method_argument546_0, ml_method_argument546_1]; let mut ml_method_values546 = ml_method_owner546.borrow_mut(); let ml_length = ml_method_values546.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values546.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let leave = { let ml_reference_array547 = instructions.clone(); let ml_reference_projection547 = ml_reference_array547.borrow().len() as f64; ml_reference_projection547 };
                                            {
                                                let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored464 = { let ml_method_owner548 = instructions.clone(); let ml_method_argument548_0 = self::JavaScriptCallInstruction::CompleteJump(0f64, 0f64); let ml_inserted = vec![ml_method_argument548_0]; let mut ml_method_values548 = ml_method_owner548.borrow_mut(); let ml_length = ml_method_values548.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values548.extend(ml_inserted); ml_length as f64 };
                                                {
                                                    let cleanup = { let ml_reference_array549 = instructions.clone(); let ml_reference_projection549 = ml_reference_array549.borrow().len() as f64; ml_reference_projection549 };
                                                    {
                                                        let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored465 = { let ml_method_owner550 = instructions.clone(); let ml_method_argument550_0 = {
                                                            let source_ml_source1_constructor_field2063 = 7f64;
                                                            {
                                                                let source_ml_source1_constructor_field2064 = 8f64;
                                                                {
                                                                    let source_ml_source1_constructor_field2065 = 0f64;
                                                                    {
                                                                        let source_ml_source1_constructor_field2066 = 0f64;
                                                                        {
                                                                            let source_ml_source1_constructor_field2067 = 0f64;
                                                                            {
                                                                                let source_ml_source1_constructor_field2070 = match owner.clone() {
                                                                                    self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, source_ml_source1_data_property_value2069, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source1_data_property_value2069.clone()
                                                                                    }
                                                                                };
                                                                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source1_constructor_field2063, source_ml_source1_constructor_field2064, source_ml_source1_constructor_field2065, source_ml_source1_constructor_field2066, source_ml_source1_constructor_field2067, self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]), source_ml_source1_constructor_field2070.clone(), None::<(Option<bool>,)>)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }; let ml_method_argument550_1 = self::JavaScriptCallInstruction::FinishFinally; let ml_inserted = vec![ml_method_argument550_0, ml_method_argument550_1]; let mut ml_method_values550 = ml_method_owner550.borrow_mut(); let ml_length = ml_method_values550.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values550.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let end = { let ml_reference_array551 = instructions.clone(); let ml_reference_projection551 = ml_reference_array551.borrow().len() as f64; ml_reference_projection551 };
                                                            {
                                                                let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored466 = { let ml_method_owner552 = instructions.clone(); let ml_method_argument552_0 = self::JavaScriptCallInstruction::Return(3f64); let ml_inserted = vec![ml_method_argument552_0]; let mut ml_method_values552 = ml_method_owner552.borrow_mut(); let ml_length = ml_method_values552.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values552.extend(ml_inserted); ml_length as f64 };
                                                                {
                                                                    let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored467 = { let ml_array_owner553 = instructions.clone(); let ml_array_argument553_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(8f64, end, 6f64); let ml_array_argument553_1: f64 = 5f64; let ml_array_argument553_2: f64 = (5f64 + 1f64); let ml_array_length = ml_array_owner553.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument553_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument553_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner553.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument553_0); } } ml_array_owner553 };
                                                                    {
                                                                        let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored468 = { let ml_array_owner554 = instructions.clone(); let ml_array_argument554_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(8f64, end, 9f64); let ml_array_argument554_1: f64 = 8f64; let ml_array_argument554_2: f64 = (8f64 + 1f64); let ml_array_length = ml_array_owner554.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument554_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument554_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner554.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument554_0); } } ml_array_owner554 };
                                                                        {
                                                                            let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored469 = { let ml_array_owner555 = instructions.clone(); let ml_array_argument555_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::PushFinally(cleanup); let ml_array_argument555_1: f64 = 14f64; let ml_array_argument555_2: f64 = (14f64 + 1f64); let ml_array_length = ml_array_owner555.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument555_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument555_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner555.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument555_0); } } ml_array_owner555 };
                                                                            {
                                                                                let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored470 = { let ml_array_owner556 = instructions.clone(); let ml_array_argument556_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(8f64, leave, (exhausted + 1f64)); let ml_array_argument556_1: f64 = exhausted; let ml_array_argument556_2: f64 = (exhausted + 1f64); let ml_array_length = ml_array_owner556.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument556_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument556_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner556.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument556_0); } } ml_array_owner556 };
                                                                                {
                                                                                    let source_ml_temporary_compile_java_script_iterable_collection_constructor_ignored471 = { let ml_array_owner557 = instructions.clone(); let ml_array_argument557_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::CompleteJump(end, 0f64); let ml_array_argument557_1: f64 = leave; let ml_array_argument557_2: f64 = (leave + 1f64); let ml_array_length = ml_array_owner557.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument557_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument557_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner557.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument557_0); } } ml_array_owner557 };
                                                                                    {
                                                                                        let source_ml_source1_constructor_field2072 = match owner.clone() {
                                                                                            self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, source_ml_source1_data_property_value2071, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source1_data_property_value2071.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source1_constructor_field2074 = match owner.clone() {
                                                                                                self::JavaScriptCollectionConstructorOwner::CollectionConstructorOwner(_, _, source_ml_source1_data_property_value2073, _, _, _, _, _, _, _) => {
                                                                                                    source_ml_source1_data_property_value2073.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source1_constructor_field2076 = self::JavaScriptSemanticValue::Undefined;
                                                                                                {
                                                                                                    let source_ml_source1_constructor_field2077 = 0f64;
                                                                                                    {
                                                                                                        let source_ml_source1_constructor_field2078 = 14f64;
                                                                                                        {
                                                                                                            let source_ml_source1_constructor_field2079 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_constructor_field2072.clone(), source_ml_source1_constructor_field2074.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]), Box::new(source_ml_source1_constructor_field2076.clone()), source_ml_source1_constructor_field2077, source_ml_source1_constructor_field2078, source_ml_source1_constructor_field2079.clone(), instructions.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_compiled_collection_call_function(functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, fn_: Option<self::JavaScriptCallFunction>) -> bool {
    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
        false
    } else {
        {
            let source_ml_temporary_append_java_script_compiled_collection_call_function_ignored473 = { let ml_method_owner558 = functions.clone(); let ml_method_argument558_0 = fn_.clone().expect("a proven present value"); let ml_inserted = vec![ml_method_argument558_0]; let mut ml_method_values558 = ml_method_owner558.borrow_mut(); let ml_length = ml_method_values558.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values558.extend(ml_inserted); ml_length as f64 };
            true
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_collection_call_program(directory: self::JavaScriptCollectionCallableDirectory, functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>) -> self::JavaScriptCallProgram {
    if match (match directory.clone() {
        self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, _, _, _, _, source_ml_source1_data_property_value2081) => {
            ((source_ml_source1_data_property_value2081.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
        }
    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
        {
            let source_ml_source1_constructor_field2084 = match directory.clone() {
                self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(source_ml_source1_data_property_value2083, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2083.clone()
                }
            };
            {
                let source_ml_source1_constructor_field2086 = match directory.clone() {
                    self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, source_ml_source1_data_property_value2085, _, _, _, _, _, _) => {
                        let source_ml_source1_data_property_value2085 = *source_ml_source1_data_property_value2085;
                        source_ml_source1_data_property_value2085.clone()
                    }
                };
                self::JavaScriptCallProgram::CallProgram(functions.clone(), source_ml_source1_constructor_field2084.clone(), Box::new(source_ml_source1_constructor_field2086.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)>, None::<(Option<self::JavaScriptArraySpeciesDirectory>,)>, None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>)
            }
        }
    } else {
        {
            let source_ml_source1_constructor_field2089 = match directory.clone() {
                self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(source_ml_source1_data_property_value2088, _, _, _, _, _, _, _) => {
                    source_ml_source1_data_property_value2088.clone()
                }
            };
            {
                let source_ml_source1_constructor_field2091 = match directory.clone() {
                    self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, source_ml_source1_data_property_value2090, _, _, _, _, _, _) => {
                        let source_ml_source1_data_property_value2090 = *source_ml_source1_data_property_value2090;
                        source_ml_source1_data_property_value2090.clone()
                    }
                };
                {
                    let source_ml_source1_constructor_field2093 = Some((match directory.clone() {
                        self::JavaScriptCollectionCallableDirectory::CollectionCallableDirectory(_, _, _, _, _, _, _, source_ml_source1_data_property_value2092) => {
                            ((source_ml_source1_data_property_value2092.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                        }
                    }).clone().unwrap_or_else(|| self::require_java_script_array_species_directory()));
                    self::JavaScriptCallProgram::CallProgram(functions.clone(), source_ml_source1_constructor_field2089.clone(), Box::new(source_ml_source1_constructor_field2091.clone()), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,)>, Some((source_ml_source1_constructor_field2093.clone(),)), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceUnit>>>>,)>)
                }
            }
        }
    }
}
