// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptPrimitiveWrapperRoots;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Result;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Result;
use super::MlTemporaryTestJavaScriptSemanticTextEqualLoop127Result;
use super::append_java_script_object_coercion;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_find_java_script_call_code_function_loop133;
use super::source_ml_temporary_order_java_script_call_code_functions_loop134;
use super::source_ml_temporary_test_java_script_semantic_text_equal_loop127;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101, 108])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Boolean(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Boolean(value) => {
                        (left_value == value)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Number(left_value_2) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(left_value_3) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, _) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Object(identity_2, _) => {
                        (left_identity_2 == identity_2)
                    }
                    _ => {
                        false
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_length(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(source_ml_source1_f371) => {
            { let ml_reference_array157 = source_ml_source1_f371.clone(); let ml_reference_projection157 = ml_reference_array157.borrow().len() as f64; ml_reference_projection157 }
        }
        self::JavaScriptSemanticValue::SourceText(_, text) => {
            text.length()
        }
        _ => {
            (-1f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_text_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    {
        let length = self::read_java_script_semantic_text_length(left.clone());
        if ((length < (0f64)) || (length != (self::read_java_script_semantic_text_length(right.clone())))) {
            false
        } else {
            if (match left.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    true
                }
                _ => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    true
                }
                _ => {
                    false
                }
            }) {
                (((((match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Null => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value543) => {
                        Some((source_ml_source1_data_property_value543.clone(),))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        None::<(self::ml_string::Text,)>
                    }
                }).clone().expect("a proven present value")).0).clone()) == ((((match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Null => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value544) => {
                        Some((source_ml_source1_data_property_value544.clone(),))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        None::<(self::ml_string::Text,)>
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        None::<(self::ml_string::Text,)>
                    }
                }).clone().expect("a proven present value")).0).clone()))
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s545 = self::source_ml_temporary_test_java_script_semantic_text_equal_loop127(left.clone(), right.clone(), length, at);
                        match source_ml_source1_s545.clone() {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop127Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop127Done => {
                                true
                            }
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop127Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop127Return(source_ml_temporary_result127) => {
                                source_ml_temporary_result127
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
pub fn find_java_script_call_code_function(program: self::JavaScriptCallProgram, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = { let ml_reference_array158 = match program.clone() {
                self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value708, _, _, _, _, _) => {
                    source_ml_source1_data_property_value708.clone()
                }
            }; let ml_reference_projection158 = ml_reference_array158.borrow().len() as f64; ml_reference_projection158 };
            {
                let source_ml_source1_s709 = self::source_ml_temporary_find_java_script_call_code_function_loop133(program.clone(), identity.clone(), low, high);
                match source_ml_source1_s709.clone() {
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop133Return(source_ml_temporary_result133) => {
                        source_ml_temporary_result133
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn order_java_script_call_code_functions(functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>> {
    {
        let ordered = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value159 = functions.borrow().clone(); ml_reference_value159 }, 0f64, f64::INFINITY)));
        {
            let width = 1f64;
            {
                let source_ml_source1_s710 = self::source_ml_temporary_order_java_script_call_code_functions_loop134(functions.clone(), ordered.clone(), width);
                match source_ml_source1_s710.clone() {
                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Done(ordered_2) => {
                        ordered_2.clone()
                    }
                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop134Return(source_ml_temporary_result134) => {
                        source_ml_temporary_result134.clone()
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_code_destination(instruction: self::JavaScriptCallInstruction) -> f64 {
    match instruction.clone() {
        self::JavaScriptCallInstruction::Constant(destination, _) => {
            destination
        }
        self::JavaScriptCallInstruction::Copy(destination_2, _) => {
            destination_2
        }
        self::JavaScriptCallInstruction::Argument(destination_3, _) => {
            destination_3
        }
        self::JavaScriptCallInstruction::ArgumentCount(destination_4) => {
            destination_4
        }
        self::JavaScriptCallInstruction::ArgumentAt(destination_5, _) => {
            destination_5
        }
        self::JavaScriptCallInstruction::FunctionBind(destination_6) => {
            destination_6
        }
        self::JavaScriptCallInstruction::BoundFunctionCall(destination_7) => {
            destination_7
        }
        self::JavaScriptCallInstruction::FunctionToString(destination_8) => {
            destination_8
        }
        self::JavaScriptCallInstruction::Call(destination_9, _, _, _) => {
            destination_9
        }
        self::JavaScriptCallInstruction::ArgumentListCall(destination_10, _, _, _, _) => {
            destination_10
        }
        self::JavaScriptCallInstruction::Construct(destination_11, _, _, _) => {
            destination_11
        }
        self::JavaScriptCallInstruction::ConstructorCheck(destination_12, _, _) => {
            destination_12
        }
        self::JavaScriptCallInstruction::ReadNewTarget(destination_13) => {
            destination_13
        }
        self::JavaScriptCallInstruction::ArrayJoinEnter(destination_14, _) => {
            destination_14
        }
        self::JavaScriptCallInstruction::ConstructErrorClass(destination_15, _, _) => {
            destination_15
        }
        self::JavaScriptCallInstruction::CallClassConstructor(destination_16, _, _, _) => {
            destination_16
        }
        self::JavaScriptCallInstruction::ConstructCollection(destination_17, _, _) => {
            destination_17
        }
        self::JavaScriptCallInstruction::CollectionAllocate(destination_18, _, _) => {
            destination_18
        }
        self::JavaScriptCallInstruction::CollectionIteratorAllocate(destination_19, _, _, _, _) => {
            destination_19
        }
        self::JavaScriptCallInstruction::ArgumentRest(destination_20, _, _, _) => {
            destination_20
        }
        self::JavaScriptCallInstruction::IteratorProtocol(destination_21, _, _, _, _, _, _, _) => {
            destination_21
        }
        self::JavaScriptCallInstruction::CollectionIteratorResult(destination_22, _, _, _) => {
            destination_22
        }
        self::JavaScriptCallInstruction::CollectionOperation(destination_23, _) => {
            destination_23
        }
        self::JavaScriptCallInstruction::PrimitiveMethod(destination_24, _) => {
            destination_24
        }
        self::JavaScriptCallInstruction::CollectionForEach(destination_25, _) => {
            destination_25
        }
        self::JavaScriptCallInstruction::ArrayConcatenation(destination_26, _, _, _) => {
            destination_26
        }
        self::JavaScriptCallInstruction::ArraySlice(destination_27, _, _) => {
            destination_27
        }
        self::JavaScriptCallInstruction::ArrayCreation(destination_28, _, _, _) => {
            destination_28
        }
        self::JavaScriptCallInstruction::ArrayCallback(destination_29, _, _) => {
            destination_29
        }
        self::JavaScriptCallInstruction::ArraySearch(destination_30, _, _) => {
            destination_30
        }
        self::JavaScriptCallInstruction::ArrayTransform(destination_31, _, _, _) => {
            destination_31
        }
        self::JavaScriptCallInstruction::ObjectEnumeration(destination_32, _, _) => {
            destination_32
        }
        self::JavaScriptCallInstruction::PropertyDefinitions(destination_33, _, _, _) => {
            destination_33
        }
        self::JavaScriptCallInstruction::PropertyReflection(destination_34, _, _, _, _, _) => {
            destination_34
        }
        self::JavaScriptCallInstruction::InstanceOf(destination_35, _, _, _, _) => {
            destination_35
        }
        self::JavaScriptCallInstruction::ObjectObservation(destination_36, _, _, _) => {
            destination_36
        }
        self::JavaScriptCallInstruction::CollectionCopy(destination_37, _, _, _, _, _, _, _) => {
            destination_37
        }
        self::JavaScriptCallInstruction::ConstructErrorValue(destination_38, _, _, _, _, _, _, _, _) => {
            destination_38
        }
        self::JavaScriptCallInstruction::ReadDerivedReceiver(destination_39) => {
            destination_39
        }
        self::JavaScriptCallInstruction::InvokeErrorIntrinsic(destination_40) => {
            destination_40
        }
        self::JavaScriptCallInstruction::InvokeHostCapability(destination_41) => {
            destination_41
        }
        self::JavaScriptCallInstruction::PushHandler(destination_42, _) => {
            destination_42
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_43, _, _, _) => {
            destination_43
        }
        self::JavaScriptCallInstruction::SameValue(destination_44, _, _) => {
            destination_44
        }
        self::JavaScriptCallInstruction::StrictEqual(destination_45, _, _) => {
            destination_45
        }
        self::JavaScriptCallInstruction::Primitive(destination_46, _, _) => {
            destination_46
        }
        self::JavaScriptCallInstruction::OrdinaryAllocate(destination_47, _) => {
            destination_47
        }
        self::JavaScriptCallInstruction::LiteralPrototype(destination_48, _, _) => {
            destination_48
        }
        self::JavaScriptCallInstruction::LiteralAccessor(destination_49, _, _, _, _, _) => {
            destination_49
        }
        self::JavaScriptCallInstruction::ObjectFromEntries(destination_50, _, _, _) => {
            destination_50
        }
        self::JavaScriptCallInstruction::PrimitiveWrapper(destination_51, _, _) => {
            destination_51
        }
        self::JavaScriptCallInstruction::SymbolAllocate(destination_52, _) => {
            destination_52
        }
        self::JavaScriptCallInstruction::SymbolDescription(destination_53, _) => {
            destination_53
        }
        self::JavaScriptCallInstruction::SymbolRegistry(destination_54, _, _) => {
            destination_54
        }
        self::JavaScriptCallInstruction::FunctionName(destination_55, _, _, _, _) => {
            destination_55
        }
        self::JavaScriptCallInstruction::ArrayAllocate(destination_56, _, _) => {
            destination_56
        }
        self::JavaScriptCallInstruction::ClosureAllocate(destination_57, _, _, _, _) => {
            destination_57
        }
        self::JavaScriptCallInstruction::ArraySpread(destination_58, _, _, _, _, _) => {
            destination_58
        }
        self::JavaScriptCallInstruction::ObjectSpread(destination_59, _, _, _, _, _) => {
            destination_59
        }
        self::JavaScriptCallInstruction::OrdinaryDataProperty(destination_60, _, _, _, _) => {
            destination_60
        }
        self::JavaScriptCallInstruction::OrdinaryGet(destination_61, _, _, _) => {
            destination_61
        }
        self::JavaScriptCallInstruction::OrdinaryDelete(destination_62, _, _) => {
            destination_62
        }
        self::JavaScriptCallInstruction::OrdinaryHas(destination_63, _, _) => {
            destination_63
        }
        self::JavaScriptCallInstruction::OrdinarySet(destination_64, _, _, _, _) => {
            destination_64
        }
        self::JavaScriptCallInstruction::OrdinaryDefine(destination_65, _, _, _, _) => {
            destination_65
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_66, _, _) => {
            destination_66
        }
        self::JavaScriptCallInstruction::PropertyDefinition(destination_67, _, _, _, _, _) => {
            destination_67
        }
        _ => {
            (-1f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn require_java_script_array_species_directory() -> self::JavaScriptArraySpeciesDirectory {
    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![116, 104, 101, 32, 99, 111, 109, 112, 105, 108, 101, 100, 32, 97, 114, 114, 97, 121, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 118, 97, 108, 105, 100, 97, 116, 101, 100, 32, 115, 112, 101, 99, 105, 101, 115, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])).0, message_present: true }).raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_array_concatenation_function(identity: self::ml::Big, realm: self::ml::Big, roots: Option<self::JavaScriptPrimitiveWrapperRoots>, species: Option<self::JavaScriptArraySpeciesDirectory>, spreadable_key: Option<self::JavaScriptSemanticValue>) -> Option<self::JavaScriptCallFunction> {
    if ({
        let source_ml_source1_comparison_operand989 = {
            let source_ml_source1_optional_chain_receiver987 = (species.clone(),);
            if match (source_ml_source1_optional_chain_receiver987.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                Some({
                    let source_ml_temporary_optional_chain143 = ((source_ml_source1_optional_chain_receiver987.clone()).0).clone().expect("a proven present value");
                    match source_ml_temporary_optional_chain143.clone() {
                        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(source_ml_source1_data_property_value988, _, _, _, _, _) => {
                            source_ml_source1_data_property_value988.clone()
                        }
                    }
                })
            } else {
                None::<self::ml::Big>
            }
        };
        !(match (source_ml_source1_comparison_operand989.clone(), realm.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
    } || {
        let source_ml_source1_comparison_operand992 = {
            let source_ml_source1_optional_chain_receiver991 = (spreadable_key.clone(),);
            if match (source_ml_source1_optional_chain_receiver991.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                Some({
                    let source_ml_temporary_optional_chain144 = ((source_ml_source1_optional_chain_receiver991.clone()).0).clone().expect("a proven present value");
                    match source_ml_temporary_optional_chain144.clone() {
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
        !(match (source_ml_source1_comparison_operand992.clone(), self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
    }) {
        None::<self::JavaScriptCallFunction>
    } else {
        {
            let code = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
            if !(self::append_java_script_object_coercion(code.clone(), 0f64, 1f64, 2f64, 3f64, 4f64, 5f64, roots.clone())) {
                None::<self::JavaScriptCallFunction>
            } else {
                Some({
                    let source_ml_temporary_compile_java_script_array_concatenation_function_ignored145 = { let ml_method_owner160 = code.clone(); let ml_method_argument160_0 = self::JavaScriptCallInstruction::Constant(6f64, Box::new(spreadable_key.clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))); let ml_method_argument160_1 = {
                        let source_ml_source1_constructor_field994 = 7f64;
                        {
                            let source_ml_source1_constructor_field995 = Some(1f64);
                            {
                                let source_ml_source1_constructor_field996 = 6f64;
                                {
                                    let source_ml_source1_constructor_field997 = species.clone().unwrap_or_else(|| self::require_java_script_array_species_directory());
                                    self::JavaScriptCallInstruction::ArrayConcatenation(source_ml_source1_constructor_field994, Some((source_ml_source1_constructor_field995.clone(),)), source_ml_source1_constructor_field996, Box::new(source_ml_source1_constructor_field997.clone()))
                                }
                            }
                        }
                    }; let ml_method_argument160_2 = self::JavaScriptCallInstruction::Return(7f64); let ml_inserted = vec![ml_method_argument160_0, ml_method_argument160_1, ml_method_argument160_2]; let mut ml_method_values160 = ml_method_owner160.borrow_mut(); let ml_length = ml_method_values160.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values160.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_source1_constructor_field1001 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source1_constructor_field1002 = 0f64;
                            {
                                let source_ml_source1_constructor_field1003 = 8f64;
                                {
                                    let source_ml_source1_constructor_field1004 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                    self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1001.clone()), source_ml_source1_constructor_field1002, source_ml_source1_constructor_field1003, source_ml_source1_constructor_field1004.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                }
                            }
                        }
                    }
                })
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_array_slice_function(identity: self::ml::Big, realm: self::ml::Big, roots: Option<self::JavaScriptPrimitiveWrapperRoots>, species: Option<self::JavaScriptArraySpeciesDirectory>) -> Option<self::JavaScriptCallFunction> {
    if {
        let source_ml_source1_comparison_operand1008 = {
            let source_ml_source1_optional_chain_receiver1006 = (species.clone(),);
            if match (source_ml_source1_optional_chain_receiver1006.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                Some({
                    let source_ml_temporary_optional_chain146 = ((source_ml_source1_optional_chain_receiver1006.clone()).0).clone().expect("a proven present value");
                    match source_ml_temporary_optional_chain146.clone() {
                        self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(source_ml_source1_data_property_value1007, _, _, _, _, _) => {
                            source_ml_source1_data_property_value1007.clone()
                        }
                    }
                })
            } else {
                None::<self::ml::Big>
            }
        };
        !(match (source_ml_source1_comparison_operand1008.clone(), realm.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
    } {
        None::<self::JavaScriptCallFunction>
    } else {
        {
            let code = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
            if !(self::append_java_script_object_coercion(code.clone(), 0f64, 1f64, 2f64, 3f64, 4f64, 5f64, roots.clone())) {
                None::<self::JavaScriptCallFunction>
            } else {
                Some({
                    let source_ml_temporary_compile_java_script_array_slice_function_ignored147 = { let ml_method_owner161 = code.clone(); let ml_method_argument161_0 = {
                        let source_ml_source1_constructor_field1010 = 6f64;
                        {
                            let source_ml_source1_constructor_field1011 = Some(1f64);
                            {
                                let source_ml_source1_constructor_field1012 = species.clone().unwrap_or_else(|| self::require_java_script_array_species_directory());
                                self::JavaScriptCallInstruction::ArraySlice(source_ml_source1_constructor_field1010, Some((source_ml_source1_constructor_field1011.clone(),)), Box::new(source_ml_source1_constructor_field1012.clone()))
                            }
                        }
                    }; let ml_method_argument161_1 = self::JavaScriptCallInstruction::Return(6f64); let ml_inserted = vec![ml_method_argument161_0, ml_method_argument161_1]; let mut ml_method_values161 = ml_method_owner161.borrow_mut(); let ml_length = ml_method_values161.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values161.extend(ml_inserted); ml_length as f64 };
                    {
                        let source_ml_source1_constructor_field1016 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source1_constructor_field1017 = 0f64;
                            {
                                let source_ml_source1_constructor_field1018 = 7f64;
                                {
                                    let source_ml_source1_constructor_field1019 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                    self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1016.clone()), source_ml_source1_constructor_field1017, source_ml_source1_constructor_field1018, source_ml_source1_constructor_field1019.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                }
                            }
                        }
                    }
                })
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_object_assignment_function(identity: self::ml::Big, realm: self::ml::Big, roots: Option<self::JavaScriptPrimitiveWrapperRoots>) -> Option<self::JavaScriptCallFunction> {
    {
        let code = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64)]));
        if !(self::append_java_script_object_coercion(code.clone(), 1f64, 2f64, 3f64, 4f64, 5f64, 6f64, roots.clone())) {
            None::<self::JavaScriptCallFunction>
        } else {
            Some({
                let source_ml_temporary_compile_java_script_object_assignment_function_ignored148 = { let ml_method_owner162 = code.clone(); let ml_method_argument162_0 = self::JavaScriptCallInstruction::ArgumentCount(7f64); let ml_method_argument162_1 = self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))); let ml_method_argument162_2 = self::JavaScriptCallInstruction::Constant(9f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))); let ml_inserted = vec![ml_method_argument162_0, ml_method_argument162_1, ml_method_argument162_2]; let mut ml_method_values162 = ml_method_owner162.borrow_mut(); let ml_length = ml_method_values162.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values162.extend(ml_inserted); ml_length as f64 };
                {
                    let loop_ = { let ml_reference_array163 = code.clone(); let ml_reference_projection163 = ml_reference_array163.borrow().len() as f64; ml_reference_projection163 };
                    {
                        let source_ml_temporary_compile_java_script_object_assignment_function_ignored149 = { let ml_method_owner164 = code.clone(); let ml_method_argument164_0 = self::JavaScriptCallInstruction::NumberBinary(5f64, 8f64, 7f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])); let ml_method_argument164_1 = self::JavaScriptCallInstruction::Branch(5f64, (loop_ + 2f64), (loop_ + 6f64)); let ml_method_argument164_2 = self::JavaScriptCallInstruction::ArgumentAt(10f64, 8f64); let ml_method_argument164_3 = {
                            let source_ml_source1_constructor_field1021 = 11f64;
                            {
                                let source_ml_source1_constructor_field1022 = 2f64;
                                {
                                    let source_ml_source1_constructor_field1023 = 10f64;
                                    {
                                        let source_ml_source1_constructor_field1025 = Some(self::ml_string::Text::from_units(vec![97, 115, 115, 105, 103, 110, 109, 101, 110, 116]));
                                        self::JavaScriptCallInstruction::ObjectSpread(source_ml_source1_constructor_field1021, source_ml_source1_constructor_field1022, source_ml_source1_constructor_field1023, realm.clone(), None::<(Option<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)>, Some((source_ml_source1_constructor_field1025.clone(),)))
                                    }
                                }
                            }
                        }; let ml_method_argument164_4 = self::JavaScriptCallInstruction::NumberBinary(8f64, 8f64, 9f64, self::ml_string::Text::from_units(vec![97, 100, 100])); let ml_method_argument164_5 = self::JavaScriptCallInstruction::Jump(loop_); let ml_method_argument164_6 = self::JavaScriptCallInstruction::Return(2f64); let ml_inserted = vec![ml_method_argument164_0, ml_method_argument164_1, ml_method_argument164_2, ml_method_argument164_3, ml_method_argument164_4, ml_method_argument164_5, ml_method_argument164_6]; let mut ml_method_values164 = ml_method_owner164.borrow_mut(); let ml_length = ml_method_values164.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values164.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_source1_constructor_field1029 = self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source1_constructor_field1030 = 0f64;
                                {
                                    let source_ml_source1_constructor_field1031 = 12f64;
                                    {
                                        let source_ml_source1_constructor_field1032 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                        self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1029.clone()), source_ml_source1_constructor_field1030, source_ml_source1_constructor_field1031, source_ml_source1_constructor_field1032.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                                    }
                                }
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
pub fn test_java_script_array_mutation_operation(operation: &self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value165 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 112, 117, 115, 104]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 112, 111, 112]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 115, 104, 105, 102, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 117, 110, 115, 104, 105, 102, 116]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 114, 101, 118, 101, 114, 115, 101]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 105, 108, 108]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 99, 111, 112, 121, 87, 105, 116, 104, 105, 110])]))).borrow().clone(); ml_reference_value165 }, operation, 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_array_mutation_prefix() -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>> {
    std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Primitive(11f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32, 109, 101, 116, 104, 111, 100, 32, 111, 98, 106, 101, 99, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![0f64]))), self::JavaScriptCallInstruction::Constant(1f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64]))))), self::JavaScriptCallInstruction::OrdinaryGet(2f64, 0f64, 1f64, 0f64), self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))), self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 108, 101, 110, 103, 116, 104]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64])))]))
}
