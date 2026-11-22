// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallBindingDeclaration;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::JavaScriptCallSourceSwitchClause;
use super::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_switch_loop63(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, depth: f64, saved: self::JavaScriptCallSourceResult, current: self::JavaScriptCallSourceBuilder, branches: std::rc::Rc<std::cell::RefCell<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>, fallback: f64, index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result {
    if (index < ({
        let source_ml_source2_nullish_field_receiver16118 = (match source.clone() {
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
            self::JavaScriptCallSource::SourceSwitch(_, _, source_ml_source2_data_property_value16117, _) => {
                Some(source_ml_source2_data_property_value16117.clone())
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
        if match (source_ml_source2_nullish_field_receiver16118.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
            {
                let source_ml_source2_nullish_field_present16119 = ((source_ml_source2_nullish_field_receiver16118.clone()).0).clone().expect("a proven present value");
                { let ml_reference_array288 = source_ml_source2_nullish_field_present16119.clone(); let ml_reference_projection288 = ml_reference_array288.borrow().len() as f64; ml_reference_projection288 }
            }
        } else {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'length')")).encode_utf16().collect(), message_present: true }).raise()
        }
    })) {
        {
            let clause = {
                let source_ml_source2_nullish_field_receiver16121 = (match source.clone() {
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
                    self::JavaScriptCallSource::SourceSwitch(_, _, source_ml_source2_data_property_value16120, _) => {
                        Some(source_ml_source2_data_property_value16120.clone())
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
                    let source_ml_source2_computed_property_key16122 = (source_ml_source2_nullish_field_receiver16121.clone()).1;
                    if match (source_ml_source2_nullish_field_receiver16121.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present16123 = ((source_ml_source2_nullish_field_receiver16121.clone()).0).clone().expect("a proven present value");
                            { let ml_reference_array289 = source_ml_source2_nullish_field_present16123.clone(); let ml_reference_position289 = source_ml_source2_computed_property_key16122; let ml_reference_projection289 = self::ml_array::optional_at(&*ml_reference_array289.borrow(), self::ml_array::number_index(ml_reference_position289)); ml_reference_projection289 }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (format!("{}{}", (format!("{}{}", (String::from("Cannot read properties of undefined (reading '")), (self::ml_number::js_number((source_ml_source2_nullish_field_receiver16121.clone()).1)))), (String::from("')")))).encode_utf16().collect(), message_present: true }).raise()
                    }
                }
            };
            if match (clause.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Return(Box::new(self::refuse_java_script_call_source(current.clone(), "switch clauses must be dense")))
            } else {
                match clause.clone().expect("a proven present value") {
                    self::JavaScriptCallSourceSwitchClause::SourceSwitchCase(clause_value, _) => {
                        let clause_value = *clause_value;
                        {
                            let value = self::lower_java_script_call_source_expression(clause_value.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                            {
                                let comparison = self::allocate_java_script_call_source_register(match value.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value16128, _) => {
                                        let source_ml_source2_data_property_value16128 = *source_ml_source2_data_property_value16128;
                                        source_ml_source2_data_property_value16128.clone()
                                    }
                                });
                                {
                                    let compared = self::append_java_script_call_source_instruction(match comparison.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value16129, _) => {
                                            let source_ml_source2_data_property_value16129 = *source_ml_source2_data_property_value16129;
                                            source_ml_source2_data_property_value16129.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::StrictEqual(match comparison.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value16130) => {
                                            source_ml_source2_data_property_value16130
                                        }
                                    }, match saved.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value16131) => {
                                            source_ml_source2_data_property_value16131
                                        }
                                    }, match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value16132) => {
                                            source_ml_source2_data_property_value16132
                                        }
                                    }));
                                    {
                                        let position = { let ml_reference_array290 = match compared.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value16133, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value16133.clone()
                                            }
                                        }; let ml_reference_projection290 = ml_reference_array290.borrow().len() as f64; ml_reference_projection290 };
                                        { let ml_cell291 = branches.clone(); let ml_value291 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value292 = ({ let ml_cell_value = branches.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value292 }, vec![position]))); *ml_cell291.borrow_mut() = ml_value291; {
                                            let current_2 = self::append_java_script_call_source_instruction(compared.clone(), self::JavaScriptCallInstruction::Branch(match comparison.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value16134) => {
                                                    source_ml_source2_data_property_value16134
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
                            self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Return(Box::new(self::refuse_java_script_call_source(current.clone(), "switch cannot contain multiple default clauses")))
                        } else {
                            {
                                let fallback_2 = index;
                                { let ml_cell293 = branches.clone(); let ml_value293 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value294 = ({ let ml_cell_value = branches.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value294 }, vec![(-1f64)]))); *ml_cell293.borrow_mut() = ml_value293; {
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
                    let value = { let ml_reference_array295 = positions.clone(); let ml_reference_position295 = middle; let ml_reference_projection295 = self::ml_array::optional_at(&*ml_reference_array295.borrow(), self::ml_array::number_index(ml_reference_position295)); ml_reference_projection295 };
                    if {
                        let source_ml_source2_comparison_operand16138 = None::<()>;
                        match (value.clone(), source_ml_source2_comparison_operand16138.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
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
        return if (source_ml_source1_temporary_index72 < ({ let ml_reference_array296 = source_ml_source1_temporary_values72.clone(); let ml_reference_projection296 = self::ml::Big::from_u128(ml_reference_array296.borrow().len() as u128); ml_reference_projection296 })) {
            {
                let declaration = { let ml_reference_array297 = source_ml_source1_temporary_values72.clone(); let ml_reference_position297 = source_ml_source1_temporary_index72.clone(); let ml_reference_projection297 = self::ml_array::at(&*ml_reference_array297.borrow(), ml_reference_position297.to_index()); ml_reference_projection297 };
                if (((!(self::test_java_script_call_code_coordinate(match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value16141, _) => {
                        source_ml_source2_data_property_value16141
                    }
                }, 65536f64)) || { let ml_values298 = { let ml_reference_value299 = bindings.borrow().clone(); ml_reference_value299 }; let ml_callback298 = { let declaration = declaration.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value16144, _, _, _) => {
                        source_ml_source2_data_property_value16144
                    }
                }) == (match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value16145, _) => {
                        source_ml_source2_data_property_value16145
                    }
                })) }) }; ml_values298.iter().cloned().enumerate().any(|(ml_index298, ml_value298)| ml_callback298(ml_value298)) }) || { let ml_values300 = { let ml_reference_value301 = entries.borrow().clone(); ml_reference_value301 }; let ml_callback300 = { let declaration = declaration.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value16148, _, _, _) => {
                        source_ml_source2_data_property_value16148
                    }
                }) == (match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value16149, _) => {
                        source_ml_source2_data_property_value16149
                    }
                })) }) }; ml_values300.iter().cloned().enumerate().any(|(ml_index300, ml_value300)| ml_callback300(ml_value300)) }) || (({
                    let source_ml_source2_type_query_operand16153 = match declaration.clone() {
                        self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(_, source_ml_source2_data_property_value16152) => {
                            source_ml_source2_data_property_value16152
                        }
                    };
                    String::from("boolean")
                }) != "boolean")) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Result::MlSource1TemporaryLowerJavaScriptCallSourceBindingScopeLoop73Return(Box::new(self::refuse_java_script_call_source(current.clone(), "binding output scopes retain distinct actual lexical identities and mutability")))
                } else {
                    {
                        let register = self::allocate_java_script_call_source_register(current.clone());
                        {
                            let current_2 = match register.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value16154, _) => {
                                    let source_ml_source2_data_property_value16154 = *source_ml_source2_data_property_value16154;
                                    source_ml_source2_data_property_value16154.clone()
                                }
                            };
                            {
                                let source_ml_source1_temporary_lower_java_script_call_source_binding_scope_ignored74 = { let ml_method_owner302 = entries.clone(); let ml_method_argument302_0 = {
                                    let source_ml_source2_constructor_field16156 = match declaration.clone() {
                                        self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value16155, _) => {
                                            source_ml_source2_data_property_value16155
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field16158 = match register.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value16157) => {
                                                source_ml_source2_data_property_value16157
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field16160 = match declaration.clone() {
                                                self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(_, source_ml_source2_data_property_value16159) => {
                                                    source_ml_source2_data_property_value16159
                                                }
                                            };
                                            {
                                                let source_ml_source2_constructor_field16161 = Some(true);
                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_constructor_field16156, source_ml_source2_constructor_field16158, source_ml_source2_constructor_field16160, Some((source_ml_source2_constructor_field16161.clone(),)))
                                            }
                                        }
                                    }
                                }; let ml_inserted = vec![ml_method_argument302_0]; let mut ml_method_values302 = ml_method_owner302.borrow_mut(); let ml_length = ml_method_values302.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values302.extend(ml_inserted); ml_length as f64 };
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
