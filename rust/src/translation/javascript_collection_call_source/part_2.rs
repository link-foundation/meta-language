// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptPrimitiveWrapperRoots;
use super::JavaScriptSemanticValue;
use super::MlOptional;
use super::MlTemporaryAppendJavaScriptObjectCoercionLoop17Result;
use super::MlTemporaryAppendJavaScriptObjectCoercionLoop20Result;
use super::MlTemporaryAppendJavaScriptObjectCoercionLoop27Result;
use super::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin1;
use super::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin3;
use super::MlTemporaryCompileJavaScriptSymbolRegistryFunctionJoin7;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_temporary_append_java_script_object_coercion_loop17;
use super::source_ml_temporary_append_java_script_object_coercion_loop20;
use super::source_ml_temporary_append_java_script_object_coercion_loop27;
use super::test_java_script_standard_array_creation_operation;
use super::test_java_script_standard_collection_for_each_operation;
use super::test_java_script_standard_object_observation_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_array_transform_operation(operation: &self::ml_string::Text) -> bool {
    (((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 109, 97, 112]))) || ((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 108, 116, 101, 114]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_array_species_getter_operation(operation: &self::ml_string::Text) -> bool {
    ((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 115, 112, 101, 99, 105, 101, 115, 32, 103, 101, 116, 116, 101, 114])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_object_enumeration_operation(operation: &self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value3 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 107, 101, 121, 115]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 118, 97, 108, 117, 101, 115]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 101, 110, 116, 114, 105, 101, 115])]))).borrow().clone(); ml_reference_value3 }, operation, 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_property_reflection_operation(operation: &self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value4 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 78, 97, 109, 101, 115]), self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 83, 121, 109, 98, 111, 108, 115]), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 111, 119, 110, 75, 101, 121, 115]), self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114])]))).borrow().clone(); ml_reference_value4 }, operation, 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_single_descriptor_operation(operation: &self::ml_string::Text) -> bool {
    (((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_property_definitions_operation(operation: &self::ml_string::Text) -> bool {
    (((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101, 114, 116, 105, 101, 115]))) || ((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_primitive_method_operation(operation: &self::ml_string::Text) -> bool {
    ((*operation) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_standard_array_creation_function(identity: self::ml::Big, realm: self::ml::Big, operation: &self::ml_string::Text, prototype: self::JavaScriptSemanticValue, iterator_key: self::JavaScriptSemanticValue) -> Option<self::JavaScriptCallFunction> {
    if (((!(self::test_java_script_standard_array_creation_operation(operation)) || match prototype.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => {
            false
        }
        _ => {
            true
        }
    }) || ((match prototype.clone() {
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
        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value10) => {
            Some((source_ml_source1_data_property_value10,))
        }
    }).clone().expect("a proven present value")).0) || match iterator_key.clone() {
        self::JavaScriptSemanticValue::Symbol(_) => {
            false
        }
        _ => {
            true
        }
    }) {
        None::<self::JavaScriptCallFunction>
    } else {
        Some({
            let source_ml_source1_constructor_field23 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source1_constructor_field24 = 0f64;
                {
                    let source_ml_source1_constructor_field25 = 4f64;
                    {
                        let source_ml_source1_constructor_field26 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                        {
                            let source_ml_source1_constructor_field27 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Constant(1f64, Box::new(prototype.clone())), self::JavaScriptCallInstruction::Constant(2f64, Box::new(iterator_key.clone())), self::JavaScriptCallInstruction::ArrayCreation(3f64, operation.clone(), 1f64, 2f64), self::JavaScriptCallInstruction::Return(3f64)]));
                            self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field23.clone()), source_ml_source1_constructor_field24, source_ml_source1_constructor_field25, source_ml_source1_constructor_field26.clone(), source_ml_source1_constructor_field27.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                        }
                    }
                }
            }
        })
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_standard_object_from_entries_function(identity: self::ml::Big, realm: self::ml::Big, prototype: self::JavaScriptSemanticValue, iterator_key: self::JavaScriptSemanticValue) -> Option<self::JavaScriptCallFunction> {
    if ((match prototype.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => {
            false
        }
        _ => {
            true
        }
    } || ((match prototype.clone() {
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
        self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value37) => {
            Some((source_ml_source1_data_property_value37,))
        }
    }).clone().expect("a proven present value")).0) || match iterator_key.clone() {
        self::JavaScriptSemanticValue::Symbol(_) => {
            false
        }
        _ => {
            true
        }
    }) {
        None::<self::JavaScriptCallFunction>
    } else {
        Some({
            let source_ml_source1_constructor_field50 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source1_constructor_field51 = 0f64;
                {
                    let source_ml_source1_constructor_field52 = 5f64;
                    {
                        let source_ml_source1_constructor_field53 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                        {
                            let source_ml_source1_constructor_field54 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::Constant(2f64, Box::new(prototype.clone())), self::JavaScriptCallInstruction::Constant(3f64, Box::new(iterator_key.clone())), self::JavaScriptCallInstruction::ObjectFromEntries(4f64, 1f64, 2f64, 3f64), self::JavaScriptCallInstruction::Return(4f64)]));
                            self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field50.clone()), source_ml_source1_constructor_field51, source_ml_source1_constructor_field52, source_ml_source1_constructor_field53.clone(), source_ml_source1_constructor_field54.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                        }
                    }
                }
            }
        })
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_standard_collection_for_each_function(identity: self::ml::Big, realm: self::ml::Big, operation: &self::ml_string::Text) -> Option<self::JavaScriptCallFunction> {
    if !(self::test_java_script_standard_collection_for_each_operation(operation)) {
        None::<self::JavaScriptCallFunction>
    } else {
        Some({
            let source_ml_source1_constructor_field58 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source1_constructor_field59 = 0f64;
                {
                    let source_ml_source1_constructor_field60 = 2f64;
                    {
                        let source_ml_source1_constructor_field61 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                        {
                            let source_ml_source1_constructor_field62 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::CollectionForEach(1f64, operation.clone()), self::JavaScriptCallInstruction::Return(1f64)]));
                            self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field58.clone()), source_ml_source1_constructor_field59, source_ml_source1_constructor_field60, source_ml_source1_constructor_field61.clone(), source_ml_source1_constructor_field62.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                        }
                    }
                }
            }
        })
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_standard_object_observation_function(identity: self::ml::Big, realm: self::ml::Big, operation: &self::ml_string::Text) -> Option<self::JavaScriptCallFunction> {
    if !(self::test_java_script_standard_object_observation_operation(operation)) {
        None::<self::JavaScriptCallFunction>
    } else {
        if ((*operation) == (self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110, 46, 116, 111, 83, 116, 114, 105, 110, 103]))) {
            Some({
                let source_ml_source1_constructor_field66 = self::JavaScriptSemanticValue::Undefined;
                {
                    let source_ml_source1_constructor_field67 = 0f64;
                    {
                        let source_ml_source1_constructor_field68 = 2f64;
                        {
                            let source_ml_source1_constructor_field69 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                            {
                                let source_ml_source1_constructor_field70 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::ObjectObservation(1f64, self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110, 46, 118, 97, 108, 117, 101, 79, 102]), 0f64, 0f64), self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))), self::JavaScriptCallInstruction::Return(1f64)]));
                                self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field66.clone()), source_ml_source1_constructor_field67, source_ml_source1_constructor_field68, source_ml_source1_constructor_field69.clone(), source_ml_source1_constructor_field70.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                            }
                        }
                    }
                }
            })
        } else {
            if ((self::ml_array::includes(&{ let ml_reference_value5 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 118, 97, 108, 117, 101, 79, 102]), self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114, 46, 118, 97, 108, 117, 101, 79, 102]), self::ml_string::Text::from_units(vec![66, 111, 111, 108, 101, 97, 110, 46, 118, 97, 108, 117, 101, 79, 102]), self::ml_string::Text::from_units(vec![66, 105, 103, 73, 110, 116, 46, 118, 97, 108, 117, 101, 79, 102])]))).borrow().clone(); ml_reference_value5 }, operation, 0f64, false, |a, b| a == b) || ((*operation) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 116, 111, 83, 116, 114, 105, 110, 103])))) || ((*operation) == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 46, 118, 97, 108, 117, 101, 79, 102])))) {
                Some({
                    let source_ml_source1_constructor_field74 = self::JavaScriptSemanticValue::Undefined;
                    {
                        let source_ml_source1_constructor_field75 = 0f64;
                        {
                            let source_ml_source1_constructor_field76 = 2f64;
                            {
                                let source_ml_source1_constructor_field77 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                {
                                    let source_ml_source1_constructor_field78 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::ObjectObservation(1f64, operation.clone(), 0f64, 0f64), self::JavaScriptCallInstruction::Return(1f64)]));
                                    self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field74.clone()), source_ml_source1_constructor_field75, source_ml_source1_constructor_field76, source_ml_source1_constructor_field77.clone(), source_ml_source1_constructor_field78.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                }
                            }
                        }
                    }
                })
            } else {
                if (((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 104, 97, 115, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121]))) || ((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46, 112, 114, 111, 112, 101, 114, 116, 121, 73, 115, 69, 110, 117, 109, 101, 114, 97, 98, 108, 101])))) {
                    Some({
                        let source_ml_source1_constructor_field82 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source1_constructor_field83 = 0f64;
                            {
                                let source_ml_source1_constructor_field84 = 4f64;
                                {
                                    let source_ml_source1_constructor_field85 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                    {
                                        let source_ml_source1_constructor_field86 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::PropertyKey(1f64, 1f64, realm.clone()), self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 110, 117, 108, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 110, 112, 117, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64]))), self::JavaScriptCallInstruction::ObjectObservation(3f64, operation.clone(), 0f64, 1f64), self::JavaScriptCallInstruction::Return(3f64)]));
                                        self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field82.clone()), source_ml_source1_constructor_field83, source_ml_source1_constructor_field84, source_ml_source1_constructor_field85.clone(), source_ml_source1_constructor_field86.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                    }
                                }
                            }
                        }
                    })
                } else {
                    {
                        let instructions = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64)]));
                        {
                            let source_ml_source1_s87 = if ((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 104, 97, 115, 79, 119, 110]))) {
                                {
                                    let source_ml_temporary_compile_java_script_standard_object_observation_function_ignored6 = { let ml_method_owner6 = instructions.clone(); let ml_method_argument6_0 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 110, 111, 110, 110, 117, 108, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 110, 112, 117, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_inserted = vec![ml_method_argument6_0]; let mut ml_method_values6 = ml_method_owner6.borrow_mut(); let ml_length = ml_method_values6.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values6.extend(ml_inserted); ml_length as f64 };
                                    self::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin1::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin1Next
                                }
                            } else {
                                self::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin1::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin1Next
                            };
                            match source_ml_source1_s87.clone() {
                                self::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin1::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin1Next => {
                                    {
                                        let source_ml_temporary_compile_java_script_standard_object_observation_function_ignored2 = { let ml_method_owner7 = instructions.clone(); let ml_method_argument7_0 = self::JavaScriptCallInstruction::Argument(2f64, 1f64); let ml_inserted = vec![ml_method_argument7_0]; let mut ml_method_values7 = ml_method_owner7.borrow_mut(); let ml_length = ml_method_values7.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values7.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let source_ml_source1_s88 = if ((*operation) == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 104, 97, 115, 79, 119, 110]))) {
                                                {
                                                    let source_ml_temporary_compile_java_script_standard_object_observation_function_ignored5 = { let ml_method_owner8 = instructions.clone(); let ml_method_argument8_0 = self::JavaScriptCallInstruction::PropertyKey(2f64, 2f64, realm.clone()); let ml_inserted = vec![ml_method_argument8_0]; let mut ml_method_values8 = ml_method_owner8.borrow_mut(); let ml_length = ml_method_values8.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values8.extend(ml_inserted); ml_length as f64 };
                                                    self::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin3::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin3Next
                                                }
                                            } else {
                                                self::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin3::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin3Next
                                            };
                                            match source_ml_source1_s88.clone() {
                                                self::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin3::MlTemporaryCompileJavaScriptStandardObjectObservationFunctionJoin3Next => {
                                                    Some({
                                                        let source_ml_temporary_compile_java_script_standard_object_observation_function_ignored4 = { let ml_method_owner9 = instructions.clone(); let ml_method_argument9_0 = self::JavaScriptCallInstruction::ObjectObservation(3f64, operation.clone(), 1f64, 2f64); let ml_method_argument9_1 = self::JavaScriptCallInstruction::Return(3f64); let ml_inserted = vec![ml_method_argument9_0, ml_method_argument9_1]; let mut ml_method_values9 = ml_method_owner9.borrow_mut(); let ml_length = ml_method_values9.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values9.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let source_ml_source1_constructor_field92 = self::JavaScriptSemanticValue::Undefined;
                                                            {
                                                                let source_ml_source1_constructor_field93 = 0f64;
                                                                {
                                                                    let source_ml_source1_constructor_field94 = 4f64;
                                                                    {
                                                                        let source_ml_source1_constructor_field95 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field92.clone()), source_ml_source1_constructor_field93, source_ml_source1_constructor_field94, source_ml_source1_constructor_field95.clone(), instructions.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                                                    }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_symbol_registry_operation(operation: &self::ml_string::Text) -> bool {
    (((*operation) == (self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 102, 111, 114]))) || ((*operation) == (self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 107, 101, 121, 70, 111, 114]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_symbol_registry_function(identity: self::ml::Big, realm: self::ml::Big, operation: &self::ml_string::Text) -> Option<self::JavaScriptCallFunction> {
    if !(self::test_java_script_symbol_registry_operation(operation)) {
        None::<self::JavaScriptCallFunction>
    } else {
        {
            let instructions = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64)]));
            {
                let source_ml_source1_s97 = if ((*operation) == (self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 102, 111, 114]))) {
                    {
                        let source_ml_temporary_compile_java_script_symbol_registry_function_ignored9 = { let ml_method_owner10 = instructions.clone(); let ml_method_argument10_0 = self::JavaScriptCallInstruction::Primitive(1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![1f64]))); let ml_inserted = vec![ml_method_argument10_0]; let mut ml_method_values10 = ml_method_owner10.borrow_mut(); let ml_length = ml_method_values10.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values10.extend(ml_inserted); ml_length as f64 };
                        self::MlTemporaryCompileJavaScriptSymbolRegistryFunctionJoin7::MlTemporaryCompileJavaScriptSymbolRegistryFunctionJoin7Next
                    }
                } else {
                    self::MlTemporaryCompileJavaScriptSymbolRegistryFunctionJoin7::MlTemporaryCompileJavaScriptSymbolRegistryFunctionJoin7Next
                };
                match source_ml_source1_s97.clone() {
                    self::MlTemporaryCompileJavaScriptSymbolRegistryFunctionJoin7::MlTemporaryCompileJavaScriptSymbolRegistryFunctionJoin7Next => {
                        Some({
                            let source_ml_temporary_compile_java_script_symbol_registry_function_ignored8 = { let ml_method_owner11 = instructions.clone(); let ml_method_argument11_0 = self::JavaScriptCallInstruction::SymbolRegistry(2f64, 1f64, if ((*operation) == (self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 46, 102, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![102, 111, 114])
                            } else {
                                self::ml_string::Text::from_units(vec![107, 101, 121, 70, 111, 114])
                            }); let ml_method_argument11_1 = self::JavaScriptCallInstruction::Return(2f64); let ml_inserted = vec![ml_method_argument11_0, ml_method_argument11_1]; let mut ml_method_values11 = ml_method_owner11.borrow_mut(); let ml_length = ml_method_values11.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values11.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_constructor_field101 = self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source1_constructor_field102 = 0f64;
                                    {
                                        let source_ml_source1_constructor_field103 = 3f64;
                                        {
                                            let source_ml_source1_constructor_field104 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                            self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field101.clone()), source_ml_source1_constructor_field102, source_ml_source1_constructor_field103, source_ml_source1_constructor_field104.clone(), instructions.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                        }
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_weak_collection_method_owner(operation: &self::ml_string::Text) -> self::ml_string::Text {
    if self::ml_array::includes(&{ let ml_reference_value12 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 103, 101, 116]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 115, 101, 116]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 104, 97, 115]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112, 46, 100, 101, 108, 101, 116, 101])]))).borrow().clone(); ml_reference_value12 }, operation, 0f64, false, |a, b| a == b) {
        self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112])
    } else {
        if self::ml_array::includes(&{ let ml_reference_value13 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 46, 97, 100, 100]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 46, 104, 97, 115]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 46, 100, 101, 108, 101, 116, 101])]))).borrow().clone(); ml_reference_value13 }, operation, 0f64, false, |a, b| a == b) {
            self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_weak_collection_operation_kind(operation: &self::ml_string::Text) -> self::ml_string::Text {
    self::read_java_script_weak_collection_method_owner(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_weak_collection_function(identity: self::ml::Big, realm: self::ml::Big, operation: &self::ml_string::Text) -> Option<self::JavaScriptCallFunction> {
    if ((self::read_java_script_weak_collection_operation_kind(operation)) == (self::ml_string::Text::from_units(vec![]))) {
        None::<self::JavaScriptCallFunction>
    } else {
        Some({
            let source_ml_source1_constructor_field193 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source1_constructor_field194 = 0f64;
                {
                    let source_ml_source1_constructor_field195 = 2f64;
                    {
                        let source_ml_source1_constructor_field196 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                        {
                            let source_ml_source1_constructor_field197 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::CollectionOperation(1f64, operation.clone()), self::JavaScriptCallInstruction::Return(1f64)]));
                            self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field193.clone()), source_ml_source1_constructor_field194, source_ml_source1_constructor_field195, source_ml_source1_constructor_field196.clone(), source_ml_source1_constructor_field197.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                        }
                    }
                }
            }
        })
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn append_java_script_object_coercion(code: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, input: f64, destination: f64, type_register: f64, key_register: f64, condition_register: f64, prototype_register: f64, roots: Option<self::JavaScriptPrimitiveWrapperRoots>) -> bool {
    if {
        let source_ml_source1_comparison_operand199 = {
            let source_ml_source1_optional_chain_receiver198 = (roots.clone(),);
            if match (source_ml_source1_optional_chain_receiver198.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                Some({
                    let source_ml_temporary_optional_chain12 = ((source_ml_source1_optional_chain_receiver198.clone()).0).clone().expect("a proven present value");
                    match source_ml_temporary_optional_chain12.clone() {
                        self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, _, _, _) => {
                            self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 87, 114, 97, 112, 112, 101, 114, 82, 111, 111, 116, 115])
                        }
                    }
                })
            } else {
                None::<self::ml_string::Text>
            }
        };
        !(match (source_ml_source1_comparison_operand199.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 87, 114, 97, 112, 112, 101, 114, 82, 111, 111, 116, 115])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
    } {
        false
    } else {
        {
            let choices = std::rc::Rc::new(std::cell::RefCell::new(vec![std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field205 = {
                    let source_ml_source1_nullish_field_receiver202 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver202.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present203 = ((source_ml_source1_nullish_field_receiver202.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present203.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, source_ml_source1_data_property_value204, _, _, _, _) => {
                                    let source_ml_source1_data_property_value204 = *source_ml_source1_data_property_value204;
                                    source_ml_source1_data_property_value204.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 116, 114, 105, 110, 103, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                };
                (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]), source_ml_source1_field205.clone())
            })), std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field210 = {
                    let source_ml_source1_nullish_field_receiver207 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver207.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present208 = ((source_ml_source1_nullish_field_receiver207.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present208.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, source_ml_source1_data_property_value209, _, _, _) => {
                                    let source_ml_source1_data_property_value209 = *source_ml_source1_data_property_value209;
                                    source_ml_source1_data_property_value209.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 110, 117, 109, 98, 101, 114, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                };
                (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114]), source_ml_source1_field210.clone())
            })), std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field215 = {
                    let source_ml_source1_nullish_field_receiver212 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver212.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present213 = ((source_ml_source1_nullish_field_receiver212.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present213.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, source_ml_source1_data_property_value214, _, _) => {
                                    let source_ml_source1_data_property_value214 = *source_ml_source1_data_property_value214;
                                    source_ml_source1_data_property_value214.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 98, 111, 111, 108, 101, 97, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                };
                (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110]), source_ml_source1_field215.clone())
            })), std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field220 = {
                    let source_ml_source1_nullish_field_receiver217 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver217.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present218 = ((source_ml_source1_nullish_field_receiver217.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present218.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, _, source_ml_source1_data_property_value219, _) => {
                                    let source_ml_source1_data_property_value219 = *source_ml_source1_data_property_value219;
                                    source_ml_source1_data_property_value219.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 116, 101, 103, 101, 114, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                };
                (self::ml_string::Text::from_units(vec![98, 105, 103, 105, 110, 116]), source_ml_source1_field220.clone())
            })), std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source1_field225 = {
                    let source_ml_source1_nullish_field_receiver222 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver222.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present223 = ((source_ml_source1_nullish_field_receiver222.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present223.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, _, _, source_ml_source1_data_property_value224) => {
                                    let source_ml_source1_data_property_value224 = *source_ml_source1_data_property_value224;
                                    source_ml_source1_data_property_value224.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 121, 109, 98, 111, 108, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                };
                (self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108]), source_ml_source1_field225.clone())
            }))]));
            {
                let source_ml_temporary_values13 = std::rc::Rc::new(std::cell::RefCell::new(vec![self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver226 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver226.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present227 = ((source_ml_source1_nullish_field_receiver226.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present227.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(source_ml_source1_data_property_value228, _, _, _, _, _) => {
                                    let source_ml_source1_data_property_value228 = *source_ml_source1_data_property_value228;
                                    source_ml_source1_data_property_value228.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 111, 98, 106, 101, 99, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }), self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver229 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver229.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present230 = ((source_ml_source1_nullish_field_receiver229.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present230.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, source_ml_source1_data_property_value231, _, _, _, _) => {
                                    let source_ml_source1_data_property_value231 = *source_ml_source1_data_property_value231;
                                    source_ml_source1_data_property_value231.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 116, 114, 105, 110, 103, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }), self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver232 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver232.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present233 = ((source_ml_source1_nullish_field_receiver232.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present233.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, source_ml_source1_data_property_value234, _, _, _) => {
                                    let source_ml_source1_data_property_value234 = *source_ml_source1_data_property_value234;
                                    source_ml_source1_data_property_value234.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 110, 117, 109, 98, 101, 114, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }), self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver235 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver235.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present236 = ((source_ml_source1_nullish_field_receiver235.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present236.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, source_ml_source1_data_property_value237, _, _) => {
                                    let source_ml_source1_data_property_value237 = *source_ml_source1_data_property_value237;
                                    source_ml_source1_data_property_value237.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 98, 111, 111, 108, 101, 97, 110, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }), self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver238 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver238.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present239 = ((source_ml_source1_nullish_field_receiver238.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present239.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, _, source_ml_source1_data_property_value240, _) => {
                                    let source_ml_source1_data_property_value240 = *source_ml_source1_data_property_value240;
                                    source_ml_source1_data_property_value240.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 105, 110, 116, 101, 103, 101, 114, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                }), self::MlOptional::Value({
                    let source_ml_source1_nullish_field_receiver241 = (roots.clone(),);
                    if match (source_ml_source1_nullish_field_receiver241.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present242 = ((source_ml_source1_nullish_field_receiver241.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present242.clone() {
                                self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(_, _, _, _, _, source_ml_source1_data_property_value243) => {
                                    let source_ml_source1_data_property_value243 = *source_ml_source1_data_property_value243;
                                    source_ml_source1_data_property_value243.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 115, 121, 109, 98, 111, 108, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                    }
                })]));
                {
                    let source_ml_temporary_index13 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s244 = self::source_ml_temporary_append_java_script_object_coercion_loop17(source_ml_temporary_values13.clone(), source_ml_temporary_index13.clone());
                        match source_ml_source1_s244.clone() {
                            self::MlTemporaryAppendJavaScriptObjectCoercionLoop17Result::MlTemporaryAppendJavaScriptObjectCoercionLoop17Done => {
                                {
                                    let source_ml_temporary_append_java_script_object_coercion_ignored18 = { let ml_method_owner14 = code.clone(); let ml_method_argument14_0 = self::JavaScriptCallInstruction::Primitive(condition_register, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 111, 98, 106, 101, 99, 116, 32, 99, 111, 101, 114, 99, 105, 98, 108, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![input]))); let ml_method_argument14_1 = self::JavaScriptCallInstruction::Primitive(condition_register, self::ml_string::Text::from_units(vec![116, 101, 115, 116, 32, 111, 98, 106, 101, 99, 116, 32, 118, 97, 108, 117, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![input]))); let ml_inserted = vec![ml_method_argument14_0, ml_method_argument14_1]; let mut ml_method_values14 = ml_method_owner14.borrow_mut(); let ml_length = ml_method_values14.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values14.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let object_branch = { let ml_reference_array15 = code.clone(); let ml_reference_projection15 = ml_reference_array15.borrow().len() as f64; ml_reference_projection15 };
                                        {
                                            let source_ml_temporary_append_java_script_object_coercion_ignored19 = { let ml_method_owner16 = code.clone(); let ml_method_argument16_0 = self::JavaScriptCallInstruction::Branch(condition_register, 0f64, (object_branch + 1f64)); let ml_method_argument16_1 = self::JavaScriptCallInstruction::Primitive(type_register, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![input]))); let ml_inserted = vec![ml_method_argument16_0, ml_method_argument16_1]; let mut ml_method_values16 = ml_method_owner16.borrow_mut(); let ml_length = ml_method_values16.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values16.extend(ml_inserted); ml_length as f64 };
                                            {
                                                let jumps = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                                                {
                                                    let source_ml_temporary_index15 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source1_s245 = self::source_ml_temporary_append_java_script_object_coercion_loop20(code.clone(), type_register, key_register, condition_register, prototype_register, jumps.clone(), choices.clone(), source_ml_temporary_index15.clone());
                                                        match source_ml_source1_s245.clone() {
                                                            self::MlTemporaryAppendJavaScriptObjectCoercionLoop20Result::MlTemporaryAppendJavaScriptObjectCoercionLoop20Done => {
                                                                {
                                                                    let source_ml_temporary_append_java_script_object_coercion_ignored21 = { let ml_method_owner17 = code.clone(); let ml_method_argument17_0 = self::JavaScriptCallInstruction::Constant(prototype_register, Box::new({
                                                                        let source_ml_source1_nullish_field_receiver246 = (roots.clone(),);
                                                                        if match (source_ml_source1_nullish_field_receiver246.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            {
                                                                                let source_ml_source1_nullish_field_present247 = ((source_ml_source1_nullish_field_receiver246.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_nullish_field_present247.clone() {
                                                                                    self::JavaScriptPrimitiveWrapperRoots::PrimitiveWrapperRoots(source_ml_source1_data_property_value248, _, _, _, _, _) => {
                                                                                        let source_ml_source1_data_property_value248 = *source_ml_source1_data_property_value248;
                                                                                        source_ml_source1_data_property_value248.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 111, 98, 106, 101, 99, 116, 80, 114, 111, 116, 111, 116, 121, 112, 101, 39, 41])).0, message_present: true }).raise()
                                                                        }
                                                                    })); let ml_inserted = vec![ml_method_argument17_0]; let mut ml_method_values17 = ml_method_owner17.borrow_mut(); let ml_length = ml_method_values17.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values17.extend(ml_inserted); ml_length as f64 };
                                                                    {
                                                                        let allocation = { let ml_reference_array18 = code.clone(); let ml_reference_projection18 = ml_reference_array18.borrow().len() as f64; ml_reference_projection18 };
                                                                        {
                                                                            let source_ml_temporary_append_java_script_object_coercion_ignored22 = { let ml_method_owner19 = code.clone(); let ml_method_argument19_0 = self::JavaScriptCallInstruction::PrimitiveWrapper(destination, input, prototype_register); let ml_inserted = vec![ml_method_argument19_0]; let mut ml_method_values19 = ml_method_owner19.borrow_mut(); let ml_length = ml_method_values19.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values19.extend(ml_inserted); ml_length as f64 };
                                                                            {
                                                                                let finish_jump = { let ml_reference_array20 = code.clone(); let ml_reference_projection20 = ml_reference_array20.borrow().len() as f64; ml_reference_projection20 };
                                                                                {
                                                                                    let source_ml_temporary_append_java_script_object_coercion_ignored23 = { let ml_method_owner21 = code.clone(); let ml_method_argument21_0 = self::JavaScriptCallInstruction::Jump(0f64); let ml_inserted = vec![ml_method_argument21_0]; let mut ml_method_values21 = ml_method_owner21.borrow_mut(); let ml_length = ml_method_values21.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values21.extend(ml_inserted); ml_length as f64 };
                                                                                    {
                                                                                        let unchanged = { let ml_reference_array22 = code.clone(); let ml_reference_projection22 = ml_reference_array22.borrow().len() as f64; ml_reference_projection22 };
                                                                                        {
                                                                                            let source_ml_temporary_append_java_script_object_coercion_ignored24 = { let ml_method_owner23 = code.clone(); let ml_method_argument23_0 = self::JavaScriptCallInstruction::Copy(destination, input); let ml_inserted = vec![ml_method_argument23_0]; let mut ml_method_values23 = ml_method_owner23.borrow_mut(); let ml_length = ml_method_values23.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values23.extend(ml_inserted); ml_length as f64 };
                                                                                            {
                                                                                                let end = { let ml_reference_array24 = code.clone(); let ml_reference_projection24 = ml_reference_array24.borrow().len() as f64; ml_reference_projection24 };
                                                                                                {
                                                                                                    let source_ml_temporary_append_java_script_object_coercion_ignored25 = { let ml_array_owner25 = code.clone(); let ml_array_argument25_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(condition_register, unchanged, (object_branch + 1f64)); let ml_array_argument25_1: f64 = object_branch; let ml_array_argument25_2: f64 = (object_branch + 1f64); let ml_array_length = ml_array_owner25.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument25_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument25_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner25.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument25_0); } } ml_array_owner25 };
                                                                                                    {
                                                                                                        let source_ml_temporary_append_java_script_object_coercion_ignored26 = { let ml_array_owner26 = code.clone(); let ml_array_argument26_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Jump(end); let ml_array_argument26_1: f64 = finish_jump; let ml_array_argument26_2: f64 = (finish_jump + 1f64); let ml_array_length = ml_array_owner26.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument26_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument26_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner26.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument26_0); } } ml_array_owner26 };
                                                                                                        {
                                                                                                            let source_ml_temporary_index16 = self::ml::Big::from_i128(0);
                                                                                                            {
                                                                                                                let source_ml_source1_s249 = self::source_ml_temporary_append_java_script_object_coercion_loop27(code.clone(), allocation, jumps.clone(), source_ml_temporary_index16.clone());
                                                                                                                match source_ml_source1_s249.clone() {
                                                                                                                    self::MlTemporaryAppendJavaScriptObjectCoercionLoop27Result::MlTemporaryAppendJavaScriptObjectCoercionLoop27Done => {
                                                                                                                        true
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
                            self::MlTemporaryAppendJavaScriptObjectCoercionLoop17Result::MlTemporaryAppendJavaScriptObjectCoercionLoop17Return(source_ml_temporary_result34) => {
                                source_ml_temporary_result34
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
pub fn test_java_script_array_sort_operation(operation: &self::ml_string::Text) -> bool {
    (((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 115, 111, 114, 116]))) || ((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 116, 111, 83, 111, 114, 116, 101, 100]))))
}
