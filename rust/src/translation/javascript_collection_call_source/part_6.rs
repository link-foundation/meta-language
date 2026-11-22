// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceSpan;
use super::JavaScriptPrimitiveWrapperRoots;
use super::JavaScriptSemanticValue;
use super::MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258;
use super::MlTemporaryRelocateJavaScriptArrayMutationInstructionLoop250Result;
use super::append_java_script_object_coercion;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::require_java_script_array_species_directory;
use super::source_ml_temporary_relocate_java_script_array_mutation_instruction_loop250;
use super::test_java_script_standard_array_callback_operation;
use super::test_java_script_standard_array_transform_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn relocate_java_script_array_mutation_instruction(instruction: self::JavaScriptCallInstruction, held: f64, offset: f64) -> Option<self::JavaScriptCallInstruction> {
    match instruction.clone() {
        self::JavaScriptCallInstruction::Constant(destination, value) => {
            let value = *value;
            Some(self::JavaScriptCallInstruction::Constant(destination, Box::new(value.clone())))
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            Some(self::JavaScriptCallInstruction::Copy(destination_2, self::replace_java_script_boxed_array_receiver_register(source, held)))
        }
        self::JavaScriptCallInstruction::Argument(destination_3, index) => {
            Some(self::JavaScriptCallInstruction::Argument(destination_3, index))
        }
        self::JavaScriptCallInstruction::ArgumentCount(destination_4) => {
            Some(self::JavaScriptCallInstruction::ArgumentCount(destination_4))
        }
        self::JavaScriptCallInstruction::ArgumentAt(destination_5, index_2) => {
            Some(self::JavaScriptCallInstruction::ArgumentAt(destination_5, self::replace_java_script_boxed_array_receiver_register(index_2, held)))
        }
        self::JavaScriptCallInstruction::FunctionBind(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::BoundFunctionCall(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::FunctionToString(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::PushFinally(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::FinishFinally => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::CompleteJump(_, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::Jump(target) => {
            Some(self::JavaScriptCallInstruction::Jump((target + offset)))
        }
        self::JavaScriptCallInstruction::Branch(condition, yes, no) => {
            Some(self::JavaScriptCallInstruction::Branch(self::replace_java_script_boxed_array_receiver_register(condition, held), (yes + offset), (no + offset)))
        }
        self::JavaScriptCallInstruction::Call(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArgumentListOpen(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArgumentListAppend(_, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::Construct(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ConstructorCheck(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ReadNewTarget(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArrayJoinLeave(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ConstructErrorClass(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::CallClassConstructor(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ConstructCollection(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::CollectionAllocate(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArgumentRest(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::CollectionOperation(_, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::PrimitiveMethod(_, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::CollectionForEach(_, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArraySlice(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArrayCreation(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArrayCallback(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArraySearch(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArrayTransform(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ObjectEnumeration(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::PropertyDefinitions(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::PropertyReflection(_, _, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::InstanceOf(_, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ObjectObservation(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::CollectionCopy(_, _, _, _, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ReadDerivedReceiver(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::InvokeErrorIntrinsic(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::InvokeHostCapability(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            Some(self::JavaScriptCallInstruction::Return(self::replace_java_script_boxed_array_receiver_register(source_2, held)))
        }
        self::JavaScriptCallInstruction::Throw(_) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::PushHandler(_, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::PopHandler => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_6, left, right, operator) => {
            Some(self::JavaScriptCallInstruction::NumberBinary(destination_6, self::replace_java_script_boxed_array_receiver_register(left, held), self::replace_java_script_boxed_array_receiver_register(right, held), operator.clone()))
        }
        self::JavaScriptCallInstruction::SameValue(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::StrictEqual(destination_7, left_2, right_2) => {
            Some(self::JavaScriptCallInstruction::StrictEqual(destination_7, self::replace_java_script_boxed_array_receiver_register(left_2, held), self::replace_java_script_boxed_array_receiver_register(right_2, held)))
        }
        self::JavaScriptCallInstruction::RequireWrite(success) => {
            Some(self::JavaScriptCallInstruction::RequireWrite(self::replace_java_script_boxed_array_receiver_register(success, held)))
        }
        self::JavaScriptCallInstruction::Primitive(destination_8, operation, arguments) => {
            {
                let operands = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                {
                    let source_ml_temporary_index249 = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_s1293 = self::source_ml_temporary_relocate_java_script_array_mutation_instruction_loop250(held, operands.clone(), arguments.clone(), source_ml_temporary_index249.clone());
                        match source_ml_source1_s1293.clone() {
                            self::MlTemporaryRelocateJavaScriptArrayMutationInstructionLoop250Result::MlTemporaryRelocateJavaScriptArrayMutationInstructionLoop250Done => {
                                Some(self::JavaScriptCallInstruction::Primitive(destination_8, operation.clone(), operands.clone()))
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallInstruction::OrdinaryAllocate(_, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::LiteralPrototype(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::LiteralAccessor(_, _, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::SymbolAllocate(_, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::SymbolDescription(_, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::SymbolRegistry(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::FunctionName(_, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArrayAllocate(_, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::OrdinaryDataProperty(_, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::OrdinaryGet(destination_9, object, key, receiver) => {
            Some(self::JavaScriptCallInstruction::OrdinaryGet(destination_9, self::replace_java_script_boxed_array_receiver_register(object, held), self::replace_java_script_boxed_array_receiver_register(key, held), self::replace_java_script_boxed_array_receiver_register(receiver, held)))
        }
        self::JavaScriptCallInstruction::OrdinaryDelete(destination_10, object_2, key_2) => {
            Some(self::JavaScriptCallInstruction::OrdinaryDelete(destination_10, self::replace_java_script_boxed_array_receiver_register(object_2, held), self::replace_java_script_boxed_array_receiver_register(key_2, held)))
        }
        self::JavaScriptCallInstruction::OrdinaryHas(destination_11, object_3, key_3) => {
            Some(self::JavaScriptCallInstruction::OrdinaryHas(destination_11, self::replace_java_script_boxed_array_receiver_register(object_3, held), self::replace_java_script_boxed_array_receiver_register(key_3, held)))
        }
        self::JavaScriptCallInstruction::OrdinarySet(destination_12, object_4, key_4, value_2, receiver_2) => {
            Some(self::JavaScriptCallInstruction::OrdinarySet(destination_12, self::replace_java_script_boxed_array_receiver_register(object_4, held), self::replace_java_script_boxed_array_receiver_register(key_4, held), self::replace_java_script_boxed_array_receiver_register(value_2, held), self::replace_java_script_boxed_array_receiver_register(receiver_2, held)))
        }
        self::JavaScriptCallInstruction::OrdinaryDefine(_, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_13, input, realm) => {
            Some(self::JavaScriptCallInstruction::PropertyKey(destination_13, self::replace_java_script_boxed_array_receiver_register(input, held), realm.clone()))
        }
        self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _, _) => {
            None::<self::JavaScriptCallInstruction>
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace_java_script_boxed_array_receiver_register(register: f64, held: f64) -> f64 {
    if (register == (0f64)) {
        held
    } else {
        register
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_search_operation(operation: &self::ml_string::Text) -> bool {
    self::ml_array::includes(&{ let ml_reference_value318 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 105, 110, 99, 108, 117, 100, 101, 115]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 105, 110, 100, 101, 120, 79, 102]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102]), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 97, 116])]))).borrow().clone(); ml_reference_value318 }, operation, 0f64, false, |a, b| a == b)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_boxed_array_read_function(identity: self::ml::Big, realm: self::ml::Big, operation: &self::ml_string::Text, roots: Option<self::JavaScriptPrimitiveWrapperRoots>, species: Option<self::JavaScriptArraySpeciesDirectory>) -> Option<self::JavaScriptCallFunction> {
    {
        let callback = self::test_java_script_standard_array_callback_operation(operation);
        {
            let search = self::test_java_script_array_search_operation(operation);
            {
                let transform = self::test_java_script_standard_array_transform_operation(operation);
                if (((!callback && !search) && !transform) || (transform && {
                    let source_ml_source1_comparison_operand1469 = {
                        let source_ml_source1_optional_chain_receiver1467 = (species.clone(),);
                        if match (source_ml_source1_optional_chain_receiver1467.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            Some({
                                let source_ml_temporary_optional_chain257 = ((source_ml_source1_optional_chain_receiver1467.clone()).0).clone().expect("a proven present value");
                                match source_ml_temporary_optional_chain257.clone() {
                                    self::JavaScriptArraySpeciesDirectory::ArraySpeciesDirectory(source_ml_source1_data_property_value1468, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value1468.clone()
                                    }
                                }
                            })
                        } else {
                            None::<self::ml::Big>
                        }
                    };
                    !(match (source_ml_source1_comparison_operand1469.clone(), realm.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                })) {
                    None::<self::JavaScriptCallFunction>
                } else {
                    {
                        let code = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
                        if !(self::append_java_script_object_coercion(code.clone(), 0f64, 1f64, 2f64, 3f64, 4f64, 5f64, roots.clone())) {
                            None::<self::JavaScriptCallFunction>
                        } else {
                            {
                                let source_ml_source1_s1481 = if (transform && !(match (species.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                    {
                                        let source_ml_temporary_compile_java_script_boxed_array_read_function_ignored260 = { let ml_method_owner319 = code.clone(); let ml_method_argument319_0 = {
                                            let source_ml_source1_constructor_field1482 = 6f64;
                                            {
                                                let source_ml_source1_constructor_field1483 = Some(1f64);
                                                {
                                                    let source_ml_source1_constructor_field1485 = species.clone().unwrap_or_else(|| self::require_java_script_array_species_directory());
                                                    self::JavaScriptCallInstruction::ArrayTransform(source_ml_source1_constructor_field1482, operation.clone(), Box::new(source_ml_source1_constructor_field1485.clone()), Some((source_ml_source1_constructor_field1483.clone(),)))
                                                }
                                            }
                                        }; let ml_inserted = vec![ml_method_argument319_0]; let mut ml_method_values319 = ml_method_owner319.borrow_mut(); let ml_length = ml_method_values319.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values319.extend(ml_inserted); ml_length as f64 };
                                        self::MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258::MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258Next
                                    }
                                } else {
                                    if callback {
                                        {
                                            let source_ml_temporary_compile_java_script_boxed_array_read_function_ignored261 = { let ml_method_owner320 = code.clone(); let ml_method_argument320_0 = {
                                                let source_ml_source1_constructor_field1486 = 6f64;
                                                {
                                                    let source_ml_source1_constructor_field1487 = Some(1f64);
                                                    self::JavaScriptCallInstruction::ArrayCallback(source_ml_source1_constructor_field1486, operation.clone(), Some((source_ml_source1_constructor_field1487.clone(),)))
                                                }
                                            }; let ml_inserted = vec![ml_method_argument320_0]; let mut ml_method_values320 = ml_method_owner320.borrow_mut(); let ml_length = ml_method_values320.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values320.extend(ml_inserted); ml_length as f64 };
                                            self::MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258::MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258Next
                                        }
                                    } else {
                                        {
                                            let source_ml_temporary_compile_java_script_boxed_array_read_function_ignored262 = { let ml_method_owner321 = code.clone(); let ml_method_argument321_0 = {
                                                let source_ml_source1_constructor_field1489 = 6f64;
                                                {
                                                    let source_ml_source1_constructor_field1490 = Some(1f64);
                                                    self::JavaScriptCallInstruction::ArraySearch(source_ml_source1_constructor_field1489, operation.clone(), Some((source_ml_source1_constructor_field1490.clone(),)))
                                                }
                                            }; let ml_inserted = vec![ml_method_argument321_0]; let mut ml_method_values321 = ml_method_owner321.borrow_mut(); let ml_length = ml_method_values321.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values321.extend(ml_inserted); ml_length as f64 };
                                            self::MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258::MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258Next
                                        }
                                    }
                                };
                                match source_ml_source1_s1481.clone() {
                                    self::MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258::MlTemporaryCompileJavaScriptBoxedArrayReadFunctionJoin258Next => {
                                        Some({
                                            let source_ml_temporary_compile_java_script_boxed_array_read_function_ignored259 = { let ml_method_owner322 = code.clone(); let ml_method_argument322_0 = self::JavaScriptCallInstruction::Return(6f64); let ml_inserted = vec![ml_method_argument322_0]; let mut ml_method_values322 = ml_method_owner322.borrow_mut(); let ml_length = ml_method_values322.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values322.extend(ml_inserted); ml_length as f64 };
                                            {
                                                let source_ml_source1_constructor_field1495 = self::JavaScriptSemanticValue::Undefined;
                                                {
                                                    let source_ml_source1_constructor_field1496 = 0f64;
                                                    {
                                                        let source_ml_source1_constructor_field1497 = 7f64;
                                                        {
                                                            let source_ml_source1_constructor_field1498 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                            self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1495.clone()), source_ml_source1_constructor_field1496, source_ml_source1_constructor_field1497, source_ml_source1_constructor_field1498.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_join_operation(operation: &self::ml_string::Text) -> bool {
    ((*operation) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 106, 111, 105, 110])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_array_join_function(identity: self::ml::Big, realm: self::ml::Big, roots: Option<self::JavaScriptPrimitiveWrapperRoots>) -> Option<self::JavaScriptCallFunction> {
    {
        let code = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
        if !(self::append_java_script_object_coercion(code.clone(), 0f64, 1f64, 7f64, 14f64, 8f64, 13f64, roots.clone())) {
            None::<self::JavaScriptCallFunction>
        } else {
            Some({
                let source_ml_temporary_compile_java_script_array_join_function_ignored263 = { let ml_method_owner323 = code.clone(); let ml_method_argument323_0 = self::JavaScriptCallInstruction::Constant(7f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64]))))); let ml_method_argument323_1 = self::JavaScriptCallInstruction::OrdinaryGet(2f64, 1f64, 7f64, 1f64); let ml_method_argument323_2 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument323_3 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 108, 101, 110, 103, 116, 104]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument323_4 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 86, 56, 32, 97, 114, 114, 97, 121, 32, 106, 111, 105, 110, 32, 108, 101, 110, 103, 116, 104]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))); let ml_method_argument323_5 = self::JavaScriptCallInstruction::Argument(3f64, 0f64); let ml_method_argument323_6 = self::JavaScriptCallInstruction::Constant(11f64, Box::new(self::JavaScriptSemanticValue::Undefined)); let ml_method_argument323_7 = self::JavaScriptCallInstruction::StrictEqual(8f64, 3f64, 11f64); let ml_inserted = vec![ml_method_argument323_0, ml_method_argument323_1, ml_method_argument323_2, ml_method_argument323_3, ml_method_argument323_4, ml_method_argument323_5, ml_method_argument323_6, ml_method_argument323_7]; let mut ml_method_values323 = ml_method_owner323.borrow_mut(); let ml_length = ml_method_values323.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values323.extend(ml_inserted); ml_length as f64 };
                {
                    let separator_branch = { let ml_reference_array324 = code.clone(); let ml_reference_projection324 = ml_reference_array324.borrow().len() as f64; ml_reference_projection324 };
                    {
                        let source_ml_temporary_compile_java_script_array_join_function_ignored264 = { let ml_method_owner325 = code.clone(); let ml_method_argument325_0 = self::JavaScriptCallInstruction::Branch(8f64, 0f64, (separator_branch + 1f64)); let ml_method_argument325_1 = self::JavaScriptCallInstruction::Primitive(3f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![3f64]))); let ml_inserted = vec![ml_method_argument325_0, ml_method_argument325_1]; let mut ml_method_values325 = ml_method_owner325.borrow_mut(); let ml_length = ml_method_values325.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values325.extend(ml_inserted); ml_length as f64 };
                        {
                            let separator_done = { let ml_reference_array326 = code.clone(); let ml_reference_projection326 = ml_reference_array326.borrow().len() as f64; ml_reference_projection326 };
                            {
                                let source_ml_temporary_compile_java_script_array_join_function_ignored265 = { let ml_method_owner327 = code.clone(); let ml_method_argument327_0 = self::JavaScriptCallInstruction::Jump(0f64); let ml_inserted = vec![ml_method_argument327_0]; let mut ml_method_values327 = ml_method_owner327.borrow_mut(); let ml_length = ml_method_values327.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values327.extend(ml_inserted); ml_length as f64 };
                                {
                                    let comma = { let ml_reference_array328 = code.clone(); let ml_reference_projection328 = ml_reference_array328.borrow().len() as f64; ml_reference_projection328 };
                                    {
                                        let source_ml_temporary_compile_java_script_array_join_function_ignored266 = { let ml_method_owner329 = code.clone(); let ml_method_argument329_0 = self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![44f64]))))); let ml_inserted = vec![ml_method_argument329_0]; let mut ml_method_values329 = ml_method_owner329.borrow_mut(); let ml_length = ml_method_values329.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values329.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let ready = { let ml_reference_array330 = code.clone(); let ml_reference_projection330 = ml_reference_array330.borrow().len() as f64; ml_reference_projection330 };
                                            {
                                                let source_ml_temporary_compile_java_script_array_join_function_ignored267 = { let ml_array_owner331 = code.clone(); let ml_array_argument331_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(8f64, comma, (separator_branch + 1f64)); let ml_array_argument331_1: f64 = separator_branch; let ml_array_argument331_2: f64 = (separator_branch + 1f64); let ml_array_length = ml_array_owner331.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument331_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument331_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner331.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument331_0); } } ml_array_owner331 };
                                                {
                                                    let source_ml_temporary_compile_java_script_array_join_function_ignored268 = { let ml_array_owner332 = code.clone(); let ml_array_argument332_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Jump(ready); let ml_array_argument332_1: f64 = separator_done; let ml_array_argument332_2: f64 = (separator_done + 1f64); let ml_array_length = ml_array_owner332.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument332_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument332_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner332.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument332_0); } } ml_array_owner332 };
                                                    {
                                                        let source_ml_temporary_compile_java_script_array_join_function_ignored269 = { let ml_method_owner333 = code.clone(); let ml_method_argument333_0 = self::JavaScriptCallInstruction::Constant(5f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))))); let ml_method_argument333_1 = self::JavaScriptCallInstruction::Constant(9f64, Box::new(self::JavaScriptSemanticValue::Number(0f64))); let ml_method_argument333_2 = self::JavaScriptCallInstruction::Constant(10f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))); let ml_method_argument333_3 = self::JavaScriptCallInstruction::Constant(12f64, Box::new(self::JavaScriptSemanticValue::Null)); let ml_method_argument333_4 = self::JavaScriptCallInstruction::Copy(4f64, 9f64); let ml_method_argument333_5 = self::JavaScriptCallInstruction::NumberBinary(8f64, 2f64, 9f64, self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])); let ml_inserted = vec![ml_method_argument333_0, ml_method_argument333_1, ml_method_argument333_2, ml_method_argument333_3, ml_method_argument333_4, ml_method_argument333_5]; let mut ml_method_values333 = ml_method_owner333.borrow_mut(); let ml_length = ml_method_values333.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values333.extend(ml_inserted); ml_length as f64 };
                                                        {
                                                            let empty = { let ml_reference_array334 = code.clone(); let ml_reference_projection334 = ml_reference_array334.borrow().len() as f64; ml_reference_projection334 };
                                                            {
                                                                let source_ml_temporary_compile_java_script_array_join_function_ignored270 = { let ml_method_owner335 = code.clone(); let ml_method_argument335_0 = self::JavaScriptCallInstruction::Branch(8f64, 0f64, (empty + 1f64)); let ml_method_argument335_1 = self::JavaScriptCallInstruction::ArrayJoinEnter(14f64, 1f64); let ml_inserted = vec![ml_method_argument335_0, ml_method_argument335_1]; let mut ml_method_values335 = ml_method_owner335.borrow_mut(); let ml_length = ml_method_values335.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values335.extend(ml_inserted); ml_length as f64 };
                                                                {
                                                                    let guard = { let ml_reference_array336 = code.clone(); let ml_reference_projection336 = ml_reference_array336.borrow().len() as f64; ml_reference_projection336 };
                                                                    {
                                                                        let source_ml_temporary_compile_java_script_array_join_function_ignored271 = { let ml_method_owner337 = code.clone(); let ml_method_argument337_0 = self::JavaScriptCallInstruction::Branch(14f64, (guard + 1f64), 0f64); let ml_inserted = vec![ml_method_argument337_0]; let mut ml_method_values337 = ml_method_owner337.borrow_mut(); let ml_length = ml_method_values337.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values337.extend(ml_inserted); ml_length as f64 };
                                                                        {
                                                                            let loop_ = { let ml_reference_array338 = code.clone(); let ml_reference_projection338 = ml_reference_array338.borrow().len() as f64; ml_reference_projection338 };
                                                                            {
                                                                                let source_ml_temporary_compile_java_script_array_join_function_ignored272 = { let ml_method_owner339 = code.clone(); let ml_method_argument339_0 = self::JavaScriptCallInstruction::NumberBinary(8f64, 4f64, 2f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])); let ml_inserted = vec![ml_method_argument339_0]; let mut ml_method_values339 = ml_method_owner339.borrow_mut(); let ml_length = ml_method_values339.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values339.extend(ml_inserted); ml_length as f64 };
                                                                                {
                                                                                    let exhausted = { let ml_reference_array340 = code.clone(); let ml_reference_projection340 = ml_reference_array340.borrow().len() as f64; ml_reference_projection340 };
                                                                                    {
                                                                                        let source_ml_temporary_compile_java_script_array_join_function_ignored273 = { let ml_method_owner341 = code.clone(); let ml_method_argument341_0 = self::JavaScriptCallInstruction::Branch(8f64, (exhausted + 1f64), 0f64); let ml_method_argument341_1 = self::JavaScriptCallInstruction::NumberBinary(8f64, 4f64, 9f64, self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])); let ml_inserted = vec![ml_method_argument341_0, ml_method_argument341_1]; let mut ml_method_values341 = ml_method_owner341.borrow_mut(); let ml_length = ml_method_values341.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values341.extend(ml_inserted); ml_length as f64 };
                                                                                        {
                                                                                            let first = { let ml_reference_array342 = code.clone(); let ml_reference_projection342 = ml_reference_array342.borrow().len() as f64; ml_reference_projection342 };
                                                                                            {
                                                                                                let source_ml_temporary_compile_java_script_array_join_function_ignored274 = { let ml_method_owner343 = code.clone(); let ml_method_argument343_0 = self::JavaScriptCallInstruction::Branch(8f64, (first + 2f64), (first + 1f64)); let ml_method_argument343_1 = self::JavaScriptCallInstruction::Primitive(5f64, self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![5f64, 3f64]))); let ml_method_argument343_2 = self::JavaScriptCallInstruction::PropertyKey(7f64, 4f64, realm.clone()); let ml_method_argument343_3 = self::JavaScriptCallInstruction::OrdinaryGet(6f64, 1f64, 7f64, 1f64); let ml_method_argument343_4 = self::JavaScriptCallInstruction::StrictEqual(8f64, 6f64, 11f64); let ml_inserted = vec![ml_method_argument343_0, ml_method_argument343_1, ml_method_argument343_2, ml_method_argument343_3, ml_method_argument343_4]; let mut ml_method_values343 = ml_method_owner343.borrow_mut(); let ml_length = ml_method_values343.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values343.extend(ml_inserted); ml_length as f64 };
                                                                                                {
                                                                                                    let undefined_element = { let ml_reference_array344 = code.clone(); let ml_reference_projection344 = ml_reference_array344.borrow().len() as f64; ml_reference_projection344 };
                                                                                                    {
                                                                                                        let source_ml_temporary_compile_java_script_array_join_function_ignored275 = { let ml_method_owner345 = code.clone(); let ml_method_argument345_0 = self::JavaScriptCallInstruction::Branch(8f64, 0f64, (undefined_element + 1f64)); let ml_method_argument345_1 = self::JavaScriptCallInstruction::StrictEqual(8f64, 6f64, 12f64); let ml_inserted = vec![ml_method_argument345_0, ml_method_argument345_1]; let mut ml_method_values345 = ml_method_owner345.borrow_mut(); let ml_length = ml_method_values345.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values345.extend(ml_inserted); ml_length as f64 };
                                                                                                        {
                                                                                                            let null_element = { let ml_reference_array346 = code.clone(); let ml_reference_projection346 = ml_reference_array346.borrow().len() as f64; ml_reference_projection346 };
                                                                                                            {
                                                                                                                let source_ml_temporary_compile_java_script_array_join_function_ignored276 = { let ml_method_owner347 = code.clone(); let ml_method_argument347_0 = self::JavaScriptCallInstruction::Branch(8f64, 0f64, (null_element + 1f64)); let ml_method_argument347_1 = self::JavaScriptCallInstruction::Primitive(6f64, self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![6f64]))); let ml_method_argument347_2 = self::JavaScriptCallInstruction::Primitive(5f64, self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![5f64, 6f64]))); let ml_inserted = vec![ml_method_argument347_0, ml_method_argument347_1, ml_method_argument347_2]; let mut ml_method_values347 = ml_method_owner347.borrow_mut(); let ml_length = ml_method_values347.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values347.extend(ml_inserted); ml_length as f64 };
                                                                                                                {
                                                                                                                    let advance = { let ml_reference_array348 = code.clone(); let ml_reference_projection348 = ml_reference_array348.borrow().len() as f64; ml_reference_projection348 };
                                                                                                                    {
                                                                                                                        let source_ml_temporary_compile_java_script_array_join_function_ignored277 = { let ml_method_owner349 = code.clone(); let ml_method_argument349_0 = self::JavaScriptCallInstruction::NumberBinary(4f64, 4f64, 10f64, self::ml_string::Text::from_units(vec![97, 100, 100])); let ml_method_argument349_1 = self::JavaScriptCallInstruction::Jump(loop_); let ml_inserted = vec![ml_method_argument349_0, ml_method_argument349_1]; let mut ml_method_values349 = ml_method_owner349.borrow_mut(); let ml_length = ml_method_values349.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values349.extend(ml_inserted); ml_length as f64 };
                                                                                                                        {
                                                                                                                            let leave = { let ml_reference_array350 = code.clone(); let ml_reference_projection350 = ml_reference_array350.borrow().len() as f64; ml_reference_projection350 };
                                                                                                                            {
                                                                                                                                let source_ml_temporary_compile_java_script_array_join_function_ignored278 = { let ml_method_owner351 = code.clone(); let ml_method_argument351_0 = self::JavaScriptCallInstruction::ArrayJoinLeave(1f64); let ml_inserted = vec![ml_method_argument351_0]; let mut ml_method_values351 = ml_method_owner351.borrow_mut(); let ml_length = ml_method_values351.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values351.extend(ml_inserted); ml_length as f64 };
                                                                                                                                {
                                                                                                                                    let finish = { let ml_reference_array352 = code.clone(); let ml_reference_projection352 = ml_reference_array352.borrow().len() as f64; ml_reference_projection352 };
                                                                                                                                    {
                                                                                                                                        let source_ml_temporary_compile_java_script_array_join_function_ignored279 = { let ml_method_owner353 = code.clone(); let ml_method_argument353_0 = self::JavaScriptCallInstruction::Return(5f64); let ml_inserted = vec![ml_method_argument353_0]; let mut ml_method_values353 = ml_method_owner353.borrow_mut(); let ml_length = ml_method_values353.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values353.extend(ml_inserted); ml_length as f64 };
                                                                                                                                        {
                                                                                                                                            let source_ml_temporary_compile_java_script_array_join_function_ignored280 = { let ml_array_owner354 = code.clone(); let ml_array_argument354_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(8f64, finish, (empty + 1f64)); let ml_array_argument354_1: f64 = empty; let ml_array_argument354_2: f64 = (empty + 1f64); let ml_array_length = ml_array_owner354.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument354_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument354_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner354.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument354_0); } } ml_array_owner354 };
                                                                                                                                            {
                                                                                                                                                let source_ml_temporary_compile_java_script_array_join_function_ignored281 = { let ml_array_owner355 = code.clone(); let ml_array_argument355_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(14f64, (guard + 1f64), finish); let ml_array_argument355_1: f64 = guard; let ml_array_argument355_2: f64 = (guard + 1f64); let ml_array_length = ml_array_owner355.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument355_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument355_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner355.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument355_0); } } ml_array_owner355 };
                                                                                                                                                {
                                                                                                                                                    let source_ml_temporary_compile_java_script_array_join_function_ignored282 = { let ml_array_owner356 = code.clone(); let ml_array_argument356_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(8f64, (exhausted + 1f64), leave); let ml_array_argument356_1: f64 = exhausted; let ml_array_argument356_2: f64 = (exhausted + 1f64); let ml_array_length = ml_array_owner356.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument356_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument356_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner356.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument356_0); } } ml_array_owner356 };
                                                                                                                                                    {
                                                                                                                                                        let source_ml_temporary_compile_java_script_array_join_function_ignored283 = { let ml_array_owner357 = code.clone(); let ml_array_argument357_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(8f64, advance, (undefined_element + 1f64)); let ml_array_argument357_1: f64 = undefined_element; let ml_array_argument357_2: f64 = (undefined_element + 1f64); let ml_array_length = ml_array_owner357.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument357_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument357_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner357.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument357_0); } } ml_array_owner357 };
                                                                                                                                                        {
                                                                                                                                                            let source_ml_temporary_compile_java_script_array_join_function_ignored284 = { let ml_array_owner358 = code.clone(); let ml_array_argument358_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(8f64, advance, (null_element + 1f64)); let ml_array_argument358_1: f64 = null_element; let ml_array_argument358_2: f64 = (null_element + 1f64); let ml_array_length = ml_array_owner358.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument358_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument358_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner358.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument358_0); } } ml_array_owner358 };
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source1_constructor_field1503 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source1_constructor_field1504 = 0f64;
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source1_constructor_field1505 = 15f64;
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source1_constructor_field1506 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                                                                                                                            self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1503.clone()), source_ml_source1_constructor_field1504, source_ml_source1_constructor_field1505, source_ml_source1_constructor_field1506.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
            })
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_object_string_function(identity: self::ml::Big, realm: self::ml::Big, roots: Option<self::JavaScriptPrimitiveWrapperRoots>, tag_key: Option<self::JavaScriptSemanticValue>) -> Option<self::JavaScriptCallFunction> {
    if {
        let source_ml_source1_comparison_operand1509 = {
            let source_ml_source1_optional_chain_receiver1508 = (tag_key.clone(),);
            if match (source_ml_source1_optional_chain_receiver1508.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                Some({
                    let source_ml_temporary_optional_chain285 = ((source_ml_source1_optional_chain_receiver1508.clone()).0).clone().expect("a proven present value");
                    match source_ml_temporary_optional_chain285.clone() {
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
        !(match (source_ml_source1_comparison_operand1509.clone(), self::ml_string::Text::from_units(vec![115, 121, 109, 98, 111, 108])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
    } {
        None::<self::JavaScriptCallFunction>
    } else {
        {
            let code = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Undefined)), self::JavaScriptCallInstruction::StrictEqual(3f64, 0f64, 2f64), self::JavaScriptCallInstruction::Branch(3f64, 0f64, 3f64), self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Null)), self::JavaScriptCallInstruction::StrictEqual(3f64, 0f64, 2f64), self::JavaScriptCallInstruction::Branch(3f64, 0f64, 6f64)]));
            if !(self::append_java_script_object_coercion(code.clone(), 0f64, 1f64, 2f64, 3f64, 4f64, 5f64, roots.clone())) {
                None::<self::JavaScriptCallFunction>
            } else {
                Some({
                    let source_ml_temporary_compile_java_script_object_string_function_ignored286 = { let ml_method_owner359 = code.clone(); let ml_method_argument359_0 = self::JavaScriptCallInstruction::ObjectObservation(6f64, self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 98, 117, 105, 108, 116, 105, 110, 84, 97, 103]), 1f64, 1f64); let ml_method_argument359_1 = self::JavaScriptCallInstruction::Constant(2f64, Box::new(tag_key.clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))); let ml_method_argument359_2 = self::JavaScriptCallInstruction::OrdinaryGet(7f64, 1f64, 2f64, 1f64); let ml_method_argument359_3 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![7f64]))); let ml_method_argument359_4 = self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![115f64, 116f64, 114f64, 105f64, 110f64, 103f64]))))); let ml_method_argument359_5 = self::JavaScriptCallInstruction::StrictEqual(4f64, 2f64, 3f64); let ml_inserted = vec![ml_method_argument359_0, ml_method_argument359_1, ml_method_argument359_2, ml_method_argument359_3, ml_method_argument359_4, ml_method_argument359_5]; let mut ml_method_values359 = ml_method_owner359.borrow_mut(); let ml_length = ml_method_values359.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values359.extend(ml_inserted); ml_length as f64 };
                    {
                        let tag_branch = { let ml_reference_array360 = code.clone(); let ml_reference_projection360 = ml_reference_array360.borrow().len() as f64; ml_reference_projection360 };
                        {
                            let source_ml_temporary_compile_java_script_object_string_function_ignored287 = { let ml_method_owner361 = code.clone(); let ml_method_argument361_0 = self::JavaScriptCallInstruction::Branch(4f64, (tag_branch + 1f64), (tag_branch + 2f64)); let ml_method_argument361_1 = self::JavaScriptCallInstruction::Copy(6f64, 7f64); let ml_method_argument361_2 = self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![91f64, 111f64, 98f64, 106f64, 101f64, 99f64, 116f64, 32f64]))))); let ml_method_argument361_3 = self::JavaScriptCallInstruction::Primitive(6f64, self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64, 6f64]))); let ml_method_argument361_4 = self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![93f64]))))); let ml_method_argument361_5 = self::JavaScriptCallInstruction::Primitive(6f64, self::ml_string::Text::from_units(vec![99, 111, 110, 99, 97, 116, 101, 110, 97, 116, 101, 32, 116, 101, 120, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![6f64, 2f64]))); let ml_method_argument361_6 = self::JavaScriptCallInstruction::Return(6f64); let ml_inserted = vec![ml_method_argument361_0, ml_method_argument361_1, ml_method_argument361_2, ml_method_argument361_3, ml_method_argument361_4, ml_method_argument361_5, ml_method_argument361_6]; let mut ml_method_values361 = ml_method_owner361.borrow_mut(); let ml_length = ml_method_values361.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values361.extend(ml_inserted); ml_length as f64 };
                            {
                                let absent = { let ml_reference_array362 = code.clone(); let ml_reference_projection362 = ml_reference_array362.borrow().len() as f64; ml_reference_projection362 };
                                {
                                    let source_ml_temporary_compile_java_script_object_string_function_ignored288 = { let ml_method_owner363 = code.clone(); let ml_method_argument363_0 = self::JavaScriptCallInstruction::Constant(6f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![91f64, 111f64, 98f64, 106f64, 101f64, 99f64, 116f64, 32f64, 85f64, 110f64, 100f64, 101f64, 102f64, 105f64, 110f64, 101f64, 100f64, 93f64]))))); let ml_method_argument363_1 = self::JavaScriptCallInstruction::Return(6f64); let ml_inserted = vec![ml_method_argument363_0, ml_method_argument363_1]; let mut ml_method_values363 = ml_method_owner363.borrow_mut(); let ml_length = ml_method_values363.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values363.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let nullish = { let ml_reference_array364 = code.clone(); let ml_reference_projection364 = ml_reference_array364.borrow().len() as f64; ml_reference_projection364 };
                                        {
                                            let source_ml_temporary_compile_java_script_object_string_function_ignored289 = { let ml_method_owner365 = code.clone(); let ml_method_argument365_0 = self::JavaScriptCallInstruction::Constant(6f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![91f64, 111f64, 98f64, 106f64, 101f64, 99f64, 116f64, 32f64, 78f64, 117f64, 108f64, 108f64, 93f64]))))); let ml_method_argument365_1 = self::JavaScriptCallInstruction::Return(6f64); let ml_inserted = vec![ml_method_argument365_0, ml_method_argument365_1]; let mut ml_method_values365 = ml_method_owner365.borrow_mut(); let ml_length = ml_method_values365.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values365.extend(ml_inserted); ml_length as f64 };
                                            {
                                                let source_ml_temporary_compile_java_script_object_string_function_ignored290 = { let ml_array_owner366 = code.clone(); let ml_array_argument366_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(3f64, absent, 3f64); let ml_array_argument366_1: f64 = 2f64; let ml_array_argument366_2: f64 = (2f64 + 1f64); let ml_array_length = ml_array_owner366.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument366_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument366_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner366.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument366_0); } } ml_array_owner366 };
                                                {
                                                    let source_ml_temporary_compile_java_script_object_string_function_ignored291 = { let ml_array_owner367 = code.clone(); let ml_array_argument367_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(3f64, nullish, 6f64); let ml_array_argument367_1: f64 = 5f64; let ml_array_argument367_2: f64 = (5f64 + 1f64); let ml_array_length = ml_array_owner367.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument367_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument367_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner367.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument367_0); } } ml_array_owner367 };
                                                    {
                                                        let source_ml_source1_constructor_field1514 = self::JavaScriptSemanticValue::Undefined;
                                                        {
                                                            let source_ml_source1_constructor_field1515 = 0f64;
                                                            {
                                                                let source_ml_source1_constructor_field1516 = 8f64;
                                                                {
                                                                    let source_ml_source1_constructor_field1517 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                    self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1514.clone()), source_ml_source1_constructor_field1515, source_ml_source1_constructor_field1516, source_ml_source1_constructor_field1517.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
                })
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_array_string_function(identity: self::ml::Big, realm: self::ml::Big, roots: Option<self::JavaScriptPrimitiveWrapperRoots>, object_string: Option<self::JavaScriptSemanticValue>) -> Option<self::JavaScriptCallFunction> {
    if ({
        let source_ml_source1_comparison_operand1520 = {
            let source_ml_source1_optional_chain_receiver1519 = (object_string.clone(),);
            if match (source_ml_source1_optional_chain_receiver1519.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                Some({
                    let source_ml_temporary_optional_chain292 = ((source_ml_source1_optional_chain_receiver1519.clone()).0).clone().expect("a proven present value");
                    match source_ml_temporary_optional_chain292.clone() {
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
        !(match (source_ml_source1_comparison_operand1520.clone(), self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
    } || !(match {
        let source_ml_source1_nullish_field_receiver1522 = (object_string.clone(),);
        if match (source_ml_source1_nullish_field_receiver1522.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
            {
                let source_ml_source1_nullish_field_present1523 = ((source_ml_source1_nullish_field_receiver1522.clone()).0).clone().expect("a proven present value");
                match source_ml_source1_nullish_field_present1523.clone() {
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
                    self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value1524) => {
                        Some(source_ml_source1_data_property_value1524)
                    }
                }
            }
        } else {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
        }
    } { Some(ml_present) => ml_present, None => false })) {
        None::<self::JavaScriptCallFunction>
    } else {
        {
            let code = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new()));
            if !(self::append_java_script_object_coercion(code.clone(), 0f64, 1f64, 2f64, 3f64, 4f64, 5f64, roots.clone())) {
                None::<self::JavaScriptCallFunction>
            } else {
                Some({
                    let source_ml_temporary_compile_java_script_array_string_function_ignored293 = { let ml_method_owner368 = code.clone(); let ml_method_argument368_0 = self::JavaScriptCallInstruction::Constant(2f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![106f64, 111f64, 105f64, 110f64]))))); let ml_method_argument368_1 = self::JavaScriptCallInstruction::OrdinaryGet(6f64, 1f64, 2f64, 1f64); let ml_method_argument368_2 = self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 121, 112, 101]), std::rc::Rc::new(std::cell::RefCell::new(vec![6f64]))); let ml_method_argument368_3 = self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![102f64, 117f64, 110f64, 99f64, 116f64, 105f64, 111f64, 110f64]))))); let ml_method_argument368_4 = self::JavaScriptCallInstruction::StrictEqual(4f64, 2f64, 3f64); let ml_inserted = vec![ml_method_argument368_0, ml_method_argument368_1, ml_method_argument368_2, ml_method_argument368_3, ml_method_argument368_4]; let mut ml_method_values368 = ml_method_owner368.borrow_mut(); let ml_length = ml_method_values368.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values368.extend(ml_inserted); ml_length as f64 };
                    {
                        let branch = { let ml_reference_array369 = code.clone(); let ml_reference_projection369 = ml_reference_array369.borrow().len() as f64; ml_reference_projection369 };
                        {
                            let source_ml_temporary_compile_java_script_array_string_function_ignored294 = { let ml_method_owner370 = code.clone(); let ml_method_argument370_0 = self::JavaScriptCallInstruction::Branch(4f64, (branch + 2f64), (branch + 1f64)); let ml_method_argument370_1 = self::JavaScriptCallInstruction::Constant(6f64, Box::new(object_string.clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))); let ml_method_argument370_2 = self::JavaScriptCallInstruction::Call(7f64, 6f64, 1f64, std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()))); let ml_method_argument370_3 = self::JavaScriptCallInstruction::Return(7f64); let ml_inserted = vec![ml_method_argument370_0, ml_method_argument370_1, ml_method_argument370_2, ml_method_argument370_3]; let mut ml_method_values370 = ml_method_owner370.borrow_mut(); let ml_length = ml_method_values370.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values370.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_source1_constructor_field1528 = self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source1_constructor_field1529 = 0f64;
                                    {
                                        let source_ml_source1_constructor_field1530 = 8f64;
                                        {
                                            let source_ml_source1_constructor_field1531 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                            self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1528.clone()), source_ml_source1_constructor_field1529, source_ml_source1_constructor_field1530, source_ml_source1_constructor_field1531.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_array_call_function(identity: self::ml::Big, realm: self::ml::Big) -> self::JavaScriptCallFunction {
    {
        let instructions = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Constant(1f64, Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), true))), self::JavaScriptCallInstruction::ArgumentCount(2f64), self::JavaScriptCallInstruction::Constant(3f64, Box::new(self::JavaScriptSemanticValue::Number(0f64))), self::JavaScriptCallInstruction::Constant(4f64, Box::new(self::JavaScriptSemanticValue::Number(1f64))), self::JavaScriptCallInstruction::ArgumentListOpen(0f64), self::JavaScriptCallInstruction::NumberBinary(5f64, 3f64, 2f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])), self::JavaScriptCallInstruction::Branch(5f64, 7f64, 11f64), self::JavaScriptCallInstruction::ArgumentAt(6f64, 3f64), self::JavaScriptCallInstruction::ArgumentListAppend(0f64, 6f64), self::JavaScriptCallInstruction::NumberBinary(3f64, 3f64, 4f64, self::ml_string::Text::from_units(vec![97, 100, 100])), self::JavaScriptCallInstruction::Jump(5f64), self::JavaScriptCallInstruction::ArgumentListCall(7f64, 0f64, 1f64, 1f64, self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])), self::JavaScriptCallInstruction::Return(7f64)]));
        {
            let source_ml_source1_constructor_field1536 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source1_constructor_field1537 = 0f64;
                {
                    let source_ml_source1_constructor_field1538 = 8f64;
                    {
                        let source_ml_source1_constructor_field1539 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                        self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1536.clone()), source_ml_source1_constructor_field1537, source_ml_source1_constructor_field1538, source_ml_source1_constructor_field1539.clone(), instructions.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_reflect_construction_function(identity: self::ml::Big, realm: self::ml::Big) -> self::JavaScriptCallFunction {
    {
        let code = std::rc::Rc::new(std::cell::RefCell::new(vec![self::JavaScriptCallInstruction::Argument(1f64, 0f64), self::JavaScriptCallInstruction::ConstructorCheck(1f64, 1f64, true), self::JavaScriptCallInstruction::Argument(2f64, 1f64), self::JavaScriptCallInstruction::ArgumentCount(4f64), self::JavaScriptCallInstruction::Constant(5f64, Box::new(self::JavaScriptSemanticValue::Number(3f64))), self::JavaScriptCallInstruction::NumberBinary(6f64, 4f64, 5f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])), self::JavaScriptCallInstruction::Branch(6f64, 7f64, 9f64), self::JavaScriptCallInstruction::Copy(3f64, 1f64), self::JavaScriptCallInstruction::Jump(10f64), self::JavaScriptCallInstruction::Argument(3f64, 2f64), self::JavaScriptCallInstruction::ConstructorCheck(3f64, 3f64, true), self::JavaScriptCallInstruction::Primitive(2f64, self::ml_string::Text::from_units(vec![114, 101, 113, 117, 105, 114, 101, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 79, 98, 106, 101, 99, 116]), std::rc::Rc::new(std::cell::RefCell::new(vec![2f64]))), self::JavaScriptCallInstruction::Constant(6f64, Box::new(self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![108f64, 101f64, 110f64, 103f64, 116f64, 104f64]))))), self::JavaScriptCallInstruction::OrdinaryGet(5f64, 2f64, 6f64, 2f64), self::JavaScriptCallInstruction::Primitive(5f64, self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]), std::rc::Rc::new(std::cell::RefCell::new(vec![5f64]))), self::JavaScriptCallInstruction::Primitive(5f64, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 32, 97, 114, 114, 97, 121, 32, 108, 105, 107, 101, 32, 108, 101, 110, 103, 116, 104]), std::rc::Rc::new(std::cell::RefCell::new(vec![5f64]))), self::JavaScriptCallInstruction::ArgumentListOpen(0f64), self::JavaScriptCallInstruction::Constant(4f64, Box::new(self::JavaScriptSemanticValue::Number(0f64))), self::JavaScriptCallInstruction::Constant(8f64, Box::new(self::JavaScriptSemanticValue::Number(1f64)))]));
        {
            let loop_ = { let ml_reference_array371 = code.clone(); let ml_reference_projection371 = ml_reference_array371.borrow().len() as f64; ml_reference_projection371 };
            {
                let source_ml_temporary_compile_java_script_reflect_construction_function_ignored295 = { let ml_method_owner372 = code.clone(); let ml_method_argument372_0 = self::JavaScriptCallInstruction::NumberBinary(6f64, 4f64, 5f64, self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110])); let ml_inserted = vec![ml_method_argument372_0]; let mut ml_method_values372 = ml_method_owner372.borrow_mut(); let ml_length = ml_method_values372.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values372.extend(ml_inserted); ml_length as f64 };
                {
                    let branch = { let ml_reference_array373 = code.clone(); let ml_reference_projection373 = ml_reference_array373.borrow().len() as f64; ml_reference_projection373 };
                    {
                        let source_ml_temporary_compile_java_script_reflect_construction_function_ignored296 = { let ml_method_owner374 = code.clone(); let ml_method_argument374_0 = self::JavaScriptCallInstruction::Branch(6f64, (branch + 1f64), 0f64); let ml_method_argument374_1 = self::JavaScriptCallInstruction::PropertyKey(6f64, 4f64, realm.clone()); let ml_method_argument374_2 = self::JavaScriptCallInstruction::OrdinaryGet(7f64, 2f64, 6f64, 2f64); let ml_method_argument374_3 = self::JavaScriptCallInstruction::ArgumentListAppend(0f64, 7f64); let ml_method_argument374_4 = self::JavaScriptCallInstruction::NumberBinary(4f64, 4f64, 8f64, self::ml_string::Text::from_units(vec![97, 100, 100])); let ml_method_argument374_5 = self::JavaScriptCallInstruction::Jump(loop_); let ml_inserted = vec![ml_method_argument374_0, ml_method_argument374_1, ml_method_argument374_2, ml_method_argument374_3, ml_method_argument374_4, ml_method_argument374_5]; let mut ml_method_values374 = ml_method_owner374.borrow_mut(); let ml_length = ml_method_values374.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values374.extend(ml_inserted); ml_length as f64 };
                        {
                            let end = { let ml_reference_array375 = code.clone(); let ml_reference_projection375 = ml_reference_array375.borrow().len() as f64; ml_reference_projection375 };
                            {
                                let source_ml_temporary_compile_java_script_reflect_construction_function_ignored297 = { let ml_array_owner376 = code.clone(); let ml_array_argument376_0: self::JavaScriptCallInstruction = self::JavaScriptCallInstruction::Branch(6f64, (branch + 1f64), end); let ml_array_argument376_1: f64 = branch; let ml_array_argument376_2: f64 = (branch + 1f64); let ml_array_length = ml_array_owner376.borrow().len(); let ml_array_start = { let ml_relative_value: f64 = ml_array_argument376_1; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; let ml_array_end = { let ml_relative_value: f64 = ml_array_argument376_2; let ml_relative = if ml_relative_value.is_nan() { 0f64 } else { ml_relative_value.trunc() }; let ml_bound = ml_array_length as f64; (if ml_relative < 0f64 { (ml_bound + ml_relative).clamp(0f64, ml_bound) } else { ml_relative.min(ml_bound) }) as usize }; { let mut ml_array_values = ml_array_owner376.borrow_mut(); if ml_array_start < ml_array_end { ml_array_values[ml_array_start..ml_array_end].fill(ml_array_argument376_0); } } ml_array_owner376 };
                                {
                                    let source_ml_temporary_compile_java_script_reflect_construction_function_ignored298 = { let ml_method_owner377 = code.clone(); let ml_method_argument377_0 = self::JavaScriptCallInstruction::ArgumentListCall(9f64, 0f64, 1f64, 3f64, self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])); let ml_method_argument377_1 = self::JavaScriptCallInstruction::Return(9f64); let ml_inserted = vec![ml_method_argument377_0, ml_method_argument377_1]; let mut ml_method_values377 = ml_method_owner377.borrow_mut(); let ml_length = ml_method_values377.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values377.extend(ml_inserted); ml_length as f64 };
                                    {
                                        let source_ml_source1_constructor_field1544 = self::JavaScriptSemanticValue::Undefined;
                                        {
                                            let source_ml_source1_constructor_field1545 = 0f64;
                                            {
                                                let source_ml_source1_constructor_field1546 = 10f64;
                                                {
                                                    let source_ml_source1_constructor_field1547 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                    self::JavaScriptCallFunction::CallFunction(identity.clone(), realm.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99]), Box::new(source_ml_source1_constructor_field1544.clone()), source_ml_source1_constructor_field1545, source_ml_source1_constructor_field1546, source_ml_source1_constructor_field1547.clone(), code.clone(), None::<(Option<self::JavaScriptSemanticValue>,)>, None::<(Option<self::JavaScriptCallSourceSpan>,)>)
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
