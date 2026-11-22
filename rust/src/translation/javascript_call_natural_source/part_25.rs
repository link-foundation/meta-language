// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
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
use super::ml_string;
use super::refuse_java_script_call_source;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_switch_loop63(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, depth: f64, saved: self::JavaScriptCallSourceResult, current: self::JavaScriptCallSourceBuilder, branches: std::rc::Rc<std::cell::RefCell<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>>, fallback: f64, index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceSwitchLoop63Result {
    if (index < ({
        let source_ml_source2_nullish_field_receiver16036 = (match source.clone() {
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
            self::JavaScriptCallSource::SourceSwitch(_, _, source_ml_source2_data_property_value16035, _) => {
                Some(source_ml_source2_data_property_value16035.clone())
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
        if match (source_ml_source2_nullish_field_receiver16036.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
            {
                let source_ml_source2_nullish_field_present16037 = ((source_ml_source2_nullish_field_receiver16036.clone()).0).clone().expect("a proven present value");
                { let ml_reference_array295 = source_ml_source2_nullish_field_present16037.clone(); let ml_reference_projection295 = ml_reference_array295.borrow().len() as f64; ml_reference_projection295 }
            }
        } else {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
        }
    })) {
        {
            let clause = {
                let source_ml_source2_nullish_field_receiver16039 = (match source.clone() {
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
                    self::JavaScriptCallSource::SourceSwitch(_, _, source_ml_source2_data_property_value16038, _) => {
                        Some(source_ml_source2_data_property_value16038.clone())
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
                    let source_ml_source2_computed_property_key16040 = (source_ml_source2_nullish_field_receiver16039.clone()).1;
                    if match (source_ml_source2_nullish_field_receiver16039.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present16041 = ((source_ml_source2_nullish_field_receiver16039.clone()).0).clone().expect("a proven present value");
                            { let ml_reference_array296 = source_ml_source2_nullish_field_present16041.clone(); let ml_reference_position296 = source_ml_source2_computed_property_key16040; let ml_reference_projection296 = self::ml_array::optional_at(&*ml_reference_array296.borrow(), self::ml_array::number_index(ml_reference_position296)); ml_reference_projection296 }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source2_nullish_field_receiver16039.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
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
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value16046, _) => {
                                        let source_ml_source2_data_property_value16046 = *source_ml_source2_data_property_value16046;
                                        source_ml_source2_data_property_value16046.clone()
                                    }
                                });
                                {
                                    let compared = self::append_java_script_call_source_instruction(match comparison.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value16047, _) => {
                                            let source_ml_source2_data_property_value16047 = *source_ml_source2_data_property_value16047;
                                            source_ml_source2_data_property_value16047.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::StrictEqual(match comparison.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value16048) => {
                                            source_ml_source2_data_property_value16048
                                        }
                                    }, match saved.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value16049) => {
                                            source_ml_source2_data_property_value16049
                                        }
                                    }, match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value16050) => {
                                            source_ml_source2_data_property_value16050
                                        }
                                    }));
                                    {
                                        let position = { let ml_reference_array297 = match compared.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source2_data_property_value16051, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value16051.clone()
                                            }
                                        }; let ml_reference_projection297 = ml_reference_array297.borrow().len() as f64; ml_reference_projection297 };
                                        { let ml_cell298 = branches.clone(); let ml_value298 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value299 = ({ let ml_cell_value = branches.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value299 }, vec![position]))); *ml_cell298.borrow_mut() = ml_value298; {
                                            let current_2 = self::append_java_script_call_source_instruction(compared.clone(), self::JavaScriptCallInstruction::Branch(match comparison.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value16052) => {
                                                    source_ml_source2_data_property_value16052
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
                                { let ml_cell300 = branches.clone(); let ml_value300 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value301 = ({ let ml_cell_value = branches.borrow().clone(); ml_cell_value }).borrow().clone(); ml_reference_value301 }, vec![(-1f64)]))); *ml_cell300.borrow_mut() = ml_value300; {
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
                    let value = { let ml_reference_array302 = positions.clone(); let ml_reference_position302 = middle; let ml_reference_projection302 = self::ml_array::optional_at(&*ml_reference_array302.borrow(), self::ml_array::number_index(ml_reference_position302)); ml_reference_projection302 };
                    if {
                        let source_ml_source2_comparison_operand16056 = None::<()>;
                        match (value.clone(), source_ml_source2_comparison_operand16056.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
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
        return if (source_ml_source1_temporary_index72 < ({ let ml_reference_array303 = source_ml_source1_temporary_values72.clone(); let ml_reference_projection303 = self::ml::Big::from_u128(ml_reference_array303.borrow().len() as u128); ml_reference_projection303 })) {
            {
                let declaration = { let ml_reference_array304 = source_ml_source1_temporary_values72.clone(); let ml_reference_position304 = source_ml_source1_temporary_index72.clone(); let ml_reference_projection304 = self::ml_array::at(&*ml_reference_array304.borrow(), ml_reference_position304.to_index()); ml_reference_projection304 };
                if (((!(self::test_java_script_call_code_coordinate(match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value16059, _) => {
                        source_ml_source2_data_property_value16059
                    }
                }, 65536f64)) || { let ml_values305 = { let ml_reference_value306 = bindings.borrow().clone(); ml_reference_value306 }; let ml_callback305 = { let declaration = declaration.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value16062, _, _, _) => {
                        source_ml_source2_data_property_value16062
                    }
                }) == (match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value16063, _) => {
                        source_ml_source2_data_property_value16063
                    }
                })) }) }; ml_values305.iter().cloned().enumerate().any(|(ml_index305, ml_value305)| ml_callback305(ml_value305)) }) || { let ml_values307 = { let ml_reference_value308 = entries.borrow().clone(); ml_reference_value308 }; let ml_callback307 = { let declaration = declaration.clone(); std::rc::Rc::new(move |row: self::JavaScriptCallSourceBinding| -> bool { ((match row.clone() {
                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value16066, _, _, _) => {
                        source_ml_source2_data_property_value16066
                    }
                }) == (match declaration.clone() {
                    self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value16067, _) => {
                        source_ml_source2_data_property_value16067
                    }
                })) }) }; ml_values307.iter().cloned().enumerate().any(|(ml_index307, ml_value307)| ml_callback307(ml_value307)) }) || (({
                    let source_ml_source2_type_query_operand16071 = match declaration.clone() {
                        self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(_, source_ml_source2_data_property_value16070) => {
                            source_ml_source2_data_property_value16070
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
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value16072, _) => {
                                    let source_ml_source2_data_property_value16072 = *source_ml_source2_data_property_value16072;
                                    source_ml_source2_data_property_value16072.clone()
                                }
                            };
                            {
                                let source_ml_source1_temporary_lower_java_script_call_source_binding_scope_ignored74 = { let ml_method_owner309 = entries.clone(); let ml_method_argument309_0 = {
                                    let source_ml_source2_constructor_field16074 = match declaration.clone() {
                                        self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(source_ml_source2_data_property_value16073, _) => {
                                            source_ml_source2_data_property_value16073
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field16076 = match register.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value16075) => {
                                                source_ml_source2_data_property_value16075
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field16078 = match declaration.clone() {
                                                self::JavaScriptCallBindingDeclaration::SourceBindingDeclaration(_, source_ml_source2_data_property_value16077) => {
                                                    source_ml_source2_data_property_value16077
                                                }
                                            };
                                            {
                                                let source_ml_source2_constructor_field16079 = Some(true);
                                                self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_constructor_field16074, source_ml_source2_constructor_field16076, source_ml_source2_constructor_field16078, Some((source_ml_source2_constructor_field16079.clone(),)))
                                            }
                                        }
                                    }
                                }; let ml_inserted = vec![ml_method_argument309_0]; let mut ml_method_values309 = ml_method_owner309.borrow_mut(); let ml_length = ml_method_values309.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values309.extend(ml_inserted); ml_length as f64 };
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
pub enum MlSource1TemporaryConstructJavaScriptLiteralErrorSourceLoop75Result {
    MlSource1TemporaryConstructJavaScriptLiteralErrorSourceLoop75Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_literal_error_source_loop75(message: &self::ml_string::Text, mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut index: f64) -> self::MlSource1TemporaryConstructJavaScriptLiteralErrorSourceLoop75Result {
    loop {
        return if (index < (message.length())) {
            {
                let source_ml_source1_temporary_construct_java_script_literal_error_source_ignored76 = { let ml_method_owner310 = units.clone(); let ml_method_argument310_0 = {
                    let source_ml_source2_string_unit_operand16080 = message.clone();
                    {
                        let source_ml_source2_string_unit_operand16081 = index;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand16087 = {
                                        let source_ml_source2_string_range_operand16082 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand16083 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand16084 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position16085 = source_ml_source2_string_range_operand16083;
                                                    {
                                                        let source_ml_source2_string_range_position16086 = source_ml_source2_string_range_operand16084;
                                                        source_ml_source2_string_range_operand16082.slice(source_ml_source2_string_range_position16085, source_ml_source2_string_range_position16086)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand16088 = 0f64;
                                        source_ml_source2_string_unit_operand16087.code_point_at(source_ml_source2_string_unit_operand16088)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand16080.clone(), source_ml_source2_string_unit_operand16081)
                    }
                }; let ml_inserted = vec![ml_method_argument310_0]; let mut ml_method_values310 = ml_method_owner310.borrow_mut(); let ml_length = ml_method_values310.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values310.extend(ml_inserted); ml_length as f64 };
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConstructJavaScriptLiteralErrorSourceLoop75Result::MlSource1TemporaryConstructJavaScriptLiteralErrorSourceLoop75Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop78Result {
    MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop78Done,
    MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop78Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_natural_source_realm_loop78(first: &self::ml_string::Text, mut source_ml_source1_temporary_values77: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut source_ml_source1_temporary_index77: self::ml::Big) -> self::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop78Result {
    loop {
        return if (source_ml_source1_temporary_index77 < ({ let ml_reference_array311 = source_ml_source1_temporary_values77.clone(); let ml_reference_projection311 = self::ml::Big::from_u128(ml_reference_array311.borrow().len() as u128); ml_reference_projection311 })) {
            {
                let realm = { let ml_reference_array312 = source_ml_source1_temporary_values77.clone(); let ml_reference_position312 = source_ml_source1_temporary_index77.clone(); let ml_reference_projection312 = self::ml_array::at(&*ml_reference_array312.borrow(), ml_reference_position312.to_index()); ml_reference_projection312 };
                if (realm != (*first)) {
                    self::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop78Result::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop78Return(false)
                } else {
                    {
                        let source_ml_source1_temporary_index77_2 = source_ml_source1_temporary_index77.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index77 = source_ml_source1_temporary_index77_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop78Result::MlSource1TemporaryTestJavaScriptNaturalSourceRealmLoop78Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop79Result {
    MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop79Done,
    MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop79Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Result {
    MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Done(f64),
    MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_natural_guard_order_loop80(mut orders: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut order: f64, mut found: f64, mut index: f64) -> self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Result {
    loop {
        return if (index < ({ let ml_reference_array313 = orders.clone(); let ml_reference_projection313 = ml_reference_array313.borrow().len() as f64; ml_reference_projection313 })) {
            if ((({ let ml_reference_array314 = orders.clone(); let ml_reference_position314 = index; let ml_reference_projection314 = self::ml_array::optional_at(&*ml_reference_array314.borrow(), self::ml_array::number_index(ml_reference_position314)); ml_reference_projection314 }).clone().unwrap_or_else(|| (-1f64))) == order) {
                if (found != ((-1f64))) {
                    self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Return(std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))
                } else {
                    {
                        let found_2 = index;
                        {
                            let index_2 = (index + 1f64);
                            {
                                (found, index) = (found_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            } else {
                {
                    let index_3 = (index + 1f64);
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Done(found)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_natural_guard_order_loop79(mut types: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut orders: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut positions: std::rc::Rc<std::cell::RefCell<Vec<f64>>>, mut order: f64) -> self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop79Result {
    loop {
        return if (order < ({ let ml_reference_array315 = orders.clone(); let ml_reference_projection315 = ml_reference_array315.borrow().len() as f64; ml_reference_projection315 })) {
            {
                let found = (-1f64);
                {
                    let index = 0f64;
                    {
                        let source_ml_source2_s16095 = self::source_ml_source1_temporary_plan_java_script_natural_guard_order_loop80(orders.clone(), order, found, index);
                        match source_ml_source2_s16095.clone() {
                            self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Done(found_2) => {
                                if ((found_2 < (0f64)) || ((({ let ml_reference_array316 = types.clone(); let ml_reference_position316 = found_2; let ml_reference_projection316 = self::ml_array::optional_at(&*ml_reference_array316.borrow(), self::ml_array::number_index(ml_reference_position316)); ml_reference_projection316 }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]))) != (self::ml_string::Text::from_units(vec![110, 97, 116])))) {
                                    self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop79Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop79Return(std::rc::Rc::new(std::cell::RefCell::new(vec![(-1f64)])))
                                } else {
                                    {
                                        let source_ml_source1_temporary_plan_java_script_natural_guard_order_ignored81 = { let ml_method_owner317 = positions.clone(); let ml_method_argument317_0 = found_2; let ml_inserted = vec![ml_method_argument317_0]; let mut ml_method_values317 = ml_method_owner317.borrow_mut(); let ml_length = ml_method_values317.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values317.extend(ml_inserted); ml_length as f64 };
                                        {
                                            let order_2 = (order + 1f64);
                                            {
                                                order = order_2;
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                            self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop80Return(source_ml_source1_temporary_result81) => {
                                self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop79Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop79Return(source_ml_source1_temporary_result81.clone())
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop79Result::MlSource1TemporaryPlanJavaScriptNaturalGuardOrderLoop79Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Result {
    MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Done(f64, f64, f64),
    MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Return(std::rc::Rc<std::cell::RefCell<Vec<f64>>>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_natural_pattern_branches_loop82(mut patterns: std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>, mut zero: f64, mut succ: f64, mut fallback: f64, mut index: f64) -> self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Result {
    loop {
        return if (index < ({ let ml_reference_array318 = patterns.clone(); let ml_reference_projection318 = ml_reference_array318.borrow().len() as f64; ml_reference_projection318 })) {
            {
                let pattern = ({ let ml_reference_array319 = patterns.clone(); let ml_reference_position319 = index; let ml_reference_projection319 = self::ml_array::optional_at(&*ml_reference_array319.borrow(), self::ml_array::number_index(ml_reference_position319)); ml_reference_projection319 }).clone().unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if (pattern == (self::ml_string::Text::from_units(vec![110, 97, 116, 90, 101, 114, 111]))) {
                    if (zero != ((-1f64))) {
                        self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                    } else {
                        {
                            let zero_2 = index;
                            {
                                let index_2 = (index + 1f64);
                                {
                                    (zero, index) = (zero_2, index_2);
                                    continue;
                                }
                            }
                        }
                    }
                } else {
                    if (pattern == (self::ml_string::Text::from_units(vec![110, 97, 116, 83, 117, 99, 99]))) {
                        if (succ != ((-1f64))) {
                            self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                        } else {
                            {
                                let succ_2 = index;
                                {
                                    let index_3 = (index + 1f64);
                                    {
                                        (succ, index) = (succ_2, index_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    } else {
                        if ((pattern == (self::ml_string::Text::from_units(vec![119, 105, 108, 100]))) || (pattern == (self::ml_string::Text::from_units(vec![98, 105, 110, 100])))) {
                            if (fallback != ((-1f64))) {
                                self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                            } else {
                                {
                                    let fallback_2 = index;
                                    {
                                        let index_4 = (index + 1f64);
                                        {
                                            (fallback, index) = (fallback_2, index_4);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Return(std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new())))
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Result::MlSource1TemporaryPlanJavaScriptNaturalPatternBranchesLoop82Done(zero, succ, fallback)
        };
    }
}
