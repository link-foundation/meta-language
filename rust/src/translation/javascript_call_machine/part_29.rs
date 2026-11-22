// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptCallInstruction;
use super::JavaScriptSemanticValue;
use super::find_java_script_call_constructor_fact;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_code_registers(
    instruction: self::JavaScriptCallInstruction,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    match instruction.clone() {
        self::JavaScriptCallInstruction::Constant(destination, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination]))
        }
        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_2, source]))
        }
        self::JavaScriptCallInstruction::Argument(destination_3, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_3]))
        }
        self::JavaScriptCallInstruction::ArgumentCount(destination_4) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_4]))
        }
        self::JavaScriptCallInstruction::ArgumentAt(destination_5, index) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_5, index]))
        }
        self::JavaScriptCallInstruction::FunctionBind(destination_6) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_6]))
        }
        self::JavaScriptCallInstruction::BoundFunctionCall(destination_7) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_7]))
        }
        self::JavaScriptCallInstruction::FunctionToString(destination_8) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_8]))
        }
        self::JavaScriptCallInstruction::Branch(condition, _, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![condition]))
        }
        self::JavaScriptCallInstruction::Call(destination_9, callee, receiver, arguments) => {
            std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source2_concat_value3724 =
                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                        destination_9,
                        callee,
                        receiver,
                    ]));
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), {
                        let ml_reference_value340 =
                            source_ml_source2_concat_value3724.borrow().clone();
                        ml_reference_value340
                    }),
                    {
                        let ml_reference_value341 = arguments.borrow().clone();
                        ml_reference_value341
                    },
                )
            }))
        }
        self::JavaScriptCallInstruction::ArgumentListAppend(_, value) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![value]))
        }
        self::JavaScriptCallInstruction::ArgumentListCall(
            destination_10,
            _,
            callee_2,
            receiver_2,
            _,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_10,
            callee_2,
            receiver_2,
        ])),
        self::JavaScriptCallInstruction::Construct(
            destination_11,
            callee_3,
            new_target,
            arguments_2,
        ) => std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value3726 =
                std::rc::Rc::new(std::cell::RefCell::new(vec![
                    destination_11,
                    callee_3,
                    new_target,
                ]));
            self::ml_array::append(
                self::ml_array::append(Vec::<f64>::new(), {
                    let ml_reference_value342 = source_ml_source2_concat_value3726.borrow().clone();
                    ml_reference_value342
                }),
                {
                    let ml_reference_value343 = arguments_2.borrow().clone();
                    ml_reference_value343
                },
            )
        })),
        self::JavaScriptCallInstruction::ConstructorCheck(destination_12, input, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_12, input]))
        }
        self::JavaScriptCallInstruction::ReadNewTarget(destination_13) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_13]))
        }
        self::JavaScriptCallInstruction::ArrayJoinEnter(destination_14, receiver_3) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_14, receiver_3]))
        }
        self::JavaScriptCallInstruction::ArrayJoinLeave(receiver_4) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![receiver_4]))
        }
        self::JavaScriptCallInstruction::ConstructErrorClass(
            destination_15,
            callee_4,
            arguments_3,
        ) => std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value3728 =
                std::rc::Rc::new(std::cell::RefCell::new(vec![destination_15, callee_4]));
            self::ml_array::append(
                self::ml_array::append(Vec::<f64>::new(), {
                    let ml_reference_value344 = source_ml_source2_concat_value3728.borrow().clone();
                    ml_reference_value344
                }),
                {
                    let ml_reference_value345 = arguments_3.borrow().clone();
                    ml_reference_value345
                },
            )
        })),
        self::JavaScriptCallInstruction::CallClassConstructor(
            destination_16,
            callee_5,
            receiver_5,
            arguments_4,
        ) => std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value3730 =
                std::rc::Rc::new(std::cell::RefCell::new(vec![
                    destination_16,
                    callee_5,
                    receiver_5,
                ]));
            self::ml_array::append(
                self::ml_array::append(Vec::<f64>::new(), {
                    let ml_reference_value346 = source_ml_source2_concat_value3730.borrow().clone();
                    ml_reference_value346
                }),
                {
                    let ml_reference_value347 = arguments_4.borrow().clone();
                    ml_reference_value347
                },
            )
        })),
        self::JavaScriptCallInstruction::ConstructCollection(
            destination_17,
            callee_6,
            arguments_5,
        ) => std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value3732 =
                std::rc::Rc::new(std::cell::RefCell::new(vec![destination_17, callee_6]));
            self::ml_array::append(
                self::ml_array::append(Vec::<f64>::new(), {
                    let ml_reference_value348 = source_ml_source2_concat_value3732.borrow().clone();
                    ml_reference_value348
                }),
                {
                    let ml_reference_value349 = arguments_5.borrow().clone();
                    ml_reference_value349
                },
            )
        })),
        self::JavaScriptCallInstruction::CollectionAllocate(destination_18, prototype, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_18, prototype]))
        }
        self::JavaScriptCallInstruction::CollectionIteratorAllocate(
            destination_19,
            prototype_2,
            _,
            _,
            instruction_input,
        ) => {
            let input_2 = ((instruction_input.clone().unwrap_or_else(|| (None::<f64>,))).0).clone();
            if {
                let source_ml_source2_comparison_operand3735 = None::<()>;
                match (
                    input_2.clone(),
                    source_ml_source2_comparison_operand3735.clone(),
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }
            } {
                std::rc::Rc::new(std::cell::RefCell::new(vec![destination_19, prototype_2]))
            } else {
                std::rc::Rc::new(std::cell::RefCell::new(vec![
                    destination_19,
                    prototype_2,
                    input_2.clone().expect("a proven present value"),
                ]))
            }
        }
        self::JavaScriptCallInstruction::ArgumentRest(destination_20, prototype_3, _, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_20, prototype_3]))
        }
        self::JavaScriptCallInstruction::IteratorProtocol(
            destination_21,
            done,
            input_3,
            iterator_key,
            _,
            _,
            _,
            _,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_21,
            done,
            input_3,
            iterator_key,
        ])),
        self::JavaScriptCallInstruction::CollectionIteratorResult(
            destination_22,
            _,
            array_prototype,
            object_prototype,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_22,
            array_prototype,
            object_prototype,
        ])),
        self::JavaScriptCallInstruction::CollectionOperation(destination_23, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_23]))
        }
        self::JavaScriptCallInstruction::PrimitiveMethod(destination_24, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_24]))
        }
        self::JavaScriptCallInstruction::CollectionForEach(destination_25, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_25]))
        }
        self::JavaScriptCallInstruction::ArrayConcatenation(
            destination_26,
            input_4,
            spreadable_key,
            _,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_26,
            (((input_4.clone().unwrap_or_else(|| (None::<f64>,))).0).clone())
                .clone()
                .unwrap_or_else(|| 0f64),
            spreadable_key,
        ])),
        self::JavaScriptCallInstruction::ArraySlice(destination_27, input_5, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_27,
                (((input_5.clone().unwrap_or_else(|| (None::<f64>,))).0).clone())
                    .clone()
                    .unwrap_or_else(|| 0f64),
            ]))
        }
        self::JavaScriptCallInstruction::ArrayCreation(
            destination_28,
            _,
            prototype_4,
            iterator_key_2,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_28,
            prototype_4,
            iterator_key_2,
        ])),
        self::JavaScriptCallInstruction::ArrayCallback(destination_29, _, input_6) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_29,
                (((input_6.clone().unwrap_or_else(|| (None::<f64>,))).0).clone())
                    .clone()
                    .unwrap_or_else(|| 0f64),
            ]))
        }
        self::JavaScriptCallInstruction::ArraySearch(destination_30, _, input_7) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_30,
                (((input_7.clone().unwrap_or_else(|| (None::<f64>,))).0).clone())
                    .clone()
                    .unwrap_or_else(|| 0f64),
            ]))
        }
        self::JavaScriptCallInstruction::ArrayTransform(destination_31, _, _, input_8) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_31,
                (((input_8.clone().unwrap_or_else(|| (None::<f64>,))).0).clone())
                    .clone()
                    .unwrap_or_else(|| 0f64),
            ]))
        }
        self::JavaScriptCallInstruction::ObjectEnumeration(destination_32, _, prototype_5) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_32, prototype_5]))
        }
        self::JavaScriptCallInstruction::PropertyDefinitions(
            destination_33,
            _,
            target,
            input_9,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_33,
            target,
            input_9,
        ])),
        self::JavaScriptCallInstruction::PropertyReflection(
            destination_34,
            _,
            input_10,
            key,
            object_prototype_2,
            array_prototype_2,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_34,
            input_10,
            key,
            object_prototype_2,
            array_prototype_2,
        ])),
        self::JavaScriptCallInstruction::InstanceOf(destination_35, left, right, key_2, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_35,
                left,
                right,
                key_2,
            ]))
        }
        self::JavaScriptCallInstruction::ObjectObservation(destination_36, _, input_11, other) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_36,
                input_11,
                other,
            ]))
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
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_37,
            target_2,
            input_12,
            adder,
            iterator_key_3,
            iterator_method,
            iterator_prototype,
            next_method,
        ])),
        self::JavaScriptCallInstruction::ConstructErrorValue(
            destination_38,
            constructor,
            new_target_2,
            prototype_6,
            message,
            present,
            _,
            _,
            _,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_38,
            constructor,
            new_target_2,
            prototype_6,
            message,
            present,
        ])),
        self::JavaScriptCallInstruction::ReadDerivedReceiver(destination_39) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_39]))
        }
        self::JavaScriptCallInstruction::InvokeErrorIntrinsic(destination_40) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_40]))
        }
        self::JavaScriptCallInstruction::InvokeHostCapability(destination_41) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_41]))
        }
        self::JavaScriptCallInstruction::Return(source_2) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![source_2]))
        }
        self::JavaScriptCallInstruction::Throw(source_3) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![source_3]))
        }
        self::JavaScriptCallInstruction::PushHandler(destination_42, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_42]))
        }
        self::JavaScriptCallInstruction::NumberBinary(destination_43, left_2, right_2, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_43,
                left_2,
                right_2,
            ]))
        }
        self::JavaScriptCallInstruction::SameValue(destination_44, left_3, right_3) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_44,
                left_3,
                right_3,
            ]))
        }
        self::JavaScriptCallInstruction::StrictEqual(destination_45, left_4, right_4) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_45,
                left_4,
                right_4,
            ]))
        }
        self::JavaScriptCallInstruction::RequireWrite(success) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![success]))
        }
        self::JavaScriptCallInstruction::Primitive(destination_46, _, arguments_6) => {
            std::rc::Rc::new(std::cell::RefCell::new({
                let source_ml_source2_concat_value3736 =
                    std::rc::Rc::new(std::cell::RefCell::new(vec![destination_46]));
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), {
                        let ml_reference_value350 =
                            source_ml_source2_concat_value3736.borrow().clone();
                        ml_reference_value350
                    }),
                    {
                        let ml_reference_value351 = arguments_6.borrow().clone();
                        ml_reference_value351
                    },
                )
            }))
        }
        self::JavaScriptCallInstruction::OrdinaryAllocate(destination_47, prototype_7) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_47, prototype_7]))
        }
        self::JavaScriptCallInstruction::LiteralPrototype(destination_48, object, value_2) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_48,
                object,
                value_2,
            ]))
        }
        self::JavaScriptCallInstruction::LiteralAccessor(
            destination_49,
            object_2,
            key_3,
            value_3,
            _,
            _,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_49,
            object_2,
            key_3,
            value_3,
        ])),
        self::JavaScriptCallInstruction::ObjectFromEntries(
            destination_50,
            input_13,
            prototype_8,
            iterator_key_4,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_50,
            input_13,
            prototype_8,
            iterator_key_4,
        ])),
        self::JavaScriptCallInstruction::PrimitiveWrapper(
            destination_51,
            input_14,
            prototype_9,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_51,
            input_14,
            prototype_9,
        ])),
        self::JavaScriptCallInstruction::SymbolAllocate(destination_52, input_15) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_52, input_15]))
        }
        self::JavaScriptCallInstruction::SymbolDescription(destination_53, input_16) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_53, input_16]))
        }
        self::JavaScriptCallInstruction::SymbolRegistry(destination_54, input_17, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_54, input_17]))
        }
        self::JavaScriptCallInstruction::FunctionName(destination_55, object_3, key_4, _, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_55,
                object_3,
                key_4,
            ]))
        }
        self::JavaScriptCallInstruction::ArrayAllocate(destination_56, prototype_10, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_56, prototype_10]))
        }
        self::JavaScriptCallInstruction::ClosureAllocate(
            destination_57,
            _,
            _,
            captures,
            receiver_6,
        ) => std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value3738 =
                std::rc::Rc::new(std::cell::RefCell::new(vec![destination_57, receiver_6]));
            self::ml_array::append(
                self::ml_array::append(Vec::<f64>::new(), {
                    let ml_reference_value352 = source_ml_source2_concat_value3738.borrow().clone();
                    ml_reference_value352
                }),
                {
                    let ml_reference_value353 = captures.borrow().clone();
                    ml_reference_value353
                },
            )
        })),
        self::JavaScriptCallInstruction::ArraySpread(
            destination_58,
            target_3,
            input_18,
            index_2,
            iterator_key_5,
            _,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_58,
            target_3,
            input_18,
            index_2,
            iterator_key_5,
        ])),
        self::JavaScriptCallInstruction::ObjectSpread(
            destination_59,
            target_4,
            input_19,
            _,
            excluded,
            _,
        ) => std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value3740 =
                std::rc::Rc::new(std::cell::RefCell::new(vec![
                    destination_59,
                    target_4,
                    input_19,
                ]));
            {
                let source_ml_source2_concat_value3741 = (((excluded
                    .clone()
                    .unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>,)))
                .0)
                    .clone())
                .clone()
                .unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())));
                self::ml_array::append(
                    self::ml_array::append(Vec::<f64>::new(), {
                        let ml_reference_value354 =
                            source_ml_source2_concat_value3740.borrow().clone();
                        ml_reference_value354
                    }),
                    {
                        let ml_reference_value355 =
                            source_ml_source2_concat_value3741.borrow().clone();
                        ml_reference_value355
                    },
                )
            }
        })),
        self::JavaScriptCallInstruction::OrdinaryDataProperty(
            destination_60,
            object_4,
            key_5,
            value_4,
            _,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_60,
            object_4,
            key_5,
            value_4,
        ])),
        self::JavaScriptCallInstruction::OrdinaryGet(
            destination_61,
            object_5,
            key_6,
            receiver_7,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_61,
            object_5,
            key_6,
            receiver_7,
        ])),
        self::JavaScriptCallInstruction::OrdinaryDelete(destination_62, object_6, key_7) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_62,
                object_6,
                key_7,
            ]))
        }
        self::JavaScriptCallInstruction::OrdinaryHas(destination_63, object_7, key_8) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![
                destination_63,
                object_7,
                key_8,
            ]))
        }
        self::JavaScriptCallInstruction::OrdinarySet(
            destination_64,
            object_8,
            key_9,
            value_5,
            receiver_8,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_64,
            object_8,
            key_9,
            value_5,
            receiver_8,
        ])),
        self::JavaScriptCallInstruction::OrdinaryDefine(
            destination_65,
            target_5,
            key_10,
            fields,
            _,
        ) => std::rc::Rc::new(std::cell::RefCell::new({
            let source_ml_source2_concat_value3742 =
                std::rc::Rc::new(std::cell::RefCell::new(vec![
                    destination_65,
                    target_5,
                    key_10,
                ]));
            self::ml_array::append(
                self::ml_array::append(Vec::<f64>::new(), {
                    let ml_reference_value356 = source_ml_source2_concat_value3742.borrow().clone();
                    ml_reference_value356
                }),
                {
                    let ml_reference_value357 = fields.borrow().clone();
                    ml_reference_value357
                },
            )
        })),
        self::JavaScriptCallInstruction::PropertyKey(destination_66, input_20, _) => {
            std::rc::Rc::new(std::cell::RefCell::new(vec![destination_66, input_20]))
        }
        self::JavaScriptCallInstruction::PropertyDefinition(
            destination_67,
            target_6,
            key_11,
            attributes,
            _,
            _,
        ) => std::rc::Rc::new(std::cell::RefCell::new(vec![
            destination_67,
            target_6,
            key_11,
            attributes,
        ])),
        _ => std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())),
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
        _ => (-1f64),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_strict_equal(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone()))
        != (self::ml_string::Text::from_units(vec![])))
        || ((self::read_java_script_semantic_value_error(right.clone()))
            != (self::ml_string::Text::from_units(vec![]))))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                115, 116, 114, 105, 99, 116, 32, 101, 113, 117, 97, 108, 105, 116, 121, 32, 114,
                101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32,
                115, 101, 109, 97, 110, 116, 105, 99, 32, 111, 112, 101, 114, 97, 110, 100, 115,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Number(left_value) => match right.clone() {
                self::JavaScriptSemanticValue::Number(value) => (left_value == value),
                _ => false,
            },
            _ => self::test_java_script_semantic_same_value(left.clone(), right.clone()),
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_array_constructor_fact(
    facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptArrayConstructorFact>>>,
    identity: self::ml::Big,
) -> f64 {
    self::find_java_script_call_constructor_fact(facts.clone(), identity.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn copy_java_script_array_constructor_fact(
    fact: self::JavaScriptArrayConstructorFact,
) -> self::JavaScriptArrayConstructorFact {
    match fact.clone() {
        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
        ) => {
            let source_ml_source2_nominal_field3925 = shared_constructor_field1.clone();
            {
                let source_ml_source2_nominal_field3927 = shared_constructor_field2;
                {
                    let source_ml_source2_nominal_field3929 = shared_constructor_field3.clone();
                    {
                        let source_ml_source2_nominal_field3931 = shared_constructor_field4;
                        {
                            let source_ml_source2_nominal_field3933 =
                                shared_constructor_field5.clone();
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                source_ml_source2_nominal_field3925.clone(),
                                source_ml_source2_nominal_field3927,
                                source_ml_source2_nominal_field3929.clone(),
                                source_ml_source2_nominal_field3931,
                                source_ml_source2_nominal_field3933.clone(),
                            )
                        }
                    }
                }
            }
        }
    }
}
