// Generated from complete js/src/translation/javascript-collection-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionCallCompilation;
use super::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Result;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_collection_constructor_kind_loop459(mut kind: self::ml_string::Text, mut source_ml_temporary_values458: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, mut source_ml_temporary_index458: self::ml::Big) -> self::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Result {
    loop {
        return if (source_ml_temporary_index458 < ({ let ml_reference_array684 = source_ml_temporary_values458.clone(); let ml_reference_projection684 = self::ml::Big::from_u128(ml_reference_array684.borrow().len() as u128); ml_reference_projection684 })) {
            {
                let instruction = { let ml_reference_array685 = source_ml_temporary_values458.clone(); let ml_reference_position685 = source_ml_temporary_index458.clone(); let ml_reference_projection685 = self::ml_array::at(&*ml_reference_array685.borrow(), ml_reference_position685.to_index()); ml_reference_projection685 };
                match instruction.clone() {
                    self::JavaScriptCallInstruction::Constant(_, _) => {
                        {
                            let source_ml_temporary_index458_2 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_2.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::Copy(_, _) => {
                        {
                            let source_ml_temporary_index458_3 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_3.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::Argument(_, _) => {
                        {
                            let source_ml_temporary_index458_4 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_4.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArgumentCount(_) => {
                        {
                            let source_ml_temporary_index458_5 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_5.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArgumentAt(_, _) => {
                        {
                            let source_ml_temporary_index458_6 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_6.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::FunctionBind(_) => {
                        {
                            let source_ml_temporary_index458_7 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_7.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::BoundFunctionCall(_) => {
                        {
                            let source_ml_temporary_index458_8 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_8.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::FunctionToString(_) => {
                        {
                            let source_ml_temporary_index458_9 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_9.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::PushFinally(_) => {
                        {
                            let source_ml_temporary_index458_10 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_10.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::FinishFinally => {
                        {
                            let source_ml_temporary_index458_11 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_11.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::CompleteJump(_, _) => {
                        {
                            let source_ml_temporary_index458_12 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_12.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::Jump(_) => {
                        {
                            let source_ml_temporary_index458_13 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_13.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::Branch(_, _, _) => {
                        {
                            let source_ml_temporary_index458_14 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_14.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_15 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_15.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArgumentListOpen(_) => {
                        {
                            let source_ml_temporary_index458_16 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_16.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArgumentListAppend(_, _) => {
                        {
                            let source_ml_temporary_index458_17 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_17.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_18 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_18.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::Construct(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_19 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_19.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ConstructorCheck(_, _, _) => {
                        {
                            let source_ml_temporary_index458_20 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_20.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ReadNewTarget(_) => {
                        {
                            let source_ml_temporary_index458_21 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_21.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
                        {
                            let source_ml_temporary_index458_22 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_22.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArrayJoinLeave(_) => {
                        {
                            let source_ml_temporary_index458_23 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_23.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ConstructErrorClass(_, _, _) => {
                        {
                            let source_ml_temporary_index458_24 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_24.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::CallClassConstructor(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_25 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_25.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ConstructCollection(_, _, _) => {
                        {
                            let source_ml_temporary_index458_26 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_26.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::CollectionAllocate(_, _, instruction_kind) => {
                        if ((kind != (self::ml_string::Text::from_units(vec![]))) || !(self::ml_array::includes(&{ let ml_reference_value686 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![77, 97, 112]), self::ml_string::Text::from_units(vec![83, 101, 116]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112]), self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])]))).borrow().clone(); ml_reference_value686 }, &instruction_kind.clone(), 0f64, false, |a, b| a == b))) {
                            self::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Result::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Return(self::ml_string::Text::from_units(vec![]))
                        } else {
                            {
                                let source_ml_temporary_index458_27 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                                {
                                    (kind, source_ml_temporary_index458) = (instruction_kind.clone(), source_ml_temporary_index458_27.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_28 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_28.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArgumentRest(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_29 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_29.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_30 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_30.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_31 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_31.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::CollectionOperation(_, _) => {
                        {
                            let source_ml_temporary_index458_32 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_32.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::PrimitiveMethod(_, _) => {
                        {
                            let source_ml_temporary_index458_33 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_33.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::CollectionForEach(_, _) => {
                        {
                            let source_ml_temporary_index458_34 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_34.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_35 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_35.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArraySlice(_, _, _) => {
                        {
                            let source_ml_temporary_index458_36 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_36.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArrayCreation(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_37 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_37.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArrayCallback(_, _, _) => {
                        {
                            let source_ml_temporary_index458_38 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_38.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArraySearch(_, _, _) => {
                        {
                            let source_ml_temporary_index458_39 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_39.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArrayTransform(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_40 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_40.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ObjectEnumeration(_, _, _) => {
                        {
                            let source_ml_temporary_index458_41 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_41.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::PropertyDefinitions(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_42 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_42.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::PropertyReflection(_, _, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_43 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_43.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::InstanceOf(_, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_44 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_44.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ObjectObservation(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_45 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_45.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::CollectionCopy(_, _, _, _, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_46 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_46.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_47 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_47.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ReadDerivedReceiver(_) => {
                        {
                            let source_ml_temporary_index458_48 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_48.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::InvokeErrorIntrinsic(_) => {
                        {
                            let source_ml_temporary_index458_49 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_49.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::InvokeHostCapability(_) => {
                        {
                            let source_ml_temporary_index458_50 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_50.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::Return(_) => {
                        {
                            let source_ml_temporary_index458_51 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_51.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::Throw(_) => {
                        {
                            let source_ml_temporary_index458_52 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_52.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::PushHandler(_, _) => {
                        {
                            let source_ml_temporary_index458_53 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_53.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::PopHandler => {
                        {
                            let source_ml_temporary_index458_54 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_54.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_55 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_55.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                        {
                            let source_ml_temporary_index458_56 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_56.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                        {
                            let source_ml_temporary_index458_57 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_57.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::RequireWrite(_) => {
                        {
                            let source_ml_temporary_index458_58 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_58.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                        {
                            let source_ml_temporary_index458_59 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_59.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::OrdinaryAllocate(_, _) => {
                        {
                            let source_ml_temporary_index458_60 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_60.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::LiteralPrototype(_, _, _) => {
                        {
                            let source_ml_temporary_index458_61 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_61.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::LiteralAccessor(_, _, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_62 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_62.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_63 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_63.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, _) => {
                        {
                            let source_ml_temporary_index458_64 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_64.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::SymbolAllocate(_, _) => {
                        {
                            let source_ml_temporary_index458_65 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_65.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::SymbolDescription(_, _) => {
                        {
                            let source_ml_temporary_index458_66 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_66.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::SymbolRegistry(_, _, _) => {
                        {
                            let source_ml_temporary_index458_67 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_67.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::FunctionName(_, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_68 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_68.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArrayAllocate(_, _, _) => {
                        {
                            let source_ml_temporary_index458_69 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_69.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_70 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_70.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_71 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_71.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_72 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_72.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::OrdinaryDataProperty(_, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_73 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_73.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                        {
                            let source_ml_temporary_index458_74 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_74.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::OrdinaryDelete(_, _, _) => {
                        {
                            let source_ml_temporary_index458_75 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_75.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::OrdinaryHas(_, _, _) => {
                        {
                            let source_ml_temporary_index458_76 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_76.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_77 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_77.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::OrdinaryDefine(_, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_78 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_78.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                        {
                            let source_ml_temporary_index458_79 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_79.clone();
                                continue;
                            }
                        }
                    }
                    self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _, _) => {
                        {
                            let source_ml_temporary_index458_80 = source_ml_temporary_index458.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index458 = source_ml_temporary_index458_80.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Result::MlTemporaryReadJavaScriptCollectionConstructorKindLoop459Done(kind.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptIterableCollectionConstructorJoin462 {
    MlTemporaryCompileJavaScriptIterableCollectionConstructorJoin462Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCompileJavaScriptCollectionObjectMethodJoin507 {
    MlTemporaryCompileJavaScriptCollectionObjectMethodJoin507Next,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Result {
    MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Done,
    MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Return(Box<self::JavaScriptCollectionCallCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_collection_call_directory_loop528(mut empty: self::JavaScriptCallProgram, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut program: self::JavaScriptCallProgram, mut at: f64) -> self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Result {
    loop {
        return if (at < ({ let ml_reference_array687 = functions.clone(); let ml_reference_projection687 = ml_reference_array687.borrow().len() as f64; ml_reference_projection687 })) {
            {
                let fn_ = { let ml_reference_array688 = functions.clone(); let ml_reference_position688 = at; let ml_reference_projection688 = self::ml_array::optional_at(&*ml_reference_array688.borrow(), self::ml_array::number_index(ml_reference_position688)); ml_reference_projection688 };
                if (match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((self::find_java_script_call_code_function(program.clone(), match fn_.clone().expect("a proven present value") {
                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value3503, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3503.clone()
                    }
                })) != at)) {
                    self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![112, 111, 114, 116, 97, 98, 108, 101, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 114, 116, 101, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop528Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Result {
    MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Done,
    MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Return(Box<self::JavaScriptCollectionCallCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_collection_call_directory_loop527(mut empty: self::JavaScriptCallProgram, mut identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>, mut functions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut source_ml_temporary_values524: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallFunction>>>, mut source_ml_temporary_index524: self::ml::Big) -> self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Result {
    loop {
        return if (source_ml_temporary_index524 < ({ let ml_reference_array689 = source_ml_temporary_values524.clone(); let ml_reference_projection689 = self::ml::Big::from_u128(ml_reference_array689.borrow().len() as u128); ml_reference_projection689 })) {
            {
                let fn_ = { let ml_reference_array690 = source_ml_temporary_values524.clone(); let ml_reference_position690 = source_ml_temporary_index524.clone(); let ml_reference_projection690 = self::ml_array::at(&*ml_reference_array690.borrow(), ml_reference_position690.to_index()); ml_reference_projection690 };
                if self::ml_array::includes(&{ let ml_reference_value691 = identities.borrow().clone(); ml_reference_value691 }, &match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value3506, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value3506.clone()
                    }
                }, 0f64, false, |a, b| a == b) {
                    self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Return(Box::new(self::JavaScriptCollectionCallCompilation::CollectionCallCompilation(Box::new(empty.clone()), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 111, 100, 101, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 44, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 109, 101, 116, 104, 111, 100, 32, 111, 119, 110, 101, 114]))))
                } else {
                    {
                        let source_ml_temporary_compile_java_script_collection_call_directory_ignored529 = { let ml_method_owner692 = identities.clone(); let ml_method_argument692_0 = match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value3507, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value3507.clone()
                            }
                        }; let ml_inserted = vec![ml_method_argument692_0]; let mut ml_method_values692 = ml_method_owner692.borrow_mut(); let ml_length = ml_method_values692.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values692.extend(ml_inserted); ml_length as f64 };
                        {
                            let source_ml_temporary_compile_java_script_collection_call_directory_ignored530 = { let ml_method_owner693 = functions.clone(); let ml_method_argument693_0 = fn_.clone(); let ml_inserted = vec![ml_method_argument693_0]; let mut ml_method_values693 = ml_method_owner693.borrow_mut(); let ml_length = ml_method_values693.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values693.extend(ml_inserted); ml_length as f64 };
                            {
                                let source_ml_temporary_index524_2 = source_ml_temporary_index524.add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_temporary_index524 = source_ml_temporary_index524_2.clone();
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Result::MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop527Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Result {
    MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Done,
    MlTemporaryCompileJavaScriptCollectionCallDirectoryLoop526Return(Box<self::JavaScriptCollectionCallCompilation>),
}
