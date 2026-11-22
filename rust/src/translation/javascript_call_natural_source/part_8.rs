// Generated from complete js/src/translation/javascript-call-natural-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceResult;
use super::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Result;
use super::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Result;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::lower_java_script_call_source_for_of;
use super::lower_java_script_call_source_iterator_loop;
use super::lower_java_script_call_source_iterator_remainder;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_primitive_arity_error;
use super::refuse_java_script_call_source;
use super::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop28;
use super::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop29;
use super::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop30;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_iterator_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::lower_java_script_call_source_for_of(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceIteratorStep(slot, realm, read_value) => {
            if ((!(self::test_java_script_call_code_coordinate(slot, 65536f64)) || (realm <= (self::ml::Big::from_i128(0)))) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 115, 116, 101, 112, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 103, 105, 111, 110, 44, 32, 114, 101, 97, 108, 109, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101, 32, 99, 104, 111, 105, 99, 101])))
            } else {
                {
                    let value = self::allocate_java_script_call_source_register(state.clone());
                    {
                        let done = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5050, _) => {
                                let source_ml_source2_data_property_value5050 = *source_ml_source2_data_property_value5050;
                                source_ml_source2_data_property_value5050.clone()
                            }
                        });
                        {
                            let source_ml_source2_constructor_field5052 = match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5051) => {
                                    source_ml_source2_data_property_value5051
                                }
                            };
                            {
                                let source_ml_source2_constructor_field5071 = self::append_java_script_call_source_instruction(match done.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5053, _) => {
                                        let source_ml_source2_data_property_value5053 = *source_ml_source2_data_property_value5053;
                                        source_ml_source2_data_property_value5053.clone()
                                    }
                                }, {
                                    let source_ml_source1_temporary_constructor_field19 = match value.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5054) => {
                                            source_ml_source2_data_property_value5054
                                        }
                                    };
                                    {
                                        let source_ml_source1_temporary_constructor_field20 = match done.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5055) => {
                                                source_ml_source2_data_property_value5055
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_constructor_field21 = 0f64;
                                            {
                                                let source_ml_source1_temporary_constructor_field22 = 0f64;
                                                if read_value {
                                                    self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source1_temporary_constructor_field19, source_ml_source1_temporary_constructor_field20, source_ml_source1_temporary_constructor_field21, source_ml_source1_temporary_constructor_field22, slot, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), None::<(Option<bool>,)>)
                                                } else {
                                                    {
                                                        let source_ml_source2_constructor_field5070 = Some(false);
                                                        self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source1_temporary_constructor_field19, source_ml_source1_temporary_constructor_field20, source_ml_source1_temporary_constructor_field21, source_ml_source1_temporary_constructor_field22, slot, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), realm.clone(), Some((source_ml_source2_constructor_field5070.clone(),)))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                });
                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field5071.clone()), source_ml_source2_constructor_field5052)
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::lower_java_script_call_source_iterator_remainder(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::lower_java_script_call_source_for_of(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::lower_java_script_call_source_iterator_loop(source.clone(), bindings.clone(), state.clone(), depth)
        }
        self::JavaScriptCallSource::SourceIteratorClose(slot_2, realm_2) => {
            if ((!(self::test_java_script_call_code_coordinate(slot_2, 65536f64)) || (realm_2 <= (self::ml::Big::from_i128(0)))) || (realm_2 > (self::ml::Big::from_i128(18446744073709551615)))) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 108, 111, 115, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 115, 108, 111, 116, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109])))
            } else {
                {
                    let value_2 = self::allocate_java_script_call_source_register(state.clone());
                    {
                        let done_2 = self::allocate_java_script_call_source_register(match value_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5072, _) => {
                                let source_ml_source2_data_property_value5072 = *source_ml_source2_data_property_value5072;
                                source_ml_source2_data_property_value5072.clone()
                            }
                        });
                        {
                            let source_ml_source2_constructor_field5074 = match value_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5073) => {
                                    source_ml_source2_data_property_value5073
                                }
                            };
                            {
                                let source_ml_source2_constructor_field5085 = self::append_java_script_call_source_instruction(match done_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5075, _) => {
                                        let source_ml_source2_data_property_value5075 = *source_ml_source2_data_property_value5075;
                                        source_ml_source2_data_property_value5075.clone()
                                    }
                                }, {
                                    let source_ml_source2_constructor_field5077 = match value_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5076) => {
                                            source_ml_source2_data_property_value5076
                                        }
                                    };
                                    {
                                        let source_ml_source2_constructor_field5079 = match done_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5078) => {
                                                source_ml_source2_data_property_value5078
                                            }
                                        };
                                        {
                                            let source_ml_source2_constructor_field5080 = 0f64;
                                            {
                                                let source_ml_source2_constructor_field5081 = 0f64;
                                                self::JavaScriptCallInstruction::IteratorProtocol(source_ml_source2_constructor_field5077, source_ml_source2_constructor_field5079, source_ml_source2_constructor_field5080, source_ml_source2_constructor_field5081, slot_2, self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]), realm_2.clone(), None::<(Option<bool>,)>)
                                            }
                                        }
                                    }
                                });
                                self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_constructor_field5085.clone()), source_ml_source2_constructor_field5074)
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSameValue(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePrimitive(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceRecur(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn lower_java_script_call_source_comparison_expression(source: self::JavaScriptCallSource, bindings: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallSourceBinding>>>, state: self::JavaScriptCallSourceBuilder, depth: f64) -> self::JavaScriptCallSourceResult {
    match source.clone() {
        self::JavaScriptCallSource::SourceConstant(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNewTarget => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgument(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDerivedReceiver => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceErrorIntrinsic => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstruct(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorClass(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceClassConstructorCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructCollection(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceVariadicCall(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentSpread(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceConstructErrorValue(_, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceThrow(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceReturn(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceAssignBinding(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceImmutableBindingWrite(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWhile(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCloneBindingCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlJump(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceControlBlock(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSwitch(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBindingScope(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLexicalScope(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceWriteExisting(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeCell(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInitializeBinding(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorBinding(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorStep(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorRemainder(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceForOf(_, _, _, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorLoop(_, _, _, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIteratorClose(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceFinally(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceTry(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceBinding(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceLocalLet(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArgumentRest(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterRest(_, _, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceParameterDefault(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceIf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceNumberBinary(source_ml_source2_f5211, source_left, source_right) => {
            let source_left = *source_left;
            let source_right = *source_right;
            {
                let left = self::lower_java_script_call_source_expression(source_left.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let left_2 = left.clone();
                    {
                        let right = self::lower_java_script_call_source_expression(source_right.clone(), bindings.clone(), match left_2.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5443, _) => {
                                let source_ml_source2_data_property_value5443 = *source_ml_source2_data_property_value5443;
                                source_ml_source2_data_property_value5443.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let result = self::allocate_java_script_call_source_register(match right.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5444, _) => {
                                    let source_ml_source2_data_property_value5444 = *source_ml_source2_data_property_value5444;
                                    source_ml_source2_data_property_value5444.clone()
                                }
                            });
                            if !(self::ml_array::includes(&{ let ml_reference_value70 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![97, 100, 100]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]), self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])]))).borrow().clone(); ml_reference_value70 }, &source_ml_source2_f5211.clone(), 0f64, false, |a, b| a == b)) {
                                self::refuse_java_script_call_source(match result.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5628, _) => {
                                        let source_ml_source2_data_property_value5628 = *source_ml_source2_data_property_value5628;
                                        source_ml_source2_data_property_value5628.clone()
                                    }
                                }, &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 78, 117, 109, 98, 101, 114, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 111, 114])))
                            } else {
                                {
                                    let source_ml_source2_nominal_field5634 = match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5635, _) => {
                                            let source_ml_source2_data_property_value5635 = *source_ml_source2_data_property_value5635;
                                            source_ml_source2_data_property_value5635.clone()
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_field5636 = match result.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5637) => {
                                                source_ml_source2_data_property_value5637
                                            }
                                        };
                                        {
                                            let source_ml_source2_nominal_override5638 = self::append_java_script_call_source_instruction(match result.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5629, _) => {
                                                    let source_ml_source2_data_property_value5629 = *source_ml_source2_data_property_value5629;
                                                    source_ml_source2_data_property_value5629.clone()
                                                }
                                            }, self::JavaScriptCallInstruction::NumberBinary(match result.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5630) => {
                                                    source_ml_source2_data_property_value5630
                                                }
                                            }, match left_2.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5631) => {
                                                    source_ml_source2_data_property_value5631
                                                }
                                            }, match right.clone() {
                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value5632) => {
                                                    source_ml_source2_data_property_value5632
                                                }
                                            }, source_ml_source2_f5211.clone()));
                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override5638.clone()), source_ml_source2_nominal_field5636)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceSameValue(source_left_2, source_right_2) => {
            let source_left_2 = *source_left_2;
            let source_right_2 = *source_right_2;
            {
                let left_3 = self::lower_java_script_call_source_expression(source_left_2.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let left_4 = left_3.clone();
                    {
                        let right_2 = self::lower_java_script_call_source_expression(source_right_2.clone(), bindings.clone(), match left_4.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5823, _) => {
                                let source_ml_source2_data_property_value5823 = *source_ml_source2_data_property_value5823;
                                source_ml_source2_data_property_value5823.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let result_2 = self::allocate_java_script_call_source_register(match right_2.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value5824, _) => {
                                    let source_ml_source2_data_property_value5824 = *source_ml_source2_data_property_value5824;
                                    source_ml_source2_data_property_value5824.clone()
                                }
                            });
                            {
                                let source_ml_source2_nominal_field6391 = match result_2.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value6392, _) => {
                                        let source_ml_source2_data_property_value6392 = *source_ml_source2_data_property_value6392;
                                        source_ml_source2_data_property_value6392.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field6393 = match result_2.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value6394) => {
                                            source_ml_source2_data_property_value6394
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override6395 = self::append_java_script_call_source_instruction(match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value6386, _) => {
                                                let source_ml_source2_data_property_value6386 = *source_ml_source2_data_property_value6386;
                                                source_ml_source2_data_property_value6386.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::SameValue(match result_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value6387) => {
                                                source_ml_source2_data_property_value6387
                                            }
                                        }, match left_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value6388) => {
                                                source_ml_source2_data_property_value6388
                                            }
                                        }, match right_2.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value6389) => {
                                                source_ml_source2_data_property_value6389
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override6395.clone()), source_ml_source2_nominal_field6393)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceStrictEqual(source_left_3, source_right_3) => {
            let source_left_3 = *source_left_3;
            let source_right_3 = *source_right_3;
            {
                let left_5 = self::lower_java_script_call_source_expression(source_left_3.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let left_6 = left_5.clone();
                    {
                        let right_3 = self::lower_java_script_call_source_expression(source_right_3.clone(), bindings.clone(), match left_6.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value6580, _) => {
                                let source_ml_source2_data_property_value6580 = *source_ml_source2_data_property_value6580;
                                source_ml_source2_data_property_value6580.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let result_3 = self::allocate_java_script_call_source_register(match right_3.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value6581, _) => {
                                    let source_ml_source2_data_property_value6581 = *source_ml_source2_data_property_value6581;
                                    source_ml_source2_data_property_value6581.clone()
                                }
                            });
                            {
                                let source_ml_source2_nominal_field7138 = match result_3.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7139, _) => {
                                        let source_ml_source2_data_property_value7139 = *source_ml_source2_data_property_value7139;
                                        source_ml_source2_data_property_value7139.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field7140 = match result_3.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7141) => {
                                            source_ml_source2_data_property_value7141
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override7142 = self::append_java_script_call_source_instruction(match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7133, _) => {
                                                let source_ml_source2_data_property_value7133 = *source_ml_source2_data_property_value7133;
                                                source_ml_source2_data_property_value7133.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::StrictEqual(match result_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7134) => {
                                                source_ml_source2_data_property_value7134
                                            }
                                        }, match left_6.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7135) => {
                                                source_ml_source2_data_property_value7135
                                            }
                                        }, match right_3.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7136) => {
                                                source_ml_source2_data_property_value7136
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override7142.clone()), source_ml_source2_nominal_field7140)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourceSnapshotStrictEqual(source_left_4, source_right_4) => {
            let source_left_4 = *source_left_4;
            let source_right_4 = *source_right_4;
            {
                let left_7 = self::lower_java_script_call_source_expression(source_left_4.clone(), bindings.clone(), state.clone(), (depth + 1f64));
                {
                    let left_9 = {
                        let saved = self::allocate_java_script_call_source_register(match left_7.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7337, _) => {
                                let source_ml_source2_data_property_value7337 = *source_ml_source2_data_property_value7337;
                                source_ml_source2_data_property_value7337.clone()
                            }
                        });
                        {
                            let left_8 = {
                                let source_ml_source2_nominal_field7342 = match saved.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7343, _) => {
                                        let source_ml_source2_data_property_value7343 = *source_ml_source2_data_property_value7343;
                                        source_ml_source2_data_property_value7343.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field7344 = match saved.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7345) => {
                                            source_ml_source2_data_property_value7345
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override7346 = self::append_java_script_call_source_instruction(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7338, _) => {
                                                let source_ml_source2_data_property_value7338 = *source_ml_source2_data_property_value7338;
                                                source_ml_source2_data_property_value7338.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::Copy(match saved.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7339) => {
                                                source_ml_source2_data_property_value7339
                                            }
                                        }, match left_7.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7340) => {
                                                source_ml_source2_data_property_value7340
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override7346.clone()), source_ml_source2_nominal_field7344)
                                    }
                                }
                            };
                            left_8.clone()
                        }
                    };
                    {
                        let right_4 = self::lower_java_script_call_source_expression(source_right_4.clone(), bindings.clone(), match left_9.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7347, _) => {
                                let source_ml_source2_data_property_value7347 = *source_ml_source2_data_property_value7347;
                                source_ml_source2_data_property_value7347.clone()
                            }
                        }, (depth + 1f64));
                        {
                            let result_4 = self::allocate_java_script_call_source_register(match right_4.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7348, _) => {
                                    let source_ml_source2_data_property_value7348 = *source_ml_source2_data_property_value7348;
                                    source_ml_source2_data_property_value7348.clone()
                                }
                            });
                            {
                                let source_ml_source2_nominal_field7905 = match result_4.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7906, _) => {
                                        let source_ml_source2_data_property_value7906 = *source_ml_source2_data_property_value7906;
                                        source_ml_source2_data_property_value7906.clone()
                                    }
                                };
                                {
                                    let source_ml_source2_nominal_field7907 = match result_4.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7908) => {
                                            source_ml_source2_data_property_value7908
                                        }
                                    };
                                    {
                                        let source_ml_source2_nominal_override7909 = self::append_java_script_call_source_instruction(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7900, _) => {
                                                let source_ml_source2_data_property_value7900 = *source_ml_source2_data_property_value7900;
                                                source_ml_source2_data_property_value7900.clone()
                                            }
                                        }, self::JavaScriptCallInstruction::StrictEqual(match result_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7901) => {
                                                source_ml_source2_data_property_value7901
                                            }
                                        }, match left_9.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7902) => {
                                                source_ml_source2_data_property_value7902
                                            }
                                        }, match right_4.clone() {
                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7903) => {
                                                source_ml_source2_data_property_value7903
                                            }
                                        }));
                                        self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override7909.clone()), source_ml_source2_nominal_field7907)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptCallSource::SourcePrimitive(operation, arguments) => {
            {
                let error = self::read_java_script_call_primitive_arity_error(&(operation), { let ml_reference_array71 = arguments.clone(); let ml_reference_projection71 = ml_reference_array71.borrow().len() as f64; ml_reference_projection71 });
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::refuse_java_script_call_source(state.clone(), &(error))
                } else {
                    {
                        let current = state.clone();
                        {
                            let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            {
                                let source_ml_source1_temporary_index26 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s7920 = self::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop28(bindings.clone(), depth, current.clone(), arguments_.clone(), arguments.clone(), source_ml_source1_temporary_index26.clone());
                                    match source_ml_source2_s7920.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Result::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop28Done(current_2, arguments__2) => {
                                            let current_2 = *current_2;
                                            {
                                                let result_5 = self::allocate_java_script_call_source_register(current_2.clone());
                                                {
                                                    let source_ml_source2_nominal_field7924 = match result_5.clone() {
                                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7925, _) => {
                                                            let source_ml_source2_data_property_value7925 = *source_ml_source2_data_property_value7925;
                                                            source_ml_source2_data_property_value7925.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_nominal_field7926 = match result_5.clone() {
                                                            self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7927) => {
                                                                source_ml_source2_data_property_value7927
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_nominal_override7928 = self::append_java_script_call_source_instruction(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value7921, _) => {
                                                                    let source_ml_source2_data_property_value7921 = *source_ml_source2_data_property_value7921;
                                                                    source_ml_source2_data_property_value7921.clone()
                                                                }
                                                            }, self::JavaScriptCallInstruction::Primitive(match result_5.clone() {
                                                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value7922) => {
                                                                    source_ml_source2_data_property_value7922
                                                                }
                                                            }, operation.clone(), arguments__2.clone()));
                                                            self::JavaScriptCallSourceResult::SourceResult(Box::new(source_ml_source2_nominal_override7928.clone()), source_ml_source2_nominal_field7926)
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
        self::JavaScriptCallSource::SourceRecur(arguments_2) => {
            if ((match state.clone() {
                self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source2_data_property_value7930, _, _, _, _, _) => {
                    source_ml_source2_data_property_value7930
                }
            }) != (0f64)) {
                self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 114, 101, 99, 117, 114, 114, 101, 110, 99, 101, 32, 116, 104, 114, 111, 117, 103, 104, 32, 97, 110, 32, 97, 99, 116, 105, 118, 101, 32, 101, 120, 99, 101, 112, 116, 105, 111, 110, 32, 104, 97, 110, 100, 108, 101, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 117, 110, 119, 105, 110, 100, 105, 110, 103])))
            } else {
                if (({ let ml_reference_array72 = arguments_2.clone(); let ml_reference_projection72 = ml_reference_array72.borrow().len() as f64; ml_reference_projection72 }) != (match state.clone() {
                    self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, source_ml_source2_data_property_value7932, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value7932
                    }
                })) {
                    self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![108, 111, 111, 112, 32, 117, 112, 100, 97, 116, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100])))
                } else {
                    {
                        let current_3 = state.clone();
                        {
                            let arguments__3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                            {
                                let source_ml_source1_temporary_index27 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s7933 = self::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop29(bindings.clone(), depth, current_3.clone(), arguments__3.clone(), arguments_2.clone(), source_ml_source1_temporary_index27.clone());
                                    match source_ml_source2_s7933.clone() {
                                        self::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Result::MlSource1TemporaryLowerJavaScriptCallSourceComparisonExpressionLoop29Done(current_4, arguments__4) => {
                                            let current_4 = *current_4;
                                            {
                                                let index = 0f64;
                                                {
                                                    let current_5 = self::source_ml_source1_temporary_lower_java_script_call_source_comparison_expression_loop30(current_4.clone(), arguments__4.clone(), index);
                                                    self::JavaScriptCallSourceResult::SourceResult(Box::new(self::append_java_script_call_source_instruction(current_5.clone(), self::JavaScriptCallInstruction::Jump(0f64))), 0f64)
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
        self::JavaScriptCallSource::SourceAllocate(_) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceClosure(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceObject(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArrayAccumulation(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceArray(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyKey(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourcePropertyDefinition(_, _, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotSet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceStringGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceGet(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceInstanceOf(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceDelete(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceHas(_, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
        self::JavaScriptCallSource::SourceSnapshotCall(_, _, _) => {
            self::refuse_java_script_call_source(state.clone(), &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 108, 111, 119, 101, 114, 105, 110, 103, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 107, 105, 110, 100])))
        }
    }
}
