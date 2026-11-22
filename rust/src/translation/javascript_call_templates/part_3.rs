// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result;
use super::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Result;
use super::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Result;
use super::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_construct_java_script_semantic_text_units_loop5;
use super::source_ml_temporary_find_java_script_call_code_function_loop8;
use super::source_ml_temporary_find_java_script_property_descriptor_entry_loop20;
use super::source_ml_temporary_read_java_script_weak_key_eligibility_loop19;
use super::source_ml_temporary_test_java_script_semantic_text_equal_loop2;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_text_equal(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    {
        let length = self::read_java_script_semantic_text_length(left.clone());
        if (length < 0f64)
            || (length != (self::read_java_script_semantic_text_length(right.clone())))
        {
            false
        } else if match left.clone() {
            self::JavaScriptSemanticValue::SourceText(_, _) => true,
            _ => false,
        } && match right.clone() {
            self::JavaScriptSemanticValue::SourceText(_, _) => true,
            _ => false,
        } {
            {
                let source_ml_source1_comparison_operand243 = match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::SourceText(
                        _,
                        source_ml_source1_data_property_value241,
                    ) => Some(source_ml_source1_data_property_value241.clone()),
                    self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Object(_, _) => None::<self::ml_string::Text>,
                };
                {
                    let source_ml_source1_comparison_operand244 = match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::SourceText(
                            _,
                            source_ml_source1_data_property_value242,
                        ) => Some(source_ml_source1_data_property_value242.clone()),
                        self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    match (
                        source_ml_source1_comparison_operand243,
                        source_ml_source1_comparison_operand244,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            }
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source1_s245 =
                        self::source_ml_temporary_test_java_script_semantic_text_equal_loop2(
                            left.clone(),
                            right.clone(),
                            length,
                            at,
                        );
                    match source_ml_source1_s245 {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done => {
                                true
                            }
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(source_ml_temporary_result2) => {
                                source_ml_temporary_result2
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_semantic_text_units(
    value: self::JavaScriptSemanticValue,
    limit: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let length = self::read_java_script_semantic_text_length(value.clone());
        if (length < 0f64) || (length > limit) {
            (self::ml_source_exception::SourceException {
                class_name: String::from("RangeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 99, 111, 110,
                    118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97,
                    116, 105, 111, 110, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s326 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units.clone(), at);
                            match source_ml_source1_s326 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    let units_2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_2 = 0f64;
                        {
                            let source_ml_source1_s327 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_2.clone(), at_2);
                            match source_ml_source1_s327 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_2.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(_) => {
                    let units_3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_3 = 0f64;
                        {
                            let source_ml_source1_s328 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_3.clone(), at_3);
                            match source_ml_source1_s328 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_3.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(_) => {
                    let units_4 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_4 = 0f64;
                        {
                            let source_ml_source1_s329 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_4.clone(), at_4);
                            match source_ml_source1_s329 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_4.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    let units_5 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_5 = 0f64;
                        {
                            let source_ml_source1_s330 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_5.clone(), at_5);
                            match source_ml_source1_s330 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_5.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_6) => units_6.clone(),
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    let units_7 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_6 = 0f64;
                        {
                            let source_ml_source1_s331 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_7.clone(), at_6);
                            match source_ml_source1_s331 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_7.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    let units_8 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_7 = 0f64;
                        {
                            let source_ml_source1_s332 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_8.clone(), at_7);
                            match source_ml_source1_s332 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_8.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    let units_9 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_8 = 0f64;
                        {
                            let source_ml_source1_s333 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop5(value.clone(), length, units_9.clone(), at_8);
                            match source_ml_source1_s333 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done => {
                                        units_9.clone()
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
pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    {
        let ml_library_inequality_1 = (self::read_java_script_semantic_value_error(value.clone()))
            != (self::ml_string::Text::from_units(vec![]));
        if ml_library_inequality_1 {
            false
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                self::JavaScriptSemanticValue::Object(_, source_ml_source1_f342) => {
                    source_ml_source1_f342
                }
                _ => false,
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_coordinate(value: f64, limit: f64) -> bool {
    ((self::ml_math::is_safe_integer(value) && (value >= 0f64)) && (value < limit))
        && ((value != 0f64) || ((1f64 / value) > 0f64))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    self::test_java_script_source_coordinate(value, limit)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_code_function(
    program: self::JavaScriptCallProgram,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array4 = match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(
                        source_ml_source1_data_property_value408,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source1_data_property_value408.clone(),
                };
                ml_reference_array4.borrow().len() as f64
            };
            {
                let source_ml_source1_s409 =
                    self::source_ml_temporary_find_java_script_call_code_function_loop8(
                        program.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s409 {
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Done => {
                        -1f64
                    }
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop8Return(source_ml_temporary_result8) => {
                        source_ml_temporary_result8
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
        self::JavaScriptCallInstruction::Constant(destination, _) => destination,
        self::JavaScriptCallInstruction::Copy(destination_2, _) => destination_2,
        self::JavaScriptCallInstruction::Argument(destination_3, _) => destination_3,
        self::JavaScriptCallInstruction::ArgumentCount(destination_4) => destination_4,
        self::JavaScriptCallInstruction::ArgumentAt(destination_5, _) => destination_5,
        self::JavaScriptCallInstruction::FunctionBind(destination_6) => destination_6,
        self::JavaScriptCallInstruction::BoundFunctionCall(destination_7) => destination_7,
        self::JavaScriptCallInstruction::FunctionToString(destination_8) => destination_8,
        self::JavaScriptCallInstruction::Call(destination_9, _, _, _) => destination_9,
        self::JavaScriptCallInstruction::ArgumentListCall(destination_10, _, _, _, _) => {
            destination_10
        }
        self::JavaScriptCallInstruction::Construct(destination_11, _, _, _) => destination_11,
        self::JavaScriptCallInstruction::ConstructorCheck(destination_12, _, _) => destination_12,
        self::JavaScriptCallInstruction::ReadNewTarget(destination_13) => destination_13,
        self::JavaScriptCallInstruction::ArrayJoinEnter(destination_14, _) => destination_14,
        self::JavaScriptCallInstruction::ConstructErrorClass(destination_15, _, _) => {
            destination_15
        }
        self::JavaScriptCallInstruction::CallClassConstructor(destination_16, _, _, _) => {
            destination_16
        }
        self::JavaScriptCallInstruction::ConstructCollection(destination_17, _, _) => {
            destination_17
        }
        self::JavaScriptCallInstruction::CollectionAllocate(destination_18, _, _) => destination_18,
        self::JavaScriptCallInstruction::CollectionIteratorAllocate(destination_19, _, _, _, _) => {
            destination_19
        }
        self::JavaScriptCallInstruction::ArgumentRest(destination_20, _, _, _) => destination_20,
        self::JavaScriptCallInstruction::IteratorProtocol(destination_21, _, _, _, _, _, _, _) => {
            destination_21
        }
        self::JavaScriptCallInstruction::CollectionIteratorResult(destination_22, _, _, _) => {
            destination_22
        }
        self::JavaScriptCallInstruction::CollectionOperation(destination_23, _) => destination_23,
        self::JavaScriptCallInstruction::PrimitiveMethod(destination_24, _) => destination_24,
        self::JavaScriptCallInstruction::CollectionForEach(destination_25, _) => destination_25,
        self::JavaScriptCallInstruction::ArrayConcatenation(destination_26, _, _, _) => {
            destination_26
        }
        self::JavaScriptCallInstruction::ArraySlice(destination_27, _, _) => destination_27,
        self::JavaScriptCallInstruction::ArrayCreation(destination_28, _, _, _) => destination_28,
        self::JavaScriptCallInstruction::ArrayCallback(destination_29, _, _) => destination_29,
        self::JavaScriptCallInstruction::ArraySearch(destination_30, _, _) => destination_30,
        self::JavaScriptCallInstruction::ArrayTransform(destination_31, _, _, _) => destination_31,
        self::JavaScriptCallInstruction::ObjectEnumeration(destination_32, _, _) => destination_32,
        self::JavaScriptCallInstruction::PropertyDefinitions(destination_33, _, _, _) => {
            destination_33
        }
        self::JavaScriptCallInstruction::PropertyReflection(destination_34, _, _, _, _, _) => {
            destination_34
        }
        self::JavaScriptCallInstruction::InstanceOf(destination_35, _, _, _, _) => destination_35,
        self::JavaScriptCallInstruction::ObjectObservation(destination_36, _, _, _) => {
            destination_36
        }
        self::JavaScriptCallInstruction::CollectionCopy(destination_37, _, _, _, _, _, _, _) => {
            destination_37
        }
        self::JavaScriptCallInstruction::ConstructErrorValue(
            destination_38,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => destination_38,
        self::JavaScriptCallInstruction::ReadDerivedReceiver(destination_39) => destination_39,
        self::JavaScriptCallInstruction::InvokeErrorIntrinsic(destination_40) => destination_40,
        self::JavaScriptCallInstruction::InvokeHostCapability(destination_41) => destination_41,
        self::JavaScriptCallInstruction::PushHandler(destination_42, _) => destination_42,
        self::JavaScriptCallInstruction::NumberBinary(destination_43, _, _, _) => destination_43,
        self::JavaScriptCallInstruction::SameValue(destination_44, _, _) => destination_44,
        self::JavaScriptCallInstruction::StrictEqual(destination_45, _, _) => destination_45,
        self::JavaScriptCallInstruction::Primitive(destination_46, _, _) => destination_46,
        self::JavaScriptCallInstruction::OrdinaryAllocate(destination_47, _) => destination_47,
        self::JavaScriptCallInstruction::LiteralPrototype(destination_48, _, _) => destination_48,
        self::JavaScriptCallInstruction::LiteralAccessor(destination_49, _, _, _, _, _) => {
            destination_49
        }
        self::JavaScriptCallInstruction::ObjectFromEntries(destination_50, _, _, _) => {
            destination_50
        }
        self::JavaScriptCallInstruction::PrimitiveWrapper(destination_51, _, _) => destination_51,
        self::JavaScriptCallInstruction::SymbolAllocate(destination_52, _) => destination_52,
        self::JavaScriptCallInstruction::SymbolDescription(destination_53, _) => destination_53,
        self::JavaScriptCallInstruction::SymbolRegistry(destination_54, _, _) => destination_54,
        self::JavaScriptCallInstruction::FunctionName(destination_55, _, _, _, _) => destination_55,
        self::JavaScriptCallInstruction::ArrayAllocate(destination_56, _, _) => destination_56,
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
        self::JavaScriptCallInstruction::OrdinaryGet(destination_61, _, _, _) => destination_61,
        self::JavaScriptCallInstruction::OrdinaryDelete(destination_62, _, _) => destination_62,
        self::JavaScriptCallInstruction::OrdinaryHas(destination_63, _, _) => destination_63,
        self::JavaScriptCallInstruction::OrdinarySet(destination_64, _, _, _, _) => destination_64,
        self::JavaScriptCallInstruction::OrdinaryDefine(destination_65, _, _, _, _) => {
            destination_65
        }
        self::JavaScriptCallInstruction::PropertyKey(destination_66, _, _) => destination_66,
        self::JavaScriptCallInstruction::PropertyDefinition(destination_67, _, _, _, _, _) => {
            destination_67
        }
        _ => -1f64,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_validated_heap_snapshot(_heap: self::JavaScriptOrdinaryHeap) -> bool {
    false
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_weak_key_eligibility(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f693) => {
            let facts = if {
                let source_ml_source1_comparison_operand761 = match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        _,
                        source_ml_source1_data_property_value760,
                        _,
                        _,
                        _,
                    ) => ((source_ml_source1_data_property_value760
                        .clone()
                        .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                    .0)
                        .clone(),
                };
                match (
                    source_ml_source1_comparison_operand761,
                    self::ml_string::Text::from_units(vec![
                        97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83, 121,
                        109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121, 32, 99, 108,
                        97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 49,
                    ]),
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            } {
                (match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value763,
                        _,
                        _,
                    ) => ((source_ml_source1_data_property_value763
                        .clone()
                        .unwrap_or_else(|| {
                            (None::<
                                std::rc::Rc<
                                    std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>,
                                >,
                            >,)
                        }))
                    .0)
                        .clone(),
                })
                .clone()
                .unwrap_or_else(|| {
                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                        self::JavaScriptSymbolRegistrationFact,
                    >::new()))
                })
            } else {
                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                    self::JavaScriptSymbolRegistrationFact,
                >::new()))
            };
            {
                let low = 0f64;
                {
                    let high = {
                        let ml_reference_array5 = facts.clone();
                        ml_reference_array5.borrow().len() as f64
                    };
                    {
                        let low_2 =
                            self::source_ml_temporary_read_java_script_weak_key_eligibility_loop19(
                                value.clone(),
                                facts.clone(),
                                low,
                                high,
                            );
                        {
                            let candidate = {
                                let ml_reference_array6 = facts.clone();
                                let ml_reference_position6 = low_2;
                                self::ml_array::optional_at(
                                    &*ml_reference_array6.borrow(),
                                    self::ml_array::number_index(ml_reference_position6),
                                )
                            };
                            {
                                let fact = if {
                                    let source_ml_source1_comparison_operand768 = {
                                        let source_ml_source1_optional_chain_receiver766 =
                                            (candidate.clone(),);
                                        if match (source_ml_source1_optional_chain_receiver766
                                            .clone())
                                        .0
                                        {
                                            Some(ml_present) => {
                                                let _ = ml_present.clone();
                                                true
                                            }
                                            None => false,
                                        } {
                                            Some({
                                                let source_ml_temporary_optional_chain18 = ((source_ml_source1_optional_chain_receiver766.clone()).0).clone().expect("a proven present value");
                                                match source_ml_temporary_optional_chain18 {
                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source1_data_property_value767, _, _) => {
                                                            source_ml_source1_data_property_value767.clone()
                                                        }
                                                    }
                                            })
                                        } else {
                                            None::<self::ml::Big>
                                        }
                                    };
                                    match (
                                        source_ml_source1_comparison_operand768,
                                        source_ml_source1_f693.clone(),
                                    ) {
                                        (Some(ml_left), ml_right) => ml_left == ml_right,
                                        _ => false,
                                    }
                                } {
                                    candidate.clone()
                                } else {
                                    None::<self::JavaScriptSymbolRegistrationFact>
                                };
                                if match (fact.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                        self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121])
                                    } else if match fact.clone().expect("a proven present value") {
                                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source1_data_property_value770, _) => {
                                                source_ml_source1_data_property_value770
                                            }
                                        } {
                                            self::ml_string::Text::from_units(vec![105, 110, 101, 108, 105, 103, 105, 98, 108, 101])
                                        } else {
                                            self::ml_string::Text::from_units(vec![101, 108, 105, 103, 105, 98, 108, 101])
                                        }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::Object(_, _) => {
            self::ml_string::Text::from_units(vec![101, 108, 105, 103, 105, 98, 108, 101])
        }
        _ => {
            self::ml_string::Text::from_units(vec![105, 110, 101, 108, 105, 103, 105, 98, 108, 101])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_property_descriptor_entry(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s771 =
                self::source_ml_temporary_find_java_script_property_descriptor_entry_loop20(
                    properties.clone(),
                    key.clone(),
                    at,
                );
            match source_ml_source1_s771 {
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Done => {
                    -1f64
                }
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop20Return(source_ml_temporary_result20) => {
                    source_ml_temporary_result20
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_property_descriptor_kind(
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        )>,
    >,
) -> self::ml_string::Text {
    {
        let data = ({ descriptor.borrow().clone() }).9 || ({ descriptor.borrow().clone() }).11;
        {
            let accessor =
                ({ descriptor.borrow().clone() }).5 || ({ descriptor.borrow().clone() }).7;
            if data && accessor {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else if data {
                self::ml_string::Text::from_units(vec![100, 97, 116, 97])
            } else if accessor {
                self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
            } else {
                self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_property_descriptor_error(
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        )>,
    >,
) -> self::ml_string::Text {
    if (self::read_java_script_property_descriptor_kind(descriptor.clone()))
        == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        let ml_product_condition_3 = { descriptor.borrow().clone() };
        if ml_product_condition_3 {
            self::ml_string::Text::from_units(vec![
                73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108,
                117, 101,
            ])
        } else {
            let ml_product_condition_2 = { descriptor.borrow().clone() };
            if ml_product_condition_2 {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                let ml_product_condition_1 = { descriptor.borrow().clone() };
                if ml_product_condition_1 {
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn test_complete_java_script_property_descriptor(
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            bool,
            bool,
            bool,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            self::JavaScriptSemanticValue,
            bool,
            bool,
            bool,
        )>,
    >,
) -> bool {
    if (!(({ descriptor.borrow().clone() }).1) || !(({ descriptor.borrow().clone() }).3))
        || ((self::read_java_script_property_descriptor_error(descriptor.clone()))
            != (self::ml_string::Text::from_units(vec![])))
    {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])) {
                ({ descriptor.borrow().clone() }).9 && ({ descriptor.borrow().clone() }).11
            } else {
                ((kind
                    == (self::ml_string::Text::from_units(vec![
                        97, 99, 99, 101, 115, 115, 111, 114,
                    ])))
                    && ({ descriptor.borrow().clone() }).5)
                    && ({ descriptor.borrow().clone() }).7
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_property_descriptor(
    kind: &self::ml_string::Text,
    configurable: bool,
    enumerable: bool,
) -> std::rc::Rc<
    std::cell::RefCell<(
        bool,
        bool,
        bool,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        self::JavaScriptSemanticValue,
        bool,
        bool,
        bool,
    )>,
> {
    std::rc::Rc::new(std::cell::RefCell::new({
        let source_ml_source1_field776 = (*kind)
            != (self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114]));
        {
            let source_ml_source1_field777 = self::JavaScriptSemanticValue::Undefined;
            {
                let source_ml_source1_field778 = (*kind)
                    != (self::ml_string::Text::from_units(vec![
                        97, 99, 99, 101, 115, 115, 111, 114,
                    ]));
                {
                    let source_ml_source1_field780 = (*kind)
                        == (self::ml_string::Text::from_units(vec![
                            97, 99, 99, 101, 115, 115, 111, 114,
                        ]));
                    {
                        let source_ml_source1_field781 = self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source1_field782 = (*kind)
                                == (self::ml_string::Text::from_units(vec![
                                    97, 99, 99, 101, 115, 115, 111, 114,
                                ]));
                            {
                                let source_ml_source1_field783 =
                                    self::JavaScriptSemanticValue::Undefined;
                                (
                                    configurable,
                                    true,
                                    enumerable,
                                    true,
                                    source_ml_source1_field781,
                                    source_ml_source1_field780,
                                    source_ml_source1_field783,
                                    source_ml_source1_field782,
                                    source_ml_source1_field777,
                                    source_ml_source1_field776,
                                    false,
                                    source_ml_source1_field778,
                                )
                            }
                        }
                    }
                }
            }
        }
    }))
}
