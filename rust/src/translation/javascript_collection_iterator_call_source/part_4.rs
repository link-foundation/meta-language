// Generated from complete js/src/translation/javascript-collection-iterator-call-source.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Result;
use super::MlTemporaryFindJavaScriptCallConstructorFactLoop6Result;
use super::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_find_java_script_call_code_function_loop7;
use super::source_ml_temporary_find_java_script_call_constructor_fact_loop6;
use super::source_ml_temporary_order_java_script_call_code_functions_loop8;

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
                self::JavaScriptSemanticValue::Null => false,
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f229) => {
                    let source_ml_source1_f246 = source_ml_source1_f229;
                    false
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f230) => {
                    let source_ml_source1_f255 = source_ml_source1_f230;
                    false
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f231) => {
                    let source_ml_source1_f264 = source_ml_source1_f231.clone();
                    false
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f232) => {
                    let source_ml_source1_f273 = source_ml_source1_f232;
                    false
                }
                self::JavaScriptSemanticValue::SourceText(
                    source_ml_source1_f233,
                    source_ml_source1_f234,
                ) => {
                    let source_ml_source1_f282 = source_ml_source1_f233.clone();
                    {
                        let source_ml_source1_f283 = source_ml_source1_f234.clone();
                        false
                    }
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f235) => {
                    let source_ml_source1_f292 = source_ml_source1_f235.clone();
                    false
                }
                self::JavaScriptSemanticValue::Object(
                    source_ml_source1_f236,
                    source_ml_source1_f237,
                ) => {
                    let source_ml_source1_f301 = source_ml_source1_f236.clone();
                    { source_ml_source1_f237 }
                }
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
pub fn find_java_script_call_constructor_fact(
    facts: Vec<self::JavaScriptArrayConstructorFact>,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = facts.len() as f64;
            {
                let source_ml_source1_s302 =
                    self::source_ml_temporary_find_java_script_call_constructor_fact_loop6(
                        facts,
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s302 {
                    self::MlTemporaryFindJavaScriptCallConstructorFactLoop6Result::MlTemporaryFindJavaScriptCallConstructorFactLoop6Done => {
                        -1f64
                    }
                    self::MlTemporaryFindJavaScriptCallConstructorFactLoop6Result::MlTemporaryFindJavaScriptCallConstructorFactLoop6Return(source_ml_temporary_result6) => {
                        source_ml_temporary_result6
                    }
                }
            }
        }
    }
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
            let high = (match &program {
                self::JavaScriptCallProgram::CallProgram(
                    source_ml_source1_data_property_value303,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source1_data_property_value303.as_slice(),
            })
            .len() as f64;
            {
                let source_ml_source1_s304 =
                    self::source_ml_temporary_find_java_script_call_code_function_loop7(
                        program.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s304 {
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Done => {
                        -1f64
                    }
                    self::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Return(source_ml_temporary_result7) => {
                        source_ml_temporary_result7
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn order_java_script_call_code_functions(
    functions: Vec<self::JavaScriptCallFunction>,
) -> Vec<self::JavaScriptCallFunction> {
    {
        let ordered = functions.clone();
        {
            let width = 1f64;
            {
                let source_ml_source1_s305 =
                    self::source_ml_temporary_order_java_script_call_code_functions_loop8(
                        functions, ordered, width,
                    );
                match source_ml_source1_s305 {
                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Done(ordered_2) => {
                        ordered_2
                    }
                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Return(source_ml_temporary_result8) => {
                        source_ml_temporary_result8
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_code_registers(
    instruction: self::JavaScriptCallInstruction,
) -> Vec<f64> {
    match instruction.clone() {
        self::JavaScriptCallInstruction::Constant(destination, source_ml_source1_f306) => {
            let source_ml_source1_f306 = *source_ml_source1_f306;
            vec![destination]
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            vec![destination_2, source]
        }
        self::JavaScriptCallInstruction::Argument(destination_3, source_ml_source1_f307) => {
            vec![destination_3]
        }
        self::JavaScriptCallInstruction::ArgumentCount(destination_4) => {
            vec![destination_4]
        }
        self::JavaScriptCallInstruction::ArgumentAt(destination_5, index) => {
            vec![destination_5, index]
        }
        self::JavaScriptCallInstruction::FunctionBind(destination_6) => {
            vec![destination_6]
        }
        self::JavaScriptCallInstruction::BoundFunctionCall(destination_7) => {
            vec![destination_7]
        }
        self::JavaScriptCallInstruction::FunctionToString(destination_8) => {
            vec![destination_8]
        }
        self::JavaScriptCallInstruction::PushFinally(source_ml_source1_f308) => Vec::<f64>::new(),
        self::JavaScriptCallInstruction::FinishFinally => Vec::<f64>::new(),
        self::JavaScriptCallInstruction::CompleteJump(
            source_ml_source1_f309,
            source_ml_source1_f310,
        ) => Vec::<f64>::new(),
        self::JavaScriptCallInstruction::Jump(source_ml_source1_f311) => Vec::<f64>::new(),
        self::JavaScriptCallInstruction::Branch(
            condition,
            source_ml_source1_f312,
            source_ml_source1_f313,
        ) => {
            vec![condition]
        }
        self::JavaScriptCallInstruction::Call(destination_9, callee, receiver, arguments) => {
            let source_ml_source1_concat_value366 = vec![destination_9, callee, receiver];
            {
                let source_ml_source1_concat_value367 = arguments;
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value366),
                    source_ml_source1_concat_value367,
                )
            }
        }
        self::JavaScriptCallInstruction::ArgumentListOpen(source_ml_source1_f314) => {
            Vec::<f64>::new()
        }
        self::JavaScriptCallInstruction::ArgumentListAppend(source_ml_source1_f315, value) => {
            vec![value]
        }
        self::JavaScriptCallInstruction::ArgumentListCall(
            destination_10,
            source_ml_source1_f316,
            callee_2,
            receiver_2,
            source_ml_source1_f317,
        ) => {
            vec![destination_10, callee_2, receiver_2]
        }
        self::JavaScriptCallInstruction::Construct(
            destination_11,
            callee_3,
            new_target,
            arguments_2,
        ) => {
            let source_ml_source1_concat_value368 = vec![destination_11, callee_3, new_target];
            {
                let source_ml_source1_concat_value369 = arguments_2;
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value368),
                    source_ml_source1_concat_value369,
                )
            }
        }
        self::JavaScriptCallInstruction::ConstructorCheck(
            destination_12,
            input,
            source_ml_source1_f318,
        ) => {
            vec![destination_12, input]
        }
        self::JavaScriptCallInstruction::ReadNewTarget(destination_13) => {
            vec![destination_13]
        }
        self::JavaScriptCallInstruction::ArrayJoinEnter(destination_14, receiver_3) => {
            vec![destination_14, receiver_3]
        }
        self::JavaScriptCallInstruction::ArrayJoinLeave(receiver_4) => {
            vec![receiver_4]
        }
        self::JavaScriptCallInstruction::ConstructErrorClass(
            destination_15,
            callee_4,
            arguments_3,
        ) => {
            let source_ml_source1_concat_value370 = vec![destination_15, callee_4];
            {
                let source_ml_source1_concat_value371 = arguments_3;
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value370),
                    source_ml_source1_concat_value371,
                )
            }
        }
        self::JavaScriptCallInstruction::CallClassConstructor(
            destination_16,
            callee_5,
            receiver_5,
            arguments_4,
        ) => {
            let source_ml_source1_concat_value372 = vec![destination_16, callee_5, receiver_5];
            {
                let source_ml_source1_concat_value373 = arguments_4;
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value372),
                    source_ml_source1_concat_value373,
                )
            }
        }
        self::JavaScriptCallInstruction::ConstructCollection(
            destination_17,
            callee_6,
            arguments_5,
        ) => {
            let source_ml_source1_concat_value374 = vec![destination_17, callee_6];
            {
                let source_ml_source1_concat_value375 = arguments_5;
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value374),
                    source_ml_source1_concat_value375,
                )
            }
        }
        self::JavaScriptCallInstruction::CollectionAllocate(
            destination_18,
            prototype,
            source_ml_source1_f319,
        ) => {
            vec![destination_18, prototype]
        }
        self::JavaScriptCallInstruction::CollectionIteratorAllocate(
            destination_19,
            prototype_2,
            source_ml_source1_f320,
            source_ml_source1_f321,
            instruction_input,
        ) => {
            let input_2 = ((instruction_input.clone().unwrap_or_else(|| (None::<f64>,))).0).clone();
            if {
                let source_ml_source1_comparison_operand376 = input_2.clone();
                {
                    let source_ml_source1_comparison_operand377 = None::<()>;
                    match (
                        source_ml_source1_comparison_operand376,
                        source_ml_source1_comparison_operand377,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }
                }
            } {
                vec![destination_19, prototype_2]
            } else {
                vec![
                    destination_19,
                    prototype_2,
                    input_2.clone().expect("a proven present value"),
                ]
            }
        }
        self::JavaScriptCallInstruction::ArgumentRest(
            destination_20,
            prototype_3,
            source_ml_source1_f322,
            source_ml_source1_f323,
        ) => {
            vec![destination_20, prototype_3]
        }
        self::JavaScriptCallInstruction::IteratorProtocol(
            destination_21,
            done,
            input_3,
            iterator_key,
            source_ml_source1_f324,
            source_ml_source1_f325,
            source_ml_source1_f326,
            source_ml_source1_f327,
        ) => {
            vec![destination_21, done, input_3, iterator_key]
        }
        self::JavaScriptCallInstruction::CollectionIteratorResult(
            destination_22,
            source_ml_source1_f328,
            array_prototype,
            object_prototype,
        ) => {
            vec![destination_22, array_prototype, object_prototype]
        }
        self::JavaScriptCallInstruction::CollectionOperation(
            destination_23,
            source_ml_source1_f329,
        ) => {
            vec![destination_23]
        }
        self::JavaScriptCallInstruction::PrimitiveMethod(
            destination_24,
            source_ml_source1_f330,
        ) => {
            vec![destination_24]
        }
        self::JavaScriptCallInstruction::CollectionForEach(
            destination_25,
            source_ml_source1_f331,
        ) => {
            vec![destination_25]
        }
        self::JavaScriptCallInstruction::ArrayConcatenation(
            destination_26,
            input_4,
            spreadable_key,
            source_ml_source1_f332,
        ) => {
            let source_ml_source1_f332 = *source_ml_source1_f332;
            vec![
                destination_26,
                (((input_4.clone().unwrap_or_else(|| (None::<f64>,))).0).clone())
                    .clone()
                    .unwrap_or_else(|| 0f64),
                spreadable_key,
            ]
        }
        self::JavaScriptCallInstruction::ArraySlice(
            destination_27,
            input_5,
            source_ml_source1_f333,
        ) => {
            let source_ml_source1_f333 = *source_ml_source1_f333;
            vec![
                destination_27,
                (((input_5.clone().unwrap_or_else(|| (None::<f64>,))).0).clone())
                    .clone()
                    .unwrap_or_else(|| 0f64),
            ]
        }
        self::JavaScriptCallInstruction::ArrayCreation(
            destination_28,
            source_ml_source1_f334,
            prototype_4,
            iterator_key_2,
        ) => {
            vec![destination_28, prototype_4, iterator_key_2]
        }
        self::JavaScriptCallInstruction::ArrayCallback(
            destination_29,
            source_ml_source1_f335,
            input_6,
        ) => {
            vec![
                destination_29,
                (((input_6.clone().unwrap_or_else(|| (None::<f64>,))).0).clone())
                    .clone()
                    .unwrap_or_else(|| 0f64),
            ]
        }
        self::JavaScriptCallInstruction::ArraySearch(
            destination_30,
            source_ml_source1_f336,
            input_7,
        ) => {
            vec![
                destination_30,
                (((input_7.clone().unwrap_or_else(|| (None::<f64>,))).0).clone())
                    .clone()
                    .unwrap_or_else(|| 0f64),
            ]
        }
        self::JavaScriptCallInstruction::ArrayTransform(
            destination_31,
            source_ml_source1_f337,
            source_ml_source1_f338,
            input_8,
        ) => {
            let source_ml_source1_f338 = *source_ml_source1_f338;
            vec![
                destination_31,
                (((input_8.clone().unwrap_or_else(|| (None::<f64>,))).0).clone())
                    .clone()
                    .unwrap_or_else(|| 0f64),
            ]
        }
        self::JavaScriptCallInstruction::ObjectEnumeration(
            destination_32,
            source_ml_source1_f339,
            prototype_5,
        ) => {
            vec![destination_32, prototype_5]
        }
        self::JavaScriptCallInstruction::PropertyDefinitions(
            destination_33,
            source_ml_source1_f340,
            target,
            input_9,
        ) => {
            vec![destination_33, target, input_9]
        }
        self::JavaScriptCallInstruction::PropertyReflection(
            destination_34,
            source_ml_source1_f341,
            input_10,
            key,
            object_prototype_2,
            array_prototype_2,
        ) => {
            vec![
                destination_34,
                input_10,
                key,
                object_prototype_2,
                array_prototype_2,
            ]
        }
        self::JavaScriptCallInstruction::InstanceOf(
            destination_35,
            left,
            right,
            key_2,
            source_ml_source1_f342,
        ) => {
            vec![destination_35, left, right, key_2]
        }
        self::JavaScriptCallInstruction::ObjectObservation(
            destination_36,
            source_ml_source1_f343,
            input_11,
            other,
        ) => {
            vec![destination_36, input_11, other]
        }
        self::JavaScriptCallInstruction::CollectionCopy(
            destination_37,
            target_2,
            input_12,
            adder,
            iterator_key_3,
            iterator_method,
            iterator_prototype,
            next_method,
        ) => {
            vec![
                destination_37,
                target_2,
                input_12,
                adder,
                iterator_key_3,
                iterator_method,
                iterator_prototype,
                next_method,
            ]
        }
        self::JavaScriptCallInstruction::ConstructErrorValue(
            destination_38,
            constructor,
            new_target_2,
            prototype_6,
            message,
            present,
            source_ml_source1_f344,
            source_ml_source1_f345,
            source_ml_source1_f346,
        ) => {
            vec![
                destination_38,
                constructor,
                new_target_2,
                prototype_6,
                message,
                present,
            ]
        }
        self::JavaScriptCallInstruction::ReadDerivedReceiver(destination_39) => {
            vec![destination_39]
        }
        self::JavaScriptCallInstruction::InvokeErrorIntrinsic(destination_40) => {
            vec![destination_40]
        }
        self::JavaScriptCallInstruction::InvokeHostCapability(destination_41) => {
            vec![destination_41]
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            vec![source_2]
        }
        self::JavaScriptCallInstruction::Throw(source_3) => {
            vec![source_3]
        }
        self::JavaScriptCallInstruction::PushHandler(destination_42, source_ml_source1_f347) => {
            vec![destination_42]
        }
        self::JavaScriptCallInstruction::PopHandler => Vec::<f64>::new(),
        self::JavaScriptCallInstruction::NumberBinary(
            destination_43,
            left_2,
            right_2,
            source_ml_source1_f348,
        ) => {
            vec![destination_43, left_2, right_2]
        }
        self::JavaScriptCallInstruction::SameValue(destination_44, left_3, right_3) => {
            vec![destination_44, left_3, right_3]
        }
        self::JavaScriptCallInstruction::StrictEqual(destination_45, left_4, right_4) => {
            vec![destination_45, left_4, right_4]
        }
        self::JavaScriptCallInstruction::RequireWrite(success) => {
            vec![success]
        }
        self::JavaScriptCallInstruction::Primitive(
            destination_46,
            source_ml_source1_f349,
            arguments_6,
        ) => {
            let source_ml_source1_concat_value378 = vec![destination_46];
            {
                let source_ml_source1_concat_value379 = arguments_6;
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value378),
                    source_ml_source1_concat_value379,
                )
            }
        }
        self::JavaScriptCallInstruction::OrdinaryAllocate(destination_47, prototype_7) => {
            vec![destination_47, prototype_7]
        }
        self::JavaScriptCallInstruction::LiteralPrototype(destination_48, object, value_2) => {
            vec![destination_48, object, value_2]
        }
        self::JavaScriptCallInstruction::LiteralAccessor(
            destination_49,
            object_2,
            key_3,
            value_3,
            source_ml_source1_f350,
            source_ml_source1_f351,
        ) => {
            vec![destination_49, object_2, key_3, value_3]
        }
        self::JavaScriptCallInstruction::ObjectFromEntries(
            destination_50,
            input_13,
            prototype_8,
            iterator_key_4,
        ) => {
            vec![destination_50, input_13, prototype_8, iterator_key_4]
        }
        self::JavaScriptCallInstruction::PrimitiveWrapper(
            destination_51,
            input_14,
            prototype_9,
        ) => {
            vec![destination_51, input_14, prototype_9]
        }
        self::JavaScriptCallInstruction::SymbolAllocate(destination_52, input_15) => {
            vec![destination_52, input_15]
        }
        self::JavaScriptCallInstruction::SymbolDescription(destination_53, input_16) => {
            vec![destination_53, input_16]
        }
        self::JavaScriptCallInstruction::SymbolRegistry(
            destination_54,
            input_17,
            source_ml_source1_f352,
        ) => {
            vec![destination_54, input_17]
        }
        self::JavaScriptCallInstruction::FunctionName(
            destination_55,
            object_3,
            key_4,
            source_ml_source1_f353,
            source_ml_source1_f354,
        ) => {
            vec![destination_55, object_3, key_4]
        }
        self::JavaScriptCallInstruction::ArrayAllocate(
            destination_56,
            prototype_10,
            source_ml_source1_f355,
        ) => {
            vec![destination_56, prototype_10]
        }
        self::JavaScriptCallInstruction::ClosureAllocate(
            destination_57,
            source_ml_source1_f356,
            source_ml_source1_f357,
            captures,
            receiver_6,
        ) => {
            let source_ml_source1_concat_value380 = vec![destination_57, receiver_6];
            {
                let source_ml_source1_concat_value381 = captures;
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value380),
                    source_ml_source1_concat_value381,
                )
            }
        }
        self::JavaScriptCallInstruction::ArraySpread(
            destination_58,
            target_3,
            input_18,
            index_2,
            iterator_key_5,
            source_ml_source1_f358,
        ) => {
            vec![destination_58, target_3, input_18, index_2, iterator_key_5]
        }
        self::JavaScriptCallInstruction::ObjectSpread(
            destination_59,
            target_4,
            input_19,
            source_ml_source1_f359,
            excluded,
            source_ml_source1_f360,
        ) => {
            let source_ml_source1_concat_value382 = vec![destination_59, target_4, input_19];
            {
                let source_ml_source1_concat_value383 =
                    (((excluded.clone().unwrap_or_else(|| (None::<Vec<f64>>,))).0).clone())
                        .clone()
                        .unwrap_or_else(|| Vec::<f64>::new());
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value382),
                    source_ml_source1_concat_value383,
                )
            }
        }
        self::JavaScriptCallInstruction::OrdinaryDataProperty(
            destination_60,
            object_4,
            key_5,
            value_4,
            source_ml_source1_f361,
        ) => {
            vec![destination_60, object_4, key_5, value_4]
        }
        self::JavaScriptCallInstruction::OrdinaryGet(
            destination_61,
            object_5,
            key_6,
            receiver_7,
        ) => {
            vec![destination_61, object_5, key_6, receiver_7]
        }
        self::JavaScriptCallInstruction::OrdinaryDelete(destination_62, object_6, key_7) => {
            vec![destination_62, object_6, key_7]
        }
        self::JavaScriptCallInstruction::OrdinaryHas(destination_63, object_7, key_8) => {
            vec![destination_63, object_7, key_8]
        }
        self::JavaScriptCallInstruction::OrdinarySet(
            destination_64,
            object_8,
            key_9,
            value_5,
            receiver_8,
        ) => {
            vec![destination_64, object_8, key_9, value_5, receiver_8]
        }
        self::JavaScriptCallInstruction::OrdinaryDefine(
            destination_65,
            target_5,
            key_10,
            fields,
            source_ml_source1_f362,
        ) => {
            let source_ml_source1_concat_value384 = vec![destination_65, target_5, key_10];
            {
                let source_ml_source1_concat_value385 = fields;
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value384),
                    source_ml_source1_concat_value385,
                )
            }
        }
        self::JavaScriptCallInstruction::PropertyKey(
            destination_66,
            input_20,
            source_ml_source1_f363,
        ) => {
            vec![destination_66, input_20]
        }
        self::JavaScriptCallInstruction::PropertyDefinition(
            destination_67,
            target_6,
            key_11,
            attributes,
            source_ml_source1_f364,
            source_ml_source1_f365,
        ) => {
            vec![destination_67, target_6, key_11, attributes]
        }
    }
}
