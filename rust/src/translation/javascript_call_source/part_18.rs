// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceControlScope;
use super::JavaScriptCallSourceControlTransfer;
use super::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result;
use super::find_java_script_call_source_instruction_position;
use super::ml;
use super::ml_array;
use super::source_ml_source1_temporary_finish_java_script_call_source_control_scope_loop61;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_call_source_control_scope(builder: self::JavaScriptCallSourceBuilder, outer: self::JavaScriptCallSourceBuilder, target: f64, end: f64, update: f64) -> self::JavaScriptCallSourceBuilder {
    {
        let transfers = (match builder.clone() {
            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12948) => {
                ((source_ml_source2_data_property_value12948.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlTransfer>>>>,))).0).clone()
            }
        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlTransfer>::new())));
        {
            let selected = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values153 = { let ml_reference_value154 = transfers.borrow().clone(); ml_reference_value154 }; let ml_callback153 = { let target = target; std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlTransfer| -> bool { ((match row.clone() {
                self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(_, source_ml_source2_data_property_value12950, _) => {
                    source_ml_source2_data_property_value12950
                }
            }) == target) }) }; ml_values153.iter().cloned().enumerate().filter_map(|(ml_index153, ml_value153)| if ml_callback153(ml_value153.clone()) { Some(ml_value153) } else { None }).collect::<Vec<_>>() }));
            if { let ml_values155 = { let ml_reference_value156 = selected.borrow().clone(); ml_reference_value156 }; let ml_callback155 = { let update = update; std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlTransfer| -> bool { (match row.clone() {
                self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(_, _, source_ml_source2_data_property_value12951) => {
                    source_ml_source2_data_property_value12951
                }
            } && (update < (0f64))) }) }; ml_values155.iter().cloned().enumerate().any(|(ml_index155, ml_value155)| ml_callback155(ml_value155)) } {
                {
                    let source_ml_source2_nominal_field12953 = match builder.clone() {
                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12954, _, _, _, _, _, _, _, _, _) => {
                            source_ml_source2_data_property_value12954.clone()
                        }
                    };
                    {
                        let source_ml_source2_nominal_field12955 = match builder.clone() {
                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12956, _, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value12956
                            }
                        };
                        {
                            let source_ml_source2_nominal_field12957 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12958, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value12958
                                }
                            };
                            {
                                let source_ml_source2_nominal_field12959 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12960, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12960
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field12961 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12962, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value12962
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field12963 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12964, _, _, _, _) => {
                                                source_ml_source2_data_property_value12964.clone()
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field12965 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12966, _, _, _) => {
                                                    source_ml_source2_data_property_value12966.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field12967 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12968, _, _) => {
                                                        source_ml_source2_data_property_value12968.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12969 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12970, _) => {
                                                            source_ml_source2_data_property_value12970.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12971 = match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12972) => {
                                                                source_ml_source2_data_property_value12972.clone()
                                                            }
                                                        };
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12953.clone(), source_ml_source2_nominal_field12955, source_ml_source2_nominal_field12957, source_ml_source2_nominal_field12959, source_ml_source2_nominal_field12961, String::from("continue requires its actual loop update or condition target"), source_ml_source2_nominal_field12965.clone(), source_ml_source2_nominal_field12967.clone(), source_ml_source2_nominal_field12969.clone(), source_ml_source2_nominal_field12971.clone())
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
                {
                    let positions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values157 = { let ml_reference_value158 = selected.borrow().clone(); ml_reference_value158 }; let ml_callback157 = std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlTransfer| -> f64 { match row.clone() {
                        self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(source_ml_source2_data_property_value12974, _, _) => {
                            source_ml_source2_data_property_value12974
                        }
                    } }); ml_values157.iter().cloned().enumerate().map(|(ml_index157, ml_value157)| ml_callback157(ml_value157)).collect::<Vec<_>>() }));
                    {
                        let invalid = {
                            let source_ml_source2_nominal_field12976 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12977, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value12977.clone()
                                }
                            };
                            {
                                let source_ml_source2_nominal_field12978 = match builder.clone() {
                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value12979, _, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value12979
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field12980 = match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value12981, _, _, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value12981
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field12982 = match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value12983, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value12983
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_field12984 = match builder.clone() {
                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value12985, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value12985
                                                }
                                            };
                                            {
                                                let source_ml_source2_nominal_field12986 = match builder.clone() {
                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value12987, _, _, _, _) => {
                                                        source_ml_source2_data_property_value12987.clone()
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_nominal_field12988 = match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value12989, _, _, _) => {
                                                            source_ml_source2_data_property_value12989.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field12990 = match builder.clone() {
                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value12991, _, _) => {
                                                                source_ml_source2_data_property_value12991.clone()
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_field12992 = match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value12993, _) => {
                                                                    source_ml_source2_data_property_value12993.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field12994 = match builder.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value12995) => {
                                                                        source_ml_source2_data_property_value12995.clone()
                                                                    }
                                                                };
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_nominal_field12976.clone(), source_ml_source2_nominal_field12978, source_ml_source2_nominal_field12980, source_ml_source2_nominal_field12982, source_ml_source2_nominal_field12984, String::from("source control transfers retain ordered unique instruction positions"), source_ml_source2_nominal_field12988.clone(), source_ml_source2_nominal_field12990.clone(), source_ml_source2_nominal_field12992.clone(), source_ml_source2_nominal_field12994.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        {
                            let previous = (-1f64);
                            {
                                let source_ml_source1_temporary_values60 = positions.clone();
                                {
                                    let source_ml_source1_temporary_index60 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s12997 = self::source_ml_source1_temporary_finish_java_script_call_source_control_scope_loop61(builder.clone(), invalid.clone(), previous, source_ml_source1_temporary_values60.clone(), source_ml_source1_temporary_index60.clone());
                                        match source_ml_source2_s12997.clone() {
                                            self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Done => {
                                                {
                                                    let instructions = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values159 = { let ml_reference_value160 = (match builder.clone() {
                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value12998, _, _, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value12998.clone()
                                                        }
                                                    }).borrow().clone(); ml_reference_value160 }; let ml_callback159 = { let positions = positions.clone(); let selected = selected.clone(); let update = update; let end = end; std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { {
                                                        let position = self::find_java_script_call_source_instruction_position(positions.clone(), index);
                                                        if (position < (0f64)) {
                                                            instruction.clone()
                                                        } else {
                                                            {
                                                                let transfer = {
                                                                    let source_ml_source2_array_read_value12999 = selected.clone();
                                                                    {
                                                                        let source_ml_source2_array_read_index13000 = position;
                                                                        { let ml_reference_array161 = source_ml_source2_array_read_value12999.clone(); let ml_reference_position161 = source_ml_source2_array_read_index13000; let ml_reference_projection161 = self::ml_array::optional_at(&*ml_reference_array161.borrow(), self::ml_array::number_index(ml_reference_position161)); ml_reference_projection161 }
                                                                    }
                                                                };
                                                                if match (transfer.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    instruction.clone()
                                                                } else {
                                                                    {
                                                                        let destination = if match transfer.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(_, _, source_ml_source2_data_property_value13001) => {
                                                                                source_ml_source2_data_property_value13001
                                                                            }
                                                                        } {
                                                                            update
                                                                        } else {
                                                                            end
                                                                        };
                                                                        match instruction.clone() {
                                                                            self::JavaScriptCallInstruction::Constant(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::Copy(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::Argument(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArgumentCount(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArgumentAt(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::FunctionBind(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::BoundFunctionCall(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::FunctionToString(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::PushFinally(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::FinishFinally => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::CompleteJump(_, depth) => {
                                                                                self::JavaScriptCallInstruction::CompleteJump(destination, depth)
                                                                            }
                                                                            self::JavaScriptCallInstruction::Jump(_) => {
                                                                                self::JavaScriptCallInstruction::Jump(destination)
                                                                            }
                                                                            self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArgumentListOpen(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArgumentListAppend(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::Construct(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ConstructorCheck(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ReadNewTarget(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArrayJoinLeave(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ConstructErrorClass(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::CallClassConstructor(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ConstructCollection(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::CollectionAllocate(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArgumentRest(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::CollectionOperation(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::PrimitiveMethod(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::CollectionForEach(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArraySlice(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArrayCreation(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArrayCallback(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArraySearch(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArrayTransform(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ObjectEnumeration(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::PropertyDefinitions(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::PropertyReflection(_, _, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::InstanceOf(_, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ObjectObservation(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::CollectionCopy(_, _, _, _, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ReadDerivedReceiver(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::InvokeErrorIntrinsic(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::InvokeHostCapability(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::Return(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::Throw(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::PopHandler => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::OrdinaryAllocate(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::LiteralPrototype(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::LiteralAccessor(_, _, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::SymbolAllocate(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::SymbolDescription(_, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::SymbolRegistry(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::FunctionName(_, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArrayAllocate(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::OrdinaryDataProperty(_, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::OrdinaryDelete(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::OrdinaryHas(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::OrdinaryDefine(_, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                            self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _, _) => {
                                                                                instruction.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } }) }; ml_values159.iter().cloned().enumerate().map(|(ml_index159, ml_value159)| ml_callback159(ml_value159, (ml_index159 as f64))).collect::<Vec<_>>() }));
                                                    {
                                                        let remaining = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values162 = { let ml_reference_value163 = transfers.borrow().clone(); ml_reference_value163 }; let ml_callback162 = { let target = target; std::rc::Rc::new(move |row: self::JavaScriptCallSourceControlTransfer| -> bool { ((match row.clone() {
                                                            self::JavaScriptCallSourceControlTransfer::SourceControlTransfer(_, source_ml_source2_data_property_value13249, _) => {
                                                                source_ml_source2_data_property_value13249
                                                            }
                                                        }) != target) }) }; ml_values162.iter().cloned().enumerate().filter_map(|(ml_index162, ml_value162)| if ml_callback162(ml_value162.clone()) { Some(ml_value162) } else { None }).collect::<Vec<_>>() }));
                                                        {
                                                            let source_ml_source2_nominal_field13252 = match builder.clone() {
                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value13253, _, _, _, _, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value13253.clone()
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_nominal_field13254 = match builder.clone() {
                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source2_data_property_value13255, _, _, _, _, _, _, _, _) => {
                                                                        source_ml_source2_data_property_value13255
                                                                    }
                                                                };
                                                                {
                                                                    let source_ml_source2_nominal_field13256 = match builder.clone() {
                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, source_ml_source2_data_property_value13257, _, _, _, _, _, _, _) => {
                                                                            source_ml_source2_data_property_value13257
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_nominal_field13258 = match builder.clone() {
                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value13259, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value13259
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_nominal_field13260 = match builder.clone() {
                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value13261, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value13261
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_nominal_field13262 = match builder.clone() {
                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, source_ml_source2_data_property_value13263, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value13263.clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_nominal_field13264 = match builder.clone() {
                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, source_ml_source2_data_property_value13265, _, _, _) => {
                                                                                            source_ml_source2_data_property_value13265.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_nominal_field13266 = match builder.clone() {
                                                                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, source_ml_source2_data_property_value13267, _, _) => {
                                                                                                source_ml_source2_data_property_value13267.clone()
                                                                                            }
                                                                                        };
                                                                                        {
                                                                                            let source_ml_source2_nominal_field13268 = match builder.clone() {
                                                                                                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13269, _) => {
                                                                                                    source_ml_source2_data_property_value13269.clone()
                                                                                                }
                                                                                            };
                                                                                            {
                                                                                                let source_ml_source2_nominal_field13270 = match builder.clone() {
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13271) => {
                                                                                                        source_ml_source2_data_property_value13271.clone()
                                                                                                    }
                                                                                                };
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override13273 = (match outer.clone() {
                                                                                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13250, _) => {
                                                                                                            ((source_ml_source2_data_property_value13250.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceControlScope>>>>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallSourceControlScope>::new())));
                                                                                                    self::JavaScriptCallSourceBuilder::SourceBuilder(instructions.clone(), source_ml_source2_nominal_field13254, source_ml_source2_nominal_field13256, source_ml_source2_nominal_field13258, source_ml_source2_nominal_field13260, source_ml_source2_nominal_field13262.clone(), source_ml_source2_nominal_field13264.clone(), source_ml_source2_nominal_field13266.clone(), Some((Some(source_ml_source2_nominal_override13273.clone()),)), Some((Some(remaining.clone()),)))
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
                                            self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Return(source_ml_source1_temporary_result61) => {
                                                let source_ml_source1_temporary_result61 = *source_ml_source1_temporary_result61;
                                                source_ml_source1_temporary_result61.clone()
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
