// Generated from complete js/src/translation/javascript-error-class-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::JavaScriptCallSourceSwitchClause;
use super::JavaScriptSemanticValue;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_source_spread_iterator_slot_loop51(mut instructions: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>>, mut iterator: f64) -> f64 {
    loop {
        return if (iterator < (65536f64)) {
            {
                let candidate = iterator;
                if !({ let ml_values431 = { let ml_reference_value432 = instructions.borrow().clone(); ml_reference_value432 }; let ml_callback431 = { let candidate = candidate; std::rc::Rc::new(move |row: self::JavaScriptCallInstruction| -> bool { match row.clone() {
                    self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, slot, _, _, _) => {
                        (slot == candidate)
                    }
                    _ => {
                        false
                    }
                } }) }; ml_values431.iter().cloned().enumerate().any(|(ml_index431, ml_value431)| ml_callback431(ml_value431)) }) {
                    iterator
                } else {
                    {
                        let iterator_2 = (iterator + 1f64);
                        {
                            iterator = iterator_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            iterator
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result {
    MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Done,
    MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop59(mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut captures: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result {
    loop {
        return if (index < ({ let ml_reference_array433 = captures.clone(); let ml_reference_projection433 = ml_reference_array433.borrow().len() as f64; ml_reference_projection433 })) {
            {
                let owner = ({ let ml_reference_array434 = captures.clone(); let ml_reference_position434 = index; let ml_reference_projection434 = self::ml_array::optional_at(&*ml_reference_array434.borrow(), self::ml_array::number_index(ml_reference_position434)); ml_reference_projection434 }).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || self::ml_array::includes(&{ let ml_reference_value435 = parameters.borrow().clone(); ml_reference_value435 }, &owner, 0f64, false, |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()))) || ((self::ml_array::index_of(&{ let ml_reference_value436 = captures.borrow().clone(); ml_reference_value436 }, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new())), 0f64, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 111, 119, 110, 101, 114, 115]))))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop59Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result {
    MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Done,
    MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_call_source_binding_reads_loop58(mut parameters: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result {
    loop {
        return if (index < ({ let ml_reference_array437 = parameters.clone(); let ml_reference_projection437 = ml_reference_array437.borrow().len() as f64; ml_reference_projection437 })) {
            {
                let owner = ({ let ml_reference_array438 = parameters.clone(); let ml_reference_position438 = index; let ml_reference_projection438 = self::ml_array::optional_at(&*ml_reference_array438.borrow(), self::ml_array::number_index(ml_reference_position438)); ml_reference_projection438 }).clone().unwrap_or_else(|| (-1f64));
                if (!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || ((self::ml_array::index_of(&{ let ml_reference_value439 = parameters.borrow().clone(); ml_reference_value439 }, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallInstruction>::new())), 0f64, self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 111, 119, 110, 101, 114, 115]))))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Result::MlSource1TemporaryCompileJavaScriptCallSourceBindingReadsLoop58Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result {
    MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Done,
    MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Return(Box<self::JavaScriptCallSourceBuilder>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_finish_java_script_call_source_control_scope_loop61(mut builder: self::JavaScriptCallSourceBuilder, mut invalid: self::JavaScriptCallSourceBuilder, mut previous: f64, mut source_ml_source1_temporary_values60: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut source_ml_source1_temporary_index60: self::ml::Big) -> self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result {
    loop {
        return if (source_ml_source1_temporary_index60 < ({ let ml_reference_array440 = source_ml_source1_temporary_values60.clone(); let ml_reference_projection440 = self::ml::Big::from_u128(ml_reference_array440.borrow().len() as u128); ml_reference_projection440 })) {
            {
                let position = { let ml_reference_array441 = source_ml_source1_temporary_values60.clone(); let ml_reference_position441 = source_ml_source1_temporary_index60.clone(); let ml_reference_projection441 = self::ml_array::at(&*ml_reference_array441.borrow(), ml_reference_position441.to_index()); ml_reference_projection441 };
                if (!(self::test_java_script_call_code_coordinate(position, { let ml_reference_array442 = match builder.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value19350, _, _, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value19350.clone()
                    }
                }; let ml_reference_projection442 = ml_reference_array442.borrow().len() as f64; ml_reference_projection442 })) || (position <= previous)) {
                    self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Return(Box::new(invalid.clone()))
                } else {
                    {
                        let instruction = ({
                            let source_ml_source2_array_read_value19352 = match builder.clone() {
                                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value19351, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value19351.clone()
                                }
                            };
                            { let ml_reference_array443 = source_ml_source2_array_read_value19352.clone(); let ml_reference_position443 = position; let ml_reference_projection443 = self::ml_array::optional_at(&*ml_reference_array443.borrow(), self::ml_array::number_index(ml_reference_position443)); ml_reference_projection443 }
                        }).clone().unwrap_or_else(|| self::JavaScriptCallInstruction::Return(0f64));
                        if (match instruction.clone() {
                            self::JavaScriptCallInstruction::Jump(_) => {
                                false
                            }
                            _ => {
                                true
                            }
                        } && match instruction.clone() {
                            self::JavaScriptCallInstruction::CompleteJump(_, _) => {
                                false
                            }
                            _ => {
                                true
                            }
                        }) {
                            self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Return(Box::new(invalid.clone()))
                        } else {
                            {
                                let source_ml_source1_temporary_index60_2 = source_ml_source1_temporary_index60.add(&self::ml::Big::from_i128(1));
                                {
                                    (previous, source_ml_source1_temporary_index60) = (position, source_ml_source1_temporary_index60_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Result::MlSource1TemporaryFinishJavaScriptCallSourceControlScopeLoop61Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_switch_loop64(bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, depth: f64, current: self::JavaScriptCallSourceBuilder, bodies: std::rc::Rc<std::cell::RefCell<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>, source_ml_source1_temporary_values62: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>, source_ml_source1_temporary_index62: self::ml::Big) -> self::JavaScriptCallSourceBuilder {
    if (source_ml_source1_temporary_index62 < ({ let ml_reference_array444 = source_ml_source1_temporary_values62.clone(); let ml_reference_projection444 = self::ml::Big::from_u128(ml_reference_array444.borrow().len() as u128); ml_reference_projection444 })) {
        {
            let clause = { let ml_reference_array445 = source_ml_source1_temporary_values62.clone(); let ml_reference_position445 = source_ml_source1_temporary_index62.clone(); let ml_reference_projection445 = self::ml_array::at(&*ml_reference_array445.borrow(), ml_reference_position445.to_index()); ml_reference_projection445 };
            { let ml_cell446 = bodies.clone(); let ml_value446 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value447 = ({ let ml_cell_value = bodies.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value447 }, vec![{ let ml_reference_array448 = match current.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value19850, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value19850.clone()
                }
            }; let ml_reference_projection448 = ml_reference_array448.borrow().len() as f64; ml_reference_projection448 }]))); *ml_cell446.borrow_mut() = ml_value446; {
                let current_2 = match self::lower_java_script_call_source_expression(match clause.clone() {
                    self::JavaScriptCallSourceSwitchClause::SourceSwitchCase(_, source_ml_source2_data_property_value19851) => {
                        let source_ml_source2_data_property_value19851 = *source_ml_source2_data_property_value19851;
                        source_ml_source2_data_property_value19851.clone()
                    }
                    self::JavaScriptCallSourceSwitchClause::SourceSwitchDefault(source_ml_source2_data_property_value19852) => {
                        let source_ml_source2_data_property_value19852 = *source_ml_source2_data_property_value19852;
                        source_ml_source2_data_property_value19852.clone()
                    }
                }, bindings.clone(), current.clone(), (depth + 1f64)) {
                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19853, _) => {
                        let source_ml_source2_data_property_value19853 = *source_ml_source2_data_property_value19853;
                        source_ml_source2_data_property_value19853.clone()
                    }
                };
                {
                    let source_ml_source1_temporary_index62_2 = source_ml_source1_temporary_index62.add(&self::ml::Big::from_i128(1));
                    self::source_ml_source1_temporary_lower_java_script_call_source_switch_loop64(bindings.clone(), depth, current_2.clone(), bodies.clone(), source_ml_source1_temporary_values62.clone(), source_ml_source1_temporary_index62_2.clone())
                }
            } }
        }
    } else {
        current.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result {
    MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Done(Box<self::JavaScriptCallSourceBuilder>, f64),
    MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_switch_loop63(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, depth: f64, saved: self::JavaScriptCallSourceResult, current: self::JavaScriptCallSourceBuilder, branches: std::rc::Rc<std::cell::RefCell<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>, fallback: f64, index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result {
    if (index < ({
        let source_ml_source2_nullish_field_receiver19858 = (match source.clone() {
            self::JavaScriptCallSource::SourceConstant(_) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceNewTarget => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceReceiver => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceArgument(_) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceDerivedReceiver => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceErrorIntrinsic => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceConstruct(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceThrow(_) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceReturn(_) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceControlJump(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceSwitch(_, _, source_ml_source2_data_property_value19857, _) => {
                Some(source_ml_source2_data_property_value19857.clone())
            }
            self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceFinally(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceTry(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceBinding(_) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceLet(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceRecur(_) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceAllocate(_) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceObject(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceGet(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceDelete(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceHas(_, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
            self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
                None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
            }
        },);
        if match (source_ml_source2_nullish_field_receiver19858.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
            {
                let source_ml_source2_nullish_field_present19859 = ((source_ml_source2_nullish_field_receiver19858.clone()).0).clone().expect("a proven present value");
                { let ml_reference_array449 = source_ml_source2_nullish_field_present19859.clone(); let ml_reference_projection449 = ml_reference_array449.borrow().len() as f64; ml_reference_projection449 }
            }
        } else {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
        }
    })) {
        {
            let clause = {
                let source_ml_source2_nullish_field_receiver19861 = (match source.clone() {
                    self::JavaScriptCallSource::SourceConstant(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceNewTarget => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceReceiver => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceArgument(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceDerivedReceiver => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceErrorIntrinsic => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceConstruct(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceThrow(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceReturn(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceControlJump(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceSwitch(_, _, source_ml_source2_data_property_value19860, _) => {
                        Some(source_ml_source2_data_property_value19860.clone())
                    }
                    self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceFinally(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceTry(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceBinding(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceLet(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceIf(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceSameValue(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceRecur(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceAllocate(_) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceObject(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceSet(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceGet(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceDelete(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceHas(_, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceCall(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                    self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
                        None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceSwitchClause>>>>
                    }
                }, index);
                {
                    let source_ml_source2_computed_property_key19862 = (source_ml_source2_nullish_field_receiver19861.clone()).1;
                    if match (source_ml_source2_nullish_field_receiver19861.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present19863 = ((source_ml_source2_nullish_field_receiver19861.clone()).0).clone().expect("a proven present value");
                            { let ml_reference_array450 = source_ml_source2_nullish_field_present19863.clone(); let ml_reference_position450 = source_ml_source2_computed_property_key19862; let ml_reference_projection450 = self::ml_array::optional_at(&*ml_reference_array450.borrow(), self::ml_array::number_index(ml_reference_position450)); ml_reference_projection450 }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source2_nullish_field_receiver19861.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
                    }
                }
            };
            if match (clause.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![115, 119, 105, 116, 99, 104, 32, 99, 108, 97, 117, 115, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101])))))
            } else {
                match clause.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceSwitchClause::SourceSwitchCase(clause_value, _) => {
                        let clause_value = *clause_value;
                        {
                            let value = self::lower_java_script_call_source_expression(clause_value.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let comparison = self::allocate_java_script_call_source_register(match value.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19868, _) => {
                                        let source_ml_source2_data_property_value19868 = *source_ml_source2_data_property_value19868;
                                        source_ml_source2_data_property_value19868.clone()
                                    }
                                });
                                {
                                    let compared = self::append_java_script_call_source_instruction(match comparison.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19869, _) => {
                                            let source_ml_source2_data_property_value19869 = *source_ml_source2_data_property_value19869;
                                            source_ml_source2_data_property_value19869.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::StrictEqual(match comparison.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19870) => {
                                            source_ml_source2_data_property_value19870
                                        }
                                    }, match saved.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19871) => {
                                            source_ml_source2_data_property_value19871
                                        }
                                    }, match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19872) => {
                                            source_ml_source2_data_property_value19872
                                        }
                                    }));
                                    {
                                        let position = { let ml_reference_array451 = match compared.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value19873, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value19873.clone()
                                            }
                                        }; let ml_reference_projection451 = ml_reference_array451.borrow().len() as f64; ml_reference_projection451 };
                                        { let ml_cell452 = branches.clone(); let ml_value452 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value453 = ({ let ml_cell_value = branches.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value453 }, vec![position]))); *ml_cell452.borrow_mut() = ml_value452; {
                                            let current_2 = self::append_java_script_call_source_instruction(compared.clone(), self::JavaScriptCallInstruction::Branch(match comparison.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19874) => {
                                                    source_ml_source2_data_property_value19874
                                                }
                                            }, 0f64, (position + 1f64)));
                                            {
                                                let index_2 = (index + 1f64);
                                                self::source_ml_source1_temporary_lower_java_script_call_source_switch_loop63(source.clone(), bindings.clone(), depth, saved.clone(), current_2.clone(), branches.clone(), fallback, index_2)
                                            }
                                        } }
                                    }
                                }
                            }
                        }
                    }
                    self::JavaScriptCallSourceSwitchClause::SourceSwitchDefault(_) => {
                        if (fallback >= (0f64)) {
                            self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![115, 119, 105, 116, 99, 104, 32, 99, 97, 110, 110, 111, 116, 32, 99, 111, 110, 116, 97, 105, 110, 32, 109, 117, 108, 116, 105, 112, 108, 101, 32, 100, 101, 102, 97, 117, 108, 116, 32, 99, 108, 97, 117, 115, 101, 115])))))
                        } else {
                            {
                                let fallback_2 = index;
                                { let ml_cell454 = branches.clone(); let ml_value454 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value455 = ({ let ml_cell_value = branches.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value455 }, vec![(-1f64)]))); *ml_cell454.borrow_mut() = ml_value454; {
                                    let index_3 = (index + 1f64);
                                    self::source_ml_source1_temporary_lower_java_script_call_source_switch_loop63(source.clone(), bindings.clone(), depth, saved.clone(), current.clone(), branches.clone(), fallback_2, index_3)
                                } }
                            }
                        }
                    }
                }
            }
        }
    } else {
        self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Done(Box::new(current.clone()), fallback)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result {
    MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Done,
    MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_source_instruction_position_loop65(mut positions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut position: f64, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let value = { let ml_reference_array456 = positions.clone(); let ml_reference_position456 = middle; let ml_reference_projection456 = self::ml_array::optional_at(&*ml_reference_array456.borrow(), self::ml_array::number_index(ml_reference_position456)); ml_reference_projection456 };
                    if {
                        let source_ml_source2_comparison_operand19878 = None::<()>;
                        match (value.clone(), source_ml_source2_comparison_operand19878.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                    } {
                        self::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Return((-1f64))
                    } else {
                        if ((value.clone().expect("a proven present value")) == position) {
                            self::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Return(middle)
                        } else {
                            if ((value.clone().expect("a proven present value")) < position) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Result::MlSource1TemporaryFindJavaScriptCallSourceInstructionPositionLoop65Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result {
    MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Done(Box<self::JavaScriptCallSourceBuilder>),
    MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_binding_scope_loop73(mut bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut current: self::JavaScriptCallSourceBuilder, mut entries: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, mut source_ml_source1_temporary_values72: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallBindingDeclaration>>>, mut source_ml_source1_temporary_index72: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result {
    loop {
        return if (source_ml_source1_temporary_index72 < ({ let ml_reference_array457 = source_ml_source1_temporary_values72.clone(); let ml_reference_projection457 = self::ml::Big::from_u128(ml_reference_array457.borrow().len() as u128); ml_reference_projection457 })) {
            {
                let declaration = { let ml_reference_array458 = source_ml_source1_temporary_values72.clone(); let ml_reference_position458 = source_ml_source1_temporary_index72.clone(); let ml_reference_projection458 = self::ml_array::at(&*ml_reference_array458.borrow(), ml_reference_position458.to_index()); ml_reference_projection458 };
                if (((!(self::test_java_script_call_code_coordinate(match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value19881, _) => {
                        source_ml_source2_data_property_value19881
                    }
                }, 65536f64)) || { let ml_values459 = { let ml_reference_value460 = bindings.borrow().clone(); ml_reference_value460 }; let ml_callback459 = { let declaration = declaration.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value19884, _, _, _) => {
                        source_ml_source2_data_property_value19884
                    }
                }) == (match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value19885, _) => {
                        source_ml_source2_data_property_value19885
                    }
                })) }) }; ml_values459.iter().cloned().enumerate().any(|(ml_index459, ml_value459)| ml_callback459(ml_value459)) }) || { let ml_values461 = { let ml_reference_value462 = entries.borrow().clone(); ml_reference_value462 }; let ml_callback461 = { let declaration = declaration.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value19888, _, _, _) => {
                        source_ml_source2_data_property_value19888
                    }
                }) == (match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value19889, _) => {
                        source_ml_source2_data_property_value19889
                    }
                })) }) }; ml_values461.iter().cloned().enumerate().any(|(ml_index461, ml_value461)| ml_callback461(ml_value461)) }) || (({
                    let source_ml_source2_type_query_operand19893 = match declaration.clone() {
                        self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(_, source_ml_source2_data_property_value19892) => {
                            source_ml_source2_data_property_value19892
                        }
                    };
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Return(Box::new(self::refuse_java_script_call_source(current.clone(), &(self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 111, 117, 116, 112, 117, 116, 32, 115, 99, 111, 112, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 120, 105, 99, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115, 32, 97, 110, 100, 32, 109, 117, 116, 97, 98, 105, 108, 105, 116, 121])))))
                } else {
                    {
                        let register = self::allocate_java_script_call_source_register(current.clone());
                        {
                            let current_2 = match register.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value19894, _) => {
                                    let source_ml_source2_data_property_value19894 = *source_ml_source2_data_property_value19894;
                                    source_ml_source2_data_property_value19894.clone()
                                }
                            };
                            {
                                let source_ml_source1_temporary_lower_java_script_call_source_binding_scope_ignored74 = { let ml_method_owner463 = entries.clone(); let ml_method_argument463_0 = {
                                    let source_ml_source2_constructor_field19896 = match declaration.clone() {
                                        self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value19895, _) => {
                                            source_ml_source2_data_property_value19895
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field19898 = match register.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value19897) => {
                                                source_ml_source2_data_property_value19897
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field19900 = match declaration.clone() {
                                                self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(_, source_ml_source2_data_property_value19899) => {
                                                    source_ml_source2_data_property_value19899
                                                }
                                            };
                                            {
                                                let source_ml_source2_constructor_field19901 = Some(true);
                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_constructor_field19896, source_ml_source2_constructor_field19898, source_ml_source2_constructor_field19900, Some((source_ml_source2_constructor_field19901.clone(),)))
                                            }
                                        }
                                    }
                                }; let ml_inserted = vec![ml_method_argument463_0]; let mut ml_method_values463 = ml_method_owner463.borrow_mut(); let ml_length = ml_method_values463.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values463.extend(ml_inserted); ml_length as f64 };
                                {
                                    let source_ml_source1_temporary_index72_2 = source_ml_source1_temporary_index72.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, source_ml_source1_temporary_index72) = (current_2.clone(), source_ml_source1_temporary_index72_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Done(Box::new(current.clone()))
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptErrorPropertyReadLoop77Result {
    MlSource1TemporaryConstructJavaScriptErrorPropertyReadLoop77Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_error_property_read_loop77(mut root: std::rc::Rc<std::cell::RefCell<(self::ml_string::Text, Option<self::JavaScriptSemanticValue>)>>, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut at: f64) -> self::MlSource1TemporaryConstructJavaScriptErrorPropertyReadLoop77Result {
    loop {
        return if (at < ((({ let ml_reference_value464 = root.borrow().clone(); ml_reference_value464 }).0).length())) {
            {
                let source_ml_source1_temporary_construct_java_script_error_property_read_ignored78 = { let ml_method_owner465 = units.clone(); let ml_method_argument465_0 = {
                    let source_ml_source2_string_unit_operand19902 = ({ let ml_reference_value466 = root.borrow().clone(); ml_reference_value466 }).0;
                    {
                        let source_ml_source2_string_unit_operand19903 = at;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index < (0f64)) || (index >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand19909 = {
                                        let source_ml_source2_string_range_operand19904 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand19905 = index;
                                            {
                                                let source_ml_source2_string_range_operand19906 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position19907 = source_ml_source2_string_range_operand19905;
                                                    {
                                                        let source_ml_source2_string_range_position19908 = source_ml_source2_string_range_operand19906;
                                                        source_ml_source2_string_range_operand19904.slice(source_ml_source2_string_range_position19907, source_ml_source2_string_range_position19908)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand19910 = 0f64;
                                        source_ml_source2_string_unit_operand19909.code_point_at(source_ml_source2_string_unit_operand19910)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand19902.clone(), source_ml_source2_string_unit_operand19903)
                    }
                }; let ml_inserted = vec![ml_method_argument465_0]; let mut ml_method_values465 = ml_method_owner465.borrow_mut(); let ml_length = ml_method_values465.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values465.extend(ml_inserted); ml_length as f64 };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConstructJavaScriptErrorPropertyReadLoop77Result::MlSource1TemporaryConstructJavaScriptErrorPropertyReadLoop77Done
        };
    }
}
